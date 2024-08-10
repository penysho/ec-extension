use crate::interface::presenter::product::product_impl::ProductPresenterImpl;
use crate::interface::{
    controller::controller::Controller, presenter::product_presenter_interface::ProductPresenter,
};
use actix_web::Responder;

impl Controller {
    pub async fn get_products(&self) -> impl Responder {
        let result = self.product_interactor.get_products().await;

        let presenter = ProductPresenterImpl::new();
        presenter.present_get_products(result).await
    }
}
