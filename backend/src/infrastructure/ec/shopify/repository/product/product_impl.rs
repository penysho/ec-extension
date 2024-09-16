use async_trait::async_trait;
use serde_json::json;

use crate::{
    domain::{
        error::error::DomainError,
        product::product::{Id as ProductId, Product},
    },
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            query_helper::ShopifyGQLQueryHelper,
            repository::{
                common::schema::GraphQLResponse,
                product::schema::{ProductsData, VariantsData},
            },
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
    async fn get_product(&self, id: &ProductId) -> Result<Product, DomainError> {
        let description_length = Product::MAX_DESCRIPTION_LENGTH;
        let first_query = format!("first: {}", Self::GET_PRODUCTS_LIMIT);
        let page_info = ShopifyGQLQueryHelper::page_info();

        let query = json!({
            "query": format!(
                "query {{
                    productVariants({first_query}, query: \"product_id:'{id}'\") {{
                        edges {{
                            node {{
                                id
                                barcode
                                inventoryQuantity
                                sku
                                position
                                price
                                createdAt
                                updatedAt
                                inventoryItem {{
                                    id
                                }}
                                product {{
                                    id
                                    title
                                    handle
                                    priceRangeV2 {{
                                        maxVariantPrice {{
                                            amount
                                        }}
                                    }}
                                    description(truncateAt: {description_length})
                                    status
                                    category {{
                                        id
                                        name
                                    }}
                                }}
                            }}
                        }}
                        {page_info}
                    }}
                }}"
            )
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

        let domains = VariantSchema::to_product_domains(products)?;

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
        let get_product_limit = Self::GET_PRODUCTS_LIMIT as usize;

        let offset = offset.unwrap_or(0) as usize;
        let limit = limit.unwrap_or(Self::GET_PRODUCTS_LIMIT) as usize;

        let mut products_cursor = None;
        let mut all_variants: Vec<VariantSchema> = Vec::new();

        let first_query = format!("first: {}", Self::GET_PRODUCTS_LIMIT);
        let page_info = ShopifyGQLQueryHelper::page_info();

        for i in 0..((limit + offset) / get_product_limit).max(1) {
            let products_after_query = products_cursor
                .as_deref()
                .map_or(String::new(), |a| format!("after: \"{}\"", a));

            let products_query = json!({
                "query": format!(
                    "query {{
                        products({first_query}, {products_after_query}) {{
                            edges {{
                                node {{
                                    id
                                    title
                                    handle
                                    priceRangeV2 {{
                                        maxVariantPrice {{
                                            amount
                                        }}
                                    }}
                                    description(truncateAt: {description_length})
                                    status
                                    category {{
                                        id
                                        name
                                    }}
                                }}
                            }}
                            {page_info}
                        }}
                    }}"
                )
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

            // If only the upper limit is acquired and the acquisition is less than or equal to the offset, skip it.
            products_cursor = products_data.page_info.end_cursor;
            if products_data.edges.len() == get_product_limit
                && get_product_limit * (i + 1) <= offset
                && products_data.page_info.has_next_page
            {
                log::debug!(
                    "Skip products. index: {:?} <= index < {:?}, offset: {:?}",
                    i,
                    (i + 1) * get_product_limit,
                    offset,
                );
                continue;
            }

            let product_ids = products_data
                .edges
                .into_iter()
                .map(|node| ShopifyGQLQueryHelper::remove_product_gid_prefix(&node.node.id))
                .collect::<Vec<String>>()
                .join(",");

            log::debug!("product_ids: {:?}", product_ids);

            let mut variants_cursor = None;
            loop {
                let variants_after_query = variants_cursor
                    .as_deref()
                    .map_or(String::new(), |a| format!("after: \"{}\"", a));

                let variants_query = json!({
                    "query": format!(
                        "query {{
                        productVariants({first_query}, {variants_after_query}, query: \"product_ids:'{product_ids}'\") {{
                            edges {{
                                node {{
                                    id
                                    barcode
                                    inventoryQuantity
                                    sku
                                    position
                                    price
                                    createdAt
                                    updatedAt
                                    inventoryItem {{
                                        id
                                    }}
                                    product {{
                                        id
                                        title
                                        handle
                                        priceRangeV2 {{
                                            maxVariantPrice {{
                                                amount
                                            }}
                                        }}
                                        description(truncateAt: {description_length})
                                        status
                                        category {{
                                            id
                                            name
                                        }}
                                    }}
                                }}
                            }}
                            {page_info}
                        }}
                    }}"
                    )
                });

                let variants_response: GraphQLResponse<VariantsData> =
                    self.client.query(&variants_query).await?;
                if let Some(errors) = variants_response.errors {
                    log::error!(
                        "Error returned in Variants response. Response: {:?}",
                        errors
                    );
                    return Err(DomainError::QueryError);
                }

                let variants_data = variants_response
                    .data
                    .ok_or(DomainError::QueryError)?
                    .product_variants;

                let variants: Vec<VariantSchema> = variants_data
                    .edges
                    .into_iter()
                    .map(|node| VariantSchema::from(node.node))
                    .collect();

                all_variants.extend(variants);

                variants_cursor = variants_data.page_info.end_cursor;
                if !variants_data.page_info.has_next_page {
                    break;
                }
            }

            if !products_data.page_info.has_next_page {
                break;
            }
        }

        let product_domains = VariantSchema::to_product_domains(all_variants)?;
        log::debug!("product_domains.len(): {}", product_domains.len());

        let start = offset % get_product_limit;
        let end = (start + limit).min(product_domains.len());
        if start >= end {
            return Ok(Vec::new());
        }

        Ok(product_domains
            .into_iter()
            .skip(start)
            .take(end - start)
            .collect::<Vec<Product>>())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::Value;

    use crate::{
        domain::error::error::DomainError,
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

    struct PageOption {
        start: usize,
        end: usize,
        has_next_page: bool,
    }

    fn mock_variants_response(opt: PageOption) -> GraphQLResponse<VariantsData> {
        let product_variants: Vec<Node<VariantNode>> = (opt.start..opt.end)
            .map(|i| Node {
                node: VariantNode {
                    id: format!("gid://shopify/ProductVariant/{i}"),
                    product: ProductNode {
                        id: format!("gid://shopify/Product/{i}"),
                        category: Some(TaxonomyCategory {
                            id: "gid://shopify/Category/111".to_string(),
                        }),
                        title: format!("Test Product {i}"),
                        price: PriceRangeV2 {
                            max_variant_price: MaxVariantPrice {
                                amount: format!("{i}.00"),
                            },
                        },
                        description: format!("Test Description {i}"),
                        status: "ACTIVE".to_string(),
                    },
                    inventory_item: InventoryNode {
                        id: "gid://shopify/InventoryItem/789012".to_string(),
                    },
                    barcode: Some("123456789012".to_string()),
                    inventory_quantity: Some(i as i32),
                    sku: Some("TESTSKU123".to_string()),
                    position: 1,
                    price: format!("{i}.00"),
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
                        has_next_page: opt.has_next_page,
                        start_cursor: None,
                        end_cursor: if opt.has_next_page {
                            Some("end_cursor".to_string())
                        } else {
                            None
                        },
                    },
                },
            }),
            errors: None,
        }
    }

    fn mock_products_response(opt: PageOption) -> GraphQLResponse<ProductsData> {
        let products: Vec<Node<ProductNode>> = (opt.start..opt.end)
            .map(|i: usize| Node {
                node: ProductNode {
                    id: format!("gid://shopify/Product/{i}"),
                    category: Some(TaxonomyCategory {
                        id: "gid://shopify/Category/111".to_string(),
                    }),
                    title: format!("Test Product {i}"),
                    price: PriceRangeV2 {
                        max_variant_price: MaxVariantPrice {
                            amount: format!("{i}.00"),
                        },
                    },
                    description: format!("Test Description {i}"),
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
                        has_next_page: opt.has_next_page,
                        start_cursor: None,
                        end_cursor: if opt.has_next_page {
                            Some("end_cursor".to_string())
                        } else {
                            None
                        },
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
            .return_once(|_| {
                Ok(mock_variants_response(PageOption {
                    start: 0,
                    end: 1,
                    has_next_page: false,
                }))
            });

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_product(&("0".to_string())).await;

        assert!(result.is_ok());
        let product = result.unwrap();
        assert_eq!(product.id(), "gid://shopify/Product/0");
        assert_eq!(*product.status(), ProductStatus::Active);
        assert_eq!(product.variants()[0].id(), "gid://shopify/ProductVariant/0");
        assert_eq!(*product.variants()[0].price(), 0);
    }

    #[tokio::test]
    async fn test_get_product_multiple_variants_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| {
                let mut mock = mock_variants_response(PageOption {
                    start: 0,
                    end: 2,
                    has_next_page: false,
                });
                mock.data.as_mut().unwrap().product_variants.edges[1]
                    .node
                    .product
                    .id = "gid://shopify/Product/0".to_string();
                Ok(mock)
            });

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_product(&("0".to_string())).await;

        assert!(result.is_ok());
        let product = result.unwrap();
        assert_eq!(product.id(), "gid://shopify/Product/0");
        assert_eq!(*product.status(), ProductStatus::Active);
        assert_eq!(product.variants().len(), 2);
        assert_eq!(product.variants()[0].id(), "gid://shopify/ProductVariant/0");
        assert_eq!(*product.variants()[0].price(), 0);
        assert_eq!(product.variants()[1].id(), "gid://shopify/ProductVariant/1");
        assert_eq!(*product.variants()[1].price(), 1);
    }

    #[tokio::test]
    async fn test_get_product_with_invalid_domain_conversion() {
        let mut client = MockECClient::new();

        let mut invalid_variant = mock_variants_response(PageOption {
            start: 0,
            end: 1,
            has_next_page: false,
        });
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

        let result = repo.get_product(&("0".to_string())).await;

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

        let result = repo.get_product(&("123456".to_string())).await;

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

        let result = repo.get_product(&("123456".to_string())).await;

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
            .return_once(|_| {
                Ok(mock_products_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });
        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_variants_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.get_products(&None, &None).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 250);

        assert_eq!(products[0].id(), "gid://shopify/Product/0");
        assert_eq!(*(products[0].status()), ProductStatus::Active);
        assert_eq!(
            products[0].variants()[0].id(),
            "gid://shopify/ProductVariant/0"
        );

        assert_eq!(*products[0].variants()[0].price(), 0);
        assert_eq!(products[249].id(), "gid://shopify/Product/249");
        assert_eq!(*(products[0].status()), ProductStatus::Active);
        assert_eq!(
            products[249].variants()[0].id(),
            "gid://shopify/ProductVariant/249"
        );
        assert_eq!(*products[249].variants()[0].price(), 249);
    }

    #[tokio::test]
    async fn test_get_products_pagination_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_products_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });
        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_variants_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });

        let repo = ProductRepositoryImpl::new(client);

        let limit = Some(10);
        let offset = Some(20);
        let result = repo.get_products(&limit, &offset).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 10);

        assert_eq!(products[0].id(), "gid://shopify/Product/20");
        assert_eq!(
            products[0].variants()[0].id(),
            "gid://shopify/ProductVariant/20"
        );

        assert_eq!(products[9].id(), "gid://shopify/Product/29");
        assert_eq!(
            products[9].variants()[0].id(),
            "gid://shopify/ProductVariant/29"
        );
    }

    #[tokio::test]
    async fn test_get_products_multiple_retrievals_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_products_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: true,
                }))
            });
        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_variants_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });
        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_products_response(PageOption {
                    start: 250,
                    end: 500,
                    has_next_page: false,
                }))
            });
        client
            .expect_query::<Value, GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_variants_response(PageOption {
                    start: 250,
                    end: 500,
                    has_next_page: false,
                }))
            });

        let repo = ProductRepositoryImpl::new(client);

        let limit = Some(480);
        let offset = Some(20);
        let result = repo.get_products(&limit, &offset).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 480);

        assert_eq!(products[0].id(), "gid://shopify/Product/20");
        assert_eq!(
            products[0].variants()[0].id(),
            "gid://shopify/ProductVariant/20"
        );

        assert_eq!(products[479].id(), "gid://shopify/Product/499");
        assert_eq!(
            products[479].variants()[0].id(),
            "gid://shopify/ProductVariant/499"
        );
    }

    #[tokio::test]
    async fn test_get_products_empty_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_products_response(PageOption {
                    start: 0,
                    end: 0,
                    has_next_page: false,
                }))
            });

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
