use derive_getters::Getters;

#[derive(Debug, Getters, Clone)]
pub struct Sku {
    value: String,
}

impl Sku {
    pub fn new(value: impl Into<String>) -> Sku {
        Sku {
            value: value.into(),
        }
    }
}
