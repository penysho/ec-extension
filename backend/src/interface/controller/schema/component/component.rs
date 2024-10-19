/// Define the request schema of the API in the controller
///
/// Components in the schema of each API are defined here.
///
/// * The top-level schema specific to each URI is not defined here (e.g. ~Request, ~Response)
/// * To avoid naming conflicts with the domain, each component should be suffixed with “Schema
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AddressSchema {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub province: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LineItemSchema {
    pub variant_id: Option<String>,
    pub quantity: u32,
    pub applied_discount: Option<DiscountSchema>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DiscountSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub value: f32,
    pub value_type: DiscountValueTypeSchema,
    pub amount_set: Option<MoneyBagSchema>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DiscountValueTypeSchema {
    Fixed,
    Percentage,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MoneyBagSchema {
    pub currency_code: CurrencyCodeSchema,
    pub amount: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CurrencyCodeSchema {
    USD,
    EUR,
    GBP,
    JPY,
}
