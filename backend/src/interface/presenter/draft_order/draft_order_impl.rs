use actix_web::web::{self, Json};
use async_trait::async_trait;

use crate::{
    domain::{
        draft_order::draft_order::{DraftOrder, Id as DraftOrderId},
        error::error::DomainError,
    },
    interface::presenter::draft_order_presenter_interface::DraftOrderPresenter,
};

use super::schema::{
    CompleteDraftOrderErrorResponse, CompleteDraftOrderResponse, DeleteDraftOrderErrorResponse,
    DeleteDraftOrderResponse, DraftOrderSchema, GetDraftOrdersErrorResponse,
    GetDraftOrdersResponse, PostDraftOrderErrorResponse, PostDraftOrderResponse,
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
    type GetDraftOrdersErrorResponse = GetDraftOrdersErrorResponse;
    /// Generate a list response of draft order information.
    async fn present_get_draft_orders(
        &self,
        result: Result<Vec<DraftOrder>, DomainError>,
    ) -> Result<Self::GetDraftOrdersResponse, Self::GetDraftOrdersErrorResponse> {
        let draft_orders = result?;
        if draft_orders.is_empty() {
            return Err(GetDraftOrdersErrorResponse::NotFound {
                object_name: "DraftOrder".to_string(),
            });
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
    type PostDraftOrderErrorResponse = PostDraftOrderErrorResponse;
    /// Generate an create response for draft order.
    async fn present_post_draft_order(
        &self,
        result: Result<DraftOrder, DomainError>,
    ) -> Result<Self::PostDraftOrderResponse, Self::PostDraftOrderErrorResponse> {
        Ok(web::Json(PostDraftOrderResponse {
            draft_order: result?.into(),
        }))
    }

    type CompleteDraftOrderResponse = Json<CompleteDraftOrderResponse>;
    type CompleteDraftOrderErrorResponse = CompleteDraftOrderErrorResponse;
    /// Generate an complete response for draft order.
    async fn present_complete_draft_order(
        &self,
        result: Result<DraftOrder, DomainError>,
    ) -> Result<Self::CompleteDraftOrderResponse, Self::CompleteDraftOrderErrorResponse> {
        Ok(web::Json(CompleteDraftOrderResponse {
            draft_order: result?.into(),
        }))
    }

    type DeleteDraftOrderResponse = Json<DeleteDraftOrderResponse>;
    type DeleteDraftOrderErrorResponse = DeleteDraftOrderErrorResponse;
    /// Generate an delete response for draft order.
    async fn present_delete_draft_order(
        &self,
        result: Result<DraftOrderId, DomainError>,
    ) -> Result<Self::DeleteDraftOrderResponse, Self::DeleteDraftOrderErrorResponse> {
        Ok(web::Json(DeleteDraftOrderResponse {
            id: result?.to_string(),
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
            amount::amount::Amount,
            money::{CurrencyCode, Money},
        },
    };

    use super::*;

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
                    Some(mock_discount()),
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

        assert!(matches!(
            result,
            Err(GetDraftOrdersErrorResponse::NotFound { .. })
        ));
    }

    #[actix_web::test]
    async fn test_present_get_draft_orders_bad_request() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter
            .present_get_draft_orders(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(
            result,
            Err(GetDraftOrdersErrorResponse::BadRequest)
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
            Err(GetDraftOrdersErrorResponse::ServiceUnavailable)
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
            Err(PostDraftOrderErrorResponse::BadRequest)
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
            Err(PostDraftOrderErrorResponse::ServiceUnavailable)
        ));
    }

    #[actix_web::test]
    async fn test_present_complete_draft_order_success() {
        let presenter = DraftOrderPresenterImpl::new();
        let draft_order = mock_draft_orders(1).remove(0);

        let result = presenter
            .present_complete_draft_order(Ok(draft_order))
            .await
            .unwrap();

        assert_eq!(result.draft_order.id, "0");
        assert_eq!(result.draft_order.name, "Test Order 0");
        assert_eq!(result.draft_order.total_price_set.amount, 100.0);
    }

    #[actix_web::test]
    async fn test_present_complete_draft_order_bad_request() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter
            .present_complete_draft_order(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(
            result,
            Err(CompleteDraftOrderErrorResponse::BadRequest)
        ));
    }

    #[actix_web::test]
    async fn test_present_complete_draft_order_service_unavailable() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter
            .present_complete_draft_order(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(CompleteDraftOrderErrorResponse::ServiceUnavailable)
        ));
    }

    #[actix_web::test]
    async fn test_present_delete_draft_order_success() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter
            .present_delete_draft_order(Ok("0".to_string()))
            .await
            .unwrap();

        assert_eq!(result.id, "0");
    }

    #[actix_web::test]
    async fn test_present_delete_draft_order_bad_request() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter
            .present_delete_draft_order(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(
            result,
            Err(DeleteDraftOrderErrorResponse::BadRequest)
        ));
    }

    #[actix_web::test]
    async fn test_present_delete_draft_order_service_unavailable() {
        let presenter = DraftOrderPresenterImpl::new();

        let result = presenter
            .present_delete_draft_order(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(DeleteDraftOrderErrorResponse::ServiceUnavailable)
        ));
    }
}
