// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Auto-bridge execution for custodial deposits
//! Handles deposits to our addresses and automatically calls bridge contracts
//!
//! ## Amount Handling
//!
//! ### EVM Deposits (EVM → MySocial)
//! - Uses `event.amount` (the amount from the Transfer event) when calling `bridgeERC20`
//! - The bridge contract calculates `amountTransfered` based on actual balance change (handles fee-on-transfer tokens)
//! - The contract then converts `amountTransfered` to MySocial decimals using `BridgeUtils.convertERC20ToMysDecimal()`:
//!   - If ERC20 decimals == MySocial decimals: no conversion (amount stays the same)
//!   - If ERC20 decimals > MySocial decimals: amount / (10 ** (erc20Decimal - mysDecimal))
//!   - Integer division truncation is expected and correct (precision loss in least significant digits)
//! - We verify the conversion formula is applied correctly and log precision loss to ensure no unexpected value loss
//!
//! ### MySocial Deposits (MySocial → EVM)
//! - Uses `event.amount` (the amount deposited to the deposit address)
//! - **IMPORTANT**: The Move bridge contract (`send_token`/`send_mys_token`) bridges the ENTIRE coin balance
//! - Therefore, we MUST split coins to get exactly `event.amount` if the coin balance exceeds it
//! - If coin balance == event.amount and we have a separate gas coin, we use the coin directly
//! - If coin balance > event.amount, we always split to get exactly event.amount before bridging

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
use mys_types::transaction::{Command, Argument, ObjectArg, Transaction, TransactionData};
use mys_types::{BRIDGE_PACKAGE_ID, TypeTag, MYS_FRAMEWORK_ADDRESS};
use shared_crypto::intent::{Intent, IntentMessage};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info, warn};

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
        info!(
            bridge_contract_address = ?self.eth_bridge_address,
            "Creating bridge contract instance for bridgeERC20 call"
        );
        let bridge = EthMysBridge::new(self.eth_bridge_address, Arc::new(signer));

        // Determine token ID from bridge config
        let token_id = self.get_token_id_for_address(event.token_address).await?;

        // CRITICAL: For fee-on-transfer tokens, the deposit address receives less than event.amount
        // We must use the actual balance, not event.amount, to avoid transaction failures
        // 
        // IMPORTANT: Query balance at the SPECIFIC BLOCK where the event occurred to avoid
        // including other deposits that happened later. This ensures we bridge exactly what
        // was received for THIS specific deposit event.
        let token_contract_for_balance = EthERC20::new(event.token_address, self.eth_provider.clone());
        
        // Query balance at the block where the deposit occurred (not current block)
        // This ensures we get the balance state exactly when this deposit happened
        use ethers::types::BlockId;
        let balance_call = token_contract_for_balance
            .balance_of(event.to_address)
            .block(BlockId::Number(ethers::types::BlockNumber::Number(event.block_number.into())));
        
        let actual_balance = balance_call
            .call()
            .await
            .map_err(|e| {
                BridgeError::Generic(format!(
                    "Failed to query deposit address balance at block {}: {:?}",
                    event.block_number, e
                ))
            })?;
        
        // CRITICAL PRODUCTION LOGIC:
        // For fee-on-transfer tokens: event.amount is what was SENT, but deposit address receives LESS
        // The balance at the event block reflects the state AFTER this transfer completed
        //
        // Key insight: We want to bridge exactly what was received for THIS deposit event.
        // - If actual_balance < event.amount: Fee-on-transfer token, use actual_balance (what was received)
        // - If actual_balance >= event.amount: Normal token, use event.amount (what was sent = what was received)
        //
        // Note: We query balance at the SPECIFIC BLOCK to get the exact state when this deposit occurred.
        // If balance > event.amount, it means there were previous deposits at that address (which should
        // have been bridged already due to sequential processing). We still use event.amount to bridge
        // only THIS deposit, not all deposits.
        let amount_to_bridge = if actual_balance < event.amount {
            // Fee-on-transfer token: deposit address received less than what was sent
            warn!(
                deposit_address = ?event.to_address,
                block_number = event.block_number,
                event_amount = ?event.amount,
                actual_balance_at_block = ?actual_balance,
                difference = ?(event.amount - actual_balance),
                "Fee-on-transfer token detected: deposit address received less than Transfer event amount. Bridging actual received amount."
            );
            actual_balance
        } else {
            // Normal token OR balance includes previous deposits
            // Use event.amount to bridge only THIS deposit (not previous deposits if any)
            if actual_balance > event.amount {
                info!(
                    deposit_address = ?event.to_address,
                    block_number = event.block_number,
                    event_amount = ?event.amount,
                    actual_balance_at_block = ?actual_balance,
                    "Balance exceeds event amount (may include previous deposits). Bridging only this deposit amount."
                );
            }
            event.amount
        };
        
        info!(
            deposit_address = ?event.to_address,
            event_amount = ?event.amount,
            actual_balance = ?actual_balance,
            amount_to_bridge = ?amount_to_bridge,
            "Determined amount to bridge (handles fee-on-transfer tokens)"
        );

        // Convert destination address to bytes (MySocial address = 32 bytes)
        let destination_bytes = recipient_info.destination_address.clone();
        
        if destination_bytes.len() != 32 {
            return Err(BridgeError::Generic(
                "Destination address must be 32 bytes for MySocial".to_string(),
            ));
        }

        // STEP 0: Estimate gas for approval transaction BEFORE checking balance
        // Note: We cannot estimate bridgeERC20 gas before approval exists (contract checks allowance)
        // So we'll estimate it after approval, using a conservative estimate for initial funding
        info!(
            token_address = ?event.token_address,
            bridge_address = ?self.eth_bridge_address,
            amount = ?event.amount,
            "Estimating gas for approval transaction"
        );

        let deposit_wallet_for_estimation = deposit_wallet.clone();
        let signer_for_estimation = SignerMiddleware::new(self.eth_provider.clone(), deposit_wallet_for_estimation);
        let token_contract = EthERC20::new(event.token_address, Arc::new(signer_for_estimation));

        // Estimate gas for approval (use amount_to_bridge, not event.amount)
        let approve_call = token_contract.approve(self.eth_bridge_address, amount_to_bridge);
        let approval_gas_estimate = approve_call.estimate_gas().await.map_err(|e| {
            BridgeError::Generic(format!("Failed to estimate gas for approval: {:?}", e))
        })?;
        let approval_gas_limit = (approval_gas_estimate.as_u64() * 120 / 100) as u64; // Add 20% buffer

        // Use conservative estimate for bridgeERC20 (will re-estimate after approval)
        // Typical bridgeERC20 calls use ~150k-200k gas, use 250k as conservative estimate
        const CONSERVATIVE_BRIDGE_GAS_LIMIT: u64 = 250_000;

        // Get current network gas price
        let gas_price = self.eth_provider.get_gas_price().await.map_err(|e| {
            BridgeError::Generic(format!("Failed to get gas price: {:?}", e))
        })?;

        info!(
            approval_gas_limit,
            bridge_gas_limit_estimate = CONSERVATIVE_BRIDGE_GAS_LIMIT,
            total_gas_limit = approval_gas_limit + CONSERVATIVE_BRIDGE_GAS_LIMIT,
            gas_price_wei = ?gas_price,
            gas_price_gwei = gas_price.as_u64() / 1_000_000_000,
            "Gas estimation complete (using conservative bridgeERC20 estimate)"
        );

        // CRITICAL: Ensure deposit address has gas using estimated approval gas + conservative bridge gas
        self.gas_manager
            .ensure_evm_deposit_has_gas_with_estimates(
                event.to_address,
                approval_gas_limit,
                CONSERVATIVE_BRIDGE_GAS_LIMIT,
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

        let approve_call = token_contract.approve(self.eth_bridge_address, amount_to_bridge);
        
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

        // STEP 2: Estimate gas for bridgeERC20 NOW that approval exists
        info!(
            token_id,
            amount = ?event.amount,
            "Estimating gas for bridgeERC20 (approval now exists)"
        );

        let bridge_call_for_estimation = bridge.bridge_erc20(
            token_id,
            amount_to_bridge,
            destination_bytes.clone().into(),
            recipient_info.destination_chain,
        );
        let bridge_gas_estimate = bridge_call_for_estimation.estimate_gas().await.map_err(|e| {
            BridgeError::Generic(format!("Failed to estimate gas for bridgeERC20: {:?}", e))
        })?;
        let bridge_gas_limit = (bridge_gas_estimate.as_u64() * 120 / 100) as u64; // Add 20% buffer

        info!(
            bridge_gas_limit,
            gas_price_gwei = gas_price.as_u64() / 1_000_000_000,
            "BridgeERC20 gas estimation complete"
        );

        // Ensure we have enough gas for the bridge transaction
        // Approval is already done, so we only need gas for bridgeERC20
        // Use 0 for approval_gas_limit since approval is complete
        self.gas_manager
            .ensure_evm_deposit_has_gas_with_estimates(
                event.to_address,
                0, // Approval already done
                bridge_gas_limit,
                gas_price,
            )
            .await?;

        // STEP 3: Call bridgeERC20(tokenID, amount, recipientAddress, destinationChainID)
        let destination_chain_id = recipient_info.destination_chain;

            info!(
                token_id,
                event_amount = ?event.amount,
                amount_to_bridge = ?amount_to_bridge,
                destination_chain = destination_chain_id,
                "Calling bridgeERC20 with actual balance (handles fee-on-transfer tokens)"
            );

        let call = bridge.bridge_erc20(
            token_id,
            amount_to_bridge,
            destination_bytes.into(),
            destination_chain_id,
        );
        
        info!(
            ?bridge_gas_limit,
            ?gas_price,
            gas_price_gwei = gas_price.as_u64() / 1_000_000_000,
            "Sending bridgeERC20 transaction with gas settings"
        );

        // Send transaction with gas settings
        let call_with_gas = call.gas(bridge_gas_limit).gas_price(gas_price);
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

            // Verify TokensDeposited event was emitted and extract amount
            use crate::abi::EthMysBridgeEvents;
            let bridge_contract = self.eth_bridge_address;
            
            let tokens_deposited_events: Vec<_> = receipt.logs
                .iter()
                .filter(|log| log.address == bridge_contract)
                .filter_map(|log| {
                    let raw_log = ethers::abi::RawLog {
                        topics: log.topics.clone(),
                        data: log.data.to_vec(),
                    };
                    EthMysBridgeEvents::decode_log(&raw_log).ok()
                })
                .filter_map(|event| {
                    if let EthMysBridgeEvents::TokensDepositedFilter(deposit_event) = event {
                        Some(deposit_event)
                    } else {
                        None
                    }
                })
                .collect();
            
            if tokens_deposited_events.is_empty() {
                error!(
                    tx_hash = ?tx_hash,
                    bridge_contract = ?bridge_contract,
                    total_logs = receipt.logs.len(),
                    bridge_logs = receipt.logs.iter().filter(|l| l.address == bridge_contract).count(),
                    "CRITICAL: TokensDeposited event not found in transaction receipt!"
                );
            } else {
                // Extract and verify the amount from the event
                for deposit_event in &tokens_deposited_events {
                    let mys_adjusted_amount = deposit_event.mys_adjusted_amount;
                    
                    // CRITICAL: Verify decimal conversion to ensure no value loss
                    // Fetch ERC20 decimals and MySocial decimals to verify conversion
                    let erc20_decimals = {
                        let token_contract = EthERC20::new(event.token_address, self.eth_provider.clone());
                        match token_contract.decimals().call().await {
                            Ok(decimals) => decimals,
                            Err(e) => {
                                warn!(
                                    ?tx_hash,
                                    token_address = ?event.token_address,
                                    error = ?e,
                                    "Failed to fetch ERC20 decimals for verification"
                                );
                                // Continue without verification if we can't fetch decimals
                                0u8
                            }
                        }
                    };
                    
                    let mys_decimals = {
                        let config_contract = EthBridgeConfig::new(
                            self.eth_bridge_config_address,
                            self.eth_provider.clone(),
                        );
                        match config_contract.token_mys_decimal_of(token_id).call().await {
                            Ok(decimals) => decimals,
                            Err(e) => {
                                warn!(
                                    ?tx_hash,
                                    token_id,
                                    error = ?e,
                                    "Failed to fetch MySocial decimals for verification"
                                );
                                // Continue without verification if we can't fetch decimals
                                0u8
                            }
                        }
                    };
                    
                    // Verify conversion if we successfully fetched both decimals
                    if erc20_decimals > 0 && mys_decimals > 0 {
                        // Calculate expected conversion using the same formula as BridgeUtils.convertERC20ToMysDecimal
                        // We use amount_to_bridge (actual balance) which matches what the contract uses (amountTransfered)
                        // For fee-on-transfer tokens, amount_to_bridge < event.amount, which is correct
                        let expected_mys_amount: u128 = if erc20_decimals == mys_decimals {
                            // Same decimals: no conversion needed
                            amount_to_bridge.as_u128()
                        } else if erc20_decimals > mys_decimals {
                            // Convert ERC20 to MySocial decimals by dividing
                            // Formula: amount / (10 ** (erc20Decimal - mysDecimal))
                            let factor = 10u128.pow((erc20_decimals - mys_decimals) as u32);
                            amount_to_bridge.as_u128() / factor
                        } else {
                            // This should never happen per contract validation, but handle it
                            warn!(
                                ?tx_hash,
                                erc20_decimals,
                                mys_decimals,
                                "Invalid decimal configuration: ERC20 decimals < MySocial decimals"
                            );
                            amount_to_bridge.as_u128()
                        };
                        
                        // The actual mys_adjusted_amount should match expected_mys_amount exactly
                        // (allowing for small precision loss from integer division)
                        if mys_adjusted_amount as u128 > expected_mys_amount {
                            error!(
                                ?tx_hash,
                                event_amount = ?event.amount,
                                amount_bridged_erc20 = ?amount_to_bridge,
                                erc20_decimals,
                                mys_decimals,
                                expected_mys_amount,
                                actual_mys_adjusted_amount = mys_adjusted_amount,
                                "CRITICAL: MySocial-adjusted amount exceeds expected conversion! Possible value loss or conversion error."
                            );
                        } else {
                            // Calculate precision loss (if any) due to integer division during conversion
                            // This only applies when converting from higher precision (ERC20) to lower precision (MySocial)
                            let (precision_loss, conversion_applied) = if erc20_decimals > mys_decimals {
                                let factor = 10u128.pow((erc20_decimals - mys_decimals) as u32);
                                let remainder = amount_to_bridge.as_u128() % factor;
                                (remainder, true)
                            } else {
                                (0, false)
                            };
                            
                            // Verify the conversion matches exactly (allowing for precision loss)
                            let conversion_matches = mys_adjusted_amount as u128 == expected_mys_amount;
                            
                            info!(
                                ?tx_hash,
                                event_amount = ?event.amount,
                                amount_bridged_erc20 = ?amount_to_bridge,
                                erc20_decimals,
                                mys_decimals,
                                expected_mys_amount,
                                actual_mys_adjusted_amount = mys_adjusted_amount,
                                conversion_matches,
                                conversion_applied,
                                precision_loss_due_to_division = precision_loss,
                                "Decimal conversion verified - bridge contract calculated MySocial-adjusted amount correctly"
                            );
                            
                            // Warn if conversion doesn't match (should only differ by precision loss)
                            if !conversion_matches {
                                let difference = if mys_adjusted_amount as u128 > expected_mys_amount {
                                    mys_adjusted_amount as u128 - expected_mys_amount
                                } else {
                                    expected_mys_amount - mys_adjusted_amount as u128
                                };
                                
                                // This should only happen due to precision loss from integer division
                                if difference != precision_loss {
                                    warn!(
                                        ?tx_hash,
                                        difference,
                                        precision_loss,
                                        "Conversion difference doesn't match expected precision loss - investigate"
                                    );
                                }
                            }
                            
                            // Warn if there's significant precision loss
                            if conversion_applied && precision_loss > 0 {
                                let precision_loss_percentage = (precision_loss as f64 / amount_to_bridge.as_u128() as f64) * 100.0;
                                if precision_loss_percentage > 0.01 {
                                    warn!(
                                        ?tx_hash,
                                        precision_loss,
                                        precision_loss_percentage,
                                        "Precision loss detected due to decimal conversion (this is expected for integer division when ERC20 decimals > MySocial decimals)"
                                    );
                                }
                            }
                        }
                    } else {
                        // Log without verification if we couldn't fetch decimals
                        info!(
                            ?tx_hash,
                            event_count = tokens_deposited_events.len(),
                            deposited_amount_erc20 = ?event.amount,
                            mys_adjusted_amount = mys_adjusted_amount,
                            token_id = deposit_event.token_id,
                            "TokensDeposited event verified (decimal verification skipped - could not fetch decimals)"
                        );
                    }
                }
            }

            // Mark as processed (store amount_to_bridge, which is what was actually bridged)
            self.storage.mark_deposit_processed(
                deposit_key,
                format!("{:?}", tx_hash),
                amount_to_bridge.to_string(),
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
        for token_id in 0u8..=20 {
            // Only check first 20 tokens for performance
            match config_contract.token_address_of(token_id).call().await {
                Ok(addr) if addr == token_address => {
                    if token_id != 0 {
                        warn!(
                            ?token_address,
                            token_id,
                            "⚠️  WARNING: BridgeConfig contract maps token address to token_id={}, but expected token_id=0. \
                             This is an EVM-side configuration issue. The BridgeConfig contract must be updated to map this \
                             token address to token_id=0. The bridge will use token_id={} as returned by the contract.",
                            token_id, token_id
                        );
                    }
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
        "Preparing to query coins for MySocial deposit"
    );

    // Get gas coin (must be different from bridge coin)
    // Always use MYS for gas
    let gas_coin_type_str = TypeTag::Struct(Box::new(StructTag {
        address: MYS_FRAMEWORK_ADDRESS.into(),
        module: ident_str!("mys").to_owned(),
        name: ident_str!("MYS").to_owned(),
        type_params: vec![],
    }))
    .to_string();

    // Get bridge object
    let bridge_object_arg = mys_client
        .get_mutable_bridge_object_arg_must_succeed()
        .await;

    // Query coins fresh right before building transaction to avoid stale data
    // This ensures we get the latest coin state after any previous transactions
    info!(
        ?deposit_mys_address,
        ?coin_type,
        amount = event.amount,
        "Querying coins for MySocial deposit"
    );

    let mys_sdk_client = mys_client.mys_client();
    let coin_type_str = coin_type.to_string();
    let coins = mys_sdk_client
        .coin_read_api()
        .get_coins(deposit_mys_address, Some(coin_type_str), None, None)
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

    // Get coin object reference immediately after querying to minimize stale data window
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
                "Coin object {} was consumed between query and transaction building. This can happen if the coin was used in another transaction. Please retry the deposit.",
                coin_to_bridge.coin_object_id
            ))
        })?
        .object_ref();

    // Get gas coin (must be different from bridge coin)
    // CRITICAL: We must always bridge exactly event.amount, not the entire coin balance
    // The Move bridge contract uses token.balance().value(), so we need to split if coin balance > event.amount
    let (gas_obj_ref, needs_split_for_gas, needs_split_for_amount) = match mys_sdk_client
        .coin_read_api()
        .select_coins(
            deposit_mys_address,
            Some(gas_coin_type_str),
            1_000_000_000, // 1 MIST minimum for gas
            vec![coin_to_bridge.coin_object_id], // Exclude the bridge coin
        )
        .await
    {
        Ok(gas_coins) => {
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
                bridge_coin_balance = coin_to_bridge.balance,
                required_amount = event.amount,
                "Selected separate gas coin"
            );
            
            // Check if we need to split the bridge coin to get exactly event.amount
            let needs_split = coin_to_bridge.balance > event.amount;
            if needs_split {
                info!(
                    coin_balance = coin_to_bridge.balance,
                    required_amount = event.amount,
                    "Bridge coin has more than required amount, will split to exact amount"
                );
            }
            (gas_obj_ref, false, needs_split)
        }
        Err(_) => {
            // No separate gas coin available - check if we can split the bridge coin
            if is_native_mys && coin_to_bridge.balance > event.amount {
                // We have native MYS coin with more than needed - split it for gas and amount
                // We'll split event.amount for bridging, and the remainder stays as gas coin
                info!(
                    coin_id = ?coin_to_bridge.coin_object_id,
                    coin_balance = coin_to_bridge.balance,
                    bridge_amount = event.amount,
                    remaining_for_gas = coin_to_bridge.balance - event.amount,
                    "No separate gas coin found, will split coin for gas and exact amount"
                );
                // Use the coin itself as gas, we'll split it in the transaction
                (coin_obj_ref, true, true)
            } else {
                // Coin has exactly event.amount or less, and no separate gas coin
                // Cannot proceed - we need gas but coin has exactly the bridge amount
                return Err(BridgeError::Generic(format!(
                    "No gas coin available. Bridge coin balance ({}) must be greater than required amount ({}) to split for gas. \
                     Please ensure deposit address has sufficient balance for both gas and bridge amount.",
                    coin_to_bridge.balance, event.amount
                )));
            }
        }
    };

    // Build transaction
    let mut builder = ProgrammableTransactionBuilder::new();
    
    // CRITICAL: Always split to exactly event.amount if coin balance > event.amount
    // The Move bridge contract bridges the entire coin balance, so we must ensure the coin has exactly event.amount
    let arg_token = if needs_split_for_amount {
        // Split the exact bridge amount from the coin
        // This creates a new coin with event.amount that we'll use for bridging
        let split_amount = event.amount;
        let split_amount_arg = builder.pure(split_amount)
            .map_err(|e| BridgeError::Generic(format!("Failed to create split amount argument: {:?}", e)))?;
        
        // Determine which coin to split from
        let coin_to_split_from = if needs_split_for_gas {
            // We're using the bridge coin as gas coin, split from GasCoin
            Argument::GasCoin
        } else {
            // We have a separate gas coin, split from the bridge coin object
            builder.obj(ObjectArg::ImmOrOwnedObject(coin_obj_ref))
                .map_err(|e| BridgeError::Generic(format!("Failed to create coin object argument: {:?}", e)))?
        };
        
        // Split coins to get exactly event.amount
        let split_result = builder.command(Command::SplitCoins(
            coin_to_split_from,
            vec![split_amount_arg],
        ));
        
        info!(
            split_amount,
            "Added SplitCoins command to create coin with exact bridge amount"
        );
        
        // Use the split coin result for bridging
        // split_result is Argument::Result(0) which refers to the split coin
        split_result
    } else if needs_split_for_gas {
        // We're using the bridge coin as gas coin and it has more than event.amount
        // Split event.amount for bridging, remainder stays as gas coin
        let split_amount = event.amount;
        let split_amount_arg = builder.pure(split_amount)
            .map_err(|e| BridgeError::Generic(format!("Failed to create split amount argument: {:?}", e)))?;
        
        let split_result = builder.command(Command::SplitCoins(
            Argument::GasCoin,
            vec![split_amount_arg],
        ));
        
        info!(
            split_amount,
            "Added SplitCoins command to separate gas and bridge amounts from gas coin"
        );
        
        split_result
    } else {
        // Coin has exactly event.amount and we have a separate gas coin - use coin directly
        info!(
            coin_balance = coin_to_bridge.balance,
            required_amount = event.amount,
            "Using coin with exact amount for bridging"
        );
        builder.obj(ObjectArg::ImmOrOwnedObject(coin_obj_ref))
            .map_err(|e| BridgeError::Generic(format!("Failed to create token argument: {:?}", e)))?
    };
    
    let arg_target_chain = builder
        .pure(recipient_info.destination_chain as u8)
        .map_err(|e| BridgeError::Generic(format!("Failed to create target_chain argument: {:?}", e)))?;
    let arg_target_address = builder
        .pure(destination_eth_address.as_bytes())
        .map_err(|e| BridgeError::Generic(format!("Failed to create target_address argument: {:?}", e)))?;
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

    // Verify that TokenDepositedEvent was emitted
    let events = response.events.ok_or_else(|| {
        BridgeError::Generic(format!(
            "Transaction succeeded but no events returned for deposit tx: {:?}",
            tx_digest
        ))
    })?;

    // Log all events for diagnostics
    info!(
        ?tx_digest,
        event_count = events.data.len(),
        event_types = ?events.data.iter().map(|e| &e.type_).collect::<Vec<_>>(),
        "Bridge transaction executed - analyzing events"
    );

    // Check for TokenDepositedEvent
    use crate::events::{init_all_struct_tags, MoveTokenDepositedEvent};
    use fastcrypto::encoding::{Encoding, Hex};
    init_all_struct_tags(); // Ensure tags are initialized
    // Access the static OnceCell directly - it's defined in events module
    let token_deposited_event_tag = {
        // Import the static by name - it's a static variable, not a type
        #[allow(non_upper_case_globals)]
        use crate::events::MysToEthTokenBridgeV1;
        MysToEthTokenBridgeV1.get().unwrap()
    };
    let token_deposited_event = events
        .data
        .iter()
        .find(|e| e.type_ == *token_deposited_event_tag);

    if let Some(event) = token_deposited_event {
        // Try to decode the event data for detailed logging
        // Use the same pattern as events.rs
        match bcs::from_bytes::<MoveTokenDepositedEvent>(event.bcs.bytes()) {
            Ok(event_data) => {
                info!(
                    ?tx_digest,
                    seq_num = event_data.seq_num,
                    source_chain = event_data.source_chain,
                    target_chain = event_data.target_chain,
                    token_type = event_data.token_type,
                    amount = event_data.amount_mys_adjusted,
                    sender = ?Hex::encode(&event_data.sender_address),
                    recipient = ?Hex::encode(&event_data.target_address),
                    "TokenDepositedEvent emitted successfully - will be picked up by orchestrator"
                );
            }
            Err(e) => {
                warn!(
                    ?tx_digest,
                    error = ?e,
                    "TokenDepositedEvent found but failed to decode event data"
                );
                info!(
                    ?tx_digest,
                    "TokenDepositedEvent emitted successfully - will be picked up by orchestrator"
                );
            }
        }
    } else {
        error!(
            ?tx_digest,
            event_types = ?events.data.iter().map(|e| &e.type_).collect::<Vec<_>>(),
            "TokenDepositedEvent not found in transaction events - bridge may not complete. \
             The orchestrator may not pick up this deposit for bridging to EVM."
        );
        // Don't fail here - the event might be picked up later by the syncer
        // But log a warning so we can diagnose
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

    #[test]
    fn test_gas_threshold_constants() {
        const MIN_GAS_BALANCE: u128 = 500_000_000;
        const FUND_AMOUNT: u128 = 1_000_000_000;

        assert!(FUND_AMOUNT > MIN_GAS_BALANCE);
        assert_eq!(MIN_GAS_BALANCE, 5u128 * 10u128.pow(8)); // 0.0000000005 ETH
    }
}

