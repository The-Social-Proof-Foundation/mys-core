// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::social::schema::{social_graph_events, social_graph_relationships};
use crate::social::models::universal_user::UniversalUserResult;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Model for a social graph relationship (follow)
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = social_graph_relationships)]
pub struct SocialGraphRelationship {
    pub id: i32,
    pub follower_address: String,
    pub following_address: String,
    pub created_at: NaiveDateTime,
}

/// DTO for creating a new social graph relationship
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_graph_relationships)]
pub struct NewSocialGraphRelationship {
    pub follower_address: String,
    pub following_address: String,
    pub created_at: NaiveDateTime,
}

/// Model for social graph events
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = social_graph_events)]
pub struct SocialGraphEvent {
    pub id: i32,
    pub event_type: String,
    pub follower_address: String,
    pub following_address: String,
    pub created_at: NaiveDateTime,
    pub event_id: Option<String>, // Changed from blockchain_tx_hash to event_id
    pub raw_event_data: Option<serde_json::Value>,
}

/// DTO for creating a new social graph event
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_graph_events)]
pub struct NewSocialGraphEvent {
    pub event_type: String,
    pub follower_address: String,
    pub following_address: String,
    pub created_at: NaiveDateTime,
    pub event_id: Option<String>, // Changed from blockchain_tx_hash to event_id
    pub raw_event_data: Option<serde_json::Value>,
}

/// DTO for querying followers or following with profile details
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowDetail {
    // Profile ID in the database
    pub id: i32,
    // Profile ID in the blockchain
    pub profile_id: Option<String>,
    // Universal user result with profile, badge, and SPT info
    #[serde(flatten)]
    pub user: UniversalUserResult,
    // Whether this profile follows back the requesting profile
    pub follows_back: bool,
    // Whether the requesting profile is following this profile
    pub is_following: bool,
}

/// Query parameters for paginating followers/following lists
#[derive(Debug, Deserialize)]
pub struct FollowsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    /// Optional viewer profile ID to calculate is_following/follows_back from viewer's perspective
    pub viewer_id: Option<String>,
    /// Optional sort: latest | earliest | alphabetical
    pub sort: Option<String>,
    /// Optional search across username, display_name, and wallet address
    pub search: Option<String>,
}
