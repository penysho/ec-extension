use actix_http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

use crate::interface::presenter::common::exception::ErrorResponseBuilder;

#[derive(Debug, Display, Error)]
pub enum PostSingInErrorResponse {
    #[display(fmt = "Authentication error.")]
    Unauthorized,
}

impl ErrorResponseBuilder for PostSingInErrorResponse {
    fn status_code(&self) -> StatusCode {
        match *self {
            PostSingInErrorResponse::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

impl ResponseError for PostSingInErrorResponse {
    fn error_response(&self) -> HttpResponse {
        <Self as ErrorResponseBuilder>::error_response(self)
    }

    fn status_code(&self) -> StatusCode {
        <Self as ErrorResponseBuilder>::status_code(self)
    }
}
