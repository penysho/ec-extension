use async_trait::async_trait;
use serde_json::json;

use crate::{
    domain::{error::error::DomainError, product::product::Product},
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::repository::{
            common::schema::GraphQLResponse,
            product::schema::{ProductsData, VariantsData},
        },
    },
    usecase::repository::product_repository_interface::ProductRepository,
};

use super::schema::VariantSchema;

/// Repository for products for Shopify.
pub struct ProductRepositoryImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> ProductRepositoryImpl<C> {
    const GET_PRODUCTS_LIMIT: u32 = 250;

    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> ProductRepository for ProductRepositoryImpl<C> {
    /// Obtain detailed product information.
    async fn get_product(&self, id: &str) -> Result<Product, DomainError> {
        let description_length = Product::MAX_DESCRIPTION_LENGTH;
        let first_query = format!("first: {}", Self::GET_PRODUCTS_LIMIT);

        let query = json!({
        "query": format!("query {{ productVariants({first_query}, query: \"product_id:'{id}'\") {{ edges {{ node {{ id barcode inventoryQuantity sku position price createdAt updatedAt inventoryItem {{ id }} product {{ id title handle priceRangeV2 {{ maxVariantPrice {{ amount }} }} description(truncateAt: {description_length}) status category {{ id name }} }} }} }} pageInfo {{ hasPreviousPage hasNextPage startCursor endCursor }} }} }}")
        });

        let graphql_response: GraphQLResponse<VariantsData> = self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let products: Vec<VariantSchema> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .product_variants
            .edges
            .into_iter()
            .map(|node| VariantSchema::from(node.node))
            .collect();

        let domains = VariantSchema::to_domains(products)?;

        if domains.is_empty() {
            log::error!("No product found for id: {}", id);
            return Err(DomainError::NotFound);
        }
        Ok(domains.into_iter().next().unwrap())
    }

    /// Retrieve multiple products.
    async fn get_products(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<Vec<Product>, DomainError> {
        let description_length = Product::MAX_DESCRIPTION_LENGTH;

        let offset = offset.unwrap_or(0);
        let limit = limit.unwrap_or(Self::GET_PRODUCTS_LIMIT);

        let mut cursor = None;
        let mut all_products: Vec<Product> = Vec::new();

        for _ in 0..(offset / Self::GET_PRODUCTS_LIMIT).max(1) {
            let first_query = format!("first: {}", Self::GET_PRODUCTS_LIMIT);
            let after_query = cursor
                .as_deref()
                .map_or(String::new(), |a| format!("after: \"{}\"", a));

            let products_query = json!({
                "query": format!("query {{ products({first_query}, {after_query}) {{ edges {{ node {{ id title handle priceRangeV2 {{ maxVariantPrice {{ amount }} }} description(truncateAt: {description_length}) status category {{ id name }} }} }} pageInfo {{ hasPreviousPage hasNextPage startCursor endCursor }} }} }}")
            });

            let products_response: GraphQLResponse<ProductsData> =
                self.client.query(&products_query).await?;
            if let Some(errors) = products_response.errors {
                log::error!(
                    "Error returned in Products response. Response: {:?}",
                    errors
                );
                return Err(DomainError::QueryError);
            }

            let products_data = products_response
                .data
                .ok_or(DomainError::QueryError)?
                .products;

            if products_data.edges.is_empty() {
                break;
            }

            let product_ids = products_data
                .edges
                .into_iter()
                .map(|node| node.node.id.replace("gid://shopify/Product/", ""))
                .collect::<Vec<String>>()
                .join(",");

            let variants_query = json!({
                "query": format!("query {{ productVariants({first_query}, query: \"product_ids:'{product_ids}'\") {{ edges {{ node {{ id barcode inventoryQuantity sku position price createdAt updatedAt inventoryItem {{ id }} product {{ id title handle priceRangeV2 {{ maxVariantPrice {{ amount }} }} description(truncateAt: {description_length}) status category {{ id name }} }} }} }} pageInfo {{ hasPreviousPage hasNextPage startCursor endCursor }} }} }}")
            });

            log::debug!("variants_query: {:?}", variants_query);

            let variants_response: GraphQLResponse<VariantsData> =
                self.client.query(&variants_query).await?;
            if let Some(errors) = variants_response.errors {
                log::error!(
                    "Error returned in Variants response. Response: {:?}",
                    errors
                );
                return Err(DomainError::QueryError);
            }

            let variants: Vec<VariantSchema> = variants_response
                .data
                .ok_or(DomainError::QueryError)?
                .product_variants
                .edges
                .into_iter()
                .map(|node| VariantSchema::from(node.node))
                .collect();

            let product_domains = VariantSchema::to_domains(variants);

            all_products.extend(product_domains?);

            cursor = products_data.page_info.end_cursor;
            if products_data.page_info.has_next_page {
                break;
            }
        }

        log::info!("all_products.len(): {}", all_products.len());

        let start = (offset).min(all_products.len() as u32) as usize;
        let end = (offset + limit).min(all_products.len() as u32) as usize;
        if start >= end {
            return Ok(Vec::new());
        }

        Ok(all_products[start..end].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::Value;

    use crate::{
        domain::product::product::ProductStatus,
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::repository::{
                common::schema::{Edges, GraphQLError, Node, PageInfo},
                product::schema::{
                    InventoryNode, MaxVariantPrice, PriceRangeV2, ProductNode, TaxonomyCategory,
                    VariantNode,
                },
            },
        },
    };

    use super::*;

    fn mock_variants_response(count: usize) -> GraphQLResponse<VariantsData> {
        let product_variants: Vec<Node<VariantNode>> = (0..count)
            .map(|i| Node {
                node: VariantNode {
                    id: format!("gid://shopify/ProductVariant/{i}"),
                    product: ProductNode {
                        id: format!("gid://shopify/Product/{i}"),
                        category: Some(TaxonomyCategory {
                            id: "gid://shopify/Category/111".to_string(),
                        }),
                        title: "Test Product".to_string(),
                        price: PriceRangeV2 {
                            max_variant_price: MaxVariantPrice {
                                amount: "100.00".to_string(),
                            },
                        },
                        description: "Test Description".to_string(),
                        status: "ACTIVE".to_string(),
                    },
                    inventory_item: InventoryNode {
                        id: "gid://shopify/InventoryItem/789012".to_string(),
                    },
                    barcode: Some("123456789012".to_string()),
                    inventory_quantity: Some(50),
                    sku: Some("TESTSKU123".to_string()),
                    position: 1,
                    price: "100.00".to_string(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
            })
            .collect();

        GraphQLResponse {
            data: Some(VariantsData {
                product_variants: Edges {
                    edges: product_variants,
                    page_info: PageInfo {
                        has_previous_page: false,
                        has_next_page: false,
                        start_cursor: None,
                        end_cursor: None,
                    },
                },
            }),
            errors: None,
        }
    }

    fn mock_products_response(count: usize) -> GraphQLResponse<ProductsData> {
        let products: Vec<Node<ProductNode>> = (0..count)
            .map(|i: usize| Node {
                node: ProductNode {
                    id: format!("gid://shopify/Product/{i}"),
                    category: Some(TaxonomyCategory {
                        id: "gid://shopify/Category/111".to_string(),
                    }),
                    title: "Test Product".to_string(),
                    price: PriceRangeV2 {
                        max_variant_price: MaxVariantPrice {
                            amount: "100.00".to_string(),
                        },
                    },
                    description: "Test Description".to_string(),
                    status: "ACTIVE".to_string(),
                },
            })
            .collect();

        GraphQLResponse {
            data: Some(ProductsData {
                products: Edges {
                    edges: products,
                    page_info: PageInfo {
                        has_previous_page: false,
                        has_next_page: false,
                        start_cursor: None,
                        end_cursor: None,
                    },
                },
            }),
            errors: None,
        }
    }

    fn mock_with_error<T>() -> GraphQLResponse<T> {
        GraphQLResponse {
            data: None,
            errors: Some(vec![GraphQLError {
                message: "Some GraphQL error".to_string(),
                extensions: None,
            }]),
        }
    }

    fn mock_with_no_data<T>() -> GraphQLResponse<T> {
        GraphQLResponse {
            data: None,
            errors: None,
        }
    }

    #[tokio::test]
    async fn test_get_product_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(mock_variants_response(10)));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_product("123456").await;

        assert!(result.is_ok());
        let product = result.unwrap();
        assert_eq!(product.id(), "gid://shopify/Product/123456");
        assert_eq!(product.name(), "Test Product");
    }

    #[tokio::test]
    async fn test_get_product_with_invalid_domain_conversion() {
        let mut client = MockECClient::new();

        let mut invalid_variant = mock_variants_response(1);
        invalid_variant
            .data
            .as_mut()
            .unwrap()
            .product_variants
            .edges[0]
            .node
            .product
            .title = "".to_string();

        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(invalid_variant));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_product("123456").await;

        assert!(result.is_err());
        if let Err(DomainError::ValidationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ValidationError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_get_product_with_graphql_error() {
        let mut client = MockECClient::new();

        let graphql_response_with_error = mock_with_error();

        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_error));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_product("123456").await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_get_product_with_missing_data() {
        let mut client = MockECClient::new();

        let graphql_response_with_no_data = mock_with_no_data();

        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_no_data));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_product("123456").await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_get_products_no_pagination_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| Ok(mock_products_response(250))); // Self::GET_PRODUCTS_LIMIT
        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(mock_variants_response(250)));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_products(&None, &None).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 250);
        assert_eq!(products[0].id(), "gid://shopify/Product/0");
        assert_eq!(*(products[0].status()), ProductStatus::Active);
        assert_eq!(products[249].id(), "gid://shopify/Product/249");
        assert_eq!(*(products[0].status()), ProductStatus::Active);
    }

    #[tokio::test]
    async fn test_get_products_pagination_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| Ok(mock_products_response(250)));
        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(mock_variants_response(250)));

        let repo = ProductRepositoryImpl::new(client);

        let limit = Some(10);
        let offset = Some(20);
        let result = repo.get_products(&limit, &offset).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 10);
        assert_eq!(products[0].id(), "gid://shopify/Product/20");
        assert_eq!(products[9].id(), "gid://shopify/Product/29");
    }

    #[tokio::test]
    async fn test_get_products_empty_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| Ok(mock_products_response(0)));

        let repo = ProductRepositoryImpl::new(client);

        let limit = Some(10);
        let offset = Some(20);
        let result = repo.get_products(&limit, &offset).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 0);
    }

    #[tokio::test]
    async fn test_get_products_with_graphql_error() {
        let mut client = MockECClient::new();

        let graphql_response_with_error = mock_with_error();

        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_error));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_products(&None, &None).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_get_products_with_missing_data() {
        let mut client = MockECClient::new();

        let graphql_response_with_no_data = mock_with_no_data();

        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_no_data));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_products(&None, &None).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
