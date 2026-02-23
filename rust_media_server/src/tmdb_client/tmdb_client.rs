use crate::movie_data::movie_data::{Cast, CreditsMovie, Crew, Genre, MovieData};
use anyhow::{Context, Result, anyhow};
use reqwest::{
    Client, Response,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    io::{self},
    path::Path,
};
use tokio::io::AsyncWriteExt;

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

// region: SEARCH MOVIE STRUCT
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

// region: MOVIE GENRES STRUCT
#[derive(Deserialize, Debug, Clone)]
pub struct MovieGenres {
    genres: Vec<Genre>,
}
impl MovieGenres {
    pub fn genres(&self) -> Vec<Genre> {
        self.genres.clone()
    }
}

// endregion

pub struct TMDBClient {
    client: Client,
}

impl TMDBClient {
    pub fn new() -> Result<Self> {
        let mut token = String::new();
        // TODO move tmdb token acquisition out of tmdb new
        println!("Enter the tmdb token ");
        match io::stdin().read_line(&mut token) {
            Err(e) => eprintln!("Erreur: {}", e),
            Ok(_n) => {}
        }
        let token = token.trim_end();

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        //TODO WARNING HeaderValue::from_str is intended to be replaced in the future by a TryFrom.
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(token).context("Failed to create header value with token")?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to build client")?;

        Ok(Self { client: client })
    }

    // region: ----- GET MOVIE DATA -----
    pub async fn get_movie_info(
        &self,
        movie_name: &str,
        movie_year: Option<u32>,
    ) -> Result<SearchedMovie> {
        let movie_result = self.fetch_movie_by_name(movie_name, movie_year).await;

        match movie_result {
            Ok(fetch_result) if !fetch_result.results.is_empty() => {
                Ok(self.get_most_popular(fetch_result))
            }
            Ok(_) => Err(anyhow!(
                "NO RESULT FOUND FOR MOVIE: {}, {:#?}",
                movie_name,
                movie_year
            )),

            Err(e) => Err(anyhow!(
                "Error fetching movie: {} {:?} \n Caused by: {:?}",
                movie_name,
                movie_year,
                e
            )),
        }
    }

