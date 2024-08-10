use crate::{
    entity::product::product::Product,
    interface::{
        controller::controller::Controller,
        presenter::{
            product::product_impl::ProductPresenterImpl,
            product_presenter_interface::ProductPresenter,
        },
    },
};
use actix_web::Responder;

impl Controller {
    pub async fn get_product(&self) -> impl Responder {
        let dummy_product = Product::new(
            "1".to_string(),
            "Product 1".to_string(),
            100,
            "This is a dummy product.".to_string(),
        );
        let result = Ok(Some(dummy_product));

        let presenter = ProductPresenterImpl::new();
        presenter.present_get_product(result).await
    }
}
