// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::Parser;
use mys_config::Config;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use tracing::info;

use mysten_metrics::start_prometheus_server;

use mys_bridge_relayer::config::RelayerConfig;

#[derive(Parser, Clone, Debug)]
#[clap(rename_all = "kebab-case")]
#[clap(name = env!("CARGO_BIN_NAME"))]
#[clap(version)]
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

    let config_path = if let Some(path) = args.config_path {
        path
    } else {
        env::current_dir()
            .expect("Couldn't get current directory")
            .join("config.yaml")
    };

    let config = RelayerConfig::load(&config_path)?;

    // Init metrics server
    let metrics_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.metric_port);
    let registry_service = start_prometheus_server(metrics_address);
    let registry = registry_service.default_registry();
    mysten_metrics::init_metrics(&registry);
    info!("Metrics server started at port {}", config.metric_port);

    mys_bridge_relayer::run(config, registry).await
}
