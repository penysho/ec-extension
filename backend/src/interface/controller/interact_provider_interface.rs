use crate::usecase::interactor::inventory_interactor_interface::InventoryInteractor;
use crate::usecase::interactor::media_interactor_interface::MediaInteractor;
use crate::usecase::interactor::product_interactor_interface::ProductInteractor;
use async_trait::async_trait;
use mockall::automock;

/// Factory interface providing Interactor.
#[automock]
#[async_trait]
pub trait InteractProvider: Send + Sync {
    async fn provide_product_interactor(&self) -> Box<dyn ProductInteractor>;
    async fn provide_media_interactor(&self) -> Box<dyn MediaInteractor>;
    async fn provide_inventory_interactor(&self) -> Box<dyn InventoryInteractor>;
}
