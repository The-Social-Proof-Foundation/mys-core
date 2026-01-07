// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::db::DbPool;
use crate::models::blocking::{
    BlockedListQuery, BlockedProfile, EnrichedBlockedProfile, PaginatedBlockedProfilesResponse,
    PaginationMetadata,
};
use crate::schema::{blocked_profiles, profiles};
use crate::api::handlers::social_proof_token::get_reservation_pool_info_for_profiles;

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

/// Query parameters for listing blocked platforms
#[derive(Debug, Deserialize)]
pub struct BlockedPlatformsQuery {
    /// Optional sort: latest | earliest | alphabetical
    pub sort: Option<String>,
    /// Optional search by platform_id
    pub search: Option<String>,
}

/// Get profiles blocked by a user with rich profile information and pagination
/// Accepts wallet address (owner_address) as input
pub async fn get_blocked_profiles(
    Path(wallet_address): Path<String>,
    Query(query): Query<BlockedListQuery>,
    State(pool): State<DbPool>,
) -> Result<Json<PaginatedBlockedProfilesResponse>, StatusCode> {
    debug!(
        "Getting enriched profiles blocked by wallet_address: {}",
        wallet_address
    );

    // Input validation
    if wallet_address.trim().is_empty() {
        debug!("Invalid wallet_address: empty string");
        return Err(StatusCode::BAD_REQUEST);
    }

    // Basic length validation to prevent potential attacks
    if wallet_address.len() > 256 {
        debug!("Invalid wallet_address: too long");
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Verify profile exists
    let profile_exists = match profiles::table
        .filter(profiles::owner_address.eq(&wallet_address))
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count > 0,
        Err(e) => {
            error!("Failed to check profile: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !profile_exists {
        debug!("Profile not found with wallet_address: {}", wallet_address);
        return Err(StatusCode::NOT_FOUND);
    }

    debug!("Using blocker wallet address: {}", wallet_address);

    // Check if we need to join with profiles table for followers_count sorting
    let sort_option = query.sort.as_ref().map(|s| s.to_lowercase());
    let needs_profile_join = sort_option.as_deref() == Some("followers_count");

    // Execute query - handle joined and non-joined cases separately due to Diesel type system
    let blocked_profiles: Vec<BlockedProfile> = if needs_profile_join {
        // Build query with join for followers_count sorting
        use diesel::dsl::sql;
        
        // Handle search and no-search cases separately to avoid Diesel type system issues
        let query_builder = if let Some(ref term) = query.search {
            if !term.trim().is_empty() {
                let pattern = format!("%{}%", term.trim());
                blocked_profiles::table
                    .filter(blocked_profiles::blocker_address.eq(&wallet_address))
                    .filter(
                        blocked_profiles::blocked_username
                            .ilike(pattern.clone())
                            .or(blocked_profiles::blocked_display_name.ilike(pattern.clone()))
                            .or(blocked_profiles::blocked_address.ilike(pattern.clone()))
                    )
                    .inner_join(
                        profiles::table.on(blocked_profiles::blocked_address.eq(profiles::owner_address))
                    )
                    .order(sql::<diesel::sql_types::Integer>("profiles.followers_count DESC"))
                    .select(BlockedProfile::as_select())
                    .into_boxed()
            } else {
                blocked_profiles::table
                    .filter(blocked_profiles::blocker_address.eq(&wallet_address))
                    .inner_join(
                        profiles::table.on(blocked_profiles::blocked_address.eq(profiles::owner_address))
                    )
                    .order(sql::<diesel::sql_types::Integer>("profiles.followers_count DESC"))
                    .select(BlockedProfile::as_select())
                    .into_boxed()
            }
        } else {
            blocked_profiles::table
                .filter(blocked_profiles::blocker_address.eq(&wallet_address))
                .inner_join(
                    profiles::table.on(blocked_profiles::blocked_address.eq(profiles::owner_address))
                )
                .order(sql::<diesel::sql_types::Integer>("profiles.followers_count DESC"))
                .select(BlockedProfile::as_select())
                .into_boxed()
        };

        match query_builder.load::<BlockedProfile>(&mut conn).await {
            Ok(results) => results,
            Err(e) => {
                error!("Failed to query blocked profiles: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    } else {
        // Build query without join for other sort options
        let mut query_builder = blocked_profiles::table
            .filter(blocked_profiles::blocker_address.eq(&wallet_address))
            .into_boxed();

        // Apply search if provided
        if let Some(ref term) = query.search {
            if !term.trim().is_empty() {
                let pattern = format!("%{}%", term.trim());
                query_builder = query_builder.filter(
                    blocked_profiles::blocked_username
                        .ilike(pattern.clone())
                        .or(blocked_profiles::blocked_display_name.ilike(pattern.clone()))
                        .or(blocked_profiles::blocked_address.ilike(pattern.clone())),
                );
            }
        }

        // Apply sort option
        match sort_option.as_deref() {
            Some("earliest") => {
                query_builder = query_builder.order(blocked_profiles::last_blocked_at.asc());
            }
            Some("alphabetical") => {
                query_builder = query_builder.order(blocked_profiles::blocked_username.asc());
            }
            _ => {
                // Default to latest
                query_builder = query_builder.order(blocked_profiles::last_blocked_at.desc());
            }
        }

        match query_builder.load::<BlockedProfile>(&mut conn).await {
            Ok(results) => results,
            Err(e) => {
                error!("Failed to query blocked profiles: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    };

    // Convert directly from BlockedProfile to EnrichedBlockedProfile (uses From trait)
    let mut enriched_blocked_profiles: Vec<EnrichedBlockedProfile> = blocked_profiles
        .into_iter()
        .map(|blocked_profile| blocked_profile.into())
        .collect();

    // Get reservation pool info for all blocked profiles
    let wallet_addresses: Vec<String> = enriched_blocked_profiles
        .iter()
        .map(|p| p.wallet_address.clone())
        .collect();

    let reservation_info = get_reservation_pool_info_for_profiles(wallet_addresses, &mut conn)
        .await
        .unwrap_or_default();

    // Add reservation pool info to each profile
    for profile in &mut enriched_blocked_profiles {
        profile.reservation_pool = reservation_info.get(&profile.wallet_address).cloned();
    }

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
        total_count, wallet_address
    );

    Ok(Json(PaginatedBlockedProfilesResponse {
        blocked_profiles: enriched_blocked_profiles,
        pagination,
        total_count,
    }))
}

/// Check if a profile is blocked by another profile
/// Accepts wallet addresses (owner_address) as input
pub async fn check_profile_blocked(
    Path((blocker_wallet, blocked_wallet)): Path<(String, String)>,
    State(pool): State<DbPool>,
) -> Result<Json<BlockCheckResponse>, StatusCode> {
    debug!(
        "Checking if wallet {} is blocked by {}",
        blocked_wallet, blocker_wallet
    );

    // Input validation
    if blocker_wallet.trim().is_empty() || blocked_wallet.trim().is_empty() {
        debug!("Invalid wallet addresses: empty string");
        return Err(StatusCode::BAD_REQUEST);
    }

    if blocker_wallet.len() > 256 || blocked_wallet.len() > 256 {
        debug!("Invalid wallet addresses: too long");
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Verify both profiles exist
    let blocker_exists = profiles::table
        .filter(profiles::owner_address.eq(&blocker_wallet))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .unwrap_or(0)
        > 0;

    if !blocker_exists {
        debug!("Blocker profile not found: {}", blocker_wallet);
        return Ok(Json(BlockCheckResponse { is_blocked: false }));
    }

    let blocked_exists = profiles::table
        .filter(profiles::owner_address.eq(&blocked_wallet))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .unwrap_or(0)
        > 0;

    if !blocked_exists {
        debug!("Blocked profile not found: {}", blocked_wallet);
        return Ok(Json(BlockCheckResponse { is_blocked: false }));
    }

    debug!(
        "Checking blocking relationship: blocker={}, blocked={}",
        blocker_wallet, blocked_wallet
    );

    // Check production blocking system (blocked_profiles table)
    let is_blocked = match blocked_profiles::table
        .filter(blocked_profiles::blocker_address.eq(&blocker_wallet))
        .filter(blocked_profiles::blocked_address.eq(&blocked_wallet))
        .select(blocked_profiles::id)
        .first::<i32>(&mut conn)
        .await
    {
        Ok(_) => {
            debug!("Found blocking relationship in production system");
            true
        }
        Err(diesel::result::Error::NotFound) => {
            debug!("No blocking relationship found");
            false
        }
        Err(e) => {
            error!("Error querying blocked_profiles table: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(BlockCheckResponse { is_blocked }))
}

/// Get platforms that have blocked a user (platform-to-profile blocking)
/// Accepts wallet address (owner_address) as input
pub async fn get_blocked_platforms(
    Path(wallet_address): Path<String>,
    Query(query): Query<BlockedPlatformsQuery>,
    State(pool): State<DbPool>,
) -> Result<Json<BlockedPlatformsResponse>, StatusCode> {
    debug!("Getting platforms blocked by wallet_address: {}", wallet_address);

    // Input validation
    if wallet_address.trim().is_empty() {
        debug!("Invalid wallet_address: empty string");
        return Err(StatusCode::BAD_REQUEST);
    }

    if wallet_address.len() > 256 {
        debug!("Invalid wallet_address: too long");
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Resolve wallet address to profile_id for profile_events query
    let profile_id = match profiles::table
        .filter(profiles::owner_address.eq(&wallet_address))
        .select(profiles::profile_id.nullable())
        .first::<Option<String>>(&mut conn)
        .await
    {
        Ok(Some(pid)) => pid,
        Ok(None) => {
            debug!("Profile found but no profile_id for wallet_address: {}", wallet_address);
            return Err(StatusCode::NOT_FOUND);
        }
        Err(_) => {
            debug!("Profile not found with wallet_address: {}", wallet_address);
            return Err(StatusCode::NOT_FOUND);
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
        .select((
            profile_events::event_type,
            profile_events::event_data,
            profile_events::created_at,
        ))
        .order(profile_events::created_at.asc())
        .load::<(String, serde_json::Value, chrono::NaiveDateTime)>(&mut conn)
        .await
    {
        Ok(events) => {
            // Track the current state of each platform and when it was last blocked
            let mut platform_states: HashMap<String, (bool, chrono::NaiveDateTime)> =
                HashMap::new();

            for (event_type, event_data, created_at) in events {
                // Check if this is a platform block event
                if let Some(true) = event_data
                    .get("is_platform_block")
                    .and_then(|v| v.as_bool())
                {
                    if let Some(platform_id) =
                        event_data.get("platform_id").and_then(|v| v.as_str())
                    {
                        match event_type.as_str() {
                            "BlockAdded" => {
                                platform_states.insert(platform_id.to_string(), (true, created_at));
                            }
                            "BlockRemoved" => {
                                platform_states
                                    .insert(platform_id.to_string(), (false, created_at));
                            }
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
        }
        Err(e) => {
            error!("Failed to query blocked platforms: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Apply search filter if provided
    let mut filtered_platforms = if let Some(ref term) = query.search {
        if !term.trim().is_empty() {
            let search_term = term.trim().to_lowercase();
            blocked_platforms
                .into_iter()
                .filter(|platform| platform.platform_id.to_lowercase().contains(&search_term))
                .collect()
        } else {
            blocked_platforms
        }
    } else {
        blocked_platforms
    };

    // Apply sort option
    let sort_option = query.sort.as_ref().map(|s| s.to_lowercase());
    match sort_option.as_deref() {
        Some("earliest") => {
            filtered_platforms.sort_by(|a, b| a.blocked_at.cmp(&b.blocked_at));
        }
        Some("alphabetical") => {
            filtered_platforms.sort_by(|a, b| a.platform_id.cmp(&b.platform_id));
        }
        _ => {
            // Default: latest (newest first)
            filtered_platforms.sort_by(|a, b| b.blocked_at.cmp(&a.blocked_at));
        }
    }

    let total = filtered_platforms.len() as i64;

    Ok(Json(BlockedPlatformsResponse {
        blocked_platforms: filtered_platforms,
        total,
    }))
}

/// Check if a profile has been blocked by a platform (platform-to-profile blocking)
/// Accepts wallet address (owner_address) as input
pub async fn check_platform_blocked(
    Path((wallet_address, platform_id)): Path<(String, String)>,
    State(pool): State<DbPool>,
) -> Result<Json<BlockCheckResponse>, StatusCode> {
    debug!(
        "Checking if profile {} has been blocked by platform {}",
        wallet_address, platform_id
    );

    // Input validation
    if wallet_address.trim().is_empty() || platform_id.trim().is_empty() {
        debug!("Invalid IDs: empty string");
        return Err(StatusCode::BAD_REQUEST);
    }

    if wallet_address.len() > 256 || platform_id.len() > 256 {
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

    // Resolve wallet address to profile_id for profile_events query
    let profile_id = match profiles::table
        .filter(profiles::owner_address.eq(&wallet_address))
        .select(profiles::profile_id.nullable())
        .first::<Option<String>>(&mut conn)
        .await
    {
        Ok(Some(pid)) => pid,
        Ok(None) => {
            debug!("Profile found but no profile_id for wallet_address: {}", wallet_address);
            return Ok(Json(BlockCheckResponse { is_blocked: false }));
        }
        Err(_) => {
            debug!("Profile not found with wallet_address: {}", wallet_address);
            return Ok(Json(BlockCheckResponse { is_blocked: false }));
        }
    };

    // Check profile_events for active platform blocks
    // We need to find if there's a BlockAdded event without a corresponding BlockRemoved event
    use crate::schema::profile_events;

    let is_blocked = match profile_events::table
        .filter(profile_events::profile_id.eq(&profile_id))
        .filter(profile_events::event_type.eq_any(vec!["BlockAdded", "BlockRemoved"]))
        .select((
            profile_events::event_type,
            profile_events::event_data,
            profile_events::created_at,
        ))
        .order(profile_events::created_at.asc())
        .load::<(String, serde_json::Value, chrono::NaiveDateTime)>(&mut conn)
        .await
    {
        Ok(events) => {
            // Filter events for this specific platform and track the final state
            let mut is_currently_blocked = false;

            for (event_type, event_data, _created_at) in events {
                // Check if this event is for the platform we're checking
                if event_data
                    .get("is_platform_block")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
                    && event_data
                        .get("platform_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        == platform_id
                {
                    match event_type.as_str() {
                        "BlockAdded" => is_currently_blocked = true,
                        "BlockRemoved" => is_currently_blocked = false,
                        _ => {} // Unknown event type, ignore
                    }
                }
            }

            is_currently_blocked
        }
        Err(e) => {
            error!("Failed to check if platform is blocked: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(BlockCheckResponse { is_blocked }))
}
