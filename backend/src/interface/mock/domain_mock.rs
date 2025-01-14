/// Define functions for common use in interface layer tests.
/// Generate a mock of the domain.
use crate::domain::{
    address::address::Address,
    customer::customer::{Customer, CustomerStatus},
    draft_order::draft_order::{DraftOrder, DraftOrderStatus},
    email::email::Email,
    inventory_item::inventory_item::{Id as InventoryItemId, InventoryItem},
    inventory_level::{
        inventory_level::InventoryLevel,
        quantity::quantity::{InventoryType, Quantity},
    },
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
    phone::phone::Phone,
    product::{
        product::{Product, ProductStatus},
        variant::{
            barcode::barcode::Barcode,
            sku::sku::Sku,
            variant::{InventoryPolicy, Variant},
        },
    },
};

use chrono::Utc;

use std::collections::HashMap;

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

pub fn mock_money() -> Money {
    let amount = Amount::new(100.0).unwrap();
    Money::new(CurrencyCode::USD, amount).expect("Failed to create mock money")
}

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
            .expect("Failed to create mock inventory item")
        })
        .collect()
}

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
            .expect("Failed to create mock product")
        })
        .collect()
}

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
            .expect("Failed to create mock media")
        })
        .collect()
}

pub fn mock_inventory_level_map(
    count: usize,
    inventory_item_id: &InventoryItemId,
) -> HashMap<InventoryItemId, Vec<InventoryLevel>> {
    let mut map: HashMap<InventoryItemId, Vec<InventoryLevel>> = HashMap::new();

    let levels = (0..count)
        .map(|i| {
            InventoryLevel::new(
                format!("{i}"),
                inventory_item_id.clone(),
                format!("{i}"),
                vec![
                    Quantity::new(10, InventoryType::Available).unwrap(),
                    Quantity::new(20, InventoryType::Committed).unwrap(),
                    Quantity::new(30, InventoryType::Incoming).unwrap(),
                    Quantity::new(40, InventoryType::Reserved).unwrap(),
                    Quantity::new(50, InventoryType::SafetyStock).unwrap(),
                    Quantity::new(60, InventoryType::Damaged).unwrap(),
                ],
            )
            .expect("Failed to create mock inventory level")
        })
        .collect();

    map.insert(inventory_item_id.clone(), levels);
    map
}

pub fn mock_inventory_levels(count: usize) -> Vec<InventoryLevel> {
    (0..count)
        .map(|i| {
            InventoryLevel::new(
                format!("{i}"),
                format!("{i}"),
                format!("{i}"),
                vec![
                    Quantity::new(10, InventoryType::Available).unwrap(),
                    Quantity::new(20, InventoryType::Committed).unwrap(),
                    Quantity::new(30, InventoryType::Incoming).unwrap(),
                    Quantity::new(40, InventoryType::Reserved).unwrap(),
                    Quantity::new(50, InventoryType::SafetyStock).unwrap(),
                    Quantity::new(60, InventoryType::Damaged).unwrap(),
                ],
            )
            .expect("Failed to create mock inventory level")
        })
        .collect()
}

pub fn mock_customers(count: usize) -> Vec<Customer> {
    (0..count)
        .map(|i| {
            Customer::new(
                format!("{i}"),
                format!("user_{i}"),
                vec![mock_address()],
                Some(mock_address()),
                format!("Test Customer {i}"),
                Some(Email::new(format!("{i}@example.com")).unwrap()),
                Some("John"),
                Some("Doe"),
                None,
                Some(Phone::new("+1234567890").unwrap()),
                Some("Note"),
                CustomerStatus::Active,
                true,
                Utc::now(),
                Utc::now(),
            )
            .expect("Failed to create mock customer")
        })
        .collect()
}
