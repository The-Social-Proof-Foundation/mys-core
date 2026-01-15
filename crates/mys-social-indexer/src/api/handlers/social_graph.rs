// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::db::DbPool;
use crate::models::social_graph::{FollowDetail, FollowsQuery};
use crate::schema::{profiles, social_graph_relationships};
use crate::api::handlers::social_proof_token::get_reservation_pool_info_for_profiles;

// ==============================================================================
// CHART DATA STRUCTURES
// ==============================================================================

#[derive(Debug, Deserialize)]
pub struct SocialGraphChartQuery {
    pub bucket: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DailyStatsPoint {
    pub day: String,
    pub event_type: String,
    pub event_count: i64,
}

#[derive(Debug, Serialize)]
pub struct DateRange {
    pub start_date: String,
    pub end_date: String,
    pub days: i32,
    pub bucket: String,
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub total_follows: i64,
    pub total_unfollows: i64,
}

#[derive(Debug, Serialize)]
pub struct SocialGraphChartData {
    pub chart_data: Vec<DailyStatsPoint>,
    pub date_range: DateRange,
    pub summary: Summary,
}

// ==============================================================================
// HELPER FUNCTIONS
// ==============================================================================

/// Convert bucket string to number of days
fn bucket_to_days(bucket: &str) -> Result<i32, String> {
    match bucket.to_lowercase().as_str() {
        "7d" => Ok(7),
        "30d" => Ok(30),
        "90d" => Ok(90),
        "180d" => Ok(180),
        "1y" => Ok(365),
        _ => Err(format!(
            "Invalid bucket '{}'. Must be one of: 7d, 30d, 90d, 180d, 1y",
            bucket
        )),
    }
}

/// Get a list of profiles that a user is following
/// Accepts wallet address (owner_address) as input
pub async fn get_following(
    State(db_pool): State<DbPool>,
    Path(wallet_address): Path<String>,
    Query(query): Query<FollowsQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    debug!(
        "Getting following for wallet_address: {}, limit: {}, offset: {}",
        wallet_address, limit, offset
    );

    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Database connection error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database error: {}", e)
                })),
            );
        }
    };

    // Resolve the input parameter to both profile_id and owner_address
    // The input could be either a profile_id or owner_address (wallet address)
    // Normalize to lowercase for case-insensitive matching
    let normalized_input = wallet_address.to_lowercase();
    let escaped_input = normalized_input.replace("'", "''");
    
    // Try to find profile first
    let profile_info_result = profiles::table
        .filter(
            diesel::dsl::sql::<diesel::sql_types::Bool>(
                &format!("LOWER(profiles.owner_address) = LOWER('{}') OR (profiles.profile_id IS NOT NULL AND LOWER(profiles.profile_id) = LOWER('{}'))", escaped_input, escaped_input)
            )
        )
        .select((
            profiles::profile_id.nullable(),
            profiles::owner_address,
        ))
        .first::<(Option<String>, String)>(&mut conn)
        .await;
    
    // Determine if we have a profile or need to use wallet-only fallback
    let (resolved_profile_id, resolved_owner_address, has_profile) = match profile_info_result {
        Ok(info) => {
            let (profile_id, owner_address) = info;
            (profile_id, owner_address, true)
        }
        Err(diesel::result::Error::NotFound) => {
            debug!("Profile not found with input: {}, checking wallet_social_graph", wallet_address);
            // Check if wallet has any social activity
            use crate::schema::wallet_social_graph;
            let wallet_exists = wallet_social_graph::table
                .filter(wallet_social_graph::wallet_address.eq(&wallet_address))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0) > 0;
            
            // Also check if wallet has any relationships
            let has_relationships = social_graph_relationships::table
                .filter(social_graph_relationships::follower_address.eq(&wallet_address))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0) > 0;
            
            if !wallet_exists && !has_relationships {
                // No profile, no wallet activity, no relationships - return empty list
                debug!("Wallet {} has no profile and no social activity", wallet_address);
                return (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "profiles": [],
                        "pagination": {
                            "total": 0,
                            "limit": limit,
                            "offset": offset,
                            "page": page,
                            "total_pages": 0
                        }
                    })),
                );
            }
            
            // Wallet exists in social graph but no profile - use wallet-only fallback
            (None, wallet_address.clone(), false)
        }
        Err(e) => {
            error!("Failed to check profile: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to check profile: {}", e)
                })),
            );
        }
    };

    // Build base query: following relationships joined with profiles for details
    // Use LEFT JOIN to include wallet-only addresses (addresses without profiles)
    // Match relationships where follower_address equals wallet address (and optionally profile_id)
    // Always use .or() structure for type consistency
    let filter_condition = social_graph_relationships::follower_address.eq(&resolved_owner_address)
        .or(social_graph_relationships::follower_address.eq(
            resolved_profile_id.as_ref().unwrap_or(&resolved_owner_address)
        ));
    
    // Use LEFT JOIN to include wallet-only addresses in the following list
    let mut following_query = social_graph_relationships::table
        .filter(filter_condition)
        .left_join(profiles::table.on(
            diesel::dsl::sql::<diesel::sql_types::Bool>(
                "profiles.owner_address = social_graph_relationships.following_address",
            )
        ))
        .select((
            profiles::id.nullable(),
            profiles::profile_id.nullable(),
            social_graph_relationships::following_address,
            profiles::username.nullable(),
            profiles::display_name.nullable(),
            profiles::profile_photo.nullable(),
        ))
        .into_boxed();

    // Apply search filter if provided
    if let Some(ref term) = query.search {
        if !term.trim().is_empty() {
            let pattern = format!("%{}%", term.trim());
            following_query = following_query.filter(
                profiles::username
                    .nullable()
                    .ilike(pattern.clone())
                    .or(profiles::display_name.nullable().ilike(pattern.clone()))
                    .or(social_graph_relationships::following_address.ilike(pattern.clone())),
            );
        }
    }

    // Apply sort option
    match query.sort.as_ref().map(|s| s.to_lowercase()).as_deref() {
        Some("earliest") => {
            following_query = following_query.order(social_graph_relationships::created_at.asc());
        }
        Some("alphabetical") => {
            // For alphabetical, sort by username if available, otherwise by wallet address
            // Use SQL COALESCE to handle null usernames
            following_query = following_query.order(
                diesel::dsl::sql::<diesel::sql_types::Text>(
                    "COALESCE(profiles.username, social_graph_relationships.following_address) ASC"
                )
            );
        }
        Some("followers_count") => {
            // For followers_count, need to join with profiles or wallet_social_graph
            // Use COALESCE to handle both cases
            following_query = following_query.order(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "COALESCE(profiles.followers_count, 0) DESC"
                )
            );
        }
        _ => {
            // Default: latest
            following_query = following_query.order(social_graph_relationships::created_at.desc());
        }
    }

    // Pagination
    following_query = following_query.limit(limit).offset(offset);

    let following_result = following_query
        .load::<(
            Option<i32>,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
        )>(&mut conn)
        .await;

    // Also get the total count for pagination info (with same search filter)
    // Use same filter condition and LEFT JOIN as main query
    // Always use .or() structure for type consistency
    let filter_condition_count = social_graph_relationships::follower_address.eq(&resolved_owner_address)
        .or(social_graph_relationships::follower_address.eq(
            resolved_profile_id.as_ref().unwrap_or(&resolved_owner_address)
        ));
    
    let mut count_query = social_graph_relationships::table
        .filter(filter_condition_count)
        .left_join(profiles::table.on(
            diesel::dsl::sql::<diesel::sql_types::Bool>(
                "profiles.owner_address = social_graph_relationships.following_address",
            )
        ))
        .into_boxed();

    if let Some(ref term) = query.search {
        if !term.trim().is_empty() {
            let pattern = format!("%{}%", term.trim());
            count_query = count_query.filter(
                profiles::username
                    .nullable()
                    .ilike(pattern.clone())
                    .or(profiles::display_name.nullable().ilike(pattern.clone()))
                    .or(social_graph_relationships::following_address.ilike(pattern.clone())),
            );
        }
    }

    let total_count = match count_query.count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    match following_result {
        Ok(follows) => {
            // Map to FollowDetail struct and calculate relationship status from viewer's perspective
            let mut follows_detail: Vec<FollowDetail> = Vec::new();

            // Get viewer's wallet address if viewer_id is provided
            let viewer_wallet = if let Some(ref viewer_id) = query.viewer_id {
                profiles::table
                    .filter(profiles::profile_id.eq(viewer_id))
                    .select(profiles::owner_address)
                    .first::<String>(&mut conn)
                    .await
                    .unwrap_or_default()
            } else {
                String::new()
            };

            // Collect wallet addresses for reservation pool lookup
            let wallet_addresses: Vec<String> = follows
                .iter()
                .map(|(_, _, owner_address, _, _, _)| owner_address.clone())
                .collect();

            // Get reservation pool info for all profiles
            let reservation_info = get_reservation_pool_info_for_profiles(wallet_addresses, &mut conn)
                .await
                .unwrap_or_default();

            for (id_opt, followed_profile_id, owner_address, username_opt, display_name, profile_photo) in
                follows
            {
                // Handle wallet-only addresses (no profile)
                let id = id_opt.unwrap_or(0);
                let username = username_opt.unwrap_or_else(|| "".to_string());
                
                // Calculate relationship status from viewer's perspective (if viewer_id provided)
                let (is_following, follows_back) = if let Some(ref viewer_id) = query.viewer_id {
                    // Check if viewer is following this address
                    let viewer_follows_this = social_graph_relationships::table
                        .filter(
                            (social_graph_relationships::follower_address
                                .eq(viewer_id)
                                .or(
                                    social_graph_relationships::follower_address.eq(&viewer_wallet)
                                ))
                            .and(
                                social_graph_relationships::following_address
                                    .eq(&followed_profile_id
                                        .clone()
                                        .unwrap_or(owner_address.clone()))
                                    .or(social_graph_relationships::following_address
                                        .eq(&owner_address)),
                            ),
                        )
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    // Check if this address follows the viewer back
                    let this_follows_viewer = social_graph_relationships::table
                        .filter(
                            (social_graph_relationships::follower_address
                                .eq(&followed_profile_id.clone().unwrap_or(owner_address.clone()))
                                .or(
                                    social_graph_relationships::follower_address.eq(&owner_address)
                                ))
                            .and(
                                social_graph_relationships::following_address
                                    .eq(viewer_id)
                                    .or(social_graph_relationships::following_address
                                        .eq(&viewer_wallet)),
                            ),
                        )
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    (viewer_follows_this, this_follows_viewer)
                } else {
                    // No viewer context - default to false
                    (false, false)
                };

                // Get reservation pool info for this address (only if it has a profile)
                let res_info = if id > 0 {
                    reservation_info.get(&owner_address).cloned()
                } else {
                    None
                };

                follows_detail.push(FollowDetail {
                    id,
                    profile_id: followed_profile_id,
                    owner_address,
                    username,
                    display_name,
                    profile_photo,
                    follows_back,
                    is_following,
                    reservation_pool: res_info,
                });
            }

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "profiles": follows_detail,
                    "pagination": {
                        "total": total_count,
                        "limit": limit,
                        "offset": offset,
                        "page": page,
                        "total_pages": total_pages
                    }
                })),
            )
        }
        Err(e) => {
            error!("Failed to fetch following profiles: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch following: {}", e)
                })),
            )
        }
    }
}

