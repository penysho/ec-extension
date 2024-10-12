use derive_getters::Getters;

use crate::domain::{error::error::DomainError, money::money_bag::MoneyBag};

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
    amount_set: MoneyBag,
}

impl Discount {
    pub fn new(
        title: Option<impl Into<String>>,
        description: Option<impl Into<String>>,
        value: f32,
        value_type: DiscountValueType,
        amount_set: MoneyBag,
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
