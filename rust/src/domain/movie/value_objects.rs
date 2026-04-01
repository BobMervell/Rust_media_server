use serde::Deserialize;

use crate::domain::movie::detailed_movie::DetailedMovie;

#[derive(Deserialize, Debug, Clone)]
pub struct Genre {
    id: i64,
    name: String,
}
impl Genre {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct MovieGenres {
    genres: Vec<Genre>,
}
impl MovieGenres {
    pub fn iter(&self) -> impl Iterator<Item = &Genre> {
        self.genres.iter()
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
        &self.results
    }
}
