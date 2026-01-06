// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use tracing::{debug, error};

use crate::db::DbPool;
use crate::models::platform::{
    Platform, PlatformEvent, PlatformWithDetails,
};
use crate::schema::{platform_blocked_profiles, platform_events, platform_memberships, platform_moderators, platforms, profiles};
use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct PlatformQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    pub search: Option<String>,
    pub primary_category: Option<String>,
    pub secondary_category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlatformEventsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    pub event_type: Option<String>,
}

/// Get a list of all platforms with pagination
pub async fn get_platforms(
    State(db_pool): State<DbPool>,
    Query(query): Query<PlatformQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    debug!(
        "Getting platforms list with limit: {}, offset: {}",
        limit, offset
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

    // Build base query with category filters
    let mut count_query = platforms::table.into_boxed();
    let mut platforms_query = platforms::table.into_boxed();

    // Apply category filters if provided
    if let Some(ref primary_cat) = query.primary_category {
        count_query = count_query.filter(platforms::primary_category.eq(primary_cat));
        platforms_query = platforms_query.filter(platforms::primary_category.eq(primary_cat));
    }
    if let Some(ref secondary_cat) = query.secondary_category {
        count_query = count_query.filter(platforms::secondary_category.eq(secondary_cat));
        platforms_query = platforms_query.filter(platforms::secondary_category.eq(secondary_cat));
    }

    // Get the total count for pagination info
    let total_count = match count_query.count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    // Query platforms with pagination
    let platforms_result = platforms_query
        .order_by(platforms::created_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<Platform>(&mut conn)
        .await;

    match platforms_result {
        Ok(platforms) => {
            // For each platform, get additional information like moderator count
            let mut platform_details = Vec::with_capacity(platforms.len());

            for platform in platforms {
                // Get moderator count
                let moderator_count = platform_moderators::table
                    .filter(platform_moderators::platform_id.eq(&platform.platform_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .await
                    .unwrap_or(0);

                // Get blocked profiles count
                let blocked_count = platform_blocked_profiles::table
                    .filter(platform_blocked_profiles::platform_id.eq(&platform.platform_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .await
                    .unwrap_or(0);

                // Convert platform_names from JSON to Vec<String>
                let platform_names: Option<Vec<String>> = platform
                    .platform_names
                    .as_ref()
                    .and_then(|json| serde_json::from_value(json.clone()).ok());

                // Convert links from JSON to Vec<String>
                let links: Option<Vec<String>> = platform
                    .links
                    .as_ref()
                    .and_then(|json| serde_json::from_value(json.clone()).ok());

                // Build response with details
                platform_details.push(PlatformWithDetails {
                    id: platform.id,
                    platform_id: platform.platform_id,
                    name: platform.name,
                    tagline: platform.tagline,
                    description: platform.description,
                    logo: platform.logo,
                    developer_address: platform.developer_address,
                    terms_of_service: platform.terms_of_service,
                    privacy_policy: platform.privacy_policy,
                    platform_names,
                    links,
                    status: platform.status,
                    status_text: PlatformWithDetails::status_to_text(platform.status),
                    release_date: platform.release_date,
                    shutdown_date: platform.shutdown_date,
                    created_at: platform.created_at,
                    updated_at: platform.updated_at,
                    is_approved: platform.is_approved,
                    approval_changed_at: platform.approval_changed_at,
                    approved_by: platform.approved_by.clone(),
                    wants_dao_governance: platform.wants_dao_governance,
                    governance_registry_id: platform.governance_registry_id.clone(),
                    delegate_count: platform.delegate_count,
                    delegate_term_epochs: platform.delegate_term_epochs,
                    max_votes_per_user: platform.max_votes_per_user,
                    min_on_chain_age_days: platform.min_on_chain_age_days,
                    proposal_submission_cost: platform.proposal_submission_cost,
                    quadratic_base_cost: platform.quadratic_base_cost,
                    quorum_votes: platform.quorum_votes,
                    voting_period_epochs: platform.voting_period_epochs,
                    treasury: platform.treasury,
                    version: platform.version,
                    primary_category: platform.primary_category,
                    secondary_category: platform.secondary_category.clone(),
                    moderator_count,
                    blocked_profiles_count: blocked_count,
                });
            }

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "platforms": platform_details,
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
            error!("Failed to fetch platforms: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch platforms: {}", e)
                })),
            )
        }
    }
}

/// Get a platform by its ID
pub async fn get_platform_by_id(
    State(db_pool): State<DbPool>,
    Path(platform_id): Path<String>,
) -> impl IntoResponse {
    debug!("Getting platform with ID: {}", platform_id);

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

    // Get the platform
    let platform_result = platforms::table
        .filter(platforms::platform_id.eq(&platform_id))
        .first::<Platform>(&mut conn)
        .await;

    match platform_result {
        Ok(platform) => {
            // Get moderator count
            let moderator_count = platform_moderators::table
                .filter(platform_moderators::platform_id.eq(&platform.platform_id))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0);

            // Get blocked profiles count
            let blocked_count = platform_blocked_profiles::table
                .filter(platform_blocked_profiles::platform_id.eq(&platform.platform_id))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0);

            // Get moderators with profile information using LEFT JOIN
            // Join platform_moderators with profiles on moderator_address = owner_address
            let moderators_result = platform_moderators::table
                .filter(platform_moderators::platform_id.eq(&platform.platform_id))
                .left_join(
                    profiles::table.on(
                        profiles::owner_address.eq(platform_moderators::moderator_address),
                    ),
                )
                .select((
                    platform_moderators::id,
                    platform_moderators::platform_id,
                    platform_moderators::moderator_address,
                    platform_moderators::added_by,
                    platform_moderators::created_at,
                    profiles::username.nullable(),
                    profiles::display_name.nullable(),
                    profiles::profile_photo.nullable(),
                    profiles::owner_address.nullable(),
                ))
                .order_by(platform_moderators::created_at.desc())
                .load::<(i32, String, String, String, NaiveDateTime, Option<String>, Option<String>, Option<String>, Option<String>)>(&mut conn)
                .await;

            let moderators: Vec<ModeratorWithProfile> = match moderators_result {
                Ok(moderators_data) => moderators_data
                    .into_iter()
                    .map(|(id, platform_id, moderator_address, added_by, created_at, username, fullname, profile_photo, wallet_address)| {
                        ModeratorWithProfile {
                            id,
                            platform_id,
                            moderator_address: moderator_address.clone(),
                            added_by,
                            created_at,
                            username,
                            fullname,
                            profile_photo,
                            wallet_address: wallet_address.or(Some(moderator_address)),
                        }
                    })
                    .collect(),
                Err(e) => {
                    error!("Failed to fetch moderators: {}", e);
                    Vec::new()
                }
            };

            // Convert platform_names from JSON to Vec<String>
            let platform_names: Option<Vec<String>> = platform
                .platform_names
                .as_ref()
                .and_then(|json| serde_json::from_value(json.clone()).ok());

            // Convert links from JSON to Vec<String>
            let links: Option<Vec<String>> = platform
                .links
                .as_ref()
                .and_then(|json| serde_json::from_value(json.clone()).ok());

            // Build response with details
            let platform_details = PlatformWithDetails {
                id: platform.id,
                platform_id: platform.platform_id,
                name: platform.name,
                tagline: platform.tagline,
                description: platform.description,
                logo: platform.logo,
                developer_address: platform.developer_address,
                terms_of_service: platform.terms_of_service,
                privacy_policy: platform.privacy_policy,
                platform_names,
                links,
                status: platform.status,
                status_text: PlatformWithDetails::status_to_text(platform.status),
                release_date: platform.release_date,
                shutdown_date: platform.shutdown_date,
                created_at: platform.created_at,
                updated_at: platform.updated_at,
                is_approved: platform.is_approved,
                approval_changed_at: platform.approval_changed_at,
                approved_by: platform.approved_by.clone(),
                wants_dao_governance: platform.wants_dao_governance,
                governance_registry_id: platform.governance_registry_id.clone(),
                delegate_count: platform.delegate_count,
                delegate_term_epochs: platform.delegate_term_epochs,
                max_votes_per_user: platform.max_votes_per_user,
                min_on_chain_age_days: platform.min_on_chain_age_days,
                proposal_submission_cost: platform.proposal_submission_cost,
                quadratic_base_cost: platform.quadratic_base_cost,
                quorum_votes: platform.quorum_votes,
                voting_period_epochs: platform.voting_period_epochs,
                treasury: platform.treasury,
                version: platform.version,
                primary_category: platform.primary_category,
                secondary_category: platform.secondary_category.clone(),
                moderator_count,
                blocked_profiles_count: blocked_count,
            };

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "platform": platform_details,
                    "moderators": moderators
                })),
            )
        }
        Err(diesel::result::Error::NotFound) => {
            debug!("Platform not found: {}", platform_id);
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Platform not found"
                })),
            )
        }
        Err(e) => {
            error!("Failed to fetch platform: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch platform: {}", e)
                })),
            )
        }
    }
}

