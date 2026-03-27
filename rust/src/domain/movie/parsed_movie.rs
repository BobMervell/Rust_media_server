use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct ParsedMovie {
    pub file_path: String,
    pub file_optional_info: String,
    pub file_title: String,
    pub file_year: String,
}

impl ParsedMovie {
    pub fn new(file_path: String) -> Result<Self> {
        let file_name = file_path.rsplit('/').next().unwrap_or(&file_path);
        let file_title;
        let file_year;
        let mut file_optional_info = "";

        if let (Some(start), Some(end)) = (file_name.find('('), file_name.find(')')) {
            file_year = &file_name[start + 1..end];
            file_title = &file_name[..start - 1];
        } else {
            return Err(anyhow!("No date value found in file: {}", file_name));
        }
        if let (Some(start), Some(end)) = (file_name.find('['), file_name.find(']')) {
            file_optional_info = &file_name[start + 1..end];
        }
        Ok(Self {
            file_path: file_path.clone(),
            file_title: file_title.to_string(),
            file_year: file_year.to_string(),
            file_optional_info: file_optional_info.to_string(),
        })
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn file_optional_info(&self) -> &str {
        &self.file_optional_info
    }

    pub fn file_title(&self) -> &str {
        &self.file_title
    }

    pub fn file_year(&self) -> &str {
        &self.file_year
    }
}
