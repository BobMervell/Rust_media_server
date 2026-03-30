use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Context, Result};
use smb::{
    Client, ClientConfig, Directory, FileAccessMask, FileDirectoryInformation, Resource, UncPath,
};

pub struct SmbInfra {
    tree: Arc<smb::Tree>,
}

impl SmbInfra {
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

    pub async fn read_directory(&self, path: &str) -> Result<Arc<Directory>> {
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

    pub fn parse_sub_path(
        &self,
        dir_entry: &FileDirectoryInformation,
        path: &str,
    ) -> Option<String> {
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

    pub fn parse_file_path(
        &self,
        file_entry: &FileDirectoryInformation,
        path: &str,
        root_path: &str,
    ) -> (String, String) {
        if path.is_empty() {
            let file_name = file_entry.file_name.to_string();
            let file_path = format!("{}/{}", root_path, file_name);
            return (file_path, file_name);
        } else {
            let file_path = format!("{}/{}/{}", root_path, path, file_entry.file_name);
            let file_name = file_entry.file_name.to_string();
            return (file_path, file_name);
        };
    }
}
