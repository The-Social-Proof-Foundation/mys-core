// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use bip32::XPub;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use prometheus::Registry;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::{error, info};

pub mod config;
pub mod address_book;
pub mod address_index;
pub mod models;
pub mod postgres_manager;
pub mod schema;
pub mod storage;
pub mod scanner;
pub mod executor;

use mys_pg_db::{Db, DbArgs};
use postgres_manager::get_connection_pool;
use storage::load_address_index;

// Keep migrations colocated with this crate.
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/migrations");

/// Main relayer entry point: runs migrations, then starts scanner and executor tasks.
pub async fn run(config: config::RelayerConfig, _registry: Registry) -> Result<()> {
    // 1. Run migrations
    run_migrations(&config.db_url).await?;
    info!("Database migrations completed");

    // 2. Initialize database pool
    let pool = get_connection_pool(config.db_url.clone()).await;

    // 3. Load xpub if provided (for address derivation)
    let xpub: Option<XPub> = if let Some(xpub_str) = &config.xpub {
        Some(
            xpub_str
                .parse()
                .map_err(|e| anyhow!("Failed to parse xpub: {e}"))?,
        )
    } else {
        None
    };

    // 4. Start scanner tasks for each EVM chain
    let mut scanner_handles = Vec::new();
    for chain_config in &config.evm_chains {
        let chain_config = chain_config.clone();
        let pool_clone = pool.clone();
        
        // Load address index for this chain
        let mut address_index = load_address_index(&pool_clone, &chain_config.chain_name).await?;
        
        // Create scanner
        let mut scanner = scanner::EvmScanner::new(chain_config, pool_clone, address_index)?;
        
        // Spawn scanner task
        let handle = tokio::spawn(async move {
            if let Err(e) = scanner.run().await {
                error!(error = %e, "Scanner task failed");
            }
        });
        scanner_handles.push(handle);
    }

    info!(
        scanner_count = scanner_handles.len(),
        "Started {} scanner task(s)",
        scanner_handles.len()
    );

    // 5. Start executor task (if not observe-only)
    let executor_handle: Option<JoinHandle<()>> = if !config.observe_only {
        let config_clone = config.clone();
        let pool_clone = pool.clone();
        Some(tokio::spawn(async move {
            match executor::RelayerExecutor::new(config_clone, pool_clone).await {
                Ok(executor) => {
                    if let Err(e) = executor.run().await {
                        error!(error = %e, "Executor task failed");
                    }
                }
                Err(e) => {
                    error!(error = %e, "Failed to initialize executor");
                }
            }
        }))
    } else {
        info!("Executor disabled (observe_only=true)");
        None
    };

    info!(
        observe_only = config.observe_only,
        metric_port = config.metric_port,
        "mys-bridge-relayer started"
    );

    // 6. Wait for all tasks or Ctrl+C
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
        _ = async {
            // Wait for any task to fail
            for handle in scanner_handles {
                let _ = handle.await;
            }
            if let Some(handle) = executor_handle {
                let _ = handle.await;
            }
        } => {
            error!("One or more tasks exited unexpectedly");
        }
    }

    Ok(())
}

pub async fn run_migrations(db_url: &str) -> Result<()> {
    let db_args = DbArgs {
        database_url: db_url.parse()?,
        ..Default::default()
    };
    let db = Db::for_write(db_args).await?;
    db.run_migrations(MIGRATIONS).await?;
    Ok(())
}
