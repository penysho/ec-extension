use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError,
    location::location::{Id as LocationId, Location},
};

#[async_trait]
pub trait LocationRepository: Send + Sync {
    async fn find_all_location_ids(&self) -> Result<Vec<LocationId>, DomainError>;

    async fn find_locations(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<Vec<Location>, DomainError>;
}
