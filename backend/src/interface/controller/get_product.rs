use crate::interface::{
    controller::controller::Controller,
    presenter::{
        product::product_impl::ProductPresenterImpl, product_presenter_interface::ProductPresenter,
    },
};
use actix_web::{web::Path, Responder};

impl Controller {
    /// Obtain detailed product information.
    pub async fn get_product(&self, path: Path<(String,)>) -> impl Responder {
        let id = &path.into_inner().0;

        let interactor = self.interact_provider.provide_product_interactor().await;
        let products = interactor.get_product(id).await;

        let presenter = ProductPresenterImpl::new();
        presenter.present_get_product(products).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::error::error::DomainError;
    use crate::domain::product::product::{Product, ProductStatus};
    use crate::domain::product::variant::barcode::barcode::Barcode;
    use crate::domain::product::variant::sku::sku::Sku;
    use crate::domain::product::variant::variant::Variant;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::usecase::interactor::product_interactor_interface::{
        MockProductInteractor, ProductInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};
    use chrono::Utc;
    use mockall::predicate::*;

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
    async fn test_get_product_success() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_product()
            .with(eq("1"))
            .returning(|_| {
                Ok(Product::new(
                    "gid://shopify/Product/1".to_string(),
                    "Test Product 1",
                    "This is a test product description.",
                    ProductStatus::Active,
                    vec![Variant::new(
                        "gid://shopify/ProductVariant/1".to_string(),
                        Some("Test Variant 1"),
                        100,
                        Some(Sku::new("TESTSKU123").unwrap()),
                        Some(Barcode::new("123456789012").unwrap()),
                        Some(50),
                        1,
                        Utc::now(),
                        Utc::now(),
                    )
                    .unwrap()],
                    Some("gid://shopify/Category/111".to_string()),
                )
                .unwrap())
            });

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_product_not_found() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_product()
            .with(eq("999"))
            .returning(|_| Err(DomainError::NotFound));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}/999"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_product_service_unavailable() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_product()
            .with(eq("1"))
            .returning(|_| Err(DomainError::SystemError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
