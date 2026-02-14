use rust_media_server::data_parser::api_response_parser::{
    update_movie_basics, update_movie_credits, update_movie_details,
};
use rust_media_server::db_interface::data_saver::DataSaver;
use rust_media_server::tmdb_client::tmdb_client::TMDBClient;
use rust_media_server::{
    directory_explorer::smb_explorer::SmbExplorer, movie_data::movie_data::MovieData,
};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let smb_explorer: SmbExplorer = smb_connect().await?;
    // let mut movies: Vec<MovieData> = smb_explorer.fetch_movies().await;

    let client = TMDBClient::new();
    let movie_test = MovieData::new("la la land (2016)/la la land (2016).mp4".to_string());
    let mut movies: Vec<MovieData> = Vec::new();
    movies.push(movie_test);

    let data_saver = DataSaver::new("movie_db.db".to_string());
    match client {
        Ok(ref client) => match data_saver {
            Ok(mut data_saver) => {
                for movie_data in movies.iter_mut() {
                    update_movie_basics(movie_data, client).await;
                    update_movie_details(movie_data, client).await;
                    update_movie_credits(movie_data, client).await;
                    println!("{}", movie_data);
                    data_saver.create_movie_table();
                    data_saver.push_movie(movie_data.clone())
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        },
        Err(ref e) => {
            println!("error: {}", e);
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
