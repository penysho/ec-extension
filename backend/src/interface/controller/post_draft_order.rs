use actix_web::{web, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{error::error::DomainError, line_item::line_item::LineItem},
    interface::presenter::{
        draft_order::draft_order_impl::DraftOrderPresenterImpl,
        draft_order_presenter_interface::DraftOrderPresenter,
    },
};

use super::{
    controller::Controller,
    interact_provider_interface::InteractProvider,
    schema::component::component::{
        AddressSchema, CurrencyCodeSchema, DiscountSchema, LineItemSchema,
    },
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
    presentment_currency_code: Option<CurrencyCodeSchema>,
    applied_discount: Option<DiscountSchema>,
}

impl<I, T> Controller<I, T>
where
    I: InteractProvider<T>,
    T: Send + Sync + 'static,
{
    /// Create a draft order.
    pub async fn post_draft_order(&self, body: web::Json<PostDraftOrderRequest>) -> impl Responder {
        let presenter = DraftOrderPresenterImpl::new();

        let line_items = body
            .line_items
            .iter()
            .map(|li| {
                let discount = li
                    .applied_discount
                    .to_owned()
                    .map(|d| d.to_domain())
                    .transpose()?;
                // TODO: Allow customized products to be accepted.
                LineItem::create(false, li.variant_id.to_owned(), li.quantity, discount)
            })
            .collect::<Result<Vec<_>, _>>()?;

        if line_items.is_empty() {
            log::error!("Line items cannot be empty.");
            return presenter
                .present_post_draft_order(Err(DomainError::InvalidRequest))
                .await;
        }

        let billing_address = body
            .billing_address
            .to_owned()
            .map(|a| a.to_domain())
            .transpose()?;

        let shipping_address = body
            .shipping_address
            .to_owned()
            .map(|a| a.to_domain())
            .transpose()?;

        let presentment_currency_code = body
            .presentment_currency_code
            .to_owned()
            .map(|c| c.to_domain())
            .transpose()?;

        let discount = body
            .applied_discount
            .to_owned()
            .map(|d| d.to_domain())
            .transpose()?;

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
                presentment_currency_code,
                discount,
            )
            .await;

        presenter.present_post_draft_order(result).await
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::interface::controller::schema::component::component::{
        CurrencyCodeSchema, DiscountSchema, DiscountValueTypeSchema, MoneySchema,
    };
    use crate::interface::mock::domain_mock::mock_draft_orders;
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
        // Configure the mocks
        let mut interact_provider = MockInteractProvider::<()>::new();
        interact_provider
            .expect_provide_draft_order_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn DraftOrderInteractor>);

        let controller = web::Data::new(Controller::new(interact_provider));

        // Create an application for testing
        test::init_service(
            App::new()
                .app_data(controller)
                .configure(actix_router::configure_routes::<MockInteractProvider<()>, ()>),
        )
        .await
    }

    #[actix_web::test]
    async fn test_post_draft_order_success() {
        let customer_id = "customer_id";

        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_create_draft_order()
            .returning(|_, _, _, _, _, _, _, _, _| Ok(mock_draft_orders(1).remove(0)));

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
                    applied_discount: Some(DiscountSchema {
                        title: Some("Discount title".to_string()),
                        description: Some("Discount description".to_string()),
                        value_type: DiscountValueTypeSchema::Fixed,
                        value: 10.0,
                        amount_set: Some(MoneySchema {
                            currency_code: CurrencyCodeSchema::USD,
                            amount: 10.0,
                        }),
                    }),
                }],
                reserve_inventory_until: Some(Utc::now()),
                tax_exempt: Some(true),
                presentment_currency_code: Some(CurrencyCodeSchema::JPY),
                applied_discount: Some(DiscountSchema {
                    title: Some("Discount title".to_string()),
                    description: Some("Discount description".to_string()),
                    value_type: DiscountValueTypeSchema::Fixed,
                    value: 10.0,
                    amount_set: Some(MoneySchema {
                        currency_code: CurrencyCodeSchema::USD,
                        amount: 10.0,
                    }),
                }),
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
                presentment_currency_code: None,
                applied_discount: None,
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
            .returning(|_, _, _, _, _, _, _, _, _| Err(DomainError::ValidationError));

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
                    applied_discount: None,
                }],
                reserve_inventory_until: None,
                tax_exempt: None,
                presentment_currency_code: None,
                applied_discount: None,
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
            .returning(|_, _, _, _, _, _, _, _, _| Err(DomainError::SystemError));

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
                    applied_discount: None,
                }],
                reserve_inventory_until: None,
                tax_exempt: None,
                presentment_currency_code: None,
                applied_discount: None,
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
