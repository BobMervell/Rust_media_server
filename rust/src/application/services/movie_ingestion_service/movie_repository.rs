use crate::{
    api::media::PersonData,
    application::abstractions::abstractions::MovieRepository,
    domain::{
        movie::detailed_movie::{DetailedMovie, EnrichedMovie},
        person::credits::CreditsMovie,
    },
    infrastructure::db_infra::sqlite_data_saver::DataSaver,
};
use anyhow::{Context, Result};
use futures::StreamExt;
use trpl::Stream;
pub struct SqliteDataSaver {
    data_saver: DataSaver,
}

impl MovieRepository for SqliteDataSaver {
    async fn save_enriched_movies(
        &mut self,
        enriched_movies: impl Stream<Item = Result<EnrichedMovie>>,
    ) -> Vec<Result<()>> {
        let mut movies: Vec<Result<EnrichedMovie>> = enriched_movies.collect().await;
        let mut output = Vec::new();
        for m in movies.iter_mut().filter_map(|m| m.as_mut().ok()) {
            let pushed_movie = self.push_data(&mut m.movie, &m.credits, &m.persons);
            output.push(pushed_movie);
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

    fn push_data(
        &mut self,
        m: &mut DetailedMovie,
        c: &CreditsMovie,
        p: &Vec<PersonData>,
    ) -> Result<()> {
        let tx = self
            .data_saver
            .conn
            .transaction()
            .context("Failed to open database transaction")?;
        let movie_id = DataSaver::push_movie(&m, &tx)
            .with_context(|| format!("Failed to push movie data for {} ", m.file_path(),))?;
        m.set_id(movie_id);

        DataSaver::push_genre(&m, &tx)
            .with_context(|| format!("Failed to push genre data for {} ", m.file_path(),))?;

        DataSaver::push_movie_genre(&m, &tx)
            .with_context(|| format!("Failed to push movie_genre data for {} ", m.file_path(),))?;

        DataSaver::push_persons(p, &tx)
            .with_context(|| format!("Failed to push persons data for {} ", m.file_path(),))?;

        DataSaver::push_credits(m.id(), &c, &tx)
            .with_context(|| format!("Failed to push credits data for {} ", m.file_path(),))?;
        tx.commit()
            .context("Failed to commit data insertion into movie table")?;

        tracing::debug!(file_path = &m.file_path(), "Movie data saved and ready");
        Ok(())
    }
}
