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

impl VariantNode {
    fn to_variant_domain(self) -> Result<Variant, DomainError> {
        let sku = self.sku.map(Sku::new).transpose()?;
        let barcode = self.barcode.map(Barcode::new).transpose()?;
        let inventory_quantity = self.inventory_quantity.map(|qty| qty as u32);

        Variant::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            None::<String>,
            self.price.parse::<f32>().unwrap_or(0.0) as u32,
            sku,
            barcode,
            inventory_quantity,
            self.position as u8,
            self.created_at,
            self.updated_at,
        )
    }

    fn to_product_domain(self) -> Result<Product, DomainError> {
        let product_id = self.product.id.clone();
        let title = self.product.title.clone();
        let description = self.product.description.clone();
        let status = match self.product.status.as_str() {
            "ACTIVE" => ProductStatus::Active,
            "ARCHIVED" => ProductStatus::Inactive,
            "DRAFT" => ProductStatus::Draft,
            _ => ProductStatus::Inactive,
        };
        let category_id = self.product.category.as_ref().map(|c| c.id.clone());
        let variant_domain = self.to_variant_domain()?;

        Product::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&product_id),
            title,
            description,
            status,
            vec![variant_domain],
            category_id,
        )
    }

    pub fn to_product_domains(variant_schemas: Vec<Self>) -> Result<Vec<Product>, DomainError> {
        if variant_schemas.is_empty() {
            return Ok(Vec::new());
        }

        let mut index_map: HashMap<String, usize> = HashMap::new();
        let mut products_domains: Vec<Product> = Vec::new();
        for variant_schema in variant_schemas {
            match index_map.get(&ShopifyGQLQueryHelper::remove_gid_prefix(
                &variant_schema.product.id,
            )) {
                Some(index) => {
                    let product = products_domains.get_mut(*index).unwrap();
                    let variant_domain = variant_schema.to_variant_domain()?;
                    let _ = product.add_variant(variant_domain);
                }
                None => {
                    variant_schema.to_product_domain().map(|product_domain| {
                        let product_id = product_domain.id().clone();
                        // keep indexMap with gid.
                        index_map.insert(product_id, products_domains.len());
                        products_domains.push(product_domain);
                    })?;
                }
            };
        }

        Ok(products_domains)
    }
}

#[derive(Debug, Deserialize)]
pub struct ProductsData {
    pub products: Edges<ProductNode>,
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
pub struct TaxonomyCategory {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct VariantsData {
    #[serde(rename = "productVariants")]
    pub product_variants: Edges<VariantNode>,
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
