// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::{debug, error};

use crate::db::DbPool;
use crate::models::social_graph::{FollowDetail, FollowsQuery};
use crate::schema::{profiles, social_graph_relationships};

/// Get a list of profiles that a user is following
pub async fn get_following(
    State(db_pool): State<DbPool>,
    Path(profile_id): Path<String>,
    Query(query): Query<FollowsQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    debug!(
        "Getting following for profile_id: {}, limit: {}, offset: {}",
        profile_id, limit, offset
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

    // First verify the profile exists using profile_id
    let profile_exists = match profiles::table
        .filter(profiles::profile_id.eq(&profile_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count > 0,
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

    if !profile_exists {
        debug!("Profile not found with profile_id: {}", profile_id);
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Profile not found"
            })),
        );
    }

    // Get profile's wallet address to handle legacy data
    let wallet_address = profiles::table
        .filter(profiles::profile_id.eq(&profile_id))
        .select(profiles::owner_address)
        .first::<String>(&mut conn)
        .await
        .unwrap_or_default();

    // Build base query: following relationships joined with profiles for details
    let mut following_query = social_graph_relationships::table
        .filter(
            social_graph_relationships::follower_address.eq(&profile_id)
                .or(social_graph_relationships::follower_address.eq(&wallet_address))
        )
        .inner_join(profiles::table.on(
            diesel::dsl::sql::<diesel::sql_types::Bool>(
                "profiles.profile_id = social_graph_relationships.following_address OR profiles.owner_address = social_graph_relationships.following_address",
            )
        ))
        .select((
            profiles::id,
            profiles::profile_id,
            profiles::owner_address,
            profiles::username,
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
                    .ilike(pattern.clone())
                    .or(profiles::display_name.ilike(pattern.clone()))
                    .or(profiles::owner_address.ilike(pattern.clone())),
            );
        }
    }

    // Apply sort option
    match query.sort.as_ref().map(|s| s.to_lowercase()).as_deref() {
        Some("earliest") => {
            following_query = following_query.order(social_graph_relationships::created_at.asc());
        }
        Some("alphabetical") => {
            following_query = following_query.order(profiles::username.asc());
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
            i32,
            Option<String>,
            String,
            String,
            Option<String>,
            Option<String>,
        )>(&mut conn)
        .await;

    // Also get the total count for pagination info (with same search filter)
    let mut count_query = social_graph_relationships::table
        .filter(
            social_graph_relationships::follower_address.eq(&profile_id)
                .or(social_graph_relationships::follower_address.eq(&wallet_address))
        )
        .inner_join(profiles::table.on(
            diesel::dsl::sql::<diesel::sql_types::Bool>(
                "profiles.profile_id = social_graph_relationships.following_address OR profiles.owner_address = social_graph_relationships.following_address",
            )
        ))
        .into_boxed();

    if let Some(ref term) = query.search {
        if !term.trim().is_empty() {
            let pattern = format!("%{}%", term.trim());
            count_query = count_query.filter(
                profiles::username
                    .ilike(pattern.clone())
                    .or(profiles::display_name.ilike(pattern.clone()))
                    .or(profiles::owner_address.ilike(pattern.clone())),
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

            for (id, followed_profile_id, owner_address, username, display_name, profile_photo) in
                follows
            {
                // Calculate relationship status from viewer's perspective (if viewer_id provided)
                let (is_following, follows_back) = if let Some(ref viewer_id) = query.viewer_id {
                    // Check if viewer is following this profile
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

                    // Check if this profile follows the viewer back
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

                follows_detail.push(FollowDetail {
                    id,
                    profile_id: followed_profile_id,
                    owner_address,
                    username,
                    display_name,
                    profile_photo,
                    follows_back,
                    is_following,
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
pub async fn get_followers(
    State(db_pool): State<DbPool>,
    Path(profile_id): Path<String>,
    Query(query): Query<FollowsQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    debug!(
        "Getting followers for profile_id: {}, limit: {}, offset: {}",
        profile_id, limit, offset
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

    // First verify the profile exists using profile_id
    let profile_exists = match profiles::table
        .filter(profiles::profile_id.eq(&profile_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count > 0,
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

    if !profile_exists {
        debug!("Profile not found with profile_id: {}", profile_id);
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Profile not found"
            })),
        );
    }

    // Get profile's wallet address to handle legacy data
    let wallet_address = profiles::table
        .filter(profiles::profile_id.eq(&profile_id))
        .select(profiles::owner_address)
        .first::<String>(&mut conn)
        .await
        .unwrap_or_default();

    // Build base query: followers joined with profiles for details
    let mut followers_query = social_graph_relationships::table
        .filter(
            social_graph_relationships::following_address.eq(&profile_id)
                .or(social_graph_relationships::following_address.eq(&wallet_address))
        )
        .inner_join(profiles::table.on(
            diesel::dsl::sql::<diesel::sql_types::Bool>(
                "profiles.profile_id = social_graph_relationships.follower_address OR profiles.owner_address = social_graph_relationships.follower_address",
            )
        ))
        .select((
            profiles::id,
            profiles::profile_id,
            profiles::owner_address,
            profiles::username,
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
                    .ilike(pattern.clone())
                    .or(profiles::display_name.ilike(pattern.clone()))
                    .or(profiles::owner_address.ilike(pattern.clone())),
            );
        }
    }

    // Apply sort option
    match query.sort.as_ref().map(|s| s.to_lowercase()).as_deref() {
        Some("earliest") => {
            followers_query = followers_query.order(social_graph_relationships::created_at.asc());
        }
        Some("alphabetical") => {
            followers_query = followers_query.order(profiles::username.asc());
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
            i32,
            Option<String>,
            String,
            String,
            Option<String>,
            Option<String>,
        )>(&mut conn)
        .await;

    // Also get the total count for pagination info (with same search filter)
    let mut count_query = social_graph_relationships::table
        .filter(
            social_graph_relationships::following_address.eq(&profile_id)
                .or(social_graph_relationships::following_address.eq(&wallet_address))
        )
        .inner_join(profiles::table.on(
            diesel::dsl::sql::<diesel::sql_types::Bool>(
                "profiles.profile_id = social_graph_relationships.follower_address OR profiles.owner_address = social_graph_relationships.follower_address",
            )
        ))
        .into_boxed();

    if let Some(ref term) = query.search {
        if !term.trim().is_empty() {
            let pattern = format!("%{}%", term.trim());
            count_query = count_query.filter(
                profiles::username
                    .ilike(pattern.clone())
                    .or(profiles::display_name.ilike(pattern.clone()))
                    .or(profiles::owner_address.ilike(pattern.clone())),
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

            for (id, follower_profile_id, owner_address, username, display_name, profile_photo) in
                follows
            {
                // Calculate relationship status from viewer's perspective (if viewer_id provided)
                let (is_following, follows_back) = if let Some(ref viewer_id) = query.viewer_id {
                    // Check if viewer is following this profile
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

                    // Check if this profile follows the viewer back
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

                follows_detail.push(FollowDetail {
                    id,
                    profile_id: follower_profile_id,
                    owner_address,
                    username,
                    display_name,
                    profile_photo,
                    follows_back,
                    is_following,
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
pub async fn check_following(
    State(db_pool): State<DbPool>,
    Path((follower_profile_id, following_profile_id)): Path<(String, String)>,
) -> impl IntoResponse {
    debug!(
        "Checking if profile {} follows profile {}",
        follower_profile_id, following_profile_id
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

    // Helper function to find profile by multiple identifiers
    async fn find_profile_identifiers(
        conn: &mut crate::db::DbConnection,
        identifier: &str,
    ) -> Result<Option<(String, String)>, diesel::result::Error> {
        // Try to find profile by profile_id, owner_address, or username
        let result = profiles::table
            .filter(
                profiles::profile_id
                    .eq(identifier)
                    .or(profiles::owner_address.eq(identifier))
                    .or(profiles::username.eq(identifier)),
            )
            .select((profiles::profile_id.nullable(), profiles::owner_address))
            .first::<(Option<String>, String)>(conn)
            .await;

        match result {
            Ok((profile_id, owner_address)) => {
                // Return (profile_id_or_address, owner_address)
                Ok(Some((
                    profile_id.unwrap_or(owner_address.clone()),
                    owner_address,
                )))
            }
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    // Find follower profile identifiers
    let (follower_profile_identifier, follower_wallet) =
        match find_profile_identifiers(&mut conn, &follower_profile_id).await {
            Ok(Some((profile_id, wallet))) => (profile_id, wallet),
            Ok(None) => {
                debug!("Follower profile not found: {}", follower_profile_id);
                return (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "error": "Follower profile not found",
                        "is_following": false
                    })),
                );
            }
            Err(e) => {
                error!("Failed to check follower profile: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Failed to check follower profile: {}", e),
                        "is_following": false
                    })),
                );
            }
        };

    // Find following profile identifiers
    let (following_profile_identifier, following_wallet) =
        match find_profile_identifiers(&mut conn, &following_profile_id).await {
            Ok(Some((profile_id, wallet))) => (profile_id, wallet),
            Ok(None) => {
                debug!("Following profile not found: {}", following_profile_id);
                return (
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({
                        "error": "Following profile not found",
                        "is_following": false
                    })),
                );
            }
            Err(e) => {
                error!("Failed to check following profile: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Failed to check following profile: {}", e),
                        "is_following": false
                    })),
                );
            }
        };

    // Check if a relationship exists
    // Note: The follower_address and following_address fields may contain various identifiers
    // (profile_ids, wallet addresses, etc.) so we check all possible combinations

    let relationship_exists = social_graph_relationships::table
        .filter(
            // Check all possible combinations of identifiers stored in the relationship table
            (social_graph_relationships::follower_address
                .eq(&follower_profile_identifier)
                .and(
                    social_graph_relationships::following_address.eq(&following_profile_identifier),
                ))
            .or(
                // Follower as profile_id, following as wallet
                social_graph_relationships::follower_address
                    .eq(&follower_profile_identifier)
                    .and(social_graph_relationships::following_address.eq(&following_wallet)),
            )
            .or(
                // Follower as wallet, following as profile_id
                social_graph_relationships::follower_address
                    .eq(&follower_wallet)
                    .and(
                        social_graph_relationships::following_address
                            .eq(&following_profile_identifier),
                    ),
            )
            .or(
                // Both as wallet addresses
                social_graph_relationships::follower_address
                    .eq(&follower_wallet)
                    .and(social_graph_relationships::following_address.eq(&following_wallet)),
            )
            .or(
                // Also check with original input parameters in case they're stored differently
                social_graph_relationships::follower_address
                    .eq(&follower_profile_id)
                    .and(social_graph_relationships::following_address.eq(&following_profile_id)),
            ),
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
                    // Check all possible combinations for reverse relationship
                    (social_graph_relationships::follower_address
                        .eq(&following_profile_identifier)
                        .and(
                            social_graph_relationships::following_address
                                .eq(&follower_profile_identifier),
                        ))
                    .or(social_graph_relationships::follower_address
                        .eq(&following_profile_identifier)
                        .and(social_graph_relationships::following_address.eq(&follower_wallet)))
                    .or(social_graph_relationships::follower_address
                        .eq(&following_wallet)
                        .and(
                            social_graph_relationships::following_address
                                .eq(&follower_profile_identifier),
                        ))
                    .or(social_graph_relationships::follower_address
                        .eq(&following_wallet)
                        .and(social_graph_relationships::following_address.eq(&follower_wallet)))
                    .or(social_graph_relationships::follower_address
                        .eq(&following_profile_id)
                        .and(
                            social_graph_relationships::following_address.eq(&follower_profile_id),
                        )),
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
pub async fn get_follow_stats(
    State(db_pool): State<DbPool>,
    Path(profile_id): Path<String>,
) -> impl IntoResponse {
    debug!("Getting follow stats for profile_id: {}", profile_id);

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

    // Get profile stats from the profiles table using profile_id
    let profile_result = profiles::table
        .filter(profiles::profile_id.eq(&profile_id))
        .select((
            profiles::followers_count,
            profiles::following_count,
            profiles::username,
            profiles::display_name.nullable(),
            profiles::profile_photo.nullable(),
        ))
        .first::<(i32, i32, String, Option<String>, Option<String>)>(&mut conn)
        .await;

    match profile_result {
        Ok((followers, following, username, display_name, profile_photo)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "profile_id": profile_id,
                "username": username,
                "display_name": display_name,
                "profile_photo": profile_photo,
                "followers_count": followers,
                "following_count": following
            })),
        ),
        Err(diesel::result::Error::NotFound) => {
            debug!("Profile not found with profile_id: {}", profile_id);
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Profile not found"
                })),
            )
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
