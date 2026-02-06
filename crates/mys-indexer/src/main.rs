// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use mys_indexer::backfill::backfill_runner::BackfillRunner;
use mys_indexer::config::{Command, RetentionConfig, SocialIndexerConfig, UploadOptions};
use mys_indexer::database::ConnectionPool;
use mys_indexer::db::setup_postgres::clear_database;
use mys_indexer::db::{
    check_db_migration_consistency, check_prunable_tables_valid, reset_database, run_migrations,
};
use mys_indexer::indexer::Indexer;
use mys_indexer::metrics::{
    spawn_connection_pool_metric_collector, start_prometheus_server, IndexerMetrics,
};
use mys_indexer::restorer::formal_snapshot::IndexerFormalSnapshotRestorer;
use mys_indexer::store::PgIndexerStore;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

/// Helper function to run social migrations
async fn run_social_migrations(
    database_url: &url::Url,
    social_config: &SocialIndexerConfig,
) -> anyhow::Result<()> {
    // Determine the social database URL - use social_database_url if provided, otherwise use main database_url
    let social_db_url = social_config
        .social_database_url
        .as_ref()
        .map(|u| u.to_string())
        .unwrap_or_else(|| database_url.to_string());

    let social_db_config = mys_indexer::social::config::Config {
        database: mys_indexer::social::config::DatabaseConfig {
            url: social_db_url,
            max_connections: social_config.social_db_max_connections,
        },
        ..Default::default()
    };

    info!("Running social migrations...");
    mys_indexer::social::db::run_migrations(&social_db_config)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run social migrations: {}", e))?;
    info!("Social migrations completed successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = mys_indexer::config::IndexerConfig::parse();

    // NOTE: this is to print out tracing like info, warn & error.
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();
    warn!("WARNING: Mys indexer is still experimental and we expect occasional breaking changes that require backfills.");

    let (_registry_service, registry) = start_prometheus_server(opts.metrics_address)?;
    mysten_metrics::init_metrics(&registry);
    let indexer_metrics = IndexerMetrics::new(&registry);

    let pool = ConnectionPool::new(
        opts.database_url.clone(),
        opts.connection_pool_config.clone(),
    )
    .await?;
    spawn_connection_pool_metric_collector(indexer_metrics.clone(), pool.clone());

    match opts.command {
        Command::Indexer {
            ingestion_config,
            snapshot_config,
            pruning_options,
            upload_options,
            social_config,
            mvr_mode,
        } => {
            // Make sure to run all migrations on startup, and also serve as a compatibility check.
            run_migrations(pool.dedicated_connection().await?).await?;
            
            // Run social migrations after main migrations
            run_social_migrations(&opts.database_url, &social_config).await?;

            let retention_config = if mvr_mode {
                warn!("Indexer in MVR mode is configured to prune `objects_history` to 2 epochs. The other tables have a 2000 epoch retention.");
                Some(RetentionConfig {
                    epochs_to_keep: 2000, // epochs, roughly 5+ years. We really just care about pruning `objects_history` per the default 2 epochs.
                    overrides: Default::default(),
                })
            } else {
                pruning_options.load_from_file()
            };
            if retention_config.is_some() {
                check_prunable_tables_valid(&mut pool.get().await?).await?;
            }

            let store = PgIndexerStore::new(pool, upload_options, indexer_metrics.clone());

            Indexer::start_writer(
                ingestion_config,
                store,
                indexer_metrics,
                snapshot_config,
                retention_config,
                CancellationToken::new(),
                mvr_mode,
                social_config,
            )
            .await?;
        }
        Command::JsonRpcService(json_rpc_config) => {
            check_db_migration_consistency(&mut pool.get().await?).await?;

            Indexer::start_reader(&json_rpc_config, &registry, pool, CancellationToken::new())
                .await?;
        }
        Command::ResetDatabase {
            force,
            skip_migrations,
        } => {
            if !force {
                return Err(anyhow::anyhow!(
                    "Resetting the DB requires use of the `--force` flag",
                ));
            }

            if skip_migrations {
                clear_database(&mut pool.dedicated_connection().await?).await?;
            } else {
                reset_database(pool.dedicated_connection().await?).await?;
                // Also run social migrations after reset
                let default_social_config = SocialIndexerConfig::default();
                run_social_migrations(&opts.database_url, &default_social_config).await?;
            }
        }
        Command::RunMigrations => {
            run_migrations(pool.dedicated_connection().await?).await?;
            // Also run social migrations
            let default_social_config = SocialIndexerConfig::default();
            run_social_migrations(&opts.database_url, &default_social_config).await?;
        }
        Command::RunBackFill {
            start,
            end,
            runner_kind,
            backfill_config,
        } => {
            let total_range = start..=end;
            BackfillRunner::run(runner_kind, pool, backfill_config, total_range).await;
        }
        Command::Restore(restore_config) => {
            let store =
                PgIndexerStore::new(pool, UploadOptions::default(), indexer_metrics.clone());
            let mut formal_restorer =
                IndexerFormalSnapshotRestorer::new(store, restore_config).await?;
            formal_restorer.restore().await?;
        }
    }

    Ok(())
}
