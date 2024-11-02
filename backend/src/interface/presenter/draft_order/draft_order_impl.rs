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
    use crate::interface::mock::domain_mock::mock_draft_orders;

    use super::*;

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
