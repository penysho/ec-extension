use async_trait::async_trait;
use derive_getters::Getters;
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

/// Resource to be authorized.
#[derive(Debug, Clone, PartialEq, Getters)]
pub struct Resource {
    resource_type: ResourceType,
    owner_user_id: Option<String>,
}

impl Resource {
    /// Create a new Resource instance.
    pub fn new(resource_type: ResourceType, owner_user_id: Option<String>) -> Self {
        Self {
            resource_type,
            owner_user_id,
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let owner_user_id = match &self.owner_user_id {
            Some(owner_user_id) => format!(" owned by {}", owner_user_id),
            None => "".to_string(),
        };
        write!(f, "{}{}", self.resource_type, owner_user_id)
    }
}

/// Resource types subject to authorization.
///
/// # Variants
/// - `Product` - Product resource.
/// - `Order` - Order resource.
/// - `Customer` - Customer resource.
/// - `Inventory` - Inventory resource.
/// - `DraftOrder` - Draft order resource.
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Product = 1,
    Order,
    Customer,
    Inventory,
    DraftOrder,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ResourceType::Product => "Product",
            ResourceType::Order => "Order",
            ResourceType::Customer => "Customer",
            ResourceType::Inventory => "Inventory",
            ResourceType::DraftOrder => "DraftOrder",
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
