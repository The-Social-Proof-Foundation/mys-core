// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{vesting_events, vesting_wallets};

// ===========================================================================
// VESTING WALLET MODELS
// ===========================================================================

/// Represents a vesting wallet in the database
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = vesting_wallets)]
pub struct VestingWallet {
    pub wallet_id: String,
    pub owner_address: String,
    pub total_amount: i64,
    pub start_time: i64,
    pub duration: i64,
    pub curve_factor: i64,
    pub claimed_amount: i64,
    pub remaining_balance: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

/// New vesting wallet for insertion
#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = vesting_wallets)]
pub struct NewVestingWallet {
    pub wallet_id: String,
    pub owner_address: String,
    pub total_amount: i64,
    pub start_time: i64,
    pub duration: i64,
    pub curve_factor: i64,
    pub claimed_amount: i64,
    pub remaining_balance: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

/// Update vesting wallet for partial updates
#[derive(Debug, Clone, PartialEq, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = vesting_wallets)]
pub struct UpdateVestingWallet {
    pub claimed_amount: Option<i64>,
    pub remaining_balance: Option<i64>,
    pub updated_at: NaiveDateTime,
}

impl NewVestingWallet {
    /// Create a new vesting wallet from a TokensVestedEvent
    pub fn from_tokens_vested_event(
        wallet_id: String,
        owner_address: String,
        total_amount: u64,
        start_time: u64,
        duration: u64,
        curve_factor: u64,
        transaction_id: String,
        timestamp: Option<u64>,
    ) -> Self {
        let now = if let Some(ts) = timestamp {
            chrono::DateTime::from_timestamp((ts / 1000) as i64, ((ts % 1000) * 1_000_000) as u32)
                .unwrap_or_else(chrono::Utc::now)
                .naive_utc()
        } else {
            chrono::Utc::now().naive_utc()
        };

        Self {
            wallet_id,
            owner_address,
            total_amount: total_amount as i64,
            start_time: start_time as i64,
            duration: duration as i64,
            curve_factor: curve_factor as i64,
            claimed_amount: 0,                      // Initially no tokens claimed
            remaining_balance: total_amount as i64, // All tokens remaining initially
            created_at: now,
            updated_at: now,
            transaction_id,
        }
    }
}

impl UpdateVestingWallet {
    /// Create an update for a vesting wallet when tokens are claimed
    /// 
    /// # Arguments
    /// * `total_claimed_amount` - Cumulative total amount claimed (not incremental)
    /// * `remaining_balance` - Remaining balance after this claim
    /// * `timestamp` - Optional timestamp for the update
    /// 
    /// # Note
    /// The `total_claimed_amount` should be calculated as `total_amount - remaining_balance`
    /// to ensure the invariant `claimed_amount + remaining_balance = total_amount` is maintained.
    pub fn from_tokens_claimed(
        total_claimed_amount: u64,
        remaining_balance: u64,
        timestamp: Option<u64>,
    ) -> Self {
        let now = if let Some(ts) = timestamp {
            chrono::DateTime::from_timestamp((ts / 1000) as i64, ((ts % 1000) * 1_000_000) as u32)
                .unwrap_or_else(chrono::Utc::now)
                .naive_utc()
        } else {
            chrono::Utc::now().naive_utc()
        };

        Self {
            claimed_amount: Some(total_claimed_amount as i64),
            remaining_balance: Some(remaining_balance as i64),
            updated_at: now,
        }
    }
}

// ===========================================================================
// VESTING EVENT MODELS
// ===========================================================================

/// Represents a vesting event in the database (TimescaleDB hypertable)
#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = vesting_events)]
pub struct VestingEvent {
    pub id: i32,
    pub wallet_id: String,
    pub event_type: String,
    pub owner_address: String,
    pub amount: i64,
    pub remaining_balance: Option<i64>,
    pub start_time: Option<i64>,
    pub duration: Option<i64>,
    pub curve_factor: Option<i64>,
    pub event_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

/// New vesting event for insertion
#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = vesting_events)]
pub struct NewVestingEvent {
    pub wallet_id: String,
    pub event_type: String,
    pub owner_address: String,
    pub amount: i64,
    pub remaining_balance: Option<i64>,
    pub start_time: Option<i64>,
    pub duration: Option<i64>,
    pub curve_factor: Option<i64>,
    pub event_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl NewVestingEvent {
    /// Create a new vesting event from a TokensVestedEvent
    pub fn from_tokens_vested_event(
        wallet_id: String,
        owner_address: String,
        total_amount: u64,
        start_time: u64,
        duration: u64,
        curve_factor: u64,
        vested_at: u64,
        transaction_id: String,
    ) -> Self {
        let event_time = chrono::DateTime::from_timestamp(
            (vested_at / 1000) as i64,
            ((vested_at % 1000) * 1_000_000) as u32,
        )
        .unwrap_or_else(chrono::Utc::now);

        Self {
            wallet_id,
            event_type: "TokensVested".to_string(),
            owner_address,
            amount: total_amount as i64,
            remaining_balance: Some(total_amount as i64), // All tokens remaining initially
            start_time: Some(start_time as i64),
            duration: Some(duration as i64),
            curve_factor: Some(curve_factor as i64),
            event_time: vested_at as i64,
            time: event_time,
            transaction_id,
        }
    }

    /// Create a new vesting event from a TokensClaimedEvent
    pub fn from_tokens_claimed_event(
        wallet_id: String,
        owner_address: String,
        claimed_amount: u64,
        remaining_balance: u64,
        claimed_at: u64,
        transaction_id: String,
    ) -> Self {
        let event_time = chrono::DateTime::from_timestamp(
            (claimed_at / 1000) as i64,
            ((claimed_at % 1000) * 1_000_000) as u32,
        )
        .unwrap_or_else(chrono::Utc::now);

        Self {
            wallet_id,
            event_type: "TokensClaimed".to_string(),
            owner_address,
            amount: claimed_amount as i64,
            remaining_balance: Some(remaining_balance as i64),
            start_time: None,   // Not relevant for claim events
            duration: None,     // Not relevant for claim events
            curve_factor: None, // Not relevant for claim events
            event_time: claimed_at as i64,
            time: event_time,
            transaction_id,
        }
    }
}

// ===========================================================================
// VESTING CONSTANTS AND UTILITIES
// ===========================================================================

/// Event types for vesting events
pub const VESTING_EVENT_TYPE_VESTED: &str = "TokensVested";
pub const VESTING_EVENT_TYPE_CLAIMED: &str = "TokensClaimed";

/// Curve factor constants
pub const CURVE_FACTOR_LINEAR: i64 = 1000; // Linear vesting
pub const CURVE_FACTOR_MIN: i64 = 100; // Minimum curve factor (logarithmic)
pub const CURVE_FACTOR_MAX: i64 = 10000; // Maximum curve factor (exponential)

/// Utility functions for working with vesting data
impl VestingWallet {
    /// Calculate the percentage of tokens claimed
    pub fn claimed_percentage(&self) -> f64 {
        if self.total_amount == 0 {
            0.0
        } else {
            (self.claimed_amount as f64 / self.total_amount as f64) * 100.0
        }
    }

