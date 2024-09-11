use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        product::{
            product::{Product, ProductStatus},
            variant::{barcode::barcode::Barcode, sku::sku::Sku, variant::Variant},
        },
    },
    infrastructure::ec::shopify::repository::common::schema::Edges,
};

#[derive(Debug, Deserialize)]
pub(super) struct ProductSchema {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) description: String,
    pub(super) status: String,
    pub(super) category_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct VariantSchema {
    pub(super) id: String,

    pub(super) product: ProductSchema,

    pub(super) price: f32,
    pub(super) sku: Option<String>,
    pub(super) barcode: Option<String>,
    pub(super) inventory_quantity: Option<i32>,
    pub(super) position: i32,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

impl From<VariantNode> for VariantSchema {
    fn from(node: VariantNode) -> Self {
        VariantSchema {
            product: ProductSchema {
                id: node.product.id,
                title: node.product.title,
                description: node.product.description,
                status: node.product.status,
                category_id: node.product.category.map(|c| c.id),
            },
            id: node.id,
            price: node.price.parse::<f32>().unwrap_or(0.0),
            barcode: node.barcode,
            inventory_quantity: node.inventory_quantity,
            sku: node.sku,
            position: node.position,
            created_at: node.created_at,
            updated_at: node.updated_at,
        }
    }
}

impl VariantSchema {
    pub(super) fn to_domain(self) -> Result<Product, DomainError> {
        let status = match self.product.status.as_str() {
            "ACTIVE" => ProductStatus::Active,
            "ARCHIVED" => ProductStatus::Inactive,
            "DRAFT" => ProductStatus::Draft,
            _ => ProductStatus::Inactive,
        };
        let sku = match self.sku {
            Some(sku) => Some(Sku::new(sku)?),
            None => None,
        };
        let barcode = match self.barcode {
            Some(barcode) => Some(Barcode::new(barcode)?),
            None => None,
        };
        let inventory_quantity = match self.inventory_quantity {
            Some(inventory_quantity) => Some(inventory_quantity as u32),
            None => None,
        };
        let title = self.product.title;

        let variants = vec![Variant::new(
            self.id,
            Some(&title),
            self.price as u32,
            sku,
            barcode,
            inventory_quantity,
            self.position as u8,
            self.created_at,
            self.updated_at,
        )?];

        Product::new(
            self.product.id,
            title,
            self.product.description,
            status,
            variants,
            self.product.category_id,
        )
    }

    pub(super) fn to_domains(
        variant_schemas: Vec<VariantSchema>,
    ) -> Result<Vec<Product>, DomainError> {
        if variant_schemas.is_empty() {
            return Ok(Vec::new());
        }

        let mut product_domain_map: HashMap<String, Product> = HashMap::new();
        let mut products_in_order: Vec<Product> = Vec::new();

        for variant_schema in variant_schemas {
            let sku = variant_schema.sku.map(Sku::new).transpose()?;
            let barcode = variant_schema.barcode.map(Barcode::new).transpose()?;
            let inventory_quantity = variant_schema.inventory_quantity.map(|qty| qty as u32);
            let title = variant_schema.product.title.clone();

            let variant_domain = Variant::new(
                variant_schema.id,
                Some(&title),
                variant_schema.price as u32,
                sku,
                barcode,
                inventory_quantity,
                variant_schema.position as u8,
                variant_schema.created_at,
                variant_schema.updated_at,
            )?;

            match product_domain_map.get_mut(&variant_schema.product.id) {
                Some(product_domain) => {
                    let _ = product_domain.add_variant(variant_domain);
                }
                None => {
                    let product_id = variant_schema.product.id.clone();
                    let title = variant_schema.product.title.clone();
                    let description = variant_schema.product.description.clone();
                    let status = match variant_schema.product.status.as_str() {
                        "ACTIVE" => ProductStatus::Active,
                        "ARCHIVED" => ProductStatus::Inactive,
                        "DRAFT" => ProductStatus::Draft,
                        _ => ProductStatus::Inactive,
                    };
                    let category_id = variant_schema.product.category_id.clone();

                    let product_domain = Product::new(
                        product_id.clone(),
                        title,
                        description,
                        status,
                        vec![variant_domain],
                        category_id,
                    )?;

                    product_domain_map.insert(product_id.clone(), product_domain.clone());
                    products_in_order.push(product_domain);
                }
            };
        }

        Ok(products_in_order)
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
pub(super) struct ProductsData {
    pub(super) products: Edges<ProductNode>,
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
    #[serde(rename = "createdAt")]
    pub(super) created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub(super) updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub(super) struct VariantsData {
    #[serde(rename = "productVariants")]
    pub(super) product_variants: Edges<VariantNode>,
}
