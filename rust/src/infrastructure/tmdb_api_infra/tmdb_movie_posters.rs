use anyhow::{Context, Result};
use axum::http::{HeaderMap, HeaderValue};
use futures::TryStreamExt;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Client,
};
use trpl::Stream;

use crate::{
    application::abstractions::abstractions::MoviesImagesFetcher,
    domain::movie::{complete_movie::CompleteMovie, detailed_movie::DetailedMovie},
    infrastructure::tmdb_api_infra::utils::fetch_and_store_image,
};

pub struct TMDBMoviesImagesFetcher {
    client: Client,
}

impl MoviesImagesFetcher for TMDBMoviesImagesFetcher {
    fn get_images(
        &self,
        detailed_movies: impl Stream<Item = Result<DetailedMovie>>,
        placeholder_path: &str,
    ) -> impl Stream<Item = Result<CompleteMovie>> {
        let complete_movies = detailed_movies.and_then(move |detailed_movie| {
            let client = self.client.clone();
            async move {
                let mut complete_movie = CompleteMovie::new(&detailed_movie, placeholder_path);

                let poster_file_path =
                    Self::update_movie_poster(&client, &detailed_movie, placeholder_path).await?;
                let backdrop_file_path =
                    Self::update_movie_backdrop(&client, &detailed_movie, placeholder_path).await?;

                complete_movie.set_backdrop_file_path(poster_file_path);
                complete_movie.set_backdrop_file_path(backdrop_file_path);

                return Ok(complete_movie);
            }
        });
        return complete_movies;
    }
}

impl TMDBMoviesImagesFetcher {
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

    pub async fn update_movie_poster(
        client: &Client,
        detailed_movie: &DetailedMovie,
        placeholder_path: &str,
    ) -> Result<String> {
        let path = detailed_movie
            .poster_path()
            .unwrap_or(format!("{}/poster", placeholder_path));
        fetch_and_store_image(
            client,
            &path,
            "movie",
            "poster",
            detailed_movie.title(),
            "w780",
        )
        .await
    }

    pub async fn update_movie_backdrop(
        client: &Client,
        detailed_movie: &DetailedMovie,
        placeholder_path: &str,
    ) -> Result<String> {
        let path = detailed_movie
            .poster_path()
            .unwrap_or(format!("{}/backdrop", placeholder_path));
        fetch_and_store_image(
            client,
            &path,
            "movie",
            "backdrop",
            detailed_movie.title(),
            "original",
        )
        .await
    }
}
