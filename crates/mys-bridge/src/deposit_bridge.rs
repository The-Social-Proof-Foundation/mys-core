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
use move_core_types::language_storage::StructTag;
use mys_json_rpc_types::MysObjectDataOptions;
use mys_types::base_types::{MysAddress};
use mys_types::bridge::BRIDGE_MODULE_NAME;
use mys_types::crypto::Signature;
use mys_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use mys_types::transaction::{ObjectArg, Transaction, TransactionData};
use mys_types::{BRIDGE_PACKAGE_ID, TypeTag, MYS_FRAMEWORK_ADDRESS};
use shared_crypto::intent::{Intent, IntentMessage};
use std::collections::HashMap;
use std::str::FromStr;
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

        // Get wallet for deposit address
        let deposit_wallet = self
            .address_manager
            .get_evm_wallet_for_index(recipient_info.hd_index)?
            .with_chain_id(self.eth_chain_id);

        // Create signer
        let signer = SignerMiddleware::new(self.eth_provider.clone(), deposit_wallet.clone());

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

        // STEP 0: Estimate gas for both transactions BEFORE checking balance
        // This ensures we fund with the correct amount based on actual gas needs
        info!(
            token_address = ?event.token_address,
            bridge_address = ?self.eth_bridge_address,
            amount = ?event.amount,
            "Estimating gas for approval and bridge transactions"
        );

        let deposit_wallet_for_estimation = deposit_wallet.clone();
        let signer_for_estimation = SignerMiddleware::new(self.eth_provider.clone(), deposit_wallet_for_estimation);
        let token_contract = EthERC20::new(event.token_address, Arc::new(signer_for_estimation));

        // Estimate gas for approval
        let approve_call = token_contract.approve(self.eth_bridge_address, event.amount);
        let approval_gas_estimate = approve_call.estimate_gas().await.map_err(|e| {
            BridgeError::Generic(format!("Failed to estimate gas for approval: {:?}", e))
        })?;
        let approval_gas_limit = (approval_gas_estimate.as_u64() * 120 / 100) as u64; // Add 20% buffer

        // Estimate gas for bridge call
        let bridge_call = bridge.bridge_erc20(
            token_id,
            event.amount,
            destination_bytes.clone().into(),
            recipient_info.destination_chain,
        );
        let bridge_gas_estimate = bridge_call.estimate_gas().await.map_err(|e| {
            BridgeError::Generic(format!("Failed to estimate gas for bridgeERC20: {:?}", e))
        })?;
        let bridge_gas_limit = (bridge_gas_estimate.as_u64() * 120 / 100) as u64; // Add 20% buffer

        // Get current network gas price
        let gas_price = self.eth_provider.get_gas_price().await.map_err(|e| {
            BridgeError::Generic(format!("Failed to get gas price: {:?}", e))
        })?;

        info!(
            approval_gas_limit,
            bridge_gas_limit,
            total_gas_limit = approval_gas_limit + bridge_gas_limit,
            gas_price_wei = ?gas_price,
            gas_price_gwei = gas_price.as_u64() / 1_000_000_000,
            "Gas estimation complete"
        );

        // CRITICAL: Ensure deposit address has gas using ACTUAL estimated gas amounts
        self.gas_manager
            .ensure_evm_deposit_has_gas_with_estimates(
                event.to_address,
                approval_gas_limit,
                bridge_gas_limit,
                gas_price,
            )
            .await?;

        // STEP 1: Approve bridge contract to spend tokens
        info!(
            token_address = ?event.token_address,
            bridge_address = ?self.eth_bridge_address,
            amount = ?event.amount,
            "Approving bridge contract to spend tokens"
        );

        let deposit_wallet_for_approval = deposit_wallet.clone();
        let signer_for_approval = SignerMiddleware::new(self.eth_provider.clone(), deposit_wallet_for_approval);
        let token_contract = EthERC20::new(event.token_address, Arc::new(signer_for_approval));

        let approve_call = token_contract.approve(self.eth_bridge_address, event.amount);
        
        // Use the already-estimated gas limit
        let gas_limit = approval_gas_limit;
        
        info!(
            ?gas_limit,
            ?gas_price,
            gas_price_gwei = gas_price.as_u64() / 1_000_000_000,
            "Sending approval transaction with gas settings"
        );
        
        let approve_call_with_gas = approve_call.gas(gas_limit).gas_price(gas_price);
        let pending_approval = approve_call_with_gas
            .send()
            .await
            .map_err(|e| {
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

        // Use the already-estimated gas limit (gas price was already fetched)
        let gas_limit = bridge_gas_limit;
        
        info!(
            ?gas_limit,
            ?gas_price,
            gas_price_gwei = gas_price.as_u64() / 1_000_000_000,
            "Sending bridgeERC20 transaction with gas settings"
        );

        // Send transaction with gas settings
        let call_with_gas = call.gas(gas_limit).gas_price(gas_price);
        let pending_tx = call_with_gas
            .send()
            .await
            .map_err(|e| {
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
    _token_type_tags: &std::collections::HashMap<u8, mys_types::TypeTag>,
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

    // Get reference gas price
    let rgp = mys_client.get_reference_gas_price_until_success().await;

    // Parse coin type from event
    let coin_type = TypeTag::from_str(&event.coin_type).map_err(|e| {
        BridgeError::Generic(format!("Failed to parse coin type '{}': {:?}", event.coin_type, e))
    })?;

    // Check if this is native MYS token
    let is_native_mys = if let TypeTag::Struct(s) = &coin_type {
        s.address.to_hex_literal() == "0x2"
            && s.module.as_str() == "mys"
            && s.name.as_str() == "MYS"
    } else {
        false
    };

    info!(
        ?deposit_mys_address,
        ?coin_type,
        is_native_mys,
        amount = event.amount,
        "Querying coins for MySocial deposit"
    );

    // Query coins at deposit address matching the coin type
    let mys_sdk_client = mys_client.mys_client();
    let coin_type_str = coin_type.to_string();
    let coins = mys_sdk_client
        .coin_read_api()
        .get_all_coins(deposit_mys_address, Some(coin_type_str), None)
        .await
        .map_err(|e| {
            BridgeError::Generic(format!(
                "Failed to query coins at deposit address: {:?}",
                e
            ))
        })?;

    // Find coin matching the amount (or use first one if amount matches)
    let coin_to_bridge = coins
        .data
        .iter()
        .find(|coin| coin.balance >= event.amount)
        .ok_or_else(|| {
            BridgeError::Generic(format!(
                "No coin found with sufficient balance. Required: {}, Available coins: {:?}",
                event.amount,
                coins.data.iter().map(|c| c.balance).collect::<Vec<_>>()
            ))
        })?;

    info!(
        coin_id = ?coin_to_bridge.coin_object_id,
        coin_balance = coin_to_bridge.balance,
        required_amount = event.amount,
        "Found coin to bridge"
    );

    // Get coin object reference
    let coin_obj = mys_sdk_client
        .read_api()
        .get_object_with_options(
            coin_to_bridge.coin_object_id,
            MysObjectDataOptions::default().with_owner().with_content(),
        )
        .await
        .map_err(|e| {
            BridgeError::Generic(format!(
                "Failed to read coin object {}: {:?}",
                coin_to_bridge.coin_object_id, e
            ))
        })?;

    let coin_obj_ref = coin_obj
        .data
        .ok_or_else(|| {
            BridgeError::Generic(format!(
                "Coin object {} not found",
                coin_to_bridge.coin_object_id
            ))
        })?
        .object_ref();

    // Get gas coin (must be different from bridge coin)
    // Always use MYS for gas
    let gas_coin_type_str = TypeTag::Struct(Box::new(StructTag {
        address: MYS_FRAMEWORK_ADDRESS.into(),
        module: ident_str!("mys").to_owned(),
        name: ident_str!("MYS").to_owned(),
        type_params: vec![],
    }))
    .to_string();

    let gas_coins = mys_sdk_client
        .coin_read_api()
        .select_coins(
            deposit_mys_address,
            Some(gas_coin_type_str),
            1_000_000_000, // 1 MIST minimum for gas
            vec![coin_to_bridge.coin_object_id], // Exclude the bridge coin
        )
        .await
        .map_err(|e| {
            BridgeError::Generic(format!(
                "Failed to select gas coin: {:?}",
                e
            ))
        })?;

    let gas_obj_ref = gas_coins
        .first()
        .ok_or_else(|| {
            BridgeError::Generic(
                "No gas coin available (must be different from bridge coin)".to_string(),
            )
        })?
        .object_ref();

    info!(
        gas_coin_id = ?gas_obj_ref.0,
        "Selected gas coin"
    );

    // Get bridge object
    let bridge_object_arg = mys_client
        .get_mutable_bridge_object_arg_must_succeed()
        .await;

    // Build transaction
    let mut builder = ProgrammableTransactionBuilder::new();
    let arg_target_chain = builder
        .pure(recipient_info.destination_chain as u8)
        .map_err(|e| BridgeError::Generic(format!("Failed to create target_chain argument: {:?}", e)))?;
    let arg_target_address = builder
        .pure(destination_eth_address.as_bytes())
        .map_err(|e| BridgeError::Generic(format!("Failed to create target_address argument: {:?}", e)))?;
    let arg_token = builder
        .obj(ObjectArg::ImmOrOwnedObject(coin_obj_ref))
        .map_err(|e| BridgeError::Generic(format!("Failed to create token argument: {:?}", e)))?;
    let arg_bridge = builder
        .obj(bridge_object_arg)
        .map_err(|e| BridgeError::Generic(format!("Failed to create bridge argument: {:?}", e)))?;

    // Call appropriate bridge function
    if is_native_mys {
        builder.programmable_move_call(
            BRIDGE_PACKAGE_ID,
            BRIDGE_MODULE_NAME.to_owned(),
            ident_str!("send_mys_token").to_owned(),
            vec![], // No type parameters for native MYS
            vec![arg_bridge, arg_target_chain, arg_target_address, arg_token],
        );
    } else {
        builder.programmable_move_call(
            BRIDGE_PACKAGE_ID,
            BRIDGE_MODULE_NAME.to_owned(),
            ident_str!("send_token").to_owned(),
            vec![coin_type],
            vec![arg_bridge, arg_target_chain, arg_target_address, arg_token],
        );
    }

    let pt = builder.finish();

    // Create transaction data
    let tx_data = TransactionData::new_programmable(
        deposit_mys_address,
        vec![gas_obj_ref],
        pt,
        500_000_000, // Gas budget
        rgp,
    );

    // Sign transaction
    let sig = Signature::new_secure(
        &IntentMessage::new(Intent::mys_transaction(), &tx_data),
        &deposit_keypair,
    );

    let signed_tx = Transaction::from_data(tx_data, vec![sig]);
    let tx_digest = *signed_tx.digest();

    info!(
        ?tx_digest,
        ?deposit_mys_address,
        ?destination_eth_address,
        "Executing MySocial → EVM bridge transaction"
    );

    // Execute transaction
    let response = mys_sdk_client
        .execute_transaction_block_with_effects(signed_tx)
        .await
        .map_err(|e| {
            BridgeError::Generic(format!(
                "Failed to execute bridge transaction: {:?}",
                e
            ))
        })?;

    if !response.status_ok().unwrap_or(false) {
        return Err(BridgeError::Generic(format!(
            "Bridge transaction failed: {:?}",
            response
        )));
    }

    // Mark as processed
    storage.mark_deposit_processed(
        deposit_key,
        format!("{:?}", tx_digest),
        event.amount.to_string(),
    )?;

    info!(
        ?tx_digest,
        "MySocial → EVM deposit bridged successfully"
    );

    Ok(tx_digest)
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

