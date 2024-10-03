use async_trait::async_trait;
use serde_json::json;

use crate::{
    domain::{
        error::error::DomainError, inventory_level::inventory_level::InventoryLevel,
        location::location::Id as LocationId, product::variant::sku::sku::Sku,
    },
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            query_helper::ShopifyGQLQueryHelper,
            repository::schema::{
                common::GraphQLResponse,
                inventory::{InventoryItemsData, InventoryLevelSchema},
            },
        },
    },
    usecase::repository::inventory_level_repository_interface::InventoryLevelRepository,
};

/// Repository for inventories for Shopify.
pub struct InventoryLevelRepositoryImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> InventoryLevelRepositoryImpl<C> {
    const SHOPIFY_ALL_INVENTORY_NAMES_FOR_QUERY: &'static str =
        "\"incoming,available,committed,reserved,damaged,safety_stock\"";

    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> InventoryLevelRepository for InventoryLevelRepositoryImpl<C> {
    /// Get inventory level information by sku.
    async fn get_inventory_level_by_sku(
        &self,
        sku: &Sku,
        location_id: &LocationId,
    ) -> Result<InventoryLevel, DomainError> {
        let first_query = ShopifyGQLQueryHelper::first_query();
        let page_info = ShopifyGQLQueryHelper::page_info();
        let inventory_names = Self::SHOPIFY_ALL_INVENTORY_NAMES_FOR_QUERY;
        let sku = sku.value();

        let query = json!({
            "query": format!(
                "query {{
                    inventoryItems({first_query}, query: \"sku:{sku}\") {{
                        edges {{
                            node {{
                                id
                                inventoryLevel(locationId: \"{location_id}\") {{
                                    id
                                    item {{
                                        id
                                    }}
                                    location {{
                                        id
                                    }}
                                    quantities(names: {inventory_names}) {{
                                        name
                                        quantity
                                    }}
                                }}
                            }}
                        }}
                        {page_info}
                    }}
                }}"
            )
        });

        let graphql_response: GraphQLResponse<InventoryItemsData> =
            self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let inventories: Vec<InventoryLevelSchema> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .inventory_items
            .edges
            .into_iter()
            .filter_map(|node| {
                node.node
                    .inventory_level
                    .map(|level| InventoryLevelSchema::from(level))
            })
            .collect();

        let domains = InventoryLevelSchema::to_domains(inventories)?;

        if domains.is_empty() {
            log::error!("No inventory level found for sku: {}", sku);
            return Err(DomainError::NotFound);
        }
        Ok(domains.into_iter().next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::Value;

    use crate::{
        domain::{
            error::error::DomainError, inventory_level::quantity::quantity::InventoryType,
            product::variant::sku::sku::Sku,
        },
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::repository::{
                inventory_level::inventory_level_impl::InventoryLevelRepositoryImpl,
                schema::{
                    common::{Edges, GraphQLError, GraphQLResponse, Node, PageInfo},
                    inventory::{
                        InventoryItemIdNode, InventoryItemNode, InventoryItemsData,
                        InventoryLevelNode, QuantityNode, VariantIdNode,
                    },
                    location::LocationNode,
                },
            },
        },
        usecase::repository::inventory_level_repository_interface::InventoryLevelRepository,
    };

    fn mock_inventory(id: u32) -> InventoryItemNode {
        InventoryItemNode {
            id: format!("gid://shopify/InventoryItem/{id}"),
            variant: VariantIdNode {
                id: format!("gid://shopify/ProductVariant/{id}"),
            },
            inventory_level: Some(InventoryLevelNode {
                id: format!("gid://shopify/InventoryLevel/{id}"),
                item: InventoryItemIdNode {
                    id: format!("gid://shopify/InventoryItem/{id}"),
                },
                location: LocationNode {
                    id: format!("gid://shopify/Location/{id}"),
                },
                quantities: vec![
                    QuantityNode {
                        quantity: 1,
                        name: "available".to_string(),
                    },
                    QuantityNode {
                        quantity: 2,
                        name: "committed".to_string(),
                    },
                    QuantityNode {
                        quantity: 3,
                        name: "reserved".to_string(),
                    },
                ],
            }),
            requires_shipping: true,
            tracked: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn mock_inventories_response_for_inventory_items_data(
        count: usize,
    ) -> GraphQLResponse<InventoryItemsData> {
        let nodes: Vec<Node<InventoryItemNode>> = (0..count)
            .map(|i: usize| Node {
                node: mock_inventory(i as u32),
            })
            .collect();

        GraphQLResponse {
            data: Some(InventoryItemsData {
                inventory_items: Edges {
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
    async fn get_inventory_level_by_sku_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<InventoryItemsData>>()
            .times(1)
            .return_once(|_| Ok(mock_inventories_response_for_inventory_items_data(1)));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo
            .get_inventory_level_by_sku(&Sku::new("0".to_string()).unwrap(), &"0".to_string())
            .await;

        assert!(result.is_ok());
        let inventory_level = result.unwrap();
        assert_eq!(inventory_level.id(), "0");
        assert_eq!(inventory_level.location_id(), "0");
        assert_eq!(
            inventory_level
                .quantities()
                .into_iter()
                .map(|q| q.quantity().clone())
                .collect::<Vec<u32>>(),
            [1, 2, 3]
        );
        assert_eq!(
            *(inventory_level.quantities()[0].inventory_type()),
            InventoryType::Available
        );
    }

    #[tokio::test]
    async fn get_inventory_level_by_sku_with_graphql_error() {
        let mut client = MockECClient::new();

        let graphql_response_with_error = mock_with_error();

        client
            .expect_query::<Value, GraphQLResponse<InventoryItemsData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_error));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo
            .get_inventory_level_by_sku(&Sku::new("0".to_string()).unwrap(), &"0".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn get_inventory_level_by_sku_with_missing_data() {
        let mut client = MockECClient::new();

        let graphql_response_with_no_data = mock_with_no_data();

        client
            .expect_query::<Value, GraphQLResponse<InventoryItemsData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_no_data));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo
            .get_inventory_level_by_sku(&Sku::new("0".to_string()).unwrap(), &"0".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
