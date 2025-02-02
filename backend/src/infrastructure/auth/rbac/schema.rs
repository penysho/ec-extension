use std::{fmt, str::FromStr};

use crate::{
    domain::error::error::DomainError,
    usecase::authorizer::authorizer_interface::{Action, Resource},
};

/// Convert resource IDs managed in Database to ENUM definitions in Resource.
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

/// DetailAction is a detailed definition of Action managed in Database.
///
/// # Variants
/// - `OwnRead` - Own read action.
/// - `OwnWrite` - Own write action.
/// - `OwnDelete` -  Own delete action.
/// - `AllRead` - All read actions.
/// - `AllWrite` - All write actions.
/// - `AllDelete` - All delete actions.
/// - `All` - All actions. Special actions only for system administrators.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum DetailAction {
    OwnRead,
    OwnWrite,
    OwnDelete,
    AllRead,
    AllWrite,
    AllDelete,
    All,
}

impl fmt::Display for DetailAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            DetailAction::OwnRead => "own_read",
            DetailAction::OwnWrite => "own_write",
            DetailAction::OwnDelete => "own_delete",
            DetailAction::AllRead => "all_read",
            DetailAction::AllWrite => "all_write",
            DetailAction::AllDelete => "all_delete",
            DetailAction::All => "all",
        };
        write!(f, "{}", value)
    }
}

/// Convert action names managed in Database to ENUM definitions in DetailAction.
impl FromStr for DetailAction {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "own_read" => Ok(DetailAction::OwnRead),
            "own_write" => Ok(DetailAction::OwnWrite),
            "own_delete" => Ok(DetailAction::OwnDelete),
            "all_read" => Ok(DetailAction::AllRead),
            "all_write" => Ok(DetailAction::AllWrite),
            "all_delete" => Ok(DetailAction::AllDelete),
            "all" => Ok(DetailAction::All),
            _ => Err(DomainError::ConversionError),
        }
    }
}

impl DetailAction {
    /// Convert DetailAction to Action.
    pub(super) fn to_actions(self) -> Vec<Action> {
        match self {
            DetailAction::OwnRead => vec![Action::Read],
            DetailAction::OwnWrite => vec![Action::Write],
            DetailAction::OwnDelete => vec![Action::Delete],
            DetailAction::AllRead => vec![Action::Read],
            DetailAction::AllWrite => vec![Action::Write],
            DetailAction::AllDelete => vec![Action::Delete],
            DetailAction::All => vec![Action::Read, Action::Write, Action::Delete],
        }
    }
}
