use actix_web::{web, Responder};
use serde::Deserialize;

use crate::{
    domain::{error::error::DomainError, product::variant::sku::sku::Sku},
    interface::presenter::{
        inventory::inventory_impl::InventoryPresenterImpl,
        inventory_presenter_interface::InventoryPresenter,
    },
    usecase::interactor::inventory_interactor_interface::GetInventoriesQuery,
};

use super::controller::Controller;

#[derive(Deserialize)]
pub struct GetInventoriesQueryParams {
    product_id: Option<String>,
    sku: Option<String>,
}

impl Controller {
    /// Get a list of inventories.
    pub async fn get_inventories(
        &self,
        params: web::Query<GetInventoriesQueryParams>,
    ) -> impl Responder {
        let presenter = InventoryPresenterImpl::new();

        let query = match validate_query_params(&params) {
            Ok(query) => query,
            Err(error) => return presenter.present_get_inventories(Err(error)).await,
        };

        let interactor = self.interact_provider.provide_inventory_interactor().await;
        let results = interactor.get_inventories_from_all_locations(&query).await;

        presenter.present_get_inventories(results).await
    }
}

fn validate_query_params(
    params: &GetInventoriesQueryParams,
) -> Result<GetInventoriesQuery, DomainError> {
    if let Some(product_id) = params.product_id.clone() {
        if !product_id.is_empty() {
            return Ok(GetInventoriesQuery::ProductId(product_id));
        }
    }

    if let Some(sku) = params.sku.clone() {
        if let Ok(valid_sku) = Sku::new(sku.clone()) {
            return Ok(GetInventoriesQuery::Sku(valid_sku));
        }
    }

    Err(DomainError::InvalidRequest)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::domain::inventory_item::inventory_item::InventoryItem;
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
            .expect_get_inventories_from_all_locations()
            .with(eq(GetInventoriesQuery::ProductId("0".to_string())))
            .returning(|_| {
                Ok((
                    vec![
                        InventoryItem::new("0", "0", true, false, Utc::now(), Utc::now()).unwrap(),
                    ],
                    vec![(
                        "0".to_string(),
                        vec![InventoryLevel::new(
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
                        .unwrap()],
                    )]
                    .into_iter()
                    .collect(),
                ))
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
            .expect_get_inventories_from_all_locations()
            .returning(|_| Ok((vec![], HashMap::new())));

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
            .expect_get_inventories_from_all_locations()
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
            .expect_get_inventories_from_all_locations()
            .returning(|_| Err(DomainError::SystemError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id=0"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
