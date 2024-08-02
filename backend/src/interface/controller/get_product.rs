use crate::interface::controller::controller::Controller;
use actix_web::Responder;

impl Controller {
    pub async fn get_product(&self) -> impl Responder {
        "product1"
    }
}
