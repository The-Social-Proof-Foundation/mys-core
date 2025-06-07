// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use mys_graphql_rpc::commands::Command;
use mys_graphql_rpc::config::{ServerConfig, ServiceConfig, Version};
use mys_graphql_rpc::server::graphiql_server::start_graphiql_server;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

// Define the `GIT_REVISION` const
bin_version::git_revision!();

// VERSION mimics what other mys binaries use for the same const
static VERSION: Version = Version {
    major: env!("CARGO_PKG_VERSION_MAJOR"),
    minor: env!("CARGO_PKG_VERSION_MINOR"),
    patch: env!("CARGO_PKG_VERSION_PATCH"),
    sha: GIT_REVISION,
    full: const_str::concat!(
        env!("CARGO_PKG_VERSION_MAJOR"),
        ".",
        env!("CARGO_PKG_VERSION_MINOR"),
        ".",
        env!("CARGO_PKG_VERSION_PATCH"),
        "-",
        GIT_REVISION
    ),
};

#[tokio::main]
async fn main() {
    // Debug: Print environment variables FIRST
    println!("=== MAIN START - Environment Variables ===");
    if let Ok(database_url) = std::env::var("DATABASE_URL") {
        println!("DATABASE_URL from env: {}", database_url);
    } else {
        println!("DATABASE_URL not set in environment");
    }
    if let Ok(rpc_url) = std::env::var("RPC_URL") {
        println!("RPC_URL from env: {}", rpc_url);
    } else {
        println!("RPC_URL not set in environment");
    }
    if let Ok(port) = std::env::var("PORT") {
        println!("PORT from env: {}", port);
    } else {
        println!("PORT not set in environment");
    }
    println!("Args: {:?}", std::env::args().collect::<Vec<_>>());
    println!("==============================");
    
    let cmd: Command = Command::parse();
    match cmd {
        Command::GenerateConfig { output } => {
            let config = ServiceConfig::default();
            let toml = toml::to_string_pretty(&config).expect("Failed to serialize configuration");

            if let Some(path) = output {
                fs::write(&path, toml).unwrap_or_else(|e| {
                    panic!("Failed to write configuration to {}: {e}", path.display())
                });
            } else {
                println!("{}", toml);
            }
        }

        Command::StartServer {
            ide,
            mut connection,
            config,
            mut tx_exec_full_node,
        } => {
            // Debug: Print environment variables
            println!("=== Environment Variables ===");
            if let Ok(database_url) = std::env::var("DATABASE_URL") {
                println!("DATABASE_URL from env: {}", database_url);
            } else {
                println!("DATABASE_URL not set in environment");
            }
            if let Ok(rpc_url) = std::env::var("RPC_URL") {
                println!("RPC_URL from env: {}", rpc_url);
            } else {
                println!("RPC_URL not set in environment");
            }
            if let Ok(port) = std::env::var("PORT") {
                println!("PORT from env: {}", port);
            } else {
                println!("PORT not set in environment");
            }
            
            // Debug: Print connection config BEFORE override
            println!("Connection config BEFORE override: {:?}", connection);
            println!("TX exec config BEFORE override: {:?}", tx_exec_full_node);
            
            // Override with environment variables if set
            if let Ok(database_url) = std::env::var("DATABASE_URL") {
                if database_url != "$DATABASE_URL" {  // Make sure it's not the literal string
                    println!("Overriding db_url with DATABASE_URL: {}", database_url);
                    connection.db_url = database_url;
                }
            }
            
            if let Ok(rpc_url) = std::env::var("RPC_URL") {
                if rpc_url != "$RPC_URL" {  // Make sure it's not the literal string
                    println!("Overriding node_rpc_url with RPC_URL: {}", rpc_url);
                    tx_exec_full_node.node_rpc_url = Some(rpc_url);
                }
            }
            
            if let Ok(port) = std::env::var("PORT") {
                if let Ok(port_num) = port.parse::<u16>() {
                    println!("Overriding port with PORT: {}", port_num);
                    connection.port = port_num;
                }
            }
            
            // Debug: Print connection config AFTER override
            println!("Connection config AFTER override: {:?}", connection);
            println!("TX exec config AFTER override: {:?}", tx_exec_full_node);
            println!("==============================");

            let service_config = service_config(config);
            let _guard = telemetry_subscribers::TelemetryConfig::new()
                .with_env()
                .init();
            let tracker = TaskTracker::new();
            let cancellation_token = CancellationToken::new();

            println!("Starting server...");
            let server_config = ServerConfig {
                connection,
                service: service_config,
                ide,
                tx_exec_full_node,
                ..ServerConfig::default()
            };

            let cancellation_token_clone = cancellation_token.clone();
            let graphql_service_handle = tracker.spawn(async move {
                start_graphiql_server(&server_config, &VERSION, cancellation_token_clone)
                    .await
                    .unwrap();
            });

            // Wait for shutdown signal
            tokio::select! {
                result = graphql_service_handle => {
                    if let Err(e) = result {
                        println!("GraphQL service crashed or exited with error: {:?}", e);
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("Ctrl+C signal received.");
                },
            }

            println!("Shutting down...");

            // Send shutdown signal to application
            cancellation_token.cancel();
            tracker.close();
            tracker.wait().await;
        }
    }
}

fn service_config(path: Option<PathBuf>) -> ServiceConfig {
    let Some(path) = path else {
        return ServiceConfig::default();
    };

    let contents = fs::read_to_string(path).expect("Reading configuration");
    ServiceConfig::read(&contents).expect("Deserializing configuration")
}
