use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PersonData {
    #[serde(rename = "id")]
    pub tmdb_id: i64,
    pub name: String,
    pub biography: String,
    #[serde(rename = "profile_path")]
    pub picture_api_path: Option<String>,
    #[serde(skip_deserializing)]
    pub picture_file_path: String,
}

impl PersonData {
    pub fn new(
        tmdb_id: i64,
        name: String,
        biography: String,
        picture_api_path: Option<String>,
        picture_file_path: String,
    ) -> Self {
        Self {
            tmdb_id,
            name,
            biography,
            picture_api_path,
            picture_file_path,
        }
    }

    pub fn tmdb_id(&self) -> i64 {
        self.tmdb_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn biography(&self) -> &str {
        &self.biography
    }

    pub fn picture_api_path(&self) -> Option<String> {
        self.picture_api_path.to_owned()
    }

    pub fn picture_file_path(&self) -> &str {
        &self.picture_file_path
    }

    pub fn set_picture_file_path(&mut self, path: String) {
        self.picture_file_path = path
    }
}
