pub use crate::{
    domain::person::person_data::PersonData,
    movie_data::movie_data::{MediaData, MovieSnapshot},
}; //expose for dart

use crate::{
    application::systems::movie_ingestion_service::MovieIngestionService,
    db_interface::data_getter::DataGetter,
    domain::services::movie_parser::MovieNameParser,
    infrastructure::{
        db_infra::sqlite_data_saver::SqliteDataSaver,
        file_explorers_infra::smb_explorer::SmbExplorer,
        tmdb_api_infra::{
            tmdb_movie_details::TMDBMoviesDetailsFetcher,
            tmdb_movie_posters::TMDBMoviesImagesFetcher,
        },
    },
    movie_data::movie_data::PersonSnapshot,
};
use anyhow::{Context, Result};

use tracing_subscriber::fmt::format::FmtSpan;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    init_tracing_subscriber();
    flutter_rust_bridge::setup_default_user_utils();
}

// Basic function that creates a default tracing subscribe that outputs only in the console
fn init_tracing_subscriber() {
    std::env::set_var("RUST_LIB_BACKTRACE", "0");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .init();

    tracing_log::LogTracer::init().ok();
}

#[flutter_rust_bridge::frb]
pub async fn start(path: &str, username: &str, password: &str, token: &str) -> String {
    println!("startinf");
    let explorer = SmbExplorer::new(path.to_owned(), username.to_owned(), password.to_owned())
        .await
        .unwrap();
    let parser = MovieNameParser {};
    let details_fetcher = TMDBMoviesDetailsFetcher::new(token).unwrap();
    let image_fetcher = TMDBMoviesImagesFetcher::new(token).unwrap();
    let saver = SqliteDataSaver::new().unwrap();
    let mut test =
        MovieIngestionService::new(explorer, parser, details_fetcher, image_fetcher, saver);
    let truc = test.ingest_movies().await;
    format!("Hello!")
}

#[flutter_rust_bridge::frb]
pub fn get_media_snapshots(media_type: &str) -> Result<Vec<MovieSnapshot>> {
    let data_getter = DataGetter::new("movie_db.db".to_owned())?;
    return data_getter.get_media_snapshot(media_type);
}

#[flutter_rust_bridge::frb]
pub fn get_media(media_id: i64) -> Result<MediaData> {
    let data_getter = DataGetter::new("movie_db.db".to_owned())?;
    return data_getter.get_media_data(media_id);
}

#[flutter_rust_bridge::frb]
pub fn get_media_cast(media_id: i64) -> Result<Vec<PersonSnapshot>> {
    let data_getter = DataGetter::new("movie_db.db".to_owned())?;
    return data_getter.get_media_cast(media_id);
}

#[flutter_rust_bridge::frb]
pub fn get_media_crew(media_id: i64) -> Result<Vec<PersonSnapshot>> {
    let data_getter = DataGetter::new("movie_db.db".to_owned())?;
    return data_getter.get_media_crew(media_id);
}

#[flutter_rust_bridge::frb]
pub fn get_person(person_tmdb_id: i64) -> Result<PersonData> {
    let data_getter = DataGetter::new("movie_db.db".to_owned())?;
    return data_getter.get_person_data(person_tmdb_id);
}

#[flutter_rust_bridge::frb]
pub async fn tempo_mount_smb() -> Result<()> {
    // mount_smb("user", "passwd", "ip", "folder_path", "mount_point")?;
    Ok(())
}

#[flutter_rust_bridge::frb]
pub fn tempo_unmount_smb() -> Result<()> {
    // unmount_smb("/mnt/smb/fluster")?;
    Ok(())
}

#[flutter_rust_bridge::frb]
pub fn open_video(path: &str) -> Result<()> {
    open::that(path).with_context(|| format!("An error occurred when opening {}", path))?;
    Ok(())
}
