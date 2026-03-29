use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Genre {
    id: i64,
    name: String,
}
impl Genre {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct MovieGenres {
    genres: Vec<Genre>,
}
impl MovieGenres {
    pub fn iter(&self) -> impl Iterator<Item = &Genre> {
        self.genres.iter()
    }
    pub fn genres(&self) -> Vec<Genre> {
        self.genres.clone()
    }
}
