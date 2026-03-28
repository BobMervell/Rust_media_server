use crate::{domain::movie::detailed_movie::DetailedMovie, movie_data::movie_data::Genre};

#[derive(Debug, Clone)]
pub struct CompleteMovie {
    id: i64,
    file_path: String,
    file_title: String,
    file_optional_info: String,
    tmdb_id: i64,
    original_title: String,
    title: String,
    genres: Vec<Genre>,
    vote_average: f32,
    release_date: String,
    summary: String,
    poster_file_path: String,
    backdrop_file_path: String,
    poster_tmdb_path: Option<String>,
    backdrop_tmdb_path: Option<String>,
}

impl CompleteMovie {
    pub fn new(detailed_movie: &DetailedMovie, placeholder_path: &str) -> Self {
        Self {
            id: 0,
            file_path: detailed_movie.file_path().to_owned(),
            file_title: detailed_movie.file_title().to_owned(),
            file_optional_info: detailed_movie.file_optional_info().to_owned(),
            tmdb_id: detailed_movie.tmdb_id(),
            original_title: detailed_movie.original_title().to_owned(),
            title: detailed_movie.title().to_owned(),
            genres: Vec::new(),
            vote_average: detailed_movie.vote_average(),
            release_date: detailed_movie.release_date().to_owned(),
            summary: detailed_movie.overview().to_owned(),
            poster_file_path: "".to_owned(),
            backdrop_file_path: "".to_owned(),
            poster_tmdb_path: detailed_movie.poster_path(),
            backdrop_tmdb_path: detailed_movie.backdrop_path(),
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn file_title(&self) -> &str {
        &self.file_title
    }

    pub fn file_optional_info(&self) -> &str {
        &self.file_optional_info
    }

    pub fn tmdb_id(&self) -> i64 {
        self.tmdb_id
    }

    pub fn original_title(&self) -> &str {
        &self.original_title
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn genres(&self) -> &[Genre] {
        &self.genres
    }

    pub fn vote_average(&self) -> f32 {
        self.vote_average
    }

    pub fn release_date(&self) -> &str {
        &self.release_date
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn poster_file_path(&self) -> &str {
        &self.poster_file_path
    }

    pub fn backdrop_file_path(&self) -> &str {
        &self.backdrop_file_path
    }

    pub fn poster_tmdb_path(&self) -> Option<&str> {
        self.poster_tmdb_path.as_deref()
    }

    pub fn backdrop_tmdb_path(&self) -> Option<&str> {
        self.backdrop_tmdb_path.as_deref()
    }

    pub fn set_poster_file_path(&mut self, new_path: String) {
        self.poster_file_path = new_path
    }

    pub fn set_backdrop_file_path(&mut self, new_path: String) {
        self.backdrop_file_path = new_path
    }

    pub fn set_genres(&mut self, new_genres: Vec<Genre>) {
        self.genres = new_genres;
    }
}
