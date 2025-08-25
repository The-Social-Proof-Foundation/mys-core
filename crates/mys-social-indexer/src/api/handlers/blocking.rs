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
use crate::schema::profiles_blocked;

/// Response type for blocked profiles list
#[derive(Debug, Serialize)]
pub struct BlockedProfilesResponse {
    pub blocked_profiles: Vec<ProfileBlockInfo>,
    pub total: i64,
}

/// Profile block information
#[derive(Debug, Serialize)]
pub struct ProfileBlockInfo {
    pub profile_id: String,
    pub blocked_at: chrono::NaiveDateTime,
}

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

/// Get profiles blocked by a user
pub async fn get_blocked_profiles(
    Path(profile_id): Path<String>,
    State(pool): State<DbPool>,
) -> Result<Json<BlockedProfilesResponse>, StatusCode> {
    debug!("Getting profiles blocked by profile_id: {}", profile_id);
    
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
    
    // Query the profiles_blocked table for blocks where this profile is the blocker
    // Note: profile_id could be either a wallet address or profile ID, so we'll try both
    let blocked_profiles: Vec<ProfileBlockInfo> = match profiles_blocked::table
        .filter(profiles_blocked::blocker_wallet_address.eq(&profile_id))
        .select((profiles_blocked::blocked_address, profiles_blocked::created_at))
        .load::<(String, chrono::NaiveDateTime)>(&mut conn)
        .await
    {
        Ok(blocks) => blocks
            .into_iter()
            .map(|(blocked_address, created_at)| ProfileBlockInfo {
                profile_id: blocked_address,
                blocked_at: created_at,
            })
            .collect::<Vec<ProfileBlockInfo>>(),
        Err(e) => {
            error!("Failed to query blocked profiles: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    let total = blocked_profiles.len() as i64;
    
    Ok(Json(BlockedProfilesResponse {
        blocked_profiles,
        total,
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
    
    // Check if there's a block record for this combination
    let is_blocked = match profiles_blocked::table
        .filter(profiles_blocked::blocker_wallet_address.eq(&blocker_profile_id))
        .filter(profiles_blocked::blocked_address.eq(&blocked_profile_id))
        .select(profiles_blocked::id)
        .first::<i32>(&mut conn)
        .await
    {
        Ok(_) => true,  // Found a record, so it's blocked
        Err(diesel::result::Error::NotFound) => false,  // No record found, not blocked
        Err(e) => {
            error!("Failed to check if profile is blocked: {}", e);
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