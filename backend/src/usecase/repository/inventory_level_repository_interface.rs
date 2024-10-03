use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError, inventory_level::inventory_level::InventoryLevel,
    location::location::Id as LocationId, product::variant::sku::sku::Sku,
};

#[async_trait]
pub trait InventoryLevelRepository: Send + Sync {
    async fn get_inventory_level_by_sku(
        &self,
        sku: &Sku,
        location_id: &LocationId,
    ) -> Result<InventoryLevel, DomainError>;
}
