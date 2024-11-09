use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError,
    inventory_item::inventory_item::InventoryItem,
    product::{product::Id as ProductId, variant::sku::sku::Sku},
};

/// Repository interface for inventory items.
#[allow(dead_code)]
#[async_trait]
pub trait InventoryItemRepository: Send + Sync {
    /// Get product inventory information by product id.
    async fn find_inventory_items_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<InventoryItem>, DomainError>;

    /// Get product inventory information by sku.
    async fn find_inventory_item_by_sku(&self, sku: &Sku) -> Result<InventoryItem, DomainError>;
}
