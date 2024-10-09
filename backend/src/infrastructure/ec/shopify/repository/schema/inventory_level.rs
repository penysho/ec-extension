use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        error::error::DomainError,
        inventory_level::{
            inventory_change::inventory_change::{InventoryChange, InventoryChangeReason},
            inventory_level::InventoryLevel,
            quantity::quantity::{InventoryType, Quantity},
        },
    },
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

use super::{common::UserErrors, location::LocationNode};

impl InventoryLevelNode {
    pub fn to_domain(self) -> Result<InventoryLevel, DomainError> {
        let quantities: Result<Vec<Quantity>, DomainError> = self
            .quantities
            .into_iter()
            .map(|quantity| quantity.to_domain())
            .collect();

        InventoryLevel::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.item.id),
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.location.id),
            quantities?,
        )
    }

    pub fn to_domains(schemas: Vec<Self>) -> Result<Vec<InventoryLevel>, DomainError> {
        schemas
            .into_iter()
            .map(|schema| schema.to_domain())
            .collect()
    }
}

impl QuantityNode {
    pub fn to_domain(self) -> Result<Quantity, DomainError> {
        let inventory_type = match self.name.as_str() {
            "available" => InventoryType::Available,
            "incoming" => InventoryType::Incoming,
            "committed" => InventoryType::Committed,
            "damaged" => InventoryType::Damaged,
            "safety_stock" => InventoryType::SafetyStock,
            "reserved" => InventoryType::Reserved,
            _ => return Err(DomainError::ValidationError),
        };

        Quantity::new(self.quantity, inventory_type)
    }
}

#[derive(Debug, Deserialize)]
pub struct InventoryLevelNode {
    pub id: String,
    pub item: InventoryItemIdNode,
    pub location: LocationNode,
    pub quantities: Vec<QuantityNode>,
}

#[derive(Debug, Deserialize)]
pub struct QuantityNode {
    pub quantity: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemIdNode {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct InventoryChangeInput {
    pub delta: i32,
    #[serde(rename = "inventoryItemId")]
    pub inventory_item_id: String,
    #[serde(rename = "ledgerDocumentUri")]
    pub ledger_document_uri: Option<String>,
    #[serde(rename = "locationId")]
    pub location_id: String,
}

#[derive(Debug, Serialize)]
pub struct InventoryAdjustQuantitiesInput {
    pub changes: Vec<InventoryChangeInput>,
    pub name: String,
    pub reason: String,
}

impl From<InventoryType> for String {
    fn from(inventory_type: InventoryType) -> Self {
        match inventory_type {
            InventoryType::Available => "available",
            InventoryType::Incoming => "incoming",
            InventoryType::Committed => "committed",
            InventoryType::Damaged => "damaged",
            InventoryType::SafetyStock => "safety_stock",
            InventoryType::Reserved => "reserved",
        }
        .to_string()
    }
}

impl From<InventoryChangeReason> for String {
    fn from(reason: InventoryChangeReason) -> Self {
        match reason {
            InventoryChangeReason::Correction => "correction",
            InventoryChangeReason::CycleCountAvailable => "cycle_count_available",
            InventoryChangeReason::Damaged => "damaged",
            InventoryChangeReason::MovementCreated => "movement_created",
            InventoryChangeReason::MovementUpdated => "movement_updated",
            InventoryChangeReason::MovementReceived => "movement_received",
            InventoryChangeReason::MovementCanceled => "movement_canceled",
            InventoryChangeReason::Other => "other",
            InventoryChangeReason::Promotion => "promotion",
            InventoryChangeReason::QualityControl => "quality_control",
            InventoryChangeReason::Received => "received",
            InventoryChangeReason::ReservationCreated => "reservation_created",
            InventoryChangeReason::ReservationDeleted => "reservation_deleted",
            InventoryChangeReason::ReservationUpdated => "reservation_updated",
        }
        .to_string()
    }
}

impl From<InventoryChange> for InventoryAdjustQuantitiesInput {
    fn from(domain: InventoryChange) -> Self {
        InventoryAdjustQuantitiesInput {
            changes: domain
                .changes()
                .into_iter()
                .map(|change| InventoryChangeInput {
                    delta: *change.delta(),
                    inventory_item_id: ShopifyGQLQueryHelper::add_inventory_item_gid_prefix(
                        change.inventory_item_id(),
                    ),
                    ledger_document_uri: change
                        .ledger_document_uri()
                        .as_ref()
                        .and_then(|l| Some(l.value().to_string())),
                    location_id: ShopifyGQLQueryHelper::add_location_gid_prefix(
                        change.location_id(),
                    ),
                })
                .collect(),
            name: domain.name().to_owned().into(),
            reason: domain.reason().to_owned().into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct InventoryAdjustQuantitiesData {
    #[serde(rename = "inventoryAdjustQuantities")]
    pub inventory_adjust_quantities: UserErrors,
}
