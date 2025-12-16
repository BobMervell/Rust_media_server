use smb::{Client, ClientConfig, Directory, FileAccessMask, FileAllInformation, FileDirectoryInformation, Resource, UncPath};
use std::{str::FromStr, sync::Arc};
use trpl::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let client = Client::new(ClientConfig::default());
    println!("bonsoir");
    // Connect to a share
    let target_path: UncPath = UncPath::from_str(r"\\server\path").unwrap();
    client.share_connect(&target_path, "username", "mdp".to_string()).await?;
    

    let tree = client.get_tree(&target_path).await?;
    let access_mask = FileAccessMask::new().with_generic_read(true);

    let resource = tree.open_existing("",access_mask).await?;


    match resource {
        Resource::File(_file) => {
            println!("We have a file");
        }
        Resource::Directory(dir) => {
            println!("We have a dir");
            // let output = dir.query_info::<FileAllInformation>().await?;
            // let truc= &output.name.file_name;
            let dir: Arc<Directory> = Arc::new(dir);
            let mut test_stream: smb::resource::iter_stream::QueryDirectoryStream<'_, FileDirectoryInformation> =  smb::Directory::query::<FileDirectoryInformation>(&dir,"*").await?;
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
        }
        Resource::Pipe(_pipe) => {
            println!("We have a pipe");
        }
    }
 
    Ok(())
}