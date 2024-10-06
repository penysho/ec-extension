use std::collections::HashMap;

use async_trait::async_trait;
use mockall::automock;

use crate::domain::error::error::DomainError;
use crate::domain::inventory_item::inventory_item::{Id as InventoryItemId, InventoryItem};
use crate::domain::inventory_level::inventory_level::InventoryLevel;
use crate::domain::inventory_level::quantity::quantity::InventoryType;
use crate::domain::product::product::Id as ProductId;
use crate::domain::product::variant::sku::sku::Sku;
use crate::domain::inventory_level::inventory_change::change::ledger_document_uri::ledger_document_uri::LedgerDocumentUri;
use crate::domain::location::location::Id as LocationId;

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

    async fn allocate_inventory_by_sku_with_location(
        &self,
        sku: &Sku,
        name: &InventoryType,
        reason: &str,
        delta: i32,
        ledger_document_uri: &Option<LedgerDocumentUri>,
        location_id: &LocationId,
    ) -> Result<InventoryLevel, DomainError>;
}
