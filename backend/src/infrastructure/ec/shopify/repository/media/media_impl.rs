use async_trait::async_trait;
use serde_json::Value;

use crate::{
    domain::{
        error::error::DomainError,
        media::{associated_id::associated_id::AssociatedId, media::Media},
        product::product::Id as ProductId,
    },
    infrastructure::{
        ec::{
            ec_client_interface::ECClient,
            shopify::{
                query_helper::ShopifyGQLQueryHelper,
                repository::schema::{
                    common::GraphQLResponse,
                    media::{MediaData, MediaNode, MediaSchema},
                },
            },
        },
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
    usecase::repository::media_repository_interface::MediaRepository,
};

/// Repository for products for Shopify.
pub struct MediaRepositoryImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> MediaRepositoryImpl<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> MediaRepository for MediaRepositoryImpl<C> {
    /// Obtain media associated with a single product ID.
    async fn get_media_by_product_id(&self, id: &ProductId) -> Result<Vec<Media>, DomainError> {
        let first_query = ShopifyGQLQueryHelper::first_query();
        let page_info = ShopifyGQLQueryHelper::page_info();

        let query = format!(
            "query {{
                files({first_query}, query: \"product_id:'{id}'\") {{
                    edges {{
                        node {{
                            id
                            fileStatus
                            alt
                            preview {{
                                image {{
                                    url
                                }}
                            }}
                            createdAt
                            updatedAt
                        }}
                    }}
                    {page_info}
                }}
            }}"
        );

        let graphql_response: GraphQLResponse<MediaData> = self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response= {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let data = graphql_response.data.ok_or(DomainError::QueryError)?.files;

        let media: Vec<MediaSchema> = data
            .edges
            .into_iter()
            .map(|node| MediaSchema::from(node.node))
            .collect();

        let media_domains: Result<Vec<Media>, DomainError> = media
            .into_iter()
            .map(|product| product.to_domain(Some(AssociatedId::Product(id.to_string()))))
            .collect();

        media_domains
    }

    /// Obtain media associated with multiple product IDs.
    async fn get_media_by_product_ids(
        &self,
        product_ids: Vec<&ProductId>,
    ) -> Result<Vec<Media>, DomainError> {
        let first_query = ShopifyGQLQueryHelper::first_query();

        let mut query = String::from("query { ");
        for (i, id) in product_ids.iter().enumerate() {
            let alias = format!("i{}", i);
            let query_part = format!(
                "{}: files({}, query: \"product_id:'{}'\") {{
                    edges {{
                        node {{
                            id
                            fileStatus
                            alt
                            preview {{
                                image {{
                                    url
                                }}
                            }}
                            createdAt
                            updatedAt
                        }}
                    }}
                }}",
                alias,
                first_query,
                ShopifyGQLQueryHelper::remove_gid_prefix(id)
            );
            query.push_str(&query_part);
        }
        query.push_str(" }");

        let graphql_response: GraphQLResponse<Value> = self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response= {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let data = graphql_response.data.ok_or(DomainError::QueryError)?;

        if !data.is_object() {
            log::error!("Expected data to be an object, but got: {:?}", data);
            return Err(DomainError::QueryError);
        }

        let mut media_schemas = Vec::new();
        for (i, _) in product_ids.iter().enumerate() {
            let alias = format!("i{}", i);

            if let Some(file_data) = data.get(&alias).and_then(|d| d.as_object()) {
                if let Some(files) = file_data.get("edges").and_then(|f| f.as_array()) {
                    for edge in files {
                        let node = &edge["node"];
                        let v: MediaNode = serde_json::from_value(node.clone()).map_err(|e| {
                            InfrastructureErrorMapper::to_domain(InfrastructureError::ParseError(e))
                        })?;
                        let media_schema = MediaSchema::from(v);
                        media_schemas.push(media_schema);
                    }
                }
            } else {
                log::error!("No data found for alias: {}", alias);
            }
        }

        let media_domains: Result<Vec<Media>, DomainError> = MediaSchema::to_domains(
            media_schemas,
            product_ids
                .into_iter()
                .map(|id| Some(AssociatedId::Product(id.to_string())))
                .collect(),
        );

        media_domains
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::{json, Value};

    use crate::{
        domain::{error::error::DomainError, media::media::MediaStatus},
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::repository::{
                media::media_impl::MediaRepositoryImpl,
                schema::{
                    common::{Edges, GraphQLError, GraphQLResponse, Node, PageInfo},
                    media::{Image, MediaData, MediaNode, MediaPreviewImage},
                },
            },
        },
        usecase::repository::media_repository_interface::MediaRepository,
    };

    fn mock_media_response(count: usize) -> GraphQLResponse<MediaData> {
        let nodes: Vec<Node<MediaNode>> = (0..count)
            .map(|i| Node {
                node: MediaNode {
                    id: format!("gid://shopify/MediaImage/{i}"),
                    file_status: "UPLOADED".to_string(),
                    alt: Some(format!("Alt text for media {i}")),
                    preview: Some(MediaPreviewImage {
                        image: Some(Image {
                            url: format!("https://example.com/MediaImage/{i}.jpg"),
                        }),
                    }),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
            })
            .collect();

        GraphQLResponse {
            data: Some(MediaData {
                files: Edges {
                    edges: nodes,
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

    fn mock_media_response_by_alias() -> GraphQLResponse<Value> {
        let mock_graphql_response = json!({
            "data": {
                "i0": {
                    "edges": [
                        {
                            "node": {
                                "alt": "Alt text for media 0",
                                "createdAt": "2024-07-30T15:37:45Z",
                                "fileStatus": "READY",
                                "id": "gid://shopify/MediaImage/0",
                                "updatedAt": "2024-07-30T15:37:45Z",
                                "preview": {
                                    "image": {
                                        "id": "gid://shopify/MediaImage/0",
                                        "url": "https://example.com/image0.jpg",
                                    }
                                }
                            }
                        }
                    ]
                },
                "i1": {
                    "edges": [
                        {
                            "node": {
                                "alt": "Alt text for media 1",
                                "createdAt": "2024-07-30T15:37:45Z",
                                "fileStatus": "READY",
                                "id": "gid://shopify/MediaImage/1",
                                "updatedAt": "2024-07-30T15:37:45Z",
                                "preview": {
                                    "image": {
                                        "id": "gid://shopify/MediaImage/1",
                                        "url": "https://example.com/image1.jpg",
                                    }
                                }
                            }
                        }
                    ]
                },
            }
        });

        serde_json::from_value(mock_graphql_response).unwrap()
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
    async fn get_media_by_product_id_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<MediaData>>()
            .times(1)
            .return_once(|_| Ok(mock_media_response(10)));

        let repo = MediaRepositoryImpl::new(client);

        let result = repo.get_media_by_product_id(&"123456".to_string()).await;

        assert!(result.is_ok());
        let media = result.unwrap();
        assert_eq!(media.len(), 10);
        assert_eq!(media[0].id(), "0");
        assert_eq!(*media[0].status(), MediaStatus::Active);
        assert_eq!(
            media[0].published_src().as_ref().unwrap().value(),
            "https://example.com/MediaImage/0.jpg"
        );
        assert_eq!(media[0].alt().as_deref().unwrap(), "Alt text for media 0");

        assert_eq!(media[9].id(), "9");
        assert_eq!(*media[9].status(), MediaStatus::Active);
        assert_eq!(
            media[9].published_src().as_ref().unwrap().value(),
            "https://example.com/MediaImage/9.jpg"
        );
        assert_eq!(media[9].alt().as_deref().unwrap(), "Alt text for media 9");
    }

    #[tokio::test]
    async fn get_media_by_product_id_with_invalid_domain_conversion() {
        let mut client = MockECClient::new();

        let mut invalid_variant = mock_media_response(1);
        invalid_variant.data.as_mut().unwrap().files.edges[0]
            .node
            .file_status = "UPLOADED".to_string();
        invalid_variant.data.as_mut().unwrap().files.edges[0]
            .node
            .preview = None;

        client
            .expect_query::<GraphQLResponse<MediaData>>()
            .times(1)
            .return_once(|_| Ok(invalid_variant));

        let repo = MediaRepositoryImpl::new(client);

        let result = repo.get_media_by_product_id(&"123456".to_string()).await;

        assert!(result.is_err());
        if let Err(DomainError::ValidationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ValidationError, but got something else");
        }
    }

    #[tokio::test]
    async fn get_media_by_product_id_with_graphql_error() {
        let mut client = MockECClient::new();

        let graphql_response_with_error = mock_with_error();

        client
            .expect_query::<GraphQLResponse<MediaData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_error));

        let repo = MediaRepositoryImpl::new(client);

        let result = repo.get_media_by_product_id(&"123456".to_string()).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn get_media_by_product_id_with_missing_data() {
        let mut client = MockECClient::new();

        let graphql_response_with_no_data = mock_with_no_data();

        client
            .expect_query::<GraphQLResponse<MediaData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_no_data));

        let repo = MediaRepositoryImpl::new(client);

        let result = repo.get_media_by_product_id(&"123456".to_string()).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn get_media_by_product_ids_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<Value>>()
            .times(1)
            .return_once(|_| Ok(mock_media_response_by_alias()));

        let repo = MediaRepositoryImpl::new(client);

        let result = repo
            .get_media_by_product_ids(vec![&"1".to_string(), &"2".to_string()])
            .await;

        assert!(result.is_ok());
        let media = result.unwrap();
        assert_eq!(media.len(), 2);
        assert_eq!(media[0].id(), "0");
        assert_eq!(media[1].id(), "1");
    }

    #[tokio::test]
    async fn get_media_by_product_ids_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<Value>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = MediaRepositoryImpl::new(client);

        let result = repo
            .get_media_by_product_ids(vec![&"1".to_string(), &"2".to_string()])
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn get_media_by_product_ids_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<Value>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = MediaRepositoryImpl::new(client);

        let result = repo
            .get_media_by_product_ids(vec![&"1".to_string(), &"2".to_string()])
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
