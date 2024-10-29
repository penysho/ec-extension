use async_trait::async_trait;
use mockall::automock;

use crate::domain::error::error::DomainError;
use crate::domain::location::location::Location;

/// Interactor interface for locations.
#[automock]
#[async_trait]
pub trait LocationInteractor {
    /// Get a list of locations.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of locations to return
    /// * `offset` - Number of locations to skip
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Location>, DomainError>` - List of locations.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the location repository fails.
    async fn get_locations(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<Vec<Location>, DomainError>;
}
