use crate::domain::customer::customer::{Customer, CustomerStatus};

use super::schema::{CustomerSchema, CustomerStatusSchema};

impl From<Customer> for CustomerSchema {
    fn from(customer: Customer) -> Self {
        CustomerSchema {
            id: customer.id().to_string(),
            addresses: customer
                .addresses()
                .to_owned()
                .into_iter()
                .map(|address| address.into())
                .collect(),
            default_address: customer
                .default_address()
                .to_owned()
                .map(|address| address.into()),
            display_name: customer.display_name().to_string(),
            email: customer
                .email()
                .as_ref()
                .map(|email| email.value().to_string()),
            first_name: customer
                .first_name()
                .as_ref()
                .map(|first_name| first_name.to_string()),
            last_name: customer
                .last_name()
                .as_ref()
                .map(|last_name| last_name.to_string()),
            phone: customer
                .phone()
                .as_ref()
                .map(|phone| phone.value().to_string()),
            note: customer.note().as_ref().map(|note| note.to_string()),
            status: customer.status().to_owned().into(),
            verified_email: *customer.verified_email(),
            created_at: *customer.created_at(),
            updated_at: *customer.updated_at(),
        }
    }
}

impl From<CustomerStatus> for CustomerStatusSchema {
    fn from(status: CustomerStatus) -> Self {
        match status {
            CustomerStatus::Active => CustomerStatusSchema::Active,
            CustomerStatus::Inactive => CustomerStatusSchema::Inactive,
        }
    }
}
