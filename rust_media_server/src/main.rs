use futures::{FutureExt, future::BoxFuture};
use smb::{
    Client, ClientConfig, Directory, FileAccessMask, FileDirectoryInformation, Resource, UncPath,
};
use std::{error::Error, fmt, io, str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use trpl::StreamExt;

struct ClientInfo {
    client: smb::Client,
    path: UncPath,
}
#[derive(Debug)]
struct MovieData {
    file_name: String,
    path: String,
}
impl fmt::Display for MovieData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.file_name, self.path)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_info = smb_connect().await?;
    let tree: Arc<smb::Tree> = client_info.client.get_tree(&client_info.path).await?;
    let out = Arc::new(Mutex::new(Vec::new()));

    scrap_smb_dir(tree, "", Arc::clone(&out)).await?;

    let result = out.lock().await;
    //println!("{:#?}", &*result);

    Ok(())
}

async fn smb_connect() -> Result<ClientInfo, Box<dyn std::error::Error>> {
    let mut path = String::new();
    println!("Enter the samba remote path");
    io::stdin().read_line(&mut path)?;
    let path = path.trim_end();

    let mut username = String::new();
    println!("Enter the username");
    io::stdin().read_line(&mut username)?;
    let username = username.trim_end();

    let mut passwd = String::new();
    println!("Enter the passord");
    io::stdin().read_line(&mut passwd)?;
    let passwd = passwd.trim_end();

    let client = Client::new(ClientConfig::default());
    let path: UncPath = UncPath::from_str(path).unwrap();
    client
        .share_connect(&path, username, passwd.to_string())
        .await?;
    Ok(ClientInfo { client, path })
}

fn scrap_smb_dir<'a>(
    tree: Arc<smb::Tree>,
    root_path: &'a str,
    out: Arc<Mutex<Vec<MovieData>>>,
) -> BoxFuture<'a, Result<(), Box<dyn Error>>> {
    async move {
        let access_mask = FileAccessMask::new().with_generic_read(true);
        let resource = tree.open_existing(root_path, access_mask).await?;

        let dir = match resource {
            Resource::Directory(d) => d,
            other => {
                let kind = match other {
                    Resource::File(_) => "file",
                    Resource::Pipe(_) => "pipe",
                    _ => "unknown",
                };
                return Err(format!(
                    "The path « {} » is not a directory but a {}",
                    root_path, kind
                )
                .into());
            }
        };
        let dir: Arc<Directory> = Arc::from(dir);

        let mut data_stream = smb::Directory::query::<FileDirectoryInformation>(&dir, "*").await?;
        while let Some(entry) = data_stream.next().await {
            match entry {
                Ok(file_info) => {
                    if file_info.file_attributes.directory() {
                        let mut sub_path = file_info.file_name.to_string();
                        if !root_path.is_empty() {
                            sub_path = format!("{}/{}", root_path, sub_path);
                        }
                        if sub_path.ends_with("/.")
                            || sub_path.ends_with("/..")
                            || sub_path == "."
                            || sub_path == ".."
                        {
                            continue;
                        }
                        // appel récursif en passant le même `out`
                        scrap_smb_dir(Arc::clone(&tree), &sub_path, Arc::clone(&out)).await?;
                    } else {
                        let movie_path = &format!("{}/{}", root_path, file_info.file_name);
                        let moviedata = create_movie_data(movie_path);
                        let mut guard = out.lock().await;
                        guard.push(moviedata);
                    }
                }
                Err(e) => {
                    eprintln!("Error retrieving entry: {:?}", e);
                    break;
                }
            }
        }

        Ok(())
    }
    .boxed()
}

fn create_movie_data(movie_path: &str) -> MovieData {
    let mut movie_data = MovieData {
        file_name: "".to_owned(),
        path: movie_path.to_owned(),
    };
    if let Some((test, name)) = movie_path.rsplit_once('/') {
        println!("Folder name: {}, file name: {}", test, name);
        movie_data.file_name = name.to_owned()
    }
    movie_data
}