/// Get platform moderators
pub async fn get_platform_moderators(
    State(db_pool): State<DbPool>,
    Path(platform_id): Path<String>,
    Query(query): Query<PlatformQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    debug!("Getting moderators for platform: {}", platform_id);

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

    // Check if platform exists
    let platform_exists = match platforms::table
        .filter(platforms::platform_id.eq(&platform_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count > 0,
        Err(e) => {
            error!("Failed to check platform: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to check platform: {}", e)
                })),
            );
        }
    };

    if !platform_exists {
        debug!("Platform not found: {}", platform_id);
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Platform not found"
            })),
        );
    }

    // Get the total count for pagination info
    let total_count = match platform_moderators::table
        .filter(platform_moderators::platform_id.eq(&platform_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count,
        Err(_) => 0,
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    // Get moderators with profile information using LEFT JOIN
    // Join platform_moderators with profiles on moderator_address = owner_address
    let moderators_result = platform_moderators::table
        .filter(platform_moderators::platform_id.eq(&platform_id))
        .left_join(
            profiles::table.on(
                profiles::owner_address.eq(platform_moderators::moderator_address),
            ),
        )
        .select((
            platform_moderators::id,
            platform_moderators::platform_id,
            platform_moderators::moderator_address,
            platform_moderators::added_by,
            platform_moderators::created_at,
            profiles::username.nullable(),
            profiles::display_name.nullable(),
            profiles::profile_photo.nullable(),
            profiles::owner_address.nullable(),
        ))
        .order_by(platform_moderators::created_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<(i32, String, String, String, NaiveDateTime, Option<String>, Option<String>, Option<String>, Option<String>)>(&mut conn)
        .await;

    match moderators_result {
        Ok(moderators_data) => {
            let moderators: Vec<ModeratorWithProfile> = moderators_data
                .into_iter()
                .map(|(id, platform_id, moderator_address, added_by, created_at, username, fullname, profile_photo, wallet_address)| {
                    ModeratorWithProfile {
                        id,
                        platform_id,
                        moderator_address: moderator_address.clone(),
                        added_by,
                        created_at,
                        username,
                        fullname,
                        profile_photo,
                        wallet_address: wallet_address.or(Some(moderator_address)),
                    }
                })
                .collect();

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "moderators": moderators,
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
            error!("Failed to fetch moderators: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch moderators: {}", e)
                })),
            )
        }
    }
}

