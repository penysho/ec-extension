use actix_web::{web, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        address::address::Address, error::error::DomainError, line_item::line_item::LineItem,
    },
    interface::presenter::{
        draft_order::draft_order_impl::DraftOrderPresenterImpl,
        draft_order_presenter_interface::DraftOrderPresenter,
    },
};

use super::{
    controller::Controller,
    schema::component::component::{AddressSchema, LineItemSchema},
};

#[derive(Serialize, Deserialize)]
pub struct PostDraftOrderRequest {
    customer_id: Option<String>,
    billing_address: Option<AddressSchema>,
    shipping_address: Option<AddressSchema>,
    note: Option<String>,
    line_items: Vec<LineItemSchema>,
    reserve_inventory_until: Option<DateTime<Utc>>,
    tax_exempt: Option<bool>,
}

impl Controller {
    /// Update the inventory of the specified SKU.
    pub async fn post_draft_order(&self, body: web::Json<PostDraftOrderRequest>) -> impl Responder {
        let presenter = DraftOrderPresenterImpl::new();

        // TODO: Allow customized products and discounts to be accepted.
        let line_items_result = body
            .line_items
            .iter()
            .map(|li| LineItem::create(false, li.variant_id.to_owned(), li.quantity, None))
            .collect::<Result<Vec<_>, _>>();
        if line_items_result.is_err() {
            return presenter
                .present_post_draft_order(Err(DomainError::InvalidRequest))
                .await;
        };

        let line_items = line_items_result.unwrap();
        if line_items.is_empty() {
            log::error!("Line items cannot be empty.");
            return presenter
                .present_post_draft_order(Err(DomainError::InvalidRequest))
                .await;
        }

        let billing_address_result = body
            .billing_address
            .as_ref()
            .map(|a| {
                Address::new(
                    a.address1.to_owned(),
                    a.address2.to_owned(),
                    a.city.to_owned(),
                    false,
                    a.country.to_owned(),
                    a.first_name.to_owned(),
                    a.last_name.to_owned(),
                    a.province.to_owned(),
                    a.zip.to_owned(),
                    a.phone.to_owned(),
                )
            })
            .transpose();
        if billing_address_result.is_err() {
            return presenter
                .present_post_draft_order(Err(DomainError::InvalidRequest))
                .await;
        }
        let billing_address = billing_address_result.unwrap();

        let shipping_address_result = body
            .shipping_address
            .as_ref()
            .map(|a| {
                Address::new(
                    a.address1.to_owned(),
                    a.address2.to_owned(),
                    a.city.to_owned(),
                    false,
                    a.country.to_owned(),
                    a.first_name.to_owned(),
                    a.last_name.to_owned(),
                    a.province.to_owned(),
                    a.zip.to_owned(),
                    a.phone.to_owned(),
                )
            })
            .transpose();
        if shipping_address_result.is_err() {
            return presenter
                .present_post_draft_order(Err(DomainError::InvalidRequest))
                .await;
        }
        let shipping_address = shipping_address_result.unwrap();

        let interactor = self
            .interact_provider
            .provide_draft_order_interactor()
            .await;

        let result = interactor
            .create_draft_order(
                body.customer_id.to_owned(),
                billing_address,
                shipping_address,
                body.note.to_owned(),
                line_items,
                body.reserve_inventory_until,
                body.tax_exempt,
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
                billing_address: Some(AddressSchema {
                    first_name: Some("John".to_string()),
                    last_name: Some("Doe".to_string()),
                    address1: Some("123 Main St".to_string()),
                    address2: None,
                    city: Some("Anytown".to_string()),
                    province: Some("CA".to_string()),
                    country: Some("US".to_string()),
                    zip: Some("12345".to_string()),
                    phone: Some("555-1234".to_string()),
                }),
                shipping_address: Some(AddressSchema {
                    first_name: Some("John".to_string()),
                    last_name: Some("Doe".to_string()),
                    address1: Some("123 Main St".to_string()),
                    address2: None,
                    city: Some("Anytown".to_string()),
                    province: Some("CA".to_string()),
                    country: Some("US".to_string()),
                    zip: Some("12345".to_string()),
                    phone: Some("555-1234".to_string()),
                }),
                note: Some("Test note".to_string()),
                line_items: vec![LineItemSchema {
                    variant_id: Some("variant_id".to_string()),
                    quantity: 2,
                }],
                reserve_inventory_until: None,
                tax_exempt: None,
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_post_draft_order_bad_request_with_line_items_empty() {
        let interactor = MockDraftOrderInteractor::new();

        let req = test::TestRequest::post()
            .uri(&format!("{BASE_URL}"))
            .set_json(PostDraftOrderRequest {
                customer_id: Some("1".to_string()),
                billing_address: None,
                shipping_address: None,
                note: Some("Test note".to_string()),
                line_items: vec![],
                reserve_inventory_until: None,
                tax_exempt: None,
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
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
                billing_address: None,
                shipping_address: None,
                note: Some("Test note".to_string()),
                line_items: vec![LineItemSchema {
                    variant_id: Some("variant_id".to_string()),
                    quantity: 2,
                }],
                reserve_inventory_until: None,
                tax_exempt: None,
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
                billing_address: None,
                shipping_address: None,
                note: Some("Test note".to_string()),
                line_items: vec![LineItemSchema {
                    variant_id: Some("variant_id".to_string()),
                    quantity: 2,
                }],
                reserve_inventory_until: None,
                tax_exempt: None,
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
