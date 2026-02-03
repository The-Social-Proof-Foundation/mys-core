// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::*;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::AsyncPgConnection;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use mys_config::Config;
use mys_data_ingestion_core::DataIngestionMetrics;
use mys_orderbook_indexer::config::IndexerConfig;
use mys_orderbook_indexer::metrics::OrderBookIndexerMetrics;
use mys_orderbook_indexer::mys_orderbook_indexer::MysOrderBookDataMapper;
use mys_orderbook_indexer::mys_orderbook_indexer::PgOrderbookPersistent;
use mys_orderbook_indexer::postgres_manager::get_connection_pool;
use mys_orderbook_indexer::server::run_server;
use mys_indexer_builder::indexer_builder::IndexerBuilder;
use mys_indexer_builder::mys_datasource::MysCheckpointDatasource;
use mys_indexer_builder::progress::{OutOfOrderSaveAfterDurationPolicy, ProgressSavingPolicy};
use mys_sdk::MysClientBuilder;
use mys_types::base_types::ObjectID;
use mysten_metrics::start_prometheus_server;
use std::net::IpAddr;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/migrations");

#[derive(Parser, Clone, Debug)]
struct Args {
    /// Path to a yaml config
    #[clap(long, short)]
    config_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    // Install default crypto provider for rustls (required for rustls 0.23+)
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let args = Args::parse();

    // load config from environment variables if no config file specified
    let config = if let Some(path) = args.config_path {
        IndexerConfig::load(&path)?
    } else {
        // Try to use environment variables
        IndexerConfig::from_env()?
    };

    // Init metrics server
    let metrics_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.metric_port);
    let registry_service = start_prometheus_server(metrics_address);
    let registry = registry_service.default_registry();
    mysten_metrics::init_metrics(&registry);
    info!("Metrics server started at port {}", config.metric_port);

    let indexer_meterics = OrderBookIndexerMetrics::new(&registry);
    let ingestion_metrics = DataIngestionMetrics::new(&registry);

    let db_url = config.db_url.clone();

    // Run database migrations using TLS-enabled connection
    info!("Running database migrations...");
    run_migrations_with_tls(&db_url).await?;
    info!("Database migrations completed successfully");

    // Log compression configuration
    info!("TimescaleDB compression settings:");
    info!("  Enabled: {}", config.compression.enabled);
    if config.compression.enabled {
        info!(
            "  High-frequency tables compression: {} hours",
            config.compression.high_frequency_compress_after_hours
        );
        info!(
            "  Medium-frequency tables compression: {} hours",
            config.compression.medium_frequency_compress_after_hours
        );
        info!(
            "  Low-frequency tables compression: {} hours",
            config.compression.low_frequency_compress_after_hours
        );
        info!("  Monitor compression with: SELECT * FROM compression_status;");
    }

    let datastore = PgOrderbookPersistent::new(
        get_connection_pool(db_url.clone()).await,
        ProgressSavingPolicy::OutOfOrderSaveAfterDuration(OutOfOrderSaveAfterDurationPolicy::new(
            tokio::time::Duration::from_secs(30),
        )),
    );

    let mys_client = Arc::new(
        MysClientBuilder::default()
            .build(config.mys_rpc_url.clone())
            .await?,
    );
    let mys_checkpoint_datasource = MysCheckpointDatasource::new(
        config.remote_store_url,
        mys_client,
        config.concurrency as usize,
        config
            .checkpoints_path
            .map(|p| p.into())
            .unwrap_or_else(|| {
                tempfile::tempdir()
                    .expect("Failed to create temp directory")
                    .keep()
                    .expect("Failed to persist temp directory")
            }),
        config.orderbook_genesis_checkpoint,
        ingestion_metrics.clone(),
        Box::new(indexer_meterics.clone()),
    );

    let service_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.service_port);
    run_server(service_address, datastore.clone());

    let indexer = IndexerBuilder::new(
        "MysOrderBookIndexer",
        mys_checkpoint_datasource,
        MysOrderBookDataMapper {
            metrics: indexer_meterics.clone(),
            package_id: ObjectID::from_hex_literal(&config.orderbook_package_id.clone())
                .unwrap_or_else(|err| panic!("Failed to parse orderbook package ID: {}", err)),
        },
        datastore,
    )
    .build();
    // Start compression monitoring if enabled
    if config.compression.enabled {
        let db_url_for_monitoring = db_url.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = log_compression_stats(&db_url_for_monitoring).await {
                    tracing::warn!("Failed to log compression stats: {}", e);
                }
                // Check compression stats every hour
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
            }
        });
    }

    indexer.start().await?;

    Ok(())
}

