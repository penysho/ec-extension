use crate::interface::controller::controller::Controller;
use crate::interface::presenter::schema::get_products::{
    GetProductsResponce, GetProductsResponceResult,
};
use actix_web::web;

impl Controller {
    pub async fn get_products(&self) -> GetProductsResponceResult {
        let pdp_redirect_url = GetProductsResponce {
            id: "product_id".to_string(),
            name: "product_name".to_string(),
            price: 100,
            description: "product_description".to_string(),
        };
        Ok(web::Json(pdp_redirect_url))
    }
}