    /// Check if the vesting wallet is fully claimed
    pub fn is_fully_claimed(&self) -> bool {
        self.remaining_balance == 0
    }

    /// Check if the vesting has started
    pub fn has_started(&self, current_time_ms: u64) -> bool {
        self.start_time <= (current_time_ms as i64)
    }

    /// Check if the vesting period has ended
    pub fn has_ended(&self, current_time_ms: u64) -> bool {
        let end_time = self.start_time + self.duration;
        (current_time_ms as i64) >= end_time
    }

    /// Get the vesting end time
    pub fn end_time(&self) -> i64 {
        self.start_time + self.duration
    }

    /// Calculate progress through vesting period (0.0 to 1.0)
    pub fn vesting_progress(&self, current_time_ms: u64) -> f64 {
        let current_time = current_time_ms as i64;
        if current_time <= self.start_time {
            0.0
        } else if current_time >= self.start_time + self.duration {
            1.0
        } else {
            let elapsed = current_time - self.start_time;
            elapsed as f64 / self.duration as f64
        }
    }
}

/// Extended vesting wallet with calculated fields for API responses
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VestingWalletWithStatus {
    #[serde(flatten)]
    pub wallet: VestingWallet,
    pub claimed_percentage: f64,
    pub is_fully_claimed: bool,
    pub has_started: bool,
    pub has_ended: bool,
    pub vesting_progress: f64,
    pub end_time: i64,
}

impl VestingWalletWithStatus {
    pub fn from_wallet(wallet: VestingWallet, current_time_ms: u64) -> Self {
        Self {
            claimed_percentage: wallet.claimed_percentage(),
            is_fully_claimed: wallet.is_fully_claimed(),
            has_started: wallet.has_started(current_time_ms),
            has_ended: wallet.has_ended(current_time_ms),
            vesting_progress: wallet.vesting_progress(current_time_ms),
            end_time: wallet.end_time(),
            wallet,
        }
    }
}
