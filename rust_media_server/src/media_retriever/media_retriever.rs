use std::sync::Arc;

/// Module that orchestrates the media retrieval pipeline.
use crate::{
    db_interface::data_saver::DataSaver,
    directory_explorer::smb_explorer::{SmbExplorer, tempo_smb_connect},
    movie_data::movie_data::{CreditsMovie, MovieData},
    tmdb_client::tmdb_client::TMDBClient,
};
use anyhow::{Context, Error, Result};
use futures::stream::{self, StreamExt};
use tokio::sync::Mutex;
use tracing::{debug_span, instrument};
use trpl::Stream;

/// Runs the primary streaming pipeline for media retrieval.
///
/// Discovers media paths, fetches movie metadata, credits, and posters from TMDB,
/// then persists the collected data and associated poster assets in order.
#[instrument]
pub async fn retrieve_media() -> Result<()> {
    let smb_explorer: SmbExplorer = tempo_smb_connect()
        .await
        .context("Failed to connect to SMB share")?;

    let client = TMDBClient::new().context("Failed to create TMDB client")?;

    let movies = smb_explorer.fetch_movies("");

    tracing::info!("Movie retrieval stream started");

    let data_saver = Arc::new(Mutex::new(
        initiate_db().context("Failed to initiate database")?,
    ));

    handle_found_movies(movies, &client, data_saver).await;

    tracing::info!("Movie retrieval stream ended");
    Ok(())
}

/// Initializes the database by creating the database file and required tables.
fn initiate_db() -> Result<DataSaver> {
    let mut data_saver = DataSaver::new("movie_db.db".to_string())
        .context("Failed to create database connection")?;
    data_saver.create_movie_table()?;
    data_saver.create_person_table()?;
    data_saver.create_genre_table()?;
    data_saver.create_movie_genre_table()?;

    tracing::info!("Data base initiated");
    Ok(data_saver)
}

// region: ---- UPDATE MOVIE DATA ----

/// Wrapper for the concurent movie handling pipeline 
async fn handle_found_movies(
    movies: impl Stream<Item = Result<MovieData, Error>>,
    client: &TMDBClient,
    data_saver: Arc<Mutex<DataSaver>>,
) {
    // let movies = Box::pin(movies);
    movies
        .for_each_concurrent(10, |movie| {
            let data_saver = Arc::clone(&data_saver);
            let client = client;
            async move {
                match movie {
                    Ok(mut movie) => {
                        let mut credits = fetch_movie_data(&mut movie, &client).await;
                        update_movie_posters(&mut movie, &client).await;
                        update_credits_posters(&mut credits, &client, &movie.file_path()).await;

                        let mut ds = data_saver.lock().await;
                        ds.push_movie_data(&movie, &credits)
                            .map_err(|e| {
                                tracing::error!(
                                    "Failed to push movie data for {} \n Caused by {:?}",
                                    movie.file_path(),
                                    e
                                );
                            })
                            .ok();
                    }
                    Err(e) => {
                        tracing::error!(" Error finding movie, \n Caused by {:?}", e)
                    }
                }
            }
        })
        .await;
}

/// Fetches movie metadata, including basic information, genres, and credits.
async fn fetch_movie_data(movie: &mut MovieData, client: &TMDBClient) -> CreditsMovie {
    let span = debug_span!("fetch_movie_data", movie_path = movie.file_path());
    let _enter = span.enter();

    update_movie_basics(movie, client)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to update movie basics for {} \n Caused by {:?}",
                movie.file_path(),
                e
            );
        })
        .ok();

    update_movie_genres(movie, client)
        .await
        .map_err(|e| {
            tracing::error!(
                "Failed to update movie genre for {} \n Caused by {:?}",
                movie.file_path(),
                e
            );
        })
        .ok();

    tracing::debug!(
        file_path = movie.file_path(),
        success = true,
        "Movie data received"
    );

    match get_movie_credits(movie, client).await {
        Ok(mut credits) => {
            filter_credits(&mut credits);
            tracing::debug!(
                file_path = movie.file_path(),
                success = true,
                "Movie credits received"
            );
            credits
        }
        Err(e) => {
            tracing::error!(
                "Failed to update movie credits for {} \n Caused by: {:?}",
                movie.file_path(),
                e
            );
            CreditsMovie::new()
        }
    }
}

