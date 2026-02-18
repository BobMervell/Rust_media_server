use crate::movie_data::movie_data::MovieData;
use crate::tmdb_client::tmdb_client::{CreditsMovie, TMDBClient};
use futures::stream::{self, StreamExt};

pub async fn update_movie_basics(movie_data: &mut MovieData, client: &TMDBClient) {
    let movie_basics = client
        .get_movie_info(
            movie_data.file_title(),
            movie_data.file_year().parse::<u32>().ok(),
        )
        .await;
    match movie_basics {
        Some(movie_basics) => {
            movie_data
                .set_tmdb_id(movie_basics.id())
                .set_original_title(movie_basics.original_title())
                .set_title(movie_basics.title())
                .set_vote_average(movie_basics.vote_average())
                .set_release_date(movie_basics.release_date())
                .set_summary(movie_basics.overview())
                .set_poster_large(movie_basics.poster_path().to_owned())
                .set_poster_snapshot(movie_basics.poster_path().to_owned())
                .set_backdrop(movie_basics.backdrop_path().to_owned());

            client.update_movie_backdrop(movie_data).await;
            client.update_movie_poster(movie_data).await;
            client.update_movie_poster_snapshot(movie_data).await;
        }
        None => {}
    }
}

pub async fn update_movie_details(movie_data: &mut MovieData, client: &TMDBClient) {
    let movie_details = client.fetch_movie_details(movie_data.tmdb_id()).await;
    match movie_details {
        Ok(movie_details) => {
            movie_data.set_genres(movie_details.genres());
        }
        Err(e) => {
            println!("movie details error: {}", e)
        }
    }
}

pub async fn update_movie_credits(movie_data: &mut MovieData, client: &TMDBClient) {
    let movie_credits = client.fetch_movie_credits(movie_data.tmdb_id()).await;

    match movie_credits {
        Ok(mut movie_credits) => {
            let results = update_cast_images(&movie_credits, client).await;
            for (index, maybe_path) in results {
                if let Some(path) = maybe_path {
                    let cast_mut = &mut movie_credits.credits_cast_mut()[index];
                    cast_mut.set_picture_path(Some(path));
                    movie_data.push_cast(cast_mut.clone());
                }
            }
            let results = update_crew_images(&movie_credits, client).await;
            for (index, maybe_path) in results {
                if let Some(path) = maybe_path {
                    let crew_mut = &mut movie_credits.credits_crew_mut()[index];
                    crew_mut.set_picture_path(Some(path));
                    movie_data.push_crew(crew_mut.clone());
                }
            }
        }
        Err(e) => {
            println!("Credits error: {}", e)
        }
    }
}

async fn update_cast_images(
    movie_credits: &CreditsMovie,
    client: &TMDBClient,
) -> Vec<(usize, Option<String>)> {
    let cast_indx: Vec<usize> = movie_credits
        .credits_cast()
        .iter()
        .enumerate()
        .filter(|(_, cast)| !cast.character().contains("uncredited"))
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
        .filter(|(_, crew)| match crew.department() {
            "Directing" => is_important_directing(crew.job()),
            "Production" => is_important_production(crew.job()),
            "Camera" => is_important_camera(crew.job()),
            "Sound" => is_important_sound(crew.job()),
            "Visual Effects" => is_important_vfx(crew.job()),
            "Writing" => is_important_writing(crew.job()),
            "Art" => is_important_art(crew.job()),
            "Costume & Make-Up" => is_important_costumes_makeup(crew.job()),
            _ => false,
        })
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

// region: Main Roles only
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
