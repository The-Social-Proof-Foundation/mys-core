// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use bip32::{ChildNumber, DerivationPath, XPub};
use ethers::core::k256::ecdsa::VerifyingKey;
use ethers::types::Address as EthAddress;
use fastcrypto::hash::{HashFunction, Keccak256};

/// Parse a BIP32 derivation path string, e.g. "m/0/5".
pub fn parse_derivation_path(path: &str) -> Result<DerivationPath> {
    Ok(path.parse::<DerivationPath>()?)
}

/// Convert a secp256k1 verifying key to an Ethereum address.
///
/// Uses Keccak256 over the uncompressed 64-byte X||Y.
pub fn eth_address_from_verifying_key(pubkey: &VerifyingKey) -> EthAddress {
    let encoded = pubkey.to_encoded_point(false);
    let bytes = encoded.as_bytes();
    // bytes[0] is the 0x04 prefix; bytes[1..65] is x||y.
    let xy = &bytes[1..];
    let hash = Keccak256::digest(xy).digest;
    EthAddress::from_slice(&hash[12..])
}

/// Derive an EVM deposit address from an xpub at a specific derivation index.
///
/// MySo's secp256k1 derivation scheme (see `crates/mys-keys/src/key_derive.rs`) is:
/// `m/54'/6976'/0'/0/{index}` where the first 3 levels are hardened.
///
/// **Important constraint for xpub-only systems:**
/// hardened derivation cannot be done from an xpub, so the xpub we store online must already be
/// derived at the last hardened node: `m/54'/6976'/0'`.
///
/// Given that xpub, we derive the remaining non-hardened path: `m/0/{index}` (change=0).
pub fn derive_evm_address_from_xpub(xpub: &XPub, index: u32) -> Result<EthAddress> {
    // Non-hardened child numbers.
    let change = ChildNumber::new(0, false).map_err(|e| anyhow!("Invalid child number: {e}"))?;
    let addr_index =
        ChildNumber::new(index, false).map_err(|e| anyhow!("Invalid child number: {e}"))?;

    let x_change = xpub.derive_child(change)?;
    let x_addr = x_change.derive_child(addr_index)?;
    let pubkey_bytes = x_addr.public_key().to_bytes();
    let vk = VerifyingKey::from_sec1_bytes(pubkey_bytes.as_slice())
        .map_err(|e| anyhow!("Failed to parse derived pubkey as secp256k1 key: {e}"))?;
    Ok(eth_address_from_verifying_key(&vk))
}