/// Retrieves and updates the basic metadata for a movie.
async fn update_movie_basics(movie: &mut MovieData, client: &TMDBClient) -> Result<()> {
    let movie_basics = client
        .get_movie_info(movie.file_title(), movie.file_year().parse::<u32>().ok())
        .await
        .with_context(|| {
            format!(
                "Failed to get movie basic info for file: {}",
                movie.file_path()
            )
        })?;

    movie
        .set_tmdb_id(movie_basics.id())
        .set_original_title(movie_basics.original_title())
        .set_title(movie_basics.title())
        .set_vote_average(movie_basics.vote_average())
        .set_release_date(movie_basics.release_date())
        .set_summary(movie_basics.overview())
        .set_poster_large(movie_basics.poster_path().to_owned())
        .set_poster_snapshot(movie_basics.poster_path().to_owned())
        .set_backdrop(movie_basics.backdrop_path().to_owned());

    Ok(())
}

/// Retrieves and updates the genres for a movie.
async fn update_movie_genres(movie: &mut MovieData, client: &TMDBClient) -> Result<()> {
    movie.set_genres(
        client
            .fetch_movie_genres(movie.tmdb_id())
            .await
            .with_context(|| {
                format!(
                    "Failed to get movie genres info for file: {}",
                    movie.file_path()
                )
            })?
            .genres(),
    );
    Ok(())
}

/// Retrieves and updates the credits for a movie.
async fn get_movie_credits(movie: &mut MovieData, client: &TMDBClient) -> Result<CreditsMovie> {
    let movie_credits = client
        .fetch_movie_credits(movie.tmdb_id())
        .await
        .with_context(|| {
            format!(
                "Failed to get movie credits info for file: {}",
                movie.file_path()
            )
        })?;
    Ok(movie_credits)
}
// endregion

// region: ---- UPDATE IMAGES ----

/// Downloads movie poster, snapshot and backdrop, updating their file paths.
async fn update_movie_posters(movie: &mut MovieData, client: &TMDBClient) {
    match client.update_movie_backdrop(movie).await {
        Ok(snapshot_path) => {
            movie.set_backdrop(Some(snapshot_path));
        }
        Err(e) => {
            tracing::error!(
                "Failed to update movie backdrop for {} \n Casued by {:?}",
                movie.file_path(),
                e
            )
        }
    }

    match client.update_movie_poster(movie).await {
        Ok(snapshot_path) => {
            movie.set_poster_large(Some(snapshot_path));
        }
        Err(e) => {
            tracing::error!(
                "Failed to update movie poster for {} \n Caused by {:?}",
                movie.file_path(),
                e
            )
        }
    }

    match client.update_movie_poster_snapshot(movie).await {
        Ok(snapshot_path) => {
            movie.set_poster_snapshot(Some(snapshot_path));
        }
        Err(e) => {
            tracing::error!(
                "Failed to update movie snapshot for {} \n Caused by {:?}",
                movie.file_path(),
                e
            )
        }
    }

    tracing::debug!(file_path = &movie.file_path(), "Movie posters downloaded")
}

/// Downloads credit profile picture,and set their file paths.
async fn update_credits_posters(credits: &mut CreditsMovie, client: &TMDBClient, movie_path: &str) {
    if let Err(e) = update_cast_images(credits, client).await {
        tracing::error!(
            "Failed to update cast images for {} \n Caused by {:?}",
            movie_path,
            e
        )
    }

    if let Err(e) = update_crew_images(credits, client).await {
        tracing::error!(
            "Failed to update cast images for: {} \n Caused by {:?}",
            movie_path,
            e
        )
    }

    tracing::debug!(file_path = movie_path, "Credits posters downloaded");
}

