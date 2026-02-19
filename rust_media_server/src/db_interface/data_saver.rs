use rusqlite::{Connection, Result, Transaction};

use crate::movie_data::movie_data::MovieData;

pub struct DataSaver {
    conn: Connection,
}

impl DataSaver {
    pub fn new(db_path: String) -> Result<Self> {
        let conn = Connection::open(&db_path)?;

        Ok(Self { conn: conn })
    }

    // region: Create tables
    fn create_index(&self, table: &str, column: &str) {
        let index_name = format!("idx_{}_{}", table.to_lowercase(), column.to_lowercase());

        let query = format!(
            "CREATE INDEX IF NOT EXISTS {} ON {}({})",
            index_name, table, column
        );

        let res = self.conn.execute(&query, []);

        if let Err(e) = res {
            println!("Error creating index {}: {}", index_name, e);
        }
    }

    pub fn create_movie_table(&mut self) {
        let res = self.conn.execute(
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
        );

        if let Err(e) = res {
            println!("Error creation table Movie: {}", e);
        }

        self.create_index("Movie", "title");
        self.create_index("Movie", "release_date");
        self.create_index("Movie", "tmdb_id");
    }

    pub fn create_person_table(&mut self) {
        let res = self.conn.execute(
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
        );
        if let Err(e) = res {
            println!("Error creation table Person: {}", e);
        }

        self.create_index("Person", "name");
        self.create_index("Person", "job_name");

        let res = self.conn.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_cast_tmdb_movie_character_job
         ON Person (tmdb_id, movie_id, character, job_name);",
            [],
        );
        if let Err(e) = res {
            println!(
                "Error creating index composite index for person table: {}",
                e
            );
        }
    }

    pub fn create_genre_table(&mut self) {
        let res = self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Genre (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL);
            ",
            (),
        );
        if let Err(e) = res {
            println!("Error creation table Genre: {}", e);
        }
    }

    pub fn create_movie_genre_table(&mut self) {
        let res = self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Movie_Genre (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                movie_id INTEGER NOT NULL,
                genre_id INTEGER NOT NULL,
                FOREIGN KEY (movie_id) REFERENCES Movie(id),
                FOREIGN KEY (genre_id) REFERENCES Genre(id)
            );",
            (),
        );

        if let Err(e) = res {
            println!("{}", e)
        }

        self.create_index("Genre", "name");
        let res = self.conn.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_movie_genre
         ON Movie_Genre (movie_id, genre_id);",
            [],
        );
        if let Err(e) = res {
            println!(
                "Error creating index composite index for genre_movie table: {}",
                e
            );
        }
    }

    // endregion

    // region: Insert movie data
    pub fn push_movie(&mut self, m: MovieData) {
        let tx = self.conn.transaction().unwrap();

        let res = tx.execute(
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
        );

        if let Err(e) = res {
            println!("Error pushing data to table movie: {}", e);
        }

        let res = tx.query_row(
            "SELECT id FROM Movie WHERE file_path = ?1",
            (m.file_path(),),
            |row| row.get(0),
        );
        match res {
            Ok(movie_id) => {
                DataSaver::push_credits(movie_id, &m, &tx);
                DataSaver::push_genre(movie_id, &m, &tx);
            }

            Err(e) => {
                println!("Error getting movie_idfrom table movie: {}", e);
            }
        }

        let res = tx.commit();

        if let Err(e) = res {
            println!("error commiting: {}", e)
        }
    }

    fn push_credits(movie_id: i64, m: &MovieData, tx: &Transaction) {
        let res = tx.prepare(
            "INSERT INTO Person (tmdb_id, movie_id, name, job_name, character, picture_path)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(tmdb_id, movie_id, character, job_name) DO NOTHING",
        );

        match res {
            Ok(mut statement) => {
                for cast in m.cast().iter() {
                    let res = statement.execute((
                        cast.tmdb_id(),
                        movie_id,
                        cast.name(),
                        "actor",
                        cast.character(),
                        cast.picture_path(),
                    ));
                    if let Err(e) = res {
                        println!("Error pushing data to table person: {}", e);
                    }
                }

                for crew in m.crew().iter() {
                    let res = statement.execute((
                        crew.tmdb_id(),
                        movie_id,
                        crew.name(),
                        crew.job(),
                        "N/A",
                        crew.picture_path(),
                    ));
                    if let Err(e) = res {
                        println!("Error pushing data to table person: {}", e);
                        println!("{}", crew)
                    }
                }
            }

            Err(e) => {
                println!("Error preparing statement for credit push: {}", e)
            }
        }
    }

    fn push_genre(movie_id: i64, m: &MovieData, tx: &Transaction) {
        for genre in m.genres().iter() {
            let res = tx.execute(
                "INSERT INTO Genre ( id, name)
                VALUES (?1, ?2)
                ON CONFLICT(id) DO NOTHING;",
                (genre.id(), genre.name()),
            );
            DataSaver::push_movie_genre(genre.id(), movie_id, tx);
            if let Err(e) = res {
                println!("Error pushing data to table genre: {}", e);
            }
        }
    }

    fn push_movie_genre(genre_id: i64, movie_id: i64, tx: &Transaction) {
        let res = tx.execute(
            "INSERT INTO Movie_Genre ( movie_id, genre_id)
                VALUES (?1, ?2)
                ON CONFLICT(movie_id, genre_id) DO NOTHING;",
            (movie_id, genre_id),
        );
        if let Err(e) = res {
            println!("Error pushing data to table movie_genre: {}", e);
        }
    }

    // endregion
}
