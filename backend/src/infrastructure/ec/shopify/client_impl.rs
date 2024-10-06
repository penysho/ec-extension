use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::Serialize;
use serde_json::json;
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
pub struct ShopifyGQLClient {
    client: Arc<Mutex<Client>>,
    config: ShopifyConfig,
}

impl ShopifyGQLClient {
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
impl ECClient for ShopifyGQLClient {
    async fn query<T>(&self, query: &str) -> Result<T, DomainError>
    where
        T: ECClientResponse + for<'de> serde::Deserialize<'de> + Send + Sync + 'static,
    {
        // Lock the mutex to get the client
        let client = self.client.lock().await;

        let response = client
            .post(self.config.store_url())
            .headers(self.build_headers())
            .json(&json!({
                "query": query,
            }))
            .send()
            .await
            .map_err(|e| {
                log::error!("Error returned by GraphQL run. Error: {:?}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?;

        let graphql_response = response.json::<T>().await.map_err(|e| {
            log::error!("Failed to parse GraphQL query response. Error: {:?}", e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
        })?;

        Ok(graphql_response)
    }

    async fn mutation<T, U>(&self, query: &str, input: &T) -> Result<U, DomainError>
    where
        T: Serialize + ?Sized + Send + Sync + 'static,
        U: ECClientResponse + for<'de> serde::Deserialize<'de> + Send + Sync + 'static,
    {
        // Lock the mutex to get the client
        let client = self.client.lock().await;

        let response = client
            .post(self.config.store_url())
            .headers(self.build_headers())
            .json(&json!({
                "query": query,
                "variables": {
                    "input": input
                },
            }))
            .send()
            .await
            .map_err(|e| {
                log::error!("Error returned by GraphQL run. Error: {:?}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?;

        let graphql_response = response.json::<U>().await.map_err(|e| {
            log::error!("Failed to parse GraphQL query response. Error: {:?}", e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
        })?;

        Ok(graphql_response)
    }
}
