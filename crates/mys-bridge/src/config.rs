// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::abi::EthBridgeConfig;
use crate::crypto::BridgeAuthorityKeyPair;
use crate::error::BridgeError;
use crate::eth_client::EthClient;
use crate::metered_eth_provider::new_metered_eth_provider;
use crate::metered_eth_provider::MeteredEthHttpProvier;
use crate::metrics::BridgeMetrics;
use crate::mys_client::MysClient;
use crate::types::{is_route_valid, BridgeAction};
use crate::utils::get_eth_contract_addresses;
use anyhow::anyhow;
use ethers::providers::Middleware;
use ethers::types::Address as EthAddress;
use futures::{future, StreamExt};
use mys_config::Config;
use mys_json_rpc_types::Coin;
use mys_keys::keypair_file::read_key;
use mys_sdk::apis::CoinReadApi;
use mys_sdk::{MysClient as MysSdkClient, MysClientBuilder};
use mys_types::base_types::ObjectRef;
use mys_types::base_types::{MysAddress, ObjectID};
use mys_types::bridge::BridgeChainId;
use mys_types::crypto::KeypairTraits;
use mys_types::crypto::{get_key_pair_from_rng, MysKeyPair, NetworkKeyPair};
use mys_types::digests::{get_mainnet_chain_identifier, get_testnet_chain_identifier};
use mys_types::event::EventID;
use mys_types::object::Owner;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EthConfig {
    /// Rpc url for Eth fullnode, used for query stuff.
    pub eth_rpc_url: String,
    /// The proxy address of MysBridge
    pub eth_bridge_proxy_address: String,
    /// The expected BridgeChainId on Eth side.
    pub eth_bridge_chain_id: u8,
    /// The starting block for EthSyncer to monitor eth contracts.
    /// It is required when `run_client` is true. Usually this is
    /// the block number when the bridge contracts are deployed.
    /// When BridgeNode starts, it reads the contract watermark from storage.
    /// If the watermark is not found, it will start from this fallback block number.
    /// If the watermark is found, it will start from the watermark.
    /// this v.s.`eth_contracts_start_block_override`:
    pub eth_contracts_start_block_fallback: Option<u64>,
    /// The starting block for EthSyncer to monitor eth contracts. It overrides
    /// the watermark in storage. This is useful when we want to reprocess the events
    /// from a specific block number.
    /// Note: this field has to be reset after starting the BridgeNode, otherwise it will
    /// reprocess the events from this block number every time it starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eth_contracts_start_block_override: Option<u64>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MysConfig {
    /// Rpc url for Mys fullnode, used for query stuff and submit transactions.
    pub mys_rpc_url: String,
    /// The expected BridgeChainId on Mys side.
    pub mys_bridge_chain_id: u8,
    /// Path of the file where bridge client key (any MysKeyPair) is stored.
    /// If `run_client` is true, and this is None, then use `bridge_authority_key_path` as client key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_key_path: Option<PathBuf>,
    /// The gas object to use for paying for gas fees for the client. It needs to
    /// be owned by the address associated with bridge client key. If not set
    /// and `run_client` is true, it will query and use the gas object with highest
    /// amount for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_gas_object: Option<ObjectID>,
    /// Override the last processed EventID for bridge module `bridge`.
    /// When set, MysSyncer will start from this cursor (exclusively) instead of the one in storage.
    /// If the cursor is not found in storage or override, the query will start from genesis.
    /// Key: mys module, Value: last processed EventID (tx_digest, event_seq).
    /// Note 1: This field should be rarely used. Only use it when you understand how to follow up.
    /// Note 2: the EventID needs to be valid, namely it must exist and matches the filter.
    /// Otherwise, it will miss one event because of fullnode Event query semantics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mys_bridge_module_last_processed_event_id_override: Option<EventID>,
}

