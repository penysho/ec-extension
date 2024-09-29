use crate::domain::error::error::DomainError;
use crate::domain::inventory::inventory::Inventory;
use crate::domain::product::product::Id as ProductId;
use async_trait::async_trait;
use mockall::automock;

/// Interactor interface for products.

#[automock]
#[async_trait]
pub trait InventoryInteractor {
    async fn get_inventories_from_all_locations_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<Inventory>, DomainError>;
}
