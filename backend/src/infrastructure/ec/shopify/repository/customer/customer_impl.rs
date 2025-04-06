use async_trait::async_trait;

use crate::{
    domain::{customer::customer::Customer, email::email::Email, error::error::DomainError},
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            gql_helper::ShopifyGQLHelper,
            repository::schema::customer::{CustomerNode, CustomersData},
            schema::GraphQLResponse,
        },
    },
    log_error,
    usecase::repository::customer_repository_interface::CustomerRepository,
};

/// Repository for Customers for Shopify.
pub struct CustomerRepositoryImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> CustomerRepositoryImpl<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> CustomerRepository for CustomerRepositoryImpl<C> {
    async fn find_customer_by_email(&self, email: &Email) -> Result<Customer, DomainError> {
        let first_query = ShopifyGQLHelper::first_query();
        let page_info = ShopifyGQLHelper::page_info();
        let address_fields = ShopifyGQLHelper::address_fields();
        let email = email.value();
        let user_id_query = ShopifyGQLHelper::metafield_query("user_id", "custom");

        let query = format!(
            "query {{
                customers({first_query}, query: \"email:{email}\") {{
                    edges {{
                        node {{
                            canDelete
                            createdAt
                            displayName
                            email
                            firstName
                            id
                            lastName
                            note
                            phone
                            state
                            updatedAt
                            validEmailAddress
                            verifiedEmail
                            addresses({first_query}) {{
                                {address_fields}
                            }}
                            image {{
                                altText
                                height
                                id
                                url
                                width
                            }}
                            defaultAddress {{
                                {address_fields}
                            }}
                            {user_id_query}
                        }}
                    }}
                    {page_info}
                }}
            }}"
        );

        let graphql_response: GraphQLResponse<CustomersData> = self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log_error!("Error returned in GraphQL response. Response."; "errors" => ?errors);
            return Err(DomainError::QueryError);
        }

        let nodes: Vec<CustomerNode> = graphql_response
            .data
            .ok_or(DomainError::QueryError)?
            .customers
            .edges
            .into_iter()
            .map(|node| node.node)
            .collect();

        let domains = CustomerNode::to_domains(nodes)?;

        if domains.is_empty() {
            log_error!("No customer found for email."; "email" => email.clone());
            return Err(DomainError::NotFound);
        }

        Ok(domains.into_iter().next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::{
        domain::{
            customer::customer::CustomerStatus, email::email::Email, error::error::DomainError,
        },
        infrastructure::ec::{
            ec_client_interface::MockECClient,
            shopify::{
                repository::{
                    customer::customer_impl::CustomerRepositoryImpl,
                    schema::{
                        address::AddressNode,
                        customer::{CustomerNode, CustomersData},
                        media::ImageNode,
                    },
                },
                schema::{Edges, GraphQLError, GraphQLResponse, Metafield, Node, PageInfo},
            },
        },
        usecase::repository::customer_repository_interface::CustomerRepository,
    };

    fn mock_customer(id: u32) -> CustomerNode {
        CustomerNode {
            id: format!("gid://shopify/Customer/{id}"),
            user_id: Metafield::<String> {
                value: format!("user_{id}"),
            },
            addresses: vec![mock_address(Some("123")), mock_address(Some("456"))],
            default_address: Some(mock_address(Some("123"))),
            display_name: "Test Customer".to_string(),
            email: Some("test@example.com".to_string()),
            first_name: Some("Test".to_string()),
            last_name: Some("Customer".to_string()),
            image: mock_image(id),
            phone: Some("+1234567890".to_string()),
            note: Some("Test note".to_string()),
            state: "ENABLED".to_string(),
            verified_email: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn mock_address(address1: Option<impl Into<String>>) -> AddressNode {
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

    fn mock_image(id: u32) -> ImageNode {
        ImageNode {
            id: Some(format!("gid://shopify/MediaImage/{id}")),
            alt_text: Some(format!("Alt text for image {id}")),
            url: format!("https://example.com/MediaImage/{id}.jpg"),
            height: Some(600),
            width: Some(500),
        }
    }

    fn mock_customers_response(count: usize) -> GraphQLResponse<CustomersData> {
        let nodes: Vec<Node<CustomerNode>> = (0..count)
            .map(|i| Node {
                node: mock_customer(i as u32),
            })
            .collect();

        GraphQLResponse {
            data: Some(CustomersData {
                customers: Edges {
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
    async fn test_find_customer_by_email_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<CustomersData>>()
            .times(1)
            .return_once(|_| Ok(mock_customers_response(1)));

        let repo = CustomerRepositoryImpl::new(client);

        let result = repo
            .find_customer_by_email(&Email::new("test@example.com".to_string()).unwrap())
            .await;

        assert!(result.is_ok());
        let customer = result.unwrap();
        assert_eq!(customer.id(), "0");
        assert_eq!(customer.user_id(), "user_0");
        assert_eq!(
            customer.email().as_ref().unwrap().value(),
            "test@example.com"
        );
        assert_eq!(customer.first_name().clone().unwrap(), "Test");
        assert_eq!(customer.last_name().clone().unwrap(), "Customer");
        assert_eq!(customer.phone().clone().unwrap().value(), "+1234567890");
        assert_eq!(customer.note().clone().unwrap(), "Test note");
        assert_eq!(*customer.status(), CustomerStatus::Active);
        assert!(customer.verified_email());
        assert_eq!(customer.addresses().len(), 2);
    }

    #[tokio::test]
    async fn test_find_customer_by_email_with_invalid_state() {
        let mut client = MockECClient::new();

        let mut invalid_response = mock_customers_response(1);
        invalid_response.data.as_mut().unwrap().customers.edges[0]
            .node
            .state = "INVALID_STATE".to_string();

        client
            .expect_query::<GraphQLResponse<CustomersData>>()
            .times(1)
            .return_once(|_| Ok(invalid_response));

        let repo = CustomerRepositoryImpl::new(client);

        let result = repo
            .find_customer_by_email(&Email::new("test@example.com".to_string()).unwrap())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::ConversionError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::ConversionError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_customer_by_email_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<CustomersData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = CustomerRepositoryImpl::new(client);

        let result = repo
            .find_customer_by_email(&Email::new("test@example.com".to_string()).unwrap())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_find_customer_by_email_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<CustomersData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = CustomerRepositoryImpl::new(client);

        let result = repo
            .find_customer_by_email(&Email::new("test@example.com".to_string()).unwrap())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
