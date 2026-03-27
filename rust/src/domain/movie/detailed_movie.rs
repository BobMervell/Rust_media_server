use serde::Deserialize;

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
    popularity: f32,
    vote_average: f32,
    release_date: String,
    overview: String,
    poster: Option<String>,
    backdrop: Option<String>,
}

impl DetailedMovie {
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

    pub fn poster(&self) -> Option<String> {
        self.poster.to_owned()
    }

    pub fn backdrop(&self) -> Option<String> {
        self.backdrop.to_owned()
    }
}
