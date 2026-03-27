#[derive(Debug, Clone)]
pub struct RawEntry {
    pub file_path: String,
}

impl RawEntry {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_owned(),
        }
    }

    pub fn file_path(&self) -> String {
        return self.file_path.to_owned();
    }
}
