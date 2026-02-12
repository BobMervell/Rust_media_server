use crate::movie_data::movie_data::MovieData;
use futures::{FutureExt, future::BoxFuture};
use smb::{
    Client, ClientConfig, Directory, FileAccessMask, FileDirectoryInformation, Resource, UncPath,
    resource::iter_stream::QueryDirectoryStream,
};
use std::{
    error::Error,
    str::FromStr,
    sync::{Arc, Mutex},
};
use trpl::StreamExt;

pub struct SmbExplorer {
    tree: Arc<smb::Tree>,
}

impl SmbExplorer {
    pub async fn new(
        path: String,
        username: String,
        password: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new(ClientConfig::default());
        let uncpath: UncPath = UncPath::from_str(&path).unwrap();
        client.share_connect(&uncpath, &username, password).await?;

        let tree = client.get_tree(&uncpath).await?;
        Ok(Self { tree: tree })
    }

    pub async fn fetch_movies(&self) -> Vec<MovieData> {
        let movies: Arc<Mutex<Vec<MovieData>>> = Arc::new(Mutex::new(Vec::new()));

        println!("starting");
        let root_path = "";
        self.explore_path(root_path.to_owned(), Arc::clone(&movies))
            .await
            .unwrap();

        return movies.lock().unwrap().clone();
    }

    fn explore_path<'a>(
        &'a self,
        root_path: String,
        movies: Arc<Mutex<Vec<MovieData>>>,
    ) -> BoxFuture<'a, Result<(), Box<dyn Error>>> {
        async move {
            let access_mask = FileAccessMask::new().with_generic_read(true);
            let resource: Result<Resource, smb::Error> =
                self.tree.open_existing(&root_path, access_mask).await;

            if let Some(dir) = self.is_directory(resource) {
                let dir: Arc<Directory> = Arc::from(dir);

                let data_stream =
                    smb::Directory::query::<FileDirectoryInformation>(&dir, "*").await;

                match data_stream {
                    Ok(data) => self.handle_stream(data, root_path, movies).await?,
                    Err(e) => {
                        println!("err: {}", e);
                        return Ok(());
                    }
                }
            } else {
            }
            Ok(())
        }
        .boxed()
    }

    fn is_directory(&self, resource: Result<Resource, smb::Error>) -> Option<Directory> {
        match resource {
            Ok(res) => {
                if let Resource::Directory(dir) = res {
                    Some(dir)
                } else {
                    None
                }
            }
            Err(e) => {
                println!("Error accessing resource: {}", e);
                None
            }
        }
    }

    async fn handle_stream<'a>(
        &self,
        mut data_stream: QueryDirectoryStream<'a, FileDirectoryInformation>,
        path: String,
        movies: Arc<Mutex<Vec<MovieData>>>,
    ) -> Result<(), Box<dyn Error>> {
        while let Some(entry) = data_stream.next().await {
            match entry {
                Ok(file_info) => {
                    if file_info.file_attributes.directory() {
                        //Parse directory path
                        if file_info.file_name == "." || file_info.file_name == ".." {
                            continue;
                        }

                        let sub_path = if path.is_empty() {
                            file_info.file_name.to_string()
                        } else {
                            format!("{}/{}", path, file_info.file_name)
                        };
                        self.explore_path(sub_path, Arc::clone(&movies)).await?;
                    } else {
                        //parse file path
                        let movie_path = if path.is_empty() {
                            file_info.file_name.to_string()
                        } else {
                            format!("{}/{}", path, file_info.file_name)
                        };
                        if self.is_video_file(&file_info.file_name.to_string())
                            && self.is_not_featurette(&path)
                        {
                            let movie = MovieData::new(movie_path);
                            println!("{}", movie);
                            movies.lock().unwrap().push(movie);
                        }
                    }
                }
                Err(e) => eprintln!("Error retrieving entry: {:?}", e),
            }
        }
        Ok(())
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
