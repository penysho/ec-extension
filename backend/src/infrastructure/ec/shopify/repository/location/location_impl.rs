use async_trait::async_trait;

use crate::{
    domain::{error::error::DomainError, location::location::Id as LocationId},
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            query_helper::ShopifyGQLQueryHelper,
            repository::schema::{common::GraphQLResponse, location::LocationsData},
        },
    },
    usecase::repository::location_repository_interface::LocationRepository,
};

/// Repository for locations for Shopify.
pub struct LocationRepositoryImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> LocationRepositoryImpl<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> LocationRepository for LocationRepositoryImpl<C> {
    /// Get information on all locations.
    async fn get_all_location_ids(&self) -> Result<Vec<LocationId>, DomainError> {
        let first_query = ShopifyGQLQueryHelper::first_query();
        let page_info = ShopifyGQLQueryHelper::page_info();

        let query = format!(
            "query {{
                locations({first_query}) {{
                    edges {{
                        node {{
                            id
                        }}
                    }}
                    {page_info}
                }}
            }}"
        );

        let graphql_response: GraphQLResponse<LocationsData> = self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let ids: Vec<LocationId> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .locations
            .edges
            .into_iter()
            .map(|node| node.node.id)
            .collect();

        Ok(ids)
    }
}
