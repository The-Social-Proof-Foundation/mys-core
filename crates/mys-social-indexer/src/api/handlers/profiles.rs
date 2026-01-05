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
use serde::{Deserialize, Serialize};

use crate::db::DbPool;
use crate::models::Profile;
use crate::models::profile_extras::ProfileBadge;
use crate::schema::{profiles, profile_badges};

#[derive(Debug, Deserialize)]
pub struct ProfileQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

/// Get a list of latest profiles with pagination in descending order by id
pub async fn latest_profiles(
    State(db_pool): State<DbPool>,
    Query(query): Query<ProfileQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database error: {}", e)
                })),
            )
        }
    };

    // Get total count for pagination info
    let total_count = match profiles::table.count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    // Get profiles in descending order by id
    let profiles_result = profiles::table
        .order_by(profiles::id.desc())
        .limit(limit)
        .offset(offset)
        .load::<Profile>(&mut conn)
        .await;

    match profiles_result {
        Ok(profiles) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "profiles": profiles,
                "pagination": {
                    "total": total_count,
                    "limit": limit,
                    "offset": offset,
                    "page": page,
                    "total_pages": total_pages
                }
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to fetch profiles: {}", e)
            })),
        ),
    }
}

/// Get a profile by address
pub async fn get_profile_by_address(
    State(db_pool): State<DbPool>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database error: {}", e)
                })),
            )
        }
    };

    let profile_result = profiles::table
        .filter(profiles::owner_address.eq(address))
        .first::<Profile>(&mut conn)
        .await;

    match profile_result {
        Ok(profile) => (
            StatusCode::OK,
            Json(serde_json::to_value(profile).unwrap_or_default()),
        ),
        Err(diesel::result::Error::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Profile not found"
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to fetch profile: {}", e)
            })),
        ),
    }
}

/// Get a profile by username
pub async fn get_profile_by_username(
    State(db_pool): State<DbPool>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database error: {}", e)
                })),
            )
        }
    };

    let profile_result = profiles::table
        .filter(profiles::username.eq(username))
        .first::<Profile>(&mut conn)
        .await;

    match profile_result {
        Ok(profile) => (
            StatusCode::OK,
            Json(serde_json::to_value(profile).unwrap_or_default()),
        ),
        Err(diesel::result::Error::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Profile not found"
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to fetch profile: {}", e)
            })),
        ),
    }
}

/// Response structure for username availability check
#[derive(Debug, Serialize)]
pub struct UsernameAvailabilityResponse {
    pub username: String,
    pub available: bool,
    pub reason: Option<String>,
}

/// Check if a username is available for registration
///
/// This endpoint validates the username format and checks database availability.
/// Returns detailed information about username availability and validation.
pub async fn check_username_availability(
    State(db_pool): State<DbPool>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    // Validate username format first (before hitting database)
    if let Some(validation_error) = validate_username(&username) {
        return (
            StatusCode::BAD_REQUEST,
            Json(UsernameAvailabilityResponse {
                username: username.clone(),
                available: false,
                reason: Some(validation_error),
            }),
        );
    }

    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UsernameAvailabilityResponse {
                    username: username.clone(),
                    available: false,
                    reason: Some(format!("Database connection error: {}", e)),
                }),
            )
        }
    };

    // Check if username exists in database (case-insensitive)
    let username_exists = match profiles::table
        .filter(profiles::username.eq(&username))
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count > 0,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UsernameAvailabilityResponse {
                    username: username.clone(),
                    available: false,
                    reason: Some(format!("Database query error: {}", e)),
                }),
            )
        }
    };

    let available = !username_exists;
    let reason = if username_exists {
        Some("Username is already taken".to_string())
    } else {
        None
    };

    (
        StatusCode::OK,
        Json(UsernameAvailabilityResponse {
            username,
            available,
            reason,
        }),
    )
}

