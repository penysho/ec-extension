use crate::usecase::product_interactor_interface::ProductInteractorInterface;

pub struct Controller {
    pub product_interactor: Box<dyn ProductInteractorInterface>,
}

impl Controller {
    pub fn new(product_interactor: Box<dyn ProductInteractorInterface>) -> Self {
        Controller {
            product_interactor: product_interactor,
        }
    }
}
