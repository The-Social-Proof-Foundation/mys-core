// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel::migration::MigrationVersion;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use regex;
use rustls;
use rustls_native_certs::load_native_certs;
use std::sync::Arc;
use tokio::time::Duration;
use tokio_postgres_rustls::MakeRustlsConnect;
use tracing;

use crate::social::config::Config;

pub mod connection_manager;
pub use connection_manager::{
    ConnectionManager, DatabaseAccess, IsolationLevel, RetryConfig, TransactionConfig,
};

pub type DbPool = Pool<AsyncPgConnection>;
pub type DbConnection = Object<AsyncPgConnection>;

// Define migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/social");

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
        self.pool
            .get()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }
}

/// Create a TLS-enabled connection using rustls
async fn establish_tls_connection(database_url: &str) -> Result<AsyncPgConnection> {
    // Install default crypto provider for rustls (required for rustls 0.23+)
    // This is a safeguard in case this function is called before rustls is installed elsewhere
    // install_default() returns Err(CryptoProvider) if already installed, which is fine - ignore it
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    
    tracing::info!("Setting up TLS connection with rustls");

    // Load native root certificates - this returns an iterator that we can iterate over
    let certs = load_native_certs().expect("Failed to load native root certificates");

    tracing::info!("Loaded {} native root certificates", certs.len());

    // Create rustls config with native root certificates
    let mut root_store = rustls::RootCertStore::empty();
    for cert in certs {
        root_store.add(cert).unwrap();
    }

    // Now that the provider is installed, we can use the regular builder
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // Create TLS connector with rustls config
    let tls = MakeRustlsConnect::new(config);
    tracing::info!("Rustls TLS connector configured with native certificates");

    // Test the TLS connection using tokio-postgres directly
    let (_client, connection) = tokio_postgres::connect(database_url, tls)
        .await
        .map_err(|e| anyhow!("Failed to establish TLS connection: {}", e))?;

    // Spawn the connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            tracing::error!("PostgreSQL connection error: {}", e);
        }
    });

    tracing::info!("TLS PostgreSQL connection test successful! 🔒");

    // Now use diesel-async which should work with SSL/TLS properly configured
    AsyncPgConnection::establish(database_url)
        .await
        .map_err(|e| anyhow!("Failed to establish diesel-async connection: {}", e))
}

/// Sets up the database connection pool
pub async fn setup_connection_pool(config: &Config) -> Result<Arc<Database>> {
    // Install default crypto provider for rustls (required for rustls 0.23+)
    // This MUST be done before ANY TLS operations, including establish_tls_connection()
    // install_default() returns Err(CryptoProvider) if already installed, which is fine - ignore it
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    
    tracing::info!("Setting up database connection pool with rustls TLS support");
    tracing::info!("Database URL: {}", mask_database_url(&config.database.url));

    // Log environment info for debugging
    if let Ok(railway_env) = std::env::var("RAILWAY_ENVIRONMENT") {
        tracing::info!("Running on Railway environment: {}", railway_env);
    }

    // Validate that the DATABASE_URL has sslmode=require for Railway
    if config.database.url.contains("railway.app")
        || config.database.url.contains("tsdb.cloud.timescale.com")
    {
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
            }
            Ok(Err(e)) => {
                tracing::warn!("Database connection failed on attempt {}: {}", attempt, e);
                last_error = Some(anyhow!("Database connection failed: {}", e));
            }
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

    Err(anyhow!(
        "Failed to establish database connection after 5 attempts"
    ))
}

