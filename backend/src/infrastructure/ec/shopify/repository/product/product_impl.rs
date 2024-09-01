use async_trait::async_trait;
use serde_json::json;

use crate::{
    domain::{error::error::DomainError, product::product::Product},
    infrastructure::ec::{
        ec_client_interface::ECClient, shopify::repository::common::schema::GraphQLResponse,
    },
    usecase::repository::product_repository_interface::ProductRepository,
};

use super::schema::{ProductData, ProductSchema, ProductsData};

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
    async fn get_product(&self, id: &str) -> Result<Option<Product>, DomainError> {
        let description_length = Product::MAX_DESCRIPTION_LENGTH;

        let query = json!({
        "query": format!("query {{ product(id: \"gid://shopify/Product/{id}\") {{ id title handle priceRangeV2 {{ maxVariantPrice {{ amount }} }} description(truncateAt: {description_length}) status category {{ id name }} }} }}")
        });

        let graphql_response: GraphQLResponse<ProductData> = self.client.query(&query).await?;
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
                .map_or(String::new(), |a| format!(", after: \"{}\"", a));

            let query = json!({
                "query": format!("query {{ products({first_query}{after_query}) {{ edges {{ node {{ id title handle priceRangeV2 {{ maxVariantPrice {{ amount }} }} description(truncateAt: {description_length}) status category {{ id name }} resourcePublicationOnCurrentPublication {{ publication {{ name id }} publishDate isPublished }} }} }} pageInfo {{ hasPreviousPage hasNextPage startCursor endCursor }} }} }}")
            });

            let graphql_response: GraphQLResponse<ProductsData> = self.client.query(&query).await?;
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

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::infrastructure::ec::{
        ec_client_interface::MockECClient,
        shopify::repository::{
            common::schema::{Edges, GraphQLError, Node, PageInfo},
            product::schema::{MaxVariantPrice, PriceRangeV2, ProductNode, TaxonomyCategory},
        },
    };

    use super::*;

    fn mock_get_product_reponse() -> GraphQLResponse<ProductData> {
        GraphQLResponse {
            data: Some(ProductData {
                product: Some(ProductNode {
                    id: "gid://shopify/Product/123456".to_string(),
                    title: "Test Product".to_string(),
                    price: PriceRangeV2 {
                        max_variant_price: MaxVariantPrice {
                            amount: "100.00".to_string(),
                        },
                    },
                    description: "Test Description".to_string(),
                    status: "ACTIVE".to_string(),
                    category: Some(TaxonomyCategory {
                        id: "gid://shopify/Category/123".to_string(),
                    }),
                }),
            }),
            errors: None,
        }
    }

    fn mock_get_products_reponse(count: usize) -> GraphQLResponse<ProductsData> {
        let products: Vec<Node<ProductNode>> = (0..count)
            .map(|i| Node {
                node: ProductNode {
                    id: format!("gid://shopify/Product/{}", i),
                    title: format!("Test Product {}", i),
                    price: PriceRangeV2 {
                        max_variant_price: MaxVariantPrice {
                            amount: format!("{}", 100 + i),
                        },
                    },
                    description: format!("Test Description {}", i),
                    status: "ACTIVE".to_string(),
                    category: Some(TaxonomyCategory {
                        id: format!("gid://shopify/Category/{}", i),
                    }),
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
            .expect_query::<Value, GraphQLResponse<ProductData>>()
            .times(1)
            .return_once(|_| Ok(mock_get_product_reponse()));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_product("123456").await;

        assert!(result.is_ok());
        let product = result.unwrap();
        assert!(product.is_some());
        let product = product.unwrap();

        assert_eq!(product.id(), "gid://shopify/Product/123456");
        assert_eq!(product.name(), "Test Product");
    }

    #[tokio::test]
    async fn test_get_product_with_graphql_error() {
        let mut client = MockECClient::new();

        let graphql_response_with_error = mock_with_error();

        client
            .expect_query::<Value, GraphQLResponse<ProductData>>()
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
            .expect_query::<Value, GraphQLResponse<ProductData>>()
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
    async fn test_get_product_with_invalid_domain_conversion() {
        let mut client = MockECClient::new();

        // Modify the response to cause `to_domain` to fail
        let graphql_response_invalid_conversion = GraphQLResponse {
            data: Some(ProductData {
                product: Some(ProductNode {
                    id: "gid://shopify/Product/invalid".to_string(),
                    title: "".to_string(), // Assuming this causes an error in `to_domain`
                    price: PriceRangeV2 {
                        max_variant_price: MaxVariantPrice {
                            amount: "100.00".to_string(),
                        },
                    },
                    description: "Test Description".to_string(),
                    status: "INVALID_STATUS".to_string(),
                    category: None,
                }),
            }),
            errors: None,
        };

        client
            .expect_query::<Value, GraphQLResponse<ProductData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_invalid_conversion));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_product("123456").await;

        assert!(result.is_err());
        // Adjust according to the specific DomainError variant expected
        if let Err(DomainError::ValidationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ValidationError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_get_products_no_pagination_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| Ok(mock_get_products_reponse(250))); // Self::GET_PRODUCTS_LIMIT

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_products(&None, &None).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 250);
        assert_eq!(products[0].id(), "gid://shopify/Product/0");
        assert_eq!(products[249].id(), "gid://shopify/Product/249");
    }

    #[tokio::test]
    async fn test_get_products_pagination_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| Ok(mock_get_products_reponse(100)));

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
            .return_once(|_| Ok(mock_get_products_reponse(0)));

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
