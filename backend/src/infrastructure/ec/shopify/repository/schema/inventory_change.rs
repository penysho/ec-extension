use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        error::error::DomainError,
        inventory_level::{
            inventory_change::inventory_change::{InventoryChange, InventoryChangeReason},
            inventory_level::InventoryLevel,
            quantity::quantity::InventoryType,
        },
    },
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

use super::{
    common::UserError, inventory_item::InventoryItemNode, inventory_level::InventoryLevelNode,
};

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

#[derive(Debug, Serialize)]
pub struct InventoryAdjustQuantitiesInput {
    pub changes: Vec<InventoryChangeInput>,
    pub name: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryChangeInput {
    pub delta: i32,
    pub inventory_item_id: String,
    pub ledger_document_uri: Option<String>,
    pub location_id: String,
}

impl InventoryAdjustmentGroupNode {
    pub fn to_inventory_level_domains(self) -> Result<Vec<InventoryLevel>, DomainError> {
        let changes = self
            .changes
            .into_iter()
            .map(|c| {
                let level_nodes = c
                    .item
                    .inventory_levels
                    .edges
                    .into_iter()
                    .map(|node| node.node)
                    .collect();
                InventoryLevelNode::to_domains(level_nodes)
            })
            .collect::<Result<Vec<Vec<_>>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        Ok(changes)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryAdjustQuantitiesData {
    pub inventory_adjust_quantities: InventoryAdjustQuantities,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryAdjustQuantities {
    pub inventory_adjustment_group: Option<InventoryAdjustmentGroupNode>,
    pub user_errors: Vec<UserError>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryAdjustmentGroupNode {
    pub changes: Vec<InventoryChangeNode>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryChangeNode {
    pub item: InventoryItemNode,
}
