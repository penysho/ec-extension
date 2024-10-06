use async_trait::async_trait;

use crate::{
    infrastructure::{
        config::config::ShopifyConfig,
        ec::shopify::{
            client_impl::ShopifyGQLClient,
            repository::{
                inventory_item::inventory_item_impl::InventoryItemRepositoryImpl,
                inventory_level::inventory_level_impl::InventoryLevelRepositoryImpl,
                location::location_impl::LocationRepositoryImpl,
                media::media_impl::MediaRepositoryImpl,
                product::product_impl::ProductRepositoryImpl,
            },
        },
    },
    interface::controller::interact_provider_interface::InteractProvider,
    usecase::interactor::{
        inventory::inventory_impl::InventoryInteractorImpl,
        inventory_interactor_interface::InventoryInteractor,
        media::media_impl::MediaInteractorImpl, media_interactor_interface::MediaInteractor,
        product::product_impl::ProductInteractorImpl,
        product_interactor_interface::ProductInteractor,
    },
};

/// Factory providing Interactor.
pub struct InteractProviderImpl {
    shopify_config: ShopifyConfig,
}

impl InteractProviderImpl {
    pub fn new(shopify_config: ShopifyConfig) -> Self {
        Self { shopify_config }
    }
}

#[async_trait]
impl InteractProvider for InteractProviderImpl {
    /// Provide Interactor for products.
    async fn provide_product_interactor(&self) -> Box<dyn ProductInteractor> {
        Box::new(ProductInteractorImpl::new(
            Box::new(ProductRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
            Box::new(MediaRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
        ))
    }

    /// Provide Interactor for media.
    async fn provide_media_interactor(&self) -> Box<dyn MediaInteractor> {
        Box::new(MediaInteractorImpl::new(Box::new(
            MediaRepositoryImpl::new(ShopifyGQLClient::new(self.shopify_config.clone())),
        )))
    }

    /// Provide Interactor for inventory.
    async fn provide_inventory_interactor(&self) -> Box<dyn InventoryInteractor> {
        Box::new(InventoryInteractorImpl::new(
            Box::new(InventoryItemRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
            Box::new(InventoryLevelRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
            Box::new(LocationRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
        ))
    }
}
