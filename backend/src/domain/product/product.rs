use derive_getters::Getters;

use crate::domain::error::error::DomainError;

use super::{category::category::Id as CategoryId, variant::variant::Variant};

pub type Id = String;

/// Represents the status of a product.
///
/// # Variants
/// - `Active` - The product is available for purchase.
/// - `Inactive` - The product is no longer available for purchase.
/// - `Draft` - The product is in the process of being created or updated.
#[derive(Debug, Clone, PartialEq)]
pub enum ProductStatus {
    Active,
    Inactive,
    Draft,
}

/// Expresses product information by part number.
///
/// # Fields
/// - `id` - The unique identifier for the product.
/// - `name` - The name of the product.
/// - `description` - A detailed description of the product.
/// - `status` - The current status of the product (e.g., Draft, Published).
/// - `variants` - A list of variants associated with the product. Variants represent different
///   configurations of the product (e.g., different sizes or colors).
/// - `category_id` - An optional field that represents the ID of the category to which the product
///   belongs. If the product is not categorized, this will be `None`.
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
        category_id: Option<impl Into<CategoryId>>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }
        let name = name.into();
        if name.is_empty() {
            log::error!("Name cannot be empty");
            return Err(DomainError::ValidationError);
        }
        let description = description.into();
        if description.len() as u32 > Self::MAX_DESCRIPTION_LENGTH {
            log::error!(
                "Description cannot be longer than {} characters",
                Self::MAX_DESCRIPTION_LENGTH
            );
            return Err(DomainError::ValidationError);
        }

        Ok(Product {
            id,
            name,
            description,
            status,
            variants,
            category_id: category_id.map(|c| c.into()),
        })
    }

    // Add variant to products with the same part number
    pub fn add_variant(&mut self, variant: Variant) -> Result<(), DomainError> {
        self.variants.push(variant);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_product() {
        let product = Product::new(
            "1",
            "Product 1",
            "Description 1",
            ProductStatus::Active,
            vec![],
            None::<CategoryId>,
        );
        assert!(product.is_ok());
    }

    #[test]
    fn test_new_product_invalid_id() {
        let product = Product::new(
            "",
            "Product 1",
            "Description 1",
            ProductStatus::Active,
            vec![],
            None::<CategoryId>,
        );
        assert!(product.is_err());
    }

    #[test]
    fn test_new_product_invalid_name() {
        let product = Product::new(
            "1",
            "",
            "Description 1",
            ProductStatus::Active,
            vec![],
            None::<CategoryId>,
        );
        assert!(product.is_err());
    }

    #[test]
    fn test_new_product_invalid_description() {
        let description = "a".repeat((Product::MAX_DESCRIPTION_LENGTH + 1) as usize);
        let product = Product::new(
            "1",
            "Product 1",
            description,
            ProductStatus::Active,
            vec![],
            None::<CategoryId>,
        );
        assert!(product.is_err());
    }
}
