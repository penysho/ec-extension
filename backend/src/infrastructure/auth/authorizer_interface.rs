use async_trait::async_trait;
use mockall::automock;

use crate::domain::error::error::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Product,
    Order,
    Customer,
    Inventory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Read,
    Write,
    Delete,
    All,
}

/// Authorization interface.
#[automock]
#[async_trait]
pub trait Authorizer: Send + Sync {
    /// Authorize the given request.
    async fn authorize(
        &self,
        user_id: &str,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), DomainError>;
}
