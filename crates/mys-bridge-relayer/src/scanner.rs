// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel_async::AsyncConnection;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::RunQueryDsl as AsyncRunQueryDsl;
use ethers::prelude::*;
use ethers::types::{Address as EthAddress, Filter, H256, U256};
use fastcrypto::hash::{HashFunction, Keccak256};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::address_index::AddressIndex;
use crate::config::EvmChainConfig;
use crate::models::{EvmDeposit, EvmScannerProgress};
use crate::postgres_manager::PgPool;
use crate::schema::{evm_deposits, evm_scanner_progress};

/// ERC20 Transfer event signature: Transfer(address,address,uint256)
/// Topic: 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
const ERC20_TRANSFER_TOPIC: H256 = H256([
    0xddu8, 0xf2u8, 0x52u8, 0xadu8, 0x1bu8, 0xe2u8, 0xc8u8, 0x9bu8,
    0x69u8, 0xc2u8, 0xb0u8, 0x68u8, 0xfcu8, 0x37u8, 0x8du8, 0xaau8,
    0x95u8, 0x2bu8, 0xa7u8, 0xf1u8, 0x63u8, 0xc4u8, 0xa1u8, 0x16u8,
    0x28u8, 0xf5u8, 0x5au8, 0x4du8, 0xf5u8, 0x23u8, 0xb3u8, 0xefu8,
]);

/// Maximum block range per eth_getLogs query (to avoid RPC limits).
const MAX_LOG_QUERY_RANGE: u64 = 1000;

/// Compute canonical asset_id: keccak256(chain_name || token_kind || token_address_or_zero)
pub fn compute_asset_id(chain_name: &str, token_kind: &str, token_address: Option<&EthAddress>) -> [u8; 32] {
    let mut preimage = Vec::new();
    preimage.extend_from_slice(chain_name.as_bytes());
    preimage.extend_from_slice(token_kind.as_bytes());
    if let Some(addr) = token_address {
        preimage.extend_from_slice(addr.as_bytes());
    } else {
        preimage.extend_from_slice(&[0u8; 20]);
    }
    Keccak256::digest(&preimage).digest
}

/// Compute deposit_hash: keccak256(chain_name || tx_hash || log_index)
pub fn compute_deposit_hash(chain_name: &str, tx_hash: &H256, log_index: i32) -> [u8; 32] {
    let mut preimage = Vec::new();
    preimage.extend_from_slice(chain_name.as_bytes());
    preimage.extend_from_slice(tx_hash.as_bytes());
    preimage.extend_from_slice(&log_index.to_be_bytes());
    Keccak256::digest(&preimage).digest
}

/// EVM block/log scanner for a single chain.
pub struct EvmScanner {
    config: EvmChainConfig,
    provider: Arc<Provider<Http>>,
    pool: PgPool,
    address_index: AddressIndex,
}

impl EvmScanner {
    pub fn new(config: EvmChainConfig, pool: PgPool, address_index: AddressIndex) -> Result<Self> {
        let provider = Provider::<Http>::try_from(&config.rpc_url)
            .map_err(|e| anyhow!("Failed to create EVM provider: {e}"))?;
        Ok(Self {
            config,
            provider: Arc::new(provider),
            pool,
            address_index,
        })
    }

    /// Main scanner loop: poll new blocks and detect deposits.
    pub async fn run(&mut self) -> Result<()> {
        info!(
            chain = %self.config.chain_name,
            genesis_block = self.config.genesis_block,
            "Starting EVM scanner"
        );

        loop {
            match self.scan_cycle().await {
                Ok(_) => {}
                Err(e) => {
                    error!(chain = %self.config.chain_name, error = %e, "Scanner cycle failed");
                    sleep(Duration::from_secs(5)).await;
                }
            }
            sleep(Duration::from_secs(2)).await; // Poll every 2 seconds
        }
    }

    async fn scan_cycle(&mut self) -> Result<()> {
        // 1. Get current scanner progress
        let mut conn = self.pool.get().await?;
        let progress: Option<EvmScannerProgress> = evm_scanner_progress::table
            .filter(evm_scanner_progress::chain_name.eq(&self.config.chain_name))
            .filter(evm_scanner_progress::scanner_name.eq("main"))
            .first(&mut conn)
            .await
            .optional()?;

        let start_block = progress
            .as_ref()
            .map(|p| p.last_scanned_block as u64 + 1)
            .unwrap_or(self.config.genesis_block);

        // 2. Get latest block
        let latest_block = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| anyhow!("Failed to get latest block: {e}"))?
            .as_u64();

