// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deposit address registration API
//! Provides endpoints for generating custodial deposit addresses

use crate::deposit_addresses::DepositAddressManager;
use crate::deposit_sig_verification::{
    verify_eth_signature, verify_mys_signature, verify_timestamp_recent,
};
use crate::error::BridgeError;
use crate::storage::{
    BridgeOrchestratorTables, DepositAddressKey, DepositRegistration, RegistrationType,
};
use axum::{extract::State, http::StatusCode, Json};
use ethers::types::Address as EthAddress;
use fastcrypto::encoding::Encoding;
use mys_types::base_types::MysAddress;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};

// Chain ID constants
const HD_COUNTER_EVM: u8 = 0;
const HD_COUNTER_MYS: u8 = 1;

/// Shared state for deposit API
pub struct DepositApiState {
    pub address_manager: Arc<DepositAddressManager>,
    pub storage: Arc<BridgeOrchestratorTables>,
}

/// Request to generate a deposit address (Option A)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateDepositRequest {
    pub auth_type: AuthType,
    pub source_address: Option<String>,
    pub signature: Option<String>,
    pub message: MessagePayload,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    MySocial,
    Ethereum,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePayload {
    pub action: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub timestamp: u64,
}

/// Response with generated deposit address
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateDepositResponse {
    pub deposit_chain: String,
    pub deposit_address: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub instructions: String,
}

/// Request to link addresses with both signatures (Option B)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkAddressesRequest {
    pub mys_address: String,
    pub mys_signature: String,
    pub eth_address: String,
    pub eth_signature: String,
    pub timestamp: u64,
}

/// Response for linked addresses
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkAddressesResponse {
    pub mys_deposit_address: String,
    pub evm_deposit_address: String,
    pub linked_mys_address: String,
    pub linked_eth_address: String,
    pub status: String,
}

/// Handler for generating deposit address (Option A)
pub async fn generate_deposit_address(
    State(state): State<Arc<DepositApiState>>,
    Json(req): Json<GenerateDepositRequest>,
) -> Result<Json<GenerateDepositResponse>, (StatusCode, String)> {
    info!(?req.auth_type, "Received deposit address generation request");

    // Verify timestamp only if signature is provided
    if req.signature.is_some() {
        if !verify_timestamp_recent(req.message.timestamp).map_err(to_status_error)? {
            return Err((
                StatusCode::BAD_REQUEST,
                "Timestamp too old or invalid".to_string(),
            ));
        }
    }

    // Reconstruct message string (needed for signature verification if provided)
    let message_str = format!(
        "Generate deposit for {}:{} at {}",
        req.message.destination_chain, req.message.destination_address, req.message.timestamp
    );

    match req.auth_type {
        AuthType::MySocial => {
            generate_for_mys_user(state, req, message_str).await
        }
        AuthType::Ethereum => {
            generate_for_eth_user(state, req, message_str).await
        }
    }
}

