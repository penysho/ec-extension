use crate::domain::product::variant::variant::Id as VariantId;
use chrono::{DateTime, Utc};

use super::inventory_level::inventory_level::InventoryLevel;

pub type Id = String;

pub struct Inventory {
    id: Id,
    variant_id: VariantId,
    inventoryLevel: Option<InventoryLevel>,
    tracked: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
