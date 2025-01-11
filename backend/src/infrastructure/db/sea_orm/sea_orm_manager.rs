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
            log::error!("Database connection error: {}", e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::DatabaseError(e))
        })?;

        conn.ping().await.map_err(|e| {
            log::error!("Database ping error: {}", e);
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
    tran: Option<DatabaseTransaction>,
}

impl SeaOrmTransactionManager {
    pub async fn new(conn: DatabaseConnection) -> Result<Self, DomainError> {
        let tran = conn.begin().await.map_err(|_| DomainError::SystemError)?;
        Ok(Self {
            conn,
            tran: Some(tran),
        })
    }
}

#[async_trait]
impl TransactionManager for SeaOrmTransactionManager {
    type Transaction = DatabaseTransaction;

    async fn get_transaction(&mut self) -> Result<&mut Self::Transaction, DomainError> {
        self.tran
            .as_mut()
            .ok_or(DomainError::SystemError)
            .map_err(|_| DomainError::SystemError)
    }

    async fn commit(&mut self) -> Result<(), DomainError> {
        self.tran
            .take()
            .ok_or(DomainError::SystemError)?
            .commit()
            .await
            .map_err(|_| DomainError::SystemError)?;
        Ok(())
    }

    async fn rollback(&mut self) -> Result<(), DomainError> {
        self.tran
            .take()
            .ok_or(DomainError::SystemError)?
            .rollback()
            .await
            .map_err(|_| DomainError::SystemError)?;
        Ok(())
    }
}
