use derive_getters::Getters;

use crate::domain::error::error::DomainError;

use super::{category::category::Id as CategoryId, variant::variant::Variant};

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
    description: String,
    status: ProductStatus,
    variants: Vec<Variant>,
    category_id: Option<CategoryId>,
}

impl Product {
    pub const MAX_DESCRIPTION_LENGTH: u32 = 10000;

    pub fn new(
        id: Id,
        name: impl Into<String>,
        description: impl Into<String>,
        status: ProductStatus,
        variants: Vec<Variant>,
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
            description,
            status,
            variants,
            category_id,
        })
    }

    pub fn add_variant(&mut self, variant: Variant) -> Result<(), DomainError> {
        self.variants.push(variant);
        Ok(())
    }
}
