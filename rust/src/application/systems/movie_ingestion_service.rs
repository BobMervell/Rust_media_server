use crate::{
    application::abstractions::abstractions::{
        FileExplorer, MovieDetailsFetcher, MovieRepository, MoviesParser,
    },
    domain::movie::detailed_movie,
};

use anyhow::Result;
use futures::StreamExt;

pub struct MovieIngestionService<E, P, D /*R*/>
where
    E: FileExplorer,
    P: MoviesParser,
    D: MovieDetailsFetcher,
    // R: MovieRepository,
{
    explorer: E,
    parser: P,
    details_fetcher: D,
    // repository: R,
}

impl<E, P, D /*R*/> MovieIngestionService<E, P, D /*R*/>
where
    E: FileExplorer,
    P: MoviesParser,
    D: MovieDetailsFetcher,
    // R: MovieRepository,
{
    pub fn new(explorer: E, parser: P, details_fetcher: D /*repository: R*/) -> Self {
        Self {
            explorer,
            parser,
            details_fetcher,
            // repository,
        }
    }

    pub async fn ingest_movies(&self) -> Result<()> {
        let path = "";

        let entries = self.explorer.get_entries(path);
        let parsed_movies = self.parser.get_movies(entries);
        let mut detailed_movie = Box::pin(self.details_fetcher.get_details(parsed_movies));
        while let Some(movie) = detailed_movie.next().await {
            println!("{:?}", movie);
        }

        Ok(())
    }
}
