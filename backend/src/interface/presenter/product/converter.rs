use crate::{
    domain::{
        media::media::Media,
        product::{
            product::{Product, ProductStatus},
            variant::variant::{InventoryPolicy, Variant},
        },
    },
    interface::presenter::media::schema::MediaSchema,
};

use super::schema::{InventoryPolicyEnum, ProductSchema, ProductStatusEnum, VariantSchema};

impl ProductSchema {
    pub(super) fn to_schema(product: Product, media: Vec<Media>) -> Self {
        ProductSchema {
            id: product.id().to_string(),
            name: product.name().to_string(),
            description: product.description().to_string(),
            status: product.status().to_owned().into(),
            category_id: product.category_id().to_owned(),
            media: media
                .into_iter()
                .map(|media| MediaSchema::from(media))
                .collect(),
            variants: product
                .variants()
                .iter()
                .map(|variant| VariantSchema::from(variant))
                .collect(),
        }
    }
}

impl From<ProductStatus> for ProductStatusEnum {
    fn from(status: ProductStatus) -> Self {
        match status {
            ProductStatus::Active => ProductStatusEnum::Active,
            ProductStatus::Inactive => ProductStatusEnum::Inactive,
            ProductStatus::Draft => ProductStatusEnum::Draft,
        }
    }
}

impl From<&Variant> for VariantSchema {
    fn from(variant: &Variant) -> Self {
        VariantSchema {
            id: variant.id().to_string(),
            name: variant.name().to_owned(),
            sku: variant.sku().as_ref().map(|sku| sku.value().to_owned()),
            barcode: variant
                .barcode()
                .as_ref()
                .map(|barcode| barcode.value().to_owned()),
            available_for_sale: *variant.available_for_sale(),
            list_order: *(variant.list_order()),
            inventory_item_id: variant.inventory_item_id().to_owned(),
            inventory_policy: variant.inventory_policy().to_owned().into(),
            inventory_quantity: *(variant.inventory_quantity()),
            price: *variant.price().value(),
            taxable: *variant.taxable(),
            tax_code: variant.tax_code().to_owned(),
            created_at: variant.created_at().to_owned(),
            updated_at: variant.updated_at().to_owned(),
        }
    }
}

impl From<InventoryPolicy> for InventoryPolicyEnum {
    fn from(policy: InventoryPolicy) -> Self {
        match policy {
            InventoryPolicy::Deny => InventoryPolicyEnum::Deny,
            InventoryPolicy::Continue => InventoryPolicyEnum::Continue,
        }
    }
}
