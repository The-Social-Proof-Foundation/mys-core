// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::models::social_graph::NewSocialGraphRelationship;
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
