// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::*;
use mys_analytics_indexer::{
    analytics_metrics::AnalyticsMetrics, errors::AnalyticsIndexerError, make_analytics_processor,
    AnalyticsIndexerConfig,
};
use mys_data_ingestion_core::{setup_single_workflow, ReaderOptions};
use mysten_service::metrics::start_basic_prometheus_server;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let config = AnalyticsIndexerConfig::parse();
    info!("Parsed config: {:#?}", config);

    // Start standard Prometheus server on port 9184
    let registry = start_basic_prometheus_server();
    mysten_metrics::init_metrics(&registry);
    let metrics = AnalyticsMetrics::new(&registry);
    let remote_store_url = config.remote_store_url.clone();
    let processor = make_analytics_processor(config, metrics)
        .await
        .map_err(|e| AnalyticsIndexerError::GenericError(e.to_string()))?;
    let watermark = processor.last_committed_checkpoint().unwrap_or_default() + 1;

    let reader_options = ReaderOptions {
        batch_size: 10,
        ..Default::default()
    };
    let (executor, exit_sender) = setup_single_workflow(
        processor,
        remote_store_url,
        watermark,
        1,
        Some(reader_options),
    )
    .await?;

    tokio::spawn(async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
        exit_sender
            .send(())
            .expect("Failed to gracefully process shutdown");
    });
    executor.await?;
    Ok(())
}
