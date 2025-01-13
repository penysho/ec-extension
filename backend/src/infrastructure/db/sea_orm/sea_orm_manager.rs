use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, TransactionTrait,
};
use tokio::sync::{Mutex, MutexGuard};

use crate::{
    domain::error::error::DomainError,
    infrastructure::{
        config::config::DatabaseConfig,
        db::transaction_manager_interface::TransactionManager,
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
};

pub struct SeaOrmConnectionProvider {
    conn: Arc<DatabaseConnection>,
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

        Ok(Self {
            conn: Arc::new(conn),
        })
    }

    pub fn get_connection(&self) -> Arc<DatabaseConnection> {
        self.conn.clone()
    }
}

#[derive(Clone)]
pub struct SeaOrmTransactionManager {
    conn: Arc<DatabaseConnection>,
    tran: Arc<Mutex<Option<DatabaseTransaction>>>,
}

impl SeaOrmTransactionManager {
    pub async fn new(conn: Arc<DatabaseConnection>) -> Result<Self, DomainError> {
        Ok(Self {
            conn,
            tran: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait]
impl TransactionManager<DatabaseTransaction> for SeaOrmTransactionManager {
    async fn begin(&self) -> Result<(), DomainError> {
        let mut lock = self.tran.lock().await;
        if lock.is_none() {
            let tran = self
                .conn
                .begin()
                .await
                .map_err(|_| DomainError::SystemError)?;
            *lock = Some(tran);
            Ok(())
        } else {
            Err(DomainError::SystemError)
        }
    }

    async fn get_transaction(
        &self,
    ) -> Result<MutexGuard<'_, Option<DatabaseTransaction>>, DomainError> {
        let lock = self.tran.lock().await;
        if lock.is_some() {
            Ok(lock)
        } else {
            Err(DomainError::SystemError)
        }
    }

    async fn commit(&self) -> Result<(), DomainError> {
        let mut lock = self.tran.lock().await;
        if let Some(tran) = lock.take() {
            tran.commit().await.map_err(|_| DomainError::SystemError)?;
            Ok(())
        } else {
            Err(DomainError::SystemError)
        }
    }

    async fn rollback(&self) -> Result<(), DomainError> {
        let mut lock = self.tran.lock().await;
        if let Some(tran) = lock.take() {
            tran.rollback()
                .await
                .map_err(|_| DomainError::SystemError)?;
            Ok(())
        } else {
            Err(DomainError::SystemError)
        }
    }
}
