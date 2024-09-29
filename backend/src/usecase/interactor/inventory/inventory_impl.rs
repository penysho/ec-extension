use async_trait::async_trait;

use crate::{
    domain::{error::error::DomainError, inventory::inventory::Inventory},
    usecase::{
        interactor::inventory_interactor_interface::{GetInventoriesQuery, InventoryInteractor},
        repository::{
            inventory_repository_interface::InventoryRepository,
            location_repository_interface::LocationRepository,
        },
    },
};

/// Inventory Interactor.
pub struct InventoryInteractorImpl {
    inventory_repository: Box<dyn InventoryRepository>,
    location_repository: Box<dyn LocationRepository>,
}

impl InventoryInteractorImpl {
    pub fn new(
        inventory_repository: Box<dyn InventoryRepository>,
        location_repository: Box<dyn LocationRepository>,
    ) -> Self {
        Self {
            inventory_repository: inventory_repository,
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
    ) -> Result<Vec<Inventory>, DomainError> {
        let location_ids = self.location_repository.get_all_location_ids().await?;

        // TODO: Process for all locations.
        match query {
            GetInventoriesQuery::ProductId(product_id) => {
                self.inventory_repository
                    .get_inventories_by_product_id(product_id, &location_ids[0])
                    .await
            }
            // TODO: Allow inventory information to be retrieved from SKU.
            _ => Err(DomainError::InvalidRequest),
        }
    }
}
