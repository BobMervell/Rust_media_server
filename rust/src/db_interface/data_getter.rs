use crate::movie_data::movie_data::MovieSnapshot;
use anyhow::{Context, Result};
use rusqlite::Connection;

pub struct DataGetter {
    conn: Connection,
}

impl DataGetter {
    pub fn new(db_path: String) -> Result<Self> {
        let conn = Connection::open(&db_path)
            .with_context(|| format!("Failed to open database connection at : {}", &db_path))?;
        Ok(Self { conn: conn })
    }

    //TODO add filters
    pub fn get_media_snapshot(&self, media_type: &str) -> Result<Vec<MovieSnapshot>> {
        let query_str = format!(
            "SELECT tmdb_id, file_path, title, vote_average AS rating, release_date, poster_snapshot
             FROM {}
             ORDER BY title COLLATE NOCASE ",
             media_type);

        let mut stmt = self
            .conn
            .prepare(&query_str)
            .with_context(|| format!("Failed to prepare statement for data selection"))?;

        let mapped_rows = stmt
            .query_map([], |row| {
                Ok(MovieSnapshot::new(
                    row.get::<_, i64>(0).unwrap_or_default(),
                    row.get::<_, String>(1).unwrap_or_default(),
                    row.get::<_, String>(2).unwrap_or_default(),
                    row.get::<_, f32>(3).unwrap_or_default(),
                    row.get::<_, String>(4).unwrap_or_default(),
                    row.get::<_, String>(5).unwrap_or_default(),
                ))
            })
            .with_context(|| format!("Failed to get select result"))?;

        Ok(mapped_rows
            .filter_map(|res| res.ok())
            .collect::<Vec<MovieSnapshot>>())
    }
}
