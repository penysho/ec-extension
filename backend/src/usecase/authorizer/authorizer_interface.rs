use async_trait::async_trait;
use mockall::automock;
use std::fmt;
use std::sync::Arc;

use crate::domain::error::error::DomainError;
use crate::usecase::user::UserInterface;

/// Authorization interface.
#[automock]
#[async_trait]
pub trait Authorizer: Send + Sync {
    /// Authorize the given request.
    async fn authorize(
        &self,
        user: Arc<dyn UserInterface>,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), DomainError>;
}

/// Resources subject to authorization.
///
/// # Variants
/// - `Product` - Product resource.
/// - `Order` - Order resource.
/// - `Customer` - Customer resource.
/// - `Inventory` - Inventory resource.
#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Product = 1,
    Order,
    Customer,
    Inventory,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Resource::Product => "Product",
            Resource::Order => "Order",
            Resource::Customer => "Customer",
            Resource::Inventory => "Inventory",
        };
        write!(f, "{}", value)
    }
}

/// Actions subject to authorization.
///
/// # Variants
/// - `Read` - Read action.
/// - `Write` - Write action.
/// - `Delete` - Delete action.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Read,
    Write,
    Delete,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Action::Read => "Read",
            Action::Write => "Write",
            Action::Delete => "Delete",
        };
        write!(f, "{}", value)
    }
}
