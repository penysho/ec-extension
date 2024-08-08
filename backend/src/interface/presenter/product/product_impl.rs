use actix_web::web::{self, Json};

use crate::{
    entity::{error::error::DomainError, product::product::Product},
    interface::presenter::{
        product_presenter_interface::ProductPresenter,
        schema::get_products::{
            GetPostsResponseError, GetProductsResponse, GetProductsResponseResult,
        },
    },
};

pub struct ProductPresenterImpl;
impl ProductPresenterImpl {
    pub fn new() -> Self {
        ProductPresenterImpl
    }
}
impl ProductPresenter<Json<GetProductsResponse>, GetPostsResponseError> for ProductPresenterImpl {
    async fn present_get_products(
        &self,
        result: Result<Product, DomainError>,
    ) -> GetProductsResponseResult {
        match result {
            Ok(product) => Ok(web::Json(GetProductsResponse {
                id: product.get_id().to_string(),
                name: product.get_name().to_string(),
                price: product.get_price(),
                description: product.get_description().to_string(),
            })),
            Err(_) => Err(GetPostsResponseError::ServiceUnavailable),
        }
    }
}
