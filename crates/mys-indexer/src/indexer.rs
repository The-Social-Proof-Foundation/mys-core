// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::env;

use anyhow::Result;
use prometheus::Registry;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use async_trait::async_trait;
use futures::future::try_join_all;
use mys_data_ingestion_core::{
    DataIngestionMetrics, IndexerExecutor, ProgressStore, ReaderOptions, WorkerPool,
};
use mys_types::messages_checkpoint::CheckpointSequenceNumber;
use mysten_metrics::spawn_monitored_task;

use crate::build_json_rpc_server;
use crate::config::{IngestionConfig, JsonRpcConfig, RetentionConfig, SnapshotLagConfig, SocialIndexerConfig};
use crate::database::ConnectionPool;
use crate::db::run_migrations;
use crate::errors::IndexerError;
use crate::handlers::checkpoint_handler::new_handlers;
use crate::handlers::objects_snapshot_handler::start_objects_snapshot_handler;
use crate::handlers::pruner::Pruner;
use crate::indexer_reader::IndexerReader;
use crate::metrics::IndexerMetrics;
use crate::store::{IndexerStore, PgIndexerStore};

pub struct Indexer;

impl Indexer {
    pub async fn start_writer(
        config: IngestionConfig,
        store: PgIndexerStore,
        metrics: IndexerMetrics,
        snapshot_config: SnapshotLagConfig,
        mut retention_config: Option<RetentionConfig>,
        cancel: CancellationToken,
        mvr_mode: bool,
        social_config: SocialIndexerConfig,
    ) -> Result<(), IndexerError> {
        info!(
            "Mys Indexer Writer (version {:?}) started...",
            env!("CARGO_PKG_VERSION")
        );
        info!("Mys Indexer Writer config: {config:?}",);
        if social_config.enable_social_indexer {
            info!("Social indexer enabled with package: {:?}", social_config.mysocial_package_address);
        }

        // Run main indexer migrations first (before social migrations)
        info!("Running main indexer migrations...");
        let main_migration_conn = store.pool().dedicated_connection().await
            .map_err(|e| IndexerError::PostgresReadError(format!("Failed to get connection for main migrations: {}", e)))?;
        run_migrations(main_migration_conn).await
            .map_err(|e| IndexerError::PostgresReadError(format!("Failed to run main migrations: {}", e)))?;
        info!("Main indexer migrations completed successfully");

        let extra_reader_options = ReaderOptions {
            batch_size: config.checkpoint_download_queue_size,
            timeout_secs: config.checkpoint_download_timeout,
            data_limit: config.checkpoint_download_queue_size_bytes,
            gc_checkpoint_files: config.gc_checkpoint_files,
            ..Default::default()
        };

        // Start objects snapshot processor, which is a separate pipeline with its ingestion pipeline.
        let (object_snapshot_worker, object_snapshot_watermark) = start_objects_snapshot_handler(
            store.clone(),
            metrics.clone(),
            snapshot_config,
            cancel.clone(),
            config.start_checkpoint,
            config.end_checkpoint,
        )
        .await?;

        if mvr_mode {
            warn!("Indexer in MVR mode is configured to prune `objects_history` to 2 epochs. The other tables have a 2000 epoch retention.");
            retention_config = Some(RetentionConfig {
                epochs_to_keep: 2000, // epochs, roughly 5+ years. We really just care about pruning `objects_history` per the default 2 epochs.
                overrides: Default::default(),
            });
        }

        if let Some(retention_config) = retention_config {
            let pruner = Pruner::new(store.clone(), retention_config, metrics.clone())?;
            let cancel_clone = cancel.clone();
            spawn_monitored_task!(pruner.start(cancel_clone));
        }

        // If we already have chain identifier indexed (i.e. the first checkpoint has been indexed),
        // then we persist protocol configs for protocol versions not yet in the db.
        // Otherwise, we would do the persisting in `commit_checkpoint` while the first cp is
        // being indexed.
        if let Some(chain_id) = IndexerStore::get_chain_identifier(&store).await? {
            store
                .persist_protocol_configs_and_feature_flags(chain_id)
                .await?;
        }

        let mut exit_senders = vec![];
        let mut executors = vec![];

        let (worker, primary_watermark) = new_handlers(
            store,
            metrics,
            cancel.clone(),
            config.start_checkpoint,
            config.end_checkpoint,
            mvr_mode,
        )
        .await?;

        // Ingestion task watermarks are snapshotted once on indexer startup based on the
        // corresponding watermark table before being handed off to the ingestion task.
        let mut watermarks = vec![
            ("primary".to_string(), primary_watermark),
            ("object_snapshot".to_string(), object_snapshot_watermark),
        ];

        // Add social watermark if enabled (starts from 0, processor will skip already processed)
        let social_watermark = if social_config.enable_social_indexer && social_config.package_address().is_some() {
            // Use start_checkpoint if provided, otherwise start from 0
            let watermark = config.start_checkpoint.unwrap_or(0);
            watermarks.push(("social".to_string(), watermark));
            Some(watermark)
        } else {
            None
        };

        let worker_count = if social_watermark.is_some() { 3 } else { 2 };
        let progress_store = ShimIndexerProgressStore::new(watermarks);
        let mut executor = IndexerExecutor::new(
            progress_store.clone(),
            worker_count,
            DataIngestionMetrics::new(&Registry::new()),
        );

        let worker_pool = WorkerPool::new(
            worker,
            "primary".to_string(),
            config.checkpoint_download_queue_size,
        );
        executor.register(worker_pool).await?;

        // Register social checkpoint processor if enabled
        if social_config.enable_social_indexer {
            if let Some(package_address) = social_config.package_address() {
                info!("Initializing social checkpoint processor for package: {}", package_address);

                // Set up social database connection
                let social_db_url = social_config.social_database_url
                    .as_ref()
                    .map(|u| u.to_string())
                    .unwrap_or_else(|| std::env::var("DATABASE_URL").unwrap_or_default());

                let social_db_config = crate::social::config::Config {
                    database: crate::social::config::DatabaseConfig {
                        url: social_db_url,
                        max_connections: social_config.social_db_max_connections,
                    },
                    ..Default::default()
                };

                match crate::social::db::setup_connection_pool(&social_db_config).await {
                    Ok(social_db) => {
                        // Run social migrations (after main migrations)
                        info!("Running social indexer migrations...");
                        if let Err(e) = crate::social::db::run_migrations(&social_db_config) {
                            warn!("Failed to run social migrations: {}. Continuing anyway...", e);
                        } else {
                            info!("Social indexer migrations completed successfully");
                        }

                        let social_processor = crate::social::blockchain::SocialCheckpointProcessor::new(
                            social_db.clone(),
                            package_address,
                        );

                        let social_worker_pool = WorkerPool::new(
                            social_processor,
                            "social".to_string(),
                            config.checkpoint_download_queue_size,
                        );
                        executor.register(social_worker_pool).await?;
                        info!("Social checkpoint processor registered successfully");

                        // Start the social API server
                        let api_db = social_db.clone();
                        let api_config = social_db_config.clone();
                        spawn_monitored_task!(async move {
                            if let Err(e) = crate::social::api::start_api_server(api_db, &api_config).await {
                                error!("Failed to start social API server: {}", e);
                            }
                        });
                        info!("Social API server started");
                    }
                    Err(e) => {
                        warn!("Failed to set up social database connection: {}. Social indexer disabled.", e);
                    }
                }
            } else {
                warn!("Social indexer enabled but no package address provided. Skipping social indexer.");
            }
        }

        let (exit_sender, exit_receiver) = oneshot::channel();
        executors.push((executor, exit_receiver));
        exit_senders.push(exit_sender);

        // in a non-colocated setup, start a separate indexer for processing object snapshots
        if config.sources.data_ingestion_path.is_none() {
            let executor = IndexerExecutor::new(
                progress_store,
                1,
                DataIngestionMetrics::new(&Registry::new()),
            );
            let (exit_sender, exit_receiver) = oneshot::channel();
            exit_senders.push(exit_sender);
            executors.push((executor, exit_receiver));
        }

        let worker_pool = WorkerPool::new(
            object_snapshot_worker,
            "object_snapshot".to_string(),
            config.checkpoint_download_queue_size,
        );
        let executor = executors.last_mut().expect("executors is not empty");
        executor.0.register(worker_pool).await?;

        // Spawn a task that links the cancellation token to the exit sender
        spawn_monitored_task!(async move {
            cancel.cancelled().await;
            for exit_sender in exit_senders {
                let _ = exit_sender.send(());
            }
        });

        info!("Starting data ingestion executor...");
        let futures = executors.into_iter().map(|(executor, exit_receiver)| {
            executor.run(
                config
                    .sources
                    .data_ingestion_path
                    .clone()
                    .unwrap_or(tempfile::tempdir().unwrap().keep()),
                config
                    .sources
                    .remote_store_url
                    .as_ref()
                    .map(|url| url.as_str().to_owned()),
                vec![],
                extra_reader_options.clone(),
                exit_receiver,
            )
        });
        try_join_all(futures).await?;
        Ok(())
    }

