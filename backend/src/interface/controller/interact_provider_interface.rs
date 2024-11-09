use async_trait::async_trait;
use mockall::automock;

use crate::usecase::interactor::customer_interactor_interface::CustomerInteractor;
use crate::usecase::interactor::draft_order_interactor_interface::DraftOrderInteractor;
use crate::usecase::interactor::inventory_interactor_interface::InventoryInteractor;
use crate::usecase::interactor::location_interactor_interface::LocationInteractor;
use crate::usecase::interactor::media_interactor_interface::MediaInteractor;
use crate::usecase::interactor::product_interactor_interface::ProductInteractor;

/// Factory interface providing Interactor.
#[allow(dead_code)]
#[automock]
#[async_trait]
pub trait InteractProvider: Send + Sync {
    /// Provide Interactor for products.
    async fn provide_product_interactor(&self) -> Box<dyn ProductInteractor>;
    /// Provide Interactor for media.
    async fn provide_media_interactor(&self) -> Box<dyn MediaInteractor>;
    /// Provide Interactor for inventory.
    async fn provide_inventory_interactor(&self) -> Box<dyn InventoryInteractor>;
    /// Provide Interactor for draft order.
    async fn provide_draft_order_interactor(&self) -> Box<dyn DraftOrderInteractor>;
    /// Provide Interactor for location.
    async fn provide_location_interactor(&self) -> Box<dyn LocationInteractor>;
    /// Provide Interactor for customer.
    async fn provide_customer_interactor(&self) -> Box<dyn CustomerInteractor>;
}