/// Generate deposit address for MySocial user (wants to bridge TO EVM)
async fn generate_for_mys_user(
    state: Arc<DepositApiState>,
    req: GenerateDepositRequest,
    message_str: String,
) -> Result<Json<GenerateDepositResponse>, (StatusCode, String)> {
    // Parse MySocial address (required for MySocial auth type)
    let source_address = req.source_address.as_ref().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            "source_address is required for MySocial auth type".to_string(),
        )
    })?;
    
    let mys_address =
        MysAddress::from_str(source_address).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Invalid MySocial address: {:?}", e),
            )
        })?;

    // Verify MySocial signature (required for MySocial auth type)
    let signature = req.signature.as_ref().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            "signature is required for MySocial auth type".to_string(),
        )
    })?;
    
    verify_mys_signature(&message_str, signature, &mys_address).map_err(to_status_error)?;

    // Parse destination EVM address
    let dest_eth_address = EthAddress::from_str(&req.message.destination_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid destination address: {:?}", e),
        )
    })?;

    // Allocate HD wallet index for MySocial chain
    let hd_index = state
        .address_manager
        .allocate_next_index(HD_COUNTER_MYS)
        .map_err(to_status_error)?;

    // Derive MySocial deposit address (user sends HERE)
    let (deposit_mys_address, _) = state
        .address_manager
        .derive_mys_deposit_address(hd_index)
        .map_err(to_status_error)?;

    // Parse destination chain
    let dest_chain_id = parse_chain_id(&req.message.destination_chain)?;

    // Store registration
    let registration = DepositRegistration {
        deposit_chain: 2, // MySocial chain ID
        deposit_address: deposit_mys_address.to_vec(),
        destination_chain: dest_chain_id,
        destination_address: dest_eth_address.as_bytes().to_vec(),
        hd_index,
        registration_type: RegistrationType::ApiMysSig,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        last_used: None,
    };

    state
        .storage
        .store_deposit_registration(DepositAddressKey::from_mys(mys_address), registration)
        .map_err(to_status_error)?;

    info!(
        ?mys_address,
        ?deposit_mys_address,
        ?dest_eth_address,
        "Generated MySocial deposit address for MySocial user"
    );

    let dest_chain = req.message.destination_chain.clone();
    let dest_addr = req.message.destination_address.clone();

    Ok(Json(GenerateDepositResponse {
        deposit_chain: "mysocial".to_string(),
        deposit_address: format!("{}", deposit_mys_address),
        destination_chain: dest_chain.clone(),
        destination_address: dest_addr.clone(),
        instructions: format!(
            "Send tokens to {} on MySocial chain, they will bridge to {} on {}",
            deposit_mys_address, dest_eth_address, dest_chain
        ),
    }))
}

/// Generate deposit address for MySocial user (wants to receive ETH)
async fn generate_for_eth_user(
    state: Arc<DepositApiState>,
    req: GenerateDepositRequest,
    message_str: String,
) -> Result<Json<GenerateDepositResponse>, (StatusCode, String)> {
    // Parse destination MySocial address (this is the user who will receive tokens)
    let dest_mys_address = MysAddress::from_str(&req.message.destination_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid MySocial destination address: {:?}", e),
        )
    })?;

    // Optional: Verify signature if provided (for spam protection)
    if let (Some(source_address), Some(signature)) = (&req.source_address, &req.signature) {
        // If signature is provided, verify it against the MySocial address
        let mys_address = MysAddress::from_str(source_address).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Invalid source MySocial address: {:?}", e),
            )
        })?;
        
        verify_mys_signature(&message_str, signature, &mys_address).map_err(to_status_error)?;
        
        info!(
            ?mys_address,
            "Verified signature for ETH deposit address generation"
        );
    }

    // Allocate HD wallet index for EVM chain
    let hd_index = state
        .address_manager
        .allocate_next_index(HD_COUNTER_EVM)
        .map_err(to_status_error)?;

    // Derive EVM deposit address (user sends HERE)
    let (deposit_evm_address, _) = state
        .address_manager
        .derive_evm_deposit_address(hd_index)
        .map_err(to_status_error)?;

    // Parse destination chain (should be MySocial)
    let dest_chain_id = parse_chain_id(&req.message.destination_chain)?;

    // Parse source chain from request or default to Base
    let source_chain_id = 12; // Default to Base Sepolia for now

    // Store registration
    let registration = DepositRegistration {
        deposit_chain: source_chain_id,
        deposit_address: deposit_evm_address.as_bytes().to_vec(),
        destination_chain: dest_chain_id,
        destination_address: dest_mys_address.to_vec(),
        hd_index,
        registration_type: RegistrationType::ApiEthSig,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        last_used: None,
    };

    state
        .storage
        .store_deposit_registration(DepositAddressKey::from_mys(dest_mys_address), registration)
        .map_err(to_status_error)?;

    info!(
        ?dest_mys_address,
        ?deposit_evm_address,
        "Generated EVM deposit address for MySocial user"
    );

    let dest_chain = req.message.destination_chain.clone();
    let dest_addr = req.message.destination_address.clone();

    Ok(Json(GenerateDepositResponse {
        deposit_chain: "base".to_string(),
        deposit_address: format!("{:?}", deposit_evm_address),
        destination_chain: dest_chain,
        destination_address: dest_addr,
        instructions: format!(
            "Send tokens to {} on Base chain, they will bridge to {} on MySocial",
            deposit_evm_address, dest_mys_address
        ),
    }))
}

