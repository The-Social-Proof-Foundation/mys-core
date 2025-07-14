// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel_async::{AsyncPgConnection, AsyncConnection};
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing;
use tokio::time::Duration;
use rustls_native_certs::load_native_certs;
use tokio_postgres_rustls::MakeRustlsConnect;
use rustls;

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

/// Create a TLS-enabled connection using rustls
async fn establish_tls_connection(database_url: &str) -> Result<AsyncPgConnection> {
    tracing::info!("Setting up TLS connection with rustls");
    
    // Load native root certificates - this returns an iterator that we can iterate over
    let certs = load_native_certs()
        .expect("Failed to load native root certificates");
    
    tracing::info!("Loaded {} native root certificates", certs.len());
    
    // Create rustls config with native root certificates
    let mut root_store = rustls::RootCertStore::empty();
    for cert in certs {
        root_store.add(cert).unwrap();
    }
    
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    
    // Create TLS connector with rustls config
    let tls = MakeRustlsConnect::new(config);
    tracing::info!("Rustls TLS connector configured with native certificates");
    
    // Test the TLS connection using tokio-postgres directly
    let (_client, connection) = tokio_postgres::connect(database_url, tls).await
        .map_err(|e| anyhow!("Failed to establish TLS connection: {}", e))?;
    
    // Spawn the connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            tracing::error!("PostgreSQL connection error: {}", e);
        }
    });
    
    tracing::info!("TLS PostgreSQL connection test successful! 🔒");
    
    // Now use diesel-async which should work with SSL/TLS properly configured
    AsyncPgConnection::establish(database_url).await
        .map_err(|e| anyhow!("Failed to establish diesel-async connection: {}", e))
}

/// Sets up the database connection pool
pub async fn setup_connection_pool(config: &Config) -> Result<Arc<Database>> {
    tracing::info!("Setting up database connection pool with rustls TLS support");
    tracing::info!("Database URL: {}", mask_database_url(&config.database.url));
    
    // Log environment info for debugging
    if let Ok(railway_env) = std::env::var("RAILWAY_ENVIRONMENT") {
        tracing::info!("Running on Railway environment: {}", railway_env);
    }
    
    // Validate that the DATABASE_URL has sslmode=require for Railway
    if config.database.url.contains("railway.app") || config.database.url.contains("tsdb.cloud.timescale.com") {
        if !config.database.url.contains("sslmode=require") {
            tracing::warn!("Railway/TimescaleDB PostgreSQL requires SSL - DATABASE_URL should include ?sslmode=require");
        } else {
            tracing::info!("SSL mode detected in DATABASE_URL: sslmode=require");
        }
    }
    
    // Test TLS connection first
    tracing::info!("Testing TLS connection with rustls...");
    match establish_tls_connection(&config.database.url).await {
        Ok(_) => {
            tracing::info!("TLS connection test successful!");
        }
        Err(e) => {
            tracing::error!("TLS connection test failed: {}", e);
            return Err(e);
        }
    }
    
    // Create connection manager - diesel-async should now work with the TLS setup
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.database.url);
    
    // Create the pool with configuration optimized for cloud deployments
    let pool = Pool::builder(manager)
        .max_size(config.database.max_connections as usize)
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
                tracing::info!("Database connection established successfully!");
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
    tracing::info!("Running database migrations...");
    
    // Log database URL for debugging (mask sensitive parts)
    let masked_url = mask_database_url(&config.database.url);
    tracing::info!("Migration connection URL (masked): {}", masked_url);
    
    // Check if this looks like a Railway database URL
    if config.database.url.contains("railway.app") {
        tracing::info!("Detected Railway PostgreSQL - ensuring SSL configuration");
    }
    
    // Validate DATABASE_URL format
    if !config.database.url.starts_with("postgres://") && !config.database.url.starts_with("postgresql://") {
        return Err(anyhow::anyhow!("Invalid DATABASE_URL format. Must start with postgres:// or postgresql://"));
    }
    
    // Check for password in URL
    if !config.database.url.contains('@') {
        return Err(anyhow::anyhow!("DATABASE_URL appears to be missing authentication credentials (no @ symbol found)"));
    }
    
    // Attempt connection with detailed error handling
    let connection_result = PgConnection::establish(&config.database.url);
    
    let mut conn = match connection_result {
        Ok(conn) => {
            tracing::info!("Database migration connection established successfully");
            conn
        },
        Err(e) => {
            tracing::error!("Failed to establish migration connection: {}", e);
            
            // Provide specific guidance based on error type
            let error_str = e.to_string();
            if error_str.contains("fe_sendauth: no password supplied") {
                tracing::error!("PostgreSQL authentication failed - no password provided");
                tracing::error!("This usually means:");
                tracing::error!("  1. DATABASE_URL is missing the password component");
                tracing::error!("  2. Railway PostgreSQL service is not properly configured");
                tracing::error!("  3. Environment variables are not being passed correctly");
                tracing::error!("Expected format: postgres://username:password@host:port/database?sslmode=require");
            } else if error_str.contains("SSL") || error_str.contains("ssl") {
                tracing::error!("SSL/TLS connection issue detected");
                tracing::error!("Railway PostgreSQL requires SSL connections");
                tracing::error!("Ensure DATABASE_URL includes ?sslmode=require parameter");
            } else if error_str.contains("timeout") {
                tracing::error!("Connection timeout - database might be starting up");
            } else if error_str.contains("refused") {
                tracing::error!("Connection refused - check host and port in DATABASE_URL");
            }
            
            return Err(anyhow::anyhow!("Failed to establish migration connection (check TLS configuration): {}", e));
        }
    };
    
    tracing::info!("Running pending database migrations...");
    
    // Run migrations
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(migrations_run) => {
            if migrations_run.is_empty() {
                tracing::info!("No pending migrations to run");
            } else {
                tracing::info!("Successfully ran {} migrations", migrations_run.len());
                for migration in &migrations_run {
                    tracing::info!("  - {}", migration);
                }
            }
        },
        Err(e) => {
            tracing::error!("Migration execution failed: {}", e);
            return Err(anyhow::anyhow!("Migration error: {}", e));
        }
    }
    
    tracing::info!("Database migrations completed successfully");
    
    Ok(())
}

/// Mask sensitive parts of database URL for logging
fn mask_database_url(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        let (before_at, after_at) = url.split_at(at_pos);
        if let Some(colon_pos) = before_at.rfind(':') {
            let (protocol_user, _password) = before_at.split_at(colon_pos);
            format!("{}:****@{}", protocol_user, after_at)
        } else {
            "postgres://****@****".to_string()
        }
    } else {
        "Invalid URL format".to_string()
    }
}