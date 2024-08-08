#[derive(Debug)]
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

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_price(&self) -> u32 {
        self.price
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
