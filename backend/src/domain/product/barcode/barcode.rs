use derive_getters::Getters;

#[derive(Debug, Getters, Clone)]
pub struct Barcode {
    value: String,
}

impl Barcode {
    pub fn new(value: impl Into<String>) -> Barcode {
        Barcode {
            value: value.into(),
        }
    }
}
