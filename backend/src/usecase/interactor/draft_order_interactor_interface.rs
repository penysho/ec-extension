use async_trait::async_trait;
use mockall::automock;

use crate::domain::{
    draft_order::draft_order::DraftOrder, email::email::Email, error::error::DomainError,
};

#[derive(Debug, Clone, PartialEq)]
pub enum GetDraftOrdersQuery {
    Email(Email),
}

/// Interactor interface for draft orders.
#[automock]
#[async_trait]
pub trait DraftOrderInteractor {
    async fn get_draft_orders(
        &self,
        query: &GetDraftOrdersQuery,
    ) -> Result<Vec<DraftOrder>, DomainError>;
}
