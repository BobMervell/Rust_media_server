use crate::movie_data::movie_data::MovieData;
use crate::tmdb_client::tmdb_client::TMDBClient;

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
            println!("test {}", e)
        }
    }
}

pub async fn update_movie_credits(movie_data: &mut MovieData, client: &TMDBClient) {
    let movie_credits = client.fetch_movie_credits(movie_data.tmdb_id()).await;

    match movie_credits {
        Ok(mut movie_credits) => {
            for mut cast in movie_credits.credits_cast_mut().iter_mut() {
                if !cast.character().contains("uncredited") {
                    client.update_cast_images(&mut cast).await;
                    movie_data.push_cast(cast.clone());
                }
            }
            for mut crew in movie_credits.credits_crew_mut().iter_mut() {
                match crew.department() {
                    "Directing" => {
                        if is_important_directing(crew.job()) {
                            client.update_crew_images(&mut crew).await;
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Production" => {
                        if is_important_production(crew.job()) {
                            client.update_crew_images(&mut crew).await;
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Camera" => {
                        if is_important_camera(crew.job()) {
                            client.update_crew_images(&mut crew).await;
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Sound" => {
                        if is_important_sound(crew.job()) {
                            client.update_crew_images(&mut crew).await;
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Visual Effects" => {
                        if is_important_vfx(crew.job()) {
                            client.update_crew_images(&mut crew).await;
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Writing" => {
                        if is_important_writing(crew.job()) {
                            client.update_crew_images(&mut crew).await;
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Art" => {
                        if is_important_art(crew.job()) {
                            client.update_crew_images(&mut crew).await;
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Costume & Make-Up" => {
                        if is_important_costumes_makeup(crew.job()) {
                            client.update_crew_images(&mut crew).await;
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            println!("Credits error: {}", e)
        }
    }
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
