use std::fmt;

use crate::domain::user::user::Id as UserId;

/// AuthorizedResource is a trait of a resource that requires authorization.
/// It is assumed that each entity implements.
pub trait AuthorizedResource: Send + Sync {
    /// Get the resource type.
    fn resource_type(&self) -> ResourceType;

    /// Get the owner user ID.
    fn owner_user_id(&self) -> Option<UserId>;
}

impl fmt::Display for dyn AuthorizedResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let owner_user_id = match &self.owner_user_id() {
            Some(owner_user_id) => format!(" owned by {}", owner_user_id),
            None => "".to_string(),
        };
        write!(f, "{}{}", self.resource_type(), owner_user_id)
    }
}

/// Resource is a resource that requires authorization.
/// Used when the target performs authorization when the entity does not yet exist.
#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    resource_type: ResourceType,
    owner_user_id: Option<UserId>,
}

impl Resource {
    /// Create a new Resource instance.
    pub fn new(resource_type: ResourceType, owner_user_id: Option<UserId>) -> Self {
        Self {
            resource_type,
            owner_user_id,
        }
    }
}

impl AuthorizedResource for Resource {
    fn resource_type(&self) -> ResourceType {
        self.resource_type.clone()
    }

    fn owner_user_id(&self) -> Option<UserId> {
        self.owner_user_id.clone()
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// Resource actions subject to authorization.
///
/// # Variants
/// - `Read` - Read action.
/// - `Write` - Write action.
/// - `Delete` - Delete action.
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceAction {
    Read,
    Write,
    Delete,
}

impl fmt::Display for ResourceAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ResourceAction::Read => "Read",
            ResourceAction::Write => "Write",
            ResourceAction::Delete => "Delete",
        };
        write!(f, "{}", value)
    }
}
