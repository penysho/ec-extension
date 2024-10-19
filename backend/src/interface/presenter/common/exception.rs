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

pub trait GenericResponseError: std::fmt::Display {
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
