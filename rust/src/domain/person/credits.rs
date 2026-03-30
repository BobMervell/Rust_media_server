use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CreditsMovie {
    #[serde(rename = "id")]
    pub movie_tmdb_id: i64,
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}
impl CreditsMovie {
    pub fn movie_tmdb_id(&self) -> i64 {
        self.movie_tmdb_id
    }
    pub fn cast(&self) -> &Vec<Cast> {
        &self.cast
    }
    pub fn crew(&self) -> &Vec<Crew> {
        &self.crew
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cast {
    #[serde(skip_deserializing)]
    id: i64,
    #[serde(rename = "id")]
    tmdb_id: i64,
    name: String,
    character: String,
    order: i32,
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
    pub fn character(&self) -> &str {
        &self.character
    }
    pub fn order(&self) -> i32 {
        self.order
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Crew {
    #[serde(skip_deserializing)]
    id: i64,
    #[serde(rename = "id")]
    tmdb_id: i64,
    name: String,

    department: String,
    job: String,
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
    pub fn department(&self) -> &str {
        &self.department
    }
    pub fn job(&self) -> &str {
        &self.job
    }
}
