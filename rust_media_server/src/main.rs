use futures::{FutureExt, future::BoxFuture};
use rust_media_server::directory_explorer::smb_explorer::SmbExplorer;
use rust_media_server::movie_data::movie_data::MovieData;
use smb::{
    Client, ClientConfig, Directory, FileAccessMask, FileDirectoryInformation, Resource, UncPath,
};
use std::{error::Error, fmt, io, str::FromStr, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let smb_explorer: SmbExplorer = smb_connect().await?;
    smb_explorer.fetch_movies().await;

    // let tree: Arc<smb::Tree> = smb_explorer.tree();
    // let out = Arc::new(Mutex::new(Vec::new()));

    // scrap_smb_dir(tree, "", Arc::clone(&out)).await?;

    // let result = out.lock().await;
    // //println!("{:#?}", &*result);

    Ok(())
}

async fn smb_connect() -> Result<SmbExplorer, Box<dyn std::error::Error>> {
    let mut path = String::new();
    println!("Enter the samba remote path");
    io::stdin().read_line(&mut path)?;
    let path = path.trim_end();

    let mut username = String::new();
    println!("Enter the username");
    io::stdin().read_line(&mut username)?;
    let username = username.trim_end();

    let mut password = String::new();
    println!("Enter the passord");
    io::stdin().read_line(&mut password)?;
    let password = password.trim_end();

    SmbExplorer::new(path.to_owned(), username.to_owned(), password.to_owned()).await
}

// fn scrap_smb_dir<'a>(
//     tree: Arc<smb::Tree>,
//     root_path: &'a str,
//     out: Arc<Mutex<Vec<MovieData>>>,
// ) -> BoxFuture<'a, Result<(), Box<dyn Error>>> {
//     async move {
//         let access_mask = FileAccessMask::new().with_generic_read(true);
//         let resource = tree.open_existing(root_path, access_mask).await?;

//         let dir = match resource {
//             Resource::Directory(d) => d,
//             other => {
//                 let kind = match other {
//                     Resource::File(_) => "file",
//                     Resource::Pipe(_) => "pipe",
//                     _ => "unknown",
//                 };
//                 return Err(format!(
//                     "The path « {} » is not a directory but a {}",
//                     root_path, kind
//                 )
//                 .into());
//             }
//         };
//         let dir: Arc<Directory> = Arc::from(dir);

//         let mut data_stream = smb::Directory::query::<FileDirectoryInformation>(&dir, "*").await?;
//         while let Some(entry) = data_stream.next().await {
//             match entry {
//                 Ok(file_info) => {
//                     if file_info.file_attributes.directory() {
//                         let mut sub_path = file_info.file_name.to_string();
//                         if !root_path.is_empty() {
//                             sub_path = format!("{}/{}", root_path, sub_path);
//                         }
//                         if sub_path.ends_with("/.")
//                             || sub_path.ends_with("/..")
//                             || sub_path == "."
//                             || sub_path == ".."
//                         {
//                             continue;
//                         }
//                         // appel récursif en passant le même `out`
//                         scrap_smb_dir(Arc::clone(&tree), &sub_path, Arc::clone(&out)).await?;
//                     } else {
//                         let movie_path = &format!("{}/{}", root_path, file_info.file_name);
//                         let moviedata = MovieData::new(movie_path.to_owned());
//                         let mut guard = out.lock().await;
//                         guard.push(moviedata);
//                     }
//                 }
//                 Err(e) => {
//                     eprintln!("Error retrieving entry: {:?}", e);
//                     break;
//                 }
//             }
//         }

//         Ok(())
//     }
//     .boxed()
// }
