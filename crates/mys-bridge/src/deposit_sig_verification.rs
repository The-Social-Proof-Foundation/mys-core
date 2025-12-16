// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Signature verification for deposit address registration
//! Supports both MySocial signatures and Ethereum EIP-191 signatures

use crate::error::{BridgeError, BridgeResult};
use ethers::core::types::Signature as EthSignature;
use ethers::types::Address as EthAddress;
use ethers::utils::hash_message;
use fastcrypto::encoding::{Base64, Encoding};
use fastcrypto::traits::ToFromBytes;
use mys_types::base_types::MysAddress;
use mys_types::signature::{AuthenticatorTrait, GenericSignature, VerifyParams};
use mys_types::signature_verification::VerifiedDigestCache;
use shared_crypto::intent::{Intent, IntentMessage, PersonalMessage};
use std::str::FromStr;
use std::sync::Arc;
use tracing::{info, warn};

/// Verify a MySocial signature on a message
pub fn verify_mys_signature(
    message: &str,
    signature_b64: &str,
    expected_address: &MysAddress,
) -> BridgeResult<bool> {
    // Decode base64 signature
    let sig_bytes = Base64::decode(signature_b64).map_err(|e| {
        BridgeError::Generic(format!("Failed to decode MySocial signature: {:?}", e))
    })?;

    // Parse as GenericSignature
    let signature = GenericSignature::from_bytes(&sig_bytes).map_err(|e| {
        BridgeError::Generic(format!("Failed to parse MySocial signature: {:?}", e))
    })?;

    // Create intent message for personal message
    let intent_msg = IntentMessage::new(
        Intent::personal_message(),
        PersonalMessage {
            message: message.as_bytes().to_vec(),
        },
    );

    // Verify signature
    signature
        .verify_claims::<PersonalMessage>(
            &intent_msg,
            *expected_address,
            &VerifyParams::default(),
            Arc::new(VerifiedDigestCache::new_empty()),
        )
        .map_err(|e| {
            warn!(
                ?expected_address,
                ?e,
                "MySocial signature verification failed"
            );
            BridgeError::Generic(format!("MySocial signature verification failed: {:?}", e))
        })?;

    info!(?expected_address, "MySocial signature verified successfully");
    Ok(true)
}

/// Verify an Ethereum EIP-191 signature
pub fn verify_eth_signature(
    message: &str,
    signature_hex: &str,
    expected_address: &EthAddress,
) -> BridgeResult<bool> {
    // Parse signature (should be hex with 0x prefix)
    let signature = EthSignature::from_str(signature_hex).map_err(|e| {
        BridgeError::Generic(format!("Failed to parse Ethereum signature: {:?}", e))
    })?;

    // EIP-191: "\x19Ethereum Signed Message:\n" + len(message) + message
    let msg_hash = hash_message(message);

    // Recover signer address
    let recovered = signature.recover(msg_hash).map_err(|e| {
        BridgeError::Generic(format!("Failed to recover Ethereum signer: {:?}", e))
    })?;

    // Compare with expected
    if &recovered != expected_address {
        warn!(
            ?expected_address,
            ?recovered,
            "Ethereum address mismatch in signature verification"
        );
        return Ok(false);
    }

    info!(?expected_address, "Ethereum signature verified successfully");
    Ok(true)
}

/// Verify timestamp is recent (within 5 minutes)
pub fn verify_timestamp_recent(timestamp: u64) -> BridgeResult<bool> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    const MAX_AGE_SECONDS: u64 = 300; // 5 minutes

    if timestamp > now + 60 {
        // Allow 1 minute clock skew forward
        warn!(timestamp, now, "Timestamp is in the future");
        return Ok(false);
    }

    if now - timestamp > MAX_AGE_SECONDS {
        warn!(
            timestamp,
            now,
            age_seconds = now - timestamp,
            "Timestamp is too old"
        );
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_validation() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Current timestamp should be valid
        assert!(verify_timestamp_recent(now).unwrap());

        // 1 minute ago should be valid
        assert!(verify_timestamp_recent(now - 60).unwrap());

        // 4 minutes ago should be valid
        assert!(verify_timestamp_recent(now - 240).unwrap());

        // 10 minutes ago should be invalid
        assert!(!verify_timestamp_recent(now - 600).unwrap());

        // Future timestamp should be invalid
        assert!(!verify_timestamp_recent(now + 120).unwrap());
    }
}