/// Validate username format according to MySocial rules
///
/// Returns None if valid, Some(error_message) if invalid
fn validate_username(username: &str) -> Option<String> {
    // Check if username is empty
    if username.is_empty() {
        return Some("Username cannot be empty".to_string());
    }

    // Check length constraints (3-30 characters)
    if username.len() < 2 {
        return Some("Username must be at least 3 characters long".to_string());
    }

    if username.len() > 50 {
        return Some("Username cannot be longer than 30 characters".to_string());
    }

    // Check if username starts or ends with underscore or hyphen
    if username.starts_with('_') || username.starts_with('-') {
        return Some("Username cannot start with underscore or hyphen".to_string());
    }

    if username.ends_with('_') || username.ends_with('-') {
        return Some("Username cannot end with underscore or hyphen".to_string());
    }

    // Check for valid characters (alphanumeric, underscore, hyphen)
    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Some(
            "Username can only contain letters, numbers, underscores, and hyphens".to_string(),
        );
    }

    // Check for consecutive special characters
    if username.contains("__")
        || username.contains("--")
        || username.contains("_-")
        || username.contains("-_")
    {
        return Some("Username cannot contain consecutive special characters".to_string());
    }

    // Check for reserved words (case-insensitive) - matches profile.move RESERVED_NAMES
    let reserved_words = [
        "admin",
        "administrator",
        "owner",
        "mod",
        "moderator",
        "staff",
        "support",
        "myso",
        "mysocial",
        "system",
        "root",
        "official",
        // Inappropriate names
        "fuck",
        "shit",
        "ass",
        "piss",
        "cunt",
        "asshole",
        "dick",
        "pussy",
        "sex",
    ];

    if reserved_words.contains(&username.to_lowercase().as_str()) {
        return Some("Username is reserved and cannot be used".to_string());
    }

    None
}

// ===========================================================================
// PROFILE BADGE HANDLERS
// ===========================================================================

/// Query parameters for fetching profile badges
#[derive(Debug, Deserialize)]
pub struct ProfileBadgeQuery {
    /// Limit for number of badges to return
    #[serde(default = "default_badge_limit")]
    pub limit: i64,

    /// Offset for pagination
    #[serde(default)]
    pub offset: i64,

    /// Filter by platform ID
    pub platform_id: Option<String>,

    /// Filter by revoked status
    pub revoked: Option<bool>,

    /// Filter by badge type/tier
    pub badge_type: Option<i16>,
}

fn default_badge_limit() -> i64 {
    20
}

/// Get all badges for a specific profile
/// Accepts wallet address (owner_address) as input
pub async fn get_profile_badges(
    Path(wallet_address): Path<String>,
    Query(query): Query<ProfileBadgeQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                })),
            )
        }
    };

    // Resolve wallet address to profile_id for profile_badges query
    let profile_id = match profiles::table
        .filter(profiles::owner_address.eq(&wallet_address))
        .select(profiles::profile_id.nullable())
        .first::<Option<String>>(&mut conn)
        .await
    {
        Ok(Some(pid)) => pid,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Profile found but no profile_id"
                })),
            )
        }
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Profile not found"
                })),
            )
        }
    };

    let limit = query.limit.min(100);
    let offset = query.offset;

    // Build the query with filters
    let mut query_builder = profile_badges::table
        .filter(profile_badges::profile_id.eq(&profile_id))
        .into_boxed();

    // Apply optional filters
    if let Some(platform_id) = &query.platform_id {
        query_builder = query_builder.filter(profile_badges::platform_id.eq(platform_id));
    }

    if let Some(revoked) = query.revoked {
        query_builder = query_builder.filter(profile_badges::revoked.eq(revoked));
    }

    if let Some(badge_type) = query.badge_type {
        query_builder = query_builder.filter(profile_badges::badge_type.eq(badge_type));
    }

    // Build count query separately (can't clone BoxedSelectStatement)
    let mut count_query = profile_badges::table
        .filter(profile_badges::profile_id.eq(&profile_id))
        .into_boxed();

    if let Some(platform_id) = &query.platform_id {
        count_query = count_query.filter(profile_badges::platform_id.eq(platform_id));
    }

    if let Some(revoked) = query.revoked {
        count_query = count_query.filter(profile_badges::revoked.eq(revoked));
    }

    if let Some(badge_type) = query.badge_type {
        count_query = count_query.filter(profile_badges::badge_type.eq(badge_type));
    }

    // Get total count for pagination
    let total_count = match count_query
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to get badge count: {}", e)
                })),
            )
        }
    };

    // Get the badges
    let badges_result = query_builder
        .order_by(profile_badges::assigned_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<ProfileBadge>(&mut conn)
        .await;

    match badges_result {
        Ok(badges) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "badges": badges,
                "pagination": {
                    "total": total_count,
                    "limit": limit,
                    "offset": offset
                }
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to fetch badges: {}", e)
            })),
        ),
    }
}

