use crate::{domain::movie::raw_entry::RawEntry, movie_data::movie_data::MovieData};
use anyhow::{anyhow, Context, Result};
use async_stream::stream;
use smb::{
    Client, ClientConfig, Directory, FileAccessMask, FileDirectoryInformation, Resource, UncPath,
};
use tracing::debug_span;

use std::{str::FromStr, sync::Arc};
use trpl::{Stream, StreamExt};

/// Represents the state and configuration for exploring an SMB shared directory.
pub struct SmbExplorer {
    tree: Arc<smb::Tree>,
}

impl SmbExplorer {
    pub async fn new(path: String, username: String, password: String) -> Result<Self> {
        let client = Client::new(ClientConfig::default());
        let uncpath: UncPath = UncPath::from_str(&path)
            .with_context(|| format!("Failed to unwrap path from string: {}", &path))?;

        client
            .share_connect(&uncpath, &username, password)
            .await
            .with_context(|| format!("Failed to connect to remote: {}", &path))?;

        let tree = client
            .get_tree(&uncpath)
            .await
            .with_context(|| format!("Failed to retrieve directory from remote: {}", &path))?;

        Ok(Self { tree: tree })
    }

    /// Recursively explores an SMB path and returns a stream of discovered movies.
    ///
    /// Traverses each subfolder, yielding a MovieData object for every video file
    /// whose file is not inside a featurette folder. The MovieData is constructed from the file name.
    pub fn fetch_movies<'a>(&'a self, path: &'a str) -> impl Stream<Item = Result<RawEntry>> + 'a {
        let span = debug_span!("fetch_movies", path = path);
        let _enter = span.enter();

        stream! {
            let dir = self
                .read_directory(path)
                .await
                .context("Failed to open directory")?;

            let mut entries = smb::Directory::query::<FileDirectoryInformation>(&dir, "*")
                .await
                .with_context(|| format!("Failed to get files info in: {}", path))?;

            while let Some(entry) = entries.try_next().await? {
                if entry.file_attributes.directory() {
                    let (is_valid, sub_path) = self.parse_sub_path(&entry, path);
                    if ! is_valid {
                        continue;
                    }
                    let mut more_movies = Box::pin(self.fetch_movies(&sub_path));
                    while let Some(movie) = more_movies.next().await {
                        yield movie
                    }
                } else {
                    let file_path = self.parse_file_path(&entry, path);
                    if !self.is_video_file(&entry.file_name.to_string()) || !self.is_not_featurette(&path) {
                        continue;
                     }
                        yield(Ok(RawEntry::new(&file_path)))
                }
            }
        }
    }

    /// Opens the given SMB file path and returns Ok(directory) if it is a folder, or an error otherwise.
    async fn read_directory(&self, path: &str) -> Result<Arc<Directory>> {
        let access_mask = FileAccessMask::new().with_generic_read(true);

        let resource = self
            .tree
            .open_existing(&path, access_mask)
            .await
            .with_context(|| format!("Failed to open ressource: {}", path))?;

        if let Resource::Directory(dir) = resource {
            Ok(Arc::from(dir))
        } else {
            return Err(anyhow!("Ressource is not a directory: {}", path));
        }
    }

    // region: ---- PARSE PATHS ----
    /// Parses a subfolder path into its components.
    fn parse_sub_path(&self, dir_entry: &FileDirectoryInformation, path: &str) -> (bool, String) {
        if dir_entry.file_name == "." || dir_entry.file_name == ".." {
            return (false, "".to_string());
        }

        let sub_path = if path.is_empty() {
            dir_entry.file_name.to_string()
        } else {
            format!("{}/{}", path, dir_entry.file_name)
        };
        return (true, sub_path);
    }

    /// Parses a file path into its components.
    fn parse_file_path(&self, file_entry: &FileDirectoryInformation, path: &str) -> String {
        if path.is_empty() {
            return file_entry.file_name.to_string();
        } else {
            return format!("{}/{}", path, file_entry.file_name);
        };
    }
    // endregion
    // region: ---- FILTER VIDEOS ----
    fn is_video_file(&self, file_name: &str) -> bool {
        let video_extensions = ["mp4", "mkv", "avi", "mov", "flv", "wmv", "webm"];

        if let Some(ext) = file_name.rsplit('.').next() {
            video_extensions.contains(&ext.to_lowercase().as_str())
        } else {
            false
        }
    }

    fn is_not_featurette(&self, file_path: &str) -> bool {
        let featurette_names = ["featurettes", "featurette", "feat"];
        if let Some(ext) = file_path.rsplit('/').next() {
            !featurette_names.contains(&ext.to_lowercase().as_str())
        } else {
            true
        }
    }
    // endregion
}
