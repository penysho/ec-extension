use chrono::{DateTime, Utc};
use derive_getters::Getters;

use crate::domain::{
    error::error::DomainError, inventory_item::inventory_item::Id as InventoryItemId,
    money::amount::amount::Amount,
};

use super::{barcode::barcode::Barcode, sku::sku::Sku};

pub type Id = String;

/// Inventory policy for the product.
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryPolicy {
    Deny,
    Continue,
}

/// Express product information in SKU units.
///
/// # Fields
///
/// * `id` - Unique identifier of the variant.
/// * `name` - Name of the variant.
/// * `sku` - Stock Keeping Unit of the variant.
/// * `barcode` - Barcode of the variant.
/// * `available_for_sale` - Whether the variant is available for sale.
/// * `list_order` - Order of the variant in the list.
/// * `inventory_item_id` - Identifier of the associated inventory item.
/// * `inventory_policy` - Inventory policy for the product.
/// * `inventory_quantity` - Quantity of the variant in inventory.
/// * `price` - Price of the variant.
/// * `taxable` - Whether the variant is taxable.
/// * `tax_code` - Tax code of the variant.
/// * `created_at` - Date and time when the variant was created.
/// * `updated_at` - Date and time when the variant was last updated.
#[derive(Debug, Getters)]
pub struct Variant {
    id: Id,
    name: Option<String>,
    sku: Option<Sku>,
    barcode: Option<Barcode>,
    available_for_sale: bool,
    list_order: u8,

    inventory_item_id: InventoryItemId,
    inventory_policy: InventoryPolicy,
    inventory_quantity: Option<u32>,

    price: Amount,
    taxable: bool,
    tax_code: Option<String>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Variant {
    pub fn new(
        id: impl Into<String>,
        name: Option<impl Into<String>>,
        sku: Option<Sku>,
        barcode: Option<Barcode>,
        available_for_sale: bool,
        list_order: u8,
        inventory_item_id: impl Into<InventoryItemId>,
        inventory_policy: InventoryPolicy,
        inventory_quantity: Option<u32>,
        price: Amount,
        taxable: bool,
        tax_code: Option<String>,
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
            sku,
            barcode,
            available_for_sale,
            list_order,
            inventory_item_id: inventory_item_id.into(),
            inventory_policy,
            inventory_quantity,
            price,
            taxable,
            tax_code,
            created_at,
            updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let variant = Variant::new(
            "1",
            Some("Test Variant"),
            Some(Sku::new("ABC123").unwrap()),
            Some(Barcode::new("1234567890").unwrap()),
            true,
            1,
            "test_inventory_id",
            InventoryPolicy::Continue,
            Some(1),
            Amount::new(100.0).unwrap(),
            true,
            Some("tax_code".to_string()),
            Utc::now(),
            Utc::now(),
        );
        assert!(variant.is_ok());
    }

    #[test]
    fn test_new_invalid_id() {
        let variant = Variant::new(
            "",
            Some("Test Variant"),
            Some(Sku::new("ABC123").unwrap()),
            Some(Barcode::new("1234567890").unwrap()),
            true,
            1,
            "test_inventory_id",
            InventoryPolicy::Continue,
            Some(1),
            Amount::new(100.0).unwrap(),
            true,
            Some("tax_code".to_string()),
            Utc::now(),
            Utc::now(),
        );
        assert!(variant.is_err());
    }

    #[test]
    fn test_new_invalid_name() {
        let variant = Variant::new(
            "1",
            Some(""),
            Some(Sku::new("ABC123").unwrap()),
            Some(Barcode::new("1234567890").unwrap()),
            true,
            1,
            "test_inventory_id",
            InventoryPolicy::Continue,
            Some(1),
            Amount::new(100.0).unwrap(),
            true,
            Some("tax_code".to_string()),
            Utc::now(),
            Utc::now(),
        );
        assert!(variant.is_err());
    }
}
