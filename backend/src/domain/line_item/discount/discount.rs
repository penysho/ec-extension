use derive_getters::Getters;

use crate::domain::{error::error::DomainError, money::money::Money};

#[derive(Debug, Clone, PartialEq)]
pub enum DiscountValueType {
    Fixed,
    Percentage,
}

#[derive(Debug, Getters)]
pub struct Discount {
    title: Option<String>,
    description: Option<String>,
    value: f32,
    value_type: DiscountValueType,
    amount: Money,
}

impl Discount {
    pub fn new(
        title: Option<impl Into<String>>,
        description: Option<impl Into<String>>,
        value: f32,
        value_type: DiscountValueType,
        amount: Money,
    ) -> Result<Self, DomainError> {
        if value < 0.0 {
            return Err(DomainError::ValidationError);
        }

        Ok(Self {
            title: title.map(|t| t.into()),
            description: description.map(|d| d.into()),
            value,
            value_type,
            amount,
        })
    }
}
