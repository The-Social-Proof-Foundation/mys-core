// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deposit monitoring for custodial deposit addresses
//! Detects when users send tokens to deposit addresses and triggers auto-bridging

use crate::error::{BridgeError, BridgeResult};
use crate::metered_eth_provider::MeteredEthHttpProvier;
use crate::storage::BridgeOrchestratorTables;
use ethers::prelude::*;
use ethers::types::Address as EthAddress;
use mys_types::base_types::MysAddress;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

/// Event representing a deposit to an EVM deposit address
#[derive(Debug, Clone)]
pub struct EvmDepositEvent {
    pub tx_hash: H256,
    pub log_index: u64,
    pub block_number: u64,
    pub token_address: EthAddress,
    pub from_address: EthAddress,
    pub to_address: EthAddress, // Our deposit address
    pub amount: U256,
}

/// Event representing a deposit to a MySocial deposit address
#[derive(Debug, Clone)]
pub struct MysDepositEvent {
    pub tx_digest: mys_types::digests::TransactionDigest,
    pub sender: MysAddress,
    pub recipient: MysAddress, // Our deposit address
    pub coin_type: String,
    pub amount: u64,
    pub timestamp: u64,
}

/// Monitors EVM chains for deposits to our generated deposit addresses
pub struct EvmDepositMonitor {
    provider: Arc<Provider<MeteredEthHttpProvier>>,
    storage: Arc<BridgeOrchestratorTables>,
    chain_id: u64,
    supported_tokens: Vec<EthAddress>,
    poll_interval: Duration,
    /// Channel to send detected deposit events for processing
    deposit_tx: tokio::sync::mpsc::UnboundedSender<EvmDepositEvent>,
}

impl EvmDepositMonitor {
    pub fn new(
        provider: Arc<Provider<MeteredEthHttpProvier>>,
        storage: Arc<BridgeOrchestratorTables>,
        chain_id: u64,
        supported_tokens: Vec<EthAddress>,
        poll_interval_secs: u64,
        deposit_tx: tokio::sync::mpsc::UnboundedSender<EvmDepositEvent>,
    ) -> Self {
        Self {
            provider,
            storage,
            chain_id,
            supported_tokens,
            poll_interval: Duration::from_secs(poll_interval_secs),
            deposit_tx,
        }
    }

    /// Run the deposit monitor (follows BridgeWatchdog pattern)
    pub async fn run(self) -> BridgeResult<()> {
        info!(
            chain_id = self.chain_id,
            "Starting EVM deposit monitor"
        );

        let mut interval = tokio::time::interval(self.poll_interval);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        
        let mut last_checked_block: Option<u64> = None;

        loop {
            interval.tick().await;
            
            if let Err(e) = self.check_for_deposits(&mut last_checked_block).await {
                error!(?e, "Error checking for EVM deposits");
            }
        }
    }

    async fn check_for_deposits(&self, last_checked_block: &mut Option<u64>) -> BridgeResult<()> {
        // Get current block number
        let current_block = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| BridgeError::Generic(format!("Failed to get block number: {:?}", e)))?
            .as_u64();

        // Determine start block
        let start_block = last_checked_block.unwrap_or(current_block.saturating_sub(1000)); // Start from 1000 blocks ago initially

        if start_block >= current_block {
            return Ok(()); // Nothing new
        }

        // Get all active deposit addresses
        let deposit_addresses = self.storage.get_all_evm_deposit_addresses();

        if deposit_addresses.is_empty() {
            // No deposit addresses registered yet - skip this poll cycle
            info!(
                start_block,
                current_block,
                "No deposit addresses registered yet, skipping poll cycle"
            );
            *last_checked_block = Some(current_block);
            return Ok(());
        }

        info!(
            start_block,
            current_block,
            deposit_count = deposit_addresses.len(),
            "Checking for EVM deposits"
        );

        let transfer_event_sig = H256::from(ethers::core::utils::keccak256(
            "Transfer(address,address,uint256)",
        ));

        let deposit_addresses_set: std::collections::HashSet<EthAddress> = deposit_addresses.iter().copied().collect();

