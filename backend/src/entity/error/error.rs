use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub struct DomainError {
    message: String,
}

impl DomainError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}


