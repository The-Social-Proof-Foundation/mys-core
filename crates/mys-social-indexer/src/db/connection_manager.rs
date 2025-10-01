// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, error, warn};

use crate::db::{Database, DbConnection};

/// Connection retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        }
    }
}

/// Transaction isolation levels
#[derive(Debug, Clone, Copy)]
pub enum IsolationLevel {
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

/// Transaction configuration
#[derive(Debug, Clone)]
pub struct TransactionConfig {
    pub isolation_level: IsolationLevel,
    pub timeout: Duration,
    pub retry_on_conflict: bool,
}

impl Default for TransactionConfig {
    fn default() -> Self {
        Self {
            isolation_level: IsolationLevel::ReadCommitted,
            timeout: Duration::from_secs(30),
            retry_on_conflict: true,
        }
    }
}

/// Standardized database connection manager with retry logic and error handling
pub struct ConnectionManager {
    db: Arc<Database>,
    retry_config: RetryConfig,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            retry_config: RetryConfig::default(),
        }
    }

    /// Create a connection manager with custom retry configuration
    pub fn with_retry_config(db: Arc<Database>, retry_config: RetryConfig) -> Self {
        Self { db, retry_config }
    }

    /// Get a database connection with retry logic
    pub async fn get_connection(&self) -> Result<DbConnection> {
        self.get_connection_with_timeout(Duration::from_secs(10))
            .await
    }

    /// Get a database connection with custom timeout
    pub async fn get_connection_with_timeout(
        &self,
        connection_timeout: Duration,
    ) -> Result<DbConnection> {
        let mut delay = self.retry_config.initial_delay;
        let mut last_error = None;

        for attempt in 0..=self.retry_config.max_retries {
            if attempt > 0 {
                debug!("Retrying database connection (attempt {})", attempt + 1);
                tokio::time::sleep(delay).await;

                // Exponential backoff
                delay = Duration::from_millis(
                    ((delay.as_millis() as f64 * self.retry_config.backoff_multiplier) as u64)
                        .min(self.retry_config.max_delay.as_millis() as u64),
                );
            }

            match timeout(connection_timeout, self.db.get_connection()).await {
                Ok(Ok(conn)) => {
                    if attempt > 0 {
                        debug!("Database connection established after {} retries", attempt);
                    }
                    return Ok(conn);
                }
                Ok(Err(e)) => {
                    last_error = Some(anyhow!("Database connection failed: {}", e));
                    warn!("Database connection attempt {} failed: {}", attempt + 1, e);
                }
                Err(_) => {
                    last_error = Some(anyhow!(
                        "Database connection timed out after {:?}",
                        connection_timeout
                    ));
                    warn!("Database connection attempt {} timed out", attempt + 1);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            anyhow!(
                "Failed to get database connection after {} attempts",
                self.retry_config.max_retries + 1
            )
        }))
    }

    /// Execute a function with a database connection, with automatic retry on connection failure
    pub async fn with_connection<F, T>(&self, f: F) -> Result<T>
    where
        F: Fn(&mut DbConnection) -> futures::future::BoxFuture<'_, Result<T>> + Send + Sync,
        T: Send,
    {
        let mut conn = self.get_connection().await?;
        f(&mut conn).await
    }

    /// Execute a function within a database transaction
    pub async fn with_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: Fn(&mut DbConnection) -> futures::future::BoxFuture<'_, Result<T>> + Send + Sync,
        T: Send,
    {
        self.with_transaction_config(f, TransactionConfig::default())
            .await
    }

    /// Execute a function within a database transaction with custom configuration
    pub async fn with_transaction_config<F, T>(&self, f: F, config: TransactionConfig) -> Result<T>
    where
        F: Fn(&mut DbConnection) -> futures::future::BoxFuture<'_, Result<T>> + Send + Sync,
        T: Send,
    {
        let mut conn = self.get_connection().await?;

        // Set isolation level
        match config.isolation_level {
            IsolationLevel::ReadCommitted => {
                diesel::sql_query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
                    .execute(&mut conn)
                    .await
                    .map_err(|e| anyhow!("Failed to set isolation level: {}", e))?;
            }
            IsolationLevel::RepeatableRead => {
                diesel::sql_query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
                    .execute(&mut conn)
                    .await
                    .map_err(|e| anyhow!("Failed to set isolation level: {}", e))?;
            }
            IsolationLevel::Serializable => {
                diesel::sql_query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
                    .execute(&mut conn)
                    .await
                    .map_err(|e| anyhow!("Failed to set isolation level: {}", e))?;
            }
        }

        // Begin transaction
        diesel::sql_query("BEGIN")
            .execute(&mut conn)
            .await
            .map_err(|e| anyhow!("Failed to begin transaction: {}", e))?;

        // Execute function with timeout
        let result = timeout(config.timeout, f(&mut conn)).await;

        match result {
            Ok(Ok(value)) => {
                // Commit transaction
                diesel::sql_query("COMMIT")
                    .execute(&mut conn)
                    .await
                    .map_err(|e| anyhow!("Failed to commit transaction: {}", e))?;
                Ok(value)
            }
            Ok(Err(e)) => {
                // Rollback on function error
                if let Err(rollback_err) = diesel::sql_query("ROLLBACK").execute(&mut conn).await {
                    error!("Failed to rollback transaction: {}", rollback_err);
                }
                Err(e)
            }
            Err(_) => {
                // Rollback on timeout
                if let Err(rollback_err) = diesel::sql_query("ROLLBACK").execute(&mut conn).await {
                    error!(
                        "Failed to rollback transaction after timeout: {}",
                        rollback_err
                    );
                }
                Err(anyhow!("Transaction timed out after {:?}", config.timeout))
            }
        }
    }

    /// Check database connectivity
    pub async fn health_check(&self) -> Result<()> {
        let mut conn = self
            .get_connection_with_timeout(Duration::from_secs(5))
            .await?;

        // Simple query to check connectivity
        diesel::sql_query("SELECT 1")
            .execute(&mut conn)
            .await
            .map_err(|e| anyhow!("Database health check failed: {}", e))?;

        Ok(())
    }

    /// Get connection pool statistics (if available)
    pub fn pool_stats(&self) -> Option<String> {
        // This would depend on the specific connection pool implementation
        // For now, return a placeholder
        Some("Pool stats not implemented yet".to_string())
    }
}

