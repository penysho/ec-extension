use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        inventory_item::inventory_item::{Id as InventoryItemId, InventoryItem},
        inventory_level::{
            inventory_change::{
                change::ledger_document_uri::ledger_document_uri::LedgerDocumentUri,
                inventory_change::InventoryChangeReason,
            },
            inventory_level::InventoryLevel,
            quantity::quantity::InventoryType,
        },
        location::location::Id as LocationId,
        product::variant::sku::sku::Sku,
    },
    usecase::{
        interactor::inventory_interactor_interface::{GetInventoriesQuery, InventoryInteractor},
        repository::{
            inventory_item_repository_interface::InventoryItemRepository,
            inventory_level_repository_interface::InventoryLevelRepository,
            location_repository_interface::LocationRepository,
        },
    },
};

/// Inventory Interactor.
pub struct InventoryInteractorImpl {
    inventory_item_repository: Box<dyn InventoryItemRepository>,
    inventory_level_repository: Box<dyn InventoryLevelRepository>,
    location_repository: Box<dyn LocationRepository>,
}

impl InventoryInteractorImpl {
    pub fn new(
        inventory_item_repository: Box<dyn InventoryItemRepository>,
        inventory_level_repository: Box<dyn InventoryLevelRepository>,
        location_repository: Box<dyn LocationRepository>,
    ) -> Self {
        Self {
            inventory_item_repository: inventory_item_repository,
            inventory_level_repository: inventory_level_repository,
            location_repository: location_repository,
        }
    }
}

#[async_trait]
impl InventoryInteractor for InventoryInteractorImpl {
    async fn get_inventories_from_all_locations(
        &self,
        query: &GetInventoriesQuery,
    ) -> Result<
        (
            Vec<InventoryItem>,
            HashMap<InventoryItemId, Vec<InventoryLevel>>,
        ),
        DomainError,
    > {
        let location_ids = self.location_repository.find_all_location_ids().await?;

        // TODO: GetInventoriesQuery::ProductId
        match query {
            GetInventoriesQuery::Sku(sku) => {
                let inventory_items = self
                    .inventory_item_repository
                    .find_inventory_item_by_sku(sku)
                    .await?;

                let mut inventory_levels: Vec<InventoryLevel> = Vec::new();
                for location_id in location_ids {
                    let inventory_level = self
                        .inventory_level_repository
                        .find_inventory_level_by_sku(sku, &location_id)
                        .await?;

                    if let Some(inventory_level) = inventory_level {
                        inventory_levels.push(inventory_level);
                    }
                }

                let mut inventory_levels_map = HashMap::new();
                inventory_levels_map.insert(inventory_items.id().clone(), inventory_levels);

                Ok((vec![inventory_items], inventory_levels_map))
            }
            _ => Err(DomainError::InvalidRequest),
        }
    }

    async fn allocate_inventory_by_sku_with_location(
        &self,
        sku: &Sku,
        name: &InventoryType,
        reason: &InventoryChangeReason,
        delta: i32,
        ledger_document_uri: &Option<LedgerDocumentUri>,
        location_id: &LocationId,
    ) -> Result<InventoryLevel, DomainError> {
        let inventory_level = self
            .inventory_level_repository
            .find_inventory_level_by_sku(sku, location_id)
            .await?
            .ok_or_else(|| {
                log::error!(
                    "InventoryLevel for the specified SKU is not found. SKU: {:?}, LocationId: {}",
                    sku,
                    location_id
                );
                DomainError::NotFound
            })?;

        let inventory_change =
            inventory_level.create_inventory_change(name, reason, delta, ledger_document_uri)?;

        let updated_levels = self
            .inventory_level_repository
            .update(inventory_change)
            .await?;

        Ok(updated_levels.into_iter().next().unwrap())
    }
}
