use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};
use serde::Serialize;

use crate::{
    entity::error::error::DomainError,
    infrastructure::{
        config::config::ShopifyConfig,
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
};

/// A client that interacts with GraphQL for Shopify.
pub struct ShopifyClient {
    client: Client,
    config: ShopifyConfig,
}

impl ShopifyClient {
    const SHOPIFY_ACCESS_TOKEN_HEADER: &'static str = "X-Shopify-Access-Token";

    pub fn new(config: ShopifyConfig) -> Self {
        Self {
            client: Client::new(),
            config: config,
        }
    }

    /// Execute a GraphQL query request for Shopify.
    pub async fn query<T>(&self, query: &T) -> Result<Response, DomainError>
    where
        T: Serialize + ?Sized,
    {
        let response = self
            .client
            .post(self.config.store_url())
            .headers(self.build_headers())
            .json(query)
            .send()
            .await
            .map_err(|e| {
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?;
        Ok(response)
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
