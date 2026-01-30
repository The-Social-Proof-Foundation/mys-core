// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::social::models::social_graph::NewSocialGraphRelationship;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Event emitted when a profile follows another profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowEvent {
    /// Address of the follower
    pub follower: String,
    /// Address of the user being followed
    pub following: String,
    /// Optional timestamp - if not provided, current time will be used
    #[serde(default)]
    pub timestamp: Option<u64>,
}

/// Event emitted when a profile unfollows another profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfollowEvent {
    /// Address of the follower who is unfollowing
    pub follower: String,
    /// Address of the user being unfollowed
    pub unfollowed: String,
    /// Optional timestamp - if not provided, current time will be used
    #[serde(default)]
    pub timestamp: Option<u64>,
}

impl FollowEvent {
    /// Convert the FollowEvent to a NewSocialGraphRelationship database model
    pub fn into_relationship(&self) -> Result<NewSocialGraphRelationship> {
        // Use provided timestamp or current time
        let timestamp = self.timestamp.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        });

        let created_at = DateTime::from_timestamp(timestamp as i64, 0)
            .unwrap_or(Utc::now())
            .naive_utc();

        Ok(NewSocialGraphRelationship {
            follower_address: self.follower.clone(),
            following_address: self.following.clone(),
            created_at,
        })
    }

}

impl UnfollowEvent {
    // UpdateProfile methods removed - count updates are now handled by database triggers
}

// =============================================================================
// PROCESS FUNCTIONS FOR CHECKPOINT PROCESSOR
// =============================================================================

use anyhow::anyhow;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::social::db::DbConnection;
use crate::social::schema::{social_graph_relationships, social_graph_events};
use crate::social::models::social_graph::NewSocialGraphEvent;

/// Process a FollowEvent and insert into the database
pub async fn process_follow_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    let event: FollowEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse FollowEvent: {}", e))?;

    let relationship = event.into_relationship()?;

    // Insert the relationship (on conflict do nothing to handle duplicates)
    diesel::insert_into(social_graph_relationships::table)
        .values(&relationship)
        .on_conflict((
            social_graph_relationships::follower_address,
            social_graph_relationships::following_address,
        ))
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert follow relationship: {}", e))?;

    // Log the event for audit trail
    let graph_event = NewSocialGraphEvent {
        event_type: "follow".to_string(),
        follower_address: event.follower.clone(),
        following_address: event.following.clone(),
        created_at: relationship.created_at,
        event_id: Some(event_id.to_string()),
        raw_event_data: Some(data.clone()),
    };

    diesel::insert_into(social_graph_events::table)
        .values(&graph_event)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert follow event: {}", e))?;

    tracing::info!("Processed FollowEvent: {} -> {}", event.follower, event.following);
    Ok(())
}

/// Process an UnfollowEvent and remove from the database
pub async fn process_unfollow_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    let event: UnfollowEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse UnfollowEvent: {}", e))?;

    let now = Utc::now().naive_utc();

    // Delete the relationship
    let deleted = diesel::delete(social_graph_relationships::table)
        .filter(social_graph_relationships::follower_address.eq(&event.follower))
        .filter(social_graph_relationships::following_address.eq(&event.unfollowed))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to delete follow relationship: {}", e))?;

    // Log the event for audit trail
    let graph_event = NewSocialGraphEvent {
        event_type: "unfollow".to_string(),
        follower_address: event.follower.clone(),
        following_address: event.unfollowed.clone(),
        created_at: now,
        event_id: Some(event_id.to_string()),
        raw_event_data: Some(data.clone()),
    };

    diesel::insert_into(social_graph_events::table)
        .values(&graph_event)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert unfollow event: {}", e))?;

    tracing::info!("Processed UnfollowEvent: {} unfollowed {} (deleted {} relationships)",
        event.follower, event.unfollowed, deleted);
    Ok(())
}
