// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::env;

/// config as loaded from `config.yaml`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexerConfig {
    pub remote_store_url: String,
    #[serde(default = "default_db_url")]
    pub db_url: String,
    /// Only provide this if you use a colocated FN
    pub checkpoints_path: Option<String>,
    pub mys_rpc_url: String,
    pub deepbook_package_id: String,
    pub deepbook_genesis_checkpoint: u64,
    pub concurrency: u64,
    pub metric_port: u16,
    pub service_port: u16,
}

impl IndexerConfig {
    /// Create config from environment variables
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            remote_store_url: env::var("REMOTE_STORE_URL")
                .unwrap_or_else(|_| "https://storage.googleapis.com/mysocial-testnet-checkpoints".to_string()),
            db_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            checkpoints_path: env::var("CHECKPOINTS_PATH").ok(),
            mys_rpc_url: env::var("MYS_RPC_URL")
                .unwrap_or_else(|_| "https://fullnode.testnet.mysocial.network:9000".to_string()),
            deepbook_package_id: env::var("DEEPBOOK_PACKAGE_ID")
                .expect("DEEPBOOK_PACKAGE_ID must be set"),
            deepbook_genesis_checkpoint: env::var("DEEPBOOK_GENESIS_CHECKPOINT")
                .unwrap_or_else(|_| "0".to_string())
                .parse()?,
            concurrency: env::var("CONCURRENCY")
                .unwrap_or_else(|_| "1".to_string())
                .parse()?,
            metric_port: env::var("METRICS_PORT")
                .unwrap_or_else(|_| "9090".to_string())
                .parse()?,
            service_port: env::var("INDEXER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
        })
    }
}

impl mys_config::Config for IndexerConfig {}

pub fn default_db_url() -> String {
    env::var("DB_URL").expect("db_url must be set in config or via the $DB_URL env var")
}
