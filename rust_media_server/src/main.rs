use rust_media_server::data_parser::api_response_parser::{
    update_movie_basics, update_movie_credits, update_movie_details,
};
use rust_media_server::{
    db_interface::data_saver::DataSaver, directory_explorer::smb_explorer::SmbExplorer,
    directory_explorer::smb_explorer::tempo_smb_connect, movie_data::movie_data::MovieData,
    tmdb_client::tmdb_client::TMDBClient,
};

use anyhow::{Context, Result};
use tracing::instrument;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    init_tracing_subscriber();

    if let Err(e) = retrieve_media().await {
        tracing::error!(error = ?e, "Application failed");
        std::process::exit(1);
    }
}

// Basic function that creates a default tracing subscribe that outputs only in the console
fn init_tracing_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .init();

    tracing_log::LogTracer::init().ok();
}

#[instrument]
async fn retrieve_media() -> Result<()> {
    let smb_explorer: SmbExplorer = tempo_smb_connect()
        .await
        .context("Failed to connect to SMB share")?;
    let mut movies: Vec<MovieData> = smb_explorer
        .fetch_movies()
        .await
        .context("Failed to retrieve movie list")?;

    let client = TMDBClient::new().context("Failed to create TMDB client")?;
    // let movie_test = MovieData::new("la la land (2016)/la la land (2016).mp4".to_string());
    // let mut movies: Vec<MovieData> = Vec::new();
    // movies.push(movie_test);

    let mut data_saver = DataSaver::new("movie_db.db".to_string())
        .context("Failed to create database connection")?;
    data_saver.create_movie_table();
    data_saver.create_person_table();
    data_saver.create_genre_table();
    data_saver.create_movie_genre_table();

    for movie_data in movies.iter_mut() {
        update_movie_basics(movie_data, &client).await;
        update_movie_details(movie_data, &client).await;
        update_movie_credits(movie_data, &client).await;
        data_saver.push_movie(movie_data.clone());
    }
    Ok(())
}