/// Get a list of approved platforms with pagination
pub async fn get_approved_platforms(
    State(db_pool): State<DbPool>,
    Query(query): Query<PlatformQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    debug!(
        "Getting approved platforms list with limit: {}, offset: {}",
        limit, offset
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

    // Build base query with approval and category filters
    let mut count_query = platforms::table
        .filter(platforms::is_approved.eq(true))
        .into_boxed();
    let mut platforms_query = platforms::table
        .filter(platforms::is_approved.eq(true))
        .into_boxed();

    // Apply category filters if provided
    if let Some(ref primary_cat) = query.primary_category {
        count_query = count_query.filter(platforms::primary_category.eq(primary_cat));
        platforms_query = platforms_query.filter(platforms::primary_category.eq(primary_cat));
    }
    if let Some(ref secondary_cat) = query.secondary_category {
        count_query = count_query.filter(platforms::secondary_category.eq(secondary_cat));
        platforms_query = platforms_query.filter(platforms::secondary_category.eq(secondary_cat));
    }

    // Get the total count for pagination info (only approved platforms)
    let total_count = match count_query.count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    // Query platforms with pagination, filtered by approval status
    let platforms_result = platforms_query
        .order_by(platforms::created_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<Platform>(&mut conn)
        .await;

    match platforms_result {
        Ok(platforms) => {
            // For each platform, get additional information like moderator count
            let mut platform_details = Vec::with_capacity(platforms.len());

            for platform in platforms {
                // Get moderator count
                let moderator_count = platform_moderators::table
                    .filter(platform_moderators::platform_id.eq(&platform.platform_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .await
                    .unwrap_or(0);

                // Get blocked profiles count
                let blocked_count = platform_blocked_profiles::table
                    .filter(platform_blocked_profiles::platform_id.eq(&platform.platform_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .await
                    .unwrap_or(0);

                // Convert platform_names from JSON to Vec<String>
                let platform_names: Option<Vec<String>> = platform
                    .platform_names
                    .as_ref()
                    .and_then(|json| serde_json::from_value(json.clone()).ok());

                // Convert links from JSON to Vec<String>
                let links: Option<Vec<String>> = platform
                    .links
                    .as_ref()
                    .and_then(|json| serde_json::from_value(json.clone()).ok());

                // Build response with details
                platform_details.push(PlatformWithDetails {
                    id: platform.id,
                    platform_id: platform.platform_id,
                    name: platform.name,
                    tagline: platform.tagline,
                    description: platform.description,
                    logo: platform.logo,
                    developer_address: platform.developer_address,
                    terms_of_service: platform.terms_of_service,
                    privacy_policy: platform.privacy_policy,
                    platform_names,
                    links,
                    status: platform.status,
                    status_text: PlatformWithDetails::status_to_text(platform.status),
                    release_date: platform.release_date,
                    shutdown_date: platform.shutdown_date,
                    created_at: platform.created_at,
                    updated_at: platform.updated_at,
                    is_approved: platform.is_approved,
                    approval_changed_at: platform.approval_changed_at,
                    approved_by: platform.approved_by.clone(),
                    wants_dao_governance: platform.wants_dao_governance,
                    governance_registry_id: platform.governance_registry_id.clone(),
                    delegate_count: platform.delegate_count,
                    delegate_term_epochs: platform.delegate_term_epochs,
                    max_votes_per_user: platform.max_votes_per_user,
                    min_on_chain_age_days: platform.min_on_chain_age_days,
                    proposal_submission_cost: platform.proposal_submission_cost,
                    quadratic_base_cost: platform.quadratic_base_cost,
                    quorum_votes: platform.quorum_votes,
                    voting_period_epochs: platform.voting_period_epochs,
                    treasury: platform.treasury,
                    version: platform.version,
                    primary_category: platform.primary_category,
                    secondary_category: platform.secondary_category.clone(),
                    moderator_count,
                    blocked_profiles_count: blocked_count,
                });
            }

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "platforms": platform_details,
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
            error!("Failed to fetch approved platforms: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch approved platforms: {}", e)
                })),
            )
        }
    }
}

/// Get the approval status of a specific platform
pub async fn get_platform_approval_status(
    State(db_pool): State<DbPool>,
    Path(platform_id): Path<String>,
) -> impl IntoResponse {
    debug!("Getting approval status for platform: {}", platform_id);

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

    // Get the platform
    let platform_result = platforms::table
        .filter(platforms::platform_id.eq(&platform_id))
        .select((
            platforms::is_approved,
            platforms::approval_changed_at,
            platforms::approved_by,
        ))
        .first::<(bool, Option<NaiveDateTime>, Option<String>)>(&mut conn)
        .await;

    match platform_result {
        Ok((is_approved, approval_changed_at, approved_by)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "platform_id": platform_id,
                "is_approved": is_approved,
                "approval_changed_at": approval_changed_at,
                "approved_by": approved_by
            })),
        ),
        Err(diesel::result::Error::NotFound) => {
            debug!("Platform not found: {}", platform_id);
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Platform not found"
                })),
            )
        }
        Err(e) => {
            error!("Failed to fetch platform approval status: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch platform approval status: {}", e)
                })),
            )
        }
    }
}

