use std::sync::Arc;

use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_cognitoidentityprovider::{types::AuthFlowType, Client as CognitoClient};
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, TokenData, Validation};
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    domain::error::error::DomainError,
    infrastructure::{
        auth::authenticator_interface::{Authenticator, IdpUser},
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
    token_use: String,
    email: String,
}

#[derive(Clone)]
pub struct CognitoAuthenticator {
    config: CognitoConfig,
    http_client: ReqwestClient,
    cognito_client: CognitoClient,
    keys: Arc<RwLock<Vec<Key>>>,
}

impl CognitoAuthenticator {
    pub fn new(cognito_config: CognitoConfig, sdk_config: SdkConfig) -> Self {
        CognitoAuthenticator {
            config: cognito_config,
            http_client: ReqwestClient::new(),
            cognito_client: CognitoClient::new(&sdk_config),
            keys: Arc::new(RwLock::new(Vec::new())),
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
                log::error!("Failed to fetch JWKs: {}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?
            .json::<Jwks>()
            .await
            .map_err(|e| {
                log::error!("Failed to parse fetch JWKs response: {}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?;

        Ok(jwks)
    }

    async fn get_jwks_key(&mut self, kid: &str) -> Result<Key, DomainError> {
        let cached_key = self
            .keys
            .read()
            .await
            .clone()
            .into_iter()
            .find(|key| key.kid == kid);
        if let Some(key) = cached_key {
            log::debug!("Found cached key: {}", kid);
            return Ok(key);
        }

        log::warn!(
            "Since the cache is not found, it is retrieved from JWKS_URI. kid: {}",
            kid
        );
        let jwks = self.fetch_jwks().await?;
        let mut keys = self.keys.write().await;
        *keys = jwks.keys.clone();

        let key = jwks.keys.into_iter().find(|key| key.kid == kid);
        key.ok_or_else(|| {
            log::error!("Failed to find key: {}", kid);
            return DomainError::AuthenticationError;
        })
    }

    fn validate_id_token(
        &self,
        id_token_value: &str,
        key: &Key,
    ) -> Result<TokenData<Claims>, DomainError> {
        // https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-verifying-a-jwt.html#amazon-cognito-user-pools-using-tokens-step-3
        let validation = &mut Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_audience(&[self.config.client_id()]);
        validation.set_issuer(&[self.get_issuer().as_str()]);

        let decoded_token = decode::<Claims>(
            id_token_value,
            &DecodingKey::from_rsa_components(&key.n, &key.e).unwrap(),
            validation,
        )
        .map_err(|e| {
            if e.kind().to_owned() == ErrorKind::ExpiredSignature {
                log::warn!("ID Token is expired");
                return DomainError::AuthenticationExpired;
            }

            log::error!("Failed to validate ID Token: {:?}", e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::JwtError(e))
        })?;

        if decoded_token.claims.token_use != "id" {
            log::error!(
                "Token is not ID Token. token_use: {}",
                decoded_token.claims.token_use
            );
            return Err(DomainError::AuthenticationError);
        }

        Ok(decoded_token)
    }
}

#[async_trait]
impl Authenticator for CognitoAuthenticator {
    async fn validate_token(
        &mut self,
        id_token: Option<String>,
        refresh_token: Option<String>,
    ) -> Result<(IdpUser, String), DomainError> {
        if id_token.is_none() && refresh_token.is_none() {
            log::error!("Neither the ID token nor the refresh token is present in the cookie.");
            return Err(DomainError::AuthenticationError);
        };

        let id_token_value = match id_token {
            Some(token) => token,
            None => {
                self.get_id_token_by_refresh_token(refresh_token.clone().unwrap())
                    .await?
            }
        };

        let header = jsonwebtoken::decode_header(&id_token_value).map_err(|e| {
            log::error!("Failed to decode header: {}", e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::JwtError(e))
        })?;

        // https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-verifying-a-jwt.html#amazon-cognito-user-pools-using-tokens-step-2
        let kid = header.kid.ok_or(DomainError::AuthenticationError)?;
        let key = self.get_jwks_key(&kid).await?;

        match self.validate_id_token(&id_token_value, &key) {
            Ok(token_data) => {
                return Ok((
                    IdpUser {
                        id: token_data.claims.sub.clone(),
                        email: token_data.claims.email.clone(),
                    },
                    id_token_value,
                ));
            }
            Err(e) => {
                if e == DomainError::AuthenticationExpired && refresh_token.is_some() {
                    log::debug!("ID Token is expired. Attempting to refresh the ID Token.");

                    let new_id_token_value = self
                        .get_id_token_by_refresh_token(refresh_token.clone().unwrap())
                        .await?;

                    match self.validate_id_token(&new_id_token_value, &key) {
                        Ok(token_data) => {
                            return Ok((
                                IdpUser {
                                    id: token_data.claims.sub.clone(),
                                    email: token_data.claims.email.clone(),
                                },
                                id_token_value,
                            ));
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }

                return Err(e);
            }
        }
    }

    async fn get_id_token_by_refresh_token(
        &self,
        refresh_token: String,
    ) -> Result<String, DomainError> {
        // https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-the-refresh-token.html#amazon-cognito-user-pools-using-the-refresh-token_initiate-token
        let result = self
            .cognito_client
            .initiate_auth()
            .client_id(self.config.client_id().to_string())
            .auth_flow(AuthFlowType::RefreshTokenAuth)
            .auth_parameters("REFRESH_TOKEN", refresh_token)
            .send()
            .await
            .map_err(|e| {
                log::error!("Failed to get ID Token by refresh token: {:?}", e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::CognitoInitiateAuthError(
                    e,
                ))
            })?
            .authentication_result
            .ok_or_else(|| {
                log::error!("Failed to get authentication result");
                DomainError::SystemError
            })?;

        let id_token = result
            .id_token()
            .ok_or_else(|| {
                log::error!("Failed to get ID token");
                DomainError::SystemError
            })?
            .to_string();

        Ok(id_token)
    }
}
