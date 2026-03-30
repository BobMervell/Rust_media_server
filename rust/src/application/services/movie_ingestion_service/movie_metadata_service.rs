use anyhow::{anyhow, Context, Result};
use futures::{stream, StreamExt, TryStreamExt};
use trpl::Stream;

use crate::{
    application::abstractions::abstractions::MovieMetadataService,
    domain::{
        movie::{
            detailed_movie::{DetailedMovie, EnrichedMovie},
            parsed_movie::ParsedMovie,
        },
        person::{credits::CreditsMovie, person_data::PersonData},
        service::{filter_credits, filter_movies},
    },
    infrastructure::external_services::tmdb_metadata_api::TmdbMetaDataApi,
};

pub struct TMDBMovieMetadataService {
    tmdb_api: TmdbMetaDataApi,
}

impl MovieMetadataService for TMDBMovieMetadataService {
    fn get_details(
        &self,
        parsed_movies: impl Stream<Item = Result<ParsedMovie>>,
    ) -> impl Stream<Item = Result<DetailedMovie>> {
        let detailed_movies = parsed_movies.and_then(|parsed_movie| {
            let tmdb_api = self.tmdb_api.clone();
            async move {
                let mut movie = Self::get_movie_basics(&tmdb_api, &parsed_movie)
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to get search response for movie: {} ({}).",
                            parsed_movie.file_title(),
                            parsed_movie.file_year()
                        )
                    })?;

                let genres = tmdb_api
                    .fetch_movie_genres(movie.tmdb_id())
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to get movie genre on tmbd for: {} ({}).",
                            parsed_movie.file_title(),
                            parsed_movie.file_year()
                        )
                    })?;
                movie.set_genre(&genres);
                Ok(movie)
            }
        });
        return detailed_movies;
    }

    fn fetch_credits(
        &self,
        detailed_movies: impl Stream<Item = Result<DetailedMovie>>,
    ) -> impl Stream<Item = Result<EnrichedMovie>> {
        let enriched_movies = detailed_movies.and_then(|detailed_movie| {
            println!("{}", detailed_movie.title());
            let tmdb_api = self.tmdb_api.clone();
            async move {
                let credits = tmdb_api
                    .fetch_movie_credits(detailed_movie.tmdb_id())
                    .await?;
                let persons = Self::get_persons_details(&tmdb_api, &credits).await;
                let enriched_movie = EnrichedMovie::new(detailed_movie, credits, persons);
                return Ok(enriched_movie);
            }
        });
        return enriched_movies;
    }
}

impl TMDBMovieMetadataService {
    pub fn new(token: &str) -> Result<Self> {
        Ok(Self {
            tmdb_api: TmdbMetaDataApi::new(token)?,
        })
    }

    async fn get_movie_basics(
        tmdb_api: &TmdbMetaDataApi,
        parsed_movie: &ParsedMovie,
    ) -> Result<DetailedMovie> {
        let movies_result = tmdb_api
            .fetch_movies(parsed_movie.file_title(), parsed_movie.file_year())
            .await;

        match movies_result {
            Ok(movies) => {
                let movie = filter_movies::get_most_popular(movies);
                Ok(movie
                    .set_file_path(parsed_movie.file_path())
                    .set_file_title(parsed_movie.file_title())
                    .set_file_optional_info(parsed_movie.file_optional_info()))
            }
            Err(e) => Err(anyhow!(
                "Failed to fetch movie: {} ({}). \n Caused by {:?}",
                parsed_movie.file_title(),
                parsed_movie.file_year(),
                e
            )),
        }
    }

    async fn get_persons_details(
        tmdb_api: &TmdbMetaDataApi,
        credits: &CreditsMovie,
    ) -> Vec<PersonData> {
        let mut person_tmdb_ids: Vec<i64> = credits
            .cast()
            .iter()
            .filter_map(|c| filter_credits::is_credited(c).then(|| c.tmdb_id()))
            .collect();

        let crew_ids: Vec<i64> = credits
            .crew()
            .iter()
            .filter_map(|c| filter_credits::is_main_crew(c).then(|| c.tmdb_id()))
            .collect();

        person_tmdb_ids.extend(crew_ids);

        let batch_size = 200;

        let persons = stream::iter(person_tmdb_ids)
            .map(|id| async move {
                tmdb_api
                    .fetch_person_details(id)
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed for id {}: {}", id, e);
                    })
                    .ok()
            })
            .buffer_unordered(batch_size)
            .filter_map(|p| async move { p })
            .collect()
            .await;

        return persons;
    }
}
