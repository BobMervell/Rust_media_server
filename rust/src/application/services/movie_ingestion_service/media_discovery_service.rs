use crate::{
    application::abstractions::MediaDiscoveryService, domain::movie::raw_entry::RawEntry,
    infrastructure::file_explorers_infra::smb_infra::SmbInfra,
};

use anyhow::{anyhow, Context, Result};
use async_stream::stream;
use smb::FileDirectoryInformation;
use tracing::debug_span;
use trpl::{Stream, StreamExt};

pub struct SmbExplorer {
    smb_infra: SmbInfra,
    root_path: String,
}

impl SmbExplorer {
    pub async fn new(path: String, username: String, password: String) -> Result<Self> {
        let infra = SmbInfra::new(path.clone(), username, password).await?;
        Ok(Self {
            smb_infra: infra,
            root_path: path,
        })
    }
}

impl MediaDiscoveryService for SmbExplorer {
    fn get_entries<'a>(&'a self, path: &'a str) -> impl Stream<Item = Result<RawEntry>> + 'a {
        let span = debug_span!("fetch_movies", path = path);
        let _enter = span.enter();
        stream! {
            let dir = self.smb_infra
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
                    let (file_path,file_name) = self.smb_infra.parse_file_path(&entry, path,&self.root_path);
                    yield Ok(RawEntry::new(&file_path, &file_name));
                    continue;
                }

                let sub_path = match self.smb_infra.parse_sub_path(&entry, path) {
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
