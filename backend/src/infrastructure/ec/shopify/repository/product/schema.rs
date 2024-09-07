use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        product::{
            barcode::barcode::Barcode,
            product::{Product, ProductStatus},
            sku::sku::Sku,
        },
    },
    infrastructure::ec::shopify::repository::common::schema::Edges,
};

#[derive(Debug, Deserialize)]
pub(super) struct ProductSchema {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) price: f64,
    pub(super) description: String,
    pub(super) status: String,
    pub(super) sku: Option<String>,
    pub(super) barcode: Option<String>,
    pub(super) inventory_quantity: Option<i32>,
    pub(super) position: i32,
    pub(super) category_id: Option<String>,
}

impl From<VariantNode> for ProductSchema {
    fn from(node: VariantNode) -> Self {
        ProductSchema {
            id: node.id,
            title: node.product.title,
            price: node
                .product
                .price
                .max_variant_price
                .amount
                .parse::<f64>()
                .unwrap_or(0.0),
            description: node.product.description,
            status: node.product.status,
            category_id: node.product.category.map(|c| c.id),
            barcode: node.barcode,
            inventory_quantity: node.inventory_quantity,
            sku: node.sku,
            position: node.position,
        }
    }
}

impl ProductSchema {
    pub(super) fn to_domain(self) -> Result<Product, DomainError> {
        let status = match self.status.as_str() {
            "ACTIVE" => ProductStatus::Active,
            "ARCHIVED" => ProductStatus::Inactive,
            "DRAFT" => ProductStatus::Draft,
            _ => ProductStatus::Inactive,
        };
        let sku = match self.sku {
            Some(sku) => Some(Sku::new(sku)),
            None => None,
        };
        let barcode = match self.barcode {
            Some(barcode) => Some(Barcode::new(barcode)),
            None => None,
        };
        let inventory_quantity = match self.inventory_quantity {
            Some(inventory_quantity) => Some(inventory_quantity as u32),
            None => None,
        };

        Product::new(
            self.id,
            self.title,
            self.price as u32,
            self.description,
            status,
            sku,
            barcode,
            inventory_quantity,
            self.position as u8,
            self.category_id,
        )
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct TaxonomyCategory {
    pub(super) id: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct MaxVariantPrice {
    pub(super) amount: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct PriceRangeV2 {
    #[serde(rename = "maxVariantPrice")]
    pub(super) max_variant_price: MaxVariantPrice,
}

#[derive(Debug, Deserialize)]
pub(super) struct InventoryNode {
    pub(super) id: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct VariantNode {
    pub(super) id: String,

    pub(super) product: ProductNode,
    #[serde(rename = "inventoryItem")]
    pub(super) inventory_item: InventoryNode,

    pub(super) barcode: Option<String>,
    #[serde(rename = "inventoryQuantity")]
    pub(super) inventory_quantity: Option<i32>,
    pub(super) sku: Option<String>,
    pub(super) position: i32,
    pub(super) price: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct ProductNode {
    pub(super) id: String,

    pub(super) category: Option<TaxonomyCategory>,

    pub(super) title: String,
    #[serde(rename = "priceRangeV2")]
    pub(super) price: PriceRangeV2,
    pub(super) description: String,
    pub(super) status: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct VariantsData {
    #[serde(rename = "productVariants")]
    pub(super) product_variants: Edges<VariantNode>,
}

#[derive(Debug, Deserialize)]
pub(super) struct VariantData {
    #[serde(rename = "productVariant")]
    pub(super) product_variant: Option<VariantNode>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(super) struct ProductsData {
    pub(super) products: Edges<ProductNode>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(super) struct ProductData {
    pub(super) product: Option<ProductNode>,
}
