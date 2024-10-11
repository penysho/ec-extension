use derive_getters::Getters;

use crate::domain::{
    error::error::DomainError, money::money::Money, product::variant::variant::Id as VariantId,
};

use super::discount::discount::Discount;

pub type Id = String;

#[derive(Debug, Getters)]
pub struct LineItem {
    id: Id,
    is_custom: bool,
    variant_id: Option<VariantId>,
    quantity: u32,
    discount: Option<Discount>,
    discounted_amount: Option<Money>,
    original_total_amount: Option<Money>,
}

impl LineItem {
    pub fn new(
        id: Id,
        is_custom: bool,
        variant_id: Option<impl Into<VariantId>>,
        quantity: u32,
        discount: Option<Discount>,
        discounted_amount: Option<Money>,
        original_total_amount: Option<Money>,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id,
            is_custom,
            variant_id: variant_id.map(|id| id.into()),
            quantity,
            discount,
            discounted_amount,
            original_total_amount,
        })
    }
}
