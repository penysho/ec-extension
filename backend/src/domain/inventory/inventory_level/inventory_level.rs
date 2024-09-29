use crate::domain::{error::error::DomainError, location::location::Id as LocationId};
use derive_getters::Getters;

use super::quantity::quantity::Quantity;

pub type Id = String;

#[derive(Debug, Getters)]
pub struct InventoryLevel {
    id: Id,
    location_id: LocationId,
    quantities: Vec<Quantity>,
}

impl InventoryLevel {
    pub fn new(
        id: impl Into<String>,
        location_id: impl Into<LocationId>,
        quantities: Vec<Quantity>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }

        Ok(Self {
            id,
            location_id: location_id.into(),
            quantities,
        })
    }
}
