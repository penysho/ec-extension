use crate::entity::product::product::Product;
use crate::interface::presenter::product::product_impl::ProductPresenterImpl;
use crate::interface::{
    controller::controller::Controller, presenter::product_presenter_interface::ProductPresenter,
};
use actix_web::Responder;

impl Controller {
    pub async fn get_products(&self) -> impl Responder {
        let dummy_product = Product::new(
            "1".to_string(),
            "Product 1".to_string(),
            100,
            "This is a dummy product.".to_string(),
        );
        let result = Ok(dummy_product);

        let presenter = ProductPresenterImpl::new();
        presenter.present_get_products(result).await
    }
}
