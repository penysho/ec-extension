use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::{
    entity::{error::error::DomainError, product::product::Product},
    infrastructure::{
        config::config::ShopifyConfig,
        error::{InfrastructureError, InfrastructureErrorMapper},
        shopify::repository::schema::{
            common::GraphQLResponse,
            product::{ProductSchema, ProductsData},
        },
    },
    usecase::repository::product_repository_interface::ProductRepository,
};

pub struct ProductRepositoryImpl {
    shopify_config: ShopifyConfig,
}

impl ProductRepositoryImpl {
    pub fn new(shopify_config: ShopifyConfig) -> Self {
        Self { shopify_config }
    }
}

#[async_trait]
impl ProductRepository for ProductRepositoryImpl {
    async fn get_products(&self) -> Result<Vec<Product>, DomainError> {
        let client = Client::new();

        let query = json!({
        "query": "query { products(first: 10, reverse: true) { edges { node { id title handle priceRangeV2 { maxVariantPrice { amount } } description(truncateAt: 500) resourcePublicationOnCurrentPublication { publication { name id } publishDate isPublished } } } } }"
        });

        let response = client
            .post(self.shopify_config.store_url())
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", self.shopify_config.access_token())
            .json(&query)
            .send()
            .await
            .map_err(|e| {
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?;

        let graphql_response = response
            .json::<GraphQLResponse<ProductsData>>()
            .await
            .map_err(|e| {
                println!("Error parsing GraphQL response: {}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?;
        if let Some(errors) = graphql_response.errors {
            println!("GraphQL Errors: {:?}", errors);
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
            .map(|node| ProductSchema {
                id: node.node.id,
                title: node.node.title,
                price: node
                    .node
                    .price
                    .max_variant_price
                    .amount
                    .parse::<f64>()
                    .unwrap_or(0.0),
                description: node.node.description,
            })
            .collect();

        let product_domains: Vec<Product> = products
            .into_iter()
            .map(|product| {
                Product::new(
                    product.id,
                    product.title,
                    product.price as u32,
                    product.description,
                )
            })
            .collect();

        Ok(product_domains)
    }
}
