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
    async fn get_media_by_product_id(&self, id: ProductId) -> Result<Vec<Media>, DomainError> {
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