pub async fn get_platform_blocked_profiles(
    State(db_pool): State<DbPool>,
    Path(platform_id): Path<String>,
    Query(query): Query<PlatformQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    debug!("Getting blocked profiles for platform: {}", platform_id);

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

    // Check if platform exists
    let platform_exists = match platforms::table
        .filter(platforms::platform_id.eq(&platform_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count > 0,
        Err(e) => {
            error!("Failed to check platform: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to check platform: {}", e)
                })),
            );
        }
    };

    if !platform_exists {
        debug!("Platform not found: {}", platform_id);
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Platform not found"
            })),
        );
    }

    // Prepare search pattern if provided
    let search_pattern = query.search.as_ref()
        .and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(format!("%{}%", trimmed))
            }
        });

    // Build base query for counting
    let mut count_query = platform_blocked_profiles::table
        .filter(platform_blocked_profiles::platform_id.eq(&platform_id))
        .left_join(
            profiles::table.on(
                profiles::owner_address.eq(platform_blocked_profiles::profile_id),
            ),
        )
        .into_boxed();

    // Apply search filter to count query if provided
    if let Some(ref pattern) = search_pattern {
        count_query = count_query.filter(
            profiles::username
                .ilike(pattern.clone())
                .or(profiles::owner_address.ilike(pattern.clone())),
        );
    }

    // Get the total count for pagination info
    let total_count = match count_query.count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    // Build query for fetching blocked profiles
    // Get blocked profiles with profile information using LEFT JOIN
    // Join platform_blocked_profiles with profiles on profile_id = owner_address
    let mut blocked_profiles_query = platform_blocked_profiles::table
        .filter(platform_blocked_profiles::platform_id.eq(&platform_id))
        .left_join(
            profiles::table.on(
                profiles::owner_address.eq(platform_blocked_profiles::profile_id),
            ),
        )
        .select((
            platform_blocked_profiles::id,
            platform_blocked_profiles::platform_id,
            platform_blocked_profiles::profile_id,
            platform_blocked_profiles::blocked_by,
            platform_blocked_profiles::created_at,
            profiles::username.nullable(),
            profiles::display_name.nullable(),
            profiles::profile_photo.nullable(),
            profiles::owner_address.nullable(),
        ))
        .order_by(platform_blocked_profiles::created_at.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    // Apply search filter to blocked profiles query if provided
    if let Some(ref pattern) = search_pattern {
        blocked_profiles_query = blocked_profiles_query.filter(
            profiles::username
                .ilike(pattern.clone())
                .or(profiles::owner_address.ilike(pattern.clone())),
        );
    }

    let blocked_profiles_result = blocked_profiles_query
        .load::<(i32, String, String, String, NaiveDateTime, Option<String>, Option<String>, Option<String>, Option<String>)>(&mut conn)
        .await;

    match blocked_profiles_result {
        Ok(blocked_data) => {
            let blocked_profiles: Vec<BlockedProfileWithProfile> = blocked_data
                .into_iter()
                .map(|(id, platform_id, profile_id, blocked_by, created_at, username, fullname, profile_photo, wallet_address)| {
                    BlockedProfileWithProfile {
                        id,
                        platform_id,
                        profile_id: profile_id.clone(),
                        blocked_by,
                        created_at,
                        username,
                        fullname,
                        profile_photo,
                        wallet_address: wallet_address.or(Some(profile_id)),
                    }
                })
                .collect();

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "blocked_profiles": blocked_profiles,
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
            error!("Failed to fetch blocked profiles: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch blocked profiles: {}", e)
                })),
            )
        }
    }
}

