use async_trait::async_trait;
use jsonwebtoken::{decode, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    domain::error::error::DomainError,
    infrastructure::{
        auth::authenticator_interface::Authenticator,
        config::config::CognitoConfig,
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct Jwks {
    keys: Vec<Key>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Key {
    kid: String,
    n: String,
    e: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    exp: usize,
    iat: usize,
    iss: String,
    sub: String,
}

#[derive(Clone)]
pub struct CognitoAuthenticator {
    config: CognitoConfig,
    http_client: Client,
    keys: Vec<Key>,
}

impl CognitoAuthenticator {
    pub fn new(config: CognitoConfig) -> Self {
        CognitoAuthenticator {
            config,
            http_client: Client::new(),
            keys: Vec::new(),
        }
    }

    fn get_issuer(&self) -> String {
        format!(
            "https://cognito-idp.{}.amazonaws.com/{}",
            self.config.region(),
            self.config.user_pool_id()
        )
    }

    async fn fetch_jwks(&self) -> Result<Jwks, DomainError> {
        let jwks = self
            .http_client
            .get(self.config.jwks_uri().as_str())
            .send()
            .await
            .map_err(|e| {
                log::error!("Failed to get jwks: {}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?
            .json::<Jwks>()
            .await
            .map_err(|e| {
                log::error!("Failed to parse get JWKs response: {}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?;

        Ok(jwks)
    }

    async fn get_jwks_key(&mut self, kid: &str) -> Result<Key, DomainError> {
        let cached_key = self.keys.clone().into_iter().find(|key| key.kid == kid);
        if let Some(key) = cached_key {
            log::debug!("Found cached key: {}", kid);
            return Ok(key);
        }

        let jwks = self.fetch_jwks().await?;
        self.keys = jwks.keys.clone();

        let key = jwks.keys.into_iter().find(|key| key.kid == kid);
        key.ok_or_else(|| {
            log::error!("Failed to find key: {}", kid);
            return DomainError::AuthenticationError;
        })
    }
}

#[async_trait]
impl Authenticator for CognitoAuthenticator {
    async fn validate_token(&mut self, token: String) -> Result<(), DomainError> {
        let header = jsonwebtoken::decode_header(&token).map_err(|e| {
            log::error!("Failed to decode header: {}", e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::JwtError(e))
        })?;

        // https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-verifying-a-jwt.html#amazon-cognito-user-pools-using-tokens-step-2
        let kid = header.kid.ok_or(DomainError::AuthenticationError)?;
        let key = self.get_jwks_key(&kid).await?;

        // https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-verifying-a-jwt.html#amazon-cognito-user-pools-using-tokens-step-3
        let validation = &mut Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_audience(&[self.config.client_id()]);
        validation.set_issuer(&[self.get_issuer().as_str()]);

        decode::<Claims>(
            &token,
            &DecodingKey::from_rsa_components(&key.n, &key.e).unwrap(),
            validation,
        )
        .map_err(|e| {
            log::error!("Failed to validate token: {}", e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::JwtError(e))
        })?;

        Ok(())
    }
}
