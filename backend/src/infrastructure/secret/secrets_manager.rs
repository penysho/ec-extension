use aws_config::SdkConfig;
use aws_secretsmanager_caching::SecretsManagerCachingClient;
use std::num::NonZeroUsize;
use std::time::Duration;

use crate::domain::error::error::DomainError;
use crate::log_error;

/// SecretsManagerClient is a client for the AWS Secrets Manager.
pub struct SecretsManagerClient {
    client: SecretsManagerCachingClient,
}

impl SecretsManagerClient {
    /// new creates a new SecretsManagerClient.
    pub async fn new(sdk_config: &SdkConfig) -> Result<Self, DomainError> {
        let client = match SecretsManagerCachingClient::from_builder(
            aws_sdk_secretsmanager::config::Builder::from(sdk_config),
            NonZeroUsize::new(10).unwrap(),
            Duration::from_secs(60),
            false,
        )
        .await
        {
            Ok(c) => c,
            Err(_) => return Err(DomainError::SystemError),
        };
        Ok(Self { client })
    }
}

impl SecretsManagerClient {
    /// get_secret gets a secret from the AWS Secrets Manager.
    pub async fn get_secret(
        &self,
        secret_name: &str,
        refresh_now: bool,
    ) -> Result<String, DomainError> {
        match self
            .client
            .get_secret_value(secret_name, None, None, refresh_now)
            .await
        {
            Ok(s) => Ok(s.secret_string.unwrap()),
            Err(e) => {
                log_error!("Failed to get secret.", "Error" => e);
                Err(DomainError::SystemError)
            }
        }
    }

    /// get_secret_json gets a secret from the AWS Secrets Manager and parses it as a JSON object.
    pub async fn get_secret_json(
        &self,
        secret_name: &str,
        refresh_now: bool,
    ) -> Result<serde_json::Value, DomainError> {
        let secret_string = self.get_secret(secret_name, refresh_now).await?;
        let secret_json: serde_json::Value = serde_json::from_str(&secret_string).map_err(|e| {
            log_error!("Failed to parse secret string.", "Error" => e);
            DomainError::SystemError
        })?;
        Ok(secret_json)
    }
}
