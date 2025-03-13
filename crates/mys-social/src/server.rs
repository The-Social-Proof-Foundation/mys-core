// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::net::SocketAddr;

use mys_sdk::MysClient;
use tracing::info;

use crate::metrics::SocialApiMetrics;
use crate::social::SocialApi;

pub async fn start_social_api(
    address: SocketAddr,
    mys_client: Arc<MysClient>,
    metrics: SocialApiMetrics,
) -> Result<(), anyhow::Error> {
    info!("Starting social API server at {}", address);
    
    let social_api = SocialApi::new(mys_client, metrics);
    let module = social_api.start_service();
    
    let server = jsonrpsee::server::ServerBuilder::default()
        .build(address)
        .await?;
    
    let _handle = server.start(module);
    
    info!("Social API server running on http://{}", address);
    
    Ok(())
}