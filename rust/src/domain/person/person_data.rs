use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PersonData {
    #[serde(rename = "id")]
    pub tmdb_id: i64,
    pub name: String,
    pub biography: String,
    #[serde(rename = "profile_path")]
    pub picture_path: Option<String>,
}

impl PersonData {
    pub fn new(
        tmdb_id: i64,
        name: String,
        biography: String,
        picture_path: Option<String>,
    ) -> Self {
        Self {
            tmdb_id,
            name,
            biography,
            picture_path,
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

    pub fn picture_path(&self) -> Option<&String> {
        self.picture_path.as_ref()
    }

    pub fn set_picture_path(&mut self, path: String) {
        self.picture_path = Some(path)
    }
}
