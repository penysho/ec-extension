use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mockall::automock;

use crate::domain::{
    address::address::Address, customer::customer::Id as CustomerId,
    draft_order::draft_order::DraftOrder, email::email::Email, error::error::DomainError,
    line_item::line_item::LineItem,
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

    async fn create_draft_order(
        &self,
        customer_id: Option<CustomerId>,
        billing_address: Option<Address>,
        shipping_address: Option<Address>,
        note: Option<String>,
        line_items: Vec<LineItem>,
        reserve_inventory_until: Option<DateTime<Utc>>,
        tax_exempt: Option<bool>,
    ) -> Result<DraftOrder, DomainError>;
}
