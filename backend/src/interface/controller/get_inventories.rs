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

use super::{controller::Controller, interactor_provider_interface::InteractorProvider};

#[derive(Deserialize)]
pub struct GetInventoriesQueryParams {
    product_id: Option<String>,
    sku: Option<String>,
}

impl<I, T, C> Controller<I, T, C>
where
    I: InteractorProvider<T, C>,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Get a list of inventories.
    pub async fn get_inventories(
        &self,
        request: actix_web::HttpRequest,
        params: web::Query<GetInventoriesQueryParams>,
    ) -> impl Responder {
        let presenter = InventoryPresenterImpl::new();

        let query = match validate_query_params(&params) {
            Ok(query) => query,
            Err(error) => return presenter.present_get_inventories(Err(error)).await,
        };

        let user = self.get_user(&request)?;
        let transaction_manager = self.get_transaction_manager(&request)?;

        let interactor = self
            .interactor_provider
            .provide_inventory_interactor(transaction_manager)
            .await;
        let results = interactor
            .get_inventories_from_all_locations(user, &query)
            .await;

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

    use crate::domain::user::user::UserInterface;
    use crate::infrastructure::auth::idp_user::IdpUser;
    use crate::infrastructure::db::sea_orm::sea_orm_manager::SeaOrmTransactionManager;
    use crate::infrastructure::db::transaction_manager_interface::TransactionManager;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interactor_provider_interface::MockInteractorProvider;
    use crate::interface::mock::domain_mock::{mock_inventory_items, mock_inventory_level_map};
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

    const BASE_URL: &'static str = "/ec-extension/inventories";

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
    async fn test_get_inventories_success() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_get_inventories_from_all_locations()
            .with(
                always(),
                eq(GetInventoriesQuery::ProductId("0".to_string())),
            )
            .returning(|_, _| {
                Ok((
                    mock_inventory_items(1),
                    mock_inventory_level_map(5, &"0".to_string()),
                ))
            });

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id=0"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_inventories_not_specified_product_id() {
        let interactor = MockInventoryInteractor::new();

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id="))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_inventories_not_found() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_get_inventories_from_all_locations()
            .returning(|_, _| Ok((vec![], HashMap::new())));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id=0"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_inventories_bad_request() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_get_inventories_from_all_locations()
            .returning(|_, _| Err(DomainError::ValidationError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id=0"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_inventories_service_unavailable() {
        let mut interactor = MockInventoryInteractor::new();
        interactor
            .expect_get_inventories_from_all_locations()
            .returning(|_, _| Err(DomainError::SystemError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?product_id=0"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