/// Get a list of profiles that follow a user
/// Accepts wallet address (owner_address) as input
pub async fn get_followers(
    State(db_pool): State<DbPool>,
    Path(wallet_address): Path<String>,
    Query(query): Query<FollowsQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    debug!(
        "Getting followers for wallet_address: {}, limit: {}, offset: {}",
        wallet_address, limit, offset
    );

    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Database connection error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database error: {}", e)
                })),
            );
        }
    };

    // Resolve the input parameter to both profile_id and owner_address
    // The input could be either a profile_id or owner_address (wallet address)
    // Normalize to lowercase for case-insensitive matching
    let normalized_input = wallet_address.to_lowercase();
    let escaped_input = normalized_input.replace("'", "''");
    
    // Try to find profile first
    let profile_info_result = profiles::table
        .filter(
            diesel::dsl::sql::<diesel::sql_types::Bool>(
                &format!("LOWER(profiles.owner_address) = LOWER('{}') OR (profiles.profile_id IS NOT NULL AND LOWER(profiles.profile_id) = LOWER('{}'))", escaped_input, escaped_input)
            )
        )
        .select((
            profiles::profile_id.nullable(),
            profiles::owner_address,
        ))
        .first::<(Option<String>, String)>(&mut conn)
        .await;
    
    // Determine if we have a profile or need to use wallet-only fallback
    let (resolved_profile_id, resolved_owner_address, has_profile) = match profile_info_result {
        Ok(info) => {
            let (profile_id, owner_address) = info;
            (profile_id, owner_address, true)
        }
        Err(diesel::result::Error::NotFound) => {
            debug!("Profile not found with input: {}, checking wallet_social_graph", wallet_address);
            // Check if wallet has any social activity
            use crate::schema::wallet_social_graph;
            let wallet_exists = wallet_social_graph::table
                .filter(wallet_social_graph::wallet_address.eq(&wallet_address))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0) > 0;
            
            // Also check if wallet has any relationships
            let has_relationships = social_graph_relationships::table
                .filter(social_graph_relationships::following_address.eq(&wallet_address))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0) > 0;
            
            if !wallet_exists && !has_relationships {
                // No profile, no wallet activity, no relationships - return empty list
                debug!("Wallet {} has no profile and no social activity", wallet_address);
                return (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "profiles": [],
                        "pagination": {
                            "total": 0,
                            "limit": limit,
                            "offset": offset,
                            "page": page,
                            "total_pages": 0
                        }
                    })),
                );
            }
            
            // Wallet exists in social graph but no profile - use wallet-only fallback
            (None, wallet_address.clone(), false)
        }
        Err(e) => {
            error!("Failed to check profile: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to check profile: {}", e)
                })),
            );
        }
    };

    // Build base query: followers joined with profiles for details
    // Use LEFT JOIN to include wallet-only addresses (addresses without profiles)
    // Match relationships where following_address equals wallet address (and optionally profile_id)
    // Always use .or() structure for type consistency
    let filter_condition = social_graph_relationships::following_address.eq(&resolved_owner_address)
        .or(social_graph_relationships::following_address.eq(
            resolved_profile_id.as_ref().unwrap_or(&resolved_owner_address)
        ));
    
    // Use LEFT JOIN to include wallet-only addresses in the followers list
    let mut followers_query = social_graph_relationships::table
        .filter(filter_condition)
        .left_join(profiles::table.on(
            diesel::dsl::sql::<diesel::sql_types::Bool>(
                "profiles.owner_address = social_graph_relationships.follower_address",
            )
        ))
        .select((
            profiles::id.nullable(),
            profiles::profile_id.nullable(),
            social_graph_relationships::follower_address,
            profiles::username.nullable(),
            profiles::display_name.nullable(),
            profiles::profile_photo.nullable(),
        ))
        .into_boxed();

    // Apply search filter if provided
    if let Some(ref term) = query.search {
        if !term.trim().is_empty() {
            let pattern = format!("%{}%", term.trim());
            followers_query = followers_query.filter(
                profiles::username
                    .nullable()
                    .ilike(pattern.clone())
                    .or(profiles::display_name.nullable().ilike(pattern.clone()))
                    .or(social_graph_relationships::follower_address.ilike(pattern.clone())),
            );
        }
    }

    // Apply sort option
    match query.sort.as_ref().map(|s| s.to_lowercase()).as_deref() {
        Some("earliest") => {
            followers_query = followers_query.order(social_graph_relationships::created_at.asc());
        }
        Some("alphabetical") => {
            // For alphabetical, sort by username if available, otherwise by wallet address
            // Use SQL COALESCE to handle null usernames
            followers_query = followers_query.order(
                diesel::dsl::sql::<diesel::sql_types::Text>(
                    "COALESCE(profiles.username, social_graph_relationships.follower_address) ASC"
                )
            );
        }
        Some("followers_count") => {
            // For followers_count, need to join with profiles or wallet_social_graph
            // Use COALESCE to handle both cases
            followers_query = followers_query.order(
                diesel::dsl::sql::<diesel::sql_types::Integer>(
                    "COALESCE(profiles.followers_count, 0) DESC"
                )
            );
        }
        _ => {
            // Default: latest
            followers_query = followers_query.order(social_graph_relationships::created_at.desc());
        }
    }

    // Pagination
    followers_query = followers_query.limit(limit).offset(offset);

    let followers_result = followers_query
        .load::<(
            Option<i32>,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
        )>(&mut conn)
        .await;

    // Also get the total count for pagination info (with same search filter)
    // Use same filter condition and LEFT JOIN as main query
    // Always use .or() structure for type consistency
    let filter_condition_count = social_graph_relationships::following_address.eq(&resolved_owner_address)
        .or(social_graph_relationships::following_address.eq(
            resolved_profile_id.as_ref().unwrap_or(&resolved_owner_address)
        ));
    
    let mut count_query = social_graph_relationships::table
        .filter(filter_condition_count)
        .left_join(profiles::table.on(
            diesel::dsl::sql::<diesel::sql_types::Bool>(
                "profiles.owner_address = social_graph_relationships.follower_address",
            )
        ))
        .into_boxed();

    if let Some(ref term) = query.search {
        if !term.trim().is_empty() {
            let pattern = format!("%{}%", term.trim());
            count_query = count_query.filter(
                profiles::username
                    .nullable()
                    .ilike(pattern.clone())
                    .or(profiles::display_name.nullable().ilike(pattern.clone()))
                    .or(social_graph_relationships::follower_address.ilike(pattern.clone())),
            );
        }
    }

    let total_count = match count_query.count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    match followers_result {
        Ok(follows) => {
            // Map to FollowDetail struct and calculate relationship status from viewer's perspective
            let mut follows_detail: Vec<FollowDetail> = Vec::new();

            // Get viewer's wallet address if viewer_id is provided
            let viewer_wallet = if let Some(ref viewer_id) = query.viewer_id {
                profiles::table
                    .filter(profiles::profile_id.eq(viewer_id))
                    .select(profiles::owner_address)
                    .first::<String>(&mut conn)
                    .await
                    .unwrap_or_default()
            } else {
                String::new()
            };

            // Collect wallet addresses for reservation pool lookup
            let wallet_addresses: Vec<String> = follows
                .iter()
                .map(|(_, _, owner_address, _, _, _)| owner_address.clone())
                .collect();

            // Get reservation pool info for all profiles
            let reservation_info = get_reservation_pool_info_for_profiles(wallet_addresses, &mut conn)
                .await
                .unwrap_or_default();

            for (id_opt, follower_profile_id, owner_address, username_opt, display_name, profile_photo) in
                follows
            {
                // Handle wallet-only addresses (no profile)
                let id = id_opt.unwrap_or(0);
                let username = username_opt.unwrap_or_else(|| "".to_string());
                
                // Calculate relationship status from viewer's perspective (if viewer_id provided)
                let (is_following, follows_back) = if let Some(ref viewer_id) = query.viewer_id {
                    // Check if viewer is following this address
                    let viewer_follows_this = social_graph_relationships::table
                        .filter(
                            (social_graph_relationships::follower_address
                                .eq(viewer_id)
                                .or(
                                    social_graph_relationships::follower_address.eq(&viewer_wallet)
                                ))
                            .and(
                                social_graph_relationships::following_address
                                    .eq(&follower_profile_id
                                        .clone()
                                        .unwrap_or(owner_address.clone()))
                                    .or(social_graph_relationships::following_address
                                        .eq(&owner_address)),
                            ),
                        )
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    // Check if this address follows the viewer back
                    let this_follows_viewer = social_graph_relationships::table
                        .filter(
                            (social_graph_relationships::follower_address
                                .eq(&follower_profile_id.clone().unwrap_or(owner_address.clone()))
                                .or(
                                    social_graph_relationships::follower_address.eq(&owner_address)
                                ))
                            .and(
                                social_graph_relationships::following_address
                                    .eq(viewer_id)
                                    .or(social_graph_relationships::following_address
                                        .eq(&viewer_wallet)),
                            ),
                        )
                        .count()
                        .get_result::<i64>(&mut conn)
                        .await
                        .unwrap_or(0)
                        > 0;

                    (viewer_follows_this, this_follows_viewer)
                } else {
                    // No viewer context - default to false
                    (false, false)
                };

                // Get reservation pool info for this address (only if it has a profile)
                let res_info = if id > 0 {
                    reservation_info.get(&owner_address).cloned()
                } else {
                    None
                };

                follows_detail.push(FollowDetail {
                    id,
                    profile_id: follower_profile_id,
                    owner_address,
                    username,
                    display_name,
                    profile_photo,
                    follows_back,
                    is_following,
                    reservation_pool: res_info,
                });
            }

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "profiles": follows_detail,
                    "pagination": {
                        "total": total_count,
                        "limit": limit,
                        "offset": offset,
                        "page": page,
                        "total_pages": total_pages
                    }
                })),
            )
        }
        Err(e) => {
            error!("Failed to fetch follower profiles: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch followers: {}", e)
                })),
            )
        }
    }
}

