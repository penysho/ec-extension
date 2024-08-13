use derive_getters::Getters;
use std::env;

use crate::infrastructure::error::InfrastructureError;

#[derive(Getters)]
pub struct AppConfig {
    log_level: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, InfrastructureError> {
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
            return Err(InfrastructureError::InitConfigError);
        }

        Ok(AppConfig {
            log_level: log_level,
        })
    }
}

#[derive(Getters, Clone)]
pub struct ShopifyConfig {
    store_url: String,
    access_token: String,
}

impl ShopifyConfig {
    pub fn new() -> Result<Self, InfrastructureError> {
        let store_url = env::var("STORE_URL").map_err(|_| {
            eprintln!("STORE_URL is not set as an environment variable");
            InfrastructureError::InitConfigError
        })?;
        let access_token = env::var("ACCESS_TOKEN").map_err(|_| {
            eprintln!("ACCESS_TOKEN is not set as an environment variable");
            InfrastructureError::InitConfigError
        })?;
        Ok(ShopifyConfig {
            store_url: store_url,
            access_token: access_token,
        })
    }
}
