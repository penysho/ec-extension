use actix_web::web::{self, Json};
use async_trait::async_trait;

use crate::{
    domain::{error::error::DomainError, location::location::Location},
    interface::presenter::location_presenter_interface::LocationPresenter,
};

use super::schema::{GetLocationsErrorResponse, GetLocationsResponse};

/// Generate a response schema for the location.
pub struct LocationPresenterImpl;
impl LocationPresenterImpl {
    pub fn new() -> Self {
        LocationPresenterImpl
    }
}

#[async_trait]
impl LocationPresenter for LocationPresenterImpl {
    type GetLocationsResponse = Json<GetLocationsResponse>;
    type GetLocationsErrorResponse = GetLocationsErrorResponse;
    /// Generate a list response of location information.
    async fn present_get_locations(
        &self,
        result: Result<Vec<Location>, DomainError>,
    ) -> Result<Self::GetLocationsResponse, Self::GetLocationsErrorResponse> {
        Ok(web::Json(GetLocationsResponse {
            locations: result?.into_iter().map(|l| l.into()).collect(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::address::address::Address;

    use super::*;

    fn mock_address() -> Address {
        Address::new(
            Some("123 Main St"),
            None::<String>,
            Some("City"),
            true,
            Some("Country"),
            Some("John"),
            Some("Doe"),
            Some("Province"),
            Some("12345"),
            Some("+1234567890"),
        )
        .expect("Failed to create mock address")
    }

    fn mock_locations(count: usize) -> Vec<Location> {
        (0..count)
            .map(|i| {
                Location::new(
                    format!("{i}"),
                    format!("{i}"),
                    true,
                    false,
                    mock_address(),
                    vec![mock_address()],
                )
                .unwrap()
            })
            .collect()
    }

    #[actix_web::test]
    async fn test_present_get_inventories_success() {
        let presenter = LocationPresenterImpl::new();

        let result = presenter
            .present_get_locations(Ok(mock_locations(10)))
            .await
            .unwrap();

        assert_eq!(result.locations.len(), 10);
        assert_eq!(result.locations[0].id, "0");
        assert_eq!(result.locations[0].name, "0");
        assert_eq!(result.locations[0].is_active, true);
        assert_eq!(
            result.locations[0].address.address1,
            Some("123 Main St".to_string())
        );

        assert_eq!(result.locations[9].id, "9");
        assert_eq!(result.locations[9].name, "9");
        assert_eq!(result.locations[9].is_active, true);
        assert_eq!(
            result.locations[9].address.address1,
            Some("123 Main St".to_string())
        );
    }

    #[actix_web::test]
    async fn test_present_get_locations_bad_request() {
        let presenter = LocationPresenterImpl::new();

        let result = presenter
            .present_get_locations(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(result, Err(GetLocationsErrorResponse::BadRequest)));
    }

    #[actix_web::test]
    async fn test_present_get_locations_service_unavailable() {
        let presenter = LocationPresenterImpl::new();

        let result = presenter
            .present_get_locations(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(GetLocationsErrorResponse::ServiceUnavailable)
        ));
    }
}
