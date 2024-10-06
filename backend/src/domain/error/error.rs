use derive_more::{Display, Error};

/// Entity of Errors.
///
/// Represents errors that can occur within the domain layer.
/// The `DomainError` enum captures various error cases that might occur during domain logic execution,
/// such as validation failures, system errors, or issues with configuration or resource retrieval.
///
/// # Variants
/// - `SystemError` - Indicates an unexpected system error.
/// - `QueryError` - Indicates that the result of a query operation is abnormal or unexpected.
/// - `InitConfigError` - Indicates a failure when trying to initialize configuration settings.
/// - `ValidationError` - Represents a domain logic validation failure.
/// - `NotFound` - Indicates that the requested resource could not be found.
/// - `InvalidRequest` - Represents an invalid request schema.
#[derive(Debug, Display, Error)]
pub enum DomainError {
    /// Indicates an unexpected system error.
    #[display(fmt = "System error.")]
    SystemError,

    /// Indicates that the result of a query operation is abnormal or unexpected.
    #[display(fmt = "The result of the query retrieval is abnormal.")]
    QueryError,

    /// Indicates a failure when trying to initialize configuration settings.
    #[display(fmt = "Configuration cannot be initialized.")]
    InitConfigError,

    /// Represents a domain logic validation failure.
    #[display(fmt = "Validation error in domain logic.")]
    ValidationError,

    /// Indicates that the requested resource could not be found.
    #[display(fmt = "Resource not found.")]
    NotFound,

    /// Represents an invalid request schema.
    #[display(fmt = "Invalid request schema.")]
    InvalidRequest,
}
