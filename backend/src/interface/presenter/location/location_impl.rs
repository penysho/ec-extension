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
    use crate::interface::mock::domain_mock::mock_locations;

    use super::*;

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
