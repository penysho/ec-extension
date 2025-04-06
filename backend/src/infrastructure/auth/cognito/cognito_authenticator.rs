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
        auth::{authenticator_interface::Authenticator, idp_user::IdpUser},
        config::config::CognitoConfig,
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
    log_debug, log_error, log_warn,
};

/// Authenticator with Cognito wrap.
#[derive(Clone)]
pub struct CognitoAuthenticator {
    config: CognitoConfig,
    http_client: ReqwestClient,
    cognito_client: CognitoClient,
    keys: Arc<RwLock<Vec<Key>>>,
}

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
                log_error!("Failed to fetch JWKs."; "error" => %e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::NetworkError(e))
            })?
            .json::<Jwks>()
            .await
            .map_err(|e| {
                log_error!("Failed to parse fetch JWKs response."; "error" => %e);
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
            log_debug!("Found cached key: {}", kid);
            return Ok(key);
        }

        log_warn!(
            "Since the cache is not found, it is retrieved from JWKS_URI. kid: {}",
            kid
        );
        let jwks = self.fetch_jwks().await?;
        let mut keys = self.keys.write().await;
        *keys = jwks.keys.clone();

        let key = jwks.keys.into_iter().find(|key| key.kid == kid);
        key.ok_or_else(|| {
            log_error!("Failed to find key."; "kid" => kid);
            return DomainError::AuthenticationError;
        })
    }

    fn verify_id_token(
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
                log_warn!("ID Token is expired");
                return DomainError::AuthenticationExpired;
            }

            log_error!("Failed to validate ID Token."; "error" => %e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::JwtError(e))
        })?;

        if decoded_token.claims.token_use != "id" {
            log_error!(
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
    async fn verify_token(
        &mut self,
        id_token: Option<&str>,
        refresh_token: Option<&str>,
    ) -> Result<(IdpUser, String), DomainError> {
        if id_token.is_none() && refresh_token.is_none() {
            log_error!("Neither the ID token nor the refresh token is present in the cookie.");
            return Err(DomainError::AuthenticationError);
        };

        let id_token_value = match id_token {
            Some(token) => token.to_string(),
            None => {
                self.get_id_token_by_refresh_token(refresh_token.unwrap())
                    .await?
            }
        };

        let header = jsonwebtoken::decode_header(&id_token_value).map_err(|e| {
            log_error!("Failed to decode header."; "error" => %e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::JwtError(e))
        })?;

        // https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-verifying-a-jwt.html#amazon-cognito-user-pools-using-tokens-step-2
        let kid = header.kid.ok_or(DomainError::AuthenticationError)?;
        let key = self.get_jwks_key(&kid).await?;

        match self.verify_id_token(&id_token_value, &key) {
            Ok(token_data) => {
                return Ok((
                    IdpUser {
                        id: token_data.claims.sub.clone(),
                        email: token_data.claims.email.clone(),
                    },
                    id_token_value.to_string(),
                ));
            }
            Err(e) if e == DomainError::AuthenticationExpired && refresh_token.is_some() => {
                log_debug!("ID Token is expired. Attempting to refresh the ID Token.");

                let new_id_token_value = self
                    .get_id_token_by_refresh_token(refresh_token.unwrap())
                    .await?;

                self.verify_id_token(&new_id_token_value, &key)
                    .map(|token_data| {
                        (
                            IdpUser {
                                id: token_data.claims.sub.clone(),
                                email: token_data.claims.email.clone(),
                            },
                            new_id_token_value,
                        )
                    })
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    async fn get_id_token_by_refresh_token(
        &self,
        refresh_token: &str,
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
                log_error!("Failed to get ID Token by refresh token."; "error" => %e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::CognitoInitiateAuthError(
                    e,
                ))
            })?
            .authentication_result
            .ok_or_else(|| {
                log_error!("Failed to get authentication result");
                DomainError::SystemError
            })?;

        let id_token = result
            .id_token()
            .ok_or_else(|| {
                log_error!("Failed to get ID token");
                DomainError::SystemError
            })?
            .to_string();

        Ok(id_token)
    }
}
