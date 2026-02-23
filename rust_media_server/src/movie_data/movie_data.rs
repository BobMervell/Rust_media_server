use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use std::fmt;

// region: ---- GENRE ----
#[derive(Deserialize, Debug, Clone)]
pub struct Genre {
    id: i64,
    name: String,
}
impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Genre ID:           {}\n\
             name:                {}",
            self.id, self.name
        )
    }
}
impl Genre {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
// endregion

// region: ---- CREDITS ----
#[derive(Deserialize, Debug, Clone)]
pub struct CreditsMovie {
    cast: Vec<Cast>,
    crew: Vec<Crew>,
}
impl CreditsMovie {
    pub fn new() -> Self {
        Self {
            cast: Vec::new(),
            crew: Vec::new(),
        }
    }

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

    pub fn set_cast_image(&mut self, indx: usize, image_path: &str) -> Result<()> {
        if indx > self.cast.len() - 1 {
            return Err(anyhow!(
                "index value out of bounds while trying to set cast image path"
            ));
        }
        self.cast[indx].set_picture_path(Some(image_path.to_owned()));
        Ok(())
    }
    pub fn set_crew_image(&mut self, indx: usize, image_path: &str) -> Result<()> {
        if indx > self.crew.len() - 1 {
            return Err(anyhow!(
                "index value out of bounds while trying to set crew image path"
            ));
        }
        self.crew[indx].set_picture_path(Some(image_path.to_owned()));
        Ok(())
    }
}
// endregion

// region: ---- CAST ----
#[derive(Deserialize, Debug, Clone)]
pub struct Cast {
    #[serde(skip_deserializing)]
    id: i64,
    #[serde(rename = "id")]
    tmdb_id: i64,
    name: String,
    #[serde(rename = "profile_path")]
    picture_path: Option<String>,
    character: String,
    order: i32,
}
impl fmt::Display for Cast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Person ID:           {}\n\
             Tmdb_id:             {}\n\
             Name:                {}\n\
             Picture path:        {:?}\n\
             Character:           {}\n\
             Order:               {}",
            self.id, self.tmdb_id, self.name, self.picture_path, self.character, self.order
        )
    }
}

impl Cast {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn tmdb_id(&self) -> i64 {
        self.tmdb_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn picture_path(&self) -> Option<&String> {
        self.picture_path.as_ref()
    }
    pub fn character(&self) -> &str {
        &self.character
    }
    pub fn order(&self) -> i32 {
        self.order
    }
    pub fn set_picture_path(&mut self, new_path: Option<String>) {
        self.picture_path = new_path
    }
}
// endregion

// region: ---- CREW ----
#[derive(Deserialize, Debug, Clone)]
pub struct Crew {
    #[serde(skip_deserializing)]
    id: i64,
    #[serde(rename = "id")]
    tmdb_id: i64,
    name: String,
    #[serde(rename = "profile_path")]
    picture_path: Option<String>,
    department: String,
    job: String,
}
impl fmt::Display for Crew {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Person ID:           {}\n\
             Tmdb_id:             {}\n\
             Name:                {}\n\
             Picture path:        {:?}\n\
             Character:           {}\n\
             Order:               {}",
            self.id, self.tmdb_id, self.name, self.picture_path, self.department, self.job
        )
    }
}

impl Crew {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn tmdb_id(&self) -> i64 {
        self.tmdb_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn picture_path(&self) -> Option<&String> {
        self.picture_path.as_ref()
    }
    pub fn department(&self) -> &str {
        &self.department
    }
    pub fn job(&self) -> &str {
        &self.job
    }
    pub fn set_picture_path(&mut self, new_path: Option<String>) {
        self.picture_path = new_path
    }
}
// endregion

#[derive(Debug, Clone)]
pub struct MovieData {
    id: i64,
    file_path: String,
    file_title: String,
    file_year: String,
    file_optional_info: String,
    tmdb_id: i64,
    original_title: String,
    title: String,
    genres: Vec<Genre>,
    vote_average: f32,
    release_date: String,
    summary: String,
    poster_large: Option<String>,
    poster_snapshot: Option<String>,
    backdrop: Option<String>,
}
// region: ---- DISPLAY ----
impl fmt::Display for MovieData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID:                  {}\n\
             File path:           {}\n\
             File title:           {}\n\
             File year:           {}\n\
             File optional info:  {}\n\
             tmdb_id:             {}\n\
             Original title:      {}\n\
             Title:               {}\n\
             Genres:           {:?}\n\
             Vote average:        {:.1}\n\
             Release date:        {}\n\
             Summary:             {}\n\
             Poster large:        {:?}\n\
             Poster snapshot:     {:?}\n\
             Backdrop:            {:?}",
            self.id,
            self.file_path,
            self.file_title,
            self.file_year,
            self.file_optional_info,
            self.tmdb_id,
            self.original_title,
            self.title,
            self.genres,
            self.vote_average,
            self.release_date,
            self.summary,
            self.poster_large,
            self.poster_snapshot,
            self.backdrop,
        )
    }
}
// endregion

