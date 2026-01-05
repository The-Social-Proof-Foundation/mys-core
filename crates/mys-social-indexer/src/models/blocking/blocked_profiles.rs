// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::schema::blocked_profiles;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Blocked profile model - represents current blocking relationships with rich profile data
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct BlockedProfile {
    pub id: i32,
    pub blocker_address: String,
    pub blocked_address: String,
    // Rich profile data for performance (denormalized from profiles table)
    pub blocked_profile_id: Option<String>,
    pub blocked_username: String,
    pub blocked_display_name: Option<String>,
    pub blocked_profile_photo: Option<String>,
    // Blocking metadata
    pub first_blocked_at: NaiveDateTime,
    pub last_blocked_at: NaiveDateTime,
    pub total_block_count: i32,
}

/// DTO for inserting a new blocked profile with rich profile data
#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct NewBlockedProfile {
    pub blocker_address: String,
    pub blocked_address: String,
    // Rich profile data for performance (denormalized from profiles table)
    pub blocked_profile_id: Option<String>,
    pub blocked_username: String,
    pub blocked_display_name: Option<String>,
    pub blocked_profile_photo: Option<String>,
    // Blocking metadata
    pub first_blocked_at: NaiveDateTime,
    pub last_blocked_at: NaiveDateTime,
    pub total_block_count: i32,
}

/// DTO for updating a blocked profile
#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct UpdateBlockedProfile {
    // Rich profile data updates (in case profile info changes)
    pub blocked_profile_id: Option<Option<String>>,
    pub blocked_username: Option<String>,
    pub blocked_display_name: Option<Option<String>>,
    pub blocked_profile_photo: Option<Option<String>>,
    // Blocking metadata updates
    pub last_blocked_at: Option<NaiveDateTime>,
    pub total_block_count: Option<i32>,
}

impl NewBlockedProfile {
    /// Create a new blocked profile record with rich profile data
    pub fn new(
        blocker_address: String,
        blocked_address: String,
        blocked_profile_id: Option<String>,
        blocked_username: String,
        blocked_display_name: Option<String>,
        blocked_profile_photo: Option<String>,
        blocked_at: NaiveDateTime,
    ) -> Self {
        Self {
            blocker_address,
            blocked_address,
            blocked_profile_id,
            blocked_username,
            blocked_display_name,
            blocked_profile_photo,
            first_blocked_at: blocked_at,
            last_blocked_at: blocked_at,
            total_block_count: 1,
        }
    }
}

/// Enriched blocked profile information for API responses (now maps directly to table fields)
#[derive(Debug, Serialize)]
pub struct EnrichedBlockedProfile {
    // Profile Identity
    pub profile_id: Option<String>,   // Blockchain profile ID
    pub wallet_address: String,       // Wallet address
    pub username: String,             // @username
    pub display_name: Option<String>, // Display name

    // Profile Media
    pub profile_photo: Option<String>, // Profile photo URL

    // Blocking Metadata
    pub blocked_at: NaiveDateTime,          // When last blocked
    pub first_blocked_at: NaiveDateTime,    // When first blocked
    pub total_block_count: i32,             // Times blocked
}

impl From<BlockedProfile> for EnrichedBlockedProfile {
    /// Convert from BlockedProfile model to API response format
    fn from(blocked_profile: BlockedProfile) -> Self {
        Self {
            profile_id: blocked_profile.blocked_profile_id,
            wallet_address: blocked_profile.blocked_address,
            username: blocked_profile.blocked_username,
            display_name: blocked_profile.blocked_display_name,
            profile_photo: blocked_profile.blocked_profile_photo,
            blocked_at: blocked_profile.last_blocked_at,
            first_blocked_at: blocked_profile.first_blocked_at,
            total_block_count: blocked_profile.total_block_count,
        }
    }
}

/// Paginated response for blocked profiles
#[derive(Debug, Serialize)]
pub struct PaginatedBlockedProfilesResponse {
    pub blocked_profiles: Vec<EnrichedBlockedProfile>,
    pub pagination: PaginationMetadata,
    pub total_count: i64,
}

/// Pagination metadata for API responses
#[derive(Debug, Serialize)]
pub struct PaginationMetadata {
    pub limit: i32,
    pub offset: Option<i32>,    // For offset-based pagination
    pub cursor: Option<String>, // For cursor-based pagination
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

/// Query parameters for listing blocked profiles
#[derive(Debug, Deserialize)]
pub struct BlockedListQuery {
    /// Optional sort: latest | earliest | alphabetical
    pub sort: Option<String>,
    /// Optional search across username, display name, and wallet address
    pub search: Option<String>,
}
