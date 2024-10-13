use async_trait::async_trait;

use crate::{
    domain::{
        customer::customer::Id as CustomerId, draft_order::draft_order::DraftOrder,
        error::error::DomainError,
    },
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            query_helper::ShopifyGQLQueryHelper,
            repository::schema::{
                common::GraphQLResponse,
                draft_order::{DraftOrderNode, DraftOrdersData},
            },
        },
    },
    usecase::repository::draft_order_repository_interface::DraftOrderRepository,
};

/// Repository for DraftOrders for Shopify.
pub struct DraftOrderRepositoryImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> DraftOrderRepositoryImpl<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> DraftOrderRepository for DraftOrderRepositoryImpl<C> {
    /// Retrieve draft order information by customer id.
    async fn get_draft_order_by_customer_id(
        &self,
        customer_id: &CustomerId,
    ) -> Result<DraftOrder, DomainError> {
        let first_query = ShopifyGQLQueryHelper::first_query();
        let page_info = ShopifyGQLQueryHelper::page_info();
        let address_fields = ShopifyGQLQueryHelper::address_fields();
        let money_bag_fields = ShopifyGQLQueryHelper::money_bag_fields();

        let query = format!(
            "query {{
                draftOrders({first_query}, query: \"customer_id:{customer_id}\") {{
                    edges {{
                        node {{
                            id
                            name
                            status
                            lineItems {{
                                id
                                isCustom
                                variant {{
                                    id
                                }}
                                quantity
                                discount {{
                                    title
                                    description
                                    value
                                    valueType
                                    amountSet {{
                                        {money_bag_fields}
                                    }}
                                }}
                                discountedTotalSet {{
                                    {money_bag_fields}
                                }}
                                originalTotalSet {{
                                    {money_bag_fields}
                                }}
                            }}
                            reserveInventoryUntil
                            subtotalPriceSet {{
                                {money_bag_fields}
                            }}
                            taxesIncluded
                            taxExempt
                            totalTaxSet {{
                                {money_bag_fields}
                            }}
                            totalDiscountsSet {{
                                {money_bag_fields}
                            }}
                            totalShippingPriceSet {{
                                {money_bag_fields}
                            }}
                            totalPriceSet {{
                                {money_bag_fields}
                            }}
                            customer {{
                                id
                            }}
                            billingAddress {{
                                {address_fields}
                            }}
                            shippingAddress {{
                                {address_fields}
                            }}
                            note
                            order {{
                                id
                            }}
                            completedAt
                            createdAt
                            updatedAt
                        }}
                    }}
                    {page_info}
                }}
            }}"
        );

        let graphql_response: GraphQLResponse<DraftOrdersData> = self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::QueryError);
        }

        let nodes: Vec<DraftOrderNode> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .draft_orders
            .edges
            .into_iter()
            .map(|node| node.node)
            .collect();

        let domains = DraftOrderNode::to_domains(nodes)?;

        if domains.is_empty() {
            log::error!("No draft order found for customer id: {}", customer_id);
            return Err(DomainError::NotFound);
        }

        Ok(domains.into_iter().next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::{
        domain::{draft_order::draft_order::DraftOrderStatus, error::error::DomainError},
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::repository::{
                draft_order::draft_order::DraftOrderRepositoryImpl,
                schema::{
                    address::AddressNode,
                    common::{Edges, GraphQLError, GraphQLResponse, Node, PageInfo},
                    draft_order::{CustomerIdNode, DraftOrderNode, DraftOrdersData, OrderIdNode},
                    line_item::{DiscountNode, LineItemNode, VariantIdNode},
                    money::{MoneyBagNode, MoneyNode},
                },
            },
        },
        usecase::repository::draft_order_repository_interface::DraftOrderRepository,
    };

    pub fn mock_draft_order(id: u32) -> DraftOrderNode {
        DraftOrderNode {
            id: format!("gid://shopify/DraftOrder/{id}"),
            name: "Test Order".to_string(),
            status: "OPEN".to_string(),
            line_items: vec![mock_line_item(id)],
            reserve_inventory_until: Some(Utc::now()),
            subtotal_price_set: mock_money_bag("100.00", "USD"),
            taxes_included: true,
            tax_exempt: false,
            total_tax_set: mock_money_bag("5.00", "USD"),
            total_discounts_set: mock_money_bag("10.00", "USD"),
            total_shipping_price_set: mock_money_bag("15.00", "USD"),
            total_price_set: mock_money_bag("110.00", "USD"),
            customer: Some(CustomerIdNode {
                id: format!("gid://shopify/Customer/{id}"),
            }),
            billing_address: mock_address(id),
            shipping_address: mock_address(id),
            note: Some("Test note".to_string()),
            order: Some(OrderIdNode {
                id: format!("gid://shopify/Order/{id}"),
            }),
            completed_at: None,
            created_at: Utc::now(),
            update_at: Utc::now(),
        }
    }

    fn mock_line_item(id: u32) -> LineItemNode {
        LineItemNode {
            id: format!("gid://shopify/LineItem/{id}"),
            is_custom: false,
            variant: Some(VariantIdNode {
                id: format!("gid://shopify/Variant/{id}"),
            }),
            quantity: 2,
            discount: Some(create_test_discount_node()),
            discounted_total_set: mock_money_bag("90.00", "USD"),
            original_total_set: mock_money_bag("100.00", "USD"),
        }
    }

    fn create_test_discount_node() -> DiscountNode {
        DiscountNode {
            title: Some("Test Discount".to_string()),
            description: "Test discount description".to_string(),
            value: "10.00".to_string(),
            value_type: "FIXED_AMOUNT".to_string(),
            amount_set: mock_money_bag("10.00", "USD"),
        }
    }

    fn mock_money_bag(amount: &str, currency: &str) -> MoneyBagNode {
        MoneyBagNode {
            shop_money: MoneyNode {
                amount: amount.to_string(),
                currency_code: currency.to_string(),
            },
        }
    }

    fn mock_address(id: u32) -> AddressNode {
        AddressNode {
            id: format!("gid://shopify/Address/{id}"),
            address1: Some("123 Test Street".to_string()),
            address2: Some("Apt 456".to_string()),
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

    fn mock_draft_orders_response(count: usize) -> GraphQLResponse<DraftOrdersData> {
        let nodes: Vec<Node<DraftOrderNode>> = (0..count)
            .map(|i| Node {
                node: mock_draft_order(i as u32),
            })
            .collect();

        GraphQLResponse {
            data: Some(DraftOrdersData {
                draft_orders: Edges {
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
    async fn test_get_draft_order_by_customer_id_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<DraftOrdersData>>()
            .times(1)
            .return_once(|_| Ok(mock_draft_orders_response(1)));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.get_draft_order_by_customer_id(&"1".to_string()).await;

        assert!(result.is_ok());
        let draft_order = result.unwrap();
        assert_eq!(draft_order.id(), "0");
        assert_eq!(draft_order.name(), "Test Order");
        assert_eq!(draft_order.status(), &DraftOrderStatus::Open);
        assert_eq!(draft_order.line_items().len(), 1);
        assert_eq!(draft_order.subtotal_price_set().amount().value(), &100.0);
        assert_eq!(draft_order.taxes_included(), &true);
        assert_eq!(draft_order.tax_exempt(), &false);
        assert_eq!(draft_order.total_tax_set().amount().value(), &5.0);
        assert_eq!(draft_order.total_discounts_set().amount().value(), &10.0);
        assert_eq!(
            draft_order.total_shipping_price_set().amount().value(),
            &15.0
        );
        assert_eq!(draft_order.total_price_set().amount().value(), &110.0);
        assert_eq!(draft_order.customer_id(), &Some("0".to_string()));
        assert_eq!(draft_order.billing_address().id(), "0");
        assert_eq!(draft_order.shipping_address().id(), "0");
        assert_eq!(draft_order.note(), &Some("Test note".to_string()));
        assert_eq!(draft_order.order_id(), &Some("0".to_string()));
        assert_eq!(draft_order.completed_at(), &None);
    }

    #[tokio::test]
    async fn test_get_draft_order_by_customer_id_with_invalid_status() {
        let mut client = MockECClient::new();

        let mut invalid_response = mock_draft_orders_response(1);
        invalid_response.data.as_mut().unwrap().draft_orders.edges[0]
            .node
            .status = "INVALID_STATUS".to_string();

        client
            .expect_query::<GraphQLResponse<DraftOrdersData>>()
            .times(1)
            .return_once(|_| Ok(invalid_response));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.get_draft_order_by_customer_id(&"1".to_string()).await;

        assert!(result.is_err());
        if let Err(DomainError::ConversionError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ConversionError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_get_draft_order_by_customer_id_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<DraftOrdersData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.get_draft_order_by_customer_id(&"1".to_string()).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_get_draft_order_by_customer_id_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<DraftOrdersData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.get_draft_order_by_customer_id(&"1".to_string()).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
