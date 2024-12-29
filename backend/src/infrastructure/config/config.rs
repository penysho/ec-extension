use derive_getters::Getters;
use std::env;

use crate::domain::error::error::DomainError;

/// AppConfig manages application settings.
#[derive(Getters, Clone)]
pub struct AppConfig {
    address: String,
    port: String,
    log_level: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, DomainError> {
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());
        if !matches!(
            log_level.as_str(),
            "error" | "warn" | "info" | "debug" | "trace" | "off"
        ) {
            eprintln!(
                "An invalid value has been set for LOG_LEVEL.
                    Set one of ERROR, WARN, INFO, DEBUG, TRACE, or OFF. LOG_LEVEL= {}",
                log_level
            );
            return Err(DomainError::InitConfigError);
        }

        let port = env::var("APP_PORT").unwrap_or_else(|_| "8011".to_string());
        let address = env::var("APP_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

        Ok(AppConfig {
            address,
            port,
            log_level,
        })
    }
}

/// ShopifyConfig manages Shopify settings.
#[derive(Getters, Clone)]
pub struct ShopifyConfig {
    store_url: String,
    access_token: String,
}

impl ShopifyConfig {
    pub fn new() -> Result<Self, DomainError> {
        let store_url = env::var("STORE_URL").map_err(|_| {
            eprintln!("STORE_URL is not set as an environment variable");
            DomainError::InitConfigError
        })?;
        let access_token = env::var("ACCESS_TOKEN").map_err(|_| {
            eprintln!("ACCESS_TOKEN is not set as an environment variable");
            DomainError::InitConfigError
        })?;
        Ok(ShopifyConfig {
            store_url,
            access_token,
        })
    }
}

/// CognitoConfig manages Cognito settings.
#[derive(Getters, Clone)]
pub struct CognitoConfig {
    user_pool_id: String,
    client_id: String,
    region: String,
    jwks_uri: String,
}

impl CognitoConfig {
    pub fn new() -> Result<Self, DomainError> {
        let user_pool_id = env::var("COGNITO_USER_POOL_ID").map_err(|_| {
            eprintln!("COGNITO_USER_POOL_ID is not set as an environment variable");
            DomainError::InitConfigError
        })?;
        let client_id = env::var("COGNITO_CLIENT_ID").map_err(|_| {
            eprintln!("COGNITO_CLIENT_ID is not set as an environment variable");
            DomainError::InitConfigError
        })?;
        let region = env::var("COGNITO_REGION").map_err(|_| {
            eprintln!("COGNITO_REGION is not set as an environment variable");
            DomainError::InitConfigError
        })?;
        let jwks_uri = env::var("COGNITO_JWKS_URI").map_err(|_| {
            eprintln!("COGNITO_JWKS_URI is not set as an environment variable");
            DomainError::InitConfigError
        })?;

        Ok(CognitoConfig {
            user_pool_id,
            client_id,
            region,
            jwks_uri,
        })
    }
}
