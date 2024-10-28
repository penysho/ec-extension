use crate::domain::{
    address::address::Address,
    error::error::DomainError,
    line_item::discount::discount::{Discount, DiscountValueType},
    money::{
        amount::amount::Amount,
        money::{CurrencyCode, Money},
    },
};

use super::component::{
    AddressSchema, CurrencyCodeSchema, DiscountSchema, DiscountValueTypeSchema, MoneySchema,
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
        let amount_set = self
            .amount_set
            .to_owned()
            .map(|money| money.to_domain())
            .transpose()?;

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

impl MoneySchema {
    pub fn to_domain(self) -> Result<Money, DomainError> {
        let currency_code = match self.currency_code.to_owned() {
            CurrencyCodeSchema::USD => CurrencyCode::USD,
            CurrencyCodeSchema::EUR => CurrencyCode::EUR,
            CurrencyCodeSchema::GBP => CurrencyCode::GBP,
            CurrencyCodeSchema::JPY => CurrencyCode::JPY,
        };
        let amount = Amount::new(self.amount.to_owned())?;

        Money::new(currency_code, amount)
    }
}

impl CurrencyCodeSchema {
    pub fn to_domain(self) -> Result<CurrencyCode, DomainError> {
        match self {
            CurrencyCodeSchema::USD => Ok(CurrencyCode::USD),
            CurrencyCodeSchema::EUR => Ok(CurrencyCode::EUR),
            CurrencyCodeSchema::GBP => Ok(CurrencyCode::GBP),
            CurrencyCodeSchema::JPY => Ok(CurrencyCode::JPY),
        }
    }
}
