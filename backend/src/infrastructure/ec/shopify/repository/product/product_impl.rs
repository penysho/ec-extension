use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        product::product::{Id as ProductId, Product},
    },
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            query_helper::ShopifyGQLQueryHelper,
            repository::schema::{
                common::GraphQLResponse,
                product::{ProductsData, VariantNode, VariantsData},
            },
        },
    },
    usecase::repository::product_repository_interface::ProductRepository,
};

/// Repository for products for Shopify.
pub struct ProductRepositoryImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> ProductRepositoryImpl<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    fn product_fields() -> String {
        let description_length = Product::MAX_DESCRIPTION_LENGTH;

        format!(
            "id
            title
            handle
            description(truncateAt: {description_length})
            status
            category {{
                id
                name
            }}"
        )
    }

    fn variant_fields() -> String {
        let product_fields = Self::product_fields();

        format!(
            "id
            title
            sku
            barcode
            availableForSale
            position
            inventoryItem {{
                id
            }}
            inventoryQuantity
            inventoryPolicy
            price
            taxable
            taxCode
            createdAt
            updatedAt
            product {{
                {product_fields}
            }}"
        )
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> ProductRepository for ProductRepositoryImpl<C> {
    async fn find_product_by_id(&self, id: &ProductId) -> Result<Product, DomainError> {
        let first_query = ShopifyGQLQueryHelper::first_query();
        let page_info = ShopifyGQLQueryHelper::page_info();
        let variant_fields = Self::variant_fields();

        let query = format!(
            "query {{
                productVariants({first_query}, query: \"product_id:'{id}'\") {{
                    edges {{
                        node {{
                            {variant_fields}
                        }}
                    }}
                    {page_info}
                }}
            }}"
        );

        let graphql_response: GraphQLResponse<VariantsData> = self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let variant_nodes: Vec<VariantNode> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .product_variants
            .edges
            .into_iter()
            .map(|node| node.node)
            .collect();

        let domains = VariantNode::to_product_domains(variant_nodes)?;

        if domains.is_empty() {
            log::error!("No product found for id: {}", id);
            return Err(DomainError::NotFound);
        }
        Ok(domains.into_iter().next().unwrap())
    }

    async fn find_products(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<Vec<Product>, DomainError> {
        let query_limit = ShopifyGQLQueryHelper::SHOPIFY_QUERY_LIMIT;

        let offset = offset.unwrap_or(0) as usize;
        let limit = limit.unwrap_or(query_limit as u32) as usize;

        let mut products_cursor = None;
        let mut all_variants: Vec<VariantNode> = Vec::new();

        let first_query = ShopifyGQLQueryHelper::first_query();
        let product_fields = Self::product_fields();
        let page_info = ShopifyGQLQueryHelper::page_info();

        for i in 0..((limit + offset) / query_limit).max(1) {
            let products_after_query = products_cursor
                .as_deref()
                .map_or(String::new(), |a| format!("after: \"{}\"", a));

            let products_query = format!(
                "query {{
                    products({first_query}, {products_after_query}) {{
                        edges {{
                            node {{
                                {product_fields}
                            }}
                        }}
                        {page_info}
                    }}
                }}"
            );

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
            if products_data.edges.len() == query_limit
                && query_limit * (i + 1) <= offset
                && products_data.page_info.has_next_page
            {
                log::debug!(
                    "Skip products. index: {:?} <= index < {:?}, offset: {:?}",
                    i,
                    (i + 1) * query_limit,
                    offset,
                );
                continue;
            }

            let product_ids = products_data
                .edges
                .into_iter()
                .map(|node| ShopifyGQLQueryHelper::remove_gid_prefix(&node.node.id))
                .collect::<Vec<String>>()
                .join(",");

            log::debug!("product_ids: {:?}", product_ids);

            let mut variants_cursor = None;
            let variant_fields = Self::variant_fields();
            loop {
                let variants_after_query = variants_cursor
                    .as_deref()
                    .map_or(String::new(), |a| format!("after: \"{}\"", a));

                let variants_query = format!(
                    "query {{
                        productVariants({first_query}, {variants_after_query}, query: \"product_ids:'{product_ids}'\") {{
                            edges {{
                                node {{
                                    {variant_fields}
                                }}
                            }}
                            {page_info}
                        }}
                    }}"
                );

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

                let variants: Vec<VariantNode> = variants_data
                    .edges
                    .into_iter()
                    .map(|node| node.node)
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

        let product_domains = VariantNode::to_product_domains(all_variants)?;
        log::debug!("product_domains.len(): {}", product_domains.len());

        let start = offset % query_limit;
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

    use crate::{
        domain::{error::error::DomainError, product::product::ProductStatus},
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::repository::schema::{
                common::{Edges, GraphQLError, Node, PageInfo},
                product::{InventoryItemIdNode, ProductNode, TaxonomyCategory, VariantNode},
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
                    title: format!("Test Variant {i}"),
                    sku: Some("TESTSKU123".to_string()),
                    barcode: Some("123456789012".to_string()),
                    available_for_sale: true,
                    position: 1,
                    inventory_item: InventoryItemIdNode {
                        id: format!("gid://shopify/InventoryItem/{i}"),
                    },
                    inventory_quantity: Some(i as i32),
                    inventory_policy: "DENY".to_string(),
                    price: format!("{i}.00"),
                    taxable: true,
                    tax_code: Some("TESTTAXCODE".to_string()),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    product: ProductNode {
                        id: format!("gid://shopify/Product/{i}"),
                        category: Some(TaxonomyCategory {
                            id: "gid://shopify/Category/111".to_string(),
                        }),
                        title: format!("Test Product {i}"),
                        description: format!("Test Description {i}"),
                        status: "ACTIVE".to_string(),
                    },
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
    async fn test_find_product_by_id_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_variants_response(PageOption {
                    start: 0,
                    end: 1,
                    has_next_page: false,
                }))
            });

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.find_product_by_id(&("0".to_string())).await;

        assert!(result.is_ok());
        let product = result.unwrap();
        assert_eq!(product.id(), "0");
        assert_eq!(*product.status(), ProductStatus::Active);
        assert_eq!(product.variants()[0].id(), "0");
    }

    #[tokio::test]
    async fn test_find_product_by_id_multiple_variants_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| {
                let mut mock = mock_variants_response(PageOption {
                    start: 0,
                    end: 2,
                    has_next_page: false,
                });

                // Unify product IDs.
                mock.data.as_mut().unwrap().product_variants.edges[1]
                    .node
                    .product
                    .id = "gid://shopify/Product/0".to_string();
                Ok(mock)
            });

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.find_product_by_id(&("0".to_string())).await;

        assert!(result.is_ok());
        let product = result.unwrap();
        assert_eq!(product.id(), "0");
        assert_eq!(*product.status(), ProductStatus::Active);

        assert_eq!(product.variants().len(), 2);
        assert_eq!(product.variants()[0].id(), "0");
        assert_eq!(product.variants()[1].id(), "1");
    }

    #[tokio::test]
    async fn test_find_product_by_id_with_invalid_domain_conversion() {
        let mut client = MockECClient::new();

        let mut invalid_response = mock_variants_response(PageOption {
            start: 0,
            end: 1,
            has_next_page: false,
        });
        invalid_response
            .data
            .as_mut()
            .unwrap()
            .product_variants
            .edges[0]
            .node
            .product
            .title = "".to_string();

        client
            .expect_query::<GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(invalid_response));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.find_product_by_id(&("0".to_string())).await;

        assert!(result.is_err());
        if let Err(DomainError::ValidationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ValidationError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_product_by_id_with_invalid_status() {
        let mut client = MockECClient::new();

        let mut invalid_response = mock_variants_response(PageOption {
            start: 0,
            end: 1,
            has_next_page: false,
        });
        invalid_response
            .data
            .as_mut()
            .unwrap()
            .product_variants
            .edges[0]
            .node
            .product
            .status = "INVALID_STATUS".to_string();

        client
            .expect_query::<GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(invalid_response));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.find_product_by_id(&("0".to_string())).await;

        assert!(result.is_err());
        if let Err(DomainError::ConversionError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ConversionError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_product_by_id_with_invalid_inventory_policy() {
        let mut client = MockECClient::new();

        let mut invalid_response = mock_variants_response(PageOption {
            start: 0,
            end: 1,
            has_next_page: false,
        });
        invalid_response
            .data
            .as_mut()
            .unwrap()
            .product_variants
            .edges[0]
            .node
            .inventory_policy = "INVALID_POLICY".to_string();

        client
            .expect_query::<GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(invalid_response));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.find_product_by_id(&("0".to_string())).await;

        assert!(result.is_err());
        if let Err(DomainError::ConversionError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ConversionError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_product_by_id_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.find_product_by_id(&("123456".to_string())).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_product_by_id_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.find_product_by_id(&("123456".to_string())).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_products_no_pagination_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_products_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });
        client
            .expect_query::<GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_variants_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.find_products(&None, &None).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 250);

        assert_eq!(products[0].id(), "0");
        assert_eq!(*(products[0].status()), ProductStatus::Active);
        assert_eq!(products[0].variants()[0].id(), "0");

        assert_eq!(products[249].id(), "249");
        assert_eq!(*(products[0].status()), ProductStatus::Active);
        assert_eq!(products[249].variants()[0].id(), "249");
    }

    #[tokio::test]
    async fn test_find_products_pagination_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_products_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });
        client
            .expect_query::<GraphQLResponse<VariantsData>>()
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
        let result = repo.find_products(&limit, &offset).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 10);

        assert_eq!(products[0].id(), "20");
        assert_eq!(products[0].variants()[0].id(), "20");

        assert_eq!(products[9].id(), "29");
        assert_eq!(products[9].variants()[0].id(), "29");
    }

    #[tokio::test]
    async fn test_find_products_multiple_retrievals_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_products_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: true,
                }))
            });
        client
            .expect_query::<GraphQLResponse<VariantsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_variants_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });
        client
            .expect_query::<GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_products_response(PageOption {
                    start: 250,
                    end: 500,
                    has_next_page: false,
                }))
            });
        client
            .expect_query::<GraphQLResponse<VariantsData>>()
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
        let result = repo.find_products(&limit, &offset).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 480);

        assert_eq!(products[0].id(), "20");
        assert_eq!(products[0].variants()[0].id(), "20");

        assert_eq!(products[479].id(), "499");
        assert_eq!(products[479].variants()[0].id(), "499");
    }

    #[tokio::test]
    async fn test_find_products_empty_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<ProductsData>>()
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
        let result = repo.find_products(&limit, &offset).await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 0);
    }

    #[tokio::test]
    async fn test_find_products_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.find_products(&None, &None).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_products_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<ProductsData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = ProductRepositoryImpl::new(client);

        let result = repo.find_products(&None, &None).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
