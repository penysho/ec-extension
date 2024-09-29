use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError,
    inventory::inventory::Inventory,
    location::location::Id as LocationId,
    product::{product::Id as ProductId, variant::sku::sku::Sku},
};

#[async_trait]
pub trait InventoryRepository: Send + Sync {
    async fn get_inventories_by_product_id(
        &self,
        product_id: &ProductId,
        location_id: &LocationId,
    ) -> Result<Vec<Inventory>, DomainError>;

    async fn get_inventories_by_sku(
        &self,
        sku: &Sku,
        location_id: &LocationId,
    ) -> Result<Inventory, DomainError>;
}