    async fn fetch_movie_by_name(
        &self,
        movie_name: &str,
        movie_year: Option<u32>,
    ) -> Result<MovieSearchResult> {
        let params = SearchParams {
            query: movie_name,
            language: "en-US",
            page: 1,
            primary_release_year: movie_year,
        };

        let url = format!("{}/search/movie", TMDB_BASE_URL);

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to get search response for movie: {} ({:?}), from url: {}",
                    movie_name, movie_year, &url
                )
            })?;
        let movie = response
            .json::<MovieSearchResult>()
            .await
            .with_context(|| {
                format!(
                    "Failed to deserialize search response for movie: {} ({:?}), from url: {}",
                    movie_name, movie_year, &url
                )
            })?;
        Ok(movie)
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

    pub async fn fetch_movie_genres(&self, tmdb_id: i64) -> Result<MovieGenres> {
        let url = format!("{}/movie/{}?language=en-US", TMDB_BASE_URL, &tmdb_id);

        let response = self.client.get(&url).send().await.with_context(|| {
            format!(
                "Failed to get detail response for movie id: {} , from url: {}",
                tmdb_id, &url
            )
        })?;
        let movie_details = response.json::<MovieGenres>().await.with_context(|| {
            format!(
                "Failed to deserialize detail response for movie id: {}, from url: {}",
                tmdb_id, &url
            )
        })?;
        Ok(movie_details)
    }

    pub async fn fetch_movie_credits(&self, tmdb_id: i64) -> Result<CreditsMovie> {
        let url = format!(
            "{}/movie/{}/credits?language=en-US",
            TMDB_BASE_URL, &tmdb_id
        );

        let response = self.client.get(&url).send().await.with_context(|| {
            format!(
                "Failed to get credit response for movie id: {} , from url: {}",
                tmdb_id, &url
            )
        })?;

        let credits_details = response.json::<CreditsMovie>().await.with_context(|| {
            format!(
                "Failed to deserialize credit response for movie id: {}, from url: {}",
                tmdb_id, &url
            )
        })?;
        Ok(credits_details)
    }
    // endregion

    // region: ----- GET IMAGES -----

    //TODO change update_images to work with results rather than option
    async fn update_images<T, FGet>(
        &self,
        item: &T,
        category: &str,
        name: &str,
        subdir: &str,
        image_size: &str,
        get_path: FGet,
    ) -> Option<String>
    where
        FGet: Fn(&T) -> Option<&String>,
    {
        let picture_path = match get_path(item) {
            Some(p) => p,
            None => return None,
        };

        let (created, dir_path) = match self.create_dir(category, subdir, name) {
            Ok(bp) => bp,
            Err(e) => {
                tracing::error!("Error creating directory for: {} \n Caused by: {}", name, e);
                return None;
            }
        };

        if !created {
            return None;
        }

        let path = Path::new(&dir_path);

        if let Err(e) = self.save_image(image_size, &picture_path, path).await {
            tracing::error!("Failed to save image for: {} \n Caused by: {}", name, e);
            return None;
        }
        return Some(dir_path);
    }

    pub async fn update_cast_images(&self, cast: &Cast) -> Option<String> {
        let cast_name = cast.name().to_owned();
        self.update_images(cast, "person", &cast_name, &cast_name, "w185", |c| {
            c.picture_path()
        })
        .await
    }

    pub async fn update_crew_images(&self, crew: &Crew) -> Option<String> {
        let crew_name = crew.name().to_owned();
        self.update_images(crew, "person", &crew_name, &crew_name, "w185", |c| {
            c.picture_path()
        })
        .await
    }

    pub async fn update_movie_poster(&self, movie: &MovieData) -> Option<String> {
        let movie_name = movie.title().to_owned();
        self.update_images(movie, "movie", "poster_large", &movie_name, "w780", |m| {
            m.poster_large()
        })
        .await
    }

    pub async fn update_movie_backdrop(&self, movie: &MovieData) -> Option<String> {
        let movie_name = movie.title().to_owned();
        self.update_images(movie, "movie", "backdrop", &movie_name, "w1280", |m| {
            m.backdrop()
        })
        .await
    }

    pub async fn update_movie_poster_snapshot(&self, movie: &MovieData) -> Option<String> {
        let movie_name = movie.title().to_owned();
        self.update_images(
            movie,
            "movie",
            "poster_snapshot",
            &movie_name,
            "w185",
            |m| m.poster_snapshot(),
        )
        .await
    }

    //returns a result tuple with true if the directory was created, and false if it already exists
    fn create_dir(
        &self,
        parent_folder_name: &str,
        folder_name: &str,
        file_name: &str,
    ) -> Result<(bool, String)> {
        let mut save_dir =
            std::env::current_dir().context("Failed to retrieve current working directory")?;
        save_dir.push("images");
        save_dir.push(parent_folder_name);
        save_dir.push(folder_name);
        let full_path = save_dir.join(file_name);

        if full_path.exists() {
            Ok((false, full_path.to_string_lossy().to_string()))
        } else {
            fs::create_dir_all(&save_dir).with_context(|| {
                format!("Failed to create directories for path {:?}", &save_dir)
            })?;
            Ok((true, full_path.to_string_lossy().to_string()))
        }
    }

    async fn save_image(&self, format: &str, picture_path: &str, full_path: &Path) -> Result<()> {
        let mut response = self
            .get_image(format, picture_path)
            .await
            .context("Failed to get image from tmdb")?;

        let mut file = tokio::fs::File::create(full_path)
            .await
            .context("Failed to create file for saving image")?;
        while let Some(chunk) = response
            .chunk()
            .await
            .context("Failed to get response chunk")?
        {
            file.write_all(&chunk)
                .await
                .context("Failed to write image file")?;
        }

        Ok(())
    }

    async fn get_image(&self, format: &str, picture_path: &str) -> Result<Response> {
        let url = format!("https://image.tmdb.org/t/p/{}/{}", format, picture_path);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to get response for url: {}", &url))?;

        if !response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
            if let Ok(content_type_str) = content_type.to_str() {
                if !content_type_str.starts_with("image/") {
                    return Err(anyhow!("Response is not an image for url: {}", &url));
                }
            }
        }
        return Ok(response);
    }
    // endregion
}
