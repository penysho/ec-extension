use async_trait::async_trait;

use crate::domain::{error::error::DomainError, inventory::inventory::Inventory};

/// Interface to generate response schema for inventories.
#[async_trait]
pub trait InventoryPresenter {
    type GetInventoriesResponse;
    type GetInventoriesResponseError;
    async fn present_get_inventories(
        &self,
        result: Result<Vec<Inventory>, DomainError>,
    ) -> Result<Self::GetInventoriesResponse, Self::GetInventoriesResponseError>;
}
