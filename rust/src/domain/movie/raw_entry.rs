#[derive(Debug, Clone)]
pub struct RawEntry {
    file_path: String,
    file_name: String,
}

impl RawEntry {
    pub fn new(file_path: &str, file_name: &str) -> Self {
        Self {
            file_path: file_path.to_owned(),
            file_name: file_name.to_owned(),
        }
    }

    pub fn file_path(&self) -> String {
        return self.file_path.to_owned();
    }

    pub fn file_name(&self) -> String {
        return self.file_name.to_owned();
    }
}
