use async_trait::async_trait;

use crate::domain::{error::error::DomainError, location::location::Id as LocationId};

#[async_trait]
pub trait LocationRepository: Send + Sync {
    async fn find_all_location_ids(&self) -> Result<Vec<LocationId>, DomainError>;
}
