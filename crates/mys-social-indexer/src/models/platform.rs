// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::schema::{
    platform_blocked_profiles, platform_events, platform_memberships, platform_moderators,
    platforms,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Platform status constants
pub const PLATFORM_STATUS_DEVELOPMENT: i16 = 0;
pub const PLATFORM_STATUS_ALPHA: i16 = 1;
pub const PLATFORM_STATUS_BETA: i16 = 2;
pub const PLATFORM_STATUS_LIVE: i16 = 3;
pub const PLATFORM_STATUS_MAINTENANCE: i16 = 4;
pub const PLATFORM_STATUS_SUNSET: i16 = 5;
pub const PLATFORM_STATUS_SHUTDOWN: i16 = 6;

/// Platform model
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = platforms)]
pub struct Platform {
    pub id: i32,
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub developer_address: String,
    pub terms_of_service: Option<String>,
    pub privacy_policy: Option<String>,
    pub platform_names: Option<serde_json::Value>,
    pub links: Option<serde_json::Value>,
    pub status: i16,
    pub release_date: Option<String>,
    pub shutdown_date: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_approved: bool,
    pub approval_changed_at: Option<NaiveDateTime>,
    pub approved_by: Option<String>,
    pub wants_dao_governance: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub delegate_count: Option<i64>,
    pub delegate_term_epochs: Option<i64>,
    pub max_votes_per_user: Option<i64>,
    pub min_on_chain_age_days: Option<i64>,
    pub proposal_submission_cost: Option<i64>,
    pub quadratic_base_cost: Option<i64>,
    pub quorum_votes: Option<i64>,
    pub voting_period_epochs: Option<i64>,
    pub treasury: Option<i64>,
    pub version: Option<i64>,
}

/// DTO for inserting a new platform
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platforms)]
pub struct NewPlatform {
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub developer_address: String,
    pub terms_of_service: Option<String>,
    pub privacy_policy: Option<String>,
    pub platform_names: Option<serde_json::Value>,
    pub links: Option<serde_json::Value>,
    pub status: i16,
    pub release_date: Option<String>,
    pub shutdown_date: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_approved: bool,
    pub approval_changed_at: Option<NaiveDateTime>,
    pub approved_by: Option<String>,
    pub wants_dao_governance: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub delegate_count: Option<i64>,
    pub delegate_term_epochs: Option<i64>,
    pub max_votes_per_user: Option<i64>,
    pub min_on_chain_age_days: Option<i64>,
    pub proposal_submission_cost: Option<i64>,
    pub quadratic_base_cost: Option<i64>,
    pub quorum_votes: Option<i64>,
    pub voting_period_epochs: Option<i64>,
    pub treasury: Option<i64>,
    pub version: Option<i64>,
}

/// DTO for updating a platform
#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = platforms)]
pub struct UpdatePlatform {
    pub name: Option<String>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub terms_of_service: Option<String>,
    pub privacy_policy: Option<String>,
    pub platform_names: Option<serde_json::Value>,
    pub links: Option<serde_json::Value>,
    pub status: Option<i16>,
    pub release_date: Option<String>,
    pub shutdown_date: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_approved: Option<bool>,
    pub approval_changed_at: Option<NaiveDateTime>,
    pub approved_by: Option<String>,
    pub wants_dao_governance: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub delegate_count: Option<i64>,
    pub delegate_term_epochs: Option<i64>,
    pub max_votes_per_user: Option<i64>,
    pub min_on_chain_age_days: Option<i64>,
    pub proposal_submission_cost: Option<i64>,
    pub quadratic_base_cost: Option<i64>,
    pub quorum_votes: Option<i64>,
    pub voting_period_epochs: Option<i64>,
    pub treasury: Option<i64>,
    pub version: Option<i64>,
}

/// Platform moderator model
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = platform_moderators)]
pub struct PlatformModerator {
    pub id: i32,
    pub platform_id: String,
    pub moderator_address: String,
    pub added_by: String,
    pub created_at: NaiveDateTime,
}

/// DTO for inserting a new platform moderator
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_moderators)]
pub struct NewPlatformModerator {
    pub platform_id: String,
    pub moderator_address: String,
    pub added_by: String,
    pub created_at: NaiveDateTime,
}

/// Platform blocked profile model
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = platform_blocked_profiles)]
pub struct PlatformBlockedProfile {
    pub id: i32,
    pub platform_id: String,
    pub profile_id: String,
    pub blocked_by: String,
    pub created_at: NaiveDateTime,
}

/// DTO for inserting a new platform blocked profile
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_blocked_profiles)]
pub struct NewPlatformBlockedProfile {
    pub platform_id: String,
    pub profile_id: String,
    pub blocked_by: String,
    pub created_at: NaiveDateTime,
}