/// Auto-relay configuration loaded from config file
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelayConfigFile {
    /// Whether auto-relay is enabled
    pub enabled: bool,
    /// Maximum number of retry attempts
    #[serde(default = "default_max_retries")]
    pub max_retries: u8,
    /// Delay between retries in seconds
    #[serde(default = "default_retry_delay")]
    pub retry_delay_seconds: u64,
    /// MySocial relay configuration
    #[serde(default)]
    pub mys: MysRelayConfigFile,
    /// EVM relay configuration
    pub evm: Option<EvmRelayConfigFile>,
}

/// MySocial-specific relay configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MysRelayConfigFile {
    /// Maximum gas budget for Mys transactions (in MIST)
    #[serde(default = "default_mys_gas_budget")]
    pub gas_budget: u64,
}

impl Default for MysRelayConfigFile {
    fn default() -> Self {
        Self {
            gas_budget: default_mys_gas_budget(),
        }
    }
}

/// EVM-specific relay configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EvmRelayConfigFile {
    /// Whether EVM relay is enabled
    pub enabled: bool,
    /// EVM RPC URL (can use ETH_RPC_URL from eth config if not specified)
    pub rpc_url: Option<String>,
    /// Bridge contract address (can use from eth config if not specified)
    pub bridge_contract_address: Option<String>,
    /// Maximum gas price in Gwei
    #[serde(default = "default_max_gas_price_gwei")]
    pub max_gas_price_gwei: u64,
    /// Percentage buffer for gas estimation
    #[serde(default = "default_gas_buffer_percent")]
    pub gas_estimate_buffer_percent: u8,
    /// Number of confirmations to wait for
    #[serde(default = "default_confirmation_blocks")]
    pub confirmation_blocks: u64,
}

fn default_max_retries() -> u8 {
    3
}

fn default_retry_delay() -> u64 {
    30
}

fn default_mys_gas_budget() -> u64 {
    100_000_000 // 0.1 MYS
}

fn default_max_gas_price_gwei() -> u64 {
    10
}

fn default_gas_buffer_percent() -> u8 {
    20
}

fn default_confirmation_blocks() -> u64 {
    2
}

/// Deposit system configuration loaded from config file
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DepositConfigFile {
    /// Whether custodial deposit system is enabled
    pub enabled: bool,
    /// Polling interval for deposit monitoring in seconds
    #[serde(default = "default_deposit_poll_interval")]
    pub poll_interval_secs: u64,
    /// Whether to automatically fund gas for deposit addresses
    #[serde(default = "default_auto_fund_gas")]
    pub auto_fund_gas: bool,
    /// Optional: specify supported ERC20 token addresses
    /// If not specified, will query from BridgeConfig contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_tokens: Option<Vec<String>>,
}

fn default_deposit_poll_interval() -> u64 {
    45
}

fn default_auto_fund_gas() -> bool {
    true
}

