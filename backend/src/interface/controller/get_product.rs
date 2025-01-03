use crate::interface::{
    controller::controller::Controller,
    presenter::{
        product::product_impl::ProductPresenterImpl, product_presenter_interface::ProductPresenter,
    },
};
use actix_web::{web::Path, Responder};

impl Controller {
    /// Get detailed product information.
    pub async fn get_product(&self, path: Path<(String,)>) -> impl Responder {
        let id = &path.into_inner().0;

        let product_interactor = self.interact_provider.provide_product_interactor().await;
        let result = product_interactor.get_product_with_media(id).await;

        let presenter = ProductPresenterImpl::new();
        presenter.present_get_product(result).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::error::error::DomainError;
    use crate::interface::controller::authorizer_interface::MockAuthorizer;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::interface::mock::domain_mock::{mock_media, mock_products};
    use crate::usecase::interactor::product_interactor_interface::{
        MockProductInteractor, ProductInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};
    use mockall::predicate::*;

    const BASE_URL: &'static str = "/ec-extension/products";

    async fn setup(
        interactor: MockProductInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interact_provider = MockInteractProvider::new();
        interact_provider
            .expect_provide_product_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn ProductInteractor>);

        let authorizer = MockAuthorizer::new();

        let controller = web::Data::new(Arc::new(Controller::new(
            Box::new(interact_provider),
            Box::new(authorizer),
        )));

        // Create an application for testing
        test::init_service(
            App::new()
                .app_data(controller)
                .configure(actix_router::configure_routes),
        )
        .await
    }

    #[actix_web::test]
    async fn test_get_product_success() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_product_with_media()
            .with(eq("0".to_string()))
            .returning(|_| Ok((mock_products(1).remove(0), mock_media(5))));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}/0"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_product_not_found() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_product_with_media()
            .with(eq("999".to_string()))
            .returning(|_| Err(DomainError::NotFound));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}/999"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_product_bad_request() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_product_with_media()
            .with(eq("0".to_string()))
            .returning(|_| Err(DomainError::ValidationError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}/0"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_product_service_unavailable() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_product_with_media()
            .with(eq("0".to_string()))
            .returning(|_| Err(DomainError::SystemError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}/0"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
