use async_trait::async_trait;
use serde_json::json;

use crate::{
    domain::{error::error::DomainError, product::product::Product},
    infrastructure::{
        error::{InfrastructureError, InfrastructureErrorMapper},
        shopify::{client::ShopifyClient, repository::common::schema::GraphQLResponse},
    },
    usecase::repository::product_repository_interface::ProductRepository,
};

use super::schema::{ProductData, ProductSchema, ProductsData};

/// Repository for products for Shopify.
pub struct ProductRepositoryImpl {
    client: ShopifyClient,
}

impl ProductRepositoryImpl {
    pub fn new(client: ShopifyClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ProductRepository for ProductRepositoryImpl {
    /// Obtain detailed product information.
    async fn get_product(&self, id: &str) -> Result<Option<Product>, DomainError> {
        let description_length = Product::MAX_DESCRIPTION_LENGTH;

        let query = json!({
        "query": format!("query {{ product(id: \"gid://shopify/Product/{id}\") {{ id title handle priceRangeV2 {{ maxVariantPrice {{ amount }} }} description(truncateAt: {description_length}) status category {{ id name }} }} }}")
        });

        let response = self.client.query(&query).await?;
        let graphql_response = response
            .json::<GraphQLResponse<ProductData>>()
            .await
            .map_err(|e| {
                log::error!("Failed to parse GraphQL response. Error= {:?}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response= {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let product_schema: Option<ProductSchema> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .product
            .map(ProductSchema::from);

        let product = match product_schema {
            Some(schema) => Some(schema.to_domain()?),
            None => None,
        };

        Ok(product)
    }

    /// Retrieve multiple products.
    async fn get_products(&self) -> Result<Vec<Product>, DomainError> {
        let count = 1000;
        let description_length = Product::MAX_DESCRIPTION_LENGTH;

        let query = json!({
        "query": format!("query {{ products(first: {count}, reverse: true) {{ edges {{ node {{ id title handle priceRangeV2 {{ maxVariantPrice {{ amount }} }} description(truncateAt: {description_length}) status category {{ id name }} resourcePublicationOnCurrentPublication {{ publication {{ name id }} publishDate isPublished }} }} }} }} }}")
        });

        let response = self.client.query(&query).await?;
        let graphql_response = response
            .json::<GraphQLResponse<ProductsData>>()
            .await
            .map_err(|e| {
                log::error!("Failed to parse GraphQL response. Error= {:?}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response= {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let products: Vec<ProductSchema> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .products
            .edges
            .into_iter()
            .map(|node| ProductSchema::from(node.node))
            .collect();

        let product_domains: Result<Vec<Product>, DomainError> = products
            .into_iter()
            .map(|product| product.to_domain())
            .collect();

        product_domains
    }
}
