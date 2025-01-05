use std::time::Duration;

use async_trait::async_trait;
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, TransactionTrait,
};

use crate::{
    domain::error::error::DomainError,
    infrastructure::{
        config::config::DatabaseConfig,
        db::transaction_manager_interface::TransactionManager,
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
};

pub struct SeaOrmConnectionProvider {
    conn: DatabaseConnection,
}

impl SeaOrmConnectionProvider {
    pub async fn new(config: DatabaseConfig) -> Result<Self, DomainError> {
        let mut opt = ConnectOptions::new(config.url());
        opt.max_connections(*config.max_connections())
            .min_connections(*config.min_connections())
            .connect_timeout(Duration::from_secs(*config.connect_timeout()))
            .acquire_timeout(Duration::from_secs(*config.acquire_timeout()))
            .idle_timeout(Duration::from_secs(*config.idle_timeout()))
            .max_lifetime(Duration::from_secs(*config.max_lifetime()));

        let conn = Database::connect(opt).await.map_err(|e| {
            InfrastructureErrorMapper::to_domain(InfrastructureError::DatabaseError(e))
        })?;

        conn.ping().await.map_err(|e| {
            InfrastructureErrorMapper::to_domain(InfrastructureError::DatabaseError(e))
        })?;

        Ok(Self { conn })
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.conn
    }
}

pub struct SeaOrmTransactionManager {
    conn: DatabaseConnection,
}

impl SeaOrmTransactionManager {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

pub struct SeaOrmTransaction {
    pub(crate) transaction: DatabaseTransaction,
}

#[async_trait]
impl TransactionManager for SeaOrmTransactionManager {
    type Transaction = SeaOrmTransaction;

    async fn begin(&self) -> Result<Self::Transaction, DomainError> {
        let transaction = self
            .conn
            .begin()
            .await
            .map_err(|_| DomainError::SystemError)?;
        Ok(SeaOrmTransaction { transaction })
    }

    async fn commit(&self, transaction: Self::Transaction) -> Result<(), DomainError> {
        transaction
            .transaction
            .commit()
            .await
            .map_err(|_| DomainError::SystemError)
    }

    async fn rollback(&self, transaction: Self::Transaction) -> Result<(), DomainError> {
        transaction
            .transaction
            .rollback()
            .await
            .map_err(|_| DomainError::SystemError)
    }
}
