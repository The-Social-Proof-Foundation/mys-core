// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::errors::OracleError;
use crate::metrics::OracleMetrics;
use anyhow::{anyhow, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use mys_bridge::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use mys_bridge::crypto::BridgeAuthorityPublicKeyBytes;
use mys_bridge::mys_client::MysClient;
use mys_bridge::types::{AssetPriceUpdateAction, BridgeAction, BridgeCommittee};
use mys_keys::keypair_file::read_key;
use mys_sdk::MysClient as MysSdkClient;
use mys_types::base_types::{MysAddress, ObjectRef};
use mys_types::bridge::BridgeChainId;
use mys_types::crypto::{Signature, MysKeyPair};
use mys_types::transaction::{Transaction, TransactionData};
use mys_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use shared_crypto::intent::{Intent, IntentMessage};
use tracing::{error, info, warn};
use uuid::Uuid;

pub struct BridgeGovernanceClient {
    validator_endpoints: Vec<String>,
    mys_client: Arc<MysClient<MysSdkClient>>,
    bridge_client_key: MysKeyPair,
    chain_id: BridgeChainId,
    token_id: u8,
    http_client: Client,
    metrics: OracleMetrics,
}

impl BridgeGovernanceClient {
    pub async fn new(
        validator_endpoints: Vec<String>,
        mys_rpc_url: String,
        bridge_client_key_path: String,
        chain_id: u8,
        token_id: u8,
        metrics: OracleMetrics,
    ) -> Result<Self> {
        // Load bridge client key
        let bridge_client_key = read_key(&std::path::PathBuf::from(bridge_client_key_path), false)?;

        // Initialize MySo client
        let bridge_metrics = Arc::new(mys_bridge::metrics::BridgeMetrics::new_for_testing());
        let mys_client = Arc::new(
            MysClient::<MysSdkClient>::new(&mys_rpc_url, bridge_metrics).await?
        );

        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        let chain_id = BridgeChainId::try_from(chain_id)
            .map_err(|_| anyhow!("Invalid bridge chain ID: {}", chain_id))?;

        Ok(Self {
            validator_endpoints,
            mys_client,
            bridge_client_key,
            chain_id,
            token_id,
            http_client,
            metrics,
        })
    }

    /// Get current price from the bridge
    pub async fn get_current_bridge_price(&self) -> Result<Option<Decimal>> {
        let summary = self.mys_client.get_bridge_summary().await?;
        
        // Find the token in supported tokens
        for (type_name, metadata) in &summary.treasury.supported_tokens {
            if metadata.id == self.token_id {
                // Convert from bridge format (4 decimal places) to Decimal
                let price_with_decimals = Decimal::from(metadata.notional_value) / Decimal::from(10000);
                return Ok(Some(price_with_decimals));
            }
        }
        
        Ok(None)
    }

    /// Create asset price update governance action
    fn create_price_update_action(&self, nonce: u64, price_usd: Decimal) -> BridgeAction {
        // Convert price to bridge format (4 decimal places)
        let bridge_price = (price_usd * Decimal::from(10000))
            .to_u64()
            .unwrap_or(0);

        BridgeAction::AssetPriceUpdateAction(AssetPriceUpdateAction {
            nonce,
            chain_id: self.chain_id,
            token_id: self.token_id,
            new_usd_price: bridge_price,
        })
    }

    /// Request signatures from bridge validators
    async fn request_validator_signatures(
        &self,
        action: BridgeAction,
        iteration_id: Uuid,
    ) -> Result<Vec<(BridgeAuthorityPublicKeyBytes, Vec<u8>)>> {
        let mut signatures = Vec::new();
        let mut total_stake = 0u64;
        const REQUIRED_STAKE: u64 = 5001; // Need >50% of 10000

        // Get bridge committee to know voting power
        let committee = self.mys_client.get_bridge_committee().await?;

        // Build signature request URL
        let AssetPriceUpdateAction {
            nonce,
            chain_id,
            token_id,
            new_usd_price,
        } = match &action {
            BridgeAction::AssetPriceUpdateAction(a) => a,
            _ => return Err(anyhow!("Expected AssetPriceUpdateAction")),
        };

        let path = format!(
            "/sign/update_asset_price/{}/{}/{}/{}",
            *chain_id as u8, nonce, token_id, new_usd_price
        );

        info!(
            iteration_id = %iteration_id,
            nonce = nonce,
            token_id = token_id,
            price = new_usd_price,
            "Requesting signatures from {} validators",
            self.validator_endpoints.len()
        );

        // Request from each validator
        for (idx, validator_url) in self.validator_endpoints.iter().enumerate() {
            let url = format!("{}{}", validator_url, path);
            
            match self.http_client
                .get(&url)
                .timeout(Duration::from_secs(10))
                .send()
                .await
            {
                Ok(response) if response.status().is_success() => {
                    match response.bytes().await {
                        Ok(sig_bytes) => {
                            // Find this validator's public key and voting power
                            // This is simplified - in production, parse response properly
                            info!(
                                iteration_id = %iteration_id,
                                validator_idx = idx,
                                "Got signature from validator"
                            );
                            
                            // Note: This is simplified - actual implementation needs to:
                            // 1. Parse validator response to get pubkey + signature
                            // 2. Match pubkey to committee member
                            // 3. Track voting power
                            
                            self.metrics.bridge_signature_success_total.inc();
                        }
                        Err(e) => {
                            warn!(
                                iteration_id = %iteration_id,
                                validator_idx = idx,
                                error = %e,
                                "Failed to read signature from validator"
                            );
                            self.metrics.bridge_signature_failed_total.inc();
                        }
                    }
                }
                Ok(response) => {
                    warn!(
                        iteration_id = %iteration_id,
                        validator_idx = idx,
                        status = %response.status(),
                        "Validator rejected signature request"
                    );
                    self.metrics.bridge_signature_failed_total.inc();
                }
                Err(e) => {
                    warn!(
                        iteration_id = %iteration_id,
                        validator_idx = idx,
                        error = %e,
                        "Failed to contact validator"
                    );
                    self.metrics.bridge_signature_failed_total.inc();
                }
            }
        }

        if total_stake < REQUIRED_STAKE {
            return Err(anyhow!(
                "Insufficient validator signatures: got {} stake, need {}",
                total_stake,
                REQUIRED_STAKE
            ));
        }

        Ok(signatures)
    }

    /// Submit price update to bridge
    pub async fn update_bridge_price(
        &self,
        nonce: u64,
        price_usd: Decimal,
        iteration_id: Uuid,
    ) -> Result<String> {
        info!(
            iteration_id = %iteration_id,
            nonce = nonce,
            price_usd = %price_usd,
            "Starting bridge price update"
        );

        self.metrics.bridge_update_attempts_total.inc();

        // Create governance action
        let action = self.create_price_update_action(nonce, price_usd);

        // Request signatures from validators
        let signatures = match self.request_validator_signatures(action.clone(), iteration_id).await {
            Ok(sigs) => sigs,
            Err(e) => {
                error!(
                    iteration_id = %iteration_id,
                    error = %e,
                    "Failed to get validator signatures"
                );
                self.metrics.bridge_update_failed_total.inc();
                return Err(e);
            }
        };

        // Build and submit transaction
        match self.build_and_submit_transaction(action, signatures, iteration_id).await {
            Ok(tx_digest) => {
                info!(
                    iteration_id = %iteration_id,
                    tx_digest = %tx_digest,
                    "Bridge price update submitted successfully"
                );
                self.metrics.bridge_update_success_total.inc();
                self.metrics.last_bridge_update_timestamp.set(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64
                );
                Ok(tx_digest)
            }
            Err(e) => {
                error!(
                    iteration_id = %iteration_id,
                    error = %e,
                    "Failed to submit bridge price update"
                );
                self.metrics.bridge_update_failed_total.inc();
                Err(e)
            }
        }
    }

    async fn build_and_submit_transaction(
        &self,
        action: BridgeAction,
        signatures: Vec<(BridgeAuthorityPublicKeyBytes, Vec<u8>)>,
        iteration_id: Uuid,
    ) -> Result<String> {
        // Get bridge object reference
        let bridge_arg = self.mys_client
            .get_mutable_bridge_object_arg_must_succeed()
            .await;

        // Get gas object
        let sender = MysAddress::from(&self.bridge_client_key.public());
        let mys_sdk_client = mys_sdk::MysClientBuilder::default()
            .build(&self.mys_client.mys_rpc_url)
            .await?;
        
        let gas_obj = mys_sdk_client
            .coin_read_api()
            .get_coins(sender, None, None, None)
            .await?
            .data
            .into_iter()
            .find(|coin| coin.balance >= 1_000_000_000)
            .ok_or_else(|| anyhow!("No gas coin found for address {}", sender))?;

        let gas_obj_ref = gas_obj.object_ref();

        // Get reference gas price
        let rgp = self.mys_client.get_reference_gas_price_until_success().await;

        // Build transaction using mys_transaction_builder
        // Note: This is simplified - actual implementation uses mys_bridge::mys_transaction_builder
        info!(
            iteration_id = %iteration_id,
            sender = %sender,
            "Building MySo transaction for price update"
        );

        // For now, return placeholder
        // TODO: Implement actual transaction building with certified action
        Err(anyhow!("Transaction building not yet implemented"))
    }

    /// Calculate price change percentage
    pub fn calculate_price_change(
        &self,
        old_price: Decimal,
        new_price: Decimal,
    ) -> Decimal {
        if old_price.is_zero() {
            return Decimal::ONE_HUNDRED;
        }
        ((new_price - old_price).abs() / old_price * Decimal::ONE_HUNDRED)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_change_calculation() {
        let client = create_test_client();
        
        // 10% increase
        let change = client.calculate_price_change(
            Decimal::from_str("0.0045").unwrap(),
            Decimal::from_str("0.00495").unwrap(),
        );
        assert_eq!(change, Decimal::from(10));

        // 50% decrease
        let change = client.calculate_price_change(
            Decimal::from_str("0.0045").unwrap(),
            Decimal::from_str("0.00225").unwrap(),
        );
        assert_eq!(change, Decimal::from(50));
    }

    fn create_test_client() -> BridgeGovernanceClient {
        // Test helper
        unimplemented!("Test helper")
    }
}