/// Platform member with profile information
#[derive(Debug, Serialize)]
pub struct PlatformMember {
    pub profile_id: String,
    pub wallet_address: String,
    pub username: String,
    pub fullname: Option<String>,
    pub profile_photo: Option<String>,
    pub joined_at: NaiveDateTime,
}

/// Platform moderator with profile information
#[derive(Debug, Serialize)]
pub struct ModeratorWithProfile {
    pub id: i32,
    pub platform_id: String,
    pub moderator_address: String,
    pub added_by: String,
    pub created_at: NaiveDateTime,
    pub username: Option<String>,
    pub fullname: Option<String>,
    pub profile_photo: Option<String>,
    pub wallet_address: Option<String>,
}

/// Platform blocked profile with profile information
#[derive(Debug, Serialize)]
pub struct BlockedProfileWithProfile {
    pub id: i32,
    pub platform_id: String,
    pub profile_id: String,
    pub blocked_by: String,
    pub created_at: NaiveDateTime,
    pub username: Option<String>,
    pub fullname: Option<String>,
    pub profile_photo: Option<String>,
    pub wallet_address: Option<String>,
}

/// Get platform members with profile information
pub async fn get_platform_members(
    State(db_pool): State<DbPool>,
    Path(platform_id): Path<String>,
    Query(query): Query<PlatformQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    debug!("Getting members for platform: {}", platform_id);

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

    // Check if platform exists
    let platform_exists = match platforms::table
        .filter(platforms::platform_id.eq(&platform_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count > 0,
        Err(e) => {
            error!("Failed to check platform: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to check platform: {}", e)
                })),
            );
        }
    };

    if !platform_exists {
        debug!("Platform not found: {}", platform_id);
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Platform not found"
            })),
        );
    }

    // Prepare search pattern if provided
    let search_pattern = query.search.as_ref()
        .and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(format!("%{}%", trimmed))
            }
        });

    let mut count_query = platform_memberships::table
        .filter(platform_memberships::platform_id.eq(&platform_id))
        .left_join(
            profiles::table.on(
                diesel::dsl::sql::<diesel::sql_types::Bool>(
                    "profiles.owner_address = platform_memberships.wallet_address",
                )
            ),
        )
        .into_boxed();

    // Apply search filter to count query if provided
    if let Some(ref pattern) = search_pattern {
        count_query = count_query.filter(
            profiles::username
                .ilike(pattern.clone())
                .or(profiles::owner_address.ilike(pattern.clone())),
        );
    }

    // Get the total count for pagination info
    let total_count = match count_query.count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    let mut members_query = platform_memberships::table
        .filter(platform_memberships::platform_id.eq(&platform_id))
        .left_join(
            profiles::table.on(
                diesel::dsl::sql::<diesel::sql_types::Bool>(
                    "profiles.owner_address = platform_memberships.wallet_address",
                )
            ),
        )
        .select((
            platform_memberships::wallet_address,
            profiles::owner_address.nullable(),
            profiles::username.nullable(),
            profiles::display_name.nullable(),
            profiles::profile_photo.nullable(),
            platform_memberships::joined_at,
        ))
        .order_by(platform_memberships::joined_at.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    // Apply search filter to members query if provided
    if let Some(ref pattern) = search_pattern {
        members_query = members_query.filter(
            profiles::username
                .ilike(pattern.clone())
                .or(profiles::owner_address.ilike(pattern.clone())),
        );
    }

    let members_result = members_query
        .load::<(String, Option<String>, Option<String>, Option<String>, Option<String>, NaiveDateTime)>(&mut conn)
        .await;

    debug!("Members query result: {:?}", members_result.is_ok());

    match members_result {
        Ok(members_data) => {
            let members: Vec<PlatformMember> = members_data
                .into_iter()
                .map(|(wallet_addr, owner_address, username, display_name, profile_photo, joined_at)| {
                    PlatformMember {
                        profile_id: wallet_addr.clone(), // wallet_address is stored here
                        wallet_address: owner_address.unwrap_or_else(|| wallet_addr.clone()),
                        username: username.unwrap_or_else(|| "unknown".to_string()),
                        fullname: display_name,
                        profile_photo,
                        joined_at,
                    }
                })
                .collect();

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "members": members,
                    "total": total_count,
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
            error!("Failed to fetch platform members: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch platform members: {}", e)
                })),
            )
        }
    }
}

