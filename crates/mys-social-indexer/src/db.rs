// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing;
use tokio::time::Duration;

use crate::config::Config;

pub type DbPool = Pool<AsyncPgConnection>;
pub type DbConnection = Object<AsyncPgConnection>;

// Define migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Database wrapper for connection pool access
#[derive(Clone)]
pub struct Database {
    pub pool: Arc<DbPool>,
}

/// Query result types for SQL queries
pub mod query_types {
    use diesel::prelude::*;
    use diesel::sql_types::*;

    /// Result type for COUNT(*) queries
    #[derive(QueryableByName, Debug)]
    pub struct CountResult {
        #[diesel(sql_type = BigInt)]
        pub count: i64,
    }

    /// Result type for proposal type query
    #[derive(QueryableByName, Debug)]
    pub struct ProposalTypeResult {
        #[diesel(sql_type = Int2)]
        pub proposal_type: i16,
    }

    /// Result type for delegate vote query
    #[derive(QueryableByName, Debug)]
    pub struct DelegateVoteResult {
        #[diesel(sql_type = Text)]
        pub delegate_address: String,
        #[diesel(sql_type = Bool)]
        pub approve: bool,
        #[diesel(sql_type = Text)]
        pub submitter: String,
    }
}

impl Database {
    /// Create a new database instance
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
    
    /// Get a connection from the pool
    pub async fn get_connection(&self) -> Result<DbConnection> {
        self.pool.get().await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }
}

/// Sets up the database connection pool
pub async fn setup_connection_pool(config: &Config) -> Result<Arc<Database>> {
    tracing::info!("Setting up database connection pool");
    tracing::info!("Database URL: {}", &config.database.url);
    
    // Log environment info for debugging
    if let Ok(railway_env) = std::env::var("RAILWAY_ENVIRONMENT") {
        tracing::info!("Running on Railway environment: {}", railway_env);
    }
    
    // Create connection manager with proper SSL configuration
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.database.url);
    
    // Create the pool with configuration optimized for cloud deployments
    let pool = Pool::builder(manager)
        .max_size(config.database.max_connections as usize)
        .create_timeout(Some(Duration::from_secs(30))) // Increased timeout for cloud DBs
        .wait_timeout(Some(Duration::from_secs(30))) // Wait timeout for getting connections
        .recycle_timeout(Some(Duration::from_secs(30))) // Recycle timeout
        .build()
        .map_err(|e| anyhow!("Failed to create connection pool: {}", e))?;
    
    tracing::info!("Database connection pool created, testing connection...");
    
    // Test the connection with retry logic for Railway/cloud environments
    let mut last_error = None;
    for attempt in 1..=5 {
        tracing::info!("Connection attempt {} of 5", attempt);
        
        match tokio::time::timeout(Duration::from_secs(15), pool.get()).await {
            Ok(Ok(_conn)) => {
                // Connection successful - getting a connection from the pool validates database access
                tracing::info!("Database connection established successfully");
                return Ok(Arc::new(Database::new(pool)));
            },
            Ok(Err(e)) => {
                tracing::warn!("Database connection failed on attempt {}: {}", attempt, e);
                last_error = Some(anyhow!("Database connection failed: {}", e));
            },
            Err(_) => {
                tracing::warn!("Database connection timed out on attempt {}", attempt);
                last_error = Some(anyhow!("Database connection timed out"));
            }
        }
        
        if attempt < 5 {
            let wait_time = Duration::from_secs(2_u64.pow(attempt - 1)); // Exponential backoff: 1s, 2s, 4s, 8s
            tracing::info!("Waiting {:?} before retry...", wait_time);
            tokio::time::sleep(wait_time).await;
        }
    }
    
    tracing::error!("All database connection attempts failed");
    if let Some(err) = last_error {
        return Err(err);
    }
    
    Err(anyhow!("Failed to establish database connection after 5 attempts"))
}

/// Run database migrations
pub fn run_migrations(config: &Config) -> Result<()> {
    // Use a regular blocking connection for migrations
    let mut conn = PgConnection::establish(&config.database.url)
        .map_err(|e| anyhow!("Failed to establish migration connection (check TLS configuration): {}", e))?;
    
    tracing::info!("Running database migrations...");
    
    // Run migrations
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Migration error: {}", e))?;
    
    tracing::info!("Database migrations completed successfully");
    
    Ok(())
}