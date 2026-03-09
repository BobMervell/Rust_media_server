pub use crate::movie_data::movie_data::MovieSnapshot; //expose for dart
use crate::{
    db_interface::data_getter::DataGetter,
    media_retriever::media_retriever::retrieve_media,
    smb_mounter::smb_mounter::{mount_smb, unmount_smb},
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
    let res = retrieve_media(path, username, password, token).await;
    tracing::info!("Hello, {:?}!", res);
    format!("Hello, {:?}!", res)
}

#[flutter_rust_bridge::frb]
pub fn get_media(media_type: &str) -> Result<Vec<MovieSnapshot>> {
    let data_getter = DataGetter::new("movie_db.db".to_owned())?;
    return data_getter.get_media_snapshot(media_type);
}

#[flutter_rust_bridge::frb]
pub async fn tempo_mount_smb() -> Result<()> {
    mount_smb("user", "passwd", "ip", "folder_path", "mount_point")?;
    Ok(())
}

#[flutter_rust_bridge::frb]
pub fn tempo_unmount_smb() -> Result<()> {
    unmount_smb("/mnt/smb/fluster")?;
    Ok(())
}

#[flutter_rust_bridge::frb]
pub fn open_video(path: &str) -> Result<()> {
    open::that(path).with_context(|| format!("An error occurred when opening {}", path))?;
    Ok(())
}
