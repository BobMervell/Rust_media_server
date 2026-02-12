use std::fmt;

#[derive(Debug, Clone)]
pub struct MovieData {
    file_path: String,
    file_title: String,
    file_year: String,
    file_optional_info: String,
    id: u32,
    original_title: String,
    title: String,
    genre_ids: Vec<i32>,
    vote_average: f32,
    release_date: String,
    sumary: String,
}

impl fmt::Display for MovieData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File path:           {}\n\
             File name:           {}\n\
             File year:           {}\n\
             File optional info:  {}\n\
             ID:                  {}\n\
             Original title:      {}\n\
             Title:               {}\n\
             Genre IDs:           {:?}\n\
             Vote average:        {:.1}\n\
             Release date:        {}\n\
             Summary:             {}",
            self.file_path,
            self.file_title,
            self.file_year,
            self.file_optional_info,
            self.id,
            self.original_title,
            self.title,
            self.genre_ids,
            self.vote_average,
            self.release_date,
            self.sumary
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
            genre_ids: vec![],
            vote_average: 0.0,
            release_date: "".to_owned(),
            sumary: "".to_owned(),
        }
    }

    // −−−−− GETTERS −−−−−

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

    // ------ SETTERS -----

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

    pub fn set_genre_ids(&mut self, new_genre_ids: Vec<i32>) -> &mut Self {
        self.genre_ids = new_genre_ids;
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

    pub fn set_sumary(&mut self, new_sumary: &str) -> &mut Self {
        self.sumary = new_sumary.to_owned();
        self
    }
}
