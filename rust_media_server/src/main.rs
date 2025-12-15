use smb::{Client, ClientConfig, FileAccessMask, FileAllInformation, Resource, UncPath};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let client = Client::new(ClientConfig::default());
    println!("bonsoir");
    // Connect to a share
    let target_path: UncPath = UncPath::from_str(r"\\server\path").unwrap();
    client.share_connect(&target_path, "name", "le mot de passe".to_string()).await?;
    

    let tree = client.get_tree(&target_path).await?;
    let access_mask = FileAccessMask::new().with_generic_read(true);

    let resource = tree.open_existing("anora (2024)",access_mask).await?;

    match &resource {
        Resource::File(_file) => {
            println!("We have a file");
        }
        Resource::Directory(dir) => {
            println!("We have a dir");
            let output = dir.query_info::<FileAllInformation>().await?;
            let truc= &output.name.file_name;
            println!("{}", truc);
        }
        Resource::Pipe(_pipe) => {
            println!("We have a pipe");
        }
    }
 
    Ok(())
}