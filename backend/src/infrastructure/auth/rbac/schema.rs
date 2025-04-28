use std::str::FromStr;

use crate::domain::{
    authorized_resource::authorized_resource::ResourceType,
    error::error::DomainError,
    user::user::{Role, UserAction},
};

/// Convert resource IDs managed in Database to ENUM definitions in Resource.
impl TryFrom<i32> for ResourceType {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ResourceType::Product),
            2 => Ok(ResourceType::Order),
            3 => Ok(ResourceType::Customer),
            4 => Ok(ResourceType::Inventory),
            5 => Ok(ResourceType::DraftOrder),
            _ => Err(DomainError::ConversionError),
        }
    }
}

/// Convert role IDs managed in Database to ENUM definitions in Role.
impl TryFrom<i32> for Role {
    type Error = DomainError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Role::Admin),
            2 => Ok(Role::Operator),
            3 => Ok(Role::Customer),
            4 => Ok(Role::NotLogin),
            _ => Err(DomainError::ConversionError),
        }
    }
}

/// Convert action names managed in Database to ENUM definitions in UserAction.
impl FromStr for UserAction {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "own_read" => Ok(UserAction::OwnRead),
            "own_write" => Ok(UserAction::OwnWrite),
            "own_delete" => Ok(UserAction::OwnDelete),
            "all_read" => Ok(UserAction::AllRead),
            "all_write" => Ok(UserAction::AllWrite),
            "all_delete" => Ok(UserAction::AllDelete),
            "all" => Ok(UserAction::All),
            _ => Err(DomainError::ConversionError),
        }
    }
}
