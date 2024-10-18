use crate::domain::{
    address::address::Address,
    error::error::DomainError,
    line_item::discount::discount::{Discount, DiscountValueType},
    money::{
        money::money::Money,
        money_bag::{CurrencyCode, MoneyBag},
    },
};

use super::component::{
    AddressSchema, CurrencyCodeSchema, DiscountSchema, DiscountValueTypeSchema, MoneyBagSchema,
};

impl AddressSchema {
    pub fn to_domain(self) -> Result<Address, DomainError> {
        Address::new(
            self.address1.to_owned(),
            self.address2.to_owned(),
            self.city.to_owned(),
            false,
            self.country.to_owned(),
            self.first_name.to_owned(),
            self.last_name.to_owned(),
            self.province.to_owned(),
            self.zip.to_owned(),
            self.phone.to_owned(),
        )
    }
}

impl DiscountSchema {
    pub fn to_domain(self) -> Result<Discount, DomainError> {
        let value_type = self.value_type.to_owned().to_domain()?;
        let amount_set = self.amount_set.to_owned().to_domain()?;

        Discount::new(
            self.title.to_owned(),
            self.description.to_owned(),
            self.value.to_owned(),
            value_type,
            amount_set,
        )
    }
}

impl DiscountValueTypeSchema {
    pub fn to_domain(self) -> Result<DiscountValueType, DomainError> {
        match self {
            DiscountValueTypeSchema::Fixed => Ok(DiscountValueType::Fixed),
            DiscountValueTypeSchema::Percentage => Ok(DiscountValueType::Percentage),
        }
    }
}

impl MoneyBagSchema {
    pub fn to_domain(self) -> Result<MoneyBag, DomainError> {
        let currency_code = match self.currency_code.to_owned() {
            CurrencyCodeSchema::USD => CurrencyCode::USD,
            CurrencyCodeSchema::EUR => CurrencyCode::EUR,
            CurrencyCodeSchema::GBP => CurrencyCode::GBP,
            CurrencyCodeSchema::JPY => CurrencyCode::JPY,
        };
        let money = Money::new(self.amount.to_owned())?;

        MoneyBag::new(currency_code, money)
    }
}
