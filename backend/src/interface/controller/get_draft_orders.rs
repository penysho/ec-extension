use actix_web::{web, Responder};
use serde::Deserialize;

use crate::{
    domain::{email::email::Email, error::error::DomainError},
    interface::presenter::{
        draft_order::draft_order_impl::DraftOrderPresenterImpl,
        draft_order_presenter_interface::DraftOrderPresenter,
    },
    usecase::interactor::draft_order_interactor_interface::GetDraftOrdersQuery,
};

use super::controller::Controller;

#[derive(Deserialize)]
pub struct GetDraftOrdersQueryParams {
    email: Option<String>,
}

impl Controller {
    /// Get a list of draft orders.
    pub async fn get_draft_orders(
        &self,
        params: web::Query<GetDraftOrdersQueryParams>,
    ) -> impl Responder {
        let presenter = DraftOrderPresenterImpl::new();

        let query = match validate_query_params(&params) {
            Ok(query) => query,
            Err(error) => return presenter.present_get_draft_orders(Err(error)).await,
        };

        let interactor = self
            .interact_provider
            .provide_draft_order_interactor()
            .await;
        let results = interactor.get_draft_orders(&query).await;

        presenter.present_get_draft_orders(results).await
    }
}

fn validate_query_params(
    params: &GetDraftOrdersQueryParams,
) -> Result<GetDraftOrdersQuery, DomainError> {
    if let Some(email) = params.email.clone() {
        if !email.is_empty() {
            return Ok(GetDraftOrdersQuery::Email(Email::new(email)?));
        }
    }

    Err(DomainError::InvalidRequest)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::address::address::Address;
    use crate::domain::draft_order::draft_order::{DraftOrder, DraftOrderStatus};
    use crate::domain::line_item::discount::discount::{Discount, DiscountValueType};
    use crate::domain::line_item::line_item::LineItem;
    use crate::domain::money::amount::amount::Amount;
    use crate::domain::money::money::{CurrencyCode, Money};
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::usecase::interactor::draft_order_interactor_interface::{
        DraftOrderInteractor, MockDraftOrderInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};
    use chrono::Utc;
    use mockall::predicate::eq;

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

    fn mock_discount() -> Discount {
        Discount::new(
            Some("Test Discount".to_string()),
            Some("Test description".to_string()),
            10.0,
            DiscountValueType::Percentage,
            Some(mock_money()),
        )
        .expect("Failed to create mock discount")
    }

    fn mock_money() -> Money {
        let amount = Amount::new(100.0).unwrap();
        Money::new(CurrencyCode::USD, amount).expect("Failed to create mock money")
    }

    fn mock_line_items(count: usize) -> Vec<LineItem> {
        (0..count)
            .map(|i| {
                LineItem::new(
                    format!("{i}"),
                    false,
                    Some("variant_id"),
                    5,
                    Some(mock_discount()),
                    mock_money(),
                    mock_money(),
                )
                .expect("Failed to create mock line item")
            })
            .collect()
    }

    fn mock_address() -> Option<Address> {
        Some(
            Address::new(
                Some("123 Main St"),
                None::<String>,
                Some("City"),
                true,
                Some("Country"),
                Some("John"),
                Some("Doe"),
                Some("Province"),
                Some("12345"),
                Some("+1234567890"),
            )
            .expect("Failed to create mock address"),
        )
    }

    #[actix_web::test]
    async fn test_get_draft_orders_success() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_get_draft_orders()
            .with(eq(GetDraftOrdersQuery::Email(
                Email::new("john@example.com").expect("Failed to create email"),
            )))
            .returning(|_| {
                Ok(vec![DraftOrder::new(
                    "0",
                    "Test Order",
                    DraftOrderStatus::Open,
                    None,
                    mock_address(),
                    mock_address(),
                    None,
                    mock_line_items(2),
                    None,
                    None,
                    mock_money(),
                    true,
                    false,
                    mock_money(),
                    mock_money(),
                    mock_money(),
                    mock_money(),
                    CurrencyCode::JPY,
                    None,
                    None,
                    Utc::now(),
                    Utc::now(),
                )
                .expect("Failed to create mock draft order")])
            });

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_draft_orders_not_specified_email() {
        let interactor = MockDraftOrderInteractor::new();

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email="))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_draft_orders_not_found() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_get_draft_orders()
            .returning(|_| Ok(vec![]));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_draft_orders_bad_request() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_get_draft_orders()
            .returning(|_| Err(DomainError::ValidationError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_draft_orders_service_unavailable() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_get_draft_orders()
            .returning(|_| Err(DomainError::SystemError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
