use async_trait::async_trait;
use serde_json::json;

use crate::{
    domain::{error::error::DomainError, product::product::Product},
    infrastructure::{
        ec::shopify::{client::ShopifyClient, repository::common::schema::GraphQLResponse},
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
    usecase::repository::product_repository_interface::ProductRepository,
};

use super::schema::{ProductData, ProductSchema, ProductsData};

/// Repository for products for Shopify.
pub struct ProductRepositoryImpl {
    client: ShopifyClient,
}

impl ProductRepositoryImpl {
    const GET_PRODUCTS_LIMIT: u32 = 250;

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
    async fn get_products(
        &self,
        offset: &Option<u32>,
        limit: &Option<u32>,
    ) -> Result<Vec<Product>, DomainError> {
        let description_length = Product::MAX_DESCRIPTION_LENGTH;

        let offset = offset.unwrap_or(0);
        let mut limit = limit.unwrap_or(Self::GET_PRODUCTS_LIMIT);
        if offset > limit {
            limit = Self::GET_PRODUCTS_LIMIT;
        }

        let mut cursor = None;
        let mut all_products: Vec<Product> = Vec::new();

        for _ in 0..(offset / limit).max(1) {
            let first_query = format!("first: {}", Self::GET_PRODUCTS_LIMIT);
            let after_query = cursor
                .as_deref()
                .map_or(String::new(), |a| format!(", after: \"{}\"", a));

            let query = json!({
                "query": format!("query {{ products({first_query}{after_query}) {{ edges {{ node {{ id title handle priceRangeV2 {{ maxVariantPrice {{ amount }} }} description(truncateAt: {description_length}) status category {{ id name }} resourcePublicationOnCurrentPublication {{ publication {{ name id }} publishDate isPublished }} }} }} pageInfo {{ hasPreviousPage hasNextPage startCursor endCursor }} }} }}")
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

            let data = graphql_response
                .data
                .ok_or(DomainError::QueryError)?
                .products;

            let products: Vec<ProductSchema> = data
                .edges
                .into_iter()
                .map(|node| ProductSchema::from(node.node))
                .collect();

            let product_domains: Result<Vec<Product>, DomainError> = products
                .into_iter()
                .map(|product| product.to_domain())
                .collect();

            all_products.extend(product_domains?);

            cursor = data.page_info.end_cursor;
            if data.page_info.has_next_page {
                break;
            }
        }

        log::info!("all_products.len() = {}", all_products.len());

        let start = (offset).min(all_products.len() as u32) as usize;
        let end = (offset + limit).min(all_products.len() as u32) as usize;
        if start >= end {
            return Ok(Vec::new());
        }

        Ok(all_products[start..end].to_vec())
    }
}
