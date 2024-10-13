use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressSchema {
    pub(super) id: String,
    pub(super) address1: Option<String>,
    pub(super) address2: Option<String>,
    pub(super) city: Option<String>,
    pub(super) coordinates_validated: bool,
    pub(super) country: Option<String>,
    pub(super) first_name: Option<String>,
    pub(super) last_name: Option<String>,
    pub(super) province: Option<String>,
    pub(super) zip: Option<String>,
    pub(super) phone: Option<String>,
}
