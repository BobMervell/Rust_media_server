use crate::movie_data::movie_data::MovieData;
use futures::{FutureExt, future::BoxFuture};
use smb::{
    Client, ClientConfig, Directory, FileAccessMask, FileDirectoryInformation, Resource, UncPath,
    client, resource::iter_stream::QueryDirectoryStream,
};
use std::{
    error::Error,
    fmt::format,
    str::FromStr,
    sync::{Arc, Mutex},
};
use trpl::StreamExt;

pub struct SmbExplorer {
    client: smb::Client,
    path: String,
    username: String,
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
        Ok(Self {
            client: client,
            path: path,
            username: username,
            tree: tree,
        })
    }

    pub async fn fetch_movies(&self) {
        let movies = Arc::new(Mutex::new(Vec::new()));

        println!("starting");
        let root_path = "";
        self.explore_path(root_path.to_owned(), Arc::clone(&movies))
            .await
            .unwrap();

        for movie in movies.lock().unwrap().iter() {
            println!("{}", movie);
        }
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
        root_path: String,
        movies: Arc<Mutex<Vec<MovieData>>>,
    ) -> Result<(), Box<dyn Error>> {
        while let Some(entry) = data_stream.next().await {
            match entry {
                Ok(file_info) => {
                    if file_info.file_attributes.directory() {
                        if file_info.file_name == "." || file_info.file_name == ".." {
                            continue;
                        }

                        let sub_path = if root_path.is_empty() {
                            file_info.file_name.to_string()
                        } else {
                            format!("{}/{}", root_path, file_info.file_name)
                        };
                        println!("{}", sub_path);
                        // appel récursif boxé
                        self.explore_path(sub_path, Arc::clone(&movies)).await?;
                    } else {
                        let movie_path = if root_path.is_empty() {
                            file_info.file_name.to_string()
                        } else {
                            format!("{}/{}", root_path, file_info.file_name)
                        };
                        movies.lock().unwrap().push(MovieData::new(movie_path));
                    }
                }
                Err(e) => eprintln!("Error retrieving entry: {:?}", e),
            }
        }
        Ok(())
    }
}
// pub fn tree(&self) -> Arc<smb::Tree> {
//     //ne clone pas tree seulement le pointer arc
//     Arc::clone(&self.tree)
// }
// fn get_file_name(&self,file_path:&str)-> &str {
//     if let Some((_folder, name)) = file_path.rsplit_once('/') {
//         println!("Folder name: {}, file name: {}", _folder, name);
//         name
//     } else {
//         file_path
//     }
// }

// fn is_video_file(&self,file_name: &str) -> bool {
//     let video_extensions = ["mp4", "mkv", "avi", "mov", "flv", "wmv", "webm"];

//     if let Some(ext) = file_name.rsplit('.').next() {
//         video_extensions.contains(&ext.to_lowercase().as_str())
//     } else {
//         false
//     }
