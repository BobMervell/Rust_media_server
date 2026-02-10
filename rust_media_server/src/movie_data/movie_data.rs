use std::fmt;

#[derive(Debug)]
pub struct MovieData {
    path: String,
}
impl fmt::Display for MovieData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
impl MovieData {
    pub fn new(path: String) -> Self {
        Self { path: path }
    }
    // −−−−− GETTERS −−−−−

    pub fn path(&self) -> &str {
        &self.path
    }
}
