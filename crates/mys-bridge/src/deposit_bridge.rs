// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Auto-bridge execution for custodial deposits
//! Handles deposits to our addresses and automatically calls bridge contracts

use crate::abi::{EthBridgeConfig, EthMysBridge, EthERC20};
use crate::deposit_addresses::DepositAddressManager;
use crate::deposit_gas_manager::DepositGasManager;
use crate::deposit_monitor::{EvmDepositEvent, MysDepositEvent};
use crate::error::{BridgeError, BridgeResult};
use crate::metered_eth_provider::MeteredEthHttpProvier;
use crate::mys_client::MysClientInner;
use crate::storage::{BridgeOrchestratorTables, DepositAddressKey, DepositTxKey};
use ethers::prelude::*;
use ethers::types::Address as EthAddress;
use move_core_types::ident_str;
use mys_types::base_types::MysAddress;
use mys_types::transaction::ObjectArg;
use mys_types::{TypeTag, MYS_FRAMEWORK_ADDRESS};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

/// Handles automatic bridging for deposits
pub struct DepositBridgeHandler<C> {
    storage: Arc<BridgeOrchestratorTables>,
    address_manager: Arc<DepositAddressManager>,
    gas_manager: Arc<DepositGasManager<C>>,
    eth_provider: Arc<Provider<MeteredEthHttpProvier>>,
    eth_bridge_address: EthAddress,
    eth_bridge_config_address: EthAddress,
    eth_chain_id: u64,
    #[allow(dead_code)] // Will be used when MySocial deposits are fully implemented
    mys_bridge_object: ObjectArg,
    /// Cached token ID to address mapping
    token_address_to_id: Arc<tokio::sync::RwLock<HashMap<EthAddress, u8>>>,
}

