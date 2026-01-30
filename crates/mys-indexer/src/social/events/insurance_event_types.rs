// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Insurance event type constants
pub const EVENT_CONFIG_INITIALIZED: &str = "ConfigInitializedEvent";
pub const EVENT_VAULT_CREATED: &str = "UnderwriterVaultCreatedEvent";
pub const EVENT_VAULT_DEPOSITED: &str = "UnderwriterVaultDepositedEvent";
pub const EVENT_VAULT_WITHDRAWN: &str = "UnderwriterVaultWithdrawnEvent";
pub const EVENT_COVERAGE_PURCHASED: &str = "CoveragePurchasedEvent";
pub const EVENT_COVERAGE_CANCELLED: &str = "CoverageCancelledEvent";
pub const EVENT_COVERAGE_CLAIMED: &str = "CoverageClaimedEvent";
pub const EVENT_CONFIG_UPDATED: &str = "ConfigUpdatedEvent";
pub const EVENT_POLICY_EXPIRED: &str = "PolicyExpiredEvent";

/// Policy status constants matching the contract
pub const STATUS_ACTIVE: u8 = 1;
pub const STATUS_CANCELLED: u8 = 2;
pub const STATUS_CLAIMED: u8 = 3;
pub const STATUS_EXPIRED: u8 = 4;

/// Transaction type constants
pub const TRANSACTION_TYPE_DEPOSIT: &str = "DEPOSIT";
pub const TRANSACTION_TYPE_WITHDRAWAL: &str = "WITHDRAWAL";

/// Policy event type constants
pub const POLICY_EVENT_PURCHASED: &str = "PURCHASED";
pub const POLICY_EVENT_CANCELLED: &str = "CANCELLED";
pub const POLICY_EVENT_CLAIMED: &str = "CLAIMED";
pub const POLICY_EVENT_EXPIRED: &str = "EXPIRED";

/// Insurance event types enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsuranceEventType {
    ConfigInitialized,
    VaultCreated,
    VaultDeposited,
    VaultWithdrawn,
    CoveragePurchased,
    CoverageCancelled,
    CoverageClaimed,
    ConfigUpdated,
    PolicyExpired,
}

impl InsuranceEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InsuranceEventType::ConfigInitialized => EVENT_CONFIG_INITIALIZED,
            InsuranceEventType::VaultCreated => EVENT_VAULT_CREATED,
            InsuranceEventType::VaultDeposited => EVENT_VAULT_DEPOSITED,
            InsuranceEventType::VaultWithdrawn => EVENT_VAULT_WITHDRAWN,
            InsuranceEventType::CoveragePurchased => EVENT_COVERAGE_PURCHASED,
            InsuranceEventType::CoverageCancelled => EVENT_COVERAGE_CANCELLED,
            InsuranceEventType::CoverageClaimed => EVENT_COVERAGE_CLAIMED,
            InsuranceEventType::ConfigUpdated => EVENT_CONFIG_UPDATED,
            InsuranceEventType::PolicyExpired => EVENT_POLICY_EXPIRED,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            s if s.ends_with(EVENT_CONFIG_INITIALIZED) => Some(InsuranceEventType::ConfigInitialized),
            s if s.ends_with(EVENT_VAULT_CREATED) => Some(InsuranceEventType::VaultCreated),
            s if s.ends_with(EVENT_VAULT_DEPOSITED) => Some(InsuranceEventType::VaultDeposited),
            s if s.ends_with(EVENT_VAULT_WITHDRAWN) => Some(InsuranceEventType::VaultWithdrawn),
            s if s.ends_with(EVENT_COVERAGE_PURCHASED) => Some(InsuranceEventType::CoveragePurchased),
            s if s.ends_with(EVENT_COVERAGE_CANCELLED) => Some(InsuranceEventType::CoverageCancelled),
            s if s.ends_with(EVENT_COVERAGE_CLAIMED) => Some(InsuranceEventType::CoverageClaimed),
            s if s.ends_with(EVENT_CONFIG_UPDATED) => Some(InsuranceEventType::ConfigUpdated),
            s if s.ends_with(EVENT_POLICY_EXPIRED) => Some(InsuranceEventType::PolicyExpired),
            _ => None,
        }
    }
}

