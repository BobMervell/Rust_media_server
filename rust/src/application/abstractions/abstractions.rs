use anyhow::Result;
use trpl::Stream;

use crate::domain::movie::{
    complete_movie::CompleteEnrichedMovie,
    detailed_movie::{DetailedMovie, EnrichedMovie},
    parsed_movie::ParsedMovie,
    raw_entry::RawEntry,
};

pub trait MediaDiscoveryService {
    fn get_entries<'a>(&'a self, path: &'a str) -> impl Stream<Item = Result<RawEntry>> + 'a;
}

pub trait MovieFactory {
    fn get_movies(
        &self,
        entries: impl Stream<Item = Result<RawEntry>>,
    ) -> impl Stream<Item = Result<ParsedMovie>>;
}

pub trait MovieMetadataService {
    fn get_details(
        &self,
        parsed_movies: impl Stream<Item = Result<ParsedMovie>>,
    ) -> impl Stream<Item = Result<DetailedMovie>>;

    fn fetch_credits(
        &self,
        detailed_movies: impl Stream<Item = Result<DetailedMovie>>,
    ) -> impl Stream<Item = Result<EnrichedMovie>>;
}

pub trait MovieAssetService {
    fn get_assets(
        &self,
        detailed_movies: impl Stream<Item = Result<EnrichedMovie>>,
        placeholder_path: &str,
    ) -> impl Stream<Item = Result<CompleteEnrichedMovie>>;
}

pub trait MovieRepository {
    async fn save_enriched_movies(
        &mut self,
        enriched_movies: impl Stream<Item = Result<CompleteEnrichedMovie>>,
    ) -> Vec<(Result<()>, Result<()>)>;
}