        for token_addr in &self.supported_tokens {
            let filter = Filter::new()
                .address(*token_addr)
                .from_block(start_block)
                .to_block(current_block)
                .topic0(transfer_event_sig);

            let logs = self
                .provider
                .get_logs(&filter)
                .await
                .map_err(|e| BridgeError::Generic(format!("Failed to get logs: {:?}", e)))?;

            let relevant_logs: Vec<_> = logs
                .into_iter()
                .filter(|log| {
                    if log.topics.len() < 3 {
                        return false;
                    }
                    let to_address = EthAddress::from(log.topics[2]);
                    deposit_addresses_set.contains(&to_address)
                })
                .collect();

            if !relevant_logs.is_empty() {
                info!(
                    token = ?token_addr,
                    log_count = relevant_logs.len(),
                    "Found {} deposit events",
                    relevant_logs.len()
                );

                for log in relevant_logs {
                    if let Err(e) = self.process_deposit_log(log).await {
                        error!(?e, "Failed to process deposit log");
                    }
                }
            }
        }

        *last_checked_block = Some(current_block);
        Ok(())
    }

    async fn process_deposit_log(&self, log: Log) -> BridgeResult<()> {
        // Parse Transfer event
        // topics[0] = Transfer event signature
        // topics[1] = from address (sender)
        // topics[2] = to address (our deposit address)
        // data = amount

        if log.topics.len() < 3 {
            return Err(BridgeError::Generic("Invalid Transfer event format".to_string()));
        }

        let from_address = EthAddress::from(log.topics[1]);
        let to_address = EthAddress::from(log.topics[2]);

        // Decode amount from data
        let amount = U256::from_big_endian(&log.data);

        let tx_hash = log
            .transaction_hash
            .ok_or_else(|| BridgeError::Generic("Log missing transaction hash".to_string()))?;

        let log_index = log
            .log_index
            .ok_or_else(|| BridgeError::Generic("Log missing log index".to_string()))?
            .as_u64();

        let block_number = log
            .block_number
            .ok_or_else(|| BridgeError::Generic("Log missing block number".to_string()))?
            .as_u64();

        let event = EvmDepositEvent {
            tx_hash,
            log_index,
            block_number,
            token_address: log.address,
            from_address,
            to_address,
            amount,
        };

        info!(
            tx_hash = ?event.tx_hash,
            to = ?event.to_address,
            amount = ?event.amount,
            "Detected EVM deposit, sending for processing"
        );

        // Send to auto-bridge handler via channel
        self.deposit_tx.send(event).map_err(|e| {
            BridgeError::Generic(format!("Failed to send deposit event: {:?}", e))
        })?;

        Ok(())
    }
}

/// Monitors MySocial chain for deposits to our generated deposit addresses
pub struct MysDepositMonitor {
    mys_client: Arc<crate::mys_client::MysBridgeClient>,
    storage: Arc<BridgeOrchestratorTables>,
    poll_interval: Duration,
    /// Channel to send detected deposit events for processing
    deposit_tx: tokio::sync::mpsc::UnboundedSender<MysDepositEvent>,
}

impl MysDepositMonitor {
    pub fn new(
        mys_client: Arc<crate::mys_client::MysBridgeClient>,
        storage: Arc<BridgeOrchestratorTables>,
        poll_interval_secs: u64,
        deposit_tx: tokio::sync::mpsc::UnboundedSender<MysDepositEvent>,
    ) -> Self {
        Self {
            mys_client,
            storage,
            poll_interval: Duration::from_secs(poll_interval_secs),
            deposit_tx,
        }
    }