/// Check if a user is following another user
/// Accepts wallet addresses (owner_address) as input
pub async fn check_following(
    State(db_pool): State<DbPool>,
    Path((follower_wallet, following_wallet)): Path<(String, String)>,
) -> impl IntoResponse {
    debug!(
        "Checking if wallet {} follows wallet {}",
        follower_wallet, following_wallet
    );

    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Database connection error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database error: {}", e)
                })),
            );
        }
    };

    // Note: Profile existence checks removed - this endpoint works with wallet addresses only
    // Check if a relationship exists using wallet addresses
    // The relationship table may store wallet addresses or profile_ids, so check both
    let relationship_exists = social_graph_relationships::table
        .filter(
            social_graph_relationships::follower_address.eq(&follower_wallet)
                .and(social_graph_relationships::following_address.eq(&following_wallet))
        )
        .count()
        .get_result::<i64>(&mut conn)
        .await;

    match relationship_exists {
        Ok(count) => {
            let is_following = count > 0;

            // Check if the following profile is also following back
            let reverse_relationship_exists = social_graph_relationships::table
                .filter(
                    social_graph_relationships::follower_address.eq(&following_wallet)
                        .and(social_graph_relationships::following_address.eq(&follower_wallet))
                )
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0);

            let following_back = reverse_relationship_exists > 0;

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "is_following": is_following,
                    "following_back": following_back
                })),
            )
        }
        Err(e) => {
            error!("Failed to check following status: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to check follow status: {}", e),
                    "is_following": false
                })),
            )
        }
    }
}

