use anyhow::Result;
use trpl::Stream;

use crate::domain::movie::raw_entry::RawEntry;

pub trait FileExplorer {
    fn get_entries<'a>(&'a self, path: &'a str) -> impl Stream<Item = Result<RawEntry>> + 'a;
}

pub trait MovieParser {
    fn new(&self) -> Self;
}

pub trait MovieDetailsFetcher {
    fn new(&self) -> Self;
}

pub trait MovieRepository {
    fn new(&self) -> Self;
}