/// Downloads cast profile picture,and set their file paths.
async fn update_cast_images(movie_credits: &mut CreditsMovie, client: &TMDBClient) -> Result<()> {
    let batch_size = 20;
    let casts = movie_credits.credits_cast();

    let updates = stream::iter(casts.into_iter().enumerate())
        .map(|(index, cast)| async move { (index, client.update_cast_images(&cast).await) })
        .buffer_unordered(batch_size)
        .collect::<Vec<_>>()
        .await;

    for (index, result) in updates {
        if let Ok(path) = result {
            movie_credits
                .set_cast_image(index, &path)
                .map_err(|e| {
                    tracing::error!(
                        "Failed to set cast image for path: {} \n Caused by {:?}",
                        path,
                        e
                    );
                })
                .ok();
        }
    }
    Ok(())
}

/// Downloads crew profile picture,and set their file paths.
async fn update_crew_images(movie_credits: &mut CreditsMovie, client: &TMDBClient) -> Result<()> {
    let batch_size = 20;
    let crews = movie_credits.credits_crew();

    let updates = stream::iter(crews.into_iter().enumerate())
        .map(|(index, crew)| async move { (index, client.update_crew_images(&crew).await) })
        .buffer_unordered(batch_size)
        .collect::<Vec<_>>()
        .await;

    for (index, result) in updates {
        if let Ok(path) = result {
            movie_credits
                .set_crew_image(index, &path)
                .map_err(|e| {
                    tracing::error!(
                        "Failed to set crew image for path: {} \n Caused by {:?}",
                        path,
                        e
                    );
                })
                .ok();
        }
    }
    Ok(())
}
// endregion

// region: ---- FILTER CREDITS ----

/// Filters movie credits to retain only credited cast members and principal crew.
fn filter_credits(credits: &mut CreditsMovie) {
    let casts = credits.credits_cast_mut();
    casts.retain(|cast| !cast.character().contains("uncredited"));

    let crew = credits.credits_crew_mut();
    crew.retain(|crew| !match crew.department() {
        "Directing" => is_important_directing(crew.job()),
        "Production" => is_important_production(crew.job()),
        "Camera" => is_important_camera(crew.job()),
        "Sound" => is_important_sound(crew.job()),
        "Visual Effects" => is_important_vfx(crew.job()),
        "Writing" => is_important_writing(crew.job()),
        "Art" => is_important_art(crew.job()),
        "Costume & Make-Up" => is_important_costumes_makeup(crew.job()),
        _ => false,
    });
}

fn is_important_directing(job: &str) -> bool {
    match job {
        "Director" => true,
        "Co-Director" => true,
        _ => false,
    }
}

fn is_important_production(job: &str) -> bool {
    match job {
        "Producer" => true,
        _ => false,
    }
}

fn is_important_camera(job: &str) -> bool {
    match job {
        "Director of Photography" => true,
        _ => false,
    }
}

fn is_important_sound(job: &str) -> bool {
    match job {
        "Original Music Composer" => true,
        "Sound Designer" => true,
        _ => false,
    }
}

fn is_important_vfx(job: &str) -> bool {
    match job {
        "VFX Supervisor" => true,
        "Visual Effects Supervisor" => true,
        "Visual Effects Art Director" => true,
        _ => false,
    }
}

fn is_important_writing(job: &str) -> bool {
    match job {
        "Writer" => true,
        "Original Film Writer" => true,
        "Co-Writer" => true,
        "Scenario Writer" => true,
        "Teleplay" => true,
        "Screenplay" => true,
        _ => false,
    }
}

fn is_important_art(job: &str) -> bool {
    match job {
        "Art Direction" => true,
        "Co-Art Director" => true,
        "Production Design" => true,
        "Art Designer" => true,
        "Set Designer" => true,
        "Property Master" => true,
        _ => false,
    }
}

fn is_important_costumes_makeup(job: &str) -> bool {
    match job {
        "Costume Designer" => true,
        "Makeup Designer" => true,
        _ => false,
    }
}
// endregion
