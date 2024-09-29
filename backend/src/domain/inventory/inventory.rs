use crate::domain::{error::error::DomainError, product::variant::variant::Id as VariantId};
use chrono::{DateTime, Utc};
use derive_getters::Getters;

use super::inventory_level::inventory_level::InventoryLevel;

pub type Id = String;

#[derive(Debug, Getters)]
pub struct Inventory {
    id: Id,
    variant_id: VariantId,
    inventory_levels: Vec<InventoryLevel>,
    requires_shipping: bool,
    tracked: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Inventory {
    pub fn new(
        id: impl Into<String>,
        variant_id: impl Into<VariantId>,
        inventory_levels: Vec<InventoryLevel>,
        requires_shipping: bool,
        tracked: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }

        Ok(Self {
            id,
            variant_id: variant_id.into(),
            inventory_levels,
            requires_shipping,
            tracked,
            created_at,
            updated_at,
        })
    }
}