    /// Run the deposit monitor (follows BridgeWatchdog pattern)
    pub async fn run(self) -> BridgeResult<()> {
        info!("Starting MySocial deposit monitor");

        let mut interval = tokio::time::interval(self.poll_interval);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            interval.tick().await;
            
            if let Err(e) = self.check_for_deposits().await {
                error!(?e, "Error checking for MySocial deposits");
            }
        }
    }

    async fn check_for_deposits(&self) -> BridgeResult<()> {
        // Get all active MySocial deposit addresses
        let deposit_addresses = self.storage.get_all_mys_deposit_addresses();

        if deposit_addresses.is_empty() {
            return Ok(()); // No deposit addresses yet
        }

        info!(
            deposit_count = deposit_addresses.len(),
            "Checking for MySocial deposits"
        );

        // For each deposit address, check for incoming coins
        for deposit_addr in deposit_addresses {
            if let Err(e) = self.check_address_for_coins(deposit_addr).await {
                error!(?deposit_addr, ?e, "Failed to check MySocial address for deposits");
            }
        }

        Ok(())
    }

    async fn check_address_for_coins(&self, address: MysAddress) -> BridgeResult<()> {
        // Query transactions sent to this deposit address
        // We'll check recent transactions and look for coin transfers
        
        let mys_sdk_client = self.mys_client.mys_client();
        
        // Query transactions for this address (as recipient)
        // Use ToAddress filter to find transactions where this address received coins
        let mut options = mys_json_rpc_types::MysTransactionBlockResponseOptions::full_content();
        options.show_balance_changes = true; // Need balance changes to detect deposits
        
        let transactions = mys_sdk_client
            .read_api()
            .query_transaction_blocks(
                mys_json_rpc_types::MysTransactionBlockResponseQuery {
                    filter: Some(mys_json_rpc_types::TransactionFilter::ToAddress(address)),
                    options: Some(options),
                },
                None, // cursor
                Some(50), // limit - check last 50 transactions
                false, // descending_order
            )
            .await
            .map_err(|e| BridgeError::Generic(format!("Failed to query transactions: {:?}", e)))?;

        for tx_block in transactions.data {
            let tx_digest = tx_block.digest;
            
            // Check if we've already processed this transaction
            let deposit_key = crate::storage::DepositTxKey::from_mys(tx_digest, 2);
            if self.storage.is_deposit_processed(&deposit_key)? {
                continue; // Already processed
            }

            // Parse transaction to find coin transfers to our deposit address
            if let Some(balance_changes) = &tx_block.balance_changes {
                for balance_change in balance_changes {
                    // Check if this is a positive balance change (coin received) for our address
                    let is_our_address = matches!(&balance_change.owner, mys_types::object::Owner::AddressOwner(addr) if *addr == address);
                    
                    if is_our_address && balance_change.amount > 0 {
                        // Extract coin type from balance change
                        let coin_type = balance_change.coin_type.clone();
                        
                        // Get sender from transaction
                        let sender = tx_block.transaction
                            .as_ref()
                            .map(|tx| tx.data.sender())
                            .copied()
                            .ok_or_else(|| BridgeError::Generic("Transaction missing sender".to_string()))?;

                        // Convert i128 amount to u64 (should be safe for positive values)
                        let amount_u64 = balance_change.amount
                            .try_into()
                            .map_err(|_| BridgeError::Generic(format!("Amount {} too large for u64", balance_change.amount)))?;

                        let event = MysDepositEvent {
                            tx_digest,
                            sender: MysAddress::from_bytes(sender.as_ref())
                                .map_err(|e| BridgeError::Generic(format!("Invalid sender address: {:?}", e)))?,
                            recipient: address,
                            coin_type: coin_type.to_string(),
                            amount: amount_u64,
                            timestamp: tx_block.timestamp_ms.unwrap_or(0),
                        };

                        info!(
                            tx_digest = ?event.tx_digest,
                            recipient = ?event.recipient,
                            amount = event.amount,
                            coin_type = event.coin_type,
                            "Detected MySocial deposit, sending for processing"
                        );

                        // Send to auto-bridge handler via channel
                        self.deposit_tx.send(event).map_err(|e| {
                            BridgeError::Generic(format!("Failed to send deposit event: {:?}", e))
                        })?;

                        // Only process one deposit per transaction
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_deposit_event_creation() {
        let event = EvmDepositEvent {
            tx_hash: H256::random(),
            log_index: 0,
            block_number: 1000,
            token_address: EthAddress::random(),
            from_address: EthAddress::random(),
            to_address: EthAddress::random(),
            amount: U256::from(1000),
        };

        assert_eq!(event.log_index, 0);
        assert_eq!(event.block_number, 1000);
    }
}

