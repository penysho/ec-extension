use chrono::{DateTime, Utc};
use derive_getters::Getters;

use crate::domain::error::error::DomainError;

use super::{barcode::barcode::Barcode, sku::sku::Sku};

pub type Id = String;

/// Express product information in SKU units.
///
/// # Fields
///
/// * `id` - Unique identifier of the variant.
/// * `name` - Name of the variant.
/// * `price` - Price of the variant.
/// * `sku` - Stock Keeping Unit of the variant.
/// * `barcode` - Barcode of the variant.
/// * `inventory_quantity` - Quantity of the variant in inventory.
/// * `list_order` - Order of the variant in the list.
/// * `created_at` - Date and time when the variant was created.
/// * `updated_at` - Date and time when the variant was last updated.
#[derive(Debug, Getters)]
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
        id: impl Into<String>,
        name: Option<impl Into<String>>,
        price: u32,
        sku: Option<Sku>,
        barcode: Option<Barcode>,
        inventory_quantity: Option<u32>,
        list_order: u8,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }
        let name = name.map(|n| n.into());
        if let Some(ref n) = name {
            if n.is_empty() {
                log::error!("Name cannot be empty");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_variant() {
        let variant = Variant::new(
            "1",
            Some("Test Variant"),
            100,
            Some(Sku::new("ABC123").unwrap()),
            Some(Barcode::new("1234567890").unwrap()),
            Some(10),
            1,
            Utc::now(),
            Utc::now(),
        );
        assert!(variant.is_ok());
    }

    #[test]
    fn test_new_variant_invalid_id() {
        let variant = Variant::new(
            "",
            Some("Test Variant"),
            100,
            Some(Sku::new("ABC123").unwrap()),
            Some(Barcode::new("1234567890").unwrap()),
            Some(10),
            1,
            Utc::now(),
            Utc::now(),
        );
        assert!(variant.is_err());
    }

    #[test]
    fn test_new_variant_invalid_name() {
        let variant = Variant::new(
            "1",
            Some(""),
            100,
            Some(Sku::new("ABC123").unwrap()),
            Some(Barcode::new("1234567890").unwrap()),
            Some(10),
            1,
            Utc::now(),
            Utc::now(),
        );
        assert!(variant.is_err());
    }
}