impl MovieData {
    pub fn new(path: &str) -> Result<Self> {
        let file_name = path.rsplit('/').next().unwrap_or(&path);
        let (file_title, file_year, file_optional_info) = Self::parse_file_name(file_name)
            .with_context(|| format!("Failed to parse file: {}", &path))?;

        Ok(Self {
            id: 0,
            file_path: path.to_owned().to_lowercase(),
            file_title: file_title.to_lowercase(),
            file_year: file_year.to_lowercase(),
            file_optional_info: file_optional_info.to_lowercase(),
            tmdb_id: 0,
            original_title: "".to_owned(),
            title: "".to_owned(),
            genres: vec![],
            vote_average: 0.0,
            release_date: "".to_owned(),
            summary: "".to_owned(),
            poster_large: None,
            poster_snapshot: None,
            backdrop: None,
        })
    }

    fn parse_file_name(name: &str) -> Result<(String, String, String)> {
        let file_title;
        let file_year;
        let mut file_optional_info = "";

        if let (Some(start), Some(end)) = (name.find('('), name.find(')')) {
            file_year = &name[start + 1..end];
            file_title = &name[..start - 1];
        } else {
            return Err(anyhow!("No date value found in file: {}", name));
        }
        if let (Some(start), Some(end)) = (name.find('['), name.find(']')) {
            file_optional_info = &name[start + 1..end];
        }
        return Ok((
            file_title.to_string(),
            file_year.to_string(),
            file_optional_info.to_string(),
        ));
    }

    // region: ----- GETTERS -----

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn file_title(&self) -> &str {
        &self.file_title
    }

    pub fn file_year(&self) -> &str {
        &self.file_year
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

    pub fn poster_large(&self) -> Option<&String> {
        self.poster_large.as_ref()
    }

    pub fn poster_snapshot(&self) -> Option<&String> {
        self.poster_snapshot.as_ref()
    }

    pub fn backdrop(&self) -> Option<&String> {
        self.backdrop.as_ref()
    }
    // endregion

    // region: ------ SETTERS -----

    pub fn set_id(&mut self, new_id: i64) -> &mut Self {
        self.id = new_id;
        self
    }

    pub fn set_file_path(&mut self, new_file_path: &str) -> &mut Self {
        self.file_path = new_file_path.to_owned();
        self
    }

    pub fn set_file_title(&mut self, new_file_title: &str) -> &mut Self {
        self.file_title = new_file_title.to_owned();
        self
    }

    pub fn set_file_year(&mut self, new_file_year: &str) -> &mut Self {
        self.file_year = new_file_year.to_owned();
        self
    }

    pub fn set_file_optional_info(&mut self, new_file_optional_info: &str) -> &mut Self {
        self.file_optional_info = new_file_optional_info.to_owned();
        self
    }

    pub fn set_tmdb_id(&mut self, new_id: i64) -> &mut Self {
        self.tmdb_id = new_id;
        self
    }

    pub fn set_original_title(&mut self, new_original_title: &str) -> &mut Self {
        self.original_title = new_original_title.to_owned();
        self
    }

    pub fn set_title(&mut self, new_title: &str) -> &mut Self {
        self.title = new_title.to_owned();
        self
    }

    pub fn set_genres(&mut self, new_genres: Vec<Genre>) -> &mut Self {
        self.genres = new_genres;
        self
    }

    pub fn set_vote_average(&mut self, new_vote_average: f32) -> &mut Self {
        self.vote_average = new_vote_average;
        self
    }

    pub fn set_release_date(&mut self, new_release_date: &str) -> &mut Self {
        self.release_date = new_release_date.to_owned();
        self
    }

    pub fn set_summary(&mut self, new_summary: &str) -> &mut Self {
        self.summary = new_summary.to_owned();
        self
    }

    pub fn set_poster_large(&mut self, new_poster_large: Option<String>) -> &mut Self {
        self.poster_large = new_poster_large;
        self
    }

    pub fn set_poster_snapshot(&mut self, new_poster_snapshot: Option<String>) -> &mut Self {
        self.poster_snapshot = new_poster_snapshot;
        self
    }

    pub fn set_backdrop(&mut self, new_backdrop: Option<String>) -> &mut Self {
        self.backdrop = new_backdrop;
        self
    }
    // endregion
}
