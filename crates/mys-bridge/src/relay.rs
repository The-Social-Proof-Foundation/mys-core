// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Auto-relay module that automatically claims approved bridge transfers
//! This enables seamless cross-chain transfers without manual claiming

use crate::error::{BridgeError, BridgeResult};
use crate::events::{
    TokenTransferAlreadyClaimed, TokenTransferClaimed, TokenTransferLimitExceed,
};
use crate::mys_client::{MysClient, MysClientInner};
use crate::storage::{BridgeOrchestratorTables, RelayKey, RelayResult};
use crate::types::{BridgeAction, ParsedTokenTransferMessage};
use mys_types::base_types::MysAddress;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

// EVM-specific imports
use ethers::prelude::*;
use ethers::types::Address as EthAddress;
use crate::abi::EthMysBridge;
use crate::crypto::BridgeAuthorityPublicKeyBytes;
use fastcrypto::traits::{KeyPair as KeyPairTrait, ToFromBytes};

/// Relay configuration
#[derive(Debug, Clone)]
pub struct RelayConfig {
    /// Whether auto-relay is enabled
    pub enabled: bool,
    /// Maximum number of retry attempts
    pub max_retries: u8,
    /// Delay between retries in seconds
    pub retry_delay_seconds: u64,
    /// Maximum gas budget for Mys transactions (in MIST)
    pub mys_gas_budget: u64,
    /// EVM relay configuration
    pub evm: Option<EvmRelayConfig>,
}

/// EVM-specific relay configuration
#[derive(Debug, Clone)]
pub struct EvmRelayConfig {
    /// Whether EVM relay is enabled
    pub enabled: bool,
    /// EVM RPC URL
    pub rpc_url: String,
    /// Bridge contract address on EVM
    pub bridge_contract_address: EthAddress,
    /// Maximum gas price in Gwei
    pub max_gas_price_gwei: u64,
    /// Percentage buffer for gas estimation (e.g., 20 = 20% buffer)
    pub gas_estimate_buffer_percent: u8,
    /// Number of confirmations to wait for
    pub confirmation_blocks: u64,
}

impl Default for RelayConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_retries: 3,
            retry_delay_seconds: 30,
            mys_gas_budget: 100_000_000, // 0.1 MYS
            evm: None, // EVM relay disabled by default
        }
    }
}

/// Auto-relay service that monitors approved transfers and automatically claims them
pub struct BridgeRelayer<C> {
    mys_client: Arc<MysClient<C>>,
    store: Arc<BridgeOrchestratorTables>,
    config: RelayConfig,
    key: mys_types::crypto::MysKeyPair,
    mys_address: mys_types::base_types::MysAddress,
    gas_object_id: mys_types::base_types::ObjectID,
    mys_token_type_tags: std::sync::Arc<arc_swap::ArcSwap<std::collections::HashMap<u8, mys_types::TypeTag>>>,
    // EVM relay fields (optional - only if EVM relay is configured)
    eth_provider: Option<Arc<Provider<Http>>>,
    eth_bridge_address: Option<EthAddress>,
    eth_chain_id: Option<u64>,
}

/// Convert a secp256k1 key pair to an Ethereum wallet
pub fn secp256k1_to_eth_wallet(
    secp_key: &fastcrypto::secp256k1::Secp256k1KeyPair,
) -> BridgeResult<Wallet<k256::ecdsa::SigningKey>> {
    // Get private key bytes (32 bytes for secp256k1)
    let privkey_bytes = secp_key.copy().private().as_bytes().to_vec();
    
    // Create ethers SigningKey from bytes
    let signing_key = k256::ecdsa::SigningKey::from_slice(&privkey_bytes)
        .map_err(|e| BridgeError::Generic(format!("Failed to create SigningKey: {:?}", e)))?;
    
    // Create wallet
    let wallet = Wallet::from(signing_key);
    
    // Verify derived address matches expected
    let expected_addr = BridgeAuthorityPublicKeyBytes::from(&secp_key.public).to_eth_address();
    if wallet.address() != expected_addr {
        return Err(BridgeError::Generic(format!(
            "Ethereum address mismatch! Derived: {:?}, Expected: {:?}",
            wallet.address(),
            expected_addr
        )));
    }
    
    info!(
        eth_address = ?wallet.address(),
        mys_pubkey = ?BridgeAuthorityPublicKeyBytes::from(&secp_key.public),
        "Successfully converted secp256k1 key to Ethereum wallet"
    );
    
    Ok(wallet)
}

