use crate::application::abstractions::abstractions::{
    FileExplorer, MovieDetailsFetcher, MovieParser, MovieRepository,
};

use anyhow::Result;
use futures::{pin_mut, StreamExt};

pub struct MovieIngestionService<E /*, P, D, R*/>
where
    E: FileExplorer,
    /*P: MovieParser,
    D: MovieDetailsFetcher,
    R: MovieRepository,*/
{
    explorer: E,
    /*parser: P,
    details_fetcher: D,
    repository: R,*/
}

impl<E /*, P, D, R*/> MovieIngestionService<E /*, P, D, R*/>
where
    E: FileExplorer,
    /*P: MovieParser,
    D: MovieDetailsFetcher,
    R: MovieRepository,*/
{
    pub fn new(explorer: E /*parser: P, details_fetcher: D, repository: R*/) -> Self {
        Self {
            explorer,
            /*parser,
            details_fetcher,
            repository,*/
        }
    }

    pub async fn ingest_movies(&self) -> Result<()> {
        let path = "";
        let entries = self.explorer.get_entries(path);

        pin_mut!(entries); // pin the stream on the stack

        while let Some(entry) = entries.next().await {
            println!("{:?}", entry); // TODO: handle Result<RawEntry, anyhow::Error>
        }

        Ok(())
    }
}
