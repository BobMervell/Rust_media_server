use crate::{
    db_interface::data_saver::DataSaver,
    directory_explorer::smb_explorer::{SmbExplorer, tempo_smb_connect},
    movie_data::movie_data::{CreditsMovie, MovieData},
    tmdb_client::tmdb_client::TMDBClient,
};
use anyhow::{Context, Result};
use futures::stream::{self, StreamExt};
use tracing::{debug_span, instrument};

#[instrument]
pub async fn retrieve_media() -> Result<()> {
    let smb_explorer: SmbExplorer = tempo_smb_connect()
        .await
        .context("Failed to connect to SMB share")?;

    let mut movies = Box::pin(smb_explorer.fetch_movies(""));

    tracing::info!("Movie retrieval stream started");

    let client = TMDBClient::new().context("Failed to create TMDB client")?;

    let mut data_saver = initiate_db().context("Failed to initiate database")?;

    while let Some(movie) = movies.next().await {
        match movie {
            Ok(mut movie) => {
                let credits = fetch_movie_data(&mut movie, &client).await?;

                update_movie_posters(&mut movie, &client).await?;
                tracing::debug!(file_path = &movie.file_path() , "Movie posters downloaded");

                update_credits_posters(&credits, &client).await?;
                tracing::debug!(file_path = &movie.file_path() , "Credits posters downloaded");

                data_saver.push_movie_data(movie, credits)?; //trace inside
            }
            Err(e) => {
                tracing::error!(" Error finding movie, \n Caused by {}", e)
            }
        }
    }

    tracing::info!("Movie retrieval stream ended");

    //movies.for_each_concurrent(10, fetch_movie_data(movie, &client)); possibke to speed up
    Ok(())
}

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

async fn fetch_movie_data(movie: &mut MovieData, client: &TMDBClient) -> Result<CreditsMovie> {
    let span = debug_span!("fetch_movie_data", movie_path = movie.file_path());
    let _enter = span.enter();

    //try join possible to speed up
    update_movie_basics(movie, client)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update movie basics for {}", movie.file_path());
            e
        })
        .ok();

    update_movie_genres(movie, client)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update movie genre for {}", movie.file_path());
            e
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
            Ok(credits)
        }
        Err(e) => {
            tracing::error!(
                "Failed to update movie credits for {} \n Caused by: {}",
                movie.file_path(),
                e
            );
            Ok(CreditsMovie::new())
        }
    }
    //tracing::debug!(file_path = movie.file_path(), success = true, "Movie data received");
}

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

async fn update_movie_posters(movie: &mut MovieData, client: &TMDBClient) -> Result<()> {
    client.update_movie_backdrop(movie).await;
    client.update_movie_poster(movie).await;
    client.update_movie_poster_snapshot(movie).await;
    Ok(())
}

async fn update_credits_posters(credits: &CreditsMovie, client: &TMDBClient) -> Result<()> {
    update_cast_images(credits, client).await;
    update_crew_images(credits, client).await;
    Ok(())
}

async fn update_cast_images(
    movie_credits: &CreditsMovie,
    client: &TMDBClient,
) -> Vec<(usize, Option<String>)> {
    let cast_indx: Vec<usize> = movie_credits
        .credits_cast()
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect();

    let batch_size = 20;

    let results: Vec<(usize, Option<String>)> = stream::iter(cast_indx)
        .map(|index| {
            let cast = &movie_credits.credits_cast()[index];

            async move {
                let new_path = client.update_cast_images(cast).await;
                (index, new_path)
            }
        })
        .buffer_unordered(batch_size)
        .collect::<Vec<_>>()
        .await;

    return results;
}

async fn update_crew_images(
    movie_credits: &CreditsMovie,
    client: &TMDBClient,
) -> Vec<(usize, Option<String>)> {
    let crew_indx: Vec<usize> = movie_credits
        .credits_crew()
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect();

    let batch_size = 20;

    let results: Vec<(usize, Option<String>)> = stream::iter(crew_indx)
        .map(|index| {
            let crew = &movie_credits.credits_crew()[index];

            async move {
                let new_path = client.update_crew_images(crew).await;
                (index, new_path)
            }
        })
        .buffer_unordered(batch_size)
        .collect::<Vec<_>>()
        .await;

    return results;
}

// region: Filter crew
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