/// Get stats for a profile (followers count, following count)
/// Accepts wallet address (owner_address) as input
pub async fn get_follow_stats(
    State(db_pool): State<DbPool>,
    Path(wallet_address): Path<String>,
) -> impl IntoResponse {
    debug!("Getting follow stats for wallet_address: {}", wallet_address);

    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Database connection error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database error: {}", e)
                })),
            );
        }
    };

    // Get profile stats from the profiles table using wallet address
    let profile_result = profiles::table
        .filter(profiles::owner_address.eq(&wallet_address))
        .select((
            profiles::followers_count,
            profiles::following_count,
            profiles::blocked_count,
            profiles::username,
            profiles::display_name.nullable(),
            profiles::profile_photo.nullable(),
            profiles::profile_id.nullable(),
        ))
        .first::<(i32, i32, i32, String, Option<String>, Option<String>, Option<String>)>(&mut conn)
        .await;

    match profile_result {
        Ok((followers, following, blocked, username, display_name, profile_photo, profile_id)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "profile_id": profile_id,
                "wallet_address": wallet_address,
                "username": username,
                "display_name": display_name,
                "profile_photo": profile_photo,
                "followers_count": followers,
                "following_count": following,
                "blocked_count": blocked
            })),
        ),
        Err(diesel::result::Error::NotFound) => {
            debug!("Profile not found with wallet_address: {}, checking wallet_social_graph", wallet_address);
            // Profile doesn't exist, check wallet_social_graph
            use crate::schema::wallet_social_graph;
            let wallet_result = wallet_social_graph::table
                .filter(wallet_social_graph::wallet_address.eq(&wallet_address))
                .select((
                    wallet_social_graph::followers_count,
                    wallet_social_graph::following_count,
                    wallet_social_graph::blocked_count,
                ))
                .first::<(i32, i32, i32)>(&mut conn)
                .await;
            
            match wallet_result {
                Ok((followers, following, blocked)) => {
                    let mut response = serde_json::Map::new();
                    response.insert("profile_id".to_string(), serde_json::Value::Null);
                    response.insert("wallet_address".to_string(), serde_json::Value::String(wallet_address.clone()));
                    response.insert("username".to_string(), serde_json::Value::Null);
                    response.insert("display_name".to_string(), serde_json::Value::Null);
                    response.insert("profile_photo".to_string(), serde_json::Value::Null);
                    response.insert("followers_count".to_string(), serde_json::Value::Number(followers.into()));
                    response.insert("following_count".to_string(), serde_json::Value::Number(following.into()));
                    response.insert("blocked_count".to_string(), serde_json::Value::Number(blocked.into()));
                    (
                        StatusCode::OK,
                        Json(serde_json::Value::Object(response)),
                    )
                }
                Err(diesel::result::Error::NotFound) => {
                    // No counts found, return zeros
                    let mut response = serde_json::Map::new();
                    response.insert("profile_id".to_string(), serde_json::Value::Null);
                    response.insert("wallet_address".to_string(), serde_json::Value::String(wallet_address.clone()));
                    response.insert("username".to_string(), serde_json::Value::Null);
                    response.insert("display_name".to_string(), serde_json::Value::Null);
                    response.insert("profile_photo".to_string(), serde_json::Value::Null);
                    response.insert("followers_count".to_string(), serde_json::Value::Number(0.into()));
                    response.insert("following_count".to_string(), serde_json::Value::Number(0.into()));
                    response.insert("blocked_count".to_string(), serde_json::Value::Number(0.into()));
                    (
                        StatusCode::OK,
                        Json(serde_json::Value::Object(response)),
                    )
                }
                Err(e) => {
                    error!("Failed to fetch wallet social graph stats: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("Failed to fetch wallet social graph stats: {}", e)
                        })),
                    )
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch profile stats: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch profile stats: {}", e)
                })),
            )
        }
    }
}

