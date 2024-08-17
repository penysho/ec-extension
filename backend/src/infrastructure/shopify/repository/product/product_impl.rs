use async_trait::async_trait;
use serde_json::json;

use crate::{
    entity::{error::error::DomainError, product::product::Product},
    infrastructure::{
        error::{InfrastructureError, InfrastructureErrorMapper},
        shopify::repository::{
            client::ShopifyClient,
            schema::{
                common::GraphQLResponse,
                product::{ProductData, ProductSchema, ProductsData},
            },
        },
    },
    usecase::repository::product_repository_interface::ProductRepository,
};

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
        let query = json!({
        "query": format!("query {{ product(id: \"gid://shopify/Product/{id}\") {{ id title handle priceRangeV2 {{ maxVariantPrice {{ amount }} }} description(truncateAt: 500) }} }}")
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
            return Err(InfrastructureErrorMapper::to_domain(
                InfrastructureError::GraphQLResponseError,
            ));
        }

        let product_schema: ProductSchema = ProductSchema::from(
            graphql_response
                .data
                .ok_or(DomainError::SystemError)?
                .product,
        );

        Ok(Some(product_schema.to_domain()))
    }

    /// Retrieve multiple products.
    async fn get_products(&self) -> Result<Vec<Product>, DomainError> {
        let query = json!({
        "query": "query { products(first: 10, reverse: true) { edges { node { id title handle priceRangeV2 { maxVariantPrice { amount } } description(truncateAt: 500) resourcePublicationOnCurrentPublication { publication { name id } publishDate isPublished } } } } }"
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
            return Err(InfrastructureErrorMapper::to_domain(
                InfrastructureError::GraphQLResponseError,
            ));
        }

        let products: Vec<ProductSchema> = graphql_response
            .data
            .ok_or(DomainError::SystemError)?
            .products
            .edges
            .into_iter()
            .map(|node| ProductSchema::from(node.node))
            .collect();

        let product_domains: Vec<Product> = products
            .into_iter()
            .map(|product| product.to_domain())
            .collect();

        Ok(product_domains)
    }
}
