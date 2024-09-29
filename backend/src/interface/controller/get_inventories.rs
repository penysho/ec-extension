use actix_web::{web, Responder};
use serde::Deserialize;

use crate::{
    domain::error::error::DomainError,
    interface::presenter::{
        inventory::inventory_impl::InventoryPresenterImpl,
        inventory_presenter_interface::InventoryPresenter,
    },
};

use super::controller::Controller;

#[derive(Deserialize)]
pub struct GetInventoriesQueryParams {
    product_id: Option<String>,
}

impl Controller {
    /// Get a list of inventories.
    pub async fn get_inventories(
        &self,
        params: web::Query<GetInventoriesQueryParams>,
    ) -> impl Responder {
        let presenter = InventoryPresenterImpl::new();

        // TODO: Allow SKUs to be retrieved by query parameter.
        let product_id = match params.product_id.clone() {
            Some(product_id) if !product_id.is_empty() => product_id,
            Some(_) => {
                return presenter
                    .present_get_inventories(Err(DomainError::InvalidRequest))
                    .await
            }
            None => {
                return presenter
                    .present_get_inventories(Err(DomainError::InvalidRequest))
                    .await
            }
        };

        let interactor = self.interact_provider.provide_inventory_interactor().await;
        let results = interactor
            .get_inventories_from_all_locations_by_product_id(&product_id)
            .await;

        presenter.present_get_inventories(results).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::inventory::inventory::Inventory;
    use crate::domain::inventory::inventory_level::inventory_level::InventoryLevel;
    use crate::domain::inventory::inventory_level::quantity::quantity::{InventoryType, Quantity};
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
    use chrono::Utc;
    use mockall::predicate::eq;

    const BASE_URL: &'static str = "/ec-extension/inventories";

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
    async fn test_get_inventories_success() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_get_inventories_from_all_locations_by_product_id()
            .with(eq("0".to_string()))
            .returning(|_| {
                Ok(vec![Inventory::new(
                    format!("0"),
                    format!("0"),
                    Some(
                        InventoryLevel::new(
                            format!("0"),
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
                        .unwrap(),
                    ),
                    true,
                    false,
                    Utc::now(),
                    Utc::now(),
                )
                .unwrap()])
            });

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id=0"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_inventories_not_specified_product_id() {
        let interactor = MockInventoryInteractor::new();

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id="))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_inventories_not_found() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_get_inventories_from_all_locations_by_product_id()
            .returning(|_| Ok(vec![]));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id=0"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_inventories_bad_request() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_get_inventories_from_all_locations_by_product_id()
            .returning(|_| Err(DomainError::ValidationError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id=0"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_inventories_service_unavailable() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_get_inventories_from_all_locations_by_product_id()
            .returning(|_| Err(DomainError::SystemError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id=0"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