        if start_block > latest_block {
            debug!(
                chain = %self.config.chain_name,
                start_block,
                latest_block,
                "No new blocks to scan"
            );
            return Ok(());
        }

        // 3. Determine scan range (respect max query range)
        let end_block = std::cmp::min(start_block + MAX_LOG_QUERY_RANGE - 1, latest_block);
        let finalized_block = latest_block.saturating_sub(self.config.required_confirmations);

        info!(
            chain = %self.config.chain_name,
            start_block,
            end_block,
            finalized_block,
            "Scanning block range"
        );

        // 4. Scan native ETH transfers
        self.scan_native_eth(start_block, end_block, finalized_block)
            .await?;

        // 5. Scan ERC20 Transfer events
        self.scan_erc20_transfers(start_block, end_block, finalized_block)
            .await?;

        // 6. Update scanner progress
        let mut conn = self.pool.get().await?;
        diesel::insert_into(evm_scanner_progress::table)
            .values((
                evm_scanner_progress::chain_name.eq(&self.config.chain_name),
                evm_scanner_progress::scanner_name.eq("main"),
                evm_scanner_progress::last_scanned_block.eq(end_block as i64),
                evm_scanner_progress::last_finalized_block.eq(finalized_block as i64),
            ))
            .on_conflict((
                evm_scanner_progress::chain_name,
                evm_scanner_progress::scanner_name,
            ))
            .do_update()
            .set((
                evm_scanner_progress::last_scanned_block.eq(end_block as i64),
                evm_scanner_progress::last_finalized_block.eq(finalized_block as i64),
            ))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Scan blocks for native ETH transfers (tx.to matches deposit address).
    async fn scan_native_eth(
        &self,
        start_block: u64,
        end_block: u64,
        finalized_block: u64,
    ) -> Result<()> {
        let mut conn = self.pool.get().await?;

        for block_num in start_block..=end_block {
            let block = self
                .provider
                .get_block_with_txs(block_num)
                .await
                .map_err(|e| anyhow!("Failed to get block {block_num}: {e}"))?
                .ok_or_else(|| anyhow!("Block {block_num} not found"))?;

            for tx in block.transactions {
                // Native ETH transfer: check if tx.to matches a deposit address
                if let Some(to) = tx.to {
                    let to_bytes: [u8; 20] = to.0;
                    if let Some(&mys_address) = self.address_index.lookup(&to_bytes) {
                        // Found a deposit!
                        let amount_wei = tx.value;
                        let tx_hash = tx.hash();

                        let asset_id = compute_asset_id(&self.config.chain_name, "native", None);
                        let deposit_hash = compute_deposit_hash(&self.config.chain_name, &tx_hash, -1);

                        let status = if block_num <= finalized_block {
                            "finalized"
                        } else {
                            "observed"
                        };

                        // Insert deposit (ignore if already exists due to UNIQUE constraint)
                        let result = diesel::insert_into(evm_deposits::table)
                            .values((
                                evm_deposits::chain_name.eq(&self.config.chain_name),
                                evm_deposits::asset_id.eq(asset_id.as_slice()),
                                evm_deposits::token_kind.eq("native"),
                                evm_deposits::token_address.eq::<Option<Vec<u8>>>(None),
                                evm_deposits::tx_hash.eq(tx_hash.as_bytes()),
                                evm_deposits::log_index.eq(-1),
                                evm_deposits::block_number.eq(block_num as i64),
                                evm_deposits::from_address.eq(tx.from.map(|a| a.as_bytes().to_vec())),
                                evm_deposits::to_address.eq(to_bytes.as_slice()),
                                evm_deposits::mys_address.eq(mys_address.as_slice()),
                                evm_deposits::amount_wei.eq(BigDecimal::from_u128(amount_wei.as_u128()).unwrap()),
                                evm_deposits::deposit_hash.eq(deposit_hash.as_slice()),
                                evm_deposits::status.eq(status),
                            ))
                            .on_conflict((
                                evm_deposits::chain_name,
                                evm_deposits::tx_hash,
                                evm_deposits::log_index,
                            ))
                            .do_nothing()
                            .execute(&mut conn)
                            .await?;

                        if result > 0 {
                            info!(
                                chain = %self.config.chain_name,
                                tx_hash = %tx_hash,
                                amount_wei = %amount_wei,
                                mys_address = %hex::encode(mys_address),
                                "Detected native ETH deposit"
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Scan ERC20 Transfer events using eth_getLogs.
    async fn scan_erc20_transfers(
        &self,
        start_block: u64,
        end_block: u64,
        finalized_block: u64,
    ) -> Result<()> {
        let mut conn = self.pool.get().await?;
        use diesel_async::RunQueryDsl as _;

        // Build filter: Transfer(address,address,uint256) events
        // Transfer event topic: 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
        let mut filter = Filter::new()
            .from_block(start_block)
            .to_block(end_block)
            .topic0(ERC20_TRANSFER_TOPIC);

        // If token allowlist is set, filter by contract addresses
        if !self.config.token_allowlist.is_empty() {
            let addresses: Result<Vec<EthAddress>, _> = self
                .config
                .token_allowlist
                .iter()
                .map(|s| s.parse::<EthAddress>())
                .collect();
            filter = filter.address(addresses?);
        }

        let logs = self
            .provider
            .get_logs(&filter)
            .await
            .map_err(|e| anyhow!("Failed to get ERC20 Transfer logs: {e}"))?;

        for log in logs {
            // Parse Transfer(address indexed from, address indexed to, uint256 value)
            if log.topics.len() != 3 {
                continue; // Invalid Transfer event
            }

            let from = EthAddress::from_slice(&log.topics[1].as_bytes()[12..]);
            let to = EthAddress::from_slice(&log.topics[2].as_bytes()[12..]);
            let amount = U256::from_big_endian(&log.data.0);

            // Check if 'to' matches a deposit address
            let to_bytes: [u8; 20] = to.0;
            if let Some(&mys_address) = self.address_index.lookup(&to_bytes) {
                let token_address = log.address;
                let tx_hash = log
                    .transaction_hash
                    .ok_or_else(|| anyhow!("Log missing transaction_hash"))?;
                let log_index = log
                    .log_index
                    .ok_or_else(|| anyhow!("Log missing log_index"))?
                    .as_u64() as i32;
                let block_number = log
                    .block_number
                    .ok_or_else(|| anyhow!("Log missing block_number"))?
                    .as_u64();

                let asset_id = compute_asset_id(&self.config.chain_name, "erc20", Some(&token_address));
                let deposit_hash = compute_deposit_hash(&self.config.chain_name, &tx_hash, log_index);

                let status = if block_number <= finalized_block {
                    "finalized"
                } else {
                    "observed"
                };

                // Insert deposit (ignore if already exists)
                let result = diesel::insert_into(evm_deposits::table)
                    .values((
                        evm_deposits::chain_name.eq(&self.config.chain_name),
                        evm_deposits::asset_id.eq(asset_id.as_slice()),
                        evm_deposits::token_kind.eq("erc20"),
                        evm_deposits::token_address.eq(Some(token_address.as_bytes().to_vec())),
                        evm_deposits::tx_hash.eq(tx_hash.as_bytes()),
                        evm_deposits::log_index.eq(log_index),
                        evm_deposits::block_number.eq(block_number as i64),
                        evm_deposits::from_address.eq(Some(from.as_bytes().to_vec())),
                        evm_deposits::to_address.eq(to_bytes.as_slice()),
                        evm_deposits::mys_address.eq(mys_address.as_slice()),
                        evm_deposits::amount_wei.eq(BigDecimal::from_u128(amount.as_u128()).unwrap()),
                        evm_deposits::deposit_hash.eq(deposit_hash.as_slice()),
                        evm_deposits::status.eq(status),
                    ))
                    .on_conflict((
                        evm_deposits::chain_name,
                        evm_deposits::tx_hash,
                        evm_deposits::log_index,
                    ))
                    .do_nothing()
                    .execute(&mut conn)
                    .await?;

                if result > 0 {
                    info!(
                        chain = %self.config.chain_name,
                        tx_hash = %tx_hash,
                        token = %token_address,
                        amount_wei = %amount,
                        mys_address = %hex::encode(mys_address),
                        "Detected ERC20 deposit"
                    );
                }
            }
        }

        Ok(())
    }
}
