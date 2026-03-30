#[derive(Debug, Clone)]
pub struct MovieSnapshot {
    pub id: i64,
    pub file_path: String,
    pub title: String,
    pub rating: f32,
    pub release_date: String,
    pub poster: String,
}

impl MovieSnapshot {
    pub fn new(
        id: i64,
        file_path: String,
        title: String,
        rating: f32,
        release_date: String,
        poster: String,
    ) -> Self {
        Self {
            id,
            file_path,
            title,
            rating,
            release_date,
            poster,
        }
    }
}
// endregion

// region: ---- MediaData ----
#[derive(Debug, Clone)]
pub struct MediaData {
    pub id: i64,
    pub file_path: String,
    pub file_optional_info: String,
    pub original_title: String,
    pub title: String,
    pub rating: f32,
    pub release_date: String,
    pub summary: String,
    pub poster: String,
    pub backdrop: String,
}

impl MediaData {
    pub fn new(
        id: i64,
        file_path: String,
        file_optional_info: String,
        original_title: String,
        title: String,
        rating: f32,
        release_date: String,
        summary: String,
        poster: String,
        backdrop: String,
    ) -> Self {
        Self {
            id,
            file_path,
            file_optional_info,
            original_title,
            title,
            rating,
            release_date,
            summary,
            poster,
            backdrop,
        }
    }
}
// endregion

// region: ---- PersonSnapshot ----
#[derive(Debug, Clone)]
pub struct PersonSnapshot {
    pub tmdb_id: i64,
    pub name: String,
    pub character: String,
    pub job_name: String,
    pub picture_path: String,
}

impl PersonSnapshot {
    pub fn new(
        tmdb_id: i64,
        name: String,
        character: String,
        job_name: String,
        picture_path: String,
    ) -> Self {
        Self {
            tmdb_id,
            name,
            character,
            job_name,
            picture_path,
        }
    }
}
// endregion
