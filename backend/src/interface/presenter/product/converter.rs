use crate::domain::{
    media::{
        media::{Media, MediaStatus},
        media_content::media_content::MediaContent,
    },
    product::{
        product::{Product, ProductStatus},
        variant::variant::Variant,
    },
};

use super::schema::{
    MediaSchema, MediaStatusEnum, ProductSchema, ProductStatusEnum, VariantSchema,
};

impl From<Media> for MediaSchema {
    fn from(media: Media) -> Self {
        let image = match media.content() {
            Some(MediaContent::Image(image)) => Some(image),
            None => None,
        };

        MediaSchema {
            id: media.id().to_string(),
            name: media.name().to_owned(),
            status: match media.status() {
                MediaStatus::Active => MediaStatusEnum::Active,
                MediaStatus::Inactive => MediaStatusEnum::Inactive,
                MediaStatus::InPreparation => MediaStatusEnum::InPreparation,
            },
            alt: image.and_then(|image| image.alt().to_owned()),
            src: image
                .and_then(|image| image.published_src().to_owned())
                .map(|src| src.value().to_owned()),
            created_at: media.created_at().to_owned(),
            updated_at: media.updated_at().to_owned(),
        }
    }
}

impl From<&Variant> for VariantSchema {
    fn from(variant: &Variant) -> Self {
        VariantSchema {
            id: variant.id().to_string(),
            name: variant.name().to_owned(),
            price: *(variant.price()),
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
