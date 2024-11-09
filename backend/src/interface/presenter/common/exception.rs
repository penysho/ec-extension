use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;

/// Struct representing a standardized error response schema for API responses.
///
/// This structure is used to define the format of JSON responses for error cases,
/// including a specific error code, message, and HTTP status code.
/// It is serialized for easy integration into JSON-based API responses.
///
/// # Fields
///
/// * `code` - A short error code representing the error type, typically derived from the HTTP status code.
/// * `message` - A user-friendly description of the error.
/// * `status` - The HTTP status code corresponding to the error response.
#[derive(Serialize)]
struct ErrorResponseSchema {
    code: String,
    message: String,
    status: u16,
}

/// Trait providing a method for generating standardized error responses.
///
/// This trait is designed for implementing error types that automatically
/// provide a structured `ErrorResponseSchema` for API responses.
/// Each type implementing this trait should define its own specific `status_code`
/// method to set the HTTP status code appropriately.
///
/// # Required Methods
///
/// * `status_code` - Defines the HTTP status code to return for the implementing error type.
pub trait ErrorResponseBuilder: std::fmt::Display {
    /// Generates an HTTP response based on the error type's defined status code
    /// and error message. The response is serialized into JSON format.
    ///
    /// # Returns
    ///
    /// An `HttpResponse` containing the standardized error response in JSON format.
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponseSchema {
            code: status_code
                .canonical_reason()
                .unwrap_or_default()
                .to_string(),
            message: self.to_string(),
            status: status_code.as_u16(),
        };
        HttpResponse::build(status_code)
            .insert_header(ContentType::json())
            .json(error_response)
    }

    /// Defines the default HTTP status code for errors.
    /// Override this method to provide a specific status code.
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

/// Macro to define standardized error enums with support for custom error responses.
///
/// This macro is intended to simplify the process of creating enums that implement
/// `ErrorResponseBuilder` and `ResponseError` traits, allowing for consistent error handling
/// and API response formatting across different error types.
///
/// # Parameters
///
/// * `$name` - The name of the error enum being defined.
/// * `$object` - A literal string representing the object or resource associated with the error
///               (e.g., `"Product"`, `"Order"`) for enhanced context in the error message.
///
/// # Generated Enums
///
/// The macro creates an error enum with the following variants:
///
/// * `NotFound` - Represents a resource-not-found error, taking an `object_name` parameter for
///                specifying the missing resource.
/// * `BadRequest` - Represents an error related to an invalid request.
/// * `ServiceUnavailable` - Represents an error indicating the service is temporarily unavailable.
#[macro_export]
macro_rules! define_error_response {
    ($name:ident, $object:literal) => {
        #[derive(Debug, Display, Error)]
        pub enum $name {
            #[display(fmt = "{object_name} not found.")]
            NotFound { object_name: String },

            #[display(fmt = "Bad request.")]
            BadRequest,

            #[display(fmt = "Service unavailable.")]
            ServiceUnavailable,
        }

        impl From<DomainError> for $name {
            fn from(err: DomainError) -> Self {
                match err {
                    DomainError::NotFound => $name::NotFound {
                        object_name: $object.to_string(),
                    },
                    DomainError::InvalidRequest | DomainError::ValidationError => $name::BadRequest,
                    _ => $name::ServiceUnavailable,
                }
            }
        }

        impl ErrorResponseBuilder for $name {
            fn status_code(&self) -> StatusCode {
                match self {
                    $name::NotFound { .. } => StatusCode::NOT_FOUND,
                    $name::BadRequest => StatusCode::BAD_REQUEST,
                    $name::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
                }
            }
        }

        impl ResponseError for $name {
            fn error_response(&self) -> HttpResponse {
                <Self as ErrorResponseBuilder>::error_response(self)
            }

            fn status_code(&self) -> StatusCode {
                <Self as ErrorResponseBuilder>::status_code(self)
            }
        }
    };
}
