use std::collections::HashMap;

use actix_web::web::{self, Json};
use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        inventory_item::inventory_item::{Id as InventoryItemId, InventoryItem},
        inventory_level::inventory_level::InventoryLevel,
    },
    interface::presenter::inventory_presenter_interface::InventoryPresenter,
};

use super::schema::{
    GetInventoriesErrorResponse, GetInventoriesResponse, InventorySchema,
    PutInventoryErrorResponse, PutInventoryResponse,
};

/// Generate a response schema for the inventory.
pub struct InventoryPresenterImpl;
impl InventoryPresenterImpl {
    pub fn new() -> Self {
        InventoryPresenterImpl
    }
}

#[async_trait]
impl InventoryPresenter for InventoryPresenterImpl {
    type GetInventoriesResponse = Json<GetInventoriesResponse>;
    type GetInventoriesErrorResponse = GetInventoriesErrorResponse;
    async fn present_get_inventories(
        &self,
        result: Result<
            (
                Vec<InventoryItem>,
                HashMap<InventoryItemId, Vec<InventoryLevel>>,
            ),
            DomainError,
        >,
    ) -> Result<Self::GetInventoriesResponse, Self::GetInventoriesErrorResponse> {
        let (inventory_items, mut inventory_levels) = result?;
        if inventory_items.is_empty() {
            return Err(GetInventoriesErrorResponse::NotFound {
                object_name: "Inventories".to_string(),
            });
        }

        let response: Vec<InventorySchema> = inventory_items
            .into_iter()
            .map(|item| {
                let level = inventory_levels.remove(item.id()).unwrap_or(vec![]);
                InventorySchema::to_schema(item, level)
            })
            .collect();

        Ok(web::Json(GetInventoriesResponse {
            inventories: response,
        }))
    }

    type PutInventoryResponse = Json<PutInventoryResponse>;
    type PutInventoryErrorResponse = PutInventoryErrorResponse;
    async fn present_put_inventory(
        &self,
        result: Result<InventoryLevel, DomainError>,
    ) -> Result<Self::PutInventoryResponse, Self::PutInventoryErrorResponse> {
        Ok(web::Json(PutInventoryResponse {
            inventory_level: result?.into(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::inventory_level::quantity::quantity::{InventoryType, Quantity},
        interface::mock::domain_mock::mock_inventory_items,
    };

    use super::*;

    fn mock_inventory_level_map(
        count: usize,
        inventory_item_id: &InventoryItemId,
    ) -> HashMap<InventoryItemId, Vec<InventoryLevel>> {
        let mut map: HashMap<InventoryItemId, Vec<InventoryLevel>> = HashMap::new();

        let levels = (0..count)
            .map(|i| {
                InventoryLevel::new(
                    format!("{i}"),
                    inventory_item_id.clone(),
                    format!("{i}"),
                    vec![
                        Quantity::new(10, InventoryType::Available).unwrap(),
                        Quantity::new(20, InventoryType::Committed).unwrap(),
                        Quantity::new(30, InventoryType::Incoming).unwrap(),
                        Quantity::new(40, InventoryType::Reserved).unwrap(),
                        Quantity::new(50, InventoryType::SafetyStock).unwrap(),
                        Quantity::new(60, InventoryType::Damaged).unwrap(),
                    ],
                )
                .unwrap()
            })
            .collect();

        map.insert(inventory_item_id.clone(), levels);
        map
    }

    #[actix_web::test]
    async fn test_present_get_inventories_success() {
        let presenter = InventoryPresenterImpl::new();
        let items = mock_inventory_items(10);
        let levels = items
            .iter()
            .map(|item| mock_inventory_level_map(5, item.id()))
            .flatten()
            .collect();

        let result = presenter
            .present_get_inventories(Ok((items, levels)))
            .await
            .unwrap();

        assert_eq!(result.inventories[0].id, "0");
        assert_eq!(result.inventories[0].variant_id, "0");
        assert_eq!(result.inventories[0].inventory_levels[0].id, "0");
        assert_eq!(result.inventories[0].inventory_levels.len(), 5);
        assert_eq!(
            result.inventories[0].inventory_levels[0].quantities.len(),
            6
        );

        assert_eq!(result.inventories[9].id, "9");
        assert_eq!(result.inventories[9].variant_id, "9");
        assert_eq!(result.inventories[9].inventory_levels[0].id, "0");
        assert_eq!(result.inventories[9].inventory_levels.len(), 5);
        assert_eq!(
            result.inventories[9].inventory_levels[0].quantities.len(),
            6
        );
    }

    #[actix_web::test]
    async fn test_present_get_inventories_not_found() {
        let presenter = InventoryPresenterImpl::new();

        let result = presenter
            .present_get_inventories(Ok((vec![], HashMap::new())))
            .await;

        assert!(matches!(
            result,
            Err(GetInventoriesErrorResponse::NotFound { .. })
        ));
    }

    #[actix_web::test]
    async fn test_present_get_inventories_bad_request() {
        let presenter = InventoryPresenterImpl::new();

        let result = presenter
            .present_get_inventories(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(
            result,
            Err(GetInventoriesErrorResponse::BadRequest)
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
            Err(GetInventoriesErrorResponse::ServiceUnavailable)
        ));
    }

    #[actix_web::test]
    async fn test_present_put_inventory_success() {
        let presenter = InventoryPresenterImpl::new();
        let mut level_map = mock_inventory_level_map(1, &("0".to_string()));
        let level = level_map.remove("0").unwrap().remove(0);

        let result = presenter.present_put_inventory(Ok(level)).await.unwrap();

        assert_eq!(result.inventory_level.id, "0");
        assert_eq!(result.inventory_level.location_id, "0");
        assert_eq!(result.inventory_level.quantities.len(), 6);
    }

    #[actix_web::test]
    async fn test_present_put_inventory_not_found() {
        let presenter = InventoryPresenterImpl::new();

        let result = presenter
            .present_put_inventory(Err(DomainError::NotFound))
            .await;

        assert!(matches!(
            result,
            Err(PutInventoryErrorResponse::NotFound { .. })
        ));
    }

    #[actix_web::test]
    async fn test_present_put_inventory_bad_request() {
        let presenter = InventoryPresenterImpl::new();

        let result = presenter
            .present_put_inventory(Err(DomainError::InvalidRequest))
            .await;

        assert!(matches!(result, Err(PutInventoryErrorResponse::BadRequest)));
    }

    #[actix_web::test]
    async fn test_present_put_inventory_service_unavailable() {
        let presenter = InventoryPresenterImpl::new();

        let result = presenter
            .present_put_inventory(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(PutInventoryErrorResponse::ServiceUnavailable)
        ));
    }
}
