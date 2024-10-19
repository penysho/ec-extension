use async_trait::async_trait;

use crate::domain::{draft_order::draft_order::DraftOrder, error::error::DomainError};

/// Interface to generate response schema for draft orders.
#[async_trait]
pub trait DraftOrderPresenter {
    type GetDraftOrdersResponse;
    type GetDraftOrdersErrorResponse;
    async fn present_get_draft_orders(
        &self,
        result: Result<Vec<DraftOrder>, DomainError>,
    ) -> Result<Self::GetDraftOrdersResponse, Self::GetDraftOrdersErrorResponse>;

    type PostDraftOrderResponse;
    type PostDraftOrderErrorResponse;
    async fn present_post_draft_order(
        &self,
        result: Result<DraftOrder, DomainError>,
    ) -> Result<Self::PostDraftOrderResponse, Self::PostDraftOrderErrorResponse>;
}