/// Check if a profile is a member of a platform
/// Accepts wallet address (owner_address) as input
pub async fn check_platform_membership(
    Path((platform_id, wallet_address)): Path<(String, String)>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    debug!(
        "Checking if wallet {} is a member of platform {}",
        wallet_address, platform_id
    );

    // Input validation
    if platform_id.trim().is_empty() || wallet_address.trim().is_empty() {
        debug!("Invalid IDs: empty string");
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Platform ID and wallet address are required"
            })),
        );
    }

    // Basic length validation to prevent potential attacks
    if platform_id.len() > 256 || wallet_address.len() > 256 {
        debug!("Invalid IDs: too long");
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Platform ID and wallet address must be 256 characters or less"
            })),
        );
    }

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

    // Verify profile exists
    let profile_exists = profiles::table
        .filter(profiles::owner_address.eq(&wallet_address))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .unwrap_or(0)
        > 0;

    if !profile_exists {
        debug!("Profile not found with wallet_address: {}", wallet_address);
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Profile not found: {}", wallet_address)
            })),
        );
    }

    // wallet_address in platform_memberships table stores wallet address
    let wallet_addr = wallet_address;

    // Check if platform exists
    let platform_exists = match platforms::table
        .filter(platforms::platform_id.eq(&platform_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count > 0,
        Err(e) => {
            error!("Failed to check platform existence: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to check platform: {}", e)
                })),
            );
        }
    };

    if !platform_exists {
        debug!("Platform not found: {}", platform_id);
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Platform not found"
            })),
        );
    }

    // Check membership in platform_memberships table and get joined_at date
    let membership_result = platform_memberships::table
        .filter(platform_memberships::platform_id.eq(&platform_id))
        .filter(platform_memberships::wallet_address.eq(&wallet_addr))
        .select(platform_memberships::joined_at)
        .first::<chrono::NaiveDateTime>(&mut conn)
        .await;

    match membership_result {
        Ok(joined_at) => {
            debug!(
                "Found membership: wallet {} is a member of platform {}",
                wallet_addr, platform_id
            );
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "is_member": true,
                    "joined_at": joined_at.format("%Y-%m-%dT%H:%M:%S%.f").to_string()
                })),
            )
        }
        Err(diesel::result::Error::NotFound) => {
            debug!(
                "No membership found: wallet {} is not a member of platform {}",
                wallet_addr, platform_id
            );
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "is_member": false
                })),
            )
        }
        Err(e) => {
            error!("Error querying platform_memberships table: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to check membership: {}", e)
                })),
            )
        }
    }
}

