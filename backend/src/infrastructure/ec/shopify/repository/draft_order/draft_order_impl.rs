use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::{
    domain::{
        customer::customer::Id as CustomerId,
        draft_order::draft_order::{DraftOrder, Id as DraftOrderId},
        error::error::DomainError,
    },
    infrastructure::{
        ec::{
            ec_client_interface::ECClient,
            shopify::{
                gql_helper::ShopifyGQLHelper,
                repository::schema::{
                    draft_order::{DraftOrderData, DraftOrderNode, DraftOrdersData},
                    draft_order_input::{
                        DraftOrderCompleteData, DraftOrderCreateData, DraftOrderDeleteData,
                        DraftOrderDeleteInput, DraftOrderInput, DraftOrderUpdateData,
                    },
                },
                schema::GraphQLResponse,
            },
        },
        error::{InfrastructureError, InfrastructureErrorMapper},
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

    fn draft_order_fields() -> String {
        let first_query = ShopifyGQLHelper::first_query();
        let page_info = ShopifyGQLHelper::page_info();
        let address_fields = ShopifyGQLHelper::address_fields();
        let money_bag_fields = ShopifyGQLHelper::money_bag_fields();
        let owner_user_id_query = ShopifyGQLHelper::metafield_query("owner_user_id", "custom");

        format!(
            "id
            name
            status
            customer {{
                id
            }}
            billingAddress {{
                {address_fields}
            }}
            shippingAddress {{
                {address_fields}
            }}
            note2
            lineItems({first_query}) {{
                edges {{
                    node {{
                        id
                        custom
                        variant {{
                            id
                        }}
                        quantity
                        appliedDiscount {{
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
                }}
                {page_info}
            }}
            reserveInventoryUntil
            appliedDiscount {{
                title
                description
                value
                valueType
                amountSet {{
                    {money_bag_fields}
                }}
            }}
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
            presentmentCurrencyCode
            order {{
                id
            }}
            {owner_user_id_query}
            completedAt
            createdAt
            updatedAt"
        )
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> DraftOrderRepository for DraftOrderRepositoryImpl<C> {
    async fn find_draft_order_by_id(&self, id: &DraftOrderId) -> Result<DraftOrder, DomainError> {
        let id = ShopifyGQLHelper::add_draft_order_gid_prefix(id);
        let draft_order_fields = Self::draft_order_fields();

        let query = format!(
            "query {{
                draftOrder(id: \"{id}\") {{
                    {draft_order_fields}
                }}
            }}"
        );

        let graphql_response: GraphQLResponse<DraftOrderData> = self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::QueryError);
        }

        DraftOrderNode::to_domain(
            graphql_response
                .data
                .ok_or(DomainError::NotFound)?
                .draft_order,
        )
    }

    async fn find_draft_orders_by_customer_id(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<DraftOrder>, DomainError> {
        let first_query = ShopifyGQLHelper::first_query();
        let page_info = ShopifyGQLHelper::page_info();
        let draft_order_fields = Self::draft_order_fields();

        // TODO: Handling draft orders exceeding 250 for a customer.
        // The lineItem in the draft order shall not exceed 250.
        let query = format!(
            "query {{
                draftOrders({first_query}, query: \"customer_id:{customer_id}\") {{
                    edges {{
                        node {{
                            {draft_order_fields}
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

        DraftOrderNode::to_domains(nodes)
    }

    async fn create(&self, draft_order: DraftOrder) -> Result<DraftOrder, DomainError> {
        let schema = DraftOrderInput::from(draft_order);
        let input = serde_json::to_value(schema).map_err(|e| {
            log::error!("Failed to parse the request structure. Error: {:?}", e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::ParseError(e))
        })?;

        let draft_order_fields = Self::draft_order_fields();
        let user_errors = ShopifyGQLHelper::user_errors();

        let query = format!(
            "mutation draftOrderCreate($input: DraftOrderInput!) {{
                draftOrderCreate(input: $input) {{
                    draftOrder {{
                        {draft_order_fields}
                    }}
                    {user_errors}
                }}
            }}",
        );

        let graphql_response: GraphQLResponse<DraftOrderCreateData> =
            self.client.mutation(&query, &input).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::SaveError);
        }

        let data = graphql_response
            .data
            .ok_or(DomainError::SaveError)?
            .draft_order_create;

        if !data.user_errors.is_empty() {
            log::error!("UserErrors returned. userErrors: {:?}", user_errors);
            return Err(DomainError::SaveError);
        }

        match data.draft_order {
            Some(draft_order) => draft_order.to_domain(),
            None => {
                log::error!("No draft order returned.");
                Err(DomainError::SaveError)
            }
        }
    }

    async fn update(&self, draft_order: DraftOrder) -> Result<DraftOrder, DomainError> {
        let completed_at = draft_order.completed_at().clone();

        match completed_at {
            Some(completed_at) if completed_at == DateTime::<Utc>::default() => {
                let id = ShopifyGQLHelper::add_draft_order_gid_prefix(draft_order.id());

                let draft_order_fields = Self::draft_order_fields();
                let user_errors = ShopifyGQLHelper::user_errors();

                let query = format!(
                    "mutation draftOrderComplete {{
                        draftOrderComplete(id: \"{id}\") {{
                            draftOrder {{
                                {draft_order_fields}
                            }}
                            {user_errors}
                        }}
                    }}",
                );

                let graphql_response: GraphQLResponse<DraftOrderCompleteData> = self
                    .client
                    .mutation(&query, &serde_json::to_value("").unwrap())
                    .await?;
                if let Some(errors) = graphql_response.errors {
                    log::error!("Error returned in GraphQL response. Response: {:?}", errors);
                    return Err(DomainError::SaveError);
                }

                let data = graphql_response
                    .data
                    .ok_or(DomainError::SaveError)?
                    .draft_order_complete;

                if !data.user_errors.is_empty() {
                    log::error!("UserErrors returned. userErrors: {:?}", user_errors);
                    return Err(DomainError::SaveError);
                }

                match data.draft_order {
                    Some(draft_order) => draft_order.to_domain(),
                    None => {
                        log::error!("No draft order returned.");
                        Err(DomainError::SaveError)
                    }
                }
            }
            _ => {
                let input =
                    serde_json::to_value(DraftOrderInput::from(draft_order)).map_err(|e| {
                        log::error!("Failed to parse the request structure. Error: {:?}", e);
                        InfrastructureErrorMapper::to_domain(InfrastructureError::ParseError(e))
                    })?;

                let draft_order_fields = Self::draft_order_fields();
                let user_errors = ShopifyGQLHelper::user_errors();

                let query = format!(
                    "mutation draftOrderUpdate($input: DraftOrderInput!) {{
                        draftOrderUpdate(input: $input) {{
                            draftOrder {{
                                {draft_order_fields}
                            }}
                            {user_errors}
                        }}
                    }}",
                );

                let graphql_response: GraphQLResponse<DraftOrderUpdateData> =
                    self.client.mutation(&query, &input).await?;
                if let Some(errors) = graphql_response.errors {
                    log::error!("Error returned in GraphQL response. Response: {:?}", errors);
                    return Err(DomainError::SaveError);
                }

                let data = graphql_response
                    .data
                    .ok_or(DomainError::SaveError)?
                    .draft_order_update;

                if !data.user_errors.is_empty() {
                    log::error!("UserErrors returned. userErrors: {:?}", user_errors);
                    return Err(DomainError::SaveError);
                }

                match data.draft_order {
                    Some(draft_order) => draft_order.to_domain(),
                    None => {
                        log::error!("No draft order returned.");
                        Err(DomainError::SaveError)
                    }
                }
            }
        }
    }

    async fn delete(&self, draft_order: DraftOrder) -> Result<DraftOrderId, DomainError> {
        let input =
            serde_json::to_value(DraftOrderDeleteInput::from(draft_order)).map_err(|e| {
                log::error!("Failed to parse the request structure. Error: {:?}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::ParseError(e))
            })?;

        let user_errors = ShopifyGQLHelper::user_errors();

        let query = format!(
            "mutation draftOrderDelete($input: DraftOrderDeleteInput!) {{
                draftOrderDelete(input: $input) {{
                    deletedId
                    {user_errors}
                }}
            }}",
        );

        let graphql_response: GraphQLResponse<DraftOrderDeleteData> =
            self.client.mutation(&query, &input).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
            return Err(DomainError::DeleteError);
        }

        let data = graphql_response
            .data
            .ok_or(DomainError::DeleteError)?
            .draft_order_delete;

        if !data.user_errors.is_empty() {
            log::error!("UserErrors returned. userErrors: {:?}", user_errors);
            return Err(DomainError::DeleteError);
        }

        match data.deleted_id {
            Some(deleted_id) => Ok(ShopifyGQLHelper::remove_gid_prefix(&deleted_id)),
            None => {
                log::error!("No draft order returned.");
                Err(DomainError::DeleteError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use serde_json::Value;

    use crate::{
        domain::{
            draft_order::draft_order::{DraftOrder, DraftOrderStatus},
            error::error::DomainError,
            money::{
                amount::amount::Amount,
                money::{CurrencyCode, Money},
            },
        },
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::{
                repository::{
                    draft_order::draft_order_impl::DraftOrderRepositoryImpl,
                    schema::{
                        address::AddressNode,
                        draft_order::{
                            CustomerIdNode, DraftOrderData, DraftOrderNode, DraftOrdersData,
                            OrderIdNode,
                        },
                        draft_order_input::{
                            DraftOrderComplete, DraftOrderCompleteData, DraftOrderCreate,
                            DraftOrderCreateData, DraftOrderDelete, DraftOrderDeleteData,
                            DraftOrderUpdate, DraftOrderUpdateData,
                        },
                        line_item::{DiscountNode, LineItemNode, VariantIdNode},
                        money::{CurrencyCodeNode, MoneyBagNode, MoneyNode},
                    },
                },
                schema::{Edges, GraphQLError, GraphQLResponse, Node, PageInfo, UserError},
            },
        },
        usecase::repository::draft_order_repository_interface::DraftOrderRepository,
    };

    pub fn mock_draft_order_node(id: u32) -> DraftOrderNode {
        DraftOrderNode {
            id: format!("gid://shopify/DraftOrder/{id}"),
            name: "Test Order".to_string(),
            status: "OPEN".to_string(),
            customer: Some(CustomerIdNode {
                id: format!("gid://shopify/Customer/{id}"),
            }),
            billing_address: mock_address_node(Some("123")),
            shipping_address: mock_address_node(Some("123")),
            note2: Some("Test note".to_string()),
            line_items: Edges {
                edges: vec![Node {
                    node: mock_line_item_node(id),
                }],
                page_info: PageInfo {
                    has_previous_page: false,
                    has_next_page: false,
                    start_cursor: None,
                    end_cursor: None,
                },
            },
            applied_discount: Some(mock_discount_node()),
            reserve_inventory_until: Some(Utc::now()),
            subtotal_price_set: mock_money_node("100.00", "USD"),
            taxes_included: true,
            tax_exempt: false,
            total_tax_set: mock_money_node("5.00", "USD"),
            total_discounts_set: mock_money_node("10.00", "USD"),
            total_shipping_price_set: mock_money_node("15.00", "USD"),
            total_price_set: mock_money_node("110.00", "USD"),
            presentment_currency_code: CurrencyCodeNode("USD".to_string()),
            order: Some(OrderIdNode {
                id: format!("gid://shopify/Order/{id}"),
            }),
            owner_user_id: "Owner".to_string(),
            completed_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn mock_line_item_node(id: u32) -> LineItemNode {
        LineItemNode {
            id: format!("gid://shopify/LineItem/{id}"),
            custom: false,
            variant: Some(VariantIdNode {
                id: format!("gid://shopify/Variant/{id}"),
            }),
            quantity: 2,
            applied_discount: Some(mock_discount_node()),
            discounted_total_set: mock_money_node("90.00", "USD"),
            original_total_set: mock_money_node("100.00", "USD"),
        }
    }

    fn mock_discount_node() -> DiscountNode {
        DiscountNode {
            title: Some("Test Discount".to_string()),
            description: "Test discount description".to_string(),
            value: 10.00,
            value_type: "FIXED_AMOUNT".to_string(),
            amount_set: mock_money_node("10.00", "USD"),
        }
    }

    fn mock_money_node(amount: &str, currency: &str) -> MoneyBagNode {
        MoneyBagNode {
            shop_money: MoneyNode {
                amount: amount.to_string(),
                currency_code: CurrencyCodeNode(currency.to_string()),
            },
        }
    }

    fn mock_address_node(address1: Option<impl Into<String>>) -> Option<AddressNode> {
        let address1 = address1.map(|a| a.into());
        Some(AddressNode {
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
        })
    }

    fn mock_money_domain() -> Money {
        let amount = Amount::new(100.0).unwrap();
        Money::new(CurrencyCode::USD, amount).expect("Failed to create mock money")
    }

    fn mock_draft_order_domain(completed: bool) -> DraftOrder {
        let mut completed_at = None;
        if completed {
            completed_at = Some(DateTime::<Utc>::default());
        }

        DraftOrder::new(
            "0",
            "Test Order",
            DraftOrderStatus::Open,
            None,
            None,
            None,
            None,
            vec![],
            None,
            None,
            mock_money_domain(),
            true,
            false,
            mock_money_domain(),
            mock_money_domain(),
            mock_money_domain(),
            mock_money_domain(),
            CurrencyCode::JPY,
            None,
            "Owner".to_string(),
            completed_at,
            Utc::now(),
            Utc::now(),
        )
        .expect("Failed to create mock draft order domain")
    }

    fn mock_draft_order_response() -> GraphQLResponse<DraftOrderData> {
        GraphQLResponse {
            data: Some(DraftOrderData {
                draft_order: mock_draft_order_node(0),
            }),
            errors: None,
        }
    }

    fn mock_draft_orders_response(count: usize) -> GraphQLResponse<DraftOrdersData> {
        let nodes: Vec<Node<DraftOrderNode>> = (0..count)
            .map(|i| Node {
                node: mock_draft_order_node(i as u32),
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

    fn mock_draft_order_create_response() -> GraphQLResponse<DraftOrderCreateData> {
        GraphQLResponse {
            data: Some(DraftOrderCreateData {
                draft_order_create: DraftOrderCreate {
                    draft_order: Some(mock_draft_order_node(0)),
                    user_errors: vec![],
                },
            }),
            errors: None,
        }
    }

    fn mock_draft_order_update_response() -> GraphQLResponse<DraftOrderUpdateData> {
        GraphQLResponse {
            data: Some(DraftOrderUpdateData {
                draft_order_update: DraftOrderUpdate {
                    draft_order: Some(mock_draft_order_node(0)),
                    user_errors: vec![],
                },
            }),
            errors: None,
        }
    }

    fn mock_draft_order_complete_response() -> GraphQLResponse<DraftOrderCompleteData> {
        GraphQLResponse {
            data: Some(DraftOrderCompleteData {
                draft_order_complete: DraftOrderComplete {
                    draft_order: Some(mock_draft_order_node(0)),
                    user_errors: vec![],
                },
            }),
            errors: None,
        }
    }

    fn mock_draft_order_delete_response() -> GraphQLResponse<DraftOrderDeleteData> {
        GraphQLResponse {
            data: Some(DraftOrderDeleteData {
                draft_order_delete: DraftOrderDelete {
                    deleted_id: Some("gid://shopify/DraftOrder/0".to_string()),
                    user_errors: vec![],
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
    async fn test_find_draft_order_by_id_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<DraftOrderData>>()
            .times(1)
            .return_once(|_| Ok(mock_draft_order_response()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.find_draft_order_by_id(&"1".to_string()).await;

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
        assert_eq!(draft_order.note(), &Some("Test note".to_string()));
        assert_eq!(draft_order.order_id(), &Some("0".to_string()));
        assert_eq!(draft_order.completed_at(), &None);
    }

    #[tokio::test]
    async fn test_find_draft_order_by_id_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<DraftOrderData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.find_draft_order_by_id(&"1".to_string()).await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_draft_order_by_id_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<DraftOrderData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.find_draft_order_by_id(&"1".to_string()).await;

        assert!(result.is_err());
        if let Err(DomainError::NotFound) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::NotFound, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_draft_orders_by_customer_id_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<DraftOrdersData>>()
            .times(1)
            .return_once(|_| Ok(mock_draft_orders_response(1)));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo
            .find_draft_orders_by_customer_id(&"1".to_string())
            .await;

        assert!(result.is_ok());
        let draft_order = result.unwrap().into_iter().next().unwrap();
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
        assert_eq!(draft_order.note(), &Some("Test note".to_string()));
        assert_eq!(draft_order.order_id(), &Some("0".to_string()));
        assert_eq!(draft_order.completed_at(), &None);
    }

    #[tokio::test]
    async fn test_find_draft_orders_by_customer_id_with_invalid_status() {
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

        let result = repo
            .find_draft_orders_by_customer_id(&"1".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::ConversionError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ConversionError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_draft_orders_by_customer_id_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<DraftOrdersData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo
            .find_draft_orders_by_customer_id(&"1".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_draft_orders_by_customer_id_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<DraftOrdersData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo
            .find_draft_orders_by_customer_id(&"1".to_string())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_create_success() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderCreateData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_draft_order_create_response()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.create(mock_draft_order_domain(false)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_with_user_errors() {
        let mut client = MockECClient::new();

        let mut response = mock_draft_order_create_response();
        response
            .data
            .as_mut()
            .unwrap()
            .draft_order_create
            .user_errors = vec![UserError {
            field: vec!["quantity".to_string()],
            message: "Quantity must be positive".to_string(),
        }];

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderCreateData>>()
            .times(1)
            .return_once(|_, _| Ok(response));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.create(mock_draft_order_domain(false)).await;

        assert!(result.is_err());
        if let Err(DomainError::SaveError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::SaveError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_create_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderCreateData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_with_error()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.create(mock_draft_order_domain(false)).await;

        assert!(result.is_err());
        if let Err(DomainError::SaveError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::SaveError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_create_with_no_data() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderCreateData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_with_no_data()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.create(mock_draft_order_domain(false)).await;
        assert!(result.is_err());
        if let Err(DomainError::SaveError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::SaveError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_update_for_update_success() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderUpdateData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_draft_order_update_response()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.update(mock_draft_order_domain(false)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_for_complete_success() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderCompleteData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_draft_order_complete_response()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.update(mock_draft_order_domain(true)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_with_user_errors() {
        let mut client = MockECClient::new();

        let mut response = mock_draft_order_update_response();
        response
            .data
            .as_mut()
            .unwrap()
            .draft_order_update
            .user_errors = vec![UserError {
            field: vec!["quantity".to_string()],
            message: "Quantity must be positive".to_string(),
        }];

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderUpdateData>>()
            .times(1)
            .return_once(|_, _| Ok(response));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.update(mock_draft_order_domain(false)).await;

        assert!(result.is_err());
        if let Err(DomainError::SaveError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::SaveError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_update_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderUpdateData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_with_error()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.update(mock_draft_order_domain(false)).await;

        assert!(result.is_err());
        if let Err(DomainError::SaveError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::SaveError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_update_with_no_data() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderUpdateData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_with_no_data()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.update(mock_draft_order_domain(false)).await;
        assert!(result.is_err());
        if let Err(DomainError::SaveError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::SaveError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_delete_success() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderDeleteData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_draft_order_delete_response()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.delete(mock_draft_order_domain(false)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "0".to_string());
    }

    #[tokio::test]
    async fn test_delete_with_user_errors() {
        let mut client = MockECClient::new();

        let mut response = mock_draft_order_delete_response();
        response
            .data
            .as_mut()
            .unwrap()
            .draft_order_delete
            .user_errors = vec![UserError {
            field: vec!["quantity".to_string()],
            message: "Quantity must be positive".to_string(),
        }];

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderDeleteData>>()
            .times(1)
            .return_once(|_, _| Ok(response));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.delete(mock_draft_order_domain(false)).await;

        assert!(result.is_err());
        if let Err(DomainError::DeleteError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::DeleteError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_delete_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderDeleteData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_with_error()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.delete(mock_draft_order_domain(false)).await;

        assert!(result.is_err());
        if let Err(DomainError::DeleteError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::DeleteError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_delete_with_no_data() {
        let mut client = MockECClient::new();

        client
            .expect_mutation::<Value, GraphQLResponse<DraftOrderDeleteData>>()
            .times(1)
            .return_once(|_, _| Ok(mock_with_no_data()));

        let repo = DraftOrderRepositoryImpl::new(client);

        let result = repo.delete(mock_draft_order_domain(false)).await;
        assert!(result.is_err());
        if let Err(DomainError::DeleteError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::DeleteError, but got something else");
        }
    }
}