/// Log TimescaleDB compression statistics
async fn log_compression_stats(database_url: &str) -> Result<()> {
    use tokio_postgres_rustls::MakeRustlsConnect;

    // Set up TLS connection
    let certs =
        rustls_native_certs::load_native_certs().expect("Failed to load native root certificates");

    let mut root_store = rustls::RootCertStore::empty();
    for cert in certs {
        if let Err(e) = root_store.add(cert) {
            tracing::warn!("Failed to add certificate to root store: {}", e);
        }
    }

    let rustls_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let tls = MakeRustlsConnect::new(rustls_config);

    let (client, conn) = tokio_postgres::connect(database_url, tls).await?;

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            tracing::warn!("Database connection error during compression monitoring: {e}");
        }
    });

    // Query compression status
    let rows = client
        .query(
            "SELECT 
                hypertable_name,
                total_chunks,
                number_compressed_chunks,
                CASE 
                    WHEN uncompressed_heap_size > 0 THEN 
                        ROUND((1 - compressed_heap_size::numeric / uncompressed_heap_size::numeric) * 100, 2)
                    ELSE 0 
                END AS compression_ratio_percent
            FROM timescaledb_information.compression_settings cs
            JOIN timescaledb_information.hypertables h ON cs.hypertable_name = h.hypertable_name
            LEFT JOIN timescaledb_information.chunks c ON h.hypertable_name = c.hypertable_name
            WHERE compression_enabled = true
            GROUP BY 
                cs.hypertable_name, 
                compressed_heap_size,
                uncompressed_heap_size,
                total_chunks,
                number_compressed_chunks
            ORDER BY hypertable_name",
            &[],
        )
        .await?;

    info!("=== TimescaleDB Compression Status ===");
    for row in rows {
        let table_name: String = row.get(0);
        let total_chunks: Option<i64> = row.get(1);
        let compressed_chunks: Option<i64> = row.get(2);
        let compression_ratio: Option<f64> = row.get(3);

        info!(
            "Table: {} | Chunks: {}/{} compressed | Savings: {}%",
            table_name,
            compressed_chunks.unwrap_or(0),
            total_chunks.unwrap_or(0),
            compression_ratio.unwrap_or(0.0)
        );
    }
    info!("========================================");

    Ok(())
}

async fn run_migrations_with_tls(database_url: &str) -> Result<()> {
    // Set up rustls for TLS connections using native certificates
    info!("Loading native root certificates for database TLS connection...");
    let certs =
        rustls_native_certs::load_native_certs().expect("Failed to load native root certificates");

    let mut root_store = rustls::RootCertStore::empty();
    for cert in certs {
        if let Err(e) = root_store.add(cert) {
            tracing::warn!("Failed to add certificate to root store: {}", e);
        }
    }

    let rustls_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
    let (client, conn) = tokio_postgres::connect(database_url, tls)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("Database connection error: {e}");
        }
    });

    let connection = AsyncPgConnection::try_from(client)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create async connection: {}", e))?;

    let _finished_migrations = tokio::task::spawn_blocking(move || {
        let mut wrapper: AsyncConnectionWrapper<AsyncPgConnection> =
            diesel_async::async_connection_wrapper::AsyncConnectionWrapper::from(connection);
        wrapper
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| format!("{:?}", e))?;
        Ok::<(), String>(())
    })
    .await?
    .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;

    Ok(())
}
