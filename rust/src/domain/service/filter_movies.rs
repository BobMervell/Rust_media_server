use crate::domain::movie::detailed_movie::{DetailedMovie, MovieDetailResult};

pub fn is_video_file(file_name: &str) -> bool {
    let video_extensions = ["mp4", "mkv", "avi", "mov", "flv", "wmv", "webm"];

    if let Some(ext) = file_name.rsplit('.').next() {
        video_extensions.contains(&ext.to_lowercase().as_str())
    } else {
        false
    }
}

pub fn is_not_featurette(file_path: &str) -> bool {
    let featurette_names = ["featurettes", "featurette", "feat"];
    if let Some(ext) = file_path.rsplit('/').next() {
        !featurette_names.contains(&ext.to_lowercase().as_str())
    } else {
        true
    }
}

pub fn get_most_popular(fetch_result: MovieDetailResult) -> DetailedMovie {
    let mut max_pop: f32 = 0.0;
    let mut result_movie = fetch_result.results()[0].clone();
    for movie in fetch_result.iter() {
        if movie.popularity() > max_pop {
            max_pop = movie.popularity();
            result_movie = movie.to_owned();
        }
    }
    return result_movie;
}
