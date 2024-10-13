use serde::{Deserialize, Serialize};

use crate::interface::presenter::money::schema::MoneyBagSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItemSchema {
    pub id: String,
    pub is_custom: bool,
    pub variant_id: Option<String>,
    pub quantity: u32,
    pub discounted_total_set: MoneyBagSchema,
    pub original_total_set: MoneyBagSchema,
}
