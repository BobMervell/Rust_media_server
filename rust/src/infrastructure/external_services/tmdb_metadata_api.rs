use anyhow::{anyhow, Context, Result};
use axum::http::{HeaderMap, HeaderValue};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Client,
};

use crate::{
    api::media::PersonData,
    domain::{
        movie::{detailed_movie::MovieDetailResult, value_objects::MovieGenres},
        person::credits::CreditsMovie,
    },
};

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

#[derive(Clone)]
pub struct TmdbMetaDataApi {
    client: Client,
}

impl TmdbMetaDataApi {
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

    pub async fn fetch_movies(
        &self,
        movie_name: &str,
        movie_year: &str,
    ) -> Result<MovieDetailResult> {
        let params = &[("query", movie_name), ("primary_release_year", movie_year)];

        let url = format!("{}/search/movie", TMDB_BASE_URL);

        let response = self
            .client
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

    pub async fn fetch_movie_genres(&self, tmdb_id: i64) -> Result<MovieGenres> {
        let url = format!("{}/movie/{}?language=en-US", TMDB_BASE_URL, &tmdb_id);

        let response = self
            .client
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

    pub async fn fetch_movie_credits(&self, tmdb_id: i64) -> Result<CreditsMovie> {
        let url = format!(
            "{}/movie/{}/credits?language=en-US",
            TMDB_BASE_URL, &tmdb_id
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to get credit response for movie id: {} , from url: {}",
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

        let credits_details = response.json::<CreditsMovie>().await.with_context(|| {
            format!(
                "Failed to deserialize credit response for movie id: {}, from url: {}",
                tmdb_id, &url
            )
        })?;
        Ok(credits_details)
    }

    pub async fn fetch_person_details(&self, tmdb_id: i64) -> Result<PersonData> {
        let url = format!("{}/person/{}", TMDB_BASE_URL, &tmdb_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to get person detail response for person id: {} , from url: {}",
                    tmdb_id, &url
                )
            })?
            .error_for_status()
            .with_context(|| {
                format!(
                    "TMDB returned error status for person id: {} , from url: {}",
                    tmdb_id, &url
                )
            })?;

        let person_details = response.json::<PersonData>().await.with_context(|| {
            format!(
                "Failed to deserialize person details response for person id: {}, from url: {}",
                tmdb_id, &url
            )
        })?;
        Ok(person_details)
    }
}
