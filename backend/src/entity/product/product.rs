use derive_getters::Getters;

/// Entity of Products.
#[derive(Debug, Getters)]
pub struct Product {
    id: String,
    name: String,
    price: u32,
    description: String,
}

impl Product {
    pub fn new(id: String, name: String, price: u32, description: String) -> Self {
        Product {
            id,
            name,
            price,
            description,
        }
    }
}
