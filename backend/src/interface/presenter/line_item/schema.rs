use serde::{Deserialize, Serialize};

use crate::interface::presenter::money::schema::MoneyBagSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItemSchema {
    pub(super) id: String,
    pub(super) is_custom: bool,
    pub(super) variant_id: Option<String>,
    pub(super) quantity: u32,
    pub(super) discounted_total_set: MoneyBagSchema,
    pub(super) original_total_set: MoneyBagSchema,
}
