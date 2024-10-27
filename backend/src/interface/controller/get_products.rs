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
    /// Get a list of products.
    pub async fn get_products(&self, params: web::Query<GetProductsQueryParams>) -> impl Responder {
        let interactor = self.interact_provider.provide_product_interactor().await;
        let results = interactor
            .get_products_with_media(&params.limit, &params.offset)
            .await;

        let presenter = ProductPresenterImpl::new();
        presenter.present_get_products(results).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::error::error::DomainError;
    use crate::domain::media::associated_id::associated_id::AssociatedId;
    use crate::domain::media::media::{Media, MediaStatus};
    use crate::domain::media::media_content::image::image::Image;
    use crate::domain::media::media_content::media_content::MediaContent;
    use crate::domain::media::src::src::Src;
    use crate::domain::money::amount::amount::Amount;
    use crate::domain::product::product::{Product, ProductStatus};
    use crate::domain::product::variant::barcode::barcode::Barcode;
    use crate::domain::product::variant::sku::sku::Sku;
    use crate::domain::product::variant::variant::{InventoryPolicy, Variant};
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
        interactor
            .expect_get_products_with_media()
            .returning(|_, _| {
                Ok((
                    vec![
                        Product::new(
                            "0",
                            "Test Product 0",
                            "This is a test product description.",
                            ProductStatus::Active,
                            vec![Variant::new(
                                "0",
                                Some("Test Variant 0"),
                                Some(Sku::new("ABC123").unwrap()),
                                Some(Barcode::new("1234567890").unwrap()),
                                true,
                                1,
                                "test_inventory_id",
                                InventoryPolicy::Continue,
                                Some(1),
                                Amount::new(100.0).unwrap(),
                                true,
                                Some("tax_code".to_string()),
                                Utc::now(),
                                Utc::now(),
                            )
                            .unwrap()],
                            Some("111"),
                        )
                        .unwrap(),
                        Product::new(
                            "1",
                            "Test Product 1",
                            "This is a test product description.",
                            ProductStatus::Active,
                            vec![Variant::new(
                                "1",
                                Some("Test Variant 1"),
                                Some(Sku::new("DEF456").unwrap()),
                                Some(Barcode::new("1234567890").unwrap()),
                                true,
                                1,
                                "test_inventory_id",
                                InventoryPolicy::Continue,
                                Some(1),
                                Amount::new(100.0).unwrap(),
                                true,
                                Some("tax_code".to_string()),
                                Utc::now(),
                                Utc::now(),
                            )
                            .unwrap()],
                            Some("111"),
                        )
                        .unwrap(),
                    ],
                    vec![
                        Media::new(
                            format!("0"),
                            Some(format!("Test Media 0")),
                            MediaStatus::Active,
                            Some(MediaContent::Image(
                                Image::new(
                                    format!("0"),
                                    Some(AssociatedId::Product("0".to_string())),
                                    Some(format!("Alt Text 0")),
                                    Some(
                                        Src::new(format!("https://example.com/uploaded.jpg"))
                                            .unwrap(),
                                    ),
                                    Some(
                                        Src::new(format!("https://example.com/published.jpg",))
                                            .unwrap(),
                                    ),
                                )
                                .unwrap(),
                            )),
                            Utc::now(),
                            Utc::now(),
                        )
                        .unwrap(),
                        Media::new(
                            format!("1"),
                            Some(format!("Test Media 1")),
                            MediaStatus::Active,
                            Some(MediaContent::Image(
                                Image::new(
                                    format!("1"),
                                    Some(AssociatedId::Product("0".to_string())),
                                    Some(format!("Alt Text 1")),
                                    Some(
                                        Src::new(format!("https://example.com/uploaded.jpg"))
                                            .unwrap(),
                                    ),
                                    Some(
                                        Src::new(format!("https://example.com/published.jpg",))
                                            .unwrap(),
                                    ),
                                )
                                .unwrap(),
                            )),
                            Utc::now(),
                            Utc::now(),
                        )
                        .unwrap(),
                    ],
                ))
            });

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_products_not_found() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_products_with_media()
            .returning(|_, _| Ok((vec![], vec![])));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_products_bad_request() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_products_with_media()
            .returning(|_, _| Err(DomainError::ValidationError));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_products_service_unavailable() {
        let mut interactor = MockProductInteractor::new();
        interactor
            .expect_get_products_with_media()
            .returning(|_, _| Err(DomainError::SystemError));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
