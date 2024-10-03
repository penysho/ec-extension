use std::collections::HashMap;

use async_trait::async_trait;
use mockall::automock;

use crate::domain::error::error::DomainError;
use crate::domain::inventory_item::inventory_item::{Id as InventoryItemId, InventoryItem};
use crate::domain::inventory_level::inventory_level::InventoryLevel;
use crate::domain::product::product::Id as ProductId;
use crate::domain::product::variant::sku::sku::Sku;

#[derive(Debug, Clone, PartialEq)]
pub enum GetInventoriesQuery {
    ProductId(ProductId),
    Sku(Sku),
}

/// Interactor interface for products.
#[automock]
#[async_trait]
pub trait InventoryInteractor {
    async fn get_inventories_from_all_locations(
        &self,
        query: &GetInventoriesQuery,
    ) -> Result<
        (
            Vec<InventoryItem>,
            HashMap<InventoryItemId, Vec<InventoryLevel>>,
        ),
        DomainError,
    >;
}
