use async_trait::async_trait;

use crate::usecase::interactor::product_interactor_interface::ProductInteractorInterface;

#[async_trait]
pub trait InteractProvider: Send + Sync {
    async fn provide_product_interactor(&self) -> Box<dyn ProductInteractorInterface>;
}
