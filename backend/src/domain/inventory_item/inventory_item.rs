use crate::domain::{error::error::DomainError, product::variant::variant::Id as VariantId};
use chrono::{DateTime, Utc};
use derive_getters::Getters;

pub type Id = String;

#[derive(Debug, Getters)]
pub struct InventoryItem {
    id: Id,
    variant_id: VariantId,
    requires_shipping: bool,
    tracked: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InventoryItem {
    pub fn new(
        id: impl Into<String>,
        variant_id: impl Into<VariantId>,
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
            requires_shipping,
            tracked,
            created_at,
            updated_at,
        })
    }
}
