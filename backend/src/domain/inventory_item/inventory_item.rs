use crate::domain::{error::error::DomainError, product::variant::variant::Id as VariantId};
use chrono::{DateTime, Utc};
use derive_getters::Getters;

pub type Id = String;

/// Represents an inventory item in the system.
///
/// The `InventoryItem` struct contains information related to a specific inventory item,
/// such as its ID, associated variant, shipping requirements, and tracking information,
/// as well as the timestamps for when the item was created and last updated.
///
/// # Fields
/// - `id` - The unique identifier for the inventory item.
/// - `variant_id` - The identifier for the associated product variant.
/// - `requires_shipping` - Indicates whether the item requires shipping.
/// - `tracked` - Indicates whether the item is tracked for inventory management purposes.
/// - `created_at` - The timestamp indicating when the item was created.
/// - `updated_at` - The timestamp indicating when the item was last updated.
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
