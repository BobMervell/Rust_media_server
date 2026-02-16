use crate::movie_data::movie_data::{Cast, Crew, Genre};
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::{error::Error, io};

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

// region: SearchMovieStructs
#[derive(Serialize)]
struct SearchParams<'a> {
    query: &'a str,
    language: &'a str,
    page: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_release_year: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchedMovie {
    id: i64,
    original_title: String,
    title: String,
    genre_ids: Vec<i64>,
    popularity: f32,
    vote_average: f32,
    release_date: String,
    overview: String,
}
impl SearchedMovie {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn original_title(&self) -> &str {
        &self.original_title
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn genre_ids(&self) -> &Vec<i64> {
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
}

#[derive(Deserialize, Debug)]
pub struct MovieSearchResult {
    results: Vec<SearchedMovie>,
}

impl MovieSearchResult {
    fn iter(&self) -> std::slice::Iter<'_, SearchedMovie> {
        self.results.iter()
    }
}
// endregion

// region: DetailMovieStructs
#[derive(Deserialize, Debug, Clone)]
pub struct DetailsMovie {
    genres: Vec<Genre>,
    poster_path: String,
}
impl DetailsMovie {
    pub fn genres(&self) -> Vec<Genre> {
        self.genres.clone()
    }
    pub fn poster_path(&self) -> String {
        self.poster_path.clone()
    }
}

// endregion

// region: CreditMovieStructs
#[derive(Deserialize, Debug, Clone)]
pub struct CreditsMovie {
    cast: Vec<Cast>,
    crew: Vec<Crew>,
}
impl CreditsMovie {
    pub fn credits_cast(&self) -> &Vec<Cast> {
        &self.cast
    }
    pub fn credits_crew(&self) -> &Vec<Crew> {
        &self.crew
    }
}
// endregion

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

    pub async fn get_movie_info(
        &self,
        movie_name: &str,
        movie_year: Option<u32>,
    ) -> Option<SearchedMovie> {
        let movie_result = self.fetch_movie_by_name(movie_name, movie_year).await;

        match movie_result {
            Ok(fetch_result) if !fetch_result.results.is_empty() => {
                Some(self.get_most_popular(fetch_result))
            }
            Ok(_) => {
                println!(
                    "NO RESULT FOUND FOR MOVIE: {}, {:#?}",
                    movie_name, movie_year
                );
                None
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

    fn get_most_popular(&self, fetch_result: MovieSearchResult) -> SearchedMovie {
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

    pub async fn fetch_movie_details(&self, tmdb_id: i64) -> Result<DetailsMovie, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/movie/{}?language=en-US",
                TMDB_BASE_URL, &tmdb_id
            ))
            .send()
            .await?;
        let body_json = response.json::<DetailsMovie>().await?;
        Ok(body_json)
    }

    pub async fn fetch_movie_credits(&self, tmdb_id: i64) -> Result<CreditsMovie, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "{}/movie/{}/credits?language=en-US",
                TMDB_BASE_URL, &tmdb_id
            ))
            .send()
            .await?;

        println!("{:?}", response);
        let body_json = response.json::<CreditsMovie>().await?;
        Ok(body_json)
    }
}
