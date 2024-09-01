use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    domain::error::error::DomainError,
    infrastructure::{
        config::config::ShopifyConfig,
        ec::ec_client_interface::{ECClient, ECClientResponse},
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
};

/// A client that interacts with GraphQL for Shopify.
pub struct ShopifyClient {
    client: Arc<Mutex<Client>>,
    config: ShopifyConfig,
}

impl ShopifyClient {
    const SHOPIFY_ACCESS_TOKEN_HEADER: &'static str = "X-Shopify-Access-Token";

    pub fn new(config: ShopifyConfig) -> Self {
        Self {
            client: Arc::new(Mutex::new(Client::new())),
            config: config,
        }
    }

    /// Generate headers to be used in GraphQL requests for Shopify.
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            Self::SHOPIFY_ACCESS_TOKEN_HEADER,
            HeaderValue::from_str(self.config.access_token()).unwrap(),
        );
        headers
    }
}

#[async_trait]
impl ECClient for ShopifyClient {
    async fn query<T, U>(&self, query: &T) -> Result<U, DomainError>
    where
        T: Serialize + ?Sized + Send + Sync + 'static,
        U: ECClientResponse + for<'de> serde::Deserialize<'de> + Send + Sync + 'static,
    {
        // Lock the mutex to get the client
        let client = self.client.lock().await;

        // Create the request
        let response = client
            .post(self.config.store_url())
            .headers(self.build_headers())
            .json(query)
            .send()
            .await
            .map_err(|e| {
                // Convert infrastructure error to domain error
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?;

        let graphql_response = response.json::<U>().await.map_err(|e| {
            log::error!("Failed to parse GraphQL response. Error= {:?}", e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
        })?;

        Ok(graphql_response)
    }
}
