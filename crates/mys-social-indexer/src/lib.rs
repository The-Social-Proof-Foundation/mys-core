// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

pub mod api;
pub mod blockchain;
pub mod config;
pub mod db;
pub mod events;
pub mod models;
pub mod schema;

use once_cell::sync::OnceCell;

// Global package address (default value that can be overridden)
static MYSOCIAL_PACKAGE_ADDRESS: OnceCell<String> = OnceCell::new();

/// Default MySocial package address if not set via environment
pub const DEFAULT_MYSOCIAL_PACKAGE_ADDRESS: &str = "0x000000000000000000000000000000000000000000000000000000000000d880";

/// Module names within the MySocial package
pub const PROFILE_MODULE_NAME: &str = "profile";
pub const PLATFORM_MODULE_NAME: &str = "platform";
pub const SOCIAL_GRAPH_MODULE_NAME: &str = "social_graph";
pub const BLOCK_LIST_MODULE_NAME: &str = "block_list";
pub const POST_MODULE_NAME: &str = "post";
pub const GOVERNANCE_MODULE_NAME: &str = "governance";
pub const SUBSCRIPTION_MODULE_NAME: &str = "subscription";

/// Common struct names
pub const PROFILE_STRUCT_NAME: &str = "Profile";

/// Governance registry types
pub const GOVERNANCE_REGISTRY_ECOSYSTEM: u8 = 0;
pub const GOVERNANCE_REGISTRY_REPUTATION: u8 = 1;
pub const GOVERNANCE_REGISTRY_COMMUNITY_NOTES: u8 = 2;

/// Governance proposal status values
pub const GOVERNANCE_STATUS_SUBMITTED: u8 = 0;
pub const GOVERNANCE_STATUS_DELEGATE_REVIEW: u8 = 1;
pub const GOVERNANCE_STATUS_COMMUNITY_VOTING: u8 = 2;
pub const GOVERNANCE_STATUS_APPROVED: u8 = 3;
pub const GOVERNANCE_STATUS_REJECTED: u8 = 4;
pub const GOVERNANCE_STATUS_IMPLEMENTED: u8 = 5;
pub const GOVERNANCE_STATUS_OWNER_RESCINDED: u8 = 6;

/// Nominee status values
pub const NOMINEE_STATUS_PENDING: u8 = 0;
pub const NOMINEE_STATUS_ELECTED: u8 = 1;
pub const NOMINEE_STATUS_REJECTED: u8 = 2;

/// Anonymous vote status constants
pub const ANONYMOUS_VOTE_STATUS_PENDING: u8 = 0;
pub const ANONYMOUS_VOTE_STATUS_DECRYPTED: u8 = 1;
pub const ANONYMOUS_VOTE_STATUS_FAILED: u8 = 2;

/// Set the MySocial package address
pub fn set_mysocial_package_address(address: String) {
    MYSOCIAL_PACKAGE_ADDRESS.set(address).unwrap_or_else(|_| {
        tracing::warn!("MySocial package address already set, ignoring new value");
    });
}

/// Get the MySocial package address
pub fn get_mysocial_package_address() -> &'static str {
    // Use hardcoded address as fallback if not set via environment variable
    MYSOCIAL_PACKAGE_ADDRESS.get().map(|s| s.as_str()).unwrap_or(DEFAULT_MYSOCIAL_PACKAGE_ADDRESS)
}