use anyhow::Result;
use trpl::Stream;

use crate::domain::movie::{parsed_movie::ParsedMovie, raw_entry::RawEntry};

pub trait FileExplorer {
    fn get_entries<'a>(&'a self, path: &'a str) -> impl Stream<Item = Result<RawEntry>> + 'a;
}

pub trait MoviesParser {
    fn get_movies(
        &self,
        entries: impl Stream<Item = Result<RawEntry>>,
    ) -> impl Stream<Item = Result<ParsedMovie>>;
}

pub trait MovieDetailsFetcher {
    fn new(&self) -> Self;
}

pub trait MovieRepository {
    fn new(&self) -> Self;
}
