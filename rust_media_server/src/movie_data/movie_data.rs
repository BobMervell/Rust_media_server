use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug, Clone)]
pub struct Cast {
    id: i32,
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
             Name:                {}\n\
             Picture path:        {:?}\n\
             Character:           {}\n\
             Order:               {}",
            self.id, self.name, self.picture_path, self.character, self.order
        )
    }
}

impl Cast {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn picture_path(&self) -> &Option<String> {
        &self.picture_path
    }
    pub fn character(&self) -> &str {
        &self.character
    }
    pub fn order(&self) -> i32 {
        self.order
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Crew {
    id: i32,
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
             Name:                {}\n\
             Picture path:        {:?}\n\
             Character:           {}\n\
             Order:               {}",
            self.id, self.name, self.picture_path, self.department, self.job
        )
    }
}

impl Crew {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn picture_path(&self) -> &Option<String> {
        &self.picture_path
    }
    pub fn department(&self) -> &str {
        &self.department
    }
    pub fn job(&self) -> &str {
        &self.job
    }
}

#[derive(Debug, Clone)]
pub struct MovieData {
    file_path: String,
    file_title: String,
    file_year: String,
    file_optional_info: String,
    id: u32,
    original_title: String,
    title: String,
    genres: Vec<String>,
    vote_average: f32,
    release_date: String,
    summary: String,
    poster_large: String,
    poster_snapshot: String,
    backdrop: String,
    cast: Vec<Cast>,
    crew: Vec<Crew>,
}

impl fmt::Display for MovieData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File path:           {}\n\
             File title:           {}\n\
             File year:           {}\n\
             File optional info:  {}\n\
             ID:                  {}\n\
             Original title:      {}\n\
             Title:               {}\n\
             Genres:           {:?}\n\
             Vote average:        {:.1}\n\
             Release date:        {}\n\
             Summary:             {}\n\
             Poster large:        {}\n\
             Poster snapshot:     {}\n\
             Backdrop:            {}\n\
             Cast;                {:?}\n\
             Crew:                {:?}",
            self.file_path,
            self.file_title,
            self.file_year,
            self.file_optional_info,
            self.id,
            self.original_title,
            self.title,
            self.genres,
            self.vote_average,
            self.release_date,
            self.summary,
            self.poster_large,
            self.poster_snapshot,
            self.backdrop,
            self.cast,
            self.crew
        )
    }
}
impl MovieData {
    pub fn new(path: String) -> Self {
        let mut file_year = "";
        let mut file_optional_info = "";
        let mut file_title = "";

        if let Some(file_name) = path.rsplit('/').next() {
            if let (Some(start), Some(end)) = (file_name.find('('), file_name.find(')')) {
                file_year = &file_name[start + 1..end];
                file_title = &file_name[..start - 1];
            } else {
                println!("No date value in file: {}", path);
            }
            if let (Some(start), Some(end)) = (file_name.find('['), file_name.find(']')) {
                file_optional_info = &file_name[start + 1..end];
            }
        }
        Self {
            file_path: path.to_owned().to_lowercase(),
            file_title: file_title.to_owned().to_lowercase(),
            file_year: file_year.to_owned().to_lowercase(),
            file_optional_info: file_optional_info.to_owned().to_lowercase(),
            id: 0,
            original_title: "".to_owned(),
            title: "".to_owned(),
            genres: vec![],
            vote_average: 0.0,
            release_date: "".to_owned(),
            summary: "".to_owned(),
            poster_large: "".to_owned(),
            poster_snapshot: "".to_owned(),
            backdrop: "".to_owned(),
            cast: vec![],
            crew: vec![],
        }
    }

    // region: ----- GETTERS -----

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

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn original_title(&self) -> &str {
        &self.original_title
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn genres(&self) -> &[String] {
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

    pub fn poster_large(&self) -> &str {
        &self.poster_large
    }

    pub fn poster_snapshot(&self) -> &str {
        &self.poster_snapshot
    }

    pub fn backdrop(&self) -> &str {
        &self.backdrop
    }

    pub fn cast(&self) -> &[Cast] {
        &self.cast
    }

    pub fn crew(&self) -> &[Crew] {
        &self.crew
    }

    // endregion

    // region: ------ SETTERS -----

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

    pub fn set_id(&mut self, new_id: u32) -> &mut Self {
        self.id = new_id;
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

    pub fn set_genres(&mut self, new_genres: Vec<String>) -> &mut Self {
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

    pub fn set_poster_large(&mut self, new_poster_large: &str) -> &mut Self {
        self.poster_large = new_poster_large.to_owned();
        self
    }

    pub fn set_poster_snapshot(&mut self, new_poster_snapshot: &str) -> &mut Self {
        self.poster_snapshot = new_poster_snapshot.to_owned();
        self
    }

    pub fn set_backdrop(&mut self, new_backdrop: &str) -> &mut Self {
        self.backdrop = new_backdrop.to_owned();
        self
    }

    pub fn push_cast(&mut self, new_cast: Cast) -> &mut Self {
        self.cast.push(new_cast);
        self
    }

    pub fn pop_cast(&mut self) -> &mut Self {
        self.cast.pop();
        self
    }

    pub fn remove_cast(&mut self, index: usize) -> &mut Self {
        self.cast.remove(index);
        self
    }

    pub fn push_crew(&mut self, new_crew: Crew) -> &mut Self {
        self.crew.push(new_crew);
        self
    }

    pub fn pop_crew(&mut self) -> &mut Self {
        self.crew.pop();
        self
    }

    pub fn remove_crew(&mut self, index: usize) -> &mut Self {
        self.crew.remove(index);
        self
    }
    // endregion
}
