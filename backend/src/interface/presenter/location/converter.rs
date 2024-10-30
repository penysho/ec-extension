use crate::domain::location::location::Location;

use super::schema::LocationSchema;

impl From<Location> for LocationSchema {
    fn from(location: Location) -> Self {
        Self {
            id: location.id().to_string(),
            name: location.name().to_string(),
            is_active: *location.is_active(),
            fulfills_online_orders: *location.fulfills_online_orders(),
            address: location.address().to_owned().into(),
            suggested_addresses: location
                .suggested_addresses()
                .to_owned()
                .into_iter()
                .map(|a| a.into())
                .collect(),
        }
    }
}
