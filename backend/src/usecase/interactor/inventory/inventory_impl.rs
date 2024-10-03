use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        inventory_item::inventory_item::{Id as InventoryItemId, InventoryItem},
        inventory_level::inventory_level::InventoryLevel,
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
    /// Retrieve inventory information for all locations based on product ID.
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
        let location_ids = self.location_repository.get_all_location_ids().await?;

        // TODO: GetInventoriesQuery::ProductId
        match query {
            GetInventoriesQuery::Sku(sku) => {
                let inventory_items = self
                    .inventory_item_repository
                    .get_inventory_item_by_sku(sku)
                    .await?;

                let mut inventory_levels: Vec<InventoryLevel> = Vec::new();
                for location_id in location_ids {
                    let inventory_level = self
                        .inventory_level_repository
                        .get_inventory_level_by_sku(sku, &location_id)
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
}
