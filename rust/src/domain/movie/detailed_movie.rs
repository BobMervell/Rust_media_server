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

#[derive(Deserialize, Debug, Clone)]
pub struct DetailedMovie {
    #[serde(skip_deserializing)]
    file_path: String,
    #[serde(skip_deserializing)]
    file_optional_info: String,
    #[serde(skip_deserializing)]
    id: i64,
    #[serde(skip_deserializing)]
    poster_file_path: String,
    #[serde(skip_deserializing)]
    backdrop_file_path: String,

    #[serde(rename = "id")]
    ext_id: i64,
    original_title: String,
    title: String,
    #[serde(skip_deserializing)]
    genres: MovieGenres,
    popularity: f32,
    vote_average: f32,
    release_date: String,
    overview: String,
    #[serde(rename = "poster_path")]
    ext_poster_path: Option<String>,
    #[serde(rename = "backdrop_path")]
    ext_backdrop_path: Option<String>,
}

impl DetailedMovie {
    pub fn set_file_path(mut self, value: &str) -> Self {
        self.file_path = value.to_owned();
        self
    }

    pub fn set_file_optional_info(mut self, value: &str) -> Self {
        self.file_optional_info = value.to_owned();
        self
    }
    pub fn set_genres(&mut self, value: &MovieGenres) {
        self.genres = value.to_owned();
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn file_optional_info(&self) -> &str {
        &self.file_optional_info
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn ext_id(&self) -> i64 {
        self.ext_id
    }

    pub fn original_title(&self) -> &str {
        &self.original_title
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn genres(&self) -> MovieGenres {
        self.genres.clone()
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
    pub fn poster_file_path(&self) -> &str {
        &self.poster_file_path
    }

    pub fn backdrop_file_path(&self) -> &str {
        &self.backdrop_file_path
    }

    pub fn ext_poster_path(&self) -> Option<String> {
        self.ext_poster_path.to_owned()
    }

    pub fn ext_backdrop_path(&self) -> Option<String> {
        self.ext_backdrop_path.to_owned()
    }

    pub fn set_id(&mut self, new_id: i64) {
        self.id = new_id
    }

    pub fn set_poster_file_path(&mut self, new_path: String) {
        self.poster_file_path = new_path
    }

    pub fn set_backdrop_file_path(&mut self, new_path: String) {
        self.backdrop_file_path = new_path
    }
}
