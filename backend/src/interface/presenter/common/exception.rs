use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponseSchema {
    code: String,
    message: String,
    status: u16,
}

pub trait ErrorResponseBuilder: std::fmt::Display {
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

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

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
