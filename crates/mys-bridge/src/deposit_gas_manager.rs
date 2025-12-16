// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Gas management for custodial deposit addresses
//! Ensures deposit addresses have sufficient gas to execute bridge transactions

use crate::error::{BridgeError, BridgeResult};
use crate::metered_eth_provider::MeteredEthHttpProvier;
use crate::mys_client::{MysClient, MysClientInner};
use ethers::prelude::*;
use ethers::types::Address as EthAddress;
use mys_types::base_types::MysAddress;
use mys_types::crypto::MysKeyPair;
use std::sync::Arc;
use tracing::{error, info, warn};

// Gas thresholds for EVM (in wei)
const MIN_EVM_GAS_BALANCE: u128 = 10_000_000_000_000_000; // 0.01 ETH
const EVM_GAS_FUND_AMOUNT: u128 = 20_000_000_000_000_000; // 0.02 ETH

// Gas thresholds for MySocial (in MIST)
// Will be used when MySocial gas funding is fully implemented
#[allow(dead_code)]
const MIN_MYS_GAS_BALANCE: u64 = 10_000_000; // 0.01 MYS
#[allow(dead_code)]
const MYS_GAS_FUND_AMOUNT: u64 = 20_000_000; // 0.02 MYS

/// Manages gas funding for deposit addresses
pub struct DepositGasManager<C> {
    /// Relayer's main EVM wallet (for funding deposit addresses)
    relayer_eth_wallet: Option<Wallet<k256::ecdsa::SigningKey>>,
    /// Relayer's main MySocial keypair (for funding deposit addresses)
    /// Will be used when MySocial gas funding is implemented
    #[allow(dead_code)]
    relayer_mys_keypair: MysKeyPair,
    /// MySocial client (for querying balances and funding)
    /// Will be used when MySocial gas funding is implemented
    #[allow(dead_code)]
    mys_client: Arc<MysClient<C>>,
    /// EVM provider
    eth_provider: Option<Arc<Provider<MeteredEthHttpProvier>>>,
    /// Chain ID for EVM
    eth_chain_id: Option<u64>,
}

