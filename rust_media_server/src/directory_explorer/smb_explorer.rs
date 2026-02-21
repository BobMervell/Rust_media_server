use crate::movie_data::movie_data::MovieData;
use anyhow::{Context, Result};
use smb::{
    Client, ClientConfig, Directory, FileAccessMask, FileDirectoryInformation, Resource, UncPath,
};
use std::{io, str::FromStr, sync::Arc};
use trpl::StreamExt;

//TODO replace tempo_smb_connect()
pub async fn tempo_smb_connect() -> Result<SmbExplorer> {
    let mut path = String::new();
    println!("Enter the samba remote path");
    io::stdin()
        .read_line(&mut path)
        .context("Failed to read remote path")?;
    let path = path.trim_end();

    let mut username = String::new();
    println!("Enter the username");
    io::stdin()
        .read_line(&mut username)
        .context("Failed to read username")?;
    let username = username.trim_end();

    let mut password = String::new();
    println!("Enter the passord");
    io::stdin()
        .read_line(&mut password)
        .context("Failed to read password")?;
    let password = password.trim_end();

    SmbExplorer::new(path.to_owned(), username.to_owned(), password.to_owned()).await
}

pub struct SmbExplorer {
    tree: Arc<smb::Tree>,
}

impl SmbExplorer {
    pub async fn new(path: String, username: String, password: String) -> Result<Self> {
        let client = Client::new(ClientConfig::default());
        let uncpath: UncPath = UncPath::from_str(&path).unwrap();
        client
            .share_connect(&uncpath, &username, password)
            .await
            .context("Failed to connect to remote")?;

        let tree = client
            .get_tree(&uncpath)
            .await
            .context("Failed to read retrieve remote directory tree")?;
        Ok(Self { tree: tree })
    }

    pub async fn fetch_movies(&self) -> Result<Vec<MovieData>> {
        let root_path = "";
        let movies = self.explore_path(root_path).await;
        return movies;
    }

    async fn explore_path(&self, path: &str) -> Result<Vec<MovieData>> {
        let mut movies = Vec::new();
        let entries = self
            .read_directory(path)
            .await
            .context("Failed to iterate over data stream")?;

        for entry in entries {
            if entry.file_attributes.directory() {
                let (is_valid, sub_path) = self.parse_sub_path(&entry, path);
                if is_valid {
                    let more_movies = Box::pin(self.explore_path(&sub_path))
                        .await
                        .with_context(|| format!("Failed to explore path: {}", sub_path))?;
                    movies.extend(more_movies);
                }
            } else {
                let file_path = self.parse_file_path(&entry, path);
                if self.is_video_file(&entry.file_name.to_string()) && self.is_not_featurette(&path)
                {
                    let res = MovieData::new(&file_path);
                    match res {
                        Ok(movie) => {
                            movies.push(movie);
                        }
                        Err(e) => {
                            // TODO add parsed failed movies to a list for user
                            tracing::error!(
                                //important to have :? for complete error
                                "Error, failed to initiate movie data: {} \n Caused by: {:?}",
                                &file_path,
                                e
                            );
                        }
                    }
                }
            }
        }
        return Ok(movies);
    }

    async fn read_directory(&self, path: &str) -> Result<Vec<FileDirectoryInformation>> {
        let mut entries = Vec::new();
        let access_mask = FileAccessMask::new().with_generic_read(true);

        let resource = self
            .tree
            .open_existing(&path, access_mask)
            .await
            .with_context(|| format!("Could not open directory: {}", path))?;

        if let Resource::Directory(dir) = resource {
            let dir: Arc<Directory> = Arc::from(dir);
            let mut data_stream = smb::Directory::query::<FileDirectoryInformation>(&dir, "*")
                .await
                .with_context(|| format!("Failed to get files info in: {}", path))?;
            while let Some(entry) = data_stream.try_next().await? {
                entries.push(entry);
            }
        }
        Ok(entries)
    }

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

    fn parse_file_path(&self, file_entry: &FileDirectoryInformation, path: &str) -> String {
        if path.is_empty() {
            return file_entry.file_name.to_string();
        } else {
            return format!("{}/{}", path, file_entry.file_name);
        };
    }

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
}
