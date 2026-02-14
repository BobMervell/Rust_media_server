use rusqlite::{Connection, Result, params};

use crate::movie_data::movie_data::MovieData;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub struct DataSaver {
    conn: Connection,
}

impl DataSaver {
    pub fn new(db_path: String) -> Result<Self> {
        let conn = Connection::open(&db_path)?;

        Ok(Self { conn: conn })
    }

    pub fn create_movie_table(&mut self) {
        let res = self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Movie (
                movie_id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT NOT NULL,
                file_optional_info TEXT,
                title TEXT NOT NULL,
                original_title TEXT NOT NULL,
                release_date DATE,
                summary TEXT NOT NULL,
                vote_average REAL NOT NULL DEFAULT 0,
                poster_large TEXT NOT NULL,
                poster_snapshot TEXT NOT NULL,
                backdrop TEXT NOT NULL
        )",
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
        INSERT INTO Movie ( file_path, file_optional_info, title, original_title,
        release_date, summary, vote_average, poster_large, poster_snapshot, backdrop)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            (
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

        match res {
            Ok(_) => {}

            Err(e) => {
                println!("{}", e)
            }
        }
    }
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
