// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use clap::*;
use colored::Colorize;
use mys::client_commands::MysClientCommands::{ProfileTransaction, ReplayBatch, ReplayTransaction};
use mys::mys_commands::MysCommand;
use mys_types::exit_main;
use tracing::debug;
use rustls;

// Define the `GIT_REVISION` and `VERSION` consts
bin_version::bin_version!();

#[derive(Parser)]
#[clap(
    name = "myso",
    about = "A Byzantine fault tolerant chain with low-latency finality and high throughput",
    rename_all = "kebab-case",
    author,
    version = VERSION,
    propagate_version = true,
)]
struct Args {
    #[clap(subcommand)]
    command: MysCommand,
}

fn main() {
    // Install default crypto provider for rustls (required for rustls 0.23+)
    // This MUST be done BEFORE creating the Tokio runtime to ensure it's available
    // to all worker threads that may spawn TLS connections
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    // Create Tokio runtime manually after rustls is installed
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    rt.block_on(async_main());
}

async fn async_main() {

    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let args = Args::parse();
    let _guard = match args.command {
        MysCommand::Console { .. } | MysCommand::KeyTool { .. } | MysCommand::Move { .. } => {
            telemetry_subscribers::TelemetryConfig::new()
                .with_log_level("error")
                .with_env()
                .init()
        }

        MysCommand::Client {
            cmd: Some(ReplayBatch { .. }),
            ..
        } => telemetry_subscribers::TelemetryConfig::new()
            .with_log_level("info")
            .with_env()
            .init(),

        MysCommand::Client {
            cmd: Some(ReplayTransaction {
                gas_info, ptb_info, ..
            }),
            ..
        } => {
            let mut config = telemetry_subscribers::TelemetryConfig::new()
                .with_log_level("info")
                .with_env();
            if gas_info {
                config = config.with_trace_target("replay_gas_info");
            }
            if ptb_info {
                config = config.with_trace_target("replay_ptb_info");
            }
            config.init()
        }

        MysCommand::Client {
            cmd: Some(ProfileTransaction { .. }),
            ..
        } => {
            // enable full logging for ProfileTransaction and ReplayTransaction
            telemetry_subscribers::TelemetryConfig::new()
                .with_env()
                .init()
        }

        _ => telemetry_subscribers::TelemetryConfig::new()
            .with_log_level("error")
            .with_env()
            .init(),
    };
    debug!("Mys CLI version: {VERSION}");
    exit_main!(args.command.execute().await);
}