/// Platform event model
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = platform_events)]
pub struct PlatformEvent {
    pub id: i32,
    pub event_type: String,
    pub platform_id: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub reasoning: Option<String>,
}

/// DTO for inserting a new platform event
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_events)]
pub struct NewPlatformEvent {
    pub event_type: String,
    pub platform_id: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub reasoning: Option<String>,
}

/// Platform with related data for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformWithDetails {
    // Platform details
    pub id: i32,
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub developer_address: String,
    pub terms_of_service: Option<String>,
    pub privacy_policy: Option<String>,
    pub platform_names: Option<Vec<String>>,
    pub links: Option<Vec<String>>,
    pub status: i16,
    pub status_text: String,
    pub release_date: Option<String>,
    pub shutdown_date: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_approved: bool,
    pub approval_changed_at: Option<NaiveDateTime>,
    pub approved_by: Option<String>,
    pub wants_dao_governance: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub delegate_count: Option<i64>,
    pub delegate_term_epochs: Option<i64>,
    pub max_votes_per_user: Option<i64>,
    pub min_on_chain_age_days: Option<i64>,
    pub proposal_submission_cost: Option<i64>,
    pub quadratic_base_cost: Option<i64>,
    pub quorum_votes: Option<i64>,
    pub voting_period_epochs: Option<i64>,
    pub treasury: Option<i64>,
    pub version: Option<i64>,
    // Related data
    pub moderator_count: i64,
    pub blocked_profiles_count: i64,
}

impl PlatformWithDetails {
    // Helper to convert platform status code to text
    pub fn status_to_text(status: i16) -> String {
        match status {
            PLATFORM_STATUS_DEVELOPMENT => "Development".to_string(),
            PLATFORM_STATUS_ALPHA => "Alpha".to_string(),
            PLATFORM_STATUS_BETA => "Beta".to_string(),
            PLATFORM_STATUS_LIVE => "Live".to_string(),
            PLATFORM_STATUS_MAINTENANCE => "Maintenance".to_string(),
            PLATFORM_STATUS_SUNSET => "Sunset".to_string(),
            PLATFORM_STATUS_SHUTDOWN => "Shutdown".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

/// Events from platform.move
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformCreatedEvent {
    #[serde(default, deserialize_with = "crate::events::event_utils::deserialize_platform_id")]
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    #[serde(default)]
    pub description: Option<String>,
    pub developer: String,
    #[serde(default)]
    pub logo: Option<String>,
    pub terms_of_service: String,
    pub privacy_policy: String,
    pub platforms: Vec<String>,
    pub links: Vec<String>,
    pub status: PlatformStatus,
    pub release_date: String,
    #[serde(default)]
    pub shutdown_date: Option<String>,
    #[serde(default)]
    pub wants_dao_governance: Option<bool>,
    #[serde(default)]
    pub governance_registry_id: Option<String>,
    #[serde(default, deserialize_with = "deserialize_u64_optional")]
    pub delegate_count: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_u64_optional")]
    pub delegate_term_epochs: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_u64_optional")]
    pub max_votes_per_user: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_u64_optional")]
    pub min_on_chain_age_days: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_u64_optional")]
    pub proposal_submission_cost: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_u64_optional")]
    pub quadratic_base_cost: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_u64_optional")]
    pub quorum_votes: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_u64_optional")]
    pub voting_period_epochs: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_u64_optional")]
    pub treasury: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_u64_optional")]
    pub version: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformApprovalChangedEvent {
    pub platform_id: String,
    #[serde(alias = "approved")]
    pub is_approved: bool,
    #[serde(alias = "changed_by")]
    pub approved_by: String,
    #[serde(default, deserialize_with = "deserialize_timestamp_optional")]
    pub changed_at: u64,
    #[serde(default)]
    pub reasoning: Option<String>,
}

// Helper function to convert milliseconds timestamp to NaiveDateTime
// All timestamp deserializers return milliseconds, so this ensures consistent conversion
// Validates that timestamp is not 0 (epoch) and is within reasonable range (2020-2100)
pub fn milliseconds_to_naive_datetime(ms: u64) -> chrono::NaiveDateTime {
    // Validate timestamp is not 0 and is within reasonable range
    let min_timestamp_ms = 1577836800000u64; // 2020-01-01 00:00:00 UTC
    let max_timestamp_ms = 4102444800000u64; // 2100-01-01 00:00:00 UTC
    
    if ms == 0 || ms < min_timestamp_ms || ms > max_timestamp_ms {
        // Invalid timestamp, use current time
        tracing::warn!(
            "Invalid timestamp {} (epoch: {}, min: {}, max: {}), using current time",
            ms,
            ms == 0,
            ms < min_timestamp_ms,
            ms > max_timestamp_ms
        );
        chrono::Utc::now().naive_utc()
    } else {
        chrono::DateTime::from_timestamp((ms / 1000) as i64, ((ms % 1000) * 1_000_000) as u32)
            .unwrap_or_else(|| {
                tracing::warn!("Failed to convert timestamp {} to NaiveDateTime, using current time", ms);
                chrono::Utc::now()
            })
            .naive_utc()
    }
}

// Standard deserializer for timestamps that accepts both string and number formats
// Returns milliseconds (consistent with blockchain timestamp format)
// Falls back to current time if parsing fails
fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct TimestampVisitor;

    impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number or string representing a timestamp in milliseconds")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            // Assume value is already in milliseconds
            Ok(value)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value.parse::<u64>() {
                Ok(ts) => Ok(ts),
                Err(e) => {
                    // Log the error but don't fail - use current time instead
                    tracing::warn!(
                        "Failed to parse timestamp string '{}': {}. Using current time instead.",
                        value,
                        e
                    );
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64;
                    Ok(current_time)
                }
            }
        }

        // Handle null or missing values
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_unit()
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            Ok(current_time)
        }
    }