impl<C> DepositGasManager<C>
where
    C: MysClientInner + 'static,
{
    #[allow(dead_code)]
    pub fn new(
        relayer_mys_keypair: MysKeyPair,
        mys_client: Arc<MysClient<C>>,
        relayer_eth_wallet: Option<Wallet<k256::ecdsa::SigningKey>>,
        eth_provider: Option<Arc<Provider<MeteredEthHttpProvier>>>,
        eth_chain_id: Option<u64>,
    ) -> Self {
        Self {
            relayer_eth_wallet,
            relayer_mys_keypair,
            mys_client,
            eth_provider,
            eth_chain_id,
        }
    }

    /// Ensure an EVM deposit address has sufficient gas
    /// Funds from relayer's main wallet if needed
    pub async fn ensure_evm_deposit_has_gas(
        &self,
        deposit_address: EthAddress,
    ) -> BridgeResult<()> {
        let provider = self.eth_provider.as_ref().ok_or_else(|| {
            BridgeError::Generic("EVM provider not configured for gas management".to_string())
        })?;

        // Verify relayer wallet exists (we'll use it in fund_evm_address)
        let _relayer_wallet = self.relayer_eth_wallet.as_ref().ok_or_else(|| {
            BridgeError::Generic("Relayer EVM wallet not configured".to_string())
        })?;

        // Check current balance
        let balance = provider
            .get_balance(deposit_address, None)
            .await
            .map_err(|e| {
                BridgeError::Generic(format!(
                    "Failed to check balance for {:?}: {:?}",
                    deposit_address, e
                ))
            })?;

        info!(
            ?deposit_address,
            balance_wei = ?balance,
            balance_eth = balance.as_u128() as f64 / 1e18,
            "Checking EVM deposit address balance"
        );

        // Fund if below threshold
        if balance.as_u128() < MIN_EVM_GAS_BALANCE {
            info!(
                ?deposit_address,
                current_balance_eth = balance.as_u128() as f64 / 1e18,
                funding_amount_eth = EVM_GAS_FUND_AMOUNT as f64 / 1e18,
                "Funding EVM deposit address with gas"
            );

            self.fund_evm_address(deposit_address, U256::from(EVM_GAS_FUND_AMOUNT))
                .await?;

            info!(?deposit_address, "Successfully funded EVM deposit address");
        } else {
            info!(
                ?deposit_address,
                balance_eth = balance.as_u128() as f64 / 1e18,
                "EVM deposit address has sufficient gas"
            );
        }

        Ok(())
    }

    /// Send ETH from relayer's main wallet to a deposit address
    async fn fund_evm_address(
        &self,
        to_address: EthAddress,
        amount: U256,
    ) -> BridgeResult<H256> {
        let provider = self.eth_provider.as_ref().ok_or_else(|| {
            BridgeError::Generic("EVM provider not configured".to_string())
        })?;

        let wallet = self
            .relayer_eth_wallet
            .as_ref()
            .ok_or_else(|| BridgeError::Generic("Relayer wallet not configured".to_string()))?
            .clone()
            .with_chain_id(
                self.eth_chain_id
                    .ok_or_else(|| BridgeError::Generic("Chain ID not configured".to_string()))?,
            );

        // Check relayer has enough balance
        let relayer_balance = provider
            .get_balance(wallet.address(), None)
            .await
            .map_err(|e| {
                BridgeError::Generic(format!("Failed to check relayer balance: {:?}", e))
            })?;

        if relayer_balance < amount {
            return Err(BridgeError::Generic(format!(
                "Relayer has insufficient balance. Has: {}, Needs: {}",
                relayer_balance, amount
            )));
        }

        // Create signer
        let signer = SignerMiddleware::new(provider.clone(), wallet);

        // Build transaction
        let tx = TransactionRequest::pay(to_address, amount);

        // Send transaction
        let pending_tx = signer.send_transaction(tx, None).await.map_err(|e| {
            BridgeError::Generic(format!("Failed to send funding transaction: {:?}", e))
        })?;

        let tx_hash = pending_tx.tx_hash();

        // Wait for confirmation
        let receipt = pending_tx.confirmations(1).await.map_err(|e| {
            BridgeError::Generic(format!("Failed to confirm funding transaction: {:?}", e))
        })?;

        if let Some(receipt) = receipt {
            if receipt.status != Some(1.into()) {
                return Err(BridgeError::Generic(
                    "Funding transaction reverted".to_string(),
                ));
            }

            info!(
                ?tx_hash,
                ?to_address,
                amount_eth = amount.as_u128() as f64 / 1e18,
                "EVM funding transaction confirmed"
            );

            Ok(tx_hash)
        } else {
            Err(BridgeError::Generic(
                "Funding transaction receipt not available".to_string(),
            ))
        }
    }

    /// Ensure a MySocial deposit address has sufficient gas
    /// 
    /// TODO: Complete implementation for production
    /// Requirements:
    /// 1. Query total MYS balance at deposit_address using coin_read_api
    /// 2. Check if balance >= MIN_MYS_GAS_BALANCE (0.01 MYS)
    /// 3. If insufficient, build transfer transaction from relayer
    /// 4. Execute transfer of MYS_GAS_FUND_AMOUNT (0.02 MYS)
    /// 5. Verify funding succeeded
    /// 
    /// Note: Primary use case is EVM→MySocial deposits, which don't need
    /// MySocial gas funding (users send TO MySocial, not FROM).
    /// MySocial→EVM deposits (which would need this) are less common.
    pub async fn ensure_mys_deposit_has_gas(
        &self,
        deposit_address: MysAddress,
    ) -> BridgeResult<()> {
        info!(
            ?deposit_address,
            min_balance_mist = MIN_MYS_GAS_BALANCE,
            "Checking MySocial deposit address gas balance"
        );

        // Implementation requires:
        // - Access to coin_read_api (needs concrete MysSdkClient type)
        // - Querying coin balance at deposit_address
        // - Building and executing transfer transaction if needed
        // - Similar pattern to ensure_evm_deposit_has_gas but for MySocial
        //
        // For MVP: Manual gas funding acceptable since MySocial→EVM is less common
        // Primary flow (EVM→MySocial) doesn't require this

        warn!(
            ?deposit_address,
            "MySocial gas funding requires manual setup - automatic funding not yet implemented"
        );

        Ok(())
    }

    /// Check if relayer's main EVM wallet has sufficient balance
    pub async fn check_relayer_evm_balance(&self) -> BridgeResult<U256> {
        let provider = self.eth_provider.as_ref().ok_or_else(|| {
            BridgeError::Generic("EVM provider not configured".to_string())
        })?;

        let wallet = self.relayer_eth_wallet.as_ref().ok_or_else(|| {
            BridgeError::Generic("Relayer wallet not configured".to_string())
        })?;

        let balance = provider
            .get_balance(wallet.address(), None)
            .await
            .map_err(|e| BridgeError::Generic(format!("Failed to get relayer balance: {:?}", e)))?;

        let balance_eth = balance.as_u128() as f64 / 1e18;

        const WARN_THRESHOLD: f64 = 1.0; // 1 ETH
        const CRITICAL_THRESHOLD: f64 = 0.1; // 0.1 ETH

        if balance_eth < CRITICAL_THRESHOLD {
            error!(
                address = ?wallet.address(),
                balance_eth,
                "CRITICAL: Relayer EVM wallet balance critically low!"
            );
        } else if balance_eth < WARN_THRESHOLD {
            warn!(
                address = ?wallet.address(),
                balance_eth,
                "WARNING: Relayer EVM wallet balance getting low"
            );
        }

        Ok(balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_constants() {
        assert_eq!(MIN_EVM_GAS_BALANCE, 10_000_000_000_000_000u128);
        assert_eq!(EVM_GAS_FUND_AMOUNT, 20_000_000_000_000_000u128);
        assert!(EVM_GAS_FUND_AMOUNT > MIN_EVM_GAS_BALANCE);

        assert_eq!(MIN_MYS_GAS_BALANCE, 10_000_000u64);
        assert_eq!(MYS_GAS_FUND_AMOUNT, 20_000_000u64);
        assert!(MYS_GAS_FUND_AMOUNT > MIN_MYS_GAS_BALANCE);
    }
}