/// Platform membership with platform details
#[derive(Debug, Serialize)]
pub struct PlatformMembership {
    #[serde(flatten)]
    pub platform: PlatformWithDetails,
    pub joined_at: NaiveDateTime,
}

/// Get all platforms a profile is a member of
/// Accepts wallet address (owner_address) as input
pub async fn get_profile_platforms(
    Path(wallet_address): Path<String>,
    Query(query): Query<PlatformQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    debug!("Getting platforms for wallet_address: {}", wallet_address);

    // Input validation
    if wallet_address.trim().is_empty() {
        debug!("Invalid wallet_address: empty string");
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Wallet address is required"
            })),
        );
    }

    // Basic length validation to prevent potential attacks
    if wallet_address.len() > 256 {
        debug!("Invalid wallet_address: too long");
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Wallet address must be 256 characters or less"
            })),
        );
    }

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

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

    // Verify profile exists
    let profile_exists = profiles::table
        .filter(profiles::owner_address.eq(&wallet_address))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .unwrap_or(0)
        > 0;

    if !profile_exists {
        debug!("Profile not found with wallet_address: {}", wallet_address);
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Profile not found"
            })),
        );
    }

    // wallet_address in platform_memberships table stores wallet address
    let wallet_addr = wallet_address;

    // Prepare search pattern if provided
    let search_pattern = query.search.as_ref().and_then(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(format!("%{}%", trimmed))
        }
    });

    // Build count query
    let mut count_query = platform_memberships::table
        .filter(platform_memberships::wallet_address.eq(&wallet_addr))
        .inner_join(platforms::table.on(platforms::platform_id.eq(platform_memberships::platform_id)))
        .into_boxed();

    // Apply search filter to count query if provided
    if let Some(ref pattern) = search_pattern {
        count_query = count_query.filter(
            platforms::name
                .ilike(pattern.clone())
                .or(platforms::platform_id.ilike(pattern.clone()))
                .or(platforms::tagline.ilike(pattern.clone())),
        );
    }

    // Get the total count for pagination info
    let total_count = match count_query.count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(e) => {
            error!("Failed to get platform memberships count: {}", e);
            0
        }
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    // Build main query - join platform_memberships with platforms
    let mut platforms_query = platform_memberships::table
        .filter(platform_memberships::wallet_address.eq(&wallet_addr))
        .inner_join(platforms::table.on(platforms::platform_id.eq(platform_memberships::platform_id)))
        .select((
            platforms::id,
            platforms::platform_id,
            platforms::name,
            platforms::tagline,
            platforms::description,
            platforms::logo,
            platforms::developer_address,
            platforms::terms_of_service,
            platforms::privacy_policy,
            platforms::platform_names,
            platforms::links,
            platforms::status,
            platforms::release_date,
            platforms::shutdown_date,
            platforms::created_at,
            platforms::updated_at,
            platforms::is_approved,
            platforms::approval_changed_at,
            platforms::approved_by,
            platforms::wants_dao_governance,
            platforms::governance_registry_id,
            platforms::delegate_count,
            platforms::delegate_term_epochs,
            platforms::max_votes_per_user,
            platforms::min_on_chain_age_days,
            platforms::proposal_submission_cost,
            platforms::quadratic_base_cost,
            platforms::quorum_votes,
            platforms::voting_period_epochs,
            platforms::treasury,
            platforms::version,
            platforms::primary_category,
            platforms::secondary_category,
            platform_memberships::joined_at,
        ))
        .order_by(platform_memberships::joined_at.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    // Apply search filter to platforms query if provided
    if let Some(ref pattern) = search_pattern {
        platforms_query = platforms_query.filter(
            platforms::name
                .ilike(pattern.clone())
                .or(platforms::platform_id.ilike(pattern.clone()))
                .or(platforms::tagline.ilike(pattern.clone())),
        );
    }

    let platforms_result = platforms_query
        .load::<(i32, String, String, String, Option<String>, Option<String>, String, Option<String>, Option<String>, Option<serde_json::Value>, Option<serde_json::Value>, i16, Option<String>, Option<String>, NaiveDateTime, NaiveDateTime, bool, Option<NaiveDateTime>, Option<String>, Option<bool>, Option<String>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, String, Option<String>, NaiveDateTime)>(&mut conn)
        .await;

    match platforms_result {
        Ok(platforms_data) => {
            let mut platform_memberships = Vec::with_capacity(platforms_data.len());

            for (id, platform_id, name, tagline, description, logo, developer_address, terms_of_service, privacy_policy, platform_names, links, status, release_date, shutdown_date, created_at, updated_at, is_approved, approval_changed_at, approved_by, wants_dao_governance, governance_registry_id, delegate_count, delegate_term_epochs, max_votes_per_user, min_on_chain_age_days, proposal_submission_cost, quadratic_base_cost, quorum_votes, voting_period_epochs, treasury, version, primary_category, secondary_category, joined_at) in platforms_data {
                // Get moderator count
                let moderator_count = platform_moderators::table
                    .filter(platform_moderators::platform_id.eq(&platform_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .await
                    .unwrap_or(0);

                // Get blocked profiles count
                let blocked_count = platform_blocked_profiles::table
                    .filter(platform_blocked_profiles::platform_id.eq(&platform_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .await
                    .unwrap_or(0);

                // Convert platform_names from JSON to Vec<String>
                let platform_names_vec: Option<Vec<String>> = platform_names
                    .as_ref()
                    .and_then(|json| serde_json::from_value(json.clone()).ok());

                // Convert links from JSON to Vec<String>
                let links_vec: Option<Vec<String>> = links
                    .as_ref()
                    .and_then(|json| serde_json::from_value(json.clone()).ok());

                // Build platform with details
                let platform = PlatformWithDetails {
                    id,
                    platform_id: platform_id.clone(),
                    name,
                    tagline,
                    description,
                    logo,
                    developer_address,
                    terms_of_service,
                    privacy_policy,
                    platform_names: platform_names_vec,
                    links: links_vec,
                    status,
                    status_text: PlatformWithDetails::status_to_text(status),
                    release_date,
                    shutdown_date,
                    created_at,
                    updated_at,
                    is_approved,
                    approval_changed_at,
                    approved_by,
                    wants_dao_governance,
                    governance_registry_id,
                    delegate_count,
                    delegate_term_epochs,
                    max_votes_per_user,
                    min_on_chain_age_days,
                    proposal_submission_cost,
                    quadratic_base_cost,
                    quorum_votes,
                    voting_period_epochs,
                    treasury,
                    version,
                    primary_category,
                    secondary_category,
                    moderator_count,
                    blocked_profiles_count: blocked_count,
                };

                platform_memberships.push(PlatformMembership {
                    platform,
                    joined_at,
                });
            }

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "platforms": platform_memberships,
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
            error!("Failed to fetch profile platforms: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch profile platforms: {}", e)
                })),
            )
        }
    }
}

