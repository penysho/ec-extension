use async_trait::async_trait;
use serde_json::json;

use crate::{
    domain::{
        error::error::DomainError, inventory::inventory::Inventory,
        location::location::Id as LocationId, product::product::Id as ProductId,
    },
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            query_helper::ShopifyGQLQueryHelper,
            repository::schema::{
                common::GraphQLResponse,
                inventory::{InventoryItemSchema, VariantsDataForInventory},
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
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> InventoryRepository for InventoryRepositoryImpl<C> {
    /// Obtain product inventory information.
    async fn get_inventories_by_product_id(
        &self,
        product_id: &ProductId,
        location_id: &LocationId,
    ) -> Result<Vec<Inventory>, DomainError> {
        let first_query = ShopifyGQLQueryHelper::first_query();
        let page_info = ShopifyGQLQueryHelper::page_info();

        let query = json!({
            "query": format!(
                "query {{
                    productVariants({first_query}, query: \"product_id:'{product_id}'\") {{
                        edges {{
                            node {{
                                id
                                inventoryItem {{
                                    id
                                    inventoryLevel(locationId: {location_id}) {{
                                        id
                                        location {{
                                            id
                                        }}
                                        quantities(names: incoming,available,committed,reserved,damaged,safety_stock) {{
                                            name
                                            quantity
                                        }}
                                    }}
                                    requiresShipping
                                    tracked
                                    createdAt
                                    updatedAt
                                }}
                                product {{
                                    id
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
}
