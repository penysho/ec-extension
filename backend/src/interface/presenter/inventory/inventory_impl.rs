use actix_web::web::{self, Json};
use async_trait::async_trait;

use crate::{
    domain::{error::error::DomainError, inventory::inventory::Inventory},
    interface::presenter::inventory_presenter_interface::InventoryPresenter,
};

use super::schema::{GetInventoriesResponse, GetInventoriesResponseError};

/// Generate a response schema for the inventory
pub struct InventoryPresenterImpl;
impl InventoryPresenterImpl {
    pub fn new() -> Self {
        InventoryPresenterImpl
    }
}

#[async_trait]
impl InventoryPresenter for InventoryPresenterImpl {
    type GetInventoriesResponse = Json<GetInventoriesResponse>;
    type GetInventoriesResponseError = GetInventoriesResponseError;
    /// Generate a list response of inventory information.
    async fn present_get_inventories(
        &self,
        result: Result<Vec<Inventory>, DomainError>,
    ) -> Result<Self::GetInventoriesResponse, Self::GetInventoriesResponseError> {
        let inventories = match result {
            Ok(inventories) => inventories,
            Err(DomainError::ValidationError) => {
                return Err(GetInventoriesResponseError::BadRequest)
            }
            Err(DomainError::InvalidRequest) => {
                return Err(GetInventoriesResponseError::BadRequest)
            }
            Err(_) => return Err(GetInventoriesResponseError::ServiceUnavailable),
        };
        if inventories.is_empty() {
            return Err(GetInventoriesResponseError::NotFound);
        }

        Ok(web::Json(GetInventoriesResponse {
            inventories: inventories.into_iter().map(|i| i.into()).collect(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::domain::inventory::inventory_level::{
        inventory_level::InventoryLevel,
        quantity::quantity::{InventoryType, Quantity},
    };

    use super::*;

    fn mock_inventories(count: usize) -> Vec<Inventory> {
        (0..count)
            .map(|i| {
                Inventory::new(
                    format!("{i}"),
                    format!("{i}"),
                    Some(
                        InventoryLevel::new(
                            format!("{i}"),
                            "location_id",
                            vec![
                                Quantity::new(10, InventoryType::Available).unwrap(),
                                Quantity::new(20, InventoryType::Committed).unwrap(),
                                Quantity::new(30, InventoryType::Incoming).unwrap(),
                                Quantity::new(40, InventoryType::Reserved).unwrap(),
                                Quantity::new(50, InventoryType::SafetyStock).unwrap(),
                                Quantity::new(60, InventoryType::Damaged).unwrap(),
                            ],
                        )
                        .unwrap(),
                    ),
                    true,
                    false,
                    Utc::now(),
                    Utc::now(),
                )
                .unwrap()
            })
            .collect()
    }

    #[actix_web::test]
    async fn test_present_get_inventories_success() {
        let presenter = InventoryPresenterImpl::new();
        let inventories = mock_inventories(10);

        let result = presenter
            .present_get_inventories(Ok(inventories))
            .await
            .unwrap();

        assert_eq!(result.inventories[0].id, "0");
        assert_eq!(result.inventories[0].variant_id, "0");
        assert_eq!(
            result.inventories[0].inventory_level.as_ref().unwrap().id,
            "0"
        );
        assert_eq!(
            result.inventories[0]
                .inventory_level
                .as_ref()
                .unwrap()
                .quantities
                .len(),
            6
        );

        assert_eq!(result.inventories[9].id, "9");
        assert_eq!(result.inventories[9].variant_id, "9");
        assert_eq!(
            result.inventories[9].inventory_level.as_ref().unwrap().id,
            "9"
        );
        assert_eq!(
            result.inventories[9]
                .inventory_level
                .as_ref()
                .unwrap()
                .quantities
                .len(),
            6
        );
    }

    #[actix_web::test]
    async fn test_present_get_inventories_not_found() {
        let presenter = InventoryPresenterImpl::new();

        let result = presenter.present_get_inventories(Ok(vec![])).await;

        assert!(matches!(result, Err(GetInventoriesResponseError::NotFound)));
    }

    #[actix_web::test]
    async fn test_present_get_inventories_bad_request() {
        let presenter = InventoryPresenterImpl::new();

        let result = presenter
            .present_get_inventories(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(
            result,
            Err(GetInventoriesResponseError::BadRequest)
        ));
    }

    #[actix_web::test]
    async fn test_present_get_inventories_service_unavailable() {
        let presenter = InventoryPresenterImpl::new();

        let result = presenter
            .present_get_inventories(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(GetInventoriesResponseError::ServiceUnavailable)
        ));
    }
}
