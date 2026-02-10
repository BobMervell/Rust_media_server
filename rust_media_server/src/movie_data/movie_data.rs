use std::fmt;

#[derive(Debug)]
pub struct MovieData {
    path: String,
    name: String,
    year: String,
    optional_info: String,
}
impl fmt::Display for MovieData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Path: {}\n Name: {}\n Year: {}\n Optional info: {}",
            self.path, self.name, self.year, self.optional_info
        )
    }
}
impl MovieData {
    pub fn new(path: String) -> Self {
        let mut year = "";
        let mut optional_info = "";
        let mut name = "";

        if let Some(file_name) = path.rsplit('/').next() {
            if let (Some(start), Some(end)) = (file_name.find('('), file_name.find(')')) {
                year = &file_name[start + 1..end];
                name = &file_name[..start - 1];
            } else {
                println!("No date value in file: {}", path);
            }
            if let (Some(start), Some(end)) = (file_name.find('['), file_name.find(']')) {
                optional_info = &file_name[start + 1..end];
            }
        }
        Self {
            path: path.to_owned(),
            name: name.to_owned(),
            year: year.to_owned(),
            optional_info: optional_info.to_owned(),
        }
    }

    // −−−−− GETTERS −−−−−

    pub fn path(&self) -> &str {
        &self.path
    }
}