/// Helper trait for structs that need database access
pub trait DatabaseAccess {
    fn get_connection_manager(&self) -> &ConnectionManager;

    /// Convenience method to get a connection
    fn get_connection(&self) -> impl std::future::Future<Output = Result<DbConnection>> + Send
    where
        Self: Sync,
    {
        async move { self.get_connection_manager().get_connection().await }
    }

    /// Convenience method to execute with connection
    fn with_connection<F, T>(&self, f: F) -> impl std::future::Future<Output = Result<T>> + Send
    where
        F: Fn(&mut DbConnection) -> futures::future::BoxFuture<'_, Result<T>> + Send + Sync,
        T: Send,
        Self: Sync,
    {
        async move { self.get_connection_manager().with_connection(f).await }
    }

    /// Convenience method to execute with transaction
    fn with_transaction<F, T>(&self, f: F) -> impl std::future::Future<Output = Result<T>> + Send
    where
        F: Fn(&mut DbConnection) -> futures::future::BoxFuture<'_, Result<T>> + Send + Sync,
        T: Send,
        Self: Sync,
    {
        async move { self.get_connection_manager().with_transaction(f).await }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // Note: These tests would require a test database setup
    // They are provided as examples of how to test the connection manager

    #[tokio::test]
    #[ignore] // Ignore by default since it requires a database
    async fn test_connection_retry() {
        // This test would require setting up a test database
        // and simulating connection failures
    }

    #[tokio::test]
    #[ignore] // Ignore by default since it requires a database
    async fn test_transaction_rollback() {
        // This test would verify that transactions are properly rolled back
        // on errors
    }
}
