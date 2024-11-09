use crate::{
    domain::{
        media::media::Media,
        product::{
            product::{Product, ProductStatus},
            variant::variant::Variant,
        },
    },
    interface::presenter::media::schema::MediaSchema,
};

use super::schema::{ProductSchema, ProductStatusEnum, VariantSchema};

impl From<&Variant> for VariantSchema {
    fn from(variant: &Variant) -> Self {
        VariantSchema {
            id: variant.id().to_string(),
            name: variant.name().to_owned(),
            price: *variant.price().value() as u32,
            sku: variant.sku().as_ref().map(|sku| sku.value().to_owned()),
            barcode: variant
                .barcode()
                .as_ref()
                .map(|barcode| barcode.value().to_owned()),
            inventory_quantity: *(variant.inventory_quantity()),
            list_order: *(variant.list_order()),
            created_at: variant.created_at().to_owned(),
            updated_at: variant.updated_at().to_owned(),
        }
    }
}

impl ProductSchema {
    pub(super) fn to_schema(product: Product, media: Vec<Media>) -> Self {
        ProductSchema {
            id: product.id().to_string(),
            name: product.name().to_string(),
            description: product.description().to_string(),
            status: match product.status() {
                ProductStatus::Active => ProductStatusEnum::Active,
                ProductStatus::Inactive => ProductStatusEnum::Inactive,
                ProductStatus::Draft => ProductStatusEnum::Draft,
            },
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
