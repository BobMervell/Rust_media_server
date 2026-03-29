use crate::{
    application::abstractions::abstractions::{
        FileExplorer, MovieRepository, MoviesDetailsFetcher, MoviesImagesFetcher, MoviesParser,
    },
    domain::{
        movie::{complete_movie, detailed_movie},
        person::credits,
    },
};

use anyhow::Result;
use futures::StreamExt;
use tracing::{debug_span, info_span};

pub struct MovieIngestionService<E, P, D, I, R>
where
    E: FileExplorer,
    P: MoviesParser,
    D: MoviesDetailsFetcher,
    I: MoviesImagesFetcher,
    R: MovieRepository,
{
    explorer: E,
    parser: P,
    details_fetcher: D,
    images_fetcher: I,
    repository: R,
}

impl<E, P, D, I, R> MovieIngestionService<E, P, D, I, R>
where
    E: FileExplorer,
    P: MoviesParser,
    D: MoviesDetailsFetcher,
    I: MoviesImagesFetcher,
    R: MovieRepository,
{
    pub fn new(
        explorer: E,
        parser: P,
        details_fetcher: D,
        images_fetcher: I,
        repository: R,
    ) -> Self {
        Self {
            explorer,
            parser,
            details_fetcher,
            images_fetcher,
            repository,
        }
    }

    pub async fn ingest_movies(&mut self) -> Result<()> {
        let span = info_span!("fetch_movie_data");
        let _enter = span.enter();
        let path = "";
        let placeholder_path = "";

        let entries = self.explorer.get_entries(path);
        let parsed_movies = self.parser.get_movies(entries);
        let detailed_movies = self.details_fetcher.get_details(parsed_movies);
        let enriched_movies = self.details_fetcher.fetch_credits(detailed_movies);
        let complete_movies = self
            .images_fetcher
            .get_images(enriched_movies, placeholder_path);
        self.repository.save_enriched_movies(complete_movies).await;
        Ok(())
    }
}
