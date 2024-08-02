use crate::interface::controller::controller::Controller;
use actix_web::Responder;

impl Controller {
    pub async fn get_products(&self) -> impl Responder {
        "product1,product2,product3"
    }
}
