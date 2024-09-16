use chrono::{DateTime, Utc};
use derive_getters::Getters;

use crate::domain::error::error::DomainError;

use super::{barcode::barcode::Barcode, sku::sku::Sku};

pub type Id = String;

#[derive(Debug, Getters, Clone)]
pub struct Variant {
    id: Id,
    name: Option<String>,
    price: u32,
    sku: Option<Sku>,
    barcode: Option<Barcode>,
    inventory_quantity: Option<u32>,
    list_order: u8,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Variant {
    pub fn new(
        id: Id,
        name: Option<impl Into<String>>,
        price: u32,
        sku: Option<Sku>,
        barcode: Option<Barcode>,
        inventory_quantity: Option<u32>,
        list_order: u8,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        let name = name.map(|n| n.into());
        if let Some(ref n) = name {
            if n.is_empty() {
                return Err(DomainError::ValidationError);
            }
        }

        Ok(Variant {
            id,
            name,
            price,
            sku,
            barcode,
            inventory_quantity,
            created_at,
            updated_at,
            list_order,
        })
    }
}