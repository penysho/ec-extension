use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        money::amount::amount::Amount,
        product::{
            product::{Product, ProductStatus},
            variant::{
                barcode::barcode::Barcode,
                sku::sku::Sku,
                variant::{InventoryPolicy, Variant},
            },
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
        let inventory_policy = match self.inventory_policy.as_str() {
            "DENY" => Ok(InventoryPolicy::Deny),
            "CONTINUE" => Ok(InventoryPolicy::Continue),
            _ => Err(DomainError::ConversionError),
        }?;

        let price = Amount::new(self.price.parse::<f64>().unwrap_or(0.0))?;

        Variant::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            Some(self.title),
            sku,
            barcode,
            self.available_for_sale,
            self.position as u8,
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.inventory_item.id),
            inventory_policy,
            inventory_quantity,
            price,
            self.taxable,
            self.tax_code,
            self.created_at,
            self.updated_at,
        )
    }

    fn to_product_domain(self) -> Result<Product, DomainError> {
        let product_id = self.product.id.clone();
        let title = self.product.title.clone();
        let description = self.product.description.clone();
        let status = match self.product.status.as_str() {
            "ACTIVE" => Ok(ProductStatus::Active),
            "ARCHIVED" => Ok(ProductStatus::Inactive),
            "DRAFT" => Ok(ProductStatus::Draft),
            _ => Err(DomainError::ConversionError),
        }?;
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
#[serde(rename_all = "camelCase")]
pub struct VariantsData {
    pub product_variants: Edges<VariantNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantNode {
    pub id: String,
    pub title: String,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub available_for_sale: bool,
    pub position: i32,

    #[serde(rename = "inventoryItem")]
    pub inventory_item: InventoryItemIdNode,
    #[serde(rename = "inventoryQuantity")]
    pub inventory_quantity: Option<i32>,
    #[serde(rename = "inventoryPolicy")]
    pub inventory_policy: String,

    pub price: String, // Money objcets. Define with string because it is not MoneyV2.
    pub taxable: bool,
    pub tax_code: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub product: ProductNode,
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemIdNode {
    pub id: String,
}
