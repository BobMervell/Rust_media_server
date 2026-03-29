use anyhow::{anyhow, Context, Result};
use axum::http::{HeaderMap, HeaderValue};
use futures::TryStreamExt;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Client,
};
use trpl::Stream;

use crate::{
    application::abstractions::abstractions::MoviesDetailsFetcher,
    domain::movie::{
        detailed_movie::{DetailedMovie, MovieDetailResult},
        parsed_movie::ParsedMovie,
        value_objects::MovieGenres,
    },
};

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

pub struct TMDBMoviesDetailsFetcher {
    client: Client,
}

impl MoviesDetailsFetcher for TMDBMoviesDetailsFetcher {
    fn get_details(
        &self,
        parsed_movies: impl Stream<Item = Result<ParsedMovie>>,
    ) -> impl Stream<Item = Result<DetailedMovie>> {
        let detailed_movies = parsed_movies.and_then(|parsed_movie| {
            let client = self.client.clone();
            async move {
                let mut movie = Self::get_movie_basics(&parsed_movie, &client)
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to get search response for movie: {} ({}).",
                            parsed_movie.file_title(),
                            parsed_movie.file_year()
                        )
                    })?;

                let genres = Self::fetch_movie_genres(&client, movie.tmdb_id())
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
}

impl TMDBMoviesDetailsFetcher {
    pub fn new(token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        //TODO WARNING HeaderValue::from_str is intended to be replaced in the future by a TryFrom.
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("bearer {}", token))
                .context("Failed to create header value with token")?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to build client")?;

        Ok(Self { client: client })
    }

    async fn get_movie_basics(
        parsed_movie: &ParsedMovie,
        client: &Client,
    ) -> Result<DetailedMovie> {
        let movies_result = Self::fetch_corresponding_movies(
            client,
            parsed_movie.file_title(),
            parsed_movie.file_year(),
        )
        .await;

        match movies_result {
            Ok(movies) => {
                let movie = Self::get_most_popular(movies);
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

    /// Fetches movie information from the TMDB API by name and year, returning the result.
    async fn fetch_corresponding_movies(
        client: &Client,
        movie_name: &str,
        movie_year: &str,
    ) -> Result<MovieDetailResult> {
        let params = &[("query", movie_name), ("primary_release_year", movie_year)];

        let url = format!("{}/search/movie", TMDB_BASE_URL);

        let response = client
            .get(&url)
            .query(&params)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to get search response for movie: {} ({:?}), from url: {}",
                    movie_name, movie_year, &url
                )
            })?
            .error_for_status()
            .with_context(|| {
                format!(
                    "TMDB returned error status for movie: {} ({:?}), from url: {}",
                    movie_name, movie_year, &url
                )
            })?;

        let movies = response
            .json::<MovieDetailResult>()
            .await
            .with_context(|| {
                format!(
                    "Failed to deserialize search response for movie: {} ({:?}), from url: {}",
                    movie_name, movie_year, &url
                )
            })?;

        if movies.results().len() == 0 {
            return Err(anyhow!(
                "No result foud for for movie: {} ({:?}), from url: {}",
                movie_name,
                movie_year,
                &url
            ));
        }

        Ok(movies)
    }

    fn get_most_popular(fetch_result: MovieDetailResult) -> DetailedMovie {
        let mut max_pop: f32 = 0.0;
        let mut result_movie = fetch_result.results()[0].clone();
        for movie in fetch_result.iter() {
            if movie.popularity() > max_pop {
                max_pop = movie.popularity();
                result_movie = movie.to_owned();
            }
        }
        return result_movie;
    }

    /// Fetches movie genres from the TMDB API by tmdbId.
    pub async fn fetch_movie_genres(client: &Client, tmdb_id: i64) -> Result<MovieGenres> {
        let url = format!("{}/movie/{}?language=en-US", TMDB_BASE_URL, &tmdb_id);

        let response = client
            .get(&url)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to get genre response for movie id: {} , from url: {}",
                    tmdb_id, &url
                )
            })?
            .error_for_status()
            .with_context(|| {
                format!(
                    "TMDB returned error status for movie id: {} , from url: {}",
                    tmdb_id, &url
                )
            })?;

        let genres = response.json::<MovieGenres>().await.with_context(|| {
            format!(
                "Failed to deserialize genre response for movie id: {}, from url: {}",
                tmdb_id, &url
            )
        })?;
        Ok(genres)
    }
}