/// Handler for linking addresses (Option B)
pub async fn link_addresses(
    State(state): State<Arc<DepositApiState>>,
    Json(req): Json<LinkAddressesRequest>,
) -> Result<Json<LinkAddressesResponse>, (StatusCode, String)> {
    info!("Received address linking request");

    // Verify timestamp
    if !verify_timestamp_recent(req.timestamp).map_err(to_status_error)? {
        return Err((
            StatusCode::BAD_REQUEST,
            "Timestamp too old or invalid".to_string(),
        ));
    }

    // Parse addresses
    let mys_address = MysAddress::from_str(&req.mys_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid MySocial address: {:?}", e),
        )
    })?;

    let eth_address = EthAddress::from_str(&req.eth_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid Ethereum address: {:?}", e),
        )
    })?;

    // Verify MySocial signature
    let mys_message = format!("Link to ETH {} at {}", req.eth_address, req.timestamp);
    verify_mys_signature(&mys_message, &req.mys_signature, &mys_address)
        .map_err(to_status_error)?;

    // Verify ETH signature  
    let eth_message = format!("Link to MYS {} at {}", req.mys_address, req.timestamp);
    verify_eth_signature(&eth_message, &req.eth_signature, &eth_address)
        .map_err(to_status_error)?;

    // Allocate indices for both chains
    let mys_hd_index = state
        .address_manager
        .allocate_next_index(HD_COUNTER_MYS)
        .map_err(to_status_error)?;

    let evm_hd_index = state
        .address_manager
        .allocate_next_index(HD_COUNTER_EVM)
        .map_err(to_status_error)?;

    // Derive MySocial deposit address
    let (deposit_mys_address, _) = state
        .address_manager
        .derive_mys_deposit_address(mys_hd_index)
        .map_err(to_status_error)?;

    // Derive EVM deposit address
    let (deposit_evm_address, _) = state
        .address_manager
        .derive_evm_deposit_address(evm_hd_index)
        .map_err(to_status_error)?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    // Store MySocial deposit registration (routes to linked ETH address)
    let mys_registration = DepositRegistration {
        deposit_chain: 2, // MySocial
        deposit_address: deposit_mys_address.to_vec(),
        destination_chain: 12, // Base (or configurable)
        destination_address: eth_address.as_bytes().to_vec(),
        hd_index: mys_hd_index,
        registration_type: RegistrationType::Linked,
        created_at: now,
        last_used: None,
    };

    state
        .storage
        .store_deposit_registration(DepositAddressKey::from_mys(mys_address), mys_registration)
        .map_err(to_status_error)?;

    // Store EVM deposit registration (routes to linked MySocial address)
    let evm_registration = DepositRegistration {
        deposit_chain: 12, // Base
        deposit_address: deposit_evm_address.as_bytes().to_vec(),
        destination_chain: 2, // MySocial
        destination_address: mys_address.to_vec(),
        hd_index: evm_hd_index,
        registration_type: RegistrationType::Linked,
        created_at: now,
        last_used: None,
    };

    state
        .storage
        .store_deposit_registration(DepositAddressKey::from_evm(eth_address), evm_registration)
        .map_err(to_status_error)?;

    info!(
        ?mys_address,
        ?eth_address,
        ?deposit_mys_address,
        ?deposit_evm_address,
        "Linked addresses successfully"
    );

    Ok(Json(LinkAddressesResponse {
        mys_deposit_address: format!("{}", deposit_mys_address),
        evm_deposit_address: format!("{:?}", deposit_evm_address),
        linked_mys_address: req.mys_address,
        linked_eth_address: req.eth_address,
        status: "linked".to_string(),
    }))
}

