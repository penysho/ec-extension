use crate::domain::error::error::DomainError;
use crate::domain::inventory::inventory::Inventory;
use crate::domain::product::product::Id as ProductId;
use crate::domain::product::variant::sku::sku::Sku;
use async_trait::async_trait;
use mockall::automock;

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
    ) -> Result<Vec<Inventory>, DomainError>;
}
