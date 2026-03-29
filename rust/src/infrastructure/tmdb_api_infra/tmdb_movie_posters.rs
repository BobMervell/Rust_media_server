use anyhow::{Context, Result};
use axum::http::{HeaderMap, HeaderValue};
use futures::{StreamExt, TryStreamExt};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Client,
};
use trpl::Stream;

use crate::{
    application::abstractions::abstractions::MoviesImagesFetcher,
    domain::{
        movie::{
            complete_movie::{CompleteEnrichedMovie, CompleteMovie},
            detailed_movie::{DetailedMovie, EnrichedMovie},
        },
        person::person_data::PersonData,
    },
    infrastructure::tmdb_api_infra::utils::fetch_and_store_image,
};

pub struct TMDBMoviesImagesFetcher {
    client: Client,
}

impl MoviesImagesFetcher for TMDBMoviesImagesFetcher {
    fn get_images(
        &self,
        detailed_movies: impl Stream<Item = Result<EnrichedMovie>>,
        placeholder_path: &str,
    ) -> impl Stream<Item = Result<CompleteEnrichedMovie>> {
        let complete_movies = detailed_movies.and_then(move |mut detailed_movie| {
            let client = self.client.clone();
            async move {
                let complete_movie = CompleteMovie::new(&detailed_movie.movie, placeholder_path);
                let mut complete_movie = complete_movie;

                let poster_file_path =
                    Self::update_movie_poster(&client, &detailed_movie.movie, placeholder_path)
                        .await?;
                let backdrop_file_path =
                    Self::update_movie_backdrop(&client, &detailed_movie.movie, placeholder_path)
                        .await?;

                complete_movie.set_backdrop_file_path(poster_file_path);
                complete_movie.set_backdrop_file_path(backdrop_file_path);

                Self::fetch_set_persons_images(
                    &client,
                    &mut detailed_movie.persons,
                    placeholder_path,
                )
                .await;
                let complete_enriched_movie = CompleteEnrichedMovie::new(
                    complete_movie,
                    detailed_movie.credits,
                    detailed_movie.persons,
                );
                return Ok(complete_enriched_movie);
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
        let path = detailed_movie.poster_path().unwrap_or_default();
        fetch_and_store_image(
            client,
            &path,
            "movie",
            "poster",
            detailed_movie.title(),
            "w780",
            placeholder_path,
        )
        .await
    }

    pub async fn update_movie_backdrop(
        client: &Client,
        detailed_movie: &DetailedMovie,
        placeholder_path: &str,
    ) -> Result<String> {
        let path = detailed_movie.poster_path().unwrap_or_default();
        fetch_and_store_image(
            client,
            &path,
            "movie",
            "backdrop",
            detailed_movie.title(),
            "original",
            placeholder_path,
        )
        .await
    }

    pub async fn fetch_set_persons_images(
        client: &Client,
        persons: &mut Vec<PersonData>,
        placeholder_path: &str,
    ) {
        let batch_size = 20;

        let tasks = persons
            .iter()
            .cloned() // clone for frb_generated
            .enumerate()
            .map(|(index, person)| async move {
                let path = Self::update_person_images(client, &person, placeholder_path).await;
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

    async fn update_person_images(
        client: &Client,
        person: &PersonData,
        placeholder_path: &str,
    ) -> Result<String> {
        let person_name = person.name().to_owned();
        let path = person.picture_api_path().unwrap_or_default();

        fetch_and_store_image(
            client,
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
