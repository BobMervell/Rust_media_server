use anyhow::{Context, Result};
use rusqlite::{Connection, Transaction};

use crate::movie_data::movie_data::{CreditsMovie, MovieData};

pub struct DataSaver {
    conn: Connection,
}

impl DataSaver {
    pub fn new(db_path: String) -> Result<Self> {
        let conn = Connection::open(&db_path)
            .with_context(|| format!("Failed to open database connection at : {}", &db_path))?;
        Ok(Self { conn: conn })
    }

    // region: Create tables
    fn create_index(&self, table: &str, column: &str) -> Result<()> {
        let index_name = format!("idx_{}_{}", table.to_lowercase(), column.to_lowercase());

        let query = format!(
            "CREATE INDEX IF NOT EXISTS {} ON {}({})",
            index_name, table, column
        );

        self.conn.execute(&query, []).with_context(|| {
            format!(
                "Failed to create index for table: {} and column: {}",
                table, column
            )
        })?;

        Ok(())
    }

    pub fn create_movie_table(&mut self) -> Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS Movie (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tmdb_id INTEGER,
                file_path TEXT NOT NULL UNIQUE,
                file_optional_info TEXT,
                title TEXT NOT NULL,
                original_title TEXT NOT NULL,
                release_date TEXT,
                summary TEXT NOT NULL,
                vote_average REAL NOT NULL DEFAULT 0,
                poster_large TEXT NOT NULL,
                poster_snapshot TEXT NOT NULL,
                backdrop TEXT NOT NULL
            )",
                (),
            )
            .context("Failed to create movie table")?;

        self.create_index("Movie", "title")?;
        self.create_index("Movie", "release_date")?;
        self.create_index("Movie", "tmdb_id")?;

        Ok(())
    }

    pub fn create_person_table(&mut self) -> Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS Person (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tmdb_id INTEGER,
                movie_id INTEGER,
                name TEXT NOT NULL,
                character TEXT,
                job_name TEXT NOT NULL,
                picture_path TEXT,
                FOREIGN KEY (movie_id) REFERENCES Movie(id)
            );
            ",
                (),
            )
            .context("Failed to create person table")?;

        self.create_index("Person", "name")?;
        self.create_index("Person", "job_name")?;

        self.conn
            .execute(
                "CREATE UNIQUE INDEX IF NOT EXISTS idx_cast_tmdb_movie_character_job
         ON Person (tmdb_id, movie_id, character, job_name);",
                [],
            )
            .context("Failed to create unique composite index for table: Person")?;

        Ok(())
    }

    pub fn create_genre_table(&mut self) -> Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS Genre (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL);
            ",
                (),
            )
            .context("Failed to create genre table")?;
        Ok(())
    }

    pub fn create_movie_genre_table(&mut self) -> Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS Movie_Genre (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                movie_id INTEGER NOT NULL,
                genre_id INTEGER NOT NULL,
                FOREIGN KEY (movie_id) REFERENCES Movie(id),
                FOREIGN KEY (genre_id) REFERENCES Genre(id)
            );",
                (),
            )
            .context("Failed to create genre table")?;

        self.create_index("Genre", "name")?;
        self.conn.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_movie_genre
         ON Movie_Genre (movie_id, genre_id);",
            [],
        ).with_context(|| {
            format!(
                "Failed to create composite index for table: movie_genre and columns: movie_id and genre_id"
            )
        })?;

        Ok(())
    }
    // endregion

    // region: Insert movie data
    pub fn push_movie_data(&mut self, m: MovieData, c: CreditsMovie) -> Result<()> {
        let tx = self
            .conn
            .transaction()
            .context("Failed to open database transaction")?;

        let movie_id = match Self::push_movie(&m, &tx) {
            Ok(id) => id,
            Err(e) => {
                tracing::error!(
                    "Failed to push movie data for {} \n Caused by: {}",
                    m.file_path(),
                    e
                );
                return Ok(());
            }
        };

        Self::push_genre(movie_id, &m, &tx)
            .map_err(|e| {
                tracing::error!("Failed to push movie genre for {}", m.file_path());
                e
            })
            .ok();

        Self::push_credits(movie_id, &c, &tx)
            .map_err(|e| {
                tracing::error!("Failed to push movie credits for {}", m.file_path());
                e
            })
            .ok();

        tx.commit()
            .context("Failed to commit data insertion into movie table")?;

        tracing::debug!(file_path = &m.file_path() , "Movie data saved");
        Ok(())
    }

    fn push_movie(m: &MovieData, tx: &Transaction) -> Result<i64> {
        tx.execute(
            "
        INSERT INTO Movie ( tmdb_id, file_path, file_optional_info, title, original_title,
        release_date, summary, vote_average, poster_large, poster_snapshot, backdrop)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        ON CONFLICT(file_path) DO NOTHING;",
            (
                m.tmdb_id(),
                m.file_path(),
                m.file_optional_info(),
                m.title(),
                m.original_title(),
                m.release_date(),
                m.summary(),
                m.vote_average(),
                m.poster_large(),
                m.poster_snapshot(),
                m.backdrop(),
            ),
        )
        .with_context(|| {
            format!(
                "Failed to insert new entry into movie table: {}",
                m.file_path()
            )
        })?;

        let movie_id = tx
            .query_row(
                "SELECT id FROM Movie WHERE file_path = ?1",
                (m.file_path(),),
                |row| row.get::<_, i64>(0),
            )
            .context("Error getting movie_id from table movie: {}")?;

        Ok(movie_id)
    }

    fn push_credits(movie_id: i64, c: &CreditsMovie, tx: &Transaction) -> Result<()> {
        let mut statement = tx
            .prepare(
                "INSERT INTO Person (tmdb_id, movie_id, name, job_name, character, picture_path)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(tmdb_id, movie_id, character, job_name) DO NOTHING",
            )
            .context("Failed to prepare statement for credit insertion into Person table")?;

        for cast in c.credits_cast().iter() {
            statement
                .execute((
                    cast.tmdb_id(),
                    movie_id,
                    cast.name(),
                    "actor",
                    cast.character(),
                    cast.picture_path(),
                ))
                .with_context(|| {
                    format!("Failed to insert cast into Person table for: {}", cast)
                })?;
        }

        for crew in c.credits_crew().iter() {
            statement
                .execute((
                    crew.tmdb_id(),
                    movie_id,
                    crew.name(),
                    crew.job(),
                    "N/A",
                    crew.picture_path(),
                ))
                .with_context(|| {
                    format!("Failed to insert crew into Person table for: {}", crew)
                })?;
        }
        Ok(())
    }

    fn push_genre(movie_id: i64, m: &MovieData, tx: &Transaction) -> Result<()> {
        for genre in m.genres().iter() {
            tx.execute(
                "INSERT INTO Genre ( id, name)
                VALUES (?1, ?2)
                ON CONFLICT(id) DO NOTHING;",
                (genre.id(), genre.name()),
            )
            .with_context(|| {
                format!("Failed to insert new entry into Genre table for: {}", genre)
            })?;

            Self::push_movie_genre(genre.id(), movie_id, tx)?;
        }
        Ok(())
    }

    fn push_movie_genre(genre_id: i64, movie_id: i64, tx: &Transaction) -> Result<()> {
        tx.execute(
            "INSERT INTO Movie_Genre ( movie_id, genre_id)
                VALUES (?1, ?2)
                ON CONFLICT(movie_id, genre_id) DO NOTHING;",
            (movie_id, genre_id),
        )
        .with_context(|| {
            format!(
                "Failed to insert entry into Movie_Genre table for genre: {} and movie {}",
                genre_id, movie_id
            )
        })?;
        Ok(())
    }
    // endregion
}
