use crate::domain::{
    media::media::{Media, MediaStatus},
    product::{
        product::{Product, ProductStatus},
        variant::variant::Variant,
    },
};

use super::schema::{
    MediaSchema, MediaStatusEnum, ProductSchema, ProductStatusEnum, VariantSchema,
};

impl ProductSchema {
    pub fn to_schema(product: Product, media: Vec<Media>) -> Self {
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

impl From<Media> for MediaSchema {
    fn from(media: Media) -> Self {
        MediaSchema {
            id: media.id().to_string(),
            status: match media.status() {
                MediaStatus::Active => MediaStatusEnum::Active,
                MediaStatus::Inactive => MediaStatusEnum::Inactive,
                MediaStatus::InPreparation => MediaStatusEnum::InPreparation,
            },
            alt: media.alt().to_owned(),
            src: media.published_src().as_ref().map(|s| s.value().to_owned()),
            created_at: media.created_at().to_owned(),
            updated_at: media.updated_at().to_owned(),
        }
    }
}

impl From<&Variant> for VariantSchema {
    fn from(variant: &Variant) -> Self {
        VariantSchema {
            id: variant.id().to_string(),
            price: *(variant.price()),
            sku: variant.sku().as_ref().map(|sku| sku.value().to_owned()),
            barcode: variant
                .barcode()
                .as_ref()
                .map(|barcode| barcode.value().to_owned()),
            inventory_quantity: variant.inventory_quantity().to_owned(),
            list_order: variant.list_order().to_owned(),
            created_at: variant.created_at().to_owned(),
            updated_at: variant.updated_at().to_owned(),
        }
    }
}
