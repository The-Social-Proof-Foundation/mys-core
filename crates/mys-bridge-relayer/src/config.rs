// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use mys_types::base_types::ObjectID;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelayerConfig {
    #[serde(default = "default_db_url")]
    pub db_url: String,

    /// Prometheus metrics listen port.
    ///
    /// Defaults to `$METRIC_PORT` if set, else Railway's `$PORT` if set, else 8080.
    #[serde(default = "default_metric_port")]
    pub metric_port: u16,

    /// If true: scan + store deposits only.
    /// If false: enable mint executor.
    #[serde(default = "default_observe_only")]
    pub observe_only: bool,

    /// MySo RPC URL for querying chain state and submitting transactions.
    #[serde(default = "default_mys_rpc_url")]
    pub mys_rpc_url: String,

    /// Bridge shared object ID (constant: 0x9).
    #[serde(default = "default_bridge_object_id")]
    pub bridge_object_id: ObjectID,

    /// Path to relayer's MySo key file (for signing mint transactions).
    #[serde(default = "default_relayer_key_path")]
    pub relayer_key_path: Option<PathBuf>,

    /// EVM chain configuration.
    #[serde(default)]
    pub evm_chains: Vec<EvmChainConfig>,

    /// HD wallet xpub (Base58 encoded) for deriving deposit addresses.
    /// Must be derived at m/54'/6976'/0' (last hardened node).
    #[serde(default = "default_xpub")]
    pub xpub: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EvmChainConfig {
    /// Chain identifier (e.g., "base-sepolia", "ethereum-mainnet").
    pub chain_name: String,

    /// EVM chain ID (e.g., 84532 for Base Sepolia).
    /// This is also used as the source_chain parameter for relayer_mint_and_transfer.
    pub chain_id: u64,

    /// MySo bridge chain ID (u8) for this EVM chain.
    /// Maps to the chain_id used in bridge.move (e.g., 10 for Ethereum mainnet, 11 for Sepolia, 12 for Base Sepolia).
    /// If not specified, defaults to chain_id as u8 (may cause issues if chain_id > 255).
    #[serde(default)]
    pub mys_chain_id: Option<u8>,

    /// EVM RPC URL for block/log scanning.
    pub rpc_url: String,

    /// Optional WebSocket RPC URL (for real-time subscriptions).
    pub ws_url: Option<String>,

    /// Starting block number to scan from.
    pub genesis_block: u64,

    /// Required confirmations before considering a block finalized.
    #[serde(default = "default_confirmations")]
    pub required_confirmations: u64,

    /// ERC20 token addresses to watch (empty = watch all).
    /// If empty, scans all ERC20 Transfer events and filters by deposit addresses.
    #[serde(default)]
    pub token_allowlist: Vec<String>,
}

impl mys_config::Config for RelayerConfig {}

fn default_observe_only() -> bool {
    true
}

fn default_confirmations() -> u64 {
    12 // Default: 12 blocks (~2 minutes on Base)
}

pub fn default_db_url() -> String {
    env::var("DB_URL").expect("db_url must be set in config or via the $DB_URL env var")
}

fn default_metric_port() -> u16 {
    if let Ok(v) = env::var("METRIC_PORT") {
        if let Ok(p) = v.parse::<u16>() {
            return p;
        }
    }
    if let Ok(v) = env::var("PORT") {
        if let Ok(p) = v.parse::<u16>() {
            return p;
        }
    }
    8080
}

fn default_mys_rpc_url() -> String {
    env::var("MYS_RPC_URL")
        .unwrap_or_else(|_| "http://fullnode.testnet.mysocial.network:8082".to_string())
}

fn default_bridge_object_id() -> ObjectID {
    // Bridge shared object ID is constant: 0x9
    ObjectID::from_hex_literal("0x0000000000000000000000000000000000000000000000000000000000000009")
        .expect("Invalid bridge object ID constant")
}

fn default_relayer_key_path() -> Option<PathBuf> {
    env::var("RELAYER_KEY_PATH").ok().map(PathBuf::from)
}

fn default_xpub() -> Option<String> {
    env::var("XPUB").ok()
}
