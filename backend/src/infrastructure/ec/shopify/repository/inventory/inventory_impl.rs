use async_trait::async_trait;
use serde_json::json;

use crate::{
    domain::{
        error::error::DomainError,
        inventory::inventory::Inventory,
        location::location::Id as LocationId,
        product::{product::Id as ProductId, variant::sku::sku::Sku},
    },
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            query_helper::ShopifyGQLQueryHelper,
            repository::schema::{
                common::GraphQLResponse,
                inventory::{InventoryItemSchema, InventoryItemsData, VariantsDataForInventory},
            },
        },
    },
    usecase::repository::inventory_repository_interface::InventoryRepository,
};

/// Repository for inventories for Shopify.
pub struct InventoryRepositoryImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> InventoryRepositoryImpl<C> {
    const SHOPIFY_ALL_INVENTORY_NAMES_FOR_QUERY: &'static str =
        "\"incoming,available,committed,reserved,damaged,safety_stock\"";

    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> InventoryRepository for InventoryRepositoryImpl<C> {
    /// Get product inventory information by product id.
    async fn get_inventories_by_product_id(
        &self,
        product_id: &ProductId,
        location_id: &LocationId,
    ) -> Result<Vec<Inventory>, DomainError> {
        let first_query = ShopifyGQLQueryHelper::first_query();
        let page_info = ShopifyGQLQueryHelper::page_info();
        let inventory_names = Self::SHOPIFY_ALL_INVENTORY_NAMES_FOR_QUERY;

        let query = json!({
            "query": format!(
                "query {{
                    productVariants({first_query}, query: \"product_id:'{product_id}'\") {{
                        edges {{
                            node {{
                                id
                                inventoryItem {{
                                    id
                                    variant {{
                                        id
                                    }}
                                    inventoryLevel(locationId: \"{location_id}\") {{
                                        id
                                        location {{
                                            id
                                        }}
                                        quantities(names: {inventory_names}) {{
                                            name
                                            quantity
                                        }}
                                    }}
                                    requiresShipping
                                    tracked
                                    createdAt
                                    updatedAt
                                }}
                            }}
                        }}
                        {page_info}
                    }}
                }}"
            )
        });

        let graphql_response: GraphQLResponse<VariantsDataForInventory> =
            self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let inventories: Vec<InventoryItemSchema> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .product_variants
            .edges
            .into_iter()
            .map(|node| InventoryItemSchema::from(node.node.inventory_item))
            .collect();

        InventoryItemSchema::to_domains(inventories)
    }

    /// Get product inventory information by sku.
    async fn get_inventories_by_sku(
        &self,
        sku: &Sku,
        location_id: &LocationId,
    ) -> Result<Inventory, DomainError> {
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
                                variant {{
                                    id
                                }}
                                inventoryLevel(locationId: \"{location_id}\") {{
                                    id
                                    location {{
                                        id
                                    }}
                                    quantities(names: {inventory_names}) {{
                                        name
                                        quantity
                                    }}
                                }}
                                requiresShipping
                                tracked
                                createdAt
                                updatedAt
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

        let inventories: Vec<InventoryItemSchema> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .inventory_items
            .edges
            .into_iter()
            .map(|node| InventoryItemSchema::from(node.node))
            .collect();

        let domains = InventoryItemSchema::to_domains(inventories)?;

        if domains.is_empty() {
            log::error!("No inventory found for sku: {}", sku);
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
            error::error::DomainError,
            inventory::inventory_level::quantity::quantity::InventoryType,
            product::variant::sku::sku::Sku,
        },
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::repository::{
                inventory::inventory_impl::InventoryRepositoryImpl,
                schema::{
                    common::{Edges, GraphQLError, GraphQLResponse, Node, PageInfo},
                    inventory::{
                        InventoryItemNode, InventoryItemsData, InventoryLevelNode, QuantityNode,
                        VariantIdNode, VariantNodeForInventory, VariantsDataForInventory,
                    },
                    location::LocationNode,
                },
            },
        },
        usecase::repository::inventory_repository_interface::InventoryRepository,
    };

    fn mock_inventory(id: u32) -> InventoryItemNode {
        InventoryItemNode {
            id: format!("gid://shopify/InventoryItem/{id}"),
            variant: VariantIdNode {
                id: format!("gid://shopify/ProductVariant/{id}"),
            },
            inventory_level: Some(InventoryLevelNode {
                id: format!("gid://shopify/InventoryLevel/{id}"),
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

    fn mock_inventories_response_for_variant_data(
        count: usize,
    ) -> GraphQLResponse<VariantsDataForInventory> {
        let nodes: Vec<Node<VariantNodeForInventory>> = (0..count)
            .map(|i| Node {
                node: VariantNodeForInventory {
                    inventory_item: mock_inventory(i as u32),
                },
            })
            .collect();

        GraphQLResponse {
            data: Some(VariantsDataForInventory {
                product_variants: Edges {
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
    async fn get_inventories_by_product_id_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<VariantsDataForInventory>>()
            .times(1)
            .return_once(|_| Ok(mock_inventories_response_for_variant_data(10)));

        let repo = InventoryRepositoryImpl::new(client);

        let result = repo
            .get_inventories_by_product_id(&"0".to_string(), &"0".to_string())
            .await;

        assert!(result.is_ok());
        let inventories = result.unwrap();
        assert_eq!(inventories.len(), 10);
        assert_eq!(inventories[0].id(), "0");
        assert_eq!(inventories[0].variant_id(), "0");
        assert_eq!(inventories[0].inventory_levels()[0].id(), "0");
        assert_eq!(inventories[0].inventory_levels()[0].location_id(), "0");
        assert_eq!(
            inventories[0].inventory_levels()[0]
                .quantities()
                .into_iter()
                .map(|q| q.quantity().clone())
                .collect::<Vec<u32>>(),
            [1, 2, 3]
        );
        assert_eq!(
            *(inventories[0].inventory_levels()[0].quantities()[0].inventory_type()),
            InventoryType::Available
        );

        assert_eq!(inventories[9].id(), "9");
        assert_eq!(inventories[9].variant_id(), "9");
        assert_eq!(inventories[9].inventory_levels()[0].id(), "9");
        assert_eq!(inventories[9].inventory_levels()[0].location_id(), "9");
    }

    #[tokio::test]
    async fn get_inventories_by_product_id_with_invalid_domain_conversion() {
        let mut client = MockECClient::new();

        let mut invalid_variant = mock_inventories_response_for_variant_data(1);
        invalid_variant
            .data
            .as_mut()
            .unwrap()
            .product_variants
            .edges[0]
            .node
            .inventory_item
            .variant
            .id = "".to_string();

        client
            .expect_query::<Value, GraphQLResponse<VariantsDataForInventory>>()
            .times(1)
            .return_once(|_| Ok(invalid_variant));

        let repo = InventoryRepositoryImpl::new(client);

        let result = repo
            .get_inventories_by_product_id(&"0".to_string(), &"0".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::ValidationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ValidationError, but got something else");
        }
    }

    #[tokio::test]
    async fn get_inventories_by_product_id_with_graphql_error() {
        let mut client = MockECClient::new();

        let graphql_response_with_error = mock_with_error();

        client
            .expect_query::<Value, GraphQLResponse<VariantsDataForInventory>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_error));

        let repo = InventoryRepositoryImpl::new(client);

        let result = repo
            .get_inventories_by_product_id(&"0".to_string(), &"0".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn get_inventories_by_product_id_with_missing_data() {
        let mut client = MockECClient::new();

        let graphql_response_with_no_data = mock_with_no_data();

        client
            .expect_query::<Value, GraphQLResponse<VariantsDataForInventory>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_no_data));

        let repo = InventoryRepositoryImpl::new(client);

        let result = repo
            .get_inventories_by_product_id(&"0".to_string(), &"0".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn get_inventories_by_sku_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<Value, GraphQLResponse<InventoryItemsData>>()
            .times(1)
            .return_once(|_| Ok(mock_inventories_response_for_inventory_items_data(1)));

        let repo = InventoryRepositoryImpl::new(client);

        let result = repo
            .get_inventories_by_sku(&Sku::new("0".to_string()).unwrap(), &"0".to_string())
            .await;

        assert!(result.is_ok());
        let inventory = result.unwrap();
        assert_eq!(inventory.id(), "0");
        assert_eq!(inventory.variant_id(), "0");
        assert_eq!(inventory.inventory_levels()[0].id(), "0");
        assert_eq!(inventory.inventory_levels()[0].location_id(), "0");
        assert_eq!(
            inventory.inventory_levels()[0]
                .quantities()
                .into_iter()
                .map(|q| q.quantity().clone())
                .collect::<Vec<u32>>(),
            [1, 2, 3]
        );
        assert_eq!(
            *(inventory.inventory_levels()[0].quantities()[0].inventory_type()),
            InventoryType::Available
        );
    }

    #[tokio::test]
    async fn get_inventories_by_sku_with_graphql_error() {
        let mut client = MockECClient::new();

        let graphql_response_with_error = mock_with_error();

        client
            .expect_query::<Value, GraphQLResponse<InventoryItemsData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_error));

        let repo = InventoryRepositoryImpl::new(client);

        let result = repo
            .get_inventories_by_sku(&Sku::new("0".to_string()).unwrap(), &"0".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn get_inventories_by_sku_with_missing_data() {
        let mut client = MockECClient::new();

        let graphql_response_with_no_data = mock_with_no_data();

        client
            .expect_query::<Value, GraphQLResponse<InventoryItemsData>>()
            .times(1)
            .return_once(|_| Ok(graphql_response_with_no_data));

        let repo = InventoryRepositoryImpl::new(client);

        let result = repo
            .get_inventories_by_sku(&Sku::new("0".to_string()).unwrap(), &"0".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
