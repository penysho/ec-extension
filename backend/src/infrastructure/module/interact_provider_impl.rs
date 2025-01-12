use std::sync::Arc;

use async_trait::async_trait;
use aws_config::SdkConfig;

use crate::{
    infrastructure::{
        auth::{
            cognito::cognito_authenticator::CognitoAuthenticator,
            rbac::rbac_authorizer::RbacAuthorizer,
        },
        config::config::{CognitoConfig, ShopifyConfig},
        db::sea_orm::sea_orm_manager::SeaOrmTransactionManager,
        ec::shopify::{
            client_impl::ShopifyGQLClient,
            query_service::product::product_impl::ProductQueryServiceImpl,
            repository::{
                customer::customer_impl::CustomerRepositoryImpl,
                draft_order::draft_order_impl::DraftOrderRepositoryImpl,
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
        auth::auth_impl::AuthInteractorImpl, auth_interactor_interface::AuthInteractor,
        customer::customer_impl::CustomerInteractorImpl,
        customer_interactor_interface::CustomerInteractor,
        draft_order::draft_order_impl::DraftOrderInteractorImpl,
        draft_order_interactor_interface::DraftOrderInteractor,
        inventory::inventory_impl::InventoryInteractorImpl,
        inventory_interactor_interface::InventoryInteractor,
        location::location_impl::LocationInteractorImpl,
        location_interactor_interface::LocationInteractor, media::media_impl::MediaInteractorImpl,
        media_interactor_interface::MediaInteractor, product::product_impl::ProductInteractorImpl,
        product_interactor_interface::ProductInteractor,
    },
};

/// Factory providing Interactor.
pub struct InteractProviderImpl {
    shopify_config: ShopifyConfig,
    cognito_config: CognitoConfig,
    aws_sdk_config: SdkConfig,
}

impl InteractProviderImpl {
    pub fn new(
        shopify_config: ShopifyConfig,
        cognito_config: CognitoConfig,
        aws_sdk_config: SdkConfig,
    ) -> Self {
        Self {
            shopify_config,
            cognito_config,
            aws_sdk_config,
        }
    }
}

#[async_trait]
impl InteractProvider for InteractProviderImpl {
    async fn provide_product_interactor(&self) -> Box<dyn ProductInteractor> {
        Box::new(ProductInteractorImpl::new(
            Box::new(ProductRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
            Box::new(MediaRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
            Box::new(ProductQueryServiceImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
        ))
    }

    async fn provide_media_interactor(&self) -> Box<dyn MediaInteractor> {
        Box::new(MediaInteractorImpl::new(Box::new(
            MediaRepositoryImpl::new(ShopifyGQLClient::new(self.shopify_config.clone())),
        )))
    }

    async fn provide_inventory_interactor(&self) -> Box<dyn InventoryInteractor> {
        Box::new(InventoryInteractorImpl::new(
            Box::new(InventoryItemRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
            Box::new(InventoryLevelRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
        ))
    }

    async fn provide_draft_order_interactor(&self) -> Box<dyn DraftOrderInteractor> {
        Box::new(DraftOrderInteractorImpl::new(
            Box::new(DraftOrderRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
            Box::new(CustomerRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
        ))
    }

    async fn provide_location_interactor(&self) -> Box<dyn LocationInteractor> {
        Box::new(LocationInteractorImpl::new(Box::new(
            LocationRepositoryImpl::new(ShopifyGQLClient::new(self.shopify_config.clone())),
        )))
    }

    async fn provide_customer_interactor(
        &self,
        transaction_manager: SeaOrmTransactionManager,
    ) -> Box<dyn CustomerInteractor> {
        Box::new(CustomerInteractorImpl::new(
            Box::new(CustomerRepositoryImpl::new(ShopifyGQLClient::new(
                self.shopify_config.clone(),
            ))),
            Arc::new(RbacAuthorizer::new(Arc::new(transaction_manager))),
        ))
    }

    async fn provide_auth_interactor(&self) -> Box<dyn AuthInteractor> {
        Box::new(AuthInteractorImpl::new(
            CognitoAuthenticator::new(self.cognito_config.clone(), self.aws_sdk_config.clone()),
            CustomerRepositoryImpl::new(ShopifyGQLClient::new(self.shopify_config.clone())),
        ))
    }
}
