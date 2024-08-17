use crate::interface::{
    controller::controller::Controller,
    presenter::{
        product::product_impl::ProductPresenterImpl, product_presenter_interface::ProductPresenter,
    },
};
use actix_web::{web::Path, Responder};

impl Controller {
    /// Obtain detailed product information.
    pub async fn get_product(&self, path: Path<(String,)>) -> impl Responder {
        let id = &path.into_inner().0;

        let interactor = self.interact_provider.provide_product_interactor().await;
        let products = interactor.get_product(id).await;

        let presenter = ProductPresenterImpl::new();
        presenter.present_get_product(products).await
    }
}
