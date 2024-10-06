use actix_web::{
    web::{self, Path},
    Responder,
};
use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        inventory_level::{
            inventory_change::{
                change::ledger_document_uri::ledger_document_uri::LedgerDocumentUri,
                inventory_change::InventoryChangeReason,
            },
            quantity::quantity::InventoryType,
        },
        product::variant::sku::sku::Sku,
    },
    interface::presenter::{
        inventory::inventory_impl::InventoryPresenterImpl,
        inventory_presenter_interface::InventoryPresenter,
    },
};

use super::controller::Controller;

#[derive(Deserialize)]
pub struct PutInventoryQuantityBySkuRequest {
    name: String,
    reason: String,
    delta: i32,
    ledger_document_uri: Option<String>,
    location_id: String,
}

impl Controller {
    /// Update the inventory of the specified SKU.
    pub async fn put_inventory_quantity_by_sku(
        &self,
        path: Path<(String,)>,
        body: web::Json<PutInventoryQuantityBySkuRequest>,
    ) -> impl Responder {
        let presenter = InventoryPresenterImpl::new();

        let sku = match Sku::new(path.into_inner().0) {
            Ok(sku) => sku,
            Err(err) => return presenter.present_put_inventory(Err(err)).await,
        };

        let name = match body.name.as_str() {
            "available" => InventoryType::Available,
            "incoming" => InventoryType::Incoming,
            "committed" => InventoryType::Committed,
            "damaged" => InventoryType::Damaged,
            "safety_stock" => InventoryType::SafetyStock,
            "reserved" => InventoryType::Reserved,
            _ => {
                log::error!("Invalid inventory type: {}", body.name);
                return presenter
                    .present_put_inventory(Err(DomainError::InvalidRequest))
                    .await;
            }
        };

        let reason = match body.reason.as_str() {
            "correction" => InventoryChangeReason::Correction,
            "cycle_count_available" => InventoryChangeReason::CycleCountAvailable,
            "damaged" => InventoryChangeReason::Damaged,
            "movement_created" => InventoryChangeReason::MovementCreated,
            "movement_updated" => InventoryChangeReason::MovementUpdated,
            "movement_received" => InventoryChangeReason::MovementReceived,
            "movement_canceled" => InventoryChangeReason::MovementCanceled,
            "other" => InventoryChangeReason::Other,
            "promotion" => InventoryChangeReason::Promotion,
            "quality_control" => InventoryChangeReason::QualityControl,
            "received" => InventoryChangeReason::Received,
            "reservation_created" => InventoryChangeReason::ReservationCreated,
            "reservation_deleted" => InventoryChangeReason::ReservationDeleted,
            "reservation_updated" => InventoryChangeReason::ReservationUpdated,
            _ => {
                log::error!("Invalid inventory change reason: {}", body.reason);
                return presenter
                    .present_put_inventory(Err(DomainError::InvalidRequest))
                    .await;
            }
        };

        if let Some(ledger_document_uri_str) = body.ledger_document_uri.as_ref() {
            if let Err(err) = LedgerDocumentUri::new(ledger_document_uri_str.to_string()) {
                return presenter.present_put_inventory(Err(err)).await;
            }
        }
        let ledger_document_uri = body
            .ledger_document_uri
            .as_ref()
            .map(|uri| LedgerDocumentUri::new(uri).unwrap());

        let interactor = self.interact_provider.provide_inventory_interactor().await;

        let result = interactor
            .allocate_inventory_by_sku_with_location(
                &sku,
                &name,
                &reason,
                body.delta,
                &ledger_document_uri,
                &body.location_id,
            )
            .await;

        presenter.present_put_inventory(result).await
    }
}
