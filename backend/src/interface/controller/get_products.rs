use crate::interface::presenter::product::product_impl::ProductPresenterImpl;
use crate::interface::{
    controller::controller::Controller, presenter::product_presenter_interface::ProductPresenter,
};
use actix_web::Responder;

impl Controller {
    /// Obtain a list of products.
    pub async fn get_products(&self) -> impl Responder {
        let interactor = self.interact_provider.provide_product_interactor().await;
        let products = interactor.get_products().await;

        let presenter = ProductPresenterImpl::new();
        presenter.present_get_products(products).await
    }
}
