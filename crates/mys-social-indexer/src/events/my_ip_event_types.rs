// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Event emitted when a new license is created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseCreatedEvent {
    pub license_id: String,
    pub creator: String,
    pub name: String,
    pub description: String,
    pub license_type: u8,
    pub permission_flags: u64,
    pub creation_time: u64,
    pub proof_of_creativity_id: Option<String>,
    pub custom_license_uri: Option<String>,
    pub revenue_recipient: Option<String>,
    pub transferable: bool,
    pub expires_at: Option<u64>,
}

/// Event emitted when a license is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseUpdatedEvent {
    pub license_id: String,
    pub updater: String,
    pub old_permission_flags: u64,
    pub new_permission_flags: u64,
    pub update_time: u64,
}

/// Event emitted when a license is transferred
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseTransferredEvent {
    pub license_id: String,
    pub from: String,
    pub to: String,
    pub transfer_time: u64,
}

/// Event emitted when a license state changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStateChangedEvent {
    pub license_id: String,
    pub old_state: u8,
    pub new_state: u8,
    pub changer: String,
    pub change_time: u64,
}

/// Event emitted when a license is linked to a post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseLinkedEvent {
    pub license_id: String,
    pub post_id: String,
    pub linker: String,
    pub link_time: u64,
}

/// Event emitted when a license is registered in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseRegisteredEvent {
    pub license_id: String,
    pub registry_id: String,
    pub creator: String,
    pub permission_flags: u64,
}

/// Event emitted when a license grant is created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseGrantedEvent {
    pub license_id: String,
    pub ip_id: String,
    pub grantor: String,
    pub grantee: String,
    pub payment_amount: u64,
    pub grant_time: u64,
    pub expiration_time: Option<u64>,
}

/// Event emitted when revenue is distributed for a license
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDistributedEvent {
    pub license_id: String,
    pub post_id: Option<String>,
    pub from_address: String,
    pub to_address: String,
    pub amount: u64,
    pub revenue_type: String,
    pub distribution_time: u64,
} 