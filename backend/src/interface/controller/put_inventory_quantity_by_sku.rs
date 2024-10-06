use actix_web::{
    web::{self, Path},
    Responder,
};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::inventory_level::inventory_level::InventoryLevel;
    use crate::domain::inventory_level::quantity::quantity::{InventoryType, Quantity};
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::usecase::interactor::inventory_interactor_interface::{
        InventoryInteractor, MockInventoryInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};
    use mockall::predicate::eq;

    const BASE_URL: &'static str = "/ec-extension/inventories/quantities/sku";

    async fn setup(
        interactor: MockInventoryInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the InteractProvider mock
        let mut interact_provider = MockInteractProvider::new();
        interact_provider
            .expect_provide_inventory_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn InventoryInteractor>);

        let controller = web::Data::new(Arc::new(Controller::new(Box::new(interact_provider))));

        // Create an application for testing
        test::init_service(
            App::new()
                .app_data(controller)
                .configure(actix_router::configure_routes),
        )
        .await
    }

    #[actix_web::test]
    async fn test_put_inventory_quantity_by_sku_success() {
        let sku = "test-sku-1";
        let delta = 10;
        let ledger_document_uri: Option<LedgerDocumentUri> = None;
        let location_id = "location_id";

        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_allocate_inventory_by_sku_with_location()
            .with(
                eq(Sku::new(sku).unwrap()),
                eq(InventoryType::Available),
                eq(InventoryChangeReason::Correction),
                eq(delta),
                eq(ledger_document_uri),
                eq(location_id.to_string()),
            )
            .returning(|_, _, _, _, _, _| {
                InventoryLevel::new(
                    "0",
                    "0",
                    "location_id",
                    vec![
                        Quantity::new(10, InventoryType::Available).unwrap(),
                        Quantity::new(20, InventoryType::Committed).unwrap(),
                        Quantity::new(30, InventoryType::Incoming).unwrap(),
                        Quantity::new(40, InventoryType::Reserved).unwrap(),
                        Quantity::new(50, InventoryType::SafetyStock).unwrap(),
                        Quantity::new(60, InventoryType::Damaged).unwrap(),
                    ],
                )
            });

        let req = test::TestRequest::put()
            .uri(&format!("{BASE_URL}/{sku}"))
            .set_json(PutInventoryQuantityBySkuRequest {
                name: "available".to_string(),
                reason: "correction".to_string(),
                delta: delta,
                ledger_document_uri: None,
                location_id: location_id.to_string(),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_put_inventory_quantity_by_sku_not_found() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_allocate_inventory_by_sku_with_location()
            .returning(|_, _, _, _, _, _| Err(DomainError::NotFound));

        let req = test::TestRequest::put()
            .uri(&format!("{BASE_URL}/test-sku-1"))
            .set_json(PutInventoryQuantityBySkuRequest {
                name: "available".to_string(),
                reason: "correction".to_string(),
                delta: 2,
                ledger_document_uri: None,
                location_id: "location_id".to_string(),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_put_inventory_quantity_by_sku_bad_request() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_allocate_inventory_by_sku_with_location()
            .returning(|_, _, _, _, _, _| Err(DomainError::ValidationError));

        let req = test::TestRequest::put()
            .uri(&format!("{BASE_URL}/test-sku-1"))
            .set_json(PutInventoryQuantityBySkuRequest {
                name: "available".to_string(),
                reason: "correction".to_string(),
                delta: 2,
                ledger_document_uri: None,
                location_id: "location_id".to_string(),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_put_inventory_quantity_by_sku_service_unavailable() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_allocate_inventory_by_sku_with_location()
            .returning(|_, _, _, _, _, _| Err(DomainError::SystemError));

        let req = test::TestRequest::put()
            .uri(&format!("{BASE_URL}/test-sku-1"))
            .set_json(PutInventoryQuantityBySkuRequest {
                name: "available".to_string(),
                reason: "correction".to_string(),
                delta: 2,
                ledger_document_uri: None,
                location_id: "location_id".to_string(),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
