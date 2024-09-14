use derive_more::{Display, Error};

use crate::domain::error::error::DomainError;

#[derive(Debug, Display, Error)]
pub enum InfrastructureError {
    #[display(fmt = "Network is out of order.")]
    NetworkError(reqwest::Error),
    #[display(fmt = "Parse error.")]
    ParseError(serde_json::Error),
}

pub struct InfrastructureErrorMapper;
impl InfrastructureErrorMapper {
    pub fn to_domain(error: InfrastructureError) -> DomainError {
        match error {
            InfrastructureError::NetworkError(_) => DomainError::SystemError,
            InfrastructureError::ParseError(_) => DomainError::SystemError,
        }
    }
}
