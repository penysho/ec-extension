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
    infrastructure::ec::shopify::{
        query_helper::ShopifyGQLQueryHelper, repository::schema::common::Edges,
    },
};

#[derive(Debug, Deserialize)]
pub struct ProductSchema {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub category_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VariantSchema {
    pub id: String,

    pub product: ProductSchema,

    pub price: f32,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub inventory_quantity: Option<i32>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    pub fn to_variant_domain(self) -> Result<Variant, DomainError> {
        let sku = self.sku.clone().map(Sku::new).transpose()?;
        let barcode = self.barcode.clone().map(Barcode::new).transpose()?;
        let inventory_quantity = self.inventory_quantity.map(|qty| qty as u32);

        Variant::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            None::<String>,
            self.price as u32,
            sku,
            barcode,
            inventory_quantity,
            self.position as u8,
            self.created_at,
            self.updated_at,
        )
    }

    pub fn to_product_domains(
        variant_schemas: Vec<VariantSchema>,
    ) -> Result<Vec<Product>, DomainError> {
        if variant_schemas.is_empty() {
            return Ok(Vec::new());
        }

        let mut index_map: HashMap<String, usize> = HashMap::new();
        let mut products_domains: Vec<Product> = Vec::new();

        for variant_schema in variant_schemas {
            match index_map.get(&variant_schema.product.id) {
                Some(index) => {
                    let product = products_domains.get_mut(*index).unwrap();
                    let variant_domain = variant_schema.to_variant_domain()?;
                    let _ = product.add_variant(variant_domain);
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
                    let variant_domain = variant_schema.to_variant_domain()?;

                    let product_domain = Product::new(
                        ShopifyGQLQueryHelper::remove_gid_prefix(&product_id),
                        title,
                        description,
                        status,
                        vec![variant_domain],
                        category_id,
                    )?;

                    // keep indexMap with gid.
                    index_map.insert(product_id.clone(), products_domains.len());
                    products_domains.push(product_domain);
                }
            };
        }

        Ok(products_domains)
    }
}

#[derive(Debug, Deserialize)]
pub struct TaxonomyCategory {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductNode {
    pub id: String,

    pub category: Option<TaxonomyCategory>,

    pub title: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductsData {
    pub products: Edges<ProductNode>,
}

#[derive(Debug, Deserialize)]
pub struct VariantNode {
    pub id: String,

    pub product: ProductNode,

    pub barcode: Option<String>,
    #[serde(rename = "inventoryQuantity")]
    pub inventory_quantity: Option<i32>,
    pub sku: Option<String>,
    pub position: i32,
    pub price: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct VariantsData {
    #[serde(rename = "productVariants")]
    pub product_variants: Edges<VariantNode>,
}
