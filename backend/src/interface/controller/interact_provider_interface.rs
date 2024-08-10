use crate::usecase::product_interactor_interface::ProductInteractorInterface;

pub trait InteractProviderInterface {
    fn provide_product_interactor(&self) -> &dyn ProductInteractorInterface;
}
