use derive_getters::Getters;

use crate::domain::error::error::DomainError;

use super::{barcode::barcode::Barcode, category::category::Id as CategoryId, sku::sku::Sku};

pub type Id = String;

#[derive(Debug, Clone)]
pub enum ProductStatus {
    Active,
    Inactive,
    Draft,
}

/// Entity of Products.
#[derive(Debug, Getters, Clone)]
pub struct Product {
    id: Id,
    name: String,
    price: u32,
    description: String,
    status: ProductStatus,
    sku: Option<Sku>,
    barcode: Option<Barcode>,
    inventory_quantity: Option<u32>,
    list_order: u8,
    category_id: Option<CategoryId>,
}

impl Product {
    pub const MAX_DESCRIPTION_LENGTH: u32 = 10000;

    pub fn new(
        id: Id,
        name: impl Into<String>,
        price: u32,
        description: impl Into<String>,
        status: ProductStatus,
        sku: Option<Sku>,
        barcode: Option<Barcode>,
        inventory_quantity: Option<u32>,
        list_order: u8,
        category_id: Option<CategoryId>,
    ) -> Result<Self, DomainError> {
        let name = name.into();
        if name.is_empty() {
            return Err(DomainError::ValidationError);
        }

        let description = description.into();
        if description.len() as u32 > Self::MAX_DESCRIPTION_LENGTH {
            return Err(DomainError::ValidationError);
        }

        Ok(Product {
            id,
            name,
            price,
            description,
            status,
            sku,
            barcode,
            inventory_quantity,
            list_order,
            category_id,
        })
    }
}