impl<C> BridgeRelayer<C>
where
    C: MysClientInner + 'static,
{
    pub async fn new(
        mys_client: Arc<MysClient<C>>,
        store: Arc<BridgeOrchestratorTables>,
        config: RelayConfig,
        key: mys_types::crypto::MysKeyPair,
        mys_address: mys_types::base_types::MysAddress,
        gas_object_id: mys_types::base_types::ObjectID,
        mys_token_type_tags: std::sync::Arc<arc_swap::ArcSwap<std::collections::HashMap<u8, mys_types::TypeTag>>>,
    ) -> BridgeResult<Self> {
        // Initialize EVM client if configured
        let (eth_provider, eth_bridge_address, eth_chain_id) = if let Some(evm_config) = &config.evm {
            if evm_config.enabled {
                // Validate key is secp256k1
                if !matches!(&key, mys_types::crypto::MysKeyPair::Secp256k1(_)) {
                    return Err(BridgeError::Generic(format!(
                        "EVM relay requires secp256k1 key, but got: {:?}",
                        key.public().scheme()
                    )));
                }
                
                // Create provider
                let provider = Provider::<Http>::try_from(evm_config.rpc_url.as_str())
                    .map_err(|e| BridgeError::Generic(format!("Failed to create provider: {:?}", e)))?
                    .interval(std::time::Duration::from_millis(2000));
                
                // Get chain ID
                let chain_id = provider
                    .get_chainid()
                    .await
                    .map_err(|e| BridgeError::Generic(format!("Failed to get chain ID: {:?}", e)))?
                    .as_u64();
                
                let provider_arc = Arc::new(provider);
                
                // Derive EVM wallet address and check balance
                let secp_key = match &key {
                    mys_types::crypto::MysKeyPair::Secp256k1(k) => k,
                    _ => unreachable!("Already validated above"),
                };
                let wallet = secp256k1_to_eth_wallet(secp_key)?;
                let eth_address = wallet.address();
                let balance = provider_arc
                    .get_balance(eth_address, None)
                    .await
                    .map_err(|e| BridgeError::Generic(format!("Failed to get EVM balance: {:?}", e)))?;
                
                info!(
                    chain_id = %chain_id,
                    bridge_contract = ?evm_config.bridge_contract_address,
                    evm_wallet = ?eth_address,
                    balance_eth = ?(balance.as_u64() as f64 / 1e18),
                    "EVM relayer initialized successfully"
                );
                
                // Warn if balance is low
                let balance_eth = balance.as_u64() as f64 / 1e18;
                if balance_eth < 0.1 {
                    warn!(
                        evm_wallet = ?eth_address,
                        balance_eth = balance_eth,
                        "EVM wallet balance is low - please fund with at least 0.5 ETH for reliable relay operation"
                    );
                }
                
                (
                    Some(provider_arc),
                    Some(evm_config.bridge_contract_address),
                    Some(chain_id),
                )
            } else {
                info!("EVM relay is disabled in configuration");
                (None, None, None)
            }
        } else {
            info!("No EVM relay configuration provided");
            (None, None, None)
        };
        
        Ok(Self {
            mys_client,
            store,
            config,
            key,
            mys_address,
            gas_object_id,
            mys_token_type_tags,
            eth_provider,
            eth_bridge_address,
            eth_chain_id,
        })
    }

    /// Check if a transfer should be auto-relayed after approval
    /// This is called after TokenTransferApproved event is detected
    pub async fn handle_approved_transfer(&self, action: &BridgeAction) -> BridgeResult<()> {
        if !self.config.enabled {
            warn!("Relay is disabled in configuration, skipping auto-relay");
            return Ok(());
        }

        let relay_key = RelayKey::new(action.chain_id() as u8, action.seq_number());

        // Check if already relayed
        if self.store.is_relayed(&relay_key)? {
            info!(?relay_key, "Transfer already relayed, skipping");
            return Ok(());
        }

        info!(
            ?relay_key,
            mys_address = ?self.mys_address,
            gas_object_id = ?self.gas_object_id,
            "Auto-relaying approved transfer"
        );

        match action {
            BridgeAction::EthToMysBridgeAction(eth_action) => {
                // EVM → MySocial: Call claim_and_transfer_token on MySocial
                self.relay_to_mys(relay_key, eth_action).await
            }
            BridgeAction::MysToEthBridgeAction(mys_action) => {
                // MySocial → EVM: Call transferBridgedTokensWithSignatures on EVM
                self.relay_to_evm(relay_key, mys_action).await
            }
            _ => {
                // Not a token transfer action
                Ok(())
            }
        }
    }

    /// Relay an approved EVM → MySocial transfer by calling claim_and_transfer_token
    async fn relay_to_mys(
        &self,
        relay_key: RelayKey,
        _action: &crate::types::EthToMysBridgeAction,
    ) -> BridgeResult<()> {
        info!(?relay_key, "Relaying to MySocial");

        // Record pending status
        self.store.record_relay(
            relay_key,
            String::from("pending"),
            RelayResult::Pending,
            None,
        )?;

        // Build and execute claim_and_transfer transaction
        match self.build_and_execute_claim_tx(relay_key).await {
            Ok(tx_digest) => {
                info!(?relay_key, ?tx_digest, "Successfully relayed to MySocial");
                self.store.record_relay(
                    relay_key,
                    tx_digest.to_string(),
                    RelayResult::Success,
                    None,
                )?;
                Ok(())
            }
            Err(e) => {
                error!(?relay_key, ?e, "Failed to relay to MySocial");
                self.store.record_relay(
                    relay_key,
                    String::from("failed"),
                    RelayResult::Failed,
                    Some(format!("{:?}", e)),
                )?;
                Err(e)
            }
        }
    }

    /// Build and execute a claim_and_transfer transaction on MySocial
    async fn build_and_execute_claim_tx(
        &self,
        relay_key: RelayKey,
    ) -> BridgeResult<mys_types::digests::TransactionDigest> {
        use move_core_types::ident_str;
        use mys_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
        use mys_types::transaction::{CallArg, TransactionData};
        use mys_types::BRIDGE_PACKAGE_ID;

        // Get the token transfer message details to determine token type
        let message = self
            .mys_client
            .get_parsed_token_transfer_message(relay_key.source_chain, relay_key.seq_num)
            .await?
            .ok_or_else(|| {
                BridgeError::Generic(format!(
                    "Message not found for relay: chain={}, seq={}",
                    relay_key.source_chain, relay_key.seq_num
                ))
            })?;

        let token_id = message.parsed_payload.token_type;
        
        // Map EVM token ID to MySocial token ID for TypeTag lookup
        // The message still contains the original token_id, but we need to map it
        // for TypeTag lookup since MySocial uses different token IDs
        let mys_token_id_for_claim = if relay_key.source_chain == 12 { // EVM chain ID
            // Use same mapping as transaction builder
            if token_id == 5 {
                0 // Map EVM token ID 5 → MySocial token ID 0
            } else {
                token_id
            }
        } else {
            token_id
        };
        
        if relay_key.source_chain == 12 && token_id != mys_token_id_for_claim {
            warn!(
                evm_token_id = token_id,
                mys_token_id = mys_token_id_for_claim,
                "Token ID mapping: Message has token_id={} but using TypeTag for token_id={}",
                token_id, mys_token_id_for_claim
            );
        }
        
        // For EVM → MySocial transfers, target_address is the MySocial recipient address
        let recipient_address = MysAddress::from_bytes(&message.parsed_payload.target_address)
            .map_err(|e| BridgeError::Generic(format!("Failed to parse recipient address: {:?}", e)))?;
        
        let amount = message.parsed_payload.amount;
        let is_mys_token = mys_token_id_for_claim == 0;
        
        info!(
            ?relay_key,
            token_id,
            is_mys_token,
            recipient_address = ?recipient_address,
            amount,
            source_chain = relay_key.source_chain,
            seq_num = relay_key.seq_num,
            "Preparing to claim tokens from treasury and transfer to recipient"
        );
        
        if is_mys_token {
            info!(
                ?relay_key,
                "Claiming MYS tokens - will unlock from native_mys_locked balance (requires treasury to be bootstrapped)"
            );
        } else {
            info!(
                ?relay_key,
                token_id,
                "Claiming foreign token - will mint new tokens (requires token type to be registered with TreasuryCap)"
            );
        }

        // Get bridge object ref
        let bridge_object_arg = self
            .mys_client
            .get_mutable_bridge_object_arg_must_succeed()
            .await;

        // Get reference gas price
        let rgp = self.mys_client.get_reference_gas_price_until_success().await;

        // Get gas coin object reference
        let (_gas_coin, gas_obj_ref, _owner) = self
            .mys_client
            .get_gas_data_panic_if_not_gas(self.gas_object_id)
            .await;

        let mut builder = ProgrammableTransactionBuilder::new();

        // Unwrap: these should not fail
        let arg_bridge = builder.obj(bridge_object_arg).unwrap();
        let arg_clock = builder.input(CallArg::CLOCK_IMM).unwrap();
        let source_chain = builder.pure(relay_key.source_chain).unwrap();
        let seq_num = builder.pure(relay_key.seq_num).unwrap();

        // Call the appropriate claim function based on token type
        if mys_token_id_for_claim == 0 {
            // MYS token (ID 0) - use claim_and_transfer_mys_token
            builder.programmable_move_call(
                BRIDGE_PACKAGE_ID,
                ident_str!("bridge").to_owned(),
                ident_str!("claim_and_transfer_mys_token").to_owned(),
                vec![], // No type arguments for MYS
                vec![arg_bridge, arg_clock, source_chain, seq_num],
            );
        } else {
            // Other tokens - use claim_and_transfer_token<T>
            let token_type_tags = (*self.mys_token_type_tags.load()).clone();
            
            // Log registered token IDs for debugging
            let registered_token_ids: Vec<u8> = token_type_tags.keys().copied().collect();
            debug!(
                ?relay_key,
                original_token_id = token_id,
                mapped_token_id = mys_token_id_for_claim,
                registered_token_ids = ?registered_token_ids,
                "Looking up TypeTag for token ID"
            );
            
            let type_tag = token_type_tags
                .get(&mys_token_id_for_claim)
                .ok_or_else(|| {
                    let registered_ids: Vec<u8> = token_type_tags.keys().copied().collect();
                    BridgeError::Generic(format!(
                        "Token ID {} (mapped from EVM token ID {}) is not registered on MySocial bridge. \
                         Registered token IDs: {:?}. \
                         This token must be registered on MySocial bridge's treasury before claiming can proceed.",
                        mys_token_id_for_claim, token_id, registered_ids
                    ))
                })?
                .clone();

            builder.programmable_move_call(
                BRIDGE_PACKAGE_ID,
                ident_str!("bridge").to_owned(),
                ident_str!("claim_and_transfer_token").to_owned(),
                vec![type_tag],
                vec![arg_bridge, arg_clock, source_chain, seq_num],
            );
        }

        let pt = builder.finish();

        // Create transaction data
        let tx_data = TransactionData::new_programmable(
            self.mys_address,
            vec![gas_obj_ref],
            pt,
            self.config.mys_gas_budget,
            rgp,
        );

        // Sign the transaction
        use mys_types::crypto::Signature;
        use shared_crypto::intent::{Intent, IntentMessage};
        
        let sig = Signature::new_secure(
            &IntentMessage::new(Intent::mys_transaction(), &tx_data),
            &self.key,
        );
        
        use mys_types::transaction::Transaction;
        let signed_tx = Transaction::from_data(tx_data, vec![sig]);

        // Execute transaction
        info!(
            ?relay_key,
            mys_sender = ?self.mys_address,
            recipient = ?recipient_address,
            "Executing claim_and_transfer transaction on MySocial"
        );
        
        let response = self
            .mys_client
            .execute_transaction_block_with_effects(signed_tx)
            .await?;
        
        if !response.status_ok().unwrap_or(false) {
            error!(
                ?relay_key,
                tx_digest = ?response.digest,
                "Claim_and_transfer transaction failed"
            );
            return Err(BridgeError::Generic(format!(
                "Transaction failed: {:?}",
                response
            )));
        }

        // Verify that tokens were actually transferred by checking for TokenTransferClaimed event
        let events = response.events.ok_or_else(|| {
            error!(
                ?relay_key,
                tx_digest = ?response.digest,
                "Transaction succeeded but no events returned"
            );
            BridgeError::Generic(format!(
                "Transaction succeeded but no events returned for relay: {:?}",
                relay_key
            ))
        })?;

        // Log all events for diagnostics
        info!(
            ?relay_key,
            tx_digest = ?response.digest,
            event_count = events.data.len(),
            event_types = ?events.data.iter().map(|e| &e.type_).collect::<Vec<_>>(),
            "Transaction executed - analyzing events"
        );

        let claimed_event = events
            .data
            .iter()
            .find(|e| e.type_ == *TokenTransferClaimed.get().unwrap());

        let limit_exceeded_event = events
            .data
            .iter()
            .find(|e| e.type_ == *TokenTransferLimitExceed.get().unwrap());

        let already_claimed_event = events
            .data
            .iter()
            .find(|e| e.type_ == *TokenTransferAlreadyClaimed.get().unwrap());

        match (claimed_event, limit_exceeded_event, already_claimed_event) {
            (Some(_), _, _) => {
                // Success - tokens were claimed and transferred
                info!(
                    ?relay_key,
                    tx_digest = ?response.digest,
                    recipient = ?recipient_address,
                    "Successfully claimed tokens from treasury and transferred to recipient"
                );
                Ok(response.digest)
            }
            (_, Some(_), _) => {
                // Transfer limit exceeded - tokens not transferred
                error!(
                    ?relay_key,
                    tx_digest = ?response.digest,
                    recipient = ?recipient_address,
                    token_id,
                    amount,
                    is_mys_token,
                    "Token transfer limit exceeded - tokens were not transferred. \
                     This can happen if the bridge limiter has reached its daily/hourly limit for this route."
                );
                Err(BridgeError::Generic(format!(
                    "Token transfer limit exceeded for relay: {:?} (token_id={}, amount={}, is_mys={}). \
                     Tokens were not transferred to recipient. Check bridge limiter configuration.",
                    relay_key, token_id, amount, is_mys_token
                )))
            }
            (_, _, Some(_)) => {
                // Already claimed - tokens not transferred
                warn!(
                    ?relay_key,
                    tx_digest = ?response.digest,
                    recipient = ?recipient_address,
                    "Tokens were already claimed - no transfer occurred"
                );
                // This is not necessarily an error - could be a retry
                Ok(response.digest)
            }
            (None, None, None) => {
                // No relevant events - this is an error
                // This could happen if:
                // - For MYS: Treasury not bootstrapped (ENativeMysNotBootstrapped) or insufficient balance
                // - For other tokens: Token type not registered or TreasuryCap missing
                // - Move abort occurred but transaction still succeeded (shouldn't happen, but log it)
                error!(
                    ?relay_key,
                    tx_digest = ?response.digest,
                    recipient = ?recipient_address,
                    token_id,
                    amount,
                    is_mys_token,
                    all_events = ?events.data.iter().map(|e| &e.type_).collect::<Vec<_>>(),
                    "Transaction succeeded but no TokenTransferClaimed, TokenTransferLimitExceed, or TokenTransferAlreadyClaimed event found. \
                     Possible causes: \
                     - For MYS tokens: Treasury not bootstrapped (native_mys_bootstrapped=false) or insufficient native_mys_locked balance \
                     - For foreign tokens: Token type not registered in treasury or TreasuryCap missing \
                     - Move abort occurred (check transaction effects for abort codes)"
                );
                Err(BridgeError::Generic(format!(
                    "Claim transaction succeeded but no transfer occurred for relay: {:?} (token_id={}, amount={}, is_mys={}). \
                     Expected TokenTransferClaimed event but found events: {:?}. \
                     For MYS tokens, verify treasury is bootstrapped with 50M MYS. \
                     For foreign tokens, verify token type is registered with TreasuryCap.",
                    relay_key,
                    token_id,
                    amount,
                    is_mys_token,
                    events.data.iter().map(|e| &e.type_).collect::<Vec<_>>()
                )))
            }
        }
    }

    /// Relay an approved MySocial → EVM transfer by calling transferBridgedTokensWithSignatures
    async fn relay_to_evm(
        &self,
        relay_key: RelayKey,
        _action: &crate::types::MysToEthBridgeAction,
    ) -> BridgeResult<()> {
        // Check if EVM relay is configured
        if self.eth_provider.is_none() {
            error!(
                ?relay_key,
                "EVM relay not configured - tokens will not be minted on EVM side. \
                 To enable EVM relay, add the following to your bridge config YAML:\n\
                 relay:\n\
                   enabled: true\n\
                   evm:\n\
                     enabled: true\n\
                     max-gas-price-gwei: 10\n\
                     gas-estimate-buffer-percent: 20\n\
                     confirmation-blocks: 2\n\
                 Note: rpc-url and bridge-contract-address will automatically use values from eth section."
            );
            return Err(BridgeError::Generic(
                "EVM relay not configured - cannot mint tokens on EVM".to_string(),
            ));
        }
        
        info!(?relay_key, "Relaying to EVM");
        
        // Check EVM wallet balance before attempting relay
        match self.check_evm_wallet_balance().await {
            Ok(()) => {
                // Balance check passed, continue
            }
            Err(e) => {
                error!(
                    ?relay_key,
                    error = ?e,
                    "Insufficient EVM wallet balance - cannot execute relay transaction. \
                     Please fund the EVM wallet derived from bridge authority key."
                );
                return Err(e);
            }
        }
        
        // Record pending status
        self.store.record_relay(
            relay_key,
            String::from("pending_evm"),
            RelayResult::Pending,
            None,
        )?;
        
        // Fetch message and signatures from MySocial
        let (message, signatures) = match self
            .fetch_mys_message_and_signatures(relay_key.source_chain, relay_key.seq_num)
            .await
        {
            Ok(data) => data,
            Err(e) => {
                error!(?relay_key, ?e, "Failed to fetch message/signatures from MySocial");
                self.store.record_relay(
                    relay_key,
                    String::from("fetch_failed"),
                    RelayResult::Failed,
                    Some(format!("{:?}", e)),
                )?;
                return Err(e);
            }
        };
        
        // Build and execute EVM transaction
        match self.relay_to_evm_internal(relay_key, message, signatures).await {
            Ok(tx_hash) => {
                info!(?relay_key, ?tx_hash, "Successfully relayed to EVM");
                self.store.record_relay(
                    relay_key,
                    format!("{:?}", tx_hash),
                    RelayResult::Success,
                    None,
                )?;
                Ok(())
            }
            Err(e) if format!("{:?}", e).contains("already processed") => {
                info!(?relay_key, "Transfer already processed on EVM");
                self.store.record_relay(
                    relay_key,
                    String::from("already_processed"),
                    RelayResult::Success,
                    None,
                )?;
                Ok(())
            }
            Err(e) => {
                error!(?relay_key, ?e, "Failed to relay to EVM");
                self.store.record_relay(
                    relay_key,
                    String::from("evm_tx_failed"),
                    RelayResult::Failed,
                    Some(format!("{:?}", e)),
                )?;
                Err(e)
            }
        }
    }
    
    /// Fetch message and signatures from MySocial bridge state
    async fn fetch_mys_message_and_signatures(
        &self,
        source_chain: u8,
        seq_num: u64,
    ) -> BridgeResult<(ParsedTokenTransferMessage, Vec<Vec<u8>>)> {
        // Get the parsed message from MySocial bridge state
        let message = self
            .mys_client
            .get_parsed_token_transfer_message(source_chain, seq_num)
            .await?
            .ok_or_else(|| {
                BridgeError::Generic(format!(
                    "Message not found on MySocial: chain={}, seq={}",
                    source_chain, seq_num
                ))
            })?;
        
        // Get signatures from MySocial bridge state
        let signatures = self
            .mys_client
            .get_token_transfer_action_onchain_signatures_until_success(source_chain, seq_num)
            .await
            .ok_or_else(|| {
                BridgeError::Generic(format!(
                    "Signatures not found on MySocial: chain={}, seq={}",
                    source_chain, seq_num
                ))
            })?;
        
        // Validate we have signatures
        if signatures.is_empty() {
            return Err(BridgeError::Generic(
                "No signatures available for EVM relay".to_string(),
            ));
        }
        
        info!(
            source_chain,
            seq_num,
            sig_count = signatures.len(),
            "Fetched message and {} signatures from MySocial",
            signatures.len()
        );
        
        Ok((message, signatures))
    }
    
    /// Build and execute EVM transaction for transferBridgedTokensWithSignatures
    async fn relay_to_evm_internal(
        &self,
        relay_key: RelayKey,
        message: ParsedTokenTransferMessage,
        signatures: Vec<Vec<u8>>,
    ) -> BridgeResult<H256> {
        let eth_provider = self.eth_provider.as_ref().ok_or_else(|| {
            BridgeError::Generic("EVM provider not initialized".to_string())
        })?;
        
        let bridge_address = self.eth_bridge_address.ok_or_else(|| {
            BridgeError::Generic("EVM bridge address not configured".to_string())
        })?;
        
        let chain_id = self.eth_chain_id.ok_or_else(|| {
            BridgeError::Generic("EVM chain ID not configured".to_string())
        })?;
        
        // Create fresh signer from key
        let secp_key = match &self.key {
            mys_types::crypto::MysKeyPair::Secp256k1(k) => k,
            _ => {
                return Err(BridgeError::Generic(
                    "Key is not secp256k1".to_string(),
                ))
            }
        };
        
        let wallet = secp256k1_to_eth_wallet(secp_key)?.with_chain_id(chain_id);
        let eth_signer = SignerMiddleware::new(eth_provider.as_ref().clone(), wallet);
        
        // Extract values before converting message (which moves it)
        use fastcrypto::encoding::{Encoding, Hex};
        let token_id = message.parsed_payload.token_type;
        let amount = message.parsed_payload.amount;
        let recipient_hex = Hex::encode(&message.parsed_payload.target_address);
        
        // Convert message to EVM format (using existing From impl!)
        let evm_message: crate::abi::eth_mys_bridge::Message = message.into();
        
        // Convert signatures to Bytes
        let evm_signatures: Vec<Bytes> = signatures
            .into_iter()
            .map(Bytes::from)
            .collect();
        
        info!(
            ?relay_key,
            bridge_address = ?bridge_address,
            chain_id = chain_id,
            sig_count = evm_signatures.len(),
            token_id = token_id,
            amount = amount,
            recipient = ?recipient_hex,
            "Building EVM transaction to call transferBridgedTokensWithSignatures"
        );
        
        // Create contract instance
        let contract = EthMysBridge::new(bridge_address, Arc::new(eth_signer));
        
        // Build the contract call
        let call = contract.transfer_bridged_tokens_with_signatures(evm_signatures, evm_message);
        
        // Estimate gas
        let gas_estimate = call.estimate_gas().await.map_err(|e| {
            error!(
                ?relay_key,
                error = ?e,
                "Gas estimation failed - check contract state and message validity"
            );
            BridgeError::Generic(format!("EVM gas estimation failed: {:?}", e))
        })?;
        
        // Add buffer to gas estimate
        let buffer_percent = self.config.evm.as_ref()
            .map(|c| c.gas_estimate_buffer_percent)
            .unwrap_or(20);
        let gas_limit = gas_estimate * (100 + buffer_percent as u32) / 100;
        
        // Get current gas price
        let gas_price = eth_provider.get_gas_price().await.map_err(|e| {
            error!(
                ?relay_key,
                error = ?e,
                "Failed to get gas price from EVM provider"
            );
            BridgeError::Generic(format!("Failed to get gas price: {:?}", e))
        })?;
        
        // Apply max gas price limit
        let max_gas_price_wei = self.config.evm.as_ref()
            .map(|c| U256::from(c.max_gas_price_gwei) * U256::exp10(9))
            .unwrap_or(U256::from(10_000_000_000u64)); // 10 Gwei default
        
        let final_gas_price = gas_price.min(max_gas_price_wei);
        
        info!(
            ?relay_key,
            gas_estimate = ?gas_estimate,
            gas_limit = ?gas_limit,
            gas_price_gwei = ?(final_gas_price.as_u64() / 1_000_000_000),
            max_gas_price_gwei = ?(max_gas_price_wei.as_u64() / 1_000_000_000),
            estimated_cost_eth = ?((gas_limit.as_u64() as f64 * final_gas_price.as_u64() as f64 / 1e18)),
            "EVM transaction gas details"
        );
        
        if gas_price > max_gas_price_wei {
            warn!(
                ?relay_key,
                current_price = ?gas_price,
                max_price = ?max_gas_price_wei,
                "Gas price exceeds maximum, using capped value"
            );
        }
        
        // Send transaction with gas settings
        let call_with_gas = call.gas(gas_limit).gas_price(final_gas_price);
        
        let pending_tx = call_with_gas
            .send()
            .await
            .map_err(|e| {
                error!(
                    ?relay_key,
                    error = ?e,
                    "Failed to send EVM transaction - check wallet balance and network connectivity"
                );
                BridgeError::Generic(format!("EVM transaction send failed: {:?}", e))
            })?;
        
        let tx_hash = pending_tx.tx_hash();
        
        // Wait for confirmation
        let confirmation_blocks = self.config.evm.as_ref()
            .map(|c| c.confirmation_blocks as usize)
            .unwrap_or(2);
        
        info!(
            ?relay_key,
            tx_hash = ?tx_hash,
            gas_limit = ?gas_limit,
            gas_price_gwei = ?(final_gas_price.as_u64() / 1_000_000_000),
            confirmation_blocks = confirmation_blocks,
            "EVM transaction sent successfully - waiting for confirmation"
        );
        
        let receipt = pending_tx
            .confirmations(confirmation_blocks)
            .await
            .map_err(|e| {
                error!(?relay_key, ?e, "Failed to get EVM confirmation");
                BridgeError::Generic(format!("EVM confirmation failed: {:?}", e))
            })?
            .ok_or_else(|| {
                error!(?relay_key, "EVM transaction was dropped");
                BridgeError::Generic("EVM transaction dropped from mempool".to_string())
            })?;
        
        // Verify transaction succeeded
        let status = receipt.status.ok_or_else(|| {
            BridgeError::Generic("EVM receipt missing status field".to_string())
        })?;
        
        if status != 1.into() {
            error!(
                ?relay_key,
                tx_hash = ?receipt.transaction_hash,
                block_number = ?receipt.block_number,
                gas_used = ?receipt.gas_used,
                "EVM transaction reverted - tokens were not minted"
            );
            return Err(BridgeError::Generic(format!(
                "EVM transaction reverted. Receipt: {:?}",
                receipt
            )));
        }
        
        info!(
            ?relay_key,
            tx_hash = ?receipt.transaction_hash,
            block_number = ?receipt.block_number,
            gas_used = ?receipt.gas_used,
            "EVM transaction confirmed successfully - tokens minted on EVM"
        );
        
        info!(
            ?relay_key,
            ?tx_hash,
            block_number = ?receipt.block_number,
            gas_used = ?receipt.gas_used,
            confirmations = confirmation_blocks,
            "EVM transaction confirmed successfully"
        );
        
        Ok(tx_hash)
    }
    
    /// Check EVM wallet balance and warn if low
    async fn check_evm_wallet_balance(&self) -> BridgeResult<()> {
        if let Some(provider) = &self.eth_provider {
            // Get Ethereum address from secp256k1 key
            let eth_address = match &self.key {
                mys_types::crypto::MysKeyPair::Secp256k1(k) => {
                    BridgeAuthorityPublicKeyBytes::from(&k.public).to_eth_address()
                }
                _ => return Ok(()), // Skip if not secp256k1
            };
            
            let balance = provider
                .get_balance(eth_address, None)
                .await
                .map_err(|e| BridgeError::Generic(format!("Failed to get EVM balance: {:?}", e)))?;
            
            let balance_eth = balance.as_u128() as f64 / 1e18;
            
            const WARN_THRESHOLD: f64 = 0.1;
            const CRITICAL_THRESHOLD: f64 = 0.01;
            
            if balance_eth < CRITICAL_THRESHOLD {
                error!(
                    balance_eth,
                    address = ?eth_address,
                    "CRITICAL: EVM wallet balance critically low! Please fund immediately."
                );
                return Err(BridgeError::Generic("Insufficient EVM funds for relay".to_string()));
            } else if balance_eth < WARN_THRESHOLD {
                warn!(
                    balance_eth,
                    address = ?eth_address,
                    "WARNING: EVM wallet balance low, please refund soon"
                );
            } else {
                info!(
                    balance_eth,
                    address = ?eth_address,
                    "EVM wallet balance OK"
                );
            }
        }
        
        Ok(())
    }

    /// Retry failed relays
    pub async fn retry_failed_relays(&self) -> BridgeResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let failed_relays = self.store.get_failed_relays();

        for (key, status) in failed_relays {
            if status.retry_count >= self.config.max_retries {
                warn!(
                    ?key,
                    retry_count = status.retry_count,
                    "Max retries reached, giving up"
                );
                continue;
            }

            info!(?key, retry_count = status.retry_count, "Retrying failed relay");

            // Wait before retrying
            tokio::time::sleep(tokio::time::Duration::from_secs(
                self.config.retry_delay_seconds,
            ))
            .await;

            // Attempt to build and execute the claim transaction
            match self.build_and_execute_claim_tx(key).await {
                Ok(tx_digest) => {
                    info!(?key, ?tx_digest, "Retry successful");
                    self.store.record_relay(
                        key,
                        tx_digest.to_string(),
                        RelayResult::Success,
                        None,
                    )?;
                }
                Err(e) => {
                    error!(?key, ?e, "Retry failed");
                    self.store.record_relay(
                        key,
                        String::from("retry_failed"),
                        RelayResult::Failed,
                        Some(format!("{:?}", e)),
                    )?;
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
    fn test_relay_config_default() {
        let config = RelayConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay_seconds, 30);
        assert_eq!(config.mys_gas_budget, 100_000_000);
    }
}

