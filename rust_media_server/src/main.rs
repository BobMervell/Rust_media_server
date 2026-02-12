use rust_media_server::directory_explorer::smb_explorer::SmbExplorer;
use rust_media_server::tmdb_client::tmdb_client::TMDBClient;
use std::{io, u32};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let smb_explorer: SmbExplorer = smb_connect().await?;
    let movies = smb_explorer.fetch_movies().await;

    let client = TMDBClient::new();

    for movie in movies.iter() {
        match client {
            Ok(ref c) => {
                let movie = c
                    .get_movie_info(movie.file_title(), movie.file_year().parse::<u32>().ok())
                    .await;
                match movie {
                    Some(movie) => {
                        println!("{:#?}", movie);
                    }
                    None => {}
                }
            }
            Err(ref e) => {
                println!("error: {}", e);
            }
        }
    }

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