    pub async fn start_reader(
        config: &JsonRpcConfig,
        registry: &Registry,
        pool: ConnectionPool,
        cancel: CancellationToken,
    ) -> Result<(), IndexerError> {
        info!(
            "Mys Indexer Reader (version {:?}) started...",
            env!("CARGO_PKG_VERSION")
        );
        let indexer_reader = IndexerReader::new(pool);
        let handle = build_json_rpc_server(registry, indexer_reader, config, cancel)
            .await
            .expect("Json rpc server should not run into errors upon start.");
        tokio::spawn(async move { handle.stopped().await })
            .await
            .expect("Rpc server task failed");

        Ok(())
    }
}

#[derive(Clone)]
struct ShimIndexerProgressStore {
    watermarks: HashMap<String, CheckpointSequenceNumber>,
}

impl ShimIndexerProgressStore {
    fn new(watermarks: Vec<(String, CheckpointSequenceNumber)>) -> Self {
        Self {
            watermarks: watermarks.into_iter().collect(),
        }
    }
}

#[async_trait]
impl ProgressStore for ShimIndexerProgressStore {
    async fn load(&mut self, task_name: String) -> Result<CheckpointSequenceNumber> {
        Ok(*self.watermarks.get(&task_name).expect("missing watermark"))
    }

    async fn save(&mut self, _: String, _: CheckpointSequenceNumber) -> Result<()> {
        Ok(())
    }
}
