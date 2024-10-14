use actix_web::web::{self, Json};
use async_trait::async_trait;

use crate::{
    domain::{draft_order::draft_order::DraftOrder, error::error::DomainError},
    interface::presenter::draft_order_presenter_interface::DraftOrderPresenter,
};

use super::schema::{
    DraftOrderSchema, GetDraftOrdersResponse, GetDraftOrdersResponseError, PostDraftOrderResponse,
    PostDraftOrderResponseError,
};

/// Generate a response schema for the draft orders.
pub struct DraftOrderPresenterImpl;
impl DraftOrderPresenterImpl {
    pub fn new() -> Self {
        DraftOrderPresenterImpl
    }
}

#[async_trait]
impl DraftOrderPresenter for DraftOrderPresenterImpl {
    type GetDraftOrdersResponse = Json<GetDraftOrdersResponse>;
    type GetDraftOrdersResponseError = GetDraftOrdersResponseError;
    /// Generate a list response of draft order information.
    async fn present_get_draft_orders(
        &self,
        result: Result<Vec<DraftOrder>, DomainError>,
    ) -> Result<Self::GetDraftOrdersResponse, Self::GetDraftOrdersResponseError> {
        let draft_orders = match result {
            Ok(draft_orders) => draft_orders,
            Err(DomainError::ValidationError) => {
                return Err(GetDraftOrdersResponseError::BadRequest)
            }
            Err(DomainError::InvalidRequest) => {
                return Err(GetDraftOrdersResponseError::BadRequest)
            }
            Err(_) => return Err(GetDraftOrdersResponseError::ServiceUnavailable),
        };
        if draft_orders.is_empty() {
            return Err(GetDraftOrdersResponseError::NotFound);
        }

        let response: Vec<DraftOrderSchema> = draft_orders
            .into_iter()
            .map(|dratft_order| dratft_order.into())
            .collect();

        Ok(web::Json(GetDraftOrdersResponse {
            draft_orders: response,
        }))
    }

    type PostDraftOrderResponse = Json<PostDraftOrderResponse>;
    type PostDraftOrderResponseError = PostDraftOrderResponseError;
    /// Generate an create response for draft order.
    async fn present_post_draft_order(
        &self,
        result: Result<DraftOrder, DomainError>,
    ) -> Result<Self::PostDraftOrderResponse, Self::PostDraftOrderResponseError> {
        let draft_order = match result {
            Ok(draft_order) => draft_order,
            Err(DomainError::ValidationError) => {
                return Err(PostDraftOrderResponseError::BadRequest)
            }
            Err(DomainError::InvalidRequest) => {
                return Err(PostDraftOrderResponseError::BadRequest)
            }
            Err(_) => return Err(PostDraftOrderResponseError::ServiceUnavailable),
        };

        Ok(web::Json(PostDraftOrderResponse {
            draft_order: draft_order.into(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::domain::{
        address::address::Address,
        draft_order::draft_order::DraftOrderStatus,
        line_item::{
            discount::discount::{Discount, DiscountValueType},
            line_item::LineItem,
        },
        money::{
            money::money::Money,
            money_bag::{CurrencyCode, MoneyBag},
        },
    };

    use super::*;

    fn mock_discount() -> Discount {
        Discount::new(
            Some("Test Discount".to_string()),
            Some("Test description".to_string()),
            10.0,
            DiscountValueType::Percentage,
            mock_money_bag(),
        )
        .expect("Failed to create mock discount")
    }

    fn mock_money_bag() -> MoneyBag {
        let money = Money::new(100.0).unwrap();
        MoneyBag::new(CurrencyCode::USD, money).expect("Failed to create mock money bag")
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
                    mock_money_bag(),
                    mock_money_bag(),
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

    fn mock_draft_orders(count: usize) -> Vec<DraftOrder> {
        (0..count)
            .map(|i| {
                DraftOrder::new(
                    format!("{i}"),
                    format!("Test Order {i}"),
                    DraftOrderStatus::Open,
                    None,
                    mock_address(),
                    mock_address(),
                    None,
                    mock_line_items(2),
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
                .expect("Failed to create mock draft order")
            })
            .collect()
    }

    #[actix_web::test]
    async fn test_present_get_draft_orders_success() {
        let presenter = DraftOrderPresenterImpl::new();
        let draft_orders = mock_draft_orders(10);

        let result = presenter
            .present_get_draft_orders(Ok(draft_orders))
            .await
            .unwrap();

        assert_eq!(result.draft_orders.len(), 10);

        assert_eq!(result.draft_orders[0].id, "0");
        assert_eq!(result.draft_orders[0].name, "Test Order 0");
        assert_eq!(result.draft_orders[0].total_price_set.amount, 100.0);

        assert_eq!(result.draft_orders[9].id, "9");
        assert_eq!(result.draft_orders[9].name, "Test Order 9");
        assert_eq!(result.draft_orders[9].total_price_set.amount, 100.0);
    }

    #[actix_web::test]
    async fn test_present_get_draft_orders_not_found() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter.present_get_draft_orders(Ok(vec![])).await;

        assert!(matches!(result, Err(GetDraftOrdersResponseError::NotFound)));
    }

    #[actix_web::test]
    async fn test_present_get_draft_orders_bad_request() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter
            .present_get_draft_orders(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(
            result,
            Err(GetDraftOrdersResponseError::BadRequest)
        ));
    }

    #[actix_web::test]
    async fn test_present_get_draft_orders_service_unavailable() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter
            .present_get_draft_orders(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(GetDraftOrdersResponseError::ServiceUnavailable)
        ));
    }

    #[actix_web::test]
    async fn test_present_post_draft_order_success() {
        let presenter = DraftOrderPresenterImpl::new();
        let draft_order = mock_draft_orders(1).remove(0);

        let result = presenter
            .present_post_draft_order(Ok(draft_order))
            .await
            .unwrap();

        assert_eq!(result.draft_order.id, "0");
        assert_eq!(result.draft_order.name, "Test Order 0");
        assert_eq!(result.draft_order.total_price_set.amount, 100.0);
    }

    #[actix_web::test]
    async fn test_present_post_draft_order_bad_request() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter
            .present_post_draft_order(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(
            result,
            Err(PostDraftOrderResponseError::BadRequest)
        ));
    }

    #[actix_web::test]
    async fn test_present_post_draft_order_service_unavailable() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter
            .present_post_draft_order(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(PostDraftOrderResponseError::ServiceUnavailable)
        ));
    }
}