/// Runtime deposit system configuration (converted from DepositConfigFile)
#[derive(Debug, Clone)]
pub struct DepositConfig {
    /// Whether custodial deposit system is enabled
    pub enabled: bool,
    /// Polling interval for deposit monitoring in seconds
    pub poll_interval_secs: u64,
    /// Whether to automatically fund gas for deposit addresses
    pub auto_fund_gas: bool,
    /// Supported ERC20 token addresses (parsed from strings)
    pub supported_tokens: Vec<EthAddress>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeNodeConfig {
    /// The port that the server listens on.
    pub server_listen_port: u16,
    /// The port that for metrics server.
    pub metrics_port: u16,
    /// Path of the file where bridge authority key (Secp256k1) is stored.
    pub bridge_authority_key_path: PathBuf,
    /// Whether to run client. If true, `mys.bridge_client_key_path`
    /// and `db_path` needs to be provided.
    pub run_client: bool,
    /// Path of the client storage. Required when `run_client` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_path: Option<PathBuf>,
    /// A list of approved governance actions. Action in this list will be signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
    /// Mys configuration
    pub mys: MysConfig,
    /// Eth configuration
    pub eth: EthConfig,
    /// Network key used for metrics pushing
    #[serde(default = "default_ed25519_key_pair")]
    pub metrics_key_pair: NetworkKeyPair,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<MetricsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchdog_config: Option<WatchdogConfig>,
    
    /// Auto-relay configuration for automatic token claiming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay: Option<RelayConfigFile>,
    
    /// Deposit system configuration for custodial deposit addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposits: Option<DepositConfigFile>,
}

pub fn default_ed25519_key_pair() -> NetworkKeyPair {
    get_key_pair_from_rng(&mut rand::rngs::OsRng).1
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetricsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_interval_seconds: Option<u64>,
    pub push_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WatchdogConfig {
    /// Total supplies to watch on Mys. Mapping from coin name to coin type tag
    pub total_supplies: BTreeMap<String, String>,
}

impl Config for BridgeNodeConfig {}

impl BridgeNodeConfig {
    pub async fn validate(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(BridgeServerConfig, Option<BridgeClientConfig>)> {
        if !is_route_valid(
            BridgeChainId::try_from(self.mys.mys_bridge_chain_id)?,
            BridgeChainId::try_from(self.eth.eth_bridge_chain_id)?,
        ) {
            return Err(anyhow!(
                "Route between Mys chain id {} and Eth chain id {} is not valid",
                self.mys.mys_bridge_chain_id,
                self.eth.eth_bridge_chain_id,
            ));
        };

        let bridge_authority_key = match read_key(&self.bridge_authority_key_path, true)? {
            MysKeyPair::Secp256k1(key) => key,
            _ => unreachable!("we required secp256k1 key in `read_key`"),
        };

        // we do this check here instead of `prepare_for_mys` below because
        // that is only called when `run_client` is true.
        let mys_client =
            Arc::new(MysClient::<MysSdkClient>::new(&self.mys.mys_rpc_url, metrics.clone()).await?);
        let bridge_committee = mys_client
            .get_bridge_committee()
            .await
            .map_err(|e| anyhow!("Error getting bridge committee: {:?}", e))?;
        if !bridge_committee.is_active_member(&bridge_authority_key.public().into()) {
            return Err(anyhow!(
                "Bridge authority key is not part of bridge committee"
            ));
        }

        let (eth_client, eth_contracts) = self.prepare_for_eth(metrics.clone()).await?;
        let bridge_summary = mys_client
            .get_bridge_summary()
            .await
            .map_err(|e| anyhow!("Error getting bridge summary: {:?}", e))?;
        if bridge_summary.chain_id != self.mys.mys_bridge_chain_id {
            anyhow::bail!(
                "Bridge chain id mismatch: expected {}, but connected to {}",
                self.mys.mys_bridge_chain_id,
                bridge_summary.chain_id
            );
        }

        // Validate approved actions that must be governace actions
        for action in &self.approved_governance_actions {
            if !action.is_governace_action() {
                anyhow::bail!(format!(
                    "{:?}",
                    BridgeError::ActionIsNotGovernanceAction(action.clone())
                ));
            }
        }
        let approved_governance_actions = self.approved_governance_actions.clone();

        let bridge_server_config = BridgeServerConfig {
            key: bridge_authority_key,
            metrics_port: self.metrics_port,
            eth_bridge_proxy_address: eth_contracts[0], // the first contract is bridge proxy
            server_listen_port: self.server_listen_port,
            mys_client: mys_client.clone(),
            eth_client: eth_client.clone(),
            approved_governance_actions,
        };
        if !self.run_client {
            return Ok((bridge_server_config, None));
        }

        // If client is enabled, prepare client config
        let (bridge_client_key, client_mys_address, gas_object_ref) =
            self.prepare_for_mys(mys_client.clone(), metrics).await?;

        let db_path = self
            .db_path
            .clone()
            .ok_or(anyhow!("`db_path` is required when `run_client` is true"))?;

        // Convert relay config from file format to runtime format
        let relay_config = self.relay.as_ref().map(|relay_cfg| {
            // Warn if deposits are enabled but relay is disabled
            if self.deposits.as_ref().map(|d| d.enabled).unwrap_or(false) && !relay_cfg.enabled {
                tracing::warn!(
                    "Deposits are enabled but relay is disabled. \
                     MySocial → EVM deposits will not be automatically bridged. \
                     Consider enabling relay in config to enable automatic token claiming."
                );
            }
            
            let evm_config = relay_cfg.evm.as_ref().map(|evm_cfg| {
                if evm_cfg.enabled {
                    // Validate that eth config has RPC URL and bridge address
                    if self.eth.eth_rpc_url.is_empty() {
                        tracing::warn!(
                            "EVM relay enabled but eth.eth-rpc-url is empty. \
                             Relay will use ETH_RPC_URL environment variable if available."
                        );
                    }
                    if self.eth.eth_bridge_proxy_address.is_empty() {
                        tracing::warn!(
                            "EVM relay enabled but eth.eth-bridge-proxy-address is empty. \
                             Relay will use ETH_BRIDGE_PROXY_ADDRESS environment variable if available."
                        );
                    }
                }
                
                let rpc_url = evm_cfg.rpc_url.clone()
                    .unwrap_or_else(|| self.eth.eth_rpc_url.clone());
                let bridge_address = evm_cfg.bridge_contract_address.clone()
                    .unwrap_or_else(|| self.eth.eth_bridge_proxy_address.clone());
                
                crate::relay::EvmRelayConfig {
                    enabled: evm_cfg.enabled,
                    rpc_url,
                    bridge_contract_address: bridge_address
                        .parse()
                        .expect("Invalid bridge contract address in config"),
                    max_gas_price_gwei: evm_cfg.max_gas_price_gwei,
                    gas_estimate_buffer_percent: evm_cfg.gas_estimate_buffer_percent,
                    confirmation_blocks: evm_cfg.confirmation_blocks,
                }
            });
            
            crate::relay::RelayConfig {
                enabled: relay_cfg.enabled,
                max_retries: relay_cfg.max_retries,
                retry_delay_seconds: relay_cfg.retry_delay_seconds,
                mys_gas_budget: relay_cfg.mys.gas_budget,
                evm: evm_config,
            }
        });

        // Convert deposit config from file format to runtime format
        let deposit_config = self.deposits.as_ref().map(|deposit_cfg| {
            // Parse supported token addresses if provided
            let supported_tokens = match &deposit_cfg.supported_tokens {
                Some(tokens) => tokens
                    .iter()
                    .filter_map(|addr_str| {
                        addr_str.parse::<EthAddress>().ok()
                    })
                    .collect(),
                None => Vec::new(), // Will be queried from BridgeConfig contract
            };
            
            // Check for environment variable override
            let poll_interval_secs = std::env::var("DEPOSIT_POLL_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(deposit_cfg.poll_interval_secs);
            
            DepositConfig {
                enabled: deposit_cfg.enabled,
                poll_interval_secs,
                auto_fund_gas: deposit_cfg.auto_fund_gas,
                supported_tokens,
            }
        });

        let bridge_client_config = BridgeClientConfig {
            mys_address: client_mys_address,
            key: bridge_client_key,
            gas_object_ref,
            metrics_port: self.metrics_port,
            mys_client: mys_client.clone(),
            eth_client: eth_client.clone(),
            db_path,
            eth_contracts,
            // in `prepare_for_eth` we check if this is None when `run_client` is true. Safe to unwrap here.
            eth_contracts_start_block_fallback: self
                .eth
                .eth_contracts_start_block_fallback
                .unwrap(),
            eth_contracts_start_block_override: self.eth.eth_contracts_start_block_override,
            mys_bridge_module_last_processed_event_id_override: self
                .mys
                .mys_bridge_module_last_processed_event_id_override,
            relay_config,
            deposit_config,
        };

        Ok((bridge_server_config, Some(bridge_client_config)))
    }

    async fn prepare_for_eth(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(Arc<EthClient<MeteredEthHttpProvier>>, Vec<EthAddress>)> {
        let bridge_proxy_address = EthAddress::from_str(&self.eth.eth_bridge_proxy_address)?;
        let provider = Arc::new(
            new_metered_eth_provider(&self.eth.eth_rpc_url, metrics.clone())
                .unwrap()
                .interval(std::time::Duration::from_millis(2000)),
        );
        let chain_id = provider.get_chainid().await?;
        let (
            committee_address,
            limiter_address,
            vault_address,
            config_address,
            _weth_address,
            _usdt_address,
            _wbtc_address,
        ) = get_eth_contract_addresses(bridge_proxy_address, &provider).await?;
        let config = EthBridgeConfig::new(config_address, provider.clone());

        if self.run_client && self.eth.eth_contracts_start_block_fallback.is_none() {
            return Err(anyhow!(
                "eth_contracts_start_block_fallback is required when run_client is true"
            ));
        }

        // If bridge chain id is Eth Mainent or Sepolia, we expect to see chain
        // identifier to match accordingly.
        let bridge_chain_id: u8 = config.chain_id().call().await?;
        if self.eth.eth_bridge_chain_id != bridge_chain_id {
            return Err(anyhow!(
                "Bridge chain id mismatch: expected {}, but connected to {}",
                self.eth.eth_bridge_chain_id,
                bridge_chain_id
            ));
        }
        if bridge_chain_id == BridgeChainId::EthMainnet as u8 && chain_id.as_u64() != 1 {
            anyhow::bail!(
                "Expected Eth chain id 1, but connected to {}",
                chain_id.as_u64()
            );
        }
        if bridge_chain_id == BridgeChainId::EthSepolia as u8 && chain_id.as_u64() != 11155111 {
            anyhow::bail!(
                "Expected Eth chain id 11155111, but connected to {}",
                chain_id.as_u64()
            );
        }
        info!(
            "Connected to Eth chain: {}, Bridge chain id: {}",
            chain_id.as_u64(),
            bridge_chain_id,
        );

        let eth_client = Arc::new(
            EthClient::<MeteredEthHttpProvier>::new(
                &self.eth.eth_rpc_url,
                HashSet::from_iter(vec![
                    bridge_proxy_address,
                    committee_address,
                    config_address,
                    limiter_address,
                    vault_address,
                ]),
                metrics,
            )
            .await?,
        );
        let contract_addresses = vec![
            bridge_proxy_address,
            committee_address,
            config_address,
            limiter_address,
            vault_address,
        ];
        Ok((eth_client, contract_addresses))
    }

    async fn prepare_for_mys(
        &self,
        mys_client: Arc<MysClient<MysSdkClient>>,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(MysKeyPair, MysAddress, ObjectRef)> {
        let bridge_client_key = match &self.mys.bridge_client_key_path {
            None => read_key(&self.bridge_authority_key_path, true),
            Some(path) => read_key(path, false),
        }?;

        // If bridge chain id is Mys Mainent or Testnet, we expect to see chain
        // identifier to match accordingly.
        let mys_identifier = mys_client
            .get_chain_identifier()
            .await
            .map_err(|e| anyhow!("Error getting chain identifier from Mys: {:?}", e))?;
        if self.mys.mys_bridge_chain_id == BridgeChainId::MysMainnet as u8
            && mys_identifier != get_mainnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected mys chain identifier {}, but connected to {}",
                self.mys.mys_bridge_chain_id,
                mys_identifier
            );
        }
        if self.mys.mys_bridge_chain_id == BridgeChainId::MysTestnet as u8
            && mys_identifier != get_testnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected mys chain identifier {}, but connected to {}",
                self.mys.mys_bridge_chain_id,
                mys_identifier
            );
        }
        info!(
            "Connected to Mys chain: {}, Bridge chain id: {}",
            mys_identifier, self.mys.mys_bridge_chain_id,
        );

        let client_mys_address = MysAddress::from(&bridge_client_key.public());

        let gas_object_id = match self.mys.bridge_client_gas_object {
            Some(id) => id,
            None => {
                let mys_client = MysClientBuilder::default()
                    .build(&self.mys.mys_rpc_url)
                    .await?;
                let coin =
                    // Minimum balance for gas object is 10 MYS
                    pick_highest_balance_coin(mys_client.coin_read_api(), client_mys_address, 10_000_000_000)
                        .await?;
                coin.coin_object_id
            }
        };
        let (gas_coin, gas_object_ref, owner) = mys_client
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await;
        if owner != Owner::AddressOwner(client_mys_address) {
            return Err(anyhow!("Gas object {:?} is not owned by bridge client key's associated mys address {:?}, but {:?}", gas_object_id, client_mys_address, owner));
        }
        let balance = gas_coin.value();
        metrics.gas_coin_balance.set(balance as i64);
        info!(
            "Starting bridge client with address: {:?}, gas object {:?}, balance: {}",
            client_mys_address, gas_object_ref.0, balance,
        );

        Ok((bridge_client_key, client_mys_address, gas_object_ref))
    }
}

pub struct BridgeServerConfig {
    pub key: BridgeAuthorityKeyPair,
    pub server_listen_port: u16,
    pub eth_bridge_proxy_address: EthAddress,
    pub metrics_port: u16,
    pub mys_client: Arc<MysClient<MysSdkClient>>,
    pub eth_client: Arc<EthClient<MeteredEthHttpProvier>>,
    /// A list of approved governance actions. Action in this list will be signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
}

pub struct BridgeClientConfig {
    pub mys_address: MysAddress,
    pub key: MysKeyPair,
    pub gas_object_ref: ObjectRef,
    pub metrics_port: u16,
    pub mys_client: Arc<MysClient<MysSdkClient>>,
    pub eth_client: Arc<EthClient<MeteredEthHttpProvier>>,
    pub db_path: PathBuf,
    pub eth_contracts: Vec<EthAddress>,
    // See `BridgeNodeConfig` for the explanation of following two fields.
    pub eth_contracts_start_block_fallback: u64,
    pub eth_contracts_start_block_override: Option<u64>,
    pub mys_bridge_module_last_processed_event_id_override: Option<EventID>,
    /// Auto-relay configuration
    pub relay_config: Option<crate::relay::RelayConfig>,
    /// Deposit system configuration
    pub deposit_config: Option<DepositConfig>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeCommitteeConfig {
    pub bridge_authority_port_and_key_path: Vec<(u64, PathBuf)>,
}

impl Config for BridgeCommitteeConfig {}

pub async fn pick_highest_balance_coin(
    coin_read_api: &CoinReadApi,
    address: MysAddress,
    minimal_amount: u64,
) -> anyhow::Result<Coin> {
    let mut highest_balance = 0;
    let mut highest_balance_coin = None;
    coin_read_api
        .get_coins_stream(address, None)
        .for_each(|coin: Coin| {
            if coin.balance > highest_balance {
                highest_balance = coin.balance;
                highest_balance_coin = Some(coin.clone());
            }
            future::ready(())
        })
        .await;
    if highest_balance_coin.is_none() {
        return Err(anyhow!("No Mys coins found for address {:?}", address));
    }
    if highest_balance < minimal_amount {
        return Err(anyhow!(
            "Found no single coin that has >= {} balance Mys for address {:?}",
            minimal_amount,
            address,
        ));
    }
    Ok(highest_balance_coin.unwrap())
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EthContractAddresses {
    pub mys_bridge: EthAddress,
    pub bridge_committee: EthAddress,
    pub bridge_config: EthAddress,
    pub bridge_limiter: EthAddress,
    pub bridge_vault: EthAddress,
}
