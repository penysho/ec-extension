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
/// - `ConversionError` - Conversion to entity failed.
/// - `SaveError` - Resource persistence failed.
/// - `DeleteError` - Resource deletion failed.
/// - `AuthenticationError` - Authentication failed.
/// - `AuthenticationExpired` - Authentication expired.
/// - `AuthorizationError` - Authorization failed.
///
/// # Example
/// ```
/// use backend::domain::error::error::DomainError;
///
/// let error = DomainError::ValidationError;
/// println!("Error: {}", error); // Output: Error: Represents a domain logic validation failure.
/// ```
///
#[derive(Debug, Display, Error, PartialEq)]
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

    /// Conversion to entity failed.
    #[display(fmt = "Failed to convert to entity.")]
    ConversionError,

    /// Resource persistence failed.
    #[display(fmt = "Resource save failed.")]
    SaveError,

    /// Resource deletion failed.
    #[display(fmt = "Resource deletion failed.")]
    DeleteError,

    /// Authentication failed.
    #[display(fmt = "Authentication failed.")]
    AuthenticationError,

    /// Authentication expired.
    #[display(fmt = "Authentication expired.")]
    AuthenticationExpired,

    /// Authorization failed.
    #[display(fmt = "Authorization failed.")]
    AuthorizationError,
}
