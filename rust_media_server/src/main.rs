use smb::{Client, ClientConfig, Directory, FileAccessMask, FileAttributes, FileDirectoryInformation, Resource, UncPath};
use std::{io, str::FromStr, sync::Arc};
use trpl::StreamExt;


struct ClientInfo {
    client: smb::Client,
    path: UncPath,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    

    let client_info = smb_connect().await?;
    let tree: Arc<smb::Tree> = client_info.client.get_tree(&client_info.path).await?;

    let path = client_info.path.to_string();
    explore_smb_dir(tree,&path).await?;

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


async fn explore_smb_dir (tree:Arc<smb::Tree>, root_path:&str) ->  Result<(), Box<dyn std::error::Error>> {
    let root_path = "";
    let access_mask = FileAccessMask::new().with_generic_read(true);
    println!("no");

    let resource = tree.open_existing("",access_mask).await?;
    println!("yes");
    match resource {
        Resource::File(_file) => {
            Err(format!("The path « {} » is not a directory but a file", root_path).into())
        }
        Resource::Pipe(_pipe) => {
            Err(format!("The path « {} » is not a directory but a pipe", root_path).into())
        }
        Resource::Directory(dir) => {
            let dir: Arc<Directory> = Arc::from(dir);
            let mut data_stream=  smb::Directory::query::<FileDirectoryInformation>(&dir,"*").await?;
            while let Some(entry) = data_stream.next().await {
                match entry {
                    Ok(file_info) => {
                        if file_info.file_attributes.directory() {
                            let sub_path = file_info.file_name.to_string();
                             if sub_path == "." || sub_path == ".." {
                                continue;
                            }
                            println!("{}", sub_path);
                            let resource = tree.open_existing(&sub_path,access_mask).await?;
                            match resource {
                                Resource::File(_file) => {
                                    println!("Unvalid  path: {}. This path leads to a file, should be a directory. ", sub_path);
                                }
                                Resource::Directory(dir) => {
                                    println!("We have a dir {}",sub_path);     
                                }
                                Resource::Pipe(_pipe) => {
                                    println!("Unvalid  path: {}. This path leads to a pipe, should be a directory. ", sub_path);
                                }
                            }

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
            Ok(())
        }
    }
}

