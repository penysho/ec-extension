use derive_getters::Getters;

use crate::domain::{error::error::DomainError, money::money::Money};

/// Represents the type of value applied by a discount.
/// A discount can either have a fixed value or be a percentage-based value.
///
/// # Variants
/// * `Fixed` - A fixed value discount.
/// * `Percentage` - A percentage-based discount.
#[derive(Debug, Clone, PartialEq)]
pub enum DiscountValueType {
    Fixed,
    Percentage,
}

/// Represents the type of value applied by a discount.
/// A discount can either have a fixed value or be a percentage-based value.
///
/// # Fields
/// * `title` - The title of the discount.
/// * `description` - The description of the discount.
/// * `value` - The value of the discount.
/// * `value_type` - The type of value applied by the discount.
/// * `amount_set` - The amount set for the discount.
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Discount {
    title: Option<String>,
    description: Option<String>,
    value: f32,
    value_type: DiscountValueType,
    amount_set: Option<Money>,
}

impl Discount {
    pub fn new(
        title: Option<impl Into<String>>,
        description: Option<impl Into<String>>,
        value: f32,
        value_type: DiscountValueType,
        amount_set: Option<Money>,
    ) -> Result<Self, DomainError> {
        if value < 0.0 {
            return Err(DomainError::ValidationError);
        }

        Ok(Self {
            title: title.map(|t| t.into()),
            description: description.map(|d| d.into()),
            value,
            value_type,
            amount_set,
        })
    }
}
