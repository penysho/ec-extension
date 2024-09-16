use derive_getters::Getters;

use crate::domain::error::error::DomainError;

use super::{category::category::Id as CategoryId, variant::variant::Variant};

pub type Id = String;

#[derive(Debug, Clone, PartialEq)]
pub enum ProductStatus {
    Active,
    Inactive,
    Draft,
}

/// Entity of Products.
#[derive(Debug, Getters)]
pub struct Product {
    id: Id,
    name: String,
    description: String,
    status: ProductStatus,
    variants: Vec<Variant>,
    category_id: Option<CategoryId>,
}

impl Product {
    pub const MAX_DESCRIPTION_LENGTH: u32 = 10000;

    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        status: ProductStatus,
        variants: Vec<Variant>,
        category_id: Option<CategoryId>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            return Err(DomainError::ValidationError);
        }
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
            description,
            status,
            variants,
            category_id,
        })
    }

    // Add variant to products with the same part number
    pub fn add_variant(&mut self, variant: Variant) -> Result<(), DomainError> {
        self.variants.push(variant);
        Ok(())
    }
}
