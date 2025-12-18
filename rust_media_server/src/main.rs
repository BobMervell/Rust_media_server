use futures::{FutureExt, future::BoxFuture};
use smb::{Client, ClientConfig, Directory, FileAccessMask, FileAllInformation, FileDirectoryInformation, Resource, UncPath};
use std::{error::Error, fmt::format, io, pin::Pin, str::FromStr, sync::Arc};
use trpl::StreamExt;


struct ClientInfo {
    client: smb::Client,
    path: UncPath,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    

    let client_info = smb_connect().await?;
    let tree: Arc<smb::Tree> = client_info.client.get_tree(&client_info.path).await?;

    explore_smb_dir(tree,"").await;

    Ok(())
}


async fn smb_connect() ->  Result<ClientInfo, Box<dyn std::error::Error>> {

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
    let passwd=passwd.trim_end();

    let client = Client::new(ClientConfig::default());
    let path: UncPath = UncPath::from_str(path).unwrap();
    client.share_connect(&path, username, passwd.to_string()).await?;
    Ok(ClientInfo {client,path})
    
}


fn explore_smb_dir <'a>(tree:Arc<smb::Tree>, root_path:&'a str) -> BoxFuture<'a, Result<String, Box<dyn Error>>> {
    Box::pin(async move {

    let access_mask = FileAccessMask::new().with_generic_read(true);
    let resource = tree.open_existing(root_path,access_mask).await?;
    println!("yes");
    match resource {
        Resource::File(file) => {
            let file_info = file.query_info::<FileAllInformation>().await?;
            println!("Well hello there !!!! {:?}", file_info.name);
            Ok (file_info.name.to_string())
        }
        Resource::Pipe(_pipe) => {
            let test = format!("The path « {} » is not a directory but a pipe", root_path);
            println!("{}",test);
            Ok(test)
       
        }
        Resource::Directory(dir) => {
            let dir: Arc<Directory> = Arc::from(dir);
            let mut data_stream=  smb::Directory::query::<FileDirectoryInformation>(&dir,"*").await?;
            while let Some(entry) = data_stream.next().await {
                match entry {
                    Ok(file_info) => {
                        if file_info.file_attributes.directory() {
                            let mut sub_path = file_info.file_name.to_string();
                            if root_path != "" {
                                sub_path = format!("{}/{}",root_path,sub_path);
                            }
                             if sub_path.ends_with("/.") || sub_path.ends_with("/..") || sub_path == "." || sub_path == ".." {
                                continue;
                            }
                            println!("{}", sub_path);
                            explore_smb_dir(Arc::clone(&tree),&sub_path).await?;

                        }
                        else if !file_info.file_attributes.directory() {
                            println!("{}", file_info.file_name)
                        }
                    }
                    Err(e) => {
                        eprintln!("Error retrieving entry: {:?}", e);
                        break; // Exit on error
                    }
                }
            }
            let test = format!("The path « {} » is not a directory but a pipe", root_path);
            Ok(test)
        }
    }
})
}


