use crate::domain::product::product::Id as ProductId;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum AssociatedId {
    Product(ProductId),
}
