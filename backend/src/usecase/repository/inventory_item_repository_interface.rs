use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError,
    inventory_item::inventory_item::InventoryItem,
    product::{product::Id as ProductId, variant::sku::sku::Sku},
};

#[async_trait]
pub trait InventoryItemRepository: Send + Sync {
    async fn find_inventory_items_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<InventoryItem>, DomainError>;

    async fn find_inventory_item_by_sku(&self, sku: &Sku) -> Result<InventoryItem, DomainError>;
}
