use rusqlite::{Connection, Result};

use crate::movie_data::movie_data::MovieData;

pub struct DataSaver {
    conn: Connection,
}

impl DataSaver {
    pub fn new(db_path: String) -> Result<Self> {
        let conn = Connection::open(&db_path)?;

        Ok(Self { conn: conn })
    }

    fn create_index(&self, table: &str, column: &str) {
        // Optionnel mais recommandé : validation simple
        if !is_valid_identifier(table) || !is_valid_identifier(column) {
            println!("Invalid table or column name");
            return;
        }

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
                movie_id INTEGER PRIMARY KEY AUTOINCREMENT,
                tmdb_id INTEGER,
                file_path TEXT NOT NULL,
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
                person_id INTEGER PRIMARY KEY AUTOINCREMENT,
                tmdb_id INTEGER,
                name TEXT NOT NULL,
                character TEXT,
                job_name TEXT NOT NULL,
                picture_path TEXT
            );
            ",
            (),
        );
        if let Err(e) = res {
            println!("Error creation table Person: {}", e);
        }
    }

    pub fn create_movie_person_table(&mut self) {
        let res = self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Movie_Person (
                movie_person_id INTEGER PRIMARY KEY AUTOINCREMENT,
                movie_id INTEGER NOT NULL,
                person_id INTEGER NOT NULL,
                FOREIGN KEY (movie_id) REFERENCES Movie(movie_id),
                FOREIGN KEY (person_id) REFERENCES Person(person_id)
            );",
            (),
        );
        match res {
            Ok(_) => {}

            Err(e) => {
                println!("{}", e)
            }
        }
    }

    pub fn push_movie(&mut self, m: MovieData) {
        let res = self.conn.execute(
            "
        INSERT INTO Movie ( tmdb_id, file_path, file_optional_info, title, original_title,
        release_date, summary, vote_average, poster_large, poster_snapshot, backdrop)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            (
                m.id(),
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
    }

    pub fn push_movie_credits(&mut self, m: MovieData) {
        for cast in m.cast().iter() {
            let res = self.conn.execute(
                "INSERT INTO Person ( tmdb_id, name, job_name, character, picture_path)
                VALUES (?1, ?2, ?3, ?4, ?5)",
                (
                    cast.id(),
                    cast.name(),
                    cast.character(),
                    "actor",
                    cast.picture_path(),
                ),
            );
            if let Err(e) = res {
                println!("Error pushing data to table movie: {}", e);
            }
        }

        for crew in m.crew().iter() {
            let res = self.conn.execute(
                "INSERT INTO Person ( tmdb_id, name, job_name, picture_path)
                VALUES (?1, ?2, ?3, ?4)",
                (
                    crew.id(),
                    crew.name(),
                    crew.job(),
                    crew.picture_path(),
                ),
            );
            if let Err(e) = res {
                println!("Error pushing data to table movie: {}", e);
            }
        }


    }
}

fn is_valid_identifier(name: &str) -> bool {
    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}
// fn main() -> Result<()> {
//     let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
//     let person_iter = stmt.query_map([], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             data: row.get(2)?,
//         })
//     })?;

//     for person in person_iter {
//         println!("Found person {:?}", person?);
//     }
//     Ok(())
// }
