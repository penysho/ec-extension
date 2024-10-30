#[cfg(test)]
use crate::domain::{
    address::address::Address,
    draft_order::draft_order::{DraftOrder, DraftOrderStatus},
    inventory_item::inventory_item::InventoryItem,
    line_item::{
        discount::discount::{Discount, DiscountValueType},
        line_item::LineItem,
    },
    location::location::Location,
    media::{
        associated_id::associated_id::AssociatedId,
        media::{Media, MediaStatus},
        media_content::{image::image::Image, media_content::MediaContent},
        src::src::Src,
    },
    money::{
        amount::amount::Amount,
        money::{CurrencyCode, Money},
    },
    product::{
        product::{Product, ProductStatus},
        variant::{
            barcode::barcode::Barcode,
            sku::sku::Sku,
            variant::{InventoryPolicy, Variant},
        },
    },
};
#[cfg(test)]
use chrono::Utc;

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
            .expect("Failed to create mock location")
        })
        .collect()
}

#[cfg(test)]
pub fn mock_discount() -> Discount {
    Discount::new(
        Some("Test Discount".to_string()),
        Some("Test description".to_string()),
        10.0,
        DiscountValueType::Percentage,
        Some(mock_money()),
    )
    .expect("Failed to create mock discount")
}

#[cfg(test)]
pub fn mock_money() -> Money {
    let amount = Amount::new(100.0).unwrap();
    Money::new(CurrencyCode::USD, amount).expect("Failed to create mock money")
}

#[cfg(test)]
pub fn mock_line_items(count: usize) -> Vec<LineItem> {
    (0..count)
        .map(|i| {
            LineItem::new(
                format!("{i}"),
                false,
                Some("variant_id"),
                5,
                Some(mock_discount()),
                mock_money(),
                mock_money(),
            )
            .expect("Failed to create mock line item")
        })
        .collect()
}

#[cfg(test)]
pub fn mock_draft_orders(count: usize) -> Vec<DraftOrder> {
    (0..count)
        .map(|i| {
            DraftOrder::new(
                format!("{i}"),
                format!("Test Order {i}"),
                DraftOrderStatus::Open,
                None,
                Some(mock_address()),
                Some(mock_address()),
                None,
                mock_line_items(2),
                None,
                Some(mock_discount()),
                mock_money(),
                true,
                false,
                mock_money(),
                mock_money(),
                mock_money(),
                mock_money(),
                CurrencyCode::JPY,
                None,
                None,
                Utc::now(),
                Utc::now(),
            )
            .expect("Failed to create mock draft order")
        })
        .collect()
}

#[cfg(test)]
pub fn mock_inventory_items(count: usize) -> Vec<InventoryItem> {
    (0..count)
        .map(|i| {
            InventoryItem::new(
                format!("{i}"),
                format!("{i}"),
                true,
                false,
                Utc::now(),
                Utc::now(),
            )
            .unwrap()
        })
        .collect()
}

#[cfg(test)]
pub fn mock_products(count: usize) -> Vec<Product> {
    (0..count)
        .map(|i| {
            Product::new(
                format!("{i}"),
                format!("Test Product {i}"),
                "This is a test product description.",
                ProductStatus::Active,
                vec![Variant::new(
                    format!("{i}"),
                    Some(format!("Test Variant {i}")),
                    Some(Sku::new("ABC123").unwrap()),
                    Some(Barcode::new("1234567890").unwrap()),
                    true,
                    1,
                    "test_inventory_id",
                    InventoryPolicy::Continue,
                    Some(1),
                    Amount::new(100.0).unwrap(),
                    true,
                    Some("tax_code".to_string()),
                    Utc::now(),
                    Utc::now(),
                )
                .unwrap()],
                Some("111"),
            )
            .unwrap()
        })
        .collect()
}

#[cfg(test)]
pub fn mock_media(count: usize) -> Vec<Media> {
    (0..count)
        .map(|i| {
            Media::new(
                format!("{i}"),
                Some(format!("Test Media {i}")),
                MediaStatus::Active,
                Some(MediaContent::Image(
                    Image::new(
                        format!("{i}"),
                        Some(AssociatedId::Product(format!("{i}"))),
                        Some(format!("Alt Text {i}")),
                        Some(Src::new(format!("https://example.com/uploaded_{}.jpg", i)).unwrap()),
                        Some(Src::new(format!("https://example.com/published_{}.jpg", i)).unwrap()),
                    )
                    .unwrap(),
                )),
                Utc::now(),
                Utc::now(),
            )
            .unwrap()
        })
        .collect()
}
