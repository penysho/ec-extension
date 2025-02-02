use std::fmt;

use crate::domain::user::user::Id as UserId;

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
