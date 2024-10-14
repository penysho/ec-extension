use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        inventory_level::{
            inventory_change::inventory_change::InventoryChange, inventory_level::InventoryLevel,
        },
        location::location::Id as LocationId,
        product::variant::sku::sku::Sku,
    },
    infrastructure::{
        ec::{
            ec_client_interface::ECClient,
            shopify::{
                query_helper::ShopifyGQLQueryHelper,
                repository::schema::{
                    common::GraphQLResponse,
                    inventory_change::{
                        InventoryAdjustQuantitiesData, InventoryAdjustQuantitiesInput,
                    },
                    inventory_item::InventoryItemsData,
                    inventory_level::InventoryLevelNode,
                },
            },
        },
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
    usecase::repository::inventory_level_repository_interface::InventoryLevelRepository,
};

/// Repository for inventories for Shopify.
pub struct InventoryLevelRepositoryImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> InventoryLevelRepositoryImpl<C> {
    const SHOPIFY_ALL_INVENTORY_NAMES_FOR_QUERY: &'static str =
        "[\"incoming\",\"available\",\"committed\",\"reserved\",\"damaged\",\"safety_stock\"]";

    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> InventoryLevelRepository for InventoryLevelRepositoryImpl<C> {
    /// Get inventory level information by sku.
    async fn find_inventory_level_by_sku(
        &self,
        sku: &Sku,
        location_id: &LocationId,
    ) -> Result<Option<InventoryLevel>, DomainError> {
        let first_query = ShopifyGQLQueryHelper::first_query();
        let page_info = ShopifyGQLQueryHelper::page_info();
        let inventory_names = Self::SHOPIFY_ALL_INVENTORY_NAMES_FOR_QUERY;
        let sku = sku.value();
        let location_id = ShopifyGQLQueryHelper::add_location_gid_prefix(location_id);

        let query = format!(
            "query {{
                inventoryItems({first_query}, query: \"sku:{sku}\") {{
                    edges {{
                        node {{
                            id
                            variant {{
                                id
                            }}
                            requiresShipping
                            tracked
                            createdAt
                            updatedAt
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
        );

        let graphql_response: GraphQLResponse<InventoryItemsData> =
            self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let nodes: Vec<InventoryLevelNode> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .inventory_items
            .edges
            .into_iter()
            .filter_map(|node| node.node.inventory_level)
            .collect();

        let domains = InventoryLevelNode::to_domains(nodes)?;

        if domains.is_empty() {
            log::info!("No inventory level found for sku: {sku}, location: {location_id}");
            return Ok(None);
        }
        Ok(domains.into_iter().next())
    }

    /// Update inventory quantity.
    async fn update(&self, inventory_change: InventoryChange) -> Result<(), DomainError> {
        let schema = InventoryAdjustQuantitiesInput::from(inventory_change);
        let input = serde_json::to_value(schema).map_err(|e| {
            log::error!("Failed to parse the request structure. Error: {:?}", e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::ParseError(e))
        })?;

        let user_errors = ShopifyGQLQueryHelper::user_errors();

        let query = format!(
            "mutation inventoryAdjustQuantities($input: InventoryAdjustQuantitiesInput!) {{
                inventoryAdjustQuantities(input: $input) {{
                    {user_errors}
                }}
            }}",
        );

        let graphql_response: GraphQLResponse<InventoryAdjustQuantitiesData> =
            self.client.mutation(&query, &input).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let user_errors = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .inventory_adjust_quantities
            .user_errors;

        if !user_errors.is_empty() {
            log::error!("UserErrors returned. userErrors: {:?}", user_errors);
            return Err(DomainError::QueryError);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::Value;

    use crate::{
        domain::{
            error::error::DomainError,
            inventory_level::{
                inventory_change::{
                    change::{
                        change::Change, ledger_document_uri::ledger_document_uri::LedgerDocumentUri,
                    },
                    inventory_change::{InventoryChange, InventoryChangeReason},
                },
                quantity::quantity::InventoryType,
            },
            product::variant::sku::sku::Sku,
        },
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::repository::{
                inventory_level::inventory_level_impl::InventoryLevelRepositoryImpl,
                schema::{
                    common::{
                        Edges, GraphQLError, GraphQLResponse, Node, PageInfo, UserError, UserErrors,
                    },
                    inventory_change::InventoryAdjustQuantitiesData,
                    inventory_item::{InventoryItemNode, InventoryItemsData, VariantIdNode},
                    inventory_level::{InventoryItemIdNode, InventoryLevelNode, QuantityNode},
                    location::LocationNode,
                },
            },
        },
        usecase::repository::inventory_level_repository_interface::InventoryLevelRepository,
    };

    fn mock_inventory_item_node(id: u32) -> InventoryItemNode {
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

    fn mock_inventory_items_response(count: usize) -> GraphQLResponse<InventoryItemsData> {
        let nodes: Vec<Node<InventoryItemNode>> = (0..count)
            .map(|i: usize| Node {
                node: mock_inventory_item_node(i as u32),
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

    fn mock_inventory_change_domain() -> InventoryChange {
        InventoryChange::new(
            InventoryType::Committed,
            InventoryChangeReason::Correction,
            vec![Change::new(
                10,
                "inventory_item_123",
                Some(LedgerDocumentUri::new("https://example.com/document").unwrap()),
                "location_456",
            )
            .unwrap()],
        )
        .unwrap()
    }

    fn mock_inventory_adjust_quantities_response() -> GraphQLResponse<InventoryAdjustQuantitiesData>
    {
        GraphQLResponse {
            data: Some(InventoryAdjustQuantitiesData {
                inventory_adjust_quantities: UserErrors {
                    user_errors: vec![],
                },
            }),
            errors: None,
        }
    }

    fn mock_inventory_adjust_quantities_response_with_user_errors(
    ) -> GraphQLResponse<InventoryAdjustQuantitiesData> {
        GraphQLResponse {
            data: Some(InventoryAdjustQuantitiesData {
                inventory_adjust_quantities: UserErrors {
                    user_errors: vec![UserError {
                        code: None,
                        field: vec!["quantity".to_string()],
                        message: "Quantity must be positive".to_string(),
                    }],
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
    async fn test_find_inventory_level_by_sku_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<InventoryItemsData>>()
            .times(1)
            .return_once(|_| Ok(mock_inventory_items_response(1)));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo
            .find_inventory_level_by_sku(&Sku::new("0".to_string()).unwrap(), &"0".to_string())
            .await;

        assert!(result.is_ok());
        let inventory_level = result.unwrap().unwrap();
        assert_eq!(inventory_level.id(), "0");
        assert_eq!(inventory_level.location_id(), "0");
        assert_eq!(
            inventory_level
                .quantities()
                .into_iter()
                .map(|q| *q.quantity())
                .collect::<Vec<i32>>(),
            [1, 2, 3]
        );
        assert_eq!(
            *(inventory_level.quantities()[0].inventory_type()),
            InventoryType::Available
        );
    }

    #[tokio::test]
    async fn test_find_inventory_level_by_sku_with_invalid_inventory_type() {
        let mut client = MockECClient::new();

        let mut invalid_response = mock_inventory_items_response(1);

        invalid_response
            .data
            .as_mut()
            .unwrap()
            .inventory_items
            .edges[0]
            .node
            .inventory_level
            .as_mut()
            .unwrap()
            .quantities[0]
            .name = "invalid".to_string();

        client
            .expect_query::<GraphQLResponse<InventoryItemsData>>()
            .times(1)
            .return_once(|_| Ok(invalid_response));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo
            .find_inventory_level_by_sku(&Sku::new("0".to_string()).unwrap(), &"0".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::ConversionError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ConversionError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_inventory_level_by_sku_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<InventoryItemsData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo
            .find_inventory_level_by_sku(&Sku::new("0".to_string()).unwrap(), &"0".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_inventory_level_by_sku_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<InventoryItemsData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo
            .find_inventory_level_by_sku(&Sku::new("0".to_string()).unwrap(), &"0".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_update_success() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<InventoryAdjustQuantitiesData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_inventory_adjust_quantities_response()));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo.update(mock_inventory_change_domain()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_with_user_errors() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<InventoryAdjustQuantitiesData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_inventory_adjust_quantities_response_with_user_errors()));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo.update(mock_inventory_change_domain()).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_update_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<InventoryAdjustQuantitiesData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_with_error()));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo.update(mock_inventory_change_domain()).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_update_with_no_data() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<InventoryAdjustQuantitiesData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_with_no_data()));

        let repo = InventoryLevelRepositoryImpl::new(client);

        let result = repo.update(mock_inventory_change_domain()).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
