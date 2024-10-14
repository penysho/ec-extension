use crate::domain::{customer::customer::Id as CustomerId, product::product::Id as ProductId};

/// ID of the resource to which the media is tied.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum AssociatedId {
    Product(ProductId),
    Customer(CustomerId),
}
