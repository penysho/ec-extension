use derive_getters::Getters;

pub type Id = String;

#[derive(Debug, Getters)]
pub struct Address {
    id: Id,
}
