use crate::movie_data::movie_data::{MediaData, MovieSnapshot, PersonData};
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
            "SELECT id, file_path, title, vote_average AS rating, release_date, poster
             FROM {}
             ORDER BY title COLLATE NOCASE ",
            media_type
        );

        let mut stmt = self
            .conn
            .prepare(&query_str)
            .with_context(|| format!("Failed to prepare statement for data selection"))?;

        let mapped_rows = stmt
            .query_map([], |row| {
                Ok(MovieSnapshot::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            })
            .with_context(|| format!("Failed to get select result"))?;

        Ok(mapped_rows
            .filter_map(|res| res.ok())
            .collect::<Vec<MovieSnapshot>>())
    }

    pub fn get_media_data(&self, media_id: i64) -> Result<MediaData> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, file_path, file_optional_info, original_title, title,
                vote_average AS rating, release_date, summary, poster, backdrop
         FROM MOVIE
         WHERE id = ?1",
            )
            .with_context(|| "Failed to prepare statement for data selection")?;

        let media = stmt
            .query_row([media_id], |row| {
                Ok(MediaData::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                    row.get(8)?,
                    row.get(9)?,
                ))
            })
            .with_context(|| "Failed to fetch media data")?;

        Ok(media)
    }

    pub fn get_media_cast(&self, media_id: i64) -> Result<Vec<PersonData>> {
        let query_str = format!(
            "SELECT tmdb_id, name, character, job_name, picture_path
             FROM Person
             WHERE movie_id = ?1 AND job_name = 'actor'
             ORDER BY id ",
        );

        let mut stmt = self
            .conn
            .prepare(&query_str)
            .with_context(|| format!("Failed to prepare statement for data selection"))?;

        let mapped_rows = stmt
            .query_map([media_id], |row| {
                Ok(PersonData::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })
            .with_context(|| format!("Failed to get select result"))?;

        Ok(mapped_rows
            .filter_map(|res| res.ok())
            .collect::<Vec<PersonData>>())
    }

    pub fn get_media_crew(&self, media_id: i64) -> Result<Vec<PersonData>> {
        let query_str = format!(
            "SELECT tmdb_id, name, character, job_name, picture_path
             FROM Person
             WHERE movie_id = ?1 AND job_name != 'actor'
             ORDER BY id ",
        );

        let mut stmt = self
            .conn
            .prepare(&query_str)
            .with_context(|| format!("Failed to prepare statement for data selection"))?;

        let mapped_rows = stmt
            .query_map([media_id], |row| {
                Ok(PersonData::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })
            .with_context(|| format!("Failed to get select result"))?;

        Ok(mapped_rows
            .filter_map(|res| res.ok())
            .collect::<Vec<PersonData>>())
    }
}