/// Get platform events for a specific platform with pagination and optional event type filtering
pub async fn get_platform_events(
    State(db_pool): State<DbPool>,
    Path(platform_id): Path<String>,
    Query(query): Query<PlatformEventsQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);

    // If page is provided, calculate the offset
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    debug!(
        "Getting platform events for platform_id: {} with limit: {}, offset: {}, event_type: {:?}",
        platform_id, limit, offset, query.event_type
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

    // Build the base query for filtering
    let mut count_query = platform_events::table
        .filter(platform_events::platform_id.eq(&platform_id))
        .into_boxed();

    let mut events_query = platform_events::table
        .filter(platform_events::platform_id.eq(&platform_id))
        .order_by(platform_events::created_at.desc())
        .into_boxed();

    // Apply event_type filter if provided
    if let Some(ref event_type) = query.event_type {
        count_query = count_query.filter(platform_events::event_type.eq(event_type));
        events_query = events_query.filter(platform_events::event_type.eq(event_type));
    }

    // Get the total count for pagination info (before applying limit/offset)
    let total_count = match count_query
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count,
        Err(e) => {
            error!("Error counting platform events: {}", e);
            0
        }
    };

    let total_pages = if total_count > 0 {
        ((total_count as f64) / (limit as f64)).ceil() as i64
    } else {
        0
    };

    // Apply pagination
    let events_result = events_query
        .limit(limit)
        .offset(offset)
        .load::<PlatformEvent>(&mut conn)
        .await;

    match events_result {
        Ok(events) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "events": events,
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
            error!("Error loading platform events: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch platform events: {}", e)
                })),
            )
        }
    }
}
