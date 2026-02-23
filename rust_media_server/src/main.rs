use rust_media_server::media_retriever::media_retriever;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    init_tracing_subscriber();

    if let Err(e) = media_retriever::retrieve_media().await {
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
