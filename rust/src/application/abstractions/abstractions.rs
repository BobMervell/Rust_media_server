use anyhow::Result;
use trpl::Stream;

use crate::domain::movie::{
    complete_movie::CompleteMovie, detailed_movie::DetailedMovie, parsed_movie::ParsedMovie,
    raw_entry::RawEntry,
};

pub trait FileExplorer {
    fn get_entries<'a>(&'a self, path: &'a str) -> impl Stream<Item = Result<RawEntry>> + 'a;
}

pub trait MoviesParser {
    fn get_movies(
        &self,
        entries: impl Stream<Item = Result<RawEntry>>,
    ) -> impl Stream<Item = Result<ParsedMovie>>;
}

pub trait MoviesDetailsFetcher {
    fn get_details(
        &self,
        parsed_movies: impl Stream<Item = Result<ParsedMovie>>,
    ) -> impl Stream<Item = Result<DetailedMovie>>;
}

pub trait MoviesImagesFetcher {
    fn get_images(
        &self,
        detailed_movies: impl Stream<Item = Result<DetailedMovie>>,
        placeholder_path: &str,
    ) -> impl Stream<Item = Result<CompleteMovie>>;
}

pub trait MoviesCreditsFetcher {
    fn get_credits(
        &self,
        parsed_movies: impl Stream<Item = Result<ParsedMovie>>,
    ) -> impl Stream<Item = Result<DetailedMovie>>;
}

pub trait MovieRepository {
    fn new(&self) -> Self;
}
