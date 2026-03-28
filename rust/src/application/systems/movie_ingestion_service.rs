use crate::application::abstractions::abstractions::{
    FileExplorer, MovieDetailsFetcher, MovieRepository, MoviesParser,
};

use anyhow::Result;
use futures::StreamExt;

pub struct MovieIngestionService<E, P /*, D, R*/>
where
    E: FileExplorer,
    P: MoviesParser,
    /*D: MovieDetailsFetcher,
    R: MovieRepository,*/
{
    explorer: E,
    parser: P,
    /*details_fetcher: D,
    repository: R,*/
}

impl<E, P /*, D, R*/> MovieIngestionService<E, P /*, D, R*/>
where
    E: FileExplorer,
    P: MoviesParser,
    /*D: MovieDetailsFetcher,
    R: MovieRepository,*/
{
    pub fn new(explorer: E, parser: P /*details_fetcher: D, repository: R*/) -> Self {
        Self {
            explorer,
            parser,
            /*etails_fetcher,
            repository,*/
        }
    }

    pub async fn ingest_movies(&self) -> Result<()> {
        let path = "";

        let entries = self.explorer.get_entries(path);
        let mut parsed_movies = Box::pin(self.parser.get_movies(entries));
        while let Some(movie) = parsed_movies.next().await {
            println!("{:?}", movie);
        }

        Ok(())
    }
}
