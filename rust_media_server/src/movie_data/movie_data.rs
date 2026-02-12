use std::fmt;

#[derive(Debug, Clone)]
pub struct MovieData {
    file_path: String,
    file_title: String,
    file_year: String,
    file_optional_info: String,
}
impl fmt::Display for MovieData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "file_path: {}\n file_name: {}\n file_year: {}\n file_optional_info: {}",
            self.file_path, self.file_title, self.file_year, self.file_optional_info
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
}
