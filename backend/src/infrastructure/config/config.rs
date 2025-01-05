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

/// DatabaseConfig manages Database settings.
#[derive(Getters, Clone)]
pub struct DatabaseConfig {
    /// Set the database URL of the pool.
    url: String,
    /// Set the maximum number of connections of the pool.
    max_connections: u32,
    /// Set the minimum number of connections of the pool.
    min_connections: u32,
    /// Set the timeout duration when acquiring a connection.
    connect_timeout: u64,
    /// Set the maximum amount of time to spend waiting for acquiring a connection.
    acquire_timeout: u64,
    /// Set the idle duration before closing a connection.
    idle_timeout: u64,
    /// Set the maximum lifetime of individual connections.
    max_lifetime: u64,
}

impl DatabaseConfig {
    pub fn new() -> Result<Self, DomainError> {
        let url = env::var("DATABASE_URL").map_err(|_| {
            eprintln!("DATABASE_URL is not set as an environment variable");
            DomainError::InitConfigError
        })?;
        let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .map(|s| s.parse::<u32>().unwrap_or(10))
            .unwrap_or(10);
        let min_connections = env::var("DATABASE_MIN_CONNECTIONS")
            .map(|s| s.parse::<u32>().unwrap_or(5))
            .unwrap_or(5);
        let connect_timeout = env::var("DATABASE_CONNECT_TIMEOUT")
            .map(|s| s.parse::<u64>().unwrap_or(10))
            .unwrap_or(10);
        let acquire_timeout = env::var("DATABASE_ACQUIRE_TIMEOUT")
            .map(|s| s.parse::<u64>().unwrap_or(10))
            .unwrap_or(10);
        let idle_timeout = env::var("DATABASE_IDLE_TIMEOUT")
            .map(|s| s.parse::<u64>().unwrap_or(300))
            .unwrap_or(300);
        let max_lifetime = env::var("DATABASE_MAX_LIFETIME")
            .map(|s| s.parse::<u64>().unwrap_or(300))
            .unwrap_or(300);

        Ok(DatabaseConfig {
            url,
            max_connections,
            min_connections,
            connect_timeout,
            acquire_timeout,
            idle_timeout,
            max_lifetime,
        })
    }
}
