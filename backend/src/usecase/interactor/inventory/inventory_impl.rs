use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        authorized_resource::authorized_resource::{
            AuthorizedResource, Resource, ResourceAction, ResourceType,
        },
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
        user::user::UserInterface,
    },
    log_error,
    usecase::{
        auth::authorizer_interface::Authorizer,
        interactor::inventory_interactor_interface::{GetInventoriesQuery, InventoryInteractor},
        repository::{
            inventory_item_repository_interface::InventoryItemRepository,
            inventory_level_repository_interface::InventoryLevelRepository,
        },
    },
};

/// Inventory Interactor.
pub struct InventoryInteractorImpl {
    inventory_item_repository: Box<dyn InventoryItemRepository>,
    inventory_level_repository: Box<dyn InventoryLevelRepository>,
    authorizer: Arc<dyn Authorizer>,
}

impl InventoryInteractorImpl {
    pub fn new(
        inventory_item_repository: Box<dyn InventoryItemRepository>,
        inventory_level_repository: Box<dyn InventoryLevelRepository>,
        authorizer: Arc<dyn Authorizer>,
    ) -> Self {
        Self {
            inventory_item_repository: inventory_item_repository,
            inventory_level_repository: inventory_level_repository,
            authorizer: authorizer,
        }
    }
}

#[async_trait]
impl InventoryInteractor for InventoryInteractorImpl {
    async fn get_inventories_from_all_locations(
        &self,
        user: Arc<dyn UserInterface>,
        query: &GetInventoriesQuery,
    ) -> Result<
        (
            Vec<InventoryItem>,
            HashMap<InventoryItemId, Vec<InventoryLevel>>,
        ),
        DomainError,
    > {
        // TODO: GetInventoriesQuery::ProductId
        match query {
            GetInventoriesQuery::Sku(sku) => {
                let inventory_items = self
                    .inventory_item_repository
                    .find_inventory_item_by_sku(sku)
                    .await?;

                // Check authorization for inventory read access
                self.authorizer
                    .authorize(
                        user.clone(),
                        vec![&inventory_items as &dyn AuthorizedResource],
                        &ResourceAction::Read,
                    )
                    .await?;

                let inventory_levels = self
                    .inventory_level_repository
                    .find_inventory_levels_by_sku(sku)
                    .await?;

                let mut inventory_levels_map = HashMap::new();
                inventory_levels_map.insert(inventory_items.id().clone(), inventory_levels);

                Ok((vec![inventory_items], inventory_levels_map))
            }
            _ => Err(DomainError::InvalidRequest),
        }
    }

    async fn allocate_inventory_by_sku_with_location(
        &self,
        user: Arc<dyn UserInterface>,
        sku: &Sku,
        name: &InventoryType,
        reason: &InventoryChangeReason,
        delta: i32,
        ledger_document_uri: &Option<LedgerDocumentUri>,
        location_id: &LocationId,
    ) -> Result<InventoryLevel, DomainError> {
        let inventory_level = self
            .inventory_level_repository
            .find_inventory_level_by_sku_with_location_id(sku, location_id)
            .await?
            .ok_or_else(|| {
                log_error!(
                    "InventoryLevel for the specified SKU is not found.",
                    "SKU" => sku.value(),
                    "LocationId" => location_id
                );
                DomainError::NotFound
            })?;

        // Check authorization for inventory write access
        self.authorizer
            .authorize(
                user.clone(),
                vec![&inventory_level as &dyn AuthorizedResource],
                &ResourceAction::Write,
            )
            .await?;

        let inventory_change =
            inventory_level.create_inventory_change(name, reason, delta, ledger_document_uri)?;

        self.inventory_level_repository
            .update(inventory_change)
            .await
    }
}
