pub use crate::movie_data::movie_data::MovieSnapshot; //expose for dart
use crate::{
    db_interface::data_getter::DataGetter, media_retriever::media_retriever::retrieve_media,
};
use anyhow::Result;
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
    let res = retrieve_media(path, username, password, token).await;
    format!("Hello, {:?}!", res)
}

#[flutter_rust_bridge::frb]
pub fn get_media(media_type: &str) -> Result<Vec<MovieSnapshot>> {
    let data_getter = DataGetter::new("movie_db.db".to_owned())?;
    return data_getter.get_media_snapshot(media_type);
}
