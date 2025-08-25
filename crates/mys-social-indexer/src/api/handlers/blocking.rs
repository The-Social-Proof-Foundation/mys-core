// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use tracing::{debug, error};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::db::DbPool;
use crate::schema::{blocked_profiles, profiles};
use crate::models::blocking::{EnrichedBlockedProfile, PaginatedBlockedProfilesResponse, PaginationMetadata};

/// Response type for blocked platforms list
#[derive(Debug, Serialize)]
pub struct BlockedPlatformsResponse {
    pub blocked_platforms: Vec<PlatformBlockInfo>,
    pub total: i64,
}

/// Platform block information
#[derive(Debug, Serialize)]
pub struct PlatformBlockInfo {
    pub platform_id: String,
    pub blocked_at: chrono::NaiveDateTime,
}

/// Response for block check
#[derive(Debug, Serialize)]
pub struct BlockCheckResponse {
    pub is_blocked: bool,
}

/// Get profiles blocked by a user with rich profile information and pagination
pub async fn get_blocked_profiles(
    Path(profile_id): Path<String>,
    State(pool): State<DbPool>,
) -> Result<Json<PaginatedBlockedProfilesResponse>, StatusCode> {
    debug!("Getting enriched profiles blocked by profile_id: {}", profile_id);
    
    // Input validation
    if profile_id.trim().is_empty() {
        debug!("Invalid profile_id: empty string");
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Basic length validation to prevent potential attacks
    if profile_id.len() > 256 {
        debug!("Invalid profile_id: too long");
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    // Enhanced query: Join blocked_profiles with profiles to get rich profile information
    // First, determine if profile_id is a wallet address or profile_id
    let blocker_address = if profile_id.starts_with("0x") {
        // It's already a wallet address
        profile_id.clone()
    } else {
        // It might be a profile ID, look up the wallet address
        match profiles::table
            .filter(profiles::profile_id.eq(&profile_id))
            .select(profiles::owner_address)
            .first::<String>(&mut conn)
            .await
        {
            Ok(addr) => addr,
            Err(_) => {
                // Assume it's a wallet address if lookup fails
                profile_id.clone()
            }
        }
    };
    
    debug!("Resolved blocker wallet address: {}", blocker_address);
    
    // Query blocked_profiles joined with profiles for rich information
    let enriched_blocked_profiles: Vec<EnrichedBlockedProfile> = match blocked_profiles::table
        .inner_join(profiles::table.on(
            blocked_profiles::blocked_address.eq(profiles::owner_address)
        ))
        .filter(blocked_profiles::blocker_address.eq(&blocker_address))
        .select((
            // From blocked_profiles
            blocked_profiles::blocked_address,
            blocked_profiles::first_blocked_at,
            blocked_profiles::last_blocked_at,
            blocked_profiles::total_block_count,
            blocked_profiles::block_list_address,
            // From profiles  
            profiles::profile_id.nullable(),
            profiles::username,
            profiles::display_name.nullable(),
            profiles::profile_photo.nullable(),
        ))
        .order_by(blocked_profiles::last_blocked_at.desc())
        .load::<(
            String, // blocked_address
            chrono::NaiveDateTime, // first_blocked_at
            chrono::NaiveDateTime, // last_blocked_at
            i32, // total_block_count
            Option<String>, // block_list_address
            Option<String>, // profile_id
            String, // username
            Option<String>, // display_name
            Option<String>, // profile_photo
        )>(&mut conn)
        .await
    {
        Ok(results) => {
            results
                .into_iter()
                .map(|(blocked_address, first_blocked_at, last_blocked_at, total_block_count, block_list_address, profile_id, username, display_name, profile_photo)| {
                    EnrichedBlockedProfile {
                        profile_id,
                        wallet_address: blocked_address,
                        username,
                        display_name,
                        profile_photo,
                        blocked_at: last_blocked_at,
                        first_blocked_at,
                        total_block_count,
                        block_list_address,
                    }
                })
                .collect()
        },
        Err(e) => {
            error!("Failed to query enriched blocked profiles: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    let total_count = enriched_blocked_profiles.len() as i64;
    
    // Create pagination metadata (for now, no pagination limits, but structure is ready)
    let pagination = PaginationMetadata {
        limit: total_count as i32,
        offset: Some(0),
        cursor: None,
        has_next_page: false,
        has_previous_page: false,
    };
    
    debug!(
        "Found {} enriched blocked profiles for blocker {}",
        total_count, blocker_address
    );
    
    Ok(Json(PaginatedBlockedProfilesResponse {
        blocked_profiles: enriched_blocked_profiles,
        pagination,
        total_count,
    }))
}

/// Check if a profile is blocked by another profile
pub async fn check_profile_blocked(
    Path((blocker_profile_id, blocked_profile_id)): Path<(String, String)>,
    State(pool): State<DbPool>,
) -> Result<Json<BlockCheckResponse>, StatusCode> {
    debug!("Checking if profile {} is blocked by {}", blocked_profile_id, blocker_profile_id);
    
    // Input validation
    if blocker_profile_id.trim().is_empty() || blocked_profile_id.trim().is_empty() {
        debug!("Invalid profile IDs: empty string");
        return Err(StatusCode::BAD_REQUEST);
    }
    
    if blocker_profile_id.len() > 256 || blocked_profile_id.len() > 256 {
        debug!("Invalid profile IDs: too long");
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    // Resolve blocker address (could be profile_id or wallet address)
    let blocker_address = if blocker_profile_id.starts_with("0x") {
        blocker_profile_id.clone()
    } else {
        match profiles::table
            .filter(profiles::profile_id.eq(&blocker_profile_id))
            .select(profiles::owner_address)
            .first::<String>(&mut conn)
            .await
        {
            Ok(addr) => addr,
            Err(_) => blocker_profile_id.clone(), // Fallback to original value
        }
    };
    
    // Resolve blocked address (could be profile_id or wallet address)
    let blocked_address = if blocked_profile_id.starts_with("0x") {
        blocked_profile_id.clone()
    } else {
        match profiles::table
            .filter(profiles::profile_id.eq(&blocked_profile_id))
            .select(profiles::owner_address)
            .first::<String>(&mut conn)
            .await
        {
            Ok(addr) => addr,
            Err(_) => blocked_profile_id.clone(), // Fallback to original value
        }
    };
    
    debug!("Resolved addresses: blocker={}, blocked={}", blocker_address, blocked_address);
    
    // Check production blocking system (blocked_profiles table)
    let is_blocked = match blocked_profiles::table
        .filter(blocked_profiles::blocker_address.eq(&blocker_address))
        .filter(blocked_profiles::blocked_address.eq(&blocked_address))
        .select(blocked_profiles::id)
        .first::<i32>(&mut conn)
        .await
    {
        Ok(_) => {
            debug!("Found blocking relationship in production system");
            true
        },
        Err(diesel::result::Error::NotFound) => {
            debug!("No blocking relationship found");
            false
        },
        Err(e) => {
            error!("Error querying blocked_profiles table: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    Ok(Json(BlockCheckResponse {
        is_blocked,
    }))
}

/// Get platforms that have blocked a user (platform-to-profile blocking)
pub async fn get_blocked_platforms(
    Path(profile_id): Path<String>,
    State(pool): State<DbPool>,
) -> Result<Json<BlockedPlatformsResponse>, StatusCode> {
    debug!("Getting platforms blocked by profile_id: {}", profile_id);
    
    // Input validation
    if profile_id.trim().is_empty() {
        debug!("Invalid profile_id: empty string");
        return Err(StatusCode::BAD_REQUEST);
    }
    
    if profile_id.len() > 256 {
        debug!("Invalid profile_id: too long");
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    // Query profile_events for platform blocking events
    // Platform blocks are stored as events with is_platform_block: true
    // We need to track the current state by processing both BlockAdded and BlockRemoved events
    use crate::schema::profile_events;
    use std::collections::HashMap;
    
    let blocked_platforms: Vec<PlatformBlockInfo> = match profile_events::table
        .filter(profile_events::profile_id.eq(&profile_id))
        .filter(profile_events::event_type.eq_any(vec!["BlockAdded", "BlockRemoved"]))
        .select((profile_events::event_type, profile_events::event_data, profile_events::created_at))
        .order(profile_events::created_at.asc())
        .load::<(String, serde_json::Value, chrono::NaiveDateTime)>(&mut conn)
        .await
    {
        Ok(events) => {
            // Track the current state of each platform and when it was last blocked
            let mut platform_states: HashMap<String, (bool, chrono::NaiveDateTime)> = HashMap::new();
            
            for (event_type, event_data, created_at) in events {
                // Check if this is a platform block event
                if let Some(true) = event_data.get("is_platform_block").and_then(|v| v.as_bool()) {
                    if let Some(platform_id) = event_data.get("platform_id").and_then(|v| v.as_str()) {
                        match event_type.as_str() {
                            "BlockAdded" => {
                                platform_states.insert(platform_id.to_string(), (true, created_at));
                            },
                            "BlockRemoved" => {
                                platform_states.insert(platform_id.to_string(), (false, created_at));
                            },
                            _ => {} // Unknown event type, ignore
                        }
                    }
                }
            }
            
            // Convert to Vec of currently blocked platforms
            platform_states
                .into_iter()
                .filter(|(_, (is_blocked, _))| *is_blocked)
                .map(|(platform_id, (_, blocked_at))| PlatformBlockInfo {
                    platform_id,
                    blocked_at,
                })
                .collect()
        },
        Err(e) => {
            error!("Failed to query blocked platforms: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    let total = blocked_platforms.len() as i64;
    
    Ok(Json(BlockedPlatformsResponse {
        blocked_platforms,
        total,
    }))
}

/// Check if a profile has been blocked by a platform (platform-to-profile blocking)
pub async fn check_platform_blocked(
    Path((profile_id, platform_id)): Path<(String, String)>,
    State(pool): State<DbPool>,
) -> Result<Json<BlockCheckResponse>, StatusCode> {
    debug!("Checking if profile {} has been blocked by platform {}", profile_id, platform_id);
    
    // Input validation
    if profile_id.trim().is_empty() || platform_id.trim().is_empty() {
        debug!("Invalid IDs: empty string");
        return Err(StatusCode::BAD_REQUEST);
    }
    
    if profile_id.len() > 256 || platform_id.len() > 256 {
        debug!("Invalid IDs: too long");
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    // Check profile_events for active platform blocks
    // We need to find if there's a BlockAdded event without a corresponding BlockRemoved event
    use crate::schema::profile_events;
    
    let is_blocked = match profile_events::table
        .filter(profile_events::profile_id.eq(&profile_id))
        .filter(profile_events::event_type.eq_any(vec!["BlockAdded", "BlockRemoved"]))
        .select((profile_events::event_type, profile_events::event_data, profile_events::created_at))
        .order(profile_events::created_at.asc())
        .load::<(String, serde_json::Value, chrono::NaiveDateTime)>(&mut conn)
        .await
    {
        Ok(events) => {
            // Filter events for this specific platform and track the final state
            let mut is_currently_blocked = false;
            
            for (event_type, event_data, _created_at) in events {
                // Check if this event is for the platform we're checking
                if event_data.get("is_platform_block").and_then(|v| v.as_bool()).unwrap_or(false) &&
                   event_data.get("platform_id").and_then(|v| v.as_str()).unwrap_or("") == platform_id {
                    
                    match event_type.as_str() {
                        "BlockAdded" => is_currently_blocked = true,
                        "BlockRemoved" => is_currently_blocked = false,
                        _ => {} // Unknown event type, ignore
                    }
                }
            }
            
            is_currently_blocked
        },
        Err(e) => {
            error!("Failed to check if platform is blocked: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    Ok(Json(BlockCheckResponse {
        is_blocked,
    }))
}