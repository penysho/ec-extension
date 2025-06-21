use crate::interface::presenter::product::product_impl::ProductPresenterImpl;
use crate::interface::{
    controller::controller::Controller, presenter::product_presenter_interface::ProductPresenter,
};
use actix_web::{web, Responder};
use serde::Deserialize;

use super::interactor_provider_interface::InteractorProvider;

#[derive(Deserialize)]
pub struct GetProductsQueryParams {
    limit: Option<u32>,
    offset: Option<u32>,
}

impl<I, T, C> Controller<I, T, C>
where
    I: InteractorProvider<T, C>,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Get a list of products.
    pub async fn get_products(
        &self,
        request: actix_web::HttpRequest,
        params: web::Query<GetProductsQueryParams>,
    ) -> impl Responder {
        let user = self.get_user(&request)?;
        let transaction_manager = self.get_transaction_manager(&request)?;

        let interactor = self
            .interactor_provider
            .provide_product_interactor(transaction_manager)
            .await;
        let results = interactor
            .get_products_with_media(user, &params.limit, &params.offset)
            .await;

        let presenter = ProductPresenterImpl::new();
        presenter.present_get_products(results).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::error::error::DomainError;
    use crate::domain::user::user::UserInterface;
    use crate::infrastructure::auth::idp_user::IdpUser;
    use crate::infrastructure::db::sea_orm::sea_orm_manager::SeaOrmTransactionManager;
    use crate::infrastructure::db::transaction_manager_interface::TransactionManager;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interactor_provider_interface::MockInteractorProvider;
    use crate::interface::mock::domain_mock::{mock_media, mock_products};
    use crate::usecase::interactor::product_interactor_interface::{
        MockProductInteractor, ProductInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::{http::StatusCode, test, App, Error};
    use actix_web::{web, HttpMessage};
    use mockall::predicate::*;
    use sea_orm::{DatabaseConnection, DatabaseTransaction};

    const BASE_URL: &'static str = "/ec-extension/products";

    async fn setup(
        interactor: MockProductInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interactor_provider =
            MockInteractorProvider::<DatabaseTransaction, Arc<DatabaseConnection>>::new();
        interactor_provider
            .expect_provide_product_interactor()
            .return_once(move |_| Box::new(interactor) as Box<dyn ProductInteractor>);

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
    async fn test_get_products_success() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_products_with_media()
            .returning(|_, _, _| Ok((mock_products(2), mock_media(2))));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_products_bad_request() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_products_with_media()
            .returning(|_, _, _| Err(DomainError::ValidationError));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_products_service_unavailable() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_products_with_media()
            .returning(|_, _, _| Err(DomainError::SystemError));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
