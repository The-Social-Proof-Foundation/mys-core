// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::net::SocketAddr;

use mys_social::SocialApi;
use mys_sdk::MysClientBuilder;
use jsonrpsee::server::ServerBuilder;
use prometheus::Registry;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    let registry = Registry::new();
    mysten_metrics::init_metrics(&registry);
    
    // Create Mys client
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:9000".to_string());
    let mys_client = MysClientBuilder::default().build(rpc_url).await?;
    let mys_client = Arc::new(mys_client);
    
    // Start Social API
    let addr: SocketAddr = "0.0.0.0:9188".parse()?;
    let metrics = mys_social::SocialApiMetrics::new(&registry);
    let social_api = SocialApi::new(mys_client, metrics);
    
    // Start server
    let server = ServerBuilder::default().build(addr).await?;
    let module = social_api.start_service();
    let _handle = server.start(module);
    // Note: We discard the ServerHandle as we'll run indefinitely
    
    info!("Social API server running on http://{}", addr);
    
    // Keep server running
    futures::future::pending::<()>().await;
    
    Ok(())
}