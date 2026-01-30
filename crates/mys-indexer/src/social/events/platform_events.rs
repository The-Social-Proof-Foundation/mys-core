// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Platform event types - corresponds to the Move module events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatformEventType {
    PlatformCreated,
    PlatformUpdated,
    ModeratorAdded,
    ModeratorRemoved,
    ProfileBlocked,
    ProfileUnblocked,
    PlatformApprovalChanged,
    UserJoinedPlatform,
    UserLeftPlatform,
    TokenAirdrop,
    TreasuryFunded,
    PlatformDeleted,
}

impl PlatformEventType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            s if s.contains("::PlatformCreatedEvent") => Some(Self::PlatformCreated),
            s if s.contains("::PlatformUpdatedEvent") => Some(Self::PlatformUpdated),
            s if s.contains("::ModeratorAddedEvent") => Some(Self::ModeratorAdded),
            s if s.contains("::ModeratorRemovedEvent") => Some(Self::ModeratorRemoved),
            s if s.contains("::PlatformBlockedProfileEvent") => Some(Self::ProfileBlocked),
            s if s.contains("::PlatformUnblockedProfileEvent") => Some(Self::ProfileUnblocked),
            s if s.contains("::PlatformApprovalChangedEvent") => {
                Some(Self::PlatformApprovalChanged)
            }
            s if s.contains("::UserJoinedPlatformEvent") => Some(Self::UserJoinedPlatform),
            s if s.contains("::UserLeftPlatformEvent") => Some(Self::UserLeftPlatform),
            s if s.contains("::TokenAirdropEvent") => Some(Self::TokenAirdrop),
            s if s.contains("::TreasuryFundedEvent") => Some(Self::TreasuryFunded),
            s if s.contains("::PlatformDeletedEvent") => Some(Self::PlatformDeleted),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::PlatformCreated => "PlatformCreatedEvent",
            Self::PlatformUpdated => "PlatformUpdatedEvent",
            Self::ModeratorAdded => "ModeratorAddedEvent",
            Self::ModeratorRemoved => "ModeratorRemovedEvent",
            Self::ProfileBlocked => "PlatformBlockedProfileEvent",
            Self::ProfileUnblocked => "PlatformUnblockedProfileEvent",
            Self::PlatformApprovalChanged => "PlatformApprovalChangedEvent",
            Self::UserJoinedPlatform => "UserJoinedPlatformEvent",
            Self::UserLeftPlatform => "UserLeftPlatformEvent",
            Self::TokenAirdrop => "TokenAirdropEvent",
            Self::TreasuryFunded => "TreasuryFundedEvent",
            Self::PlatformDeleted => "PlatformDeletedEvent",
        }
    }
}

/// Helper method to extract a platform ID from an event
pub fn extract_platform_id(event_data: &Value) -> Option<String> {
    // Try standard format first
    let platform_id = event_data
        .get("platform_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    if platform_id.is_some() {
        return platform_id;
    }

    // Try blockchain object format with fields.platform_id
    if let Some(fields) = event_data.get("fields") {
        if let Some(platform_id) = fields.get("platform_id") {
            if let Some(id_str) = platform_id.as_str() {
                return Some(id_str.to_string());
            }
        }
    }

    // Try content.fields format
    if let Some(content) = event_data.get("content") {
        if let Some(fields) = content.get("fields") {
            if let Some(platform_id) = fields.get("platform_id") {
                if let Some(id_str) = platform_id.as_str() {
                    return Some(id_str.to_string());
                }
            }
        }
    }

    // Try array/tuple formats that might be in the move structure
    if let Some(array) = event_data.as_array() {
        if !array.is_empty() {
            if let Some(id_str) = array[0].as_str() {
                return Some(id_str.to_string());
            }
        }
    }

    // Log failure for debugging
    tracing::warn!(
        "Failed to extract platform_id from event data: {}",
        serde_json::to_string_pretty(event_data).unwrap_or_default()
    );

    None
}

#[cfg(test)]
mod tests {
    use super::PlatformEventType;

    #[test]
    fn detects_block_and_unblock_events() {
        assert_eq!(
            PlatformEventType::from_str("0x1::platform::PlatformBlockedProfileEvent"),
            Some(PlatformEventType::ProfileBlocked)
        );
        assert_eq!(
            PlatformEventType::from_str("0x1::platform::PlatformUnblockedProfileEvent"),
            Some(PlatformEventType::ProfileUnblocked)
        );
    }
}

// =============================================================================
// PROCESS FUNCTIONS FOR CHECKPOINT PROCESSOR
// =============================================================================

use anyhow::{anyhow, Result};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::social::db::DbConnection;
use crate::social::schema::{platforms, platform_moderators, platform_blocked_profiles,
    platform_events, platform_memberships, platform_token_airdrops};
use crate::social::models::platform::{
    NewPlatform, NewPlatformModerator, NewPlatformBlockedProfile, NewPlatformEvent,
    NewPlatformMembership, NewPlatformTokenAirdrop, PlatformCreatedEvent, PlatformUpdatedEvent,
    PlatformApprovalChangedEvent, ModeratorAddedEvent, ModeratorRemovedEvent,
    PlatformBlockedProfileEvent, PlatformUnblockedProfileEvent, UserJoinedPlatformEvent,
    UserLeftPlatformEvent, TokenAirdropEvent, PlatformDeletedEvent,
    milliseconds_to_naive_datetime,
};

/// Process a PlatformCreatedEvent and insert into the database
pub async fn process_platform_created_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: PlatformCreatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PlatformCreatedEvent: {}", e))?;

    let now = Utc::now().naive_utc();

    let platform = NewPlatform {
        platform_id: event.platform_id.clone(),
        name: event.name.clone(),
        tagline: event.tagline.clone(),
        description: event.description.clone(),
        logo: event.logo.clone(),
        developer_address: event.developer.clone(),
        terms_of_service: Some(event.terms_of_service.clone()),
        privacy_policy: Some(event.privacy_policy.clone()),
        platform_names: Some(serde_json::to_value(&event.platforms).unwrap_or_default()),
        links: Some(serde_json::to_value(&event.links).unwrap_or_default()),
        status: event.status.status as i16,
        release_date: Some(event.release_date.clone()),
        shutdown_date: event.shutdown_date.clone(),
        created_at: now,
        updated_at: now,
        is_approved: false, // Platforms start unapproved
        approval_changed_at: None,
        approved_by: None,
        wants_dao_governance: event.wants_dao_governance,
        governance_registry_id: event.governance_registry_id.clone(),
        delegate_count: event.delegate_count.map(|v| v as i64),
        delegate_term_epochs: event.delegate_term_epochs.map(|v| v as i64),
        max_votes_per_user: event.max_votes_per_user.map(|v| v as i64),
        min_on_chain_age_days: event.min_on_chain_age_days.map(|v| v as i64),
        proposal_submission_cost: event.proposal_submission_cost.map(|v| v as i64),
        quadratic_base_cost: event.quadratic_base_cost.map(|v| v as i64),
        quorum_votes: event.quorum_votes.map(|v| v as i64),
        voting_period_epochs: event.voting_period_epochs.map(|v| v as i64),
        treasury: event.treasury.map(|v| v as i64),
        version: event.version.map(|v| v as i64),
        primary_category: event.primary_category.clone(),
        secondary_category: event.secondary_category.clone(),
        deleted_at: None,
    };

    diesel::insert_into(platforms::table)
        .values(&platform)
        .on_conflict(platforms::platform_id)
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert platform: {}", e))?;

    // Log the event
    let platform_event = NewPlatformEvent {
        event_type: "PlatformCreated".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed PlatformCreatedEvent for platform_id: {}", event.platform_id);
    Ok(())
}

/// Process a PlatformUpdatedEvent and update the database
pub async fn process_platform_updated_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: PlatformUpdatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PlatformUpdatedEvent: {}", e))?;

    let updated_at = milliseconds_to_naive_datetime(event.updated_at);

    diesel::update(platforms::table)
        .filter(platforms::platform_id.eq(&event.platform_id))
        .set((
            platforms::name.eq(&event.name),
            platforms::tagline.eq(&event.tagline),
            platforms::description.eq(Some(&event.description)),
            platforms::terms_of_service.eq(Some(&event.terms_of_service)),
            platforms::privacy_policy.eq(Some(&event.privacy_policy)),
            platforms::platform_names.eq(Some(serde_json::to_value(&event.platforms).unwrap_or_default())),
            platforms::links.eq(Some(serde_json::to_value(&event.links).unwrap_or_default())),
            platforms::status.eq(event.status.status as i16),
            platforms::release_date.eq(Some(&event.release_date)),
            platforms::shutdown_date.eq(&event.shutdown_date),
            platforms::updated_at.eq(updated_at),
            platforms::primary_category.eq(&event.primary_category),
            platforms::secondary_category.eq(&event.secondary_category),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to update platform: {}", e))?;

    // Log the event
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "PlatformUpdated".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed PlatformUpdatedEvent for platform_id: {}", event.platform_id);
    Ok(())
}

/// Process a PlatformApprovalChangedEvent and update the database
pub async fn process_platform_approval_changed_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: PlatformApprovalChangedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PlatformApprovalChangedEvent: {}", e))?;

    let changed_at = milliseconds_to_naive_datetime(event.changed_at);

    diesel::update(platforms::table)
        .filter(platforms::platform_id.eq(&event.platform_id))
        .set((
            platforms::is_approved.eq(event.is_approved),
            platforms::approval_changed_at.eq(Some(changed_at)),
            platforms::approved_by.eq(Some(&event.approved_by)),
            platforms::updated_at.eq(changed_at),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to update platform approval: {}", e))?;

    // Log the event
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "ApprovalChanged".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: event.reasoning.clone(),
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed PlatformApprovalChangedEvent for platform_id: {} approved: {}",
        event.platform_id, event.is_approved);
    Ok(())
}

/// Process a ModeratorAddedEvent and insert into the database
pub async fn process_moderator_added_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: ModeratorAddedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse ModeratorAddedEvent: {}", e))?;

    let now = Utc::now().naive_utc();

    let moderator = NewPlatformModerator {
        platform_id: event.platform_id.clone(),
        moderator_address: event.moderator_address.clone(),
        added_by: event.added_by.clone(),
        created_at: now,
    };

    diesel::insert_into(platform_moderators::table)
        .values(&moderator)
        .on_conflict((platform_moderators::platform_id, platform_moderators::moderator_address))
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert moderator: {}", e))?;

    // Log the event
    let platform_event = NewPlatformEvent {
        event_type: "ModeratorAdded".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed ModeratorAddedEvent for platform: {} moderator: {}",
        event.platform_id, event.moderator_address);
    Ok(())
}

/// Process a ModeratorRemovedEvent and delete from the database
pub async fn process_moderator_removed_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: ModeratorRemovedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse ModeratorRemovedEvent: {}", e))?;

    diesel::delete(platform_moderators::table)
        .filter(platform_moderators::platform_id.eq(&event.platform_id))
        .filter(platform_moderators::moderator_address.eq(&event.moderator_address))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to delete moderator: {}", e))?;

    // Log the event
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "ModeratorRemoved".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed ModeratorRemovedEvent for platform: {} moderator: {}",
        event.platform_id, event.moderator_address);
    Ok(())
}

/// Process a PlatformBlockedProfileEvent and insert into the database
pub async fn process_platform_blocked_profile_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: PlatformBlockedProfileEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PlatformBlockedProfileEvent: {}", e))?;

    let now = Utc::now().naive_utc();

    let blocked = NewPlatformBlockedProfile {
        platform_id: event.platform_id.clone(),
        wallet_address: event.profile_id.clone(), // profile_id is the wallet address
        blocked_by: event.blocked_by.clone(),
        created_at: now,
    };

    diesel::insert_into(platform_blocked_profiles::table)
        .values(&blocked)
        .on_conflict((platform_blocked_profiles::platform_id, platform_blocked_profiles::wallet_address))
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert platform blocked profile: {}", e))?;

    // Log the event
    let platform_event = NewPlatformEvent {
        event_type: "PlatformBlockedProfile".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed PlatformBlockedProfileEvent for platform: {} profile: {}",
        event.platform_id, event.profile_id);
    Ok(())
}

/// Process a PlatformUnblockedProfileEvent and delete from the database
pub async fn process_platform_unblocked_profile_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: PlatformUnblockedProfileEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PlatformUnblockedProfileEvent: {}", e))?;

    diesel::delete(platform_blocked_profiles::table)
        .filter(platform_blocked_profiles::platform_id.eq(&event.platform_id))
        .filter(platform_blocked_profiles::wallet_address.eq(&event.profile_id))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to delete platform blocked profile: {}", e))?;

    // Log the event
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "PlatformUnblockedProfile".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed PlatformUnblockedProfileEvent for platform: {} profile: {}",
        event.platform_id, event.profile_id);
    Ok(())
}

/// Process a UserJoinedPlatformEvent and insert into the database
pub async fn process_user_joined_platform_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: UserJoinedPlatformEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse UserJoinedPlatformEvent: {}", e))?;

    let joined_at = milliseconds_to_naive_datetime(event.timestamp);

    let membership = NewPlatformMembership {
        platform_id: event.platform_id.clone(),
        wallet_address: event.wallet_address.clone(),
        joined_at,
    };

    diesel::insert_into(platform_memberships::table)
        .values(&membership)
        .on_conflict((platform_memberships::platform_id, platform_memberships::wallet_address))
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert platform membership: {}", e))?;

    // Log the event
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "UserJoinedPlatform".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed UserJoinedPlatformEvent for platform: {} user: {}",
        event.platform_id, event.wallet_address);
    Ok(())
}

/// Process a UserLeftPlatformEvent and delete from the database
pub async fn process_user_left_platform_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: UserLeftPlatformEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse UserLeftPlatformEvent: {}", e))?;

    diesel::delete(platform_memberships::table)
        .filter(platform_memberships::platform_id.eq(&event.platform_id))
        .filter(platform_memberships::wallet_address.eq(&event.wallet_address))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to delete platform membership: {}", e))?;

    // Log the event
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "UserLeftPlatform".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed UserLeftPlatformEvent for platform: {} user: {}",
        event.platform_id, event.wallet_address);
    Ok(())
}

/// Process a TokenAirdropEvent and insert into the database
pub async fn process_token_airdrop_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: TokenAirdropEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse TokenAirdropEvent: {}", e))?;

    let now = Utc::now().naive_utc();

    let airdrop = NewPlatformTokenAirdrop {
        platform_id: event.platform_id.clone(),
        recipient: event.recipient.clone(),
        amount: event.amount as i64,
        reason_code: event.reason_code as i16,
        executed_by: event.executed_by.clone(),
        timestamp: event.timestamp as i64,
        created_at: now,
        event_id: Some(event_id.to_string()),
    };

    diesel::insert_into(platform_token_airdrops::table)
        .values(&airdrop)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert token airdrop: {}", e))?;

    // Log the event
    let platform_event = NewPlatformEvent {
        event_type: "TokenAirdrop".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: None,
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed TokenAirdropEvent for platform: {} recipient: {} amount: {}",
        event.platform_id, event.recipient, event.amount);
    Ok(())
}

/// Process a PlatformDeletedEvent and mark as deleted in the database
pub async fn process_platform_deleted_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
) -> Result<()> {
    let event: PlatformDeletedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PlatformDeletedEvent: {}", e))?;

    let deleted_at = milliseconds_to_naive_datetime(event.timestamp);

    diesel::update(platforms::table)
        .filter(platforms::platform_id.eq(&event.platform_id))
        .set((
            platforms::deleted_at.eq(Some(deleted_at)),
            platforms::updated_at.eq(deleted_at),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to mark platform as deleted: {}", e))?;

    // Log the event
    let now = Utc::now().naive_utc();
    let platform_event = NewPlatformEvent {
        event_type: "PlatformDeleted".to_string(),
        platform_id: event.platform_id.clone(),
        event_data: data.clone(),
        event_id: Some(event_id.to_string()),
        created_at: now,
        reasoning: event.reasoning.clone(),
    };

    diesel::insert_into(platform_events::table)
        .values(&platform_event)
        .execute(conn)
        .await
        .ok();

    tracing::info!("Processed PlatformDeletedEvent for platform_id: {}", event.platform_id);
    Ok(())
}
