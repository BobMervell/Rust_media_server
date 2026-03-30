use serde::Deserialize;

use crate::domain::{
    movie::value_objects::MovieGenres,
    person::{credits::CreditsMovie, person_data::PersonData},
};

pub struct EnrichedMovie {
    pub movie: DetailedMovie,
    pub credits: CreditsMovie,
    pub persons: Vec<PersonData>,
}
impl EnrichedMovie {
    pub fn new(movie: DetailedMovie, credits: CreditsMovie, persons: Vec<PersonData>) -> Self {
        Self {
            movie,
            credits,
            persons,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MovieDetailResult {
    results: Vec<DetailedMovie>,
}

impl MovieDetailResult {
    pub fn iter(&self) -> std::slice::Iter<'_, DetailedMovie> {
        self.results.iter()
    }

    pub fn results(&self) -> &[DetailedMovie] {
        return &self.results;
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DetailedMovie {
    #[serde(skip_deserializing)]
    file_path: String,
    #[serde(skip_deserializing)]
    file_title: String,
    #[serde(skip_deserializing)]
    file_optional_info: String,
    #[serde(rename = "id")]
    tmdb_id: i64,
    original_title: String,
    title: String,
    genre_ids: Vec<i64>,
    #[serde(skip_deserializing)]
    genre: MovieGenres,
    popularity: f32,
    vote_average: f32,
    release_date: String,
    overview: String,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
}

impl DetailedMovie {
    pub fn set_file_path(mut self, value: &str) -> Self {
        self.file_path = value.to_owned();
        self
    }

    pub fn set_file_title(mut self, value: &str) -> Self {
        self.file_title = value.to_owned();
        self
    }

    pub fn set_file_optional_info(mut self, value: &str) -> Self {
        self.file_optional_info = value.to_owned();
        self
    }
    pub fn set_genre(&mut self, value: &MovieGenres) {
        self.genre = value.to_owned();
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

    pub fn genre_ids(&self) -> &[i64] {
        &self.genre_ids
    }

    pub fn genre(&self) -> MovieGenres {
        self.genre.clone()
    }

    pub fn popularity(&self) -> f32 {
        self.popularity
    }

    pub fn vote_average(&self) -> f32 {
        self.vote_average
    }

    pub fn release_date(&self) -> &str {
        &self.release_date
    }

    pub fn overview(&self) -> &str {
        &self.overview
    }

    pub fn poster_path(&self) -> Option<String> {
        self.poster_path.to_owned()
    }

    pub fn backdrop_path(&self) -> Option<String> {
        self.backdrop_path.to_owned()
    }
}
