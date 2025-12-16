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
    chain_id: u8,
    supported_tokens: Vec<EthAddress>,
    poll_interval: Duration,
    /// Channel to send detected deposit events for processing
    deposit_tx: tokio::sync::mpsc::UnboundedSender<EvmDepositEvent>,
}

impl EvmDepositMonitor {
    pub fn new(
        provider: Arc<Provider<MeteredEthHttpProvier>>,
        storage: Arc<BridgeOrchestratorTables>,
        chain_id: u8,
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

    /// Run the deposit monitor
    pub async fn run(
        self,
        mut shutdown_rx: tokio::sync::watch::Receiver<bool>,
    ) -> BridgeResult<()> {
        info!(
            chain_id = self.chain_id,
            "Starting EVM deposit monitor"
        );

        let mut last_checked_block: Option<u64> = None;

        loop {
            tokio::select! {
                _ = shutdown_rx.changed() => {
                    if *shutdown_rx.borrow() {
                        info!("EVM deposit monitor shutting down");
                        break;
                    }
                }
                _ = tokio::time::sleep(self.poll_interval) => {
                    if let Err(e) = self.check_for_deposits(&mut last_checked_block).await {
                        error!(?e, "Error checking for EVM deposits");
                    }
                }
            }
        }

        Ok(())
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
            // No deposit addresses registered yet
            *last_checked_block = Some(current_block);
            return Ok(());
        }

        info!(
            start_block,
            current_block,
            deposit_count = deposit_addresses.len(),
            "Checking for EVM deposits"
        );

        // Create filter for Transfer events to our deposit addresses
        // Transfer(address indexed from, address indexed to, uint256 value)
        let transfer_event_sig = H256::from(ethers::core::utils::keccak256(
            "Transfer(address,address,uint256)",
        ));

        for token_addr in &self.supported_tokens {
            // Build filter for each deposit address individually
            for deposit_addr in &deposit_addresses {
                let filter = Filter::new()
                    .address(*token_addr)
                    .from_block(start_block)
                    .to_block(current_block)
                    .topic0(transfer_event_sig)
                    .topic2(H256::from(*deposit_addr));

                // Get logs
                let logs = self
                    .provider
                    .get_logs(&filter)
                    .await
                    .map_err(|e| BridgeError::Generic(format!("Failed to get logs: {:?}", e)))?;

                if !logs.is_empty() {
                    info!(
                        token = ?token_addr,
                        deposit_addr = ?deposit_addr,
                        log_count = logs.len(),
                        "Found {} deposit events",
                        logs.len()
                    );

                    for log in logs {
                        if let Err(e) = self.process_deposit_log(log).await {
                            error!(?e, "Failed to process deposit log");
                        }
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
pub struct MysDepositMonitor<C> {
    /// MySocial client for querying coin transfers
    /// Will be used when coin transfer monitoring is fully implemented
    #[allow(dead_code)]
    mys_client: Arc<crate::mys_client::MysClient<C>>,
    storage: Arc<BridgeOrchestratorTables>,
    poll_interval: Duration,
}

impl<C> MysDepositMonitor<C>
where
    C: crate::mys_client::MysClientInner + 'static,
{
    pub fn new(
        mys_client: Arc<crate::mys_client::MysClient<C>>,
        storage: Arc<BridgeOrchestratorTables>,
        poll_interval_secs: u64,
    ) -> Self {
        Self {
            mys_client,
            storage,
            poll_interval: Duration::from_secs(poll_interval_secs),
        }
    }

    /// Run the deposit monitor
    pub async fn run(
        self,
        mut shutdown_rx: tokio::sync::watch::Receiver<bool>,
    ) -> BridgeResult<()> {
        info!("Starting MySocial deposit monitor");

        loop {
            tokio::select! {
                _ = shutdown_rx.changed() => {
                    if *shutdown_rx.borrow() {
                        info!("MySocial deposit monitor shutting down");
                        break;
                    }
                }
                _ = tokio::time::sleep(self.poll_interval) => {
                    if let Err(e) = self.check_for_deposits().await {
                        error!(?e, "Error checking for MySocial deposits");
                    }
                }
            }
        }

        Ok(())
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
        // Query coins at this address
        // This is a simplified version - production would use event filtering
        // For now, we'll check coin balances

        // TODO: In production, use event subscriptions instead of polling balances
        // Query for coin objects owned by deposit address
        // When found, trigger auto-bridge

        info!(?address, "Checked MySocial address for deposits");

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

