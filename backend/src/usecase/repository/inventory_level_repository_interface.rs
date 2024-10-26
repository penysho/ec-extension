use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError,
    inventory_level::{
        inventory_change::inventory_change::InventoryChange, inventory_level::InventoryLevel,
    },
    location::location::Id as LocationId,
    product::variant::sku::sku::Sku,
};

#[async_trait]
pub trait InventoryLevelRepository: Send + Sync {
    async fn find_inventory_level_by_sku_with_location_id(
        &self,
        sku: &Sku,
        location_id: &LocationId,
    ) -> Result<Option<InventoryLevel>, DomainError>;

    async fn update(
        &self,
        inventory_change: InventoryChange,
    ) -> Result<InventoryLevel, DomainError>;
}
