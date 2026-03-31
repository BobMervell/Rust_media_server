use anyhow::{Context, Result};
use futures::{StreamExt, TryStreamExt};

use trpl::Stream;

use crate::{
    application::abstractions::abstractions::MovieAssetService,
    domain::{
        movie::detailed_movie::{DetailedMovie, EnrichedMovie},
        person::person_data::PersonData,
    },
    infrastructure::{external_services::tmdb_asset_api::TmdbAssetApi, os_infra::file_system},
};

pub struct TMDBMovieAssetService {
    tmdb_api: TmdbAssetApi,
}

impl MovieAssetService for TMDBMovieAssetService {
    fn get_assets(
        &self,
        detailed_movies: impl Stream<Item = Result<EnrichedMovie>>,
        placeholder_path: &str,
    ) -> impl Stream<Item = Result<EnrichedMovie>> {
        let complete_movies = detailed_movies.and_then(move |mut detailed_movie| {
            let tmdb_api = self.tmdb_api.clone();
            async move {
                let poster_file_path = Self::fetch_save_movie_poster(
                    &tmdb_api,
                    &detailed_movie.movie,
                    placeholder_path,
                )
                .await?;
                let backdrop_file_path = Self::fetch_save_movie_backdrop(
                    &tmdb_api,
                    &detailed_movie.movie,
                    placeholder_path,
                )
                .await?;

                detailed_movie.movie.set_poster_file_path(poster_file_path);
                detailed_movie
                    .movie
                    .set_backdrop_file_path(backdrop_file_path);

                Self::fetch_save_persons_profile(
                    &tmdb_api,
                    &mut detailed_movie.persons,
                    placeholder_path,
                )
                .await;
                return Ok(detailed_movie);
            }
        });
        return complete_movies;
    }
}

impl TMDBMovieAssetService {
    pub fn new(token: &str) -> Result<Self> {
        Ok(Self {
            tmdb_api: TmdbAssetApi::new(token)?,
        })
    }
    pub async fn fetch_save_movie_poster(
        tmdb_api: &TmdbAssetApi,
        detailed_movie: &DetailedMovie,
        placeholder_path: &str,
    ) -> Result<String> {
        let path = detailed_movie.ext_poster_path().unwrap_or_default();
        Self::fetch_and_store_image(
            tmdb_api,
            &path,
            "movie",
            "poster",
            detailed_movie.title(),
            "w780",
            placeholder_path,
        )
        .await
    }

    pub async fn fetch_save_movie_backdrop(
        tmdb_api: &TmdbAssetApi,
        detailed_movie: &DetailedMovie,
        placeholder_path: &str,
    ) -> Result<String> {
        let path = detailed_movie.ext_backdrop_path().unwrap_or_default();
        Self::fetch_and_store_image(
            tmdb_api,
            &path,
            "movie",
            "backdrop",
            detailed_movie.title(),
            "original",
            placeholder_path,
        )
        .await
    }

    pub async fn fetch_save_persons_profile(
        tmdb_api: &TmdbAssetApi,
        persons: &mut Vec<PersonData>,
        placeholder_path: &str,
    ) {
        let batch_size = 50;

        let tasks = persons
            .iter()
            .cloned() // clone for frb_generated
            .enumerate()
            .map(|(index, person)| async move {
                let path =
                    Self::fetch_save_person_profile(tmdb_api, &person, placeholder_path).await;
                (index, person, path)
            })
            .collect::<Vec<_>>();

        let results = futures::stream::iter(tasks)
            .buffer_unordered(batch_size)
            .collect::<Vec<_>>()
            .await;

        for (index, mut person, result) in results {
            if let Ok(path) = result {
                person.set_picture_file_path(path);
                persons[index] = person;
            }
        }
    }

    async fn fetch_and_store_image(
        tmdb_api: &TmdbAssetApi,
        picture_path: &str,
        category: &str,
        name: &str,
        subdir: &str,
        image_size: &str,
        placeholder_path: &str,
    ) -> Result<String> {
        let (created, file_path) = file_system::create_dir(category, subdir, name)
            .with_context(|| format!("Error creating directory for: {}", name))?;

        if !created {
            tracing::debug!("Picture path already exists: {}", file_path);
            return Ok(file_path);
        }

        match tmdb_api.get_image(image_size, picture_path).await {
            Ok(mut image) => {
                file_system::save_image(&mut image, &file_path)
                    .await
                    .with_context(|| format!("Failed to save image for: {}", name))?;

                return Ok(file_path);
            }
            Err(e) => {
                tracing::debug!("Failed to get image for: {}.\n Caused by {}", name, e);
                return Ok(placeholder_path.to_owned());
            }
        }
    }

    async fn fetch_save_person_profile(
        tmdb_api: &TmdbAssetApi,
        person: &PersonData,
        placeholder_path: &str,
    ) -> Result<String> {
        let person_name = person.name().to_owned();
        let path = person.picture_api_path().unwrap_or_default();

        Self::fetch_and_store_image(
            tmdb_api,
            &path,
            "person",
            &person_name,
            &person_name,
            "w300",
            placeholder_path,
        )
        .await
    }
}