    deserializer.deserialize_any(TimestampVisitor)
}

// Version that handles missing fields or null values
// Returns milliseconds (consistent with blockchain timestamp format)
// Treats 0 as invalid (missing) and returns current time instead
fn deserialize_timestamp_optional<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Try to deserialize, falling back to current time instead of 0
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    Option::deserialize(deserializer).map(|opt_val: Option<serde_json::Value>| match opt_val {
        Some(serde_json::Value::Number(n)) => {
            if let Some(val) = n.as_u64() {
                // Treat 0 as invalid/missing, use current time instead
                if val == 0 {
                    tracing::warn!("Received timestamp value 0, treating as missing and using current time");
                    current_time
                } else {
                    val
                }
            } else {
                current_time
            }
        }
        Some(serde_json::Value::String(s)) => {
            match s.parse::<u64>() {
                Ok(val) => {
                    // Treat 0 as invalid/missing, use current time instead
                    if val == 0 {
                        tracing::warn!("Received timestamp string '0', treating as missing and using current time");
                        current_time
                    } else {
                        val
                    }
                }
                Err(_) => current_time,
            }
        }
        _ => current_time,
    })
}

// Deserializer for u64 values that may come as strings or numbers from blockchain
fn deserialize_u64_optional<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::deserialize(deserializer).map(|opt_val: Option<serde_json::Value>| match opt_val {
        Some(serde_json::Value::Number(n)) => n.as_u64(),
        Some(serde_json::Value::String(s)) => s.parse::<u64>().ok(),
        _ => None,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformUpdatedEvent {
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: String,
    pub terms_of_service: String,
    pub privacy_policy: String,
    pub platforms: Vec<String>,
    pub links: Vec<String>,
    pub status: PlatformStatus,
    pub release_date: String,
    pub shutdown_date: Option<String>,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub updated_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformStatus {
    #[serde(deserialize_with = "crate::events::event_utils::deserialize_status_field")]
    pub status: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModeratorAddedEvent {
    pub platform_id: String,
    pub moderator_address: String,
    pub added_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModeratorRemovedEvent {
    pub platform_id: String,
    pub moderator_address: String,
    pub removed_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformBlockedProfileEvent {
    pub platform_id: String,
    pub profile_id: String,
    pub blocked_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformUnblockedProfileEvent {
    pub platform_id: String,
    pub profile_id: String,
    pub unblocked_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJoinedPlatformEvent {
    pub profile_id: String,
    pub platform_id: String,
    #[serde(default)]
    pub user: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLeftPlatformEvent {
    pub profile_id: String,
    pub platform_id: String,
    #[serde(default)]
    pub user: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryFundedEvent {
    pub platform_id: String,
    #[serde(deserialize_with = "deserialize_u64_from_string_or_number")]
    pub amount: u64,
    pub funded_by: String,
    #[serde(deserialize_with = "deserialize_u64_from_string_or_number")]
    pub new_balance: u64,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: u64,
}

// Helper deserializer for u64 that accepts both string and number
fn deserialize_u64_from_string_or_number<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Visitor;
    struct U64Visitor;

    impl<'de> Visitor<'de> for U64Visitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number or string representing a u64")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value.parse::<u64>().map_err(serde::de::Error::custom)
        }
    }

    deserializer.deserialize_any(U64Visitor)
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_memberships)]
pub struct NewPlatformMembership {
    pub platform_id: String,
    pub profile_id: String,
    pub joined_at: NaiveDateTime,
}

// Note: PlatformRelationship, NewPlatformRelationship, and UpdatePlatformRelationship
// have been removed in favor of using platform_memberships table
