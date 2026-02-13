use rust_media_server::tmdb_client::tmdb_client::TMDBClient;
use rust_media_server::{
    directory_explorer::smb_explorer::SmbExplorer, movie_data::movie_data::MovieData,
};
use std::{io, u32};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let smb_explorer: SmbExplorer = smb_connect().await?;
    // let mut movies: Vec<MovieData> = smb_explorer.fetch_movies().await;

    let client = TMDBClient::new();
    let movie_test = MovieData::new("/iron man (2008) [test].mp4".to_string());
    let mut movies: Vec<MovieData> = Vec::new();
    movies.push(movie_test);

    match client {
        Ok(ref client) => {
            for movie_data in movies.iter_mut() {
                println!("a");
                update_movie_basics(movie_data, client).await;
                println!("b");
                update_movie_details(movie_data, client).await;
                println!("c: {}", movie_data);
            }
        }
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

async fn update_movie_basics(movie_data: &mut MovieData, client: &TMDBClient) {
    let movie_basics = client
        .get_movie_info(
            movie_data.file_title(),
            movie_data.file_year().parse::<u32>().ok(),
        )
        .await;
    match movie_basics {
        Some(movie_basics) => {
            movie_data
                .set_id(movie_basics.id())
                .set_original_title(movie_basics.original_title())
                .set_title(movie_basics.title())
                .set_vote_average(movie_basics.vote_average())
                .set_release_date(movie_basics.release_date())
                .set_sumary(movie_basics.overview());
        }
        None => {}
    }
}

async fn update_movie_details(movie_data: &mut MovieData, client: &TMDBClient) {
    let movie_details = client.fetch_movie_details(*movie_data.id()).await;

    match movie_details {
        Ok(movie_details) => {
            let genres = movie_details
                .genres()
                .into_iter()
                .map(|g| g.name())
                .collect();
            movie_data.set_genres(genres);
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}
