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
                .set_id(movie_basics.id())
                .set_original_title(movie_basics.original_title())
                .set_title(movie_basics.title())
                .set_vote_average(movie_basics.vote_average())
                .set_release_date(movie_basics.release_date())
                .set_summary(movie_basics.overview());
        }
        None => {}
    }
}

pub async fn update_movie_details(movie_data: &mut MovieData, client: &TMDBClient) {
    let movie_details = client.fetch_movie_details(movie_data.id()).await;
    match movie_details {
        Ok(movie_details) => {
            movie_data.set_genres(movie_details.genres());
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}

pub async fn update_movie_credits(movie_data: &mut MovieData, client: &TMDBClient) {
    let movie_credits = client.fetch_movie_credits(movie_data.id()).await;

    match movie_credits {
        Ok(movie_credits) => {
            for cast in movie_credits.credits_cast().iter() {
                if !cast.character().contains("uncredited") {
                    movie_data.push_cast(cast.clone());
                }
            }
            for crew in movie_credits.credits_crew().iter() {
                match crew.department() {
                    "Directing" => {
                        if is_important_directing(crew.job()) {
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Production" => {
                        if is_important_production(crew.job()) {
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Camera" => {
                        if is_important_camera(crew.job()) {
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Sound" => {
                        if is_important_sound(crew.job()) {
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Visual Effects" => {
                        if is_important_vfx(crew.job()) {
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Writing" => {
                        if is_important_writing(crew.job()) {
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Art" => {
                        if is_important_art(crew.job()) {
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    "Costume & Make-Up" => {
                        if is_important_costumes_makeup(crew.job()) {
                            movie_data.push_crew(crew.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            println!("{}", e)
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