/// Run database migrations
pub async fn run_migrations(config: &Config) -> Result<()> {
    // Install default crypto provider for rustls (required for rustls 0.23+)
    // This must be done before any TLS operations, including database connections
    // install_default() returns Err(CryptoProvider) if already installed, which is fine - ignore it
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    
    tracing::info!("Running database migrations...");

    // Log database URL for debugging (mask sensitive parts)
    let masked_url = mask_database_url(&config.database.url);
    tracing::info!("Migration connection URL (masked): {}", masked_url);

    // Check if this looks like a Railway database URL
    if config.database.url.contains("railway.app") {
        tracing::info!("Detected Railway PostgreSQL - ensuring SSL configuration");
    }

    // Validate DATABASE_URL format
    if !config.database.url.starts_with("postgres://")
        && !config.database.url.starts_with("postgresql://")
    {
        return Err(anyhow::anyhow!(
            "Invalid DATABASE_URL format. Must start with postgres:// or postgresql://"
        ));
    }

    // Check for password in URL
    if !config.database.url.contains('@') {
        return Err(anyhow::anyhow!(
            "DATABASE_URL appears to be missing authentication credentials (no @ symbol found)"
        ));
    }

    // Attempt async connection with detailed error handling
    let conn = match AsyncPgConnection::establish(&config.database.url).await {
        Ok(conn) => {
            tracing::info!("Database migration connection established successfully");
            conn
        }
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

            return Err(anyhow::anyhow!(
                "Failed to establish migration connection (check TLS configuration): {}",
                e
            ));
        }
    };

    tracing::info!("Running pending database migrations...");

    // Run migrations using async wrapper pattern
    let mut wrapper: AsyncConnectionWrapper<AsyncPgConnection> =
        AsyncConnectionWrapper::from(conn);
    let migrations_run = tokio::task::spawn_blocking(move || {
        wrapper
            .run_pending_migrations(MIGRATIONS)
            .map(|versions| versions.iter().map(MigrationVersion::as_owned).collect::<Vec<_>>())
    })
    .await
    .map_err(|e| anyhow!("Migration task panicked: {}", e))?
    .map_err(|e| {
        let error_msg = format!("{}", e);
        let error_debug = format!("{:?}", e);
        
        tracing::error!("Migration execution failed: {}", error_msg);
        tracing::error!("Migration error details (debug): {}", error_debug);
        
        // Try to extract which migration failed from the error message
        // Diesel migration errors often include the migration name/version in the format:
        // "Failed to run migration <version> with: <error>"
        // or "Migration <version> failed: <error>"
        if let Some(captured) = error_msg
            .split("Failed to run")
            .nth(1)
            .and_then(|s| s.split("with:").next())
            .or_else(|| error_msg.split("Migration").nth(1).and_then(|s| s.split("failed:").next()))
        {
            let migration_name = captured.trim();
            tracing::error!("Failed migration appears to be: {}", migration_name);
            tracing::error!("Check migration file: migrations/social/{}/up.sql", migration_name);
        }
        
        // Extract migration version patterns (e.g., "20260122165006" or similar timestamps)
        let migration_pattern = regex::Regex::new(r"\d{14,}").ok();
        if let Some(re) = migration_pattern {
            if let Some(cap) = re.find(&error_msg) {
                let migration_version = cap.as_str();
                tracing::error!("Failed migration version pattern detected: {}", migration_version);
                tracing::error!("Check migration files matching: migrations/social/{}*/up.sql", migration_version);
            }
        }
        
        // Provide specific guidance for common errors
        if error_msg.contains("Cannot perform this operation outside of a transaction") {
            tracing::error!("This error typically indicates:");
            tracing::error!("  1. A migration SQL file contains explicit BEGIN/COMMIT statements");
            tracing::error!("  2. A migration tries to perform operations that conflict with diesel's automatic transaction wrapping");
            tracing::error!("  3. A migration uses SAVEPOINT or other subtransaction features incorrectly");
            tracing::error!("  4. Connection state issues with AsyncConnectionWrapper in spawn_blocking");
            tracing::error!("  5. A migration attempts DDL operations that cannot run inside a transaction");
            tracing::error!("Please check migration SQL files for explicit transaction control statements.");
        } else if error_msg.contains("current transaction is aborted") {
            tracing::error!("Transaction was aborted - this may indicate a SQL syntax error or constraint violation");
        } else if error_msg.contains("relation") && error_msg.contains("does not exist") {
            tracing::error!("Table or relation does not exist - check migration order and dependencies");
        }
        
        anyhow::anyhow!("Migration error: {}", error_msg)
    })?;

    if migrations_run.is_empty() {
        tracing::info!("No pending migrations to run");
    } else {
        tracing::info!("Successfully ran {} migrations", migrations_run.len());
        for migration in &migrations_run {
            tracing::info!("  - {}", migration);
        }
    }

    tracing::info!("Database migrations completed successfully");

    let migration_name = "20260103210000_refresh_profile_daily_stats_initial";
    if migrations_run.iter().any(|m| m.to_string().contains(migration_name)) {
        // Use a separate connection for the refresh operation to ensure we're not in a transaction
        // TimescaleDB's refresh_continuous_aggregate needs to run outside of any transaction
        match AsyncPgConnection::establish(&config.database.url).await {
            Ok(mut refresh_conn) => {
                if let Err(e) = refresh_profile_daily_stats_aggregate(&mut refresh_conn).await {
                    tracing::warn!("Failed to refresh profile_daily_stats continuous aggregate: {}. It will be populated by the automatic refresh policy.", e);
                }
            }
            Err(e) => {
                tracing::warn!("Failed to establish connection for refresh operation: {}. The continuous aggregate will be populated by the automatic refresh policy.", e);
            }
        }
    }

    Ok(())
}

async fn refresh_profile_daily_stats_aggregate(conn: &mut AsyncPgConnection) -> Result<()> {
    use diesel::sql_query;
    
    // No need for manual COMMIT - we're using a fresh connection that's not in a transaction
    // TimescaleDB's refresh_continuous_aggregate must run outside of a transaction
    sql_query("CALL refresh_continuous_aggregate('profile_daily_stats', NULL, NULL)")
        .execute(conn)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to refresh continuous aggregate: {}", e))?;
    
    sql_query(
        "INSERT INTO continuous_aggregate_refresh_status (view_name, last_manual_refresh, notes)
         VALUES ('profile_daily_stats', NOW(), 'Initial historical data refresh')
         ON CONFLICT (view_name) DO UPDATE
         SET last_manual_refresh = NOW(),
             notes = 'Initial historical data refresh'"
    )
    .execute(conn)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to update tracking table: {}", e))?;
    
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
