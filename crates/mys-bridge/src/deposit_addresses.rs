// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! HD Wallet derivation for custodial deposit addresses
//! Generates deterministic deposit addresses from master bridge authority key

use crate::error::{BridgeError, BridgeResult};
use crate::storage::BridgeOrchestratorTables;
use ethers::prelude::*;
use ethers::types::Address as EthAddress;
use fastcrypto::hash::{HashFunction, Keccak256};
use fastcrypto::secp256k1::{Secp256k1KeyPair, Secp256k1PrivateKey};
use fastcrypto::traits::{KeyPair as KeyPairTrait, ToFromBytes};
use mys_types::base_types::MysAddress;
use mys_types::crypto::MysKeyPair;
use std::sync::Arc;
use tracing::info;

// Chain ID constants for HD wallet counter
// Note: These will be used when wiring deposit system into node.rs
#[allow(dead_code)]
const HD_COUNTER_EVM: u8 = 0;
#[allow(dead_code)]
const HD_COUNTER_MYS: u8 = 1;

/// Manages HD wallet derivation for deposit addresses
pub struct DepositAddressManager {
    master_key: Secp256k1KeyPair,
    store: Arc<BridgeOrchestratorTables>,
}

impl DepositAddressManager {
    /// Create new deposit address manager
    pub fn new(master_key: Secp256k1KeyPair, store: Arc<BridgeOrchestratorTables>) -> Self {
        info!("Initializing DepositAddressManager");
        Self { master_key, store }
    }

    /// Derive EVM deposit address using simple hash-based derivation
    /// Note: For production BIP-32/BIP-44, but starting simple for MVP
    pub fn derive_evm_deposit_address(
        &self,
        index: u32,
    ) -> BridgeResult<(EthAddress, Wallet<k256::ecdsa::SigningKey>)> {
        // Simple derivation: Hash(master_privkey || "evm_deposit" || index)
        let mut derivation_data = Vec::new();
        derivation_data.extend_from_slice(self.master_key.copy().private().as_bytes());
        derivation_data.extend_from_slice(b"evm_deposit");
        derivation_data.extend_from_slice(&index.to_be_bytes());

        let hash = Keccak256::digest(&derivation_data);

        // Create private key from hash
        let child_privkey_bytes = hash.digest;
        let signing_key = k256::ecdsa::SigningKey::from_slice(&child_privkey_bytes)
            .map_err(|e| BridgeError::Generic(format!("Failed to create signing key: {:?}", e)))?;

        // Create wallet
        let wallet = Wallet::from(signing_key);
        let address = wallet.address();

        info!(
            index,
            ?address,
            "Derived EVM deposit address"
        );

        Ok((address, wallet))
    }

    /// Derive MySocial deposit address using hash-based derivation
    pub fn derive_mys_deposit_address(&self, index: u32) -> BridgeResult<(MysAddress, MysKeyPair)> {
        // Derivation: Hash(master_pubkey || "mys_deposit" || index)
        let mut derivation_data = Vec::new();
        derivation_data.extend_from_slice(self.master_key.public().as_bytes());
        derivation_data.extend_from_slice(b"mys_deposit");
        derivation_data.extend_from_slice(&index.to_be_bytes());

        let hash = Keccak256::digest(&derivation_data);

        // Create private key from hash
        let child_privkey = Secp256k1PrivateKey::from_bytes(&hash.digest).map_err(|e| {
            BridgeError::Generic(format!("Failed to create MySocial private key: {:?}", e))
        })?;

        let child_keypair = Secp256k1KeyPair::from(child_privkey);
        let address = MysAddress::from(child_keypair.public());

        info!(
            index,
            ?address,
            "Derived MySocial deposit address"
        );

        Ok((address, MysKeyPair::Secp256k1(child_keypair)))
    }

    /// Allocate next HD wallet index for a specific chain
    pub fn allocate_next_index(&self, chain_type: u8) -> BridgeResult<u32> {
        let current_index = self
            .store
            .get_hd_wallet_counter(chain_type)?
            .unwrap_or(0);

        let next_index = current_index + 1;

        // Update counter in storage
        self.store.set_hd_wallet_counter(chain_type, next_index)?;

        info!(chain_type, next_index, "Allocated HD wallet index");

        Ok(current_index)
    }

    /// Get EVM wallet for a specific index (for signing bridge transactions)
    pub fn get_evm_wallet_for_index(
        &self,
        index: u32,
    ) -> BridgeResult<Wallet<k256::ecdsa::SigningKey>> {
        let (_address, wallet) = self.derive_evm_deposit_address(index)?;
        Ok(wallet)
    }

    /// Get MySocial keypair for a specific index (for signing bridge transactions)
    pub fn get_mys_keypair_for_index(&self, index: u32) -> BridgeResult<MysKeyPair> {
        let (_address, keypair) = self.derive_mys_deposit_address(index)?;
        Ok(keypair)
    }

    /// Verify an address was derived from our master key
    pub fn verify_deposit_address(&self, address: &EthAddress, index: u32) -> BridgeResult<bool> {
        let (derived_address, _) = self.derive_evm_deposit_address(index)?;
        Ok(&derived_address == address)
    }

    /// Verify a MySocial address was derived from our master key
    pub fn verify_mys_deposit_address(
        &self,
        address: &MysAddress,
        index: u32,
    ) -> BridgeResult<bool> {
        let (derived_address, _) = self.derive_mys_deposit_address(index)?;
        Ok(&derived_address == address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fastcrypto::traits::KeyPair;
    use mys_types::crypto::get_key_pair;

    #[test]
    fn test_evm_derivation_deterministic() {
        let (_, master_key): (_, Secp256k1KeyPair) = get_key_pair();
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let manager = DepositAddressManager::new(master_key, store);

        // Derive same index twice
        let (addr1, _) = manager.derive_evm_deposit_address(0).unwrap();
        let (addr2, _) = manager.derive_evm_deposit_address(0).unwrap();

        // Should be identical
        assert_eq!(addr1, addr2);

        // Different indices should produce different addresses
        let (addr3, _) = manager.derive_evm_deposit_address(1).unwrap();
        assert_ne!(addr1, addr3);
    }

    #[test]
    fn test_mys_derivation_deterministic() {
        let (_, master_key): (_, Secp256k1KeyPair) = get_key_pair();
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let manager = DepositAddressManager::new(master_key, store);

        // Derive same index twice
        let (addr1, _) = manager.derive_mys_deposit_address(0).unwrap();
        let (addr2, _) = manager.derive_mys_deposit_address(0).unwrap();

        // Should be identical
        assert_eq!(addr1, addr2);

        // Different indices should produce different addresses
        let (addr3, _) = manager.derive_mys_deposit_address(1).unwrap();
        assert_ne!(addr1, addr3);
    }

    #[test]
    fn test_index_allocation() {
        let (_, master_key): (_, Secp256k1KeyPair) = get_key_pair();
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let manager = DepositAddressManager::new(master_key, store);

        // Allocate EVM indices
        let idx1 = manager.allocate_next_index(HD_COUNTER_EVM).unwrap();
        let idx2 = manager.allocate_next_index(HD_COUNTER_EVM).unwrap();
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);

        // MySocial indices should be independent
        let idx3 = manager.allocate_next_index(HD_COUNTER_MYS).unwrap();
        assert_eq!(idx3, 0);
    }
}

