use async_trait::async_trait;

use crate::domain::{error::error::DomainError, location::location::Location};

/// Interface to generate response schema for locations.
#[async_trait]
pub trait LocationPresenter {
    type GetLocationsResponse;
    type GetLocationsErrorResponse;
    async fn present_get_locations(
        &self,
        result: Result<Vec<Location>, DomainError>,
    ) -> Result<Self::GetLocationsResponse, Self::GetLocationsErrorResponse>;
}
