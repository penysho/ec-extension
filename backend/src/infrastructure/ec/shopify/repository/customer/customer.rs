use async_trait::async_trait;

use crate::{
    domain::{customer::customer::Customer, email::email::Email, error::error::DomainError},
    infrastructure::ec::{
        ec_client_interface::ECClient,
        shopify::{
            query_helper::ShopifyGQLQueryHelper,
            repository::schema::{
                common::GraphQLResponse,
                customer::{CustomerNode, CustomersData},
            },
        },
    },
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
    /// Retrieve customer information by email.
    async fn get_customer_by_email(&self, email: &Email) -> Result<Customer, DomainError> {
        let first_query = ShopifyGQLQueryHelper::first_query();
        let page_info = ShopifyGQLQueryHelper::page_info();
        let address_fields = ShopifyGQLQueryHelper::address_fields();
        let email = email.value();

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
                        }}
                    }}
                    {page_info}
                }}
            }}"
        );

        let graphql_response: GraphQLResponse<CustomersData> = self.client.query(&query).await?;
        if let Some(errors) = graphql_response.errors {
            log::error!("Error returned in GraphQL response. Response: {:?}", errors);
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
            log::error!("No customer found for email: {}", email);
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
            shopify::repository::{
                customer::customer::CustomerRepositoryImpl,
                schema::{
                    address::AddressNode,
                    common::{Edges, GraphQLError, GraphQLResponse, Node, PageInfo},
                    customer::{CustomerNode, CustomersData},
                    media::ImageNode,
                },
            },
        },
        usecase::repository::customer_repository_interface::CustomerRepository,
    };

    fn mock_customer(id: u32) -> CustomerNode {
        CustomerNode {
            id: format!("gid://shopify/Customer/{id}"),
            addresses: vec![mock_address(id), mock_address(id + 1)],
            can_delete: true,
            default_address: Some(mock_address(id)),
            display_name: "Test Customer".to_string(),
            email: Some("test@example.com".to_string()),
            first_name: Some("Test".to_string()),
            last_name: Some("Customer".to_string()),
            image: Some(mock_image(id)),
            phone: Some("+1234567890".to_string()),
            note: Some("Test note".to_string()),
            status: "ENABLED".to_string(),
            verified_email: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
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

    fn mock_image(id: u32) -> ImageNode {
        ImageNode {
            id: format!("gid://shopify/MediaImage/{id}"),
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
    async fn get_customer_by_email_success() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<CustomersData>>()
            .times(1)
            .return_once(|_| Ok(mock_customers_response(1)));

        let repo = CustomerRepositoryImpl::new(client);

        let result = repo
            .get_customer_by_email(&Email::new("test@example.com".to_string()).unwrap())
            .await;

        dbg!(&result);

        assert!(result.is_ok());
        let customer = result.unwrap();
        assert_eq!(customer.id(), "0");
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
        assert_eq!(customer.default_address_id().clone().unwrap(), "0");
    }

    #[tokio::test]
    async fn get_customer_by_email_with_graphql_error() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<CustomersData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_error()));

        let repo = CustomerRepositoryImpl::new(client);

        let result = repo
            .get_customer_by_email(&Email::new("test@example.com".to_string()).unwrap())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }

    #[tokio::test]
    async fn get_customer_by_email_with_missing_data() {
        let mut client = MockECClient::new();

        client
            .expect_query::<GraphQLResponse<CustomersData>>()
            .times(1)
            .return_once(|_| Ok(mock_with_no_data()));

        let repo = CustomerRepositoryImpl::new(client);

        let result = repo
            .get_customer_by_email(&Email::new("test@example.com".to_string()).unwrap())
            .await;

        assert!(result.is_err());
        if let Err(DomainError::QueryError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::QueryError, but got something else");
        }
    }
}
