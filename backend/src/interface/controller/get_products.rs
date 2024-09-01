use crate::interface::presenter::product::product_impl::ProductPresenterImpl;
use crate::interface::{
    controller::controller::Controller, presenter::product_presenter_interface::ProductPresenter,
};
use actix_web::{web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetProductsQueryParams {
    limit: Option<u32>,
    offset: Option<u32>,
}

impl Controller {
    /// Obtain a list of products.
    pub async fn get_products(&self, params: web::Query<GetProductsQueryParams>) -> impl Responder {
        let interactor = self.interact_provider.provide_product_interactor().await;
        let products = interactor.get_products(&params.limit, &params.offset).await;

        let presenter = ProductPresenterImpl::new();
        presenter.present_get_products(products).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::error::error::DomainError;
    use crate::domain::media::media::{Media, MediaStatus};
    use crate::domain::product::product::{Product, ProductStatus};
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::interface::presenter::product::schema::GetProductsResponse;
    use crate::usecase::interactor::product_interactor_interface::{
        MockProductInteractor, ProductInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};

    const BASE_URL: &'static str = "/ec-extension/products";

    async fn setup(
        interactor: MockProductInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the InteractProvider mock
        let mut interact_provider = MockInteractProvider::new();
        interact_provider
            .expect_provide_product_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn ProductInteractor>);

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
    async fn test_get_products_success() {
        let mut interactor = MockProductInteractor::new();
        interactor.expect_get_products().returning(|_, _| {
            Ok(vec![
                Product::new(
                    "1".to_string(),
                    "Test Product 1".to_string(),
                    100,
                    "Description".to_string(),
                    ProductStatus::Active,
                    Some("1".to_string()),
                    vec![Media::new(
                        "1".to_string(),
                        "Test Media 1".to_string(),
                        MediaStatus::Active,
                        Some("https://example.com/image.jpg".to_string()),
                    )
                    .unwrap()],
                )
                .unwrap(),
                Product::new(
                    "2".to_string(),
                    "Test Product 2".to_string(),
                    200,
                    "Description 2".to_string(),
                    ProductStatus::Active,
                    Some("2".to_string()),
                    vec![Media::new(
                        "2".to_string(),
                        "Test Media 2".to_string(),
                        MediaStatus::Active,
                        Some("https://example.com/image.jpg".to_string()),
                    )
                    .unwrap()],
                )
                .unwrap(),
            ])
        });

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let products: GetProductsResponse = test::read_body_json(resp).await;
        assert_eq!(products.products.len(), 2);
    }

    #[actix_web::test]
    async fn test_get_products_empty() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_products()
            .returning(|_, _| Ok(vec![]));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let products: GetProductsResponse = test::read_body_json(resp).await;
        assert!(products.products.is_empty());
    }

    #[actix_web::test]
    async fn test_get_products_service_unavailable() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_products()
            .returning(|_, _| Err(DomainError::SystemError));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
