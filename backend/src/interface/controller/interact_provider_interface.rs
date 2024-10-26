use async_trait::async_trait;
use mockall::automock;

use crate::usecase::interactor::draft_order_interactor_interface::DraftOrderInteractor;
use crate::usecase::interactor::inventory_interactor_interface::InventoryInteractor;
use crate::usecase::interactor::media_interactor_interface::MediaInteractor;
use crate::usecase::interactor::product_interactor_interface::ProductInteractor;

/// Factory interface providing Interactor.
#[allow(dead_code)]
#[automock]
#[async_trait]
pub trait InteractProvider: Send + Sync {
    async fn provide_product_interactor(&self) -> Box<dyn ProductInteractor>;
    async fn provide_media_interactor(&self) -> Box<dyn MediaInteractor>;
    async fn provide_inventory_interactor(&self) -> Box<dyn InventoryInteractor>;
    async fn provide_draft_order_interactor(&self) -> Box<dyn DraftOrderInteractor>;
}
