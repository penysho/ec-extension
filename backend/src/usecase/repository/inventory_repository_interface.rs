use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError, inventory::inventory::Inventory,
    location::location::Id as LocationId, product::product::Id as ProductId,
};

#[async_trait]
pub trait InventoryRepository: Send + Sync {
    async fn get_inventories_by_product_id(
        &self,
        product_id: &ProductId,
        location_id: &LocationId,
    ) -> Result<Vec<Inventory>, DomainError>;
}
