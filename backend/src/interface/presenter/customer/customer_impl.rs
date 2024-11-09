use actix_web::web::{self, Json};
use async_trait::async_trait;

use crate::{
    domain::{customer::customer::Customer, error::error::DomainError},
    interface::presenter::customer_presenter_interface::CustomerPresenter,
};

use super::schema::{CustomerSchema, GetCustomersErrorResponse, GetCustomersResponse};

/// Generate a response schema for the customers.
pub struct CustomerPresenterImpl;
impl CustomerPresenterImpl {
    pub fn new() -> Self {
        CustomerPresenterImpl
    }
}

#[async_trait]
impl CustomerPresenter for CustomerPresenterImpl {
    type GetCustomersResponse = Json<GetCustomersResponse>;
    type GetCustomersErrorResponse = GetCustomersErrorResponse;
    async fn present_get_customers(
        &self,
        result: Result<Vec<Customer>, DomainError>,
    ) -> Result<Self::GetCustomersResponse, Self::GetCustomersErrorResponse> {
        let customers = result?;
        if customers.is_empty() {
            return Err(GetCustomersErrorResponse::NotFound {
                object_name: "Customer".to_string(),
            });
        }

        let response: Vec<CustomerSchema> = customers
            .into_iter()
            .map(|customer| customer.into())
            .collect();

        Ok(web::Json(GetCustomersResponse {
            customers: response,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::interface::mock::domain_mock::mock_customers;

    use super::*;

    #[actix_web::test]
    async fn test_present_get_customers_success() {
        let presenter = CustomerPresenterImpl::new();
        let customers = mock_customers(10);

        let result = presenter
            .present_get_customers(Ok(customers))
            .await
            .unwrap();

        assert_eq!(result.customers.len(), 10);

        assert_eq!(result.customers[0].id, "0");
        assert_eq!(result.customers[0].display_name, "Test Customer 0");
        assert_eq!(result.customers[0].email, Some("0@example.com".to_string()));

        assert_eq!(result.customers[9].id, "9");
        assert_eq!(result.customers[9].display_name, "Test Customer 9");
        assert_eq!(result.customers[9].email, Some("9@example.com".to_string()));
    }

    #[actix_web::test]
    async fn test_present_get_customers_not_found() {
        let presenter = CustomerPresenterImpl::new();

        let result = presenter.present_get_customers(Ok(vec![])).await;

        assert!(matches!(
            result,
            Err(GetCustomersErrorResponse::NotFound { .. })
        ));
    }

    #[actix_web::test]
    async fn test_present_get_customers_bad_request() {
        let presenter = CustomerPresenterImpl::new();

        let result = presenter
            .present_get_customers(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(result, Err(GetCustomersErrorResponse::BadRequest)));
    }

    #[actix_web::test]
    async fn test_present_get_customers_service_unavailable() {
        let presenter = CustomerPresenterImpl::new();

        let result = presenter
            .present_get_customers(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(GetCustomersErrorResponse::ServiceUnavailable)
        ));
    }
}
