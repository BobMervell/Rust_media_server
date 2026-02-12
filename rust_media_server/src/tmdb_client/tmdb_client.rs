use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::{error::Error, io};

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

#[derive(Serialize)]
struct SearchParams<'a> {
    query: &'a str,
    language: &'a str,
    page: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_release_year: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Movie {
    id: u32,
    original_title: String,
    genre_ids: Vec<i32>,
    popularity: f32,
    vote_average: f32,
    video: bool,
    release_date: String,
    overview: String,
}

#[derive(Deserialize, Debug)]
pub struct MovieSearchResult {
    results: Vec<Movie>,
}

impl MovieSearchResult {
    fn iter(&self) -> std::slice::Iter<'_, Movie> {
        self.results.iter()
    }
}

pub struct TMDBClient {
    client: Client,
}

impl TMDBClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut token = String::new();
        println!("Enter the tmdb token ");
        match io::stdin().read_line(&mut token) {
            Err(e) => eprintln!("Erreur: {}", e),
            Ok(_n) => {}
        }
        let token = token.trim_end();

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(token)?);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self { client: client })
    }

    pub async fn get_movie_info(&self, movie_name: &str, movie_year: Option<u32>) -> Option<Movie> {
        let movie_result = self.fetch_movie_by_name(movie_name, movie_year).await;

        match movie_result {
            Ok(fetch_result) => {
                return Some(self.get_most_popular(fetch_result));
            }
            Err(e) => {
                println!("error: {}", e);
                return None;
            }
        }
    }

    async fn fetch_movie_by_name(
        &self,
        movie_name: &str,
        movie_year: Option<u32>,
    ) -> Result<MovieSearchResult, reqwest::Error> {
        let params = SearchParams {
            query: movie_name,
            language: "en-US",
            page: 1,
            primary_release_year: movie_year,
        };

        let response = self
            .client
            .get(format!("{}/search/movie", TMDB_BASE_URL))
            .query(&params)
            .send()
            .await?;
        let body_json = response.json::<MovieSearchResult>().await?;
        Ok(body_json)
    }

    fn get_most_popular(&self, fetch_result: MovieSearchResult) -> Movie {
        let mut max_pop: f32 = 0.0;
        let mut result_movie = fetch_result.results[0].clone();
        for movie in fetch_result.iter() {
            if movie.popularity > max_pop {
                max_pop = movie.popularity;
                result_movie = movie.to_owned();
            }
        }
        return result_movie;
    }
}
