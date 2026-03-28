use crate::{
    application::abstractions::abstractions::FileExplorer, domain::movie::raw_entry::RawEntry,
};

use anyhow::{anyhow, Context, Result};
use async_stream::stream;
use smb::{
    Client, ClientConfig, Directory, FileAccessMask, FileDirectoryInformation, Resource, UncPath,
};
use std::{str::FromStr, sync::Arc};
use tracing::debug_span;
use trpl::{Stream, StreamExt};

pub struct SmbExplorer {
    tree: Arc<smb::Tree>,
    root_path: String,
}

impl FileExplorer for SmbExplorer {
    fn get_entries<'a>(&'a self, path: &'a str) -> impl Stream<Item = Result<RawEntry>> + 'a {
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

            while let Some(entry_res) = entries.next().await {
                let entry = match entry_res {
                    Err(e) => {
                        yield Err(anyhow!("Failed to get entry: {:?}", e));
                        continue;
                    }
                    Ok(e) => e,
                };

                let is_dir = entry.file_attributes.directory();
                if !is_dir {
                    let (file_path,file_name) = self.parse_file_path(&entry, path);
                    yield Ok(RawEntry::new(&file_path, &file_name));
                    continue;
                }

                let sub_path = match self.parse_sub_path(&entry, path) {
                    Some(p) => p,
                    None => continue,
                };

                let mut sub_stream = Box::pin(self.get_entries(&sub_path));
                while let Some(entry_result) = sub_stream.next().await {
                    yield entry_result;
                }
            }
        }
    }
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

        Ok(Self {
            tree: tree,
            root_path: path,
        })
    }

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

    fn parse_sub_path(&self, dir_entry: &FileDirectoryInformation, path: &str) -> Option<String> {
        if dir_entry.file_name == "." || dir_entry.file_name == ".." {
            return None;
        }

        let sub_path = if path.is_empty() {
            dir_entry.file_name.to_string()
        } else {
            format!("{}/{}", path, dir_entry.file_name)
        };
        return Some(sub_path);
    }

    fn parse_file_path(
        &self,
        file_entry: &FileDirectoryInformation,
        path: &str,
    ) -> (String, String) {
        if path.is_empty() {
            let file_name = file_entry.file_name.to_string();
            let file_path = format!("{}/{}", self.root_path, file_name);
            return (file_path, file_name);
        } else {
            let file_path = format!("{}/{}/{}", self.root_path, path, file_entry.file_name);
            let file_name = file_entry.file_name.to_string();
            return (file_path, file_name);
        };
    }
}
