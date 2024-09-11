use async_trait::async_trait;
use serde_json::json;

use crate::{
    domain::{error::error::DomainError, media::media::Media, product::product::Id as ProductId},
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::repository::{
            common::schema::GraphQLResponse,
            media::schema::{MediaData, MediaSchema},
        },
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
    /// Retrieve multiple media.
    async fn get_media_by_product_id(&self, id: &ProductId) -> Result<Vec<Media>, DomainError> {
        let query = json!({
            "query": format!("query {{ node(product_id: \"gid://shopify/Product/{id}\") {{ edges {{ node {{ id fileStatus alt preview {{ image {{ url }} }} createdAt updatedAt }} }} pageInfo {{ hasPreviousPage hasNextPage startCursor endCursor }} }} }}")
        });

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
            .map(|product| product.to_domain())
            .collect();

        media_domains
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::Value;

    use crate::{
        domain::error::error::DomainError,
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::repository::{
                common::schema::{Edges, GraphQLError, GraphQLResponse, Node, PageInfo},
                media::{
                    media_impl::MediaRepositoryImpl,
                    schema::{Image, MediaData, MediaNode, MediaPreviewImage},
                },
            },
        },
        usecase::repository::media_repository_interface::MediaRepository,
    };

    fn mock_media_response(count: usize) -> GraphQLResponse<MediaData> {
        let media_nodes: Vec<Node<MediaNode>> = (0..count)
            .map(|i| Node {
                node: MediaNode {
                    id: format!("gid://shopify/Media/{i}"),
                    file_status: "UPLOADED".to_string(),
                    alt: Some(format!("Alt text for media {i}")),
                    preview: Some(MediaPreviewImage {
                        image: Some(Image {
                            url: format!("https://example.com/media/{i}.jpg"),
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
                    edges: media_nodes,
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
    async fn get_media_by_product_id_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<MediaData>>()
            .times(1)
            .return_once(|_| Ok(mock_media_response(10)));

        let repo = MediaRepositoryImpl::new(client);

        let result = repo.get_media_by_product_id(&"123456".to_string()).await;

        assert!(result.is_ok());
        let media = result.unwrap();
        assert_eq!(media.len(), 10);
        assert_eq!(media[0].id(), "gid://shopify/Media/0");
        assert_eq!(media[9].id(), "gid://shopify/Media/9");
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
            .expect_query::<Value, GraphQLResponse<MediaData>>()
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
            .expect_query::<Value, GraphQLResponse<MediaData>>()
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
            .expect_query::<Value, GraphQLResponse<MediaData>>()
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
}
