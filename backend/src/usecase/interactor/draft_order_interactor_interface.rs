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
    /// Get draft orders by query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to get draft orders.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<DraftOrder>, DomainError>` - The result of the operation.
    ///   - `Ok(Vec<DraftOrder>)` - The draft orders.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the draft order repository fails.
    async fn get_draft_orders(
        &self,
        query: &GetDraftOrdersQuery,
    ) -> Result<Vec<DraftOrder>, DomainError>;

    /// create draft order.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - The customer id.
    /// * `billing_address` - The billing address.
    /// * `shipping_address` - The shipping address.
    /// * `note` - The note.
    /// * `line_items` - The line items.
    /// * `reserve_inventory_until` - The reserve inventory until.
    /// * `tax_exempt` - The tax exempt.
    ///
    /// # Returns
    ///
    /// * `Result<DraftOrder, DomainError>` - The result of the operation.
    ///   - `Ok(DraftOrder)` - The draft order.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the draft order repository fails.
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
