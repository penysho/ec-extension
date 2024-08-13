use derive_more::{Display, Error};

use crate::entity::error::error::DomainError;

#[derive(Debug, Display, Error)]
pub enum InfrastructureError {
    #[display(fmt = "Network is out of order.")]
    NetworkError(reqwest::Error),
    #[display(fmt = "Error returned in GraphQL Response.")]
    GraphQLResponseError,
    InitConfigError,
}

pub struct InfrastructureErrorMapper;
impl InfrastructureErrorMapper {
    pub fn to_domain(error: InfrastructureError) -> DomainError {
        match error {
            InfrastructureError::NetworkError(_) => DomainError::SystemError,
            InfrastructureError::GraphQLResponseError => DomainError::SystemError,
            InfrastructureError::InitConfigError => DomainError::SystemError,
        }
    }
}