/// Query deposit addresses for a given address
pub async fn query_deposit_addresses(
    State(state): State<Arc<DepositApiState>>,
    axum::extract::Path(address): axum::extract::Path<String>,
) -> Result<Json<QueryDepositResponse>, (StatusCode, String)> {
    info!(address, "Querying deposit addresses");

    // Try parsing as MySocial address first
    if let Ok(mys_addr) = MysAddress::from_str(&address) {
        let registrations = state
            .storage
            .get_deposit_registrations(&DepositAddressKey::from_mys(mys_addr))
            .map_err(to_status_error)?
            .unwrap_or_default();

        return Ok(Json(QueryDepositResponse {
            source_address: address,
            registrations: registrations
                .into_iter()
                .map(|r| RegistrationInfo {
                    deposit_chain: chain_id_to_name(r.deposit_chain),
                    deposit_address: format_address(&r.deposit_address, r.deposit_chain),
                    destination_chain: chain_id_to_name(r.destination_chain),
                    destination_address: format_address(&r.destination_address, r.destination_chain),
                    registration_type: format!("{:?}", r.registration_type),
                    created_at: r.created_at,
                })
                .collect(),
        }));
    }

    // Try parsing as EVM address
    if let Ok(eth_addr) = EthAddress::from_str(&address) {
        let registrations = state
            .storage
            .get_deposit_registrations(&DepositAddressKey::from_evm(eth_addr))
            .map_err(to_status_error)?
            .unwrap_or_default();

        return Ok(Json(QueryDepositResponse {
            source_address: address,
            registrations: registrations
                .into_iter()
                .map(|r| RegistrationInfo {
                    deposit_chain: chain_id_to_name(r.deposit_chain),
                    deposit_address: format_address(&r.deposit_address, r.deposit_chain),
                    destination_chain: chain_id_to_name(r.destination_chain),
                    destination_address: format_address(&r.destination_address, r.destination_chain),
                    registration_type: format!("{:?}", r.registration_type),
                    created_at: r.created_at,
                })
                .collect(),
        }));
    }

    Err((
        StatusCode::BAD_REQUEST,
        "Invalid address format".to_string(),
    ))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDepositResponse {
    pub source_address: String,
    pub registrations: Vec<RegistrationInfo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationInfo {
    pub deposit_chain: String,
    pub deposit_address: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub registration_type: String,
    pub created_at: u64,
}

// Helper functions

fn parse_chain_id(chain_name: &str) -> Result<u8, (StatusCode, String)> {
    match chain_name.to_lowercase().as_str() {
        "mysocial" | "mys" => Ok(2),
        "base" | "base-sepolia" => Ok(12),
        "ethereum" | "eth" => Ok(1),
        _ => Err((
            StatusCode::BAD_REQUEST,
            format!("Unsupported chain: {}", chain_name),
        )),
    }
}

fn chain_id_to_name(chain_id: u8) -> String {
    match chain_id {
        1 => "ethereum".to_string(),
        2 => "mysocial".to_string(),
        12 => "base".to_string(),
        _ => format!("chain_{}", chain_id),
    }
}

fn format_address(address_bytes: &[u8], chain_id: u8) -> String {
    if chain_id == 2 {
        // MySocial address (32 bytes)
        if let Ok(addr) = MysAddress::from_bytes(address_bytes) {
            format!("{}", addr)
        } else {
            format!("0x{}", fastcrypto::encoding::Hex::encode(address_bytes))
        }
    } else {
        // EVM address (20 bytes)
        if address_bytes.len() == 20 {
            format!("{:?}", EthAddress::from_slice(address_bytes))
        } else {
            format!("0x{}", fastcrypto::encoding::Hex::encode(address_bytes))
        }
    }
}

fn to_status_error(err: BridgeError) -> (StatusCode, String) {
    error!(?err, "API error");
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
}

