use async_trait::async_trait;
use mockall::automock;
use std::fmt;
use std::str::FromStr;
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

impl TryFrom<i32> for Resource {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Resource::Product),
            2 => Ok(Resource::Order),
            3 => Ok(Resource::Customer),
            4 => Ok(Resource::Inventory),
            _ => Err(DomainError::ConversionError),
        }
    }
}

/// Actions subject to authorization.
///
/// # Variants
/// - `Read` - Read action.
/// - `Write` - Write action.
/// - `Delete` - Delete action.
/// - `All` - All actions.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Read,
    Write,
    Delete,
    All,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Action::Read => "Read",
            Action::Write => "Write",
            Action::Delete => "Delete",
            Action::All => "All",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for Action {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Read" => Ok(Action::Read),
            "Write" => Ok(Action::Write),
            "Delete" => Ok(Action::Delete),
            "All" => Ok(Action::All),
            _ => Err(DomainError::ConversionError),
        }
    }
}
