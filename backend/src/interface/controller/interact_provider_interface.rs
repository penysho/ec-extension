use crate::usecase::interactor::product_interactor_interface::ProductInteractor;
use async_trait::async_trait;
use mockall::automock;

/// Factory interface providing Interactor.
#[automock]
#[async_trait]
pub trait InteractProvider: Send + Sync {
    async fn provide_product_interactor(&self) -> Box<dyn ProductInteractor>;
}
