use async_trait::async_trait;

use crate::{
    infrastructure::{
        config::config::ShopifyConfig,
        shopify::{
            client::ShopifyClient, repository::product::product_impl::ProductRepositoryImpl,
        },
    },
    interface::controller::interact_provider_interface::InteractProvider,
    usecase::interactor::{
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
        Box::new(ProductInteractorImpl::new(Box::new(
            ProductRepositoryImpl::new(ShopifyClient::new(self.shopify_config.clone())),
        )))
    }
}
