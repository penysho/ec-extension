#[cfg(test)]
use crate::domain::{address::address::Address, location::location::Location};

#[cfg(test)]
pub fn mock_address() -> Address {
    Address::new(
        Some("123 Main St"),
        None::<String>,
        Some("City"),
        true,
        Some("Country"),
        Some("John"),
        Some("Doe"),
        Some("Province"),
        Some("12345"),
        Some("+1234567890"),
    )
    .expect("Failed to create mock address")
}

#[cfg(test)]
pub fn mock_locations(count: usize) -> Vec<Location> {
    (0..count)
        .map(|i| {
            Location::new(
                format!("{i}"),
                format!("{i}"),
                true,
                false,
                mock_address(),
                vec![mock_address()],
            )
            .unwrap()
        })
        .collect()
}
