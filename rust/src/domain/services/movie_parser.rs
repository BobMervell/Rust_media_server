use crate::{
    application::abstractions::abstractions::MovieFactory,
    domain::movie::{parsed_movie::ParsedMovie, raw_entry::RawEntry},
};
use anyhow::{anyhow, Result};
use trpl::{Stream, StreamExt};
pub struct MovieNameParser {}

impl MovieFactory for MovieNameParser {
    fn get_movies(
        &self,
        entries: impl Stream<Item = Result<RawEntry>>,
    ) -> impl Stream<Item = Result<ParsedMovie>> {
        let parsed_movies = entries.filter_map(|entry_res| match entry_res {
            Ok(entry) => {
                if Self::is_video_file(&entry.file_name())
                    && Self::is_not_featurette(&entry.file_name())
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

impl MovieNameParser {
    fn is_video_file(file_name: &str) -> bool {
        let video_extensions = ["mp4", "mkv", "avi", "mov", "flv", "wmv", "webm"];

        if let Some(ext) = file_name.rsplit('.').next() {
            video_extensions.contains(&ext.to_lowercase().as_str())
        } else {
            false
        }
    }

    fn is_not_featurette(file_path: &str) -> bool {
        let featurette_names = ["featurettes", "featurette", "feat"];
        if let Some(ext) = file_path.rsplit('/').next() {
            !featurette_names.contains(&ext.to_lowercase().as_str())
        } else {
            true
        }
    }
}
