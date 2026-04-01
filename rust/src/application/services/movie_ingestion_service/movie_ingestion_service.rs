use crate::application::abstractions::{
    MediaDiscoveryService, MovieAssetService, MovieFactory, MovieMetadataService, MovieRepository,
};

use anyhow::Result;
use tracing::info_span;

pub struct MovieIngestionService<D, F, M, A, R>
where
    D: MediaDiscoveryService,
    F: MovieFactory,
    M: MovieMetadataService,
    A: MovieAssetService,
    R: MovieRepository,
{
    discovery_service: D,
    movie_factory: F,
    metadata_service: M,
    asset_service: A,
    repository: R,
}

impl<D, F, M, A, R> MovieIngestionService<D, F, M, A, R>
where
    D: MediaDiscoveryService,
    F: MovieFactory,
    M: MovieMetadataService,
    A: MovieAssetService,
    R: MovieRepository,
{
    pub fn new(
        discovery_service: D,
        movie_factory: F,
        metadata_service: M,
        asset_service: A,
        repository: R,
    ) -> Self {
        Self {
            discovery_service,
            movie_factory,
            metadata_service,
            asset_service,
            repository,
        }
    }

    pub async fn ingest_movies(&mut self) -> Result<()> {
        let span = info_span!("fetch_movie_data");
        let _enter = span.enter();
        let path = "";
        let placeholder_path = "";

        let entries = self.discovery_service.get_entries(path);
        let parsed_movies = self.movie_factory.get_movies(entries);
        let detailed_movies = self.metadata_service.get_details(parsed_movies);
        let enriched_movies = self.metadata_service.fetch_credits(detailed_movies);
        let complete_movies = self
            .asset_service
            .get_assets(enriched_movies, placeholder_path);
        self.repository.save_enriched_movies(complete_movies).await;
        Ok(())
    }
}