/// Get social graph daily statistics chart data
pub async fn get_social_graph_chart_data(
    Query(params): Query<SocialGraphChartQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    // Parse and validate bucket parameter
    let bucket_str = params.bucket.as_deref().unwrap_or("30d").to_lowercase();
    let days = match bucket_to_days(&bucket_str) {
        Ok(d) => d,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": e
                })),
            );
        }
    };

    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                })),
            );
        }
    };

    // Calculate date range
    let end_date = Utc::now().date_naive();
    let start_date = end_date - chrono::Duration::days(days as i64);

    let query = "
        SELECT 
            day::DATE as day,
            event_type,
            event_count::BIGINT as event_count
        FROM social_graph_daily_stats
        WHERE day >= $1::DATE
        ORDER BY day ASC, event_type ASC
    ";

    #[derive(QueryableByName, Debug)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct ChartQueryResult {
        #[diesel(sql_type = Date)]
        day: NaiveDate,
        #[diesel(sql_type = Text)]
        event_type: String,
        #[diesel(sql_type = Int8)]
        event_count: i64,
    }

    let results: Vec<ChartQueryResult> = match diesel::sql_query(query)
        .bind::<Date, _>(start_date)
        .load::<ChartQueryResult>(&mut conn)
        .await
    {
        Ok(data) => data,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to query chart data: {}", e)
                })),
            );
        }
    };

    // Transform results into response format
    let chart_data: Vec<DailyStatsPoint> = results
        .into_iter()
        .map(|r| DailyStatsPoint {
            day: r.day.format("%Y-%m-%d").to_string(),
            event_type: r.event_type,
            event_count: r.event_count,
        })
        .collect();

    // Calculate summary statistics
    let total_follows: i64 = chart_data
        .iter()
        .filter(|p| p.event_type == "follow")
        .map(|p| p.event_count)
        .sum();

    let total_unfollows: i64 = chart_data
        .iter()
        .filter(|p| p.event_type == "unfollow")
        .map(|p| p.event_count)
        .sum();

    let response = SocialGraphChartData {
        chart_data,
        date_range: DateRange {
            start_date: start_date.format("%Y-%m-%d").to_string(),
            end_date: end_date.format("%Y-%m-%d").to_string(),
            days,
            bucket: bucket_str,
        },
        summary: Summary {
            total_follows,
            total_unfollows,
        },
    };

    (StatusCode::OK, Json(serde_json::json!(response)))
}
