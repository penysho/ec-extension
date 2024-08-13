use async_trait::async_trait;

use crate::usecase::interactor::product_interactor_interface::ProductInteractorInterface;

/// Factory interface providing Interactor.
#[async_trait]
pub trait InteractProvider: Send + Sync {
    async fn provide_product_interactor(&self) -> Box<dyn ProductInteractorInterface>;
}
