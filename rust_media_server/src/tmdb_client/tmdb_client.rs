use crate::movie_data::movie_data::{Cast, Crew, Genre, MovieData};
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self},
    io::{self},
    path::Path,
};
use tokio::io::AsyncWriteExt;

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
    backdrop_path: Option<String>,
    poster_path: Option<String>,
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
    pub fn backdrop_path(&self) -> &Option<String> {
        &self.backdrop_path
    }
    pub fn poster_path(&self) -> &Option<String> {
        &self.poster_path
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

    pub fn credits_cast_mut(&mut self) -> &mut Vec<Cast> {
        &mut self.cast
    }
    pub fn credits_crew_mut(&mut self) -> &mut Vec<Crew> {
        &mut self.crew
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

    // region: ----- Get movies info -----
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
                println!("error fetching movie: {}", e);
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

        let body_json = response.json::<CreditsMovie>().await?;
        Ok(body_json)
    }
    // endregion

    // region: ----- Get images -----

    async fn update_images<T, FGet, FSet>(
        &self,
        item: &mut T,
        category: &str,
        name: &str,
        subdir: &str,
        image_size: &str,
        get_path: FGet,
        set_path: FSet,
    ) where
        FGet: Fn(&T) -> Option<&String>,
        FSet: Fn(&mut T, Option<String>),
    {
        let picture_path = match get_path(item) {
            Some(p) => p,
            None => return,
        };

        let (created, dir_path) = match self.create_dir(category, subdir, name) {
            Ok(bp) => bp,
            Err(e) => {
                println!("Error creating directory for {}: {}", name, e);
                return;
            }
        };

        if !created {
            return;
        }

        let path = Path::new(&dir_path);

        if let Err(e) = self.save_image(image_size, &picture_path, path).await {
            println!("{}", e);
            return;
        }

        set_path(item, Some(dir_path));
    }

    pub async fn update_cast_images(&self, cast: &mut Cast) {
        let cast_name = cast.name().to_owned();
        self.update_images(
            cast,
            "person",
            &cast_name,
            &cast_name,
            "w185",
            |c| c.picture_path(),
            |c, path| c.set_picture_path(path),
        )
        .await;
    }

    pub async fn update_crew_images(&self, crew: &mut Crew) {
        let crew_name = crew.name().to_owned();
        self.update_images(
            crew,
            "person",
            &crew_name,
            &crew_name,
            "w185",
            |c| c.picture_path(),
            |c, path| c.set_picture_path(path),
        )
        .await;
    }

    pub async fn update_movie_poster(&self, movie: &mut MovieData) {
        let movie_name = movie.title().to_owned();
        self.update_images(
            movie,
            "movie",
            "poster_large",
            &movie_name,
            "w780",
            |m| m.poster_large(),
            |m, path| {
                m.set_poster_large(path);
            },
        )
        .await;
    }

    pub async fn update_movie_backdrop(&self, movie: &mut MovieData) {
        let movie_name = movie.title().to_owned();
        self.update_images(
            movie,
            "movie",
            "backdrop",
            &movie_name,
            "w1280",
            |m| m.backdrop(),
            |m, path| {
                m.set_backdrop(path);
            },
        )
        .await;
    }

    pub async fn update_movie_poster_snapshot(&self, movie: &mut MovieData) {
        let movie_name = movie.title().to_owned();
        self.update_images(
            movie,
            "movie",
            "poster_snapshot",
            &movie_name,
            "w185",
            |m| m.poster_snapshot(),
            |m, path| {
                m.set_poster_snapshot(path);
            },
        )
        .await;
    }

    //returns a result tuple with true if the directory was created, and false if it already exists
    fn create_dir(
        &self,
        parent_folder_name: &str,
        folder_name: &str,
        file_name: &str,
    ) -> Result<(bool, String), std::io::Error> {
        let mut save_dir = std::env::current_dir()?;
        save_dir.push("images");
        save_dir.push(parent_folder_name);
        save_dir.push(folder_name);
        let full_path = save_dir.join(file_name);

        if full_path.exists() {
            Ok((false, full_path.to_string_lossy().to_string()))
        } else {
            fs::create_dir_all(&save_dir)?;
            Ok((true, full_path.to_string_lossy().to_string()))
        }
    }

    async fn save_image(
        &self,
        format: &str,
        picture_path: &str,
        full_path: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("https://image.tmdb.org/t/p/{}/{}", format, picture_path);

        let mut response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            println!("HTTP error: {}", response.status());
            return Ok(());
        }

        if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
            if let Ok(content_type_str) = content_type.to_str() {
                if !content_type_str.starts_with("image/") {
                    println!("Response is not an image");
                    return Ok(());
                }
            }

            let mut file = tokio::fs::File::create(full_path).await?;
            while let Some(chunk) = response.chunk().await? {
                file.write_all(&chunk).await?;
            }
        }
        Ok(())
    }

    // endregion
}
