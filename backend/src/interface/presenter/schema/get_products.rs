use actix_web::{http::StatusCode, web::Json, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use super::exception::GenericResponseError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProductsResponse {
    pub products: Vec<Product>,
}

#[derive(Debug, Display, Error)]
pub enum GetPostsResponseError {
    #[display(fmt = "Service unavailable. Give it some time and try again.")]
    ServiceUnavailable,
}

impl GenericResponseError for GetPostsResponseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            GetPostsResponseError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl ResponseError for GetPostsResponseError {
    fn error_response(&self) -> HttpResponse {
        <Self as GenericResponseError>::error_response(self)
    }

    fn status_code(&self) -> StatusCode {
        <Self as GenericResponseError>::status_code(self)
    }
}

pub type GetProductsResponseResult = Result<Json<GetProductsResponse>, GetPostsResponseError>;
