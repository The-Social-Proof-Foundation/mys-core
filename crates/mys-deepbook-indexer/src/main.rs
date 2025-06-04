// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use mysten_metrics::start_prometheus_server;
use std::net::IpAddr;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use mys_config::Config;
use mys_data_ingestion_core::DataIngestionMetrics;
use mys_deepbook_indexer::config::IndexerConfig;
use mys_deepbook_indexer::metrics::DeepBookIndexerMetrics;
use mys_deepbook_indexer::postgres_manager::get_connection_pool;
use mys_deepbook_indexer::server::run_server;
use mys_deepbook_indexer::mys_deepbook_indexer::PgDeepbookPersistent;
use mys_deepbook_indexer::mys_deepbook_indexer::MysDeepBookDataMapper;
use mys_indexer_builder::indexer_builder::IndexerBuilder;
use mys_indexer_builder::progress::{OutOfOrderSaveAfterDurationPolicy, ProgressSavingPolicy};
use mys_indexer_builder::mys_datasource::MysCheckpointDatasource;
use mys_sdk::MysClientBuilder;
use mys_types::base_types::ObjectID;
use tracing::info;
use diesel_migrations::MigrationHarness;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::AsyncPgConnection;
use diesel_async::AsyncConnection;

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

    let indexer_meterics = DeepBookIndexerMetrics::new(&registry);
    let ingestion_metrics = DataIngestionMetrics::new(&registry);

    let db_url = config.db_url.clone();
    
    // Run database migrations using TLS-enabled connection
    info!("Running database migrations...");
    run_migrations_with_tls(&db_url).await?;
    info!("Database migrations completed successfully");
    
    let datastore = PgDeepbookPersistent::new(
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
            .unwrap_or(tempfile::tempdir()?.into_path()),
        config.deepbook_genesis_checkpoint,
        ingestion_metrics.clone(),
        Box::new(indexer_meterics.clone()),
    );

    let service_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.service_port);
    run_server(service_address, datastore.clone());

    let indexer = IndexerBuilder::new(
        "MysDeepBookIndexer",
        mys_checkpoint_datasource,
        MysDeepBookDataMapper {
            metrics: indexer_meterics.clone(),
            package_id: ObjectID::from_hex_literal(&config.deepbook_package_id.clone())
                .unwrap_or_else(|err| panic!("Failed to parse deepbook package ID: {}", err)),
        },
        datastore,
    )
    .build();
    indexer.start().await?;

    Ok(())
}

async fn run_migrations_with_tls(database_url: &str) -> Result<()> {
    // Set up rustls for TLS connections
    let rustls_config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(std::sync::Arc::new(SkipServerCertCheck))
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
    
    let connection = AsyncPgConnection::try_from(client).await
        .map_err(|e| anyhow::anyhow!("Failed to create async connection: {}", e))?;
    
    let _finished_migrations = tokio::task::spawn_blocking(move || {
        let mut wrapper: AsyncConnectionWrapper<AsyncPgConnection> = 
            diesel_async::async_connection_wrapper::AsyncConnectionWrapper::from(connection);
        wrapper.run_pending_migrations(MIGRATIONS).map_err(|e| format!("{:?}", e))?;
        Ok::<(), String>(())
    })
    .await?
    .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;
    
    Ok(())
}

fn root_certs() -> rustls::RootCertStore {
    rustls::RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
    }
}

/// Skip performing strict certificate verification to handle cloud database certificates
#[derive(Debug)]
struct SkipServerCertCheck;

impl rustls::client::danger::ServerCertVerifier for SkipServerCertCheck {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        rustls::client::WebPkiServerVerifier::builder(std::sync::Arc::new(root_certs()))
            .build()
            .unwrap()
            .supported_verify_schemes()
    }
}
