use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError,
    inventory_level::{
        inventory_change::inventory_change::InventoryChange, inventory_level::InventoryLevel,
    },
    location::location::Id as LocationId,
    product::variant::sku::sku::Sku,
};

/// Repository interface for inventory levels.
#[async_trait]
pub trait InventoryLevelRepository: Send + Sync {
    /// Get inventory level information by sku with location id.
    async fn find_inventory_level_by_sku_with_location_id(
        &self,
        sku: &Sku,
        location_id: &LocationId,
    ) -> Result<Option<InventoryLevel>, DomainError>;

    /// Get inventory level information by sku with location id.
    async fn find_inventory_levels_by_sku(
        &self,
        sku: &Sku,
    ) -> Result<Vec<InventoryLevel>, DomainError>;

    /// Update inventory quantity.
    async fn update(
        &self,
        inventory_change: InventoryChange,
    ) -> Result<InventoryLevel, DomainError>;
}
