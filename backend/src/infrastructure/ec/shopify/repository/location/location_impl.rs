use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        location::location::{Id as LocationId, Location},
    },
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            query_helper::ShopifyGQLQueryHelper,
            repository::schema::{
                common::GraphQLResponse,
                location::{LocationNode, LocationsData},
            },
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
    /// Get id on all locations.
    async fn find_all_location_ids(&self) -> Result<Vec<LocationId>, DomainError> {
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

    /// Retrieve multiple locations.
    async fn find_locations(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<Vec<Location>, DomainError> {
        let query_limit = ShopifyGQLQueryHelper::SHOPIFY_QUERY_LIMIT;

        let offset = offset.unwrap_or(0) as usize;
        let limit = limit.unwrap_or(query_limit as u32) as usize;

        let mut cursor = None;
        let mut all_nodes: Vec<LocationNode> = Vec::new();

        let first_query = ShopifyGQLQueryHelper::first_query();
        let page_info = ShopifyGQLQueryHelper::page_info();

        for i in 0..((limit + offset) / query_limit).max(1) {
            let after_query = cursor
                .as_deref()
                .map_or(String::new(), |a| format!("after: \"{}\"", a));

            let query = format!(
                "query {{
                    locations({first_query}, {after_query}) {{
                        edges {{
                            node {{
                                id
                            }}
                        }}
                        {page_info}
                    }}
                }}"
            );

            let graphql_response: GraphQLResponse<LocationsData> =
                self.client.query(&query).await?;
            if let Some(errors) = graphql_response.errors {
                log::error!("Error returned in GraphQL response. Response: {:?}", errors);
                return Err(DomainError::QueryError);
            }

            let data = graphql_response
                .data
                .ok_or(DomainError::QueryError)?
                .locations;

            if data.edges.is_empty() {
                break;
            }

            // If only the upper limit is acquired and the acquisition is less than or equal to the offset, skip it.
            cursor = data.page_info.end_cursor;
            if data.edges.len() == query_limit
                && query_limit * (i + 1) <= offset
                && data.page_info.has_next_page
            {
                log::debug!(
                    "Skip locations. index: {:?} <= index < {:?}, offset: {:?}",
                    i,
                    (i + 1) * query_limit,
                    offset,
                );
                continue;
            }

            let nodes: Vec<LocationNode> = data.edges.into_iter().map(|node| node.node).collect();

            all_nodes.extend(nodes);

            if !data.page_info.has_next_page {
                break;
            }
        }

        let domains = LocationNode::to_domains(all_nodes)?;
        log::debug!("domains.len(): {}", domains.len());

        let start = offset % query_limit;
        let end = (start + limit).min(domains.len());
        if start >= end {
            return Ok(Vec::new());
        }

        Ok(domains
            .into_iter()
            .skip(start)
            .take(end - start)
            .collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::error::error::DomainError,
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::repository::{
                location::location_impl::LocationRepositoryImpl,
                schema::{
                    address::AddressNode,
                    common::{Edges, GraphQLError, GraphQLResponse, Node, PageInfo},
                    location::{LocationNode, LocationsData},
                },
            },
        },
        usecase::repository::location_repository_interface::LocationRepository,
    };

    fn mock_location_node(id: u32) -> LocationNode {
        LocationNode {
            id: format!("gid://shopify/Location/{id}"),
            name: "Some location".to_string(),
            is_active: true,
            fulfills_online_orders: true,
            address: mock_address_node(Some(id.to_string())),
            suggested_addresses: vec![mock_address_node(Some(id.to_string()))],
        }
    }

    fn mock_address_node(address1: Option<impl Into<String>>) -> AddressNode {
        let address1 = address1.map(|a| a.into());
        AddressNode {
            address1: address1,
            address2: Some("Apt 123".to_string()),
            city: Some("Test City".to_string()),
            coordinates_validated: true,
            country: Some("Test Country".to_string()),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            province: Some("Test Province".to_string()),
            zip: Some("12345".to_string()),
            phone: Some("+1234567890".to_string()),
        }
    }

    struct PageOption {
        start: usize,
        end: usize,
        has_next_page: bool,
    }

    fn mock_locations_response(opt: PageOption) -> GraphQLResponse<LocationsData> {
        let nodes: Vec<Node<LocationNode>> = (opt.start..opt.end)
            .map(|i: usize| Node {
                node: mock_location_node(i as u32),
            })
            .collect();

        GraphQLResponse {
            data: Some(LocationsData {
                locations: Edges {
                    edges: nodes,
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
    async fn test_find_locations_no_pagination_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<LocationsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_locations_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });

        let repo = LocationRepositoryImpl::new(client);

        let result = repo.find_locations(&None, &None).await;

        assert!(result.is_ok());
        let locations = result.unwrap();
        assert_eq!(locations.len(), 250);

        assert_eq!(locations[0].id(), "0");
        assert_eq!(locations[0].name(), "Some location");
        assert_eq!(locations[0].is_active(), &true);
        assert_eq!(locations[0].fulfills_online_orders(), &true);
        assert_eq!(locations[0].address().address1(), &Some("0".to_string()));
        assert_eq!(
            locations[0].address().address2(),
            &Some("Apt 123".to_string())
        );

        assert_eq!(locations[9].id(), "9");
        assert_eq!(locations[9].name(), "Some location");
        assert_eq!(locations[9].is_active(), &true);
        assert_eq!(locations[9].fulfills_online_orders(), &true);
        assert_eq!(locations[9].address().address1(), &Some("9".to_string()));
        assert_eq!(
            locations[9].address().address2(),
            &Some("Apt 123".to_string())
        );
    }

    #[tokio::test]
    async fn test_find_locations_pagination_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<LocationsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_locations_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: false,
                }))
            });

        let repo = LocationRepositoryImpl::new(client);

        let result = repo.find_locations(&Some(10), &Some(20)).await;

        assert!(result.is_ok());
        let locations = result.unwrap();
        assert_eq!(locations.len(), 10);

        assert_eq!(locations[0].id(), "20");
        assert_eq!(locations[0].name(), "Some location");
        assert_eq!(locations[0].is_active(), &true);
        assert_eq!(locations[0].fulfills_online_orders(), &true);
        assert_eq!(locations[0].address().address1(), &Some("20".to_string()));
        assert_eq!(
            locations[0].address().address2(),
            &Some("Apt 123".to_string())
        );

        assert_eq!(locations[9].id(), "29");
        assert_eq!(locations[9].name(), "Some location");
        assert_eq!(locations[9].is_active(), &true);
        assert_eq!(locations[9].fulfills_online_orders(), &true);
        assert_eq!(locations[9].address().address1(), &Some("29".to_string()));
        assert_eq!(
            locations[9].address().address2(),
            &Some("Apt 123".to_string())
        );
    }

    #[tokio::test]
    async fn test_find_locations_multiple_retrievals_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<LocationsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_locations_response(PageOption {
                    start: 0,
                    end: 250,
                    has_next_page: true,
                }))
            });
        client
            .expect_query::<GraphQLResponse<LocationsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_locations_response(PageOption {
                    start: 250,
                    end: 500,
                    has_next_page: false,
                }))
            });

        let repo = LocationRepositoryImpl::new(client);

        let result = repo.find_locations(&Some(480), &Some(20)).await;

        assert!(result.is_ok());
        let locations = result.unwrap();
        assert_eq!(locations.len(), 480);

        assert_eq!(locations[0].id(), "20");

        assert_eq!(locations[479].id(), "499");
    }

    #[tokio::test]
    async fn test_find_locations_empty_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<LocationsData>>()
            .times(1)
            .return_once(|_| {
                Ok(mock_locations_response(PageOption {
                    start: 0,
                    end: 0,
                    has_next_page: false,
                }))
            });

        let repo = LocationRepositoryImpl::new(client);

        let result = repo.find_locations(&None, &None).await;

        assert!(result.is_ok());
        let locations = result.unwrap();
        assert!(locations.is_empty());
    }

    #[tokio::test]
    async fn test_find_locations_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<LocationsData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = LocationRepositoryImpl::new(client);

        let result = repo.find_locations(&None, &None).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_locations_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<LocationsData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = LocationRepositoryImpl::new(client);

        let result = repo.find_locations(&None, &None).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
