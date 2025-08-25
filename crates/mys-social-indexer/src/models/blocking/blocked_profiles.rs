// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::blocked_profiles;

/// Blocked profile model - represents current blocking relationships
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct BlockedProfile {
    pub id: i32,
    pub blocker_address: String,
    pub blocked_address: String,
    pub block_list_address: Option<String>,
    pub first_blocked_at: NaiveDateTime,
    pub last_blocked_at: NaiveDateTime,
    pub total_block_count: i32,
}

/// DTO for inserting a new blocked profile
#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct NewBlockedProfile {
    pub blocker_address: String,
    pub blocked_address: String,
    pub block_list_address: Option<String>,
    pub first_blocked_at: NaiveDateTime,
    pub last_blocked_at: NaiveDateTime,
    pub total_block_count: i32,
}

/// DTO for updating a blocked profile
#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct UpdateBlockedProfile {
    pub block_list_address: Option<String>,
    pub last_blocked_at: Option<NaiveDateTime>,
    pub total_block_count: Option<i32>,
}

impl NewBlockedProfile {
    /// Create a new blocked profile record
    pub fn new(
        blocker_address: String,
        blocked_address: String,
        block_list_address: Option<String>,
        blocked_at: NaiveDateTime,
    ) -> Self {
        Self {
            blocker_address,
            blocked_address,
            block_list_address,
            first_blocked_at: blocked_at,
            last_blocked_at: blocked_at,
            total_block_count: 1,
        }
    }
}

/// Enriched blocked profile information for API responses
#[derive(Debug, Serialize)]
pub struct EnrichedBlockedProfile {
    // Profile Identity
    pub profile_id: Option<String>,           // Blockchain profile ID
    pub wallet_address: String,               // Wallet address
    pub username: String,                     // @username
    pub display_name: Option<String>,         // Display name
    
    // Profile Media
    pub profile_photo: Option<String>,        // Profile photo URL
    
    // Blocking Metadata  
    pub blocked_at: NaiveDateTime,            // When last blocked
    pub first_blocked_at: NaiveDateTime,      // When first blocked
    pub total_block_count: i32,               // Times blocked
    pub block_list_address: Option<String>,   // Block list object ID
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
    pub offset: Option<i32>,                  // For offset-based pagination
    pub cursor: Option<String>,               // For cursor-based pagination  
    pub has_next_page: bool,
    pub has_previous_page: bool,
}
