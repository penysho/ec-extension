use std::collections::HashMap;

use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError,
    inventory_item::inventory_item::{Id as InventoryItemId, InventoryItem},
    inventory_level::inventory_level::InventoryLevel,
};

/// Interface to generate response schema for inventories.
#[async_trait]
pub trait InventoryPresenter {
    type GetInventoriesResponse;
    type GetInventoriesErrorResponse;
    /// Generate a list response of inventory information.
    async fn present_get_inventories(
        &self,
        result: Result<
            (
                Vec<InventoryItem>,
                HashMap<InventoryItemId, Vec<InventoryLevel>>,
            ),
            DomainError,
        >,
    ) -> Result<Self::GetInventoriesResponse, Self::GetInventoriesErrorResponse>;

    type PutInventoryResponse;
    type PutInventoryErrorResponse;
    /// Generate an update response for inventory information.
    async fn present_put_inventory(
        &self,
        result: Result<InventoryLevel, DomainError>,
    ) -> Result<Self::PutInventoryResponse, Self::PutInventoryErrorResponse>;
}
