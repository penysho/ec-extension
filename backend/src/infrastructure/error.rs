use aws_sdk_cognitoidentityprovider::operation::initiate_auth::InitiateAuthError;
use derive_more::{Display, Error};

use crate::domain::error::error::DomainError;

#[derive(Debug, Display, Error)]
pub enum InfrastructureError {
    #[display(fmt = "Network is out of order.")]
    NetworkError(reqwest::Error),
    #[display(fmt = "Parse error.")]
    ParseError(serde_json::Error),
    #[display(fmt = "JWT error.")]
    JwtError(jsonwebtoken::errors::Error),
    #[display(fmt = "Cognito InitiateAuth execution failed.")]
    CognitoInitiateAuthError(aws_sdk_cognitoidentityprovider::error::SdkError<InitiateAuthError>),
}

pub struct InfrastructureErrorMapper;
impl InfrastructureErrorMapper {
    pub fn to_domain(error: InfrastructureError) -> DomainError {
        match error {
            InfrastructureError::NetworkError(_) => DomainError::SystemError,
            InfrastructureError::ParseError(_) => DomainError::SystemError,
            InfrastructureError::JwtError(_) => DomainError::AuthenticationError,
            InfrastructureError::CognitoInitiateAuthError(_) => DomainError::AuthenticationError,
        }
    }
}
