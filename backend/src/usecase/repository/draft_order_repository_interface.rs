use async_trait::async_trait;

use crate::domain::{
    customer::customer::Id as CustomerId,
    draft_order::draft_order::{DraftOrder, Id as DraftOrderId},
    error::error::DomainError,
};

/// Repository interface for draft orders.
#[async_trait]
pub trait DraftOrderRepository: Send + Sync {
    /// Retrieves draft orders for a given ID.
    async fn find_draft_order_by_id(&self, id: &DraftOrderId) -> Result<DraftOrder, DomainError>;

    /// Retrieve draft order information by customer id.
    async fn find_draft_orders_by_customer_id(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<DraftOrder>, DomainError>;

    /// Create a draft order
    async fn create(&self, draft_order: DraftOrder) -> Result<DraftOrder, DomainError>;

    // Update a draft order
    async fn update(&self, draft_order: DraftOrder) -> Result<DraftOrder, DomainError>;

    // Delete a draft order
    async fn delete(&self, draft_order: DraftOrder) -> Result<DraftOrderId, DomainError>;
}