/// Query parameters for getting a badge by ID
#[derive(Debug, Deserialize)]
pub struct BadgeByIdQuery {
    /// Profile ID (required to uniquely identify the badge)
    pub profile_id: Option<String>,
}

/// Get a specific badge by badge_id
pub async fn get_profile_badge_by_id(
    Path(badge_id): Path<String>,
    Query(query): Query<BadgeByIdQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let profile_id = match query.profile_id {
        Some(id) => id,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "profile_id query parameter is required"
                })),
            )
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
            )
        }
    };

    let badge_result = profile_badges::table
        .filter(profile_badges::badge_id.eq(&badge_id))
        .filter(profile_badges::profile_id.eq(&profile_id))
        .first::<ProfileBadge>(&mut conn)
        .await;

    match badge_result {
        Ok(badge) => (StatusCode::OK, Json(serde_json::to_value(badge).unwrap_or_default())),
        Err(diesel::result::Error::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Badge not found"
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to fetch badge: {}", e)
            })),
        ),
    }
}

/// Query parameters for listing all badges across profiles
#[derive(Debug, Deserialize)]
pub struct BadgesQuery {
    /// Limit for number of badges to return
    #[serde(default = "default_badge_limit")]
    pub limit: i64,

    /// Offset for pagination
    #[serde(default)]
    pub offset: i64,

    /// Filter by profile ID
    pub profile_id: Option<String>,

    /// Filter by platform ID
    pub platform_id: Option<String>,

    /// Filter by revoked status
    pub revoked: Option<bool>,

    /// Filter by badge type/tier
    pub badge_type: Option<i16>,
}

/// List all badges across all profiles with optional filtering
pub async fn get_badges(
    Query(query): Query<BadgesQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                })),
            )
        }
    };

    let limit = query.limit.min(100);
    let offset = query.offset;

    // Build the query with filters
    let mut query_builder = profile_badges::table.into_boxed();

    // Apply optional filters
    if let Some(profile_id) = &query.profile_id {
        query_builder = query_builder.filter(profile_badges::profile_id.eq(profile_id));
    }

    if let Some(platform_id) = &query.platform_id {
        query_builder = query_builder.filter(profile_badges::platform_id.eq(platform_id));
    }

    if let Some(revoked) = query.revoked {
        query_builder = query_builder.filter(profile_badges::revoked.eq(revoked));
    }

    if let Some(badge_type) = query.badge_type {
        query_builder = query_builder.filter(profile_badges::badge_type.eq(badge_type));
    }

    // Build count query separately (can't clone BoxedSelectStatement)
    let mut count_query = profile_badges::table.into_boxed();

    if let Some(profile_id) = &query.profile_id {
        count_query = count_query.filter(profile_badges::profile_id.eq(profile_id));
    }

    if let Some(platform_id) = &query.platform_id {
        count_query = count_query.filter(profile_badges::platform_id.eq(platform_id));
    }

    if let Some(revoked) = query.revoked {
        count_query = count_query.filter(profile_badges::revoked.eq(revoked));
    }

    if let Some(badge_type) = query.badge_type {
        count_query = count_query.filter(profile_badges::badge_type.eq(badge_type));
    }

    // Get total count for pagination
    let total_count = match count_query
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to get badge count: {}", e)
                })),
            )
        }
    };

    // Get the badges
    let badges_result = query_builder
        .order_by(profile_badges::assigned_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<ProfileBadge>(&mut conn)
        .await;

    match badges_result {
        Ok(badges) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "badges": badges,
                "pagination": {
                    "total": total_count,
                    "limit": limit,
                    "offset": offset
                }
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to fetch badges: {}", e)
            })),
        ),
    }
}
