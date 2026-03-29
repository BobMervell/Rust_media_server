use std::any::TypeId;

use crate::{
    application::abstractions::abstractions::{MovieRepository, MoviesParser},
    db_interface::data_saver::DataSaver,
    domain::movie::complete_movie::CompleteEnrichedMovie,
};
use anyhow::{anyhow, Context, Result};
use futures::StreamExt;
use trpl::Stream;
pub struct SqliteDataSaver {
    data_saver: DataSaver,
}

impl MovieRepository for SqliteDataSaver {
    async fn save_enriched_movies(
        &mut self,
        enriched_movies: impl Stream<Item = Result<CompleteEnrichedMovie>>,
    ) -> Vec<(Result<()>, Result<()>)> {
        let movies: Vec<Result<CompleteEnrichedMovie>> = enriched_movies.collect().await;
        let mut output = Vec::new();
        for movie in movies {
            match movie {
                Ok(m) => {
                    println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAA \n {}", m.movie.title());
                    let pushed_persons = self.data_saver.push_persons(m.persons);
                    let pushed_movie = self.data_saver.push_movie_data(&m.movie, &m.credits);
                    output.push((pushed_persons, pushed_movie));
                }
                Err(e) => {
                    //TODO
                }
            }
        }
        return output;
    }
}

impl SqliteDataSaver {
    pub fn new() -> Result<Self> {
        let mut data_saver = DataSaver::new("movie_db.db".to_string())
            .context("Failed to create database connection")?;
        data_saver.create_movie_table()?;
        data_saver.create_person_table()?;
        data_saver.create_genre_table()?;
        data_saver.create_movie_genre_table()?;
        data_saver.create_credits_table()?;
        Ok(Self { data_saver })
    }
}
