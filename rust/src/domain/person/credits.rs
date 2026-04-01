use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CreditsMovie {
    #[serde(rename = "id")]
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}
impl CreditsMovie {
    pub fn cast(&self) -> &Vec<Cast> {
        &self.cast
    }
    pub fn crew(&self) -> &Vec<Crew> {
        &self.crew
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cast {
    #[serde(rename = "id")]
    ext_id: i64,
    name: String,
    character: String,
}
impl Cast {
    pub fn ext_id(&self) -> i64 {
        self.ext_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn character(&self) -> &str {
        &self.character
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Crew {
    #[serde(rename = "id")]
    ext_id: i64,
    name: String,

    department: String,
    job: String,
}

impl Crew {
    pub fn ext_id(&self) -> i64 {
        self.ext_id
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
