use derive_more::{Display, Error};

/// Entity of Errors.
#[derive(Debug, Display, Error)]
pub enum DomainError {
    #[display(fmt = "System error.")]
    SystemError,
    #[display(fmt = "The result of the query retrieval is abnormal.")]
    QueryError,
    #[display(fmt = "Configuration cannot be initialized.")]
    InitConfigError,
    #[display(fmt = "Validation error in domain logic.")]
    ValidationError,
    #[display(fmt = "Resource not found.")]
    NotFound,
    #[display(fmt = "Invalid request schema.")]
    InvalidRequest,
}
