use smb::{Client, ClientConfig, Directory, FileAccessMask, FileDirectoryInformation, Resource, UncPath};
use std::{io, str::FromStr, sync::Arc};
use trpl::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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

    let resource = smb_connect(path, username, passwd).await?;

    match resource {
        Resource::File(_file) => {
            println!("Unvalid  path: {}. This path leads to a file, should be a directory. ", path);
        }
        Resource::Directory(dir) => {
            println!("We have a dir");
            explore_smb_dir(dir).await?;
           
        }
        Resource::Pipe(_pipe) => {
            println!("Unvalid  path: {}. This path leads to a pipe, should be a directory. ", path);
        }
    }

    Ok(())
}


async fn smb_connect(path:&str,username:&str,passwd:&str) ->  Result<Resource, Box<dyn std::error::Error>> {
    let client = Client::new(ClientConfig::default());
    let target_path: UncPath = UncPath::from_str(path).unwrap();
    client.share_connect(&target_path, username, passwd.to_string()).await?;
    

    let tree = client.get_tree(&target_path).await?;
    let access_mask = FileAccessMask::new().with_generic_read(true);

    let resource = tree.open_existing("",access_mask).await?;
    return Ok(resource);
}


async fn explore_smb_dir (root_dir:Directory) ->  Result<(), Box<dyn std::error::Error>> {
    let root_dir: Arc<Directory> = Arc::new(root_dir);
    let mut test_stream=  smb::Directory::query::<FileDirectoryInformation>(&root_dir,"*").await?;
        // Iterate through the entries in the directory using next()
    while let Some(entry) = test_stream.next().await {
        match entry {
            Ok(file_info) => {
                println!("Found file: {:?}", file_info.file_name);
            }
            Err(e) => {
                eprintln!("Error retrieving entry: {:?}", e);
                break; // Exit on error
            }
        }
    }
     Ok(())
}