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
    log_error,
};

use super::{controller::Controller, interactor_provider_interface::InteractorProvider};

#[derive(Serialize, Deserialize)]
pub struct PutInventoryQuantityBySkuRequest {
    name: String,
    reason: String,
    delta: i32,
    ledger_document_uri: Option<String>,
    location_id: String,
}

impl<I, T, C> Controller<I, T, C>
where
    I: InteractorProvider<T, C>,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Update the inventory of the specified SKU.
    pub async fn put_inventory_quantity_by_sku(
        &self,
        request: actix_web::HttpRequest,
        path: Path<(String,)>,
        body: web::Json<PutInventoryQuantityBySkuRequest>,
    ) -> impl Responder {
        let presenter = InventoryPresenterImpl::new();

        let sku = Sku::new(path.into_inner().0)?;

        let name = match body.name.as_str() {
            "available" => InventoryType::Available,
            "incoming" => InventoryType::Incoming,
            "committed" => InventoryType::Committed,
            "damaged" => InventoryType::Damaged,
            "safety_stock" => InventoryType::SafetyStock,
            "reserved" => InventoryType::Reserved,
            _ => {
                log_error!("Invalid inventory type.", "name" => body.name.clone());
                Err(DomainError::InvalidRequest)?
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
                log_error!("Invalid inventory change reason.", "reason" => body.reason.clone());
                Err(DomainError::InvalidRequest)?
            }
        };

        let ledger_document_uri = body
            .ledger_document_uri
            .as_ref()
            .map(|uri| LedgerDocumentUri::new(uri))
            .transpose()?;

        let user = self.get_user(&request)?;
        let transaction_manager = self.get_transaction_manager(&request)?;

        let interactor = self
            .interactor_provider
            .provide_inventory_interactor(transaction_manager)
            .await;

        let result = interactor
            .allocate_inventory_by_sku_with_location(
                user,
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

    use crate::domain::inventory_level::quantity::quantity::InventoryType;
    use crate::domain::user::user::UserInterface;
    use crate::infrastructure::auth::idp_user::IdpUser;
    use crate::infrastructure::db::sea_orm::sea_orm_manager::SeaOrmTransactionManager;
    use crate::infrastructure::db::transaction_manager_interface::TransactionManager;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interactor_provider_interface::MockInteractorProvider;
    use crate::interface::mock::domain_mock::mock_inventory_levels;
    use crate::usecase::interactor::inventory_interactor_interface::{
        InventoryInteractor, MockInventoryInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error, HttpMessage};
    use mockall::predicate::{always, eq};
    use sea_orm::{DatabaseConnection, DatabaseTransaction};

    const BASE_URL: &'static str = "/ec-extension/inventories/quantities/sku";

    async fn setup(
        interactor: MockInventoryInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interactor_provider =
            MockInteractorProvider::<DatabaseTransaction, Arc<DatabaseConnection>>::new();
        interactor_provider
            .expect_provide_inventory_interactor()
            .return_once(move |_| Box::new(interactor) as Box<dyn InventoryInteractor>);

        let controller = web::Data::new(Controller::new(interactor_provider));

        // Create an application for testing
        test::init_service(App::new().app_data(controller).configure(
            actix_router::configure_routes::<
                MockInteractorProvider<DatabaseTransaction, Arc<DatabaseConnection>>,
                DatabaseTransaction,
                Arc<DatabaseConnection>,
            >,
        ))
        .await
    }

    fn add_extensions(req: &Request) {
        req.extensions_mut()
            .insert(Arc::new(IdpUser::default()) as Arc<dyn UserInterface>);
        req.extensions_mut()
            .insert(Arc::new(SeaOrmTransactionManager::default())
                as Arc<
                    dyn TransactionManager<DatabaseTransaction, Arc<DatabaseConnection>>,
                >);
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
                always(),
                eq(Sku::new(sku).unwrap()),
                eq(InventoryType::Available),
                eq(InventoryChangeReason::Correction),
                eq(delta),
                eq(ledger_document_uri),
                eq(location_id.to_string()),
            )
            .returning(|_, _, _, _, _, _, _| Ok(mock_inventory_levels(1).remove(0)));

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
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_put_inventory_quantity_by_sku_not_found() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_allocate_inventory_by_sku_with_location()
            .returning(|_, _, _, _, _, _, _| Err(DomainError::NotFound));

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
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_put_inventory_quantity_by_sku_bad_request() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_allocate_inventory_by_sku_with_location()
            .returning(|_, _, _, _, _, _, _| Err(DomainError::ValidationError));

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
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_put_inventory_quantity_by_sku_service_unavailable() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_allocate_inventory_by_sku_with_location()
            .returning(|_, _, _, _, _, _, _| Err(DomainError::SystemError));

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
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
