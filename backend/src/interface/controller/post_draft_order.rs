use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{error::error::DomainError, line_item::line_item::LineItem},
    interface::presenter::{
        draft_order::draft_order_impl::DraftOrderPresenterImpl,
        draft_order_presenter_interface::DraftOrderPresenter,
    },
};

use super::controller::Controller;

#[derive(Serialize, Deserialize)]
pub struct LineItemSchema {
    pub is_custom: bool,
    pub variant_id: Option<String>,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize)]
pub struct PostDraftOrderRequest {
    customer_id: Option<String>,
    note: Option<String>,
    line_items: Vec<LineItemSchema>,
}

impl Controller {
    /// Update the inventory of the specified SKU.
    pub async fn post_draft_order(&self, body: web::Json<PostDraftOrderRequest>) -> impl Responder {
        let presenter = DraftOrderPresenterImpl::new();

        let line_items = body
            .line_items
            .iter()
            .map(|li| LineItem::create(li.is_custom, li.variant_id.to_owned(), li.quantity, None))
            .collect::<Result<Vec<_>, _>>();
        if line_items.is_err() {
            return presenter
                .present_post_draft_order(Err(DomainError::InvalidRequest))
                .await;
        };

        let interactor = self
            .interact_provider
            .provide_draft_order_interactor()
            .await;

        let result = interactor
            .create_draft_order(
                body.customer_id.to_owned(),
                None,
                None,
                body.note.to_owned(),
                line_items.unwrap(),
                None,
                None,
            )
            .await;

        presenter.present_post_draft_order(result).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::draft_order::draft_order::{DraftOrder, DraftOrderStatus};
    use crate::domain::money::money::money::Money;
    use crate::domain::money::money_bag::{CurrencyCode, MoneyBag};
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::usecase::interactor::draft_order_interactor_interface::DraftOrderInteractor;
    use crate::usecase::interactor::draft_order_interactor_interface::MockDraftOrderInteractor;

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};
    use chrono::Utc;

    const BASE_URL: &'static str = "/ec-extension/orders/draft";

    async fn setup(
        interactor: MockDraftOrderInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the InteractProvider mock
        let mut interact_provider = MockInteractProvider::new();
        interact_provider
            .expect_provide_draft_order_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn DraftOrderInteractor>);

        let controller = web::Data::new(Arc::new(Controller::new(Box::new(interact_provider))));

        // Create an application for testing
        test::init_service(
            App::new()
                .app_data(controller)
                .configure(actix_router::configure_routes),
        )
        .await
    }

    fn mock_money_bag() -> MoneyBag {
        let money = Money::new(100.0).unwrap();
        MoneyBag::new(CurrencyCode::USD, money).expect("Failed to create mock money bag")
    }

    #[actix_web::test]
    async fn test_post_draft_order_success() {
        let customer_id = "customer_id";

        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_create_draft_order()
            .returning(|_, _, _, _, _, _, _| {
                DraftOrder::new(
                    format!("1"),
                    format!("Test Order 1"),
                    DraftOrderStatus::Open,
                    None,
                    None,
                    None,
                    None,
                    vec![],
                    None,
                    mock_money_bag(),
                    true,
                    false,
                    mock_money_bag(),
                    mock_money_bag(),
                    mock_money_bag(),
                    mock_money_bag(),
                    None,
                    None,
                    Utc::now(),
                    Utc::now(),
                )
            });

        let req = test::TestRequest::post()
            .uri(&format!("{BASE_URL}"))
            .set_json(PostDraftOrderRequest {
                customer_id: Some(customer_id.to_string()),
                note: Some("Test note".to_string()),
                line_items: vec![LineItemSchema {
                    is_custom: false,
                    variant_id: Some("variant_id".to_string()),
                    quantity: 2,
                }],
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_post_draft_order_bad_request() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_create_draft_order()
            .returning(|_, _, _, _, _, _, _| Err(DomainError::ValidationError));

        let req = test::TestRequest::post()
            .uri(&format!("{BASE_URL}"))
            .set_json(PostDraftOrderRequest {
                customer_id: Some("1".to_string()),
                note: Some("Test note".to_string()),
                line_items: vec![LineItemSchema {
                    is_custom: false,
                    variant_id: Some("variant_id".to_string()),
                    quantity: 2,
                }],
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_post_draft_order_service_unavailable() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_create_draft_order()
            .returning(|_, _, _, _, _, _, _| Err(DomainError::SystemError));

        let req = test::TestRequest::post()
            .uri(&format!("{BASE_URL}"))
            .set_json(PostDraftOrderRequest {
                customer_id: Some("1".to_string()),
                note: Some("Test note".to_string()),
                line_items: vec![LineItemSchema {
                    is_custom: false,
                    variant_id: Some("variant_id".to_string()),
                    quantity: 2,
                }],
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
