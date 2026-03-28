use crate::{
    application::abstractions::abstractions::MoviesParser,
    domain::movie::{parsed_movie::ParsedMovie, raw_entry::RawEntry},
};
use anyhow::Result;
use trpl::{Stream, StreamExt};
pub struct MovieNameParser {}

impl MoviesParser for MovieNameParser {
    fn get_movies(
        &self,
        entries: impl Stream<Item = Result<RawEntry>>,
    ) -> impl Stream<Item = Result<ParsedMovie>> {
        let parsed_movies = entries.map(|entry_res| {
            entry_res.and_then(|entry| ParsedMovie::new(entry.file_path(), entry.file_name()))
        });
        return parsed_movies;
    }
}
