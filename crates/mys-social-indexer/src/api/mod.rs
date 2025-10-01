// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod handlers;
pub mod routes;
pub use routes::build_router;

use anyhow::Result;
use axum::http::Method;
use axum_server::bind;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::config::Config;
use crate::db::Database;

/// API server for the indexer
pub struct ApiServer {
    /// The database connection
    db: Arc<Database>,
    /// The address to bind to
    addr: SocketAddr,
}

impl ApiServer {
    /// Create a new API server
    pub fn new(db: Arc<Database>, addr: SocketAddr) -> Self {
        Self { db, addr }
    }

    /// Start the API server
    pub async fn start(&self) -> Result<()> {
        // Build the router using our routes module
        let mut app = build_router(self.db.clone());

        // Add CORS middleware
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(Any)
            .allow_headers(Any);

        // Add CORS to the router
        app = app.layer(cors);

        info!("Starting API server on {}", self.addr);

        // Start the server
        bind(self.addr).serve(app.into_make_service()).await?;

        Ok(())
    }
}

/// Set up the API server
pub async fn start_api_server(db: Arc<Database>, config: &Config) -> Result<()> {
    // Get the API server address from the config
    let api_addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port).parse()?;

    info!("Configuring API server at {}", api_addr);

    // Create the server
    let server = ApiServer::new(db, api_addr);

    // Start the server
    server.start().await
}
