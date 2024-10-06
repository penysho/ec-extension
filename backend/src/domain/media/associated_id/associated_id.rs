use crate::domain::product::product::Id as ProductId;

/// ID of the resource to which the media is tied.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum AssociatedId {
    Product(ProductId),
}