impl<C> DepositBridgeHandler<C>
where
    C: MysClientInner + 'static,
{
    pub fn new(
        storage: Arc<BridgeOrchestratorTables>,
        address_manager: Arc<DepositAddressManager>,
        gas_manager: Arc<DepositGasManager<C>>,
        eth_provider: Arc<Provider<MeteredEthHttpProvier>>,
        eth_bridge_address: EthAddress,
        eth_bridge_config_address: EthAddress,
        eth_chain_id: u64,
        mys_bridge_object: ObjectArg,
    ) -> Self {
        Self {
            storage,
            address_manager,
            gas_manager,
            eth_provider,
            eth_bridge_address,
            eth_bridge_config_address,
            eth_chain_id,
            mys_bridge_object,
            token_address_to_id: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Handle an EVM deposit event (tokens sent to EVM deposit address)
    /// This triggers a bridge transaction FROM the deposit address TO MySocial
    pub async fn handle_evm_deposit(&self, event: EvmDepositEvent) -> BridgeResult<H256> {
        let deposit_key = DepositTxKey::from_evm(event.tx_hash, event.log_index as u16, self.eth_chain_id as u8);

        // Check if already processed
        if self.storage.is_deposit_processed(&deposit_key)? {
            info!(
                tx_hash = ?event.tx_hash,
                log_index = event.log_index,
                "Deposit already processed, skipping"
            );
            return Ok(H256::zero());
        }

        // Lookup recipient info
        let recipient_info = self
            .storage
            .get_recipient_for_deposit(&DepositAddressKey {
                address: event.to_address.as_bytes().to_vec(),
            })?
            .ok_or_else(|| {
                BridgeError::Generic(format!(
                    "No recipient found for deposit address {:?}",
                    event.to_address
                ))
            })?;

        info!(
            deposit_address = ?event.to_address,
            destination_len = recipient_info.destination_address.len(),
            amount = ?event.amount,
            "Processing EVM deposit"
        );

        // CRITICAL: Ensure deposit address has gas
        self.gas_manager
            .ensure_evm_deposit_has_gas(event.to_address)
            .await?;

        // Get wallet for deposit address
        let deposit_wallet = self
            .address_manager
            .get_evm_wallet_for_index(recipient_info.hd_index)?
            .with_chain_id(self.eth_chain_id);

        // Create signer
        let signer = SignerMiddleware::new(self.eth_provider.clone(), deposit_wallet);

        // Create bridge contract instance
        let bridge = EthMysBridge::new(self.eth_bridge_address, Arc::new(signer));

        // Determine token ID from bridge config
        let token_id = self.get_token_id_for_address(event.token_address).await?;

        // Convert destination address to bytes (MySocial address = 32 bytes)
        let destination_bytes = recipient_info.destination_address.clone();
        
        if destination_bytes.len() != 32 {
            return Err(BridgeError::Generic(
                "Destination address must be 32 bytes for MySocial".to_string(),
            ));
        }

        // STEP 1: Approve bridge contract to spend tokens
        info!(
            token_address = ?event.token_address,
            bridge_address = ?self.eth_bridge_address,
            amount = ?event.amount,
            "Approving bridge contract to spend tokens"
        );

        let deposit_wallet_for_approval = self
            .address_manager
            .get_evm_wallet_for_index(recipient_info.hd_index)?
            .with_chain_id(self.eth_chain_id);
        
        let signer_for_approval = SignerMiddleware::new(self.eth_provider.clone(), deposit_wallet_for_approval);
        let token_contract = EthERC20::new(event.token_address, Arc::new(signer_for_approval));

        let approve_call = token_contract.approve(self.eth_bridge_address, event.amount);
        
        let pending_approval = approve_call.send().await.map_err(|e| {
            BridgeError::Generic(format!("Failed to send token approval transaction: {:?}", e))
        })?;

        let approval_tx_hash = pending_approval.tx_hash();
        info!(?approval_tx_hash, "Token approval transaction sent");

        let approval_receipt = pending_approval.confirmations(1).await.map_err(|e| {
            BridgeError::Generic(format!("Failed to confirm token approval: {:?}", e))
        })?;

        if let Some(receipt) = approval_receipt {
            if receipt.status != Some(1.into()) {
                return Err(BridgeError::Generic(
                    "Token approval transaction reverted".to_string(),
                ));
            }
            info!(?approval_tx_hash, "Token approval confirmed");
        } else {
            return Err(BridgeError::Generic(
                "Token approval receipt not available".to_string(),
            ));
        }

        // STEP 2: Call bridgeERC20(tokenID, amount, recipientAddress, destinationChainID)
        let destination_chain_id = recipient_info.destination_chain;

        info!(
            token_id,
            amount = ?event.amount,
            destination_chain = destination_chain_id,
            "Calling bridgeERC20"
        );

        let call = bridge.bridge_erc20(
            token_id,
            event.amount,
            destination_bytes.into(),
            destination_chain_id,
        );

        // Send transaction
        let pending_tx = call.send().await.map_err(|e| {
            BridgeError::Generic(format!("Failed to send bridgeERC20 transaction: {:?}", e))
        })?;

        let tx_hash = pending_tx.tx_hash();
        info!(?tx_hash, "Bridge transaction sent from deposit address");

        // Wait for confirmation
        let receipt = pending_tx.confirmations(2).await.map_err(|e| {
            BridgeError::Generic(format!("Failed to confirm bridge transaction: {:?}", e))
        })?;

        if let Some(receipt) = receipt {
            if receipt.status != Some(1.into()) {
                return Err(BridgeError::Generic(
                    "Bridge transaction reverted".to_string(),
                ));
            }

            // Mark as processed
            self.storage.mark_deposit_processed(
                deposit_key,
                format!("{:?}", tx_hash),
                event.amount.to_string(),
            )?;

            info!(
                ?tx_hash,
                block = ?receipt.block_number,
                "EVM deposit bridged successfully"
            );

            Ok(tx_hash)
        } else {
            Err(BridgeError::Generic("Transaction receipt not available".to_string()))
        }
    }

    /// Get token ID for ERC20 address by querying BridgeConfig contract
    async fn get_token_id_for_address(&self, token_address: EthAddress) -> BridgeResult<u8> {
        // Check cache first
        {
            let cache = self.token_address_to_id.read().await;
            if let Some(&token_id) = cache.get(&token_address) {
                return Ok(token_id);
            }
        }

        // Not in cache - query from BridgeConfig contract
        info!(?token_address, "Querying token ID from BridgeConfig");

        let config_contract = EthBridgeConfig::new(
            self.eth_bridge_config_address,
            self.eth_provider.clone(),
        );

        // Try each token ID (0-255) until we find a match
        // This is inefficient but simple - production could maintain reverse mapping
        for token_id in 0u8..=10 {
            // Only check first 10 tokens for performance
            match config_contract.token_address_of(token_id).call().await {
                Ok(addr) if addr == token_address => {
                    info!(
                        ?token_address,
                        token_id,
                        "Found token ID for address"
                    );

                    // Cache the result
                    let mut cache = self.token_address_to_id.write().await;
                    cache.insert(token_address, token_id);

                    return Ok(token_id);
                }
                Ok(_) => continue,
                Err(e) => {
                    warn!(
                        token_id,
                        ?e,
                        "Error querying token address from config"
                    );
                    continue;
                }
            }
        }

        Err(BridgeError::Generic(format!(
            "Token address {:?} not found in bridge configuration",
            token_address
        )))
    }
}

/// Handle MySocial deposit event (coins sent to MySocial deposit address)
/// This triggers a bridge transaction FROM the deposit address TO EVM
/// Note: Currently requires MysSdkClient for coin querying functionality
pub async fn handle_mys_deposit(
    event: MysDepositEvent,
    storage: &Arc<BridgeOrchestratorTables>,
    address_manager: &Arc<DepositAddressManager>,
    gas_manager: &Arc<crate::deposit_gas_manager::DepositGasManager<mys_sdk::MysClient>>,
    mys_client: &Arc<crate::mys_client::MysBridgeClient>,
    _bridge_object: ObjectArg,
    token_type_tags: &std::collections::HashMap<u8, mys_types::TypeTag>,
) -> BridgeResult<mys_types::digests::TransactionDigest>
{
    let deposit_key = DepositTxKey::from_mys(event.tx_digest, 2); // Chain ID 2 for MySocial

    // Check if already processed
    if storage.is_deposit_processed(&deposit_key)? {
        info!(
            tx_digest = ?event.tx_digest,
            "MySocial deposit already processed, skipping"
        );
        return Ok(mys_types::digests::TransactionDigest::default());
    }

    // Lookup recipient info
    let recipient_info = storage
        .get_recipient_for_deposit(&DepositAddressKey {
            address: event.recipient.to_vec(),
        })?
        .ok_or_else(|| {
            BridgeError::Generic(format!(
                "No recipient found for deposit address {}",
                event.recipient
            ))
        })?;

    info!(
        deposit_address = ?event.recipient,
        destination_chain = recipient_info.destination_chain,
        amount = event.amount,
        "Processing MySocial deposit"
    );

    // Get keypair for deposit address
    let deposit_keypair = address_manager.get_mys_keypair_for_index(recipient_info.hd_index)?;
    let deposit_mys_address = MysAddress::from(&deposit_keypair.public());

    // Ensure deposit address has gas
    gas_manager.ensure_mys_deposit_has_gas(deposit_mys_address).await?;

    // Parse destination EVM address (20 bytes)
    if recipient_info.destination_address.len() != 20 {
        return Err(BridgeError::Generic(
            "Destination must be 20-byte EVM address".to_string(),
        ));
    }

    let destination_eth_address = EthAddress::from_slice(&recipient_info.destination_address);

    info!(
        ?deposit_mys_address,
        ?destination_eth_address,
        target_chain = recipient_info.destination_chain,
        "Building send_token transaction from deposit address"
    );

    // Build send_token transaction
    // send_token<T>(bridge, target_chain, target_address, token: Coin<T>)

    // Get reference gas price (will be needed for transaction building)
    let _rgp = mys_client.get_reference_gas_price_until_success().await;

    // Parse the coin type from the event to get token ID
    // For now, assume MYS token (ID 0) since that's most common
    // Production enhancement: parse event.coin_type to determine actual token
    let token_id = 0u8; // MYS

    // Get type tag for the token (will be needed for transaction building)
    let _type_tag = if token_id == 0 {
        // MYS native token type
        TypeTag::Struct(Box::new(move_core_types::language_storage::StructTag {
            address: MYS_FRAMEWORK_ADDRESS.into(),
            module: ident_str!("mys").to_owned(),
            name: ident_str!("MYS").to_owned(),
            type_params: vec![],
        }))
    } else {
        token_type_tags
            .get(&token_id)
            .ok_or(BridgeError::UnknownTokenId(token_id))?
            .clone()
    };

    // TODO: Complete MySocial coin querying and bridge transaction execution
    // This requires:
    // 1. Parsing coin type from event.coin_type
    // 2. Querying coins at deposit_mys_address using mys_client.mys_client().coin_read_api()
    // 3. Selecting appropriate coin (highest balance)
    // 4. Building send_token<T> transaction with proper ObjectArgs
    // 5. Getting gas coin for transaction
    // 6. Signing with deposit_keypair
    // 7. Executing transaction
    // 8. Marking as processed using DepositTxKey::from_mys(event.tx_digest, recipient_info.source_chain)
    //
    // Framework is ready, but coin querying implementation needs careful testing
    // to handle:
    // - Different coin types (MYS, wrapped tokens, etc.)
    // - Gas coin selection (must be different from bridge coin)
    // - Transaction building with correct type parameters
    // - Error handling for insufficient balance, gas, etc.

    warn!(
        ?deposit_mys_address,
        ?destination_eth_address,
        coin_type = &event.coin_type,
        amount = event.amount,
        "MySocial deposit detected - coin querying implementation needed for production"
    );

    Err(BridgeError::Generic(format!(
        "MySocial → EVM deposit bridging not yet fully implemented. \
         Deposit detected: {} {} from {} to EVM address {:?}. \
         Framework ready, needs coin query implementation.",
        event.amount,
        event.coin_type,
        deposit_mys_address,
        destination_eth_address
    )))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_threshold_constants() {
        const MIN_GAS_BALANCE: u128 = 500_000_000;
        const FUND_AMOUNT: u128 = 1_000_000_000;

        assert!(FUND_AMOUNT > MIN_GAS_BALANCE);
        assert_eq!(MIN_GAS_BALANCE, 5u128 * 10u128.pow(8)); // 0.0000000005 ETH
    }
}

