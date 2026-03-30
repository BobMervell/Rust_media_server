use crate::{
    application::abstractions::abstractions::MovieFactory,
    domain::{
        movie::{parsed_movie::ParsedMovie, raw_entry::RawEntry},
        utils,
    },
};
use anyhow::{anyhow, Result};
use trpl::{Stream, StreamExt};
pub struct MovieExtractor {}

impl MovieFactory for MovieExtractor {
    fn get_movies(
        &self,
        entries: impl Stream<Item = Result<RawEntry>>,
    ) -> impl Stream<Item = Result<ParsedMovie>> {
        let parsed_movies = entries.filter_map(|entry_res| match entry_res {
            Ok(entry) => {
                if utils::is_video_file(&entry.file_name())
                    && utils::is_not_featurette(&entry.file_name())
                {
                    return Some(ParsedMovie::new(entry.file_path(), entry.file_name()));
                }
                return None;
            }
            Err(e) => Some(Err(anyhow!("Failed to parse entry.\n Caused by: {}", e))),
        });
        return parsed_movies;
    }
}
