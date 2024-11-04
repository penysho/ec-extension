use async_trait::async_trait;

use crate::{
    domain::error::error::DomainError,
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            gql_helper::ShopifyGQLHelper, query_service::schema::product::RelatedProductsData,
            schema::GraphQLResponse,
        },
    },
    usecase::query_service::{
        dto::product::ProductDTO,
        product_query_service_interface::{ProductQueryService, RelatedProductFilter},
    },
};

/// Query service for products for Shopify.
pub struct ProductQueryServiceImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> ProductQueryServiceImpl<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> ProductQueryService for ProductQueryServiceImpl<C> {
    async fn search_related_products(
        &self,
        filter: &RelatedProductFilter,
    ) -> Result<Vec<ProductDTO>, DomainError> {
        let first_query = ShopifyGQLHelper::first_query();
        let page_info = ShopifyGQLHelper::page_info();
        let category_id = &filter.category_id;

        let query = format!(
            "query {{
                    products({first_query}, sortKey: UPDATED_AT, query: \"category_id:{category_id}\") {{
                        edges {{
                            node {{
                                id
                                title
                                handle
                                vender
                                priceRangeV2 {{
                                    maxVariantPrice {{
                                        amount
                                    }}
                                }}
                                featuredMedia {{
                                    preview {{
                                        image {{
                                            url
                                        }}
                                    }}
                                }}
                            }}
                        }}
                        {page_info}
                    }}
                }}"
        );

        let response: GraphQLResponse<RelatedProductsData> = self.client.query(&query).await?;
        if let Some(errors) = response.errors {
            log::error!(
                "Error returned in Products response. Response: {:?}",
                errors
            );
            return Err(DomainError::QueryError);
        }

        Ok(response
            .data
            .ok_or(DomainError::QueryError)?
            .products
            .edges
            .into_iter()
            .map(|node| node.node.into())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::ec::{
        ec_client_interface::MockECClient,
        shopify::{
            query_service::schema::product::{
                ImageNode, MediaNode, MediaPreviewImageNode, MoneyV2Node, PriceRangeV2Node,
                ProductNode,
            },
            schema::{Edges, GraphQLError, Node, PageInfo},
        },
    };

    use super::*;

    struct PageOption {
        start: usize,
        end: usize,
        has_next_page: bool,
    }

    fn mock_related_products_response(opt: PageOption) -> GraphQLResponse<RelatedProductsData> {
        let products: Vec<Node<ProductNode>> = (opt.start..opt.end)
            .map(|i: usize| Node {
                node: ProductNode {
                    id: format!("gid://shopify/Product/{i}"),
                    title: format!("Test Product {i}"),
                    handle: format!("test-product-{i}"),
                    vendor: format!("Test Vendor {i}"),
                    price_range_v2: PriceRangeV2Node {
                        max_variant_price: MoneyV2Node {
                            amount: format!("{i}"),
                        },
                    },
                    featured_media: Some(MediaNode {
                        preview: Some(MediaPreviewImageNode {
                            image: Some(ImageNode {
                                url: format!("https://example.com/MediaImage/{i}.jpg"),
                            }),
                        }),
                    }),
                },
            })
            .collect();

        GraphQLResponse {
            data: Some(RelatedProductsData {
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
    async fn test_search_related_productsd_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<RelatedProductsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_related_products_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });

        let repo = ProductQueryServiceImpl::new(client);

        let result = repo
            .search_related_products(&RelatedProductFilter {
                category_id: "0".to_string(),
            })
            .await;

        assert!(result.is_ok());
        let products = result.unwrap();
        assert_eq!(products.len(), 250);

        assert_eq!(products[0].id, "0");
        assert_eq!(products[0].name, "Test Product 0");
        assert_eq!(products[0].handle, "test-product-0");
        assert_eq!(products[0].vendor, "Test Vendor 0");
        assert_eq!(products[0].price, 0.0);
        assert_eq!(
            products[0].featured_media_url,
            Some("https://example.com/MediaImage/0.jpg".to_string())
        );

        assert_eq!(products[249].id, "249");
        assert_eq!(products[249].name, "Test Product 249");
        assert_eq!(products[249].handle, "test-product-249");
        assert_eq!(products[249].vendor, "Test Vendor 249");
        assert_eq!(products[249].price, 249.0);
        assert_eq!(
            products[249].featured_media_url,
            Some("https://example.com/MediaImage/249.jpg".to_string())
        );
    }

    #[tokio::test]
    async fn test_search_related_productsd_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<RelatedProductsData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = ProductQueryServiceImpl::new(client);

        let result = repo
            .search_related_products(&RelatedProductFilter {
                category_id: "0".to_string(),
            })
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_search_related_productsd_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<RelatedProductsData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = ProductQueryServiceImpl::new(client);

        let result = repo
            .search_related_products(&RelatedProductFilter {
                category_id: "0".to_string(),
            })
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
