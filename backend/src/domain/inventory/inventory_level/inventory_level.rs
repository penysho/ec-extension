use crate::domain::location::location::Id as LocationId;
use chrono::{DateTime, Utc};

use super::quantity::quantity::Quantity;

pub type Id = String;

pub struct InventoryLevel {
    id: Id,
    location_id: LocationId,
    quantities: Vec<Quantity>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
