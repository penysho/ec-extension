use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::domain::authorized_resource::authorized_resource::{ResourceAction, ResourceType};

pub type Id = String;
/// User interfaces commonly used in applications.
pub trait UserInterface: Send + Sync {
    /// Get the user identifier.
    fn id(&self) -> &str;
    /// Get the email address of the user.
    fn email(&self) -> &str;
    #[allow(dead_code)]
    /// Get the roles of the user.
    fn roles(&self) -> Vec<Role>;
    /// Get the permissions of the user.
    fn permissions(&self) -> HashMap<ResourceType, HashSet<UserAction>>;
}

/// User roles.
///
/// # Variants
/// - `Admin` - Admin role.
/// - `Operator` - Operator role.
/// - `Customer` - Customer role.
/// - `NotLogin` - Not login role.
#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Admin = 1,
    Operator,
    Customer,
    NotLogin,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Role::Admin => "Admin",
            Role::Operator => "Operator",
            Role::Customer => "Customer",
            Role::NotLogin => "NotLogin",
        };
        write!(f, "{}", value)
    }
}

/// User actions.
///
/// # Variants
/// - `OwnRead` - Own read action.
/// - `OwnWrite` - Own write action.
/// - `OwnDelete` -  Own delete action.
/// - `AllRead` - All read actions.
/// - `AllWrite` - All write actions.
/// - `AllDelete` - All delete actions.
/// - `All` - All actions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserAction {
    OwnRead,
    OwnWrite,
    OwnDelete,
    AllRead,
    AllWrite,
    AllDelete,
    All,
}

impl fmt::Display for UserAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            UserAction::OwnRead => "OwnRead",
            UserAction::OwnWrite => "OwnWrite",
            UserAction::OwnDelete => "OwnDelete",
            UserAction::AllRead => "AllRead",
            UserAction::AllWrite => "AllWrite",
            UserAction::AllDelete => "AllDelete",
            UserAction::All => "All",
        };
        write!(f, "{}", value)
    }
}

impl UserAction {
    /// Check if the action is own action.
    pub fn is_own_action(&self) -> bool {
        match self {
            UserAction::OwnRead | UserAction::OwnWrite | UserAction::OwnDelete => true,
            _ => false,
        }
    }

    /// Convert UserAction to ResourceAction.
    pub fn to_resource_actions(self) -> Vec<ResourceAction> {
        match self {
            UserAction::OwnRead => vec![ResourceAction::Read],
            UserAction::OwnWrite => vec![ResourceAction::Write],
            UserAction::OwnDelete => vec![ResourceAction::Delete],
            UserAction::AllRead => vec![ResourceAction::Read],
            UserAction::AllWrite => vec![ResourceAction::Write],
            UserAction::AllDelete => vec![ResourceAction::Delete],
            UserAction::All => vec![
                ResourceAction::Read,
                ResourceAction::Write,
                ResourceAction::Delete,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::authorized_resource::authorized_resource::ResourceAction;

    use super::*;

    #[test]
    fn test_user_action_to_actions() {
        assert_eq!(
            UserAction::OwnRead.to_resource_actions(),
            vec![ResourceAction::Read]
        );
        assert_eq!(
            UserAction::OwnWrite.to_resource_actions(),
            vec![ResourceAction::Write]
        );
        assert_eq!(
            UserAction::OwnDelete.to_resource_actions(),
            vec![ResourceAction::Delete]
        );
        assert_eq!(
            UserAction::AllRead.to_resource_actions(),
            vec![ResourceAction::Read]
        );
        assert_eq!(
            UserAction::AllWrite.to_resource_actions(),
            vec![ResourceAction::Write]
        );
        assert_eq!(
            UserAction::AllDelete.to_resource_actions(),
            vec![ResourceAction::Delete]
        );
        assert_eq!(
            UserAction::All.to_resource_actions(),
            vec![
                ResourceAction::Read,
                ResourceAction::Write,
                ResourceAction::Delete
            ]
        );
    }

    #[test]
    fn test_user_action_is_own_action() {
        assert!(UserAction::OwnRead.is_own_action());
        assert!(UserAction::OwnWrite.is_own_action());
        assert!(UserAction::OwnDelete.is_own_action());
        assert!(!UserAction::AllRead.is_own_action());
        assert!(!UserAction::AllWrite.is_own_action());
        assert!(!UserAction::AllDelete.is_own_action());
        assert!(!UserAction::All.is_own_action());
    }
}
