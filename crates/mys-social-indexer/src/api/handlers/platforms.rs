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
    Platform, PlatformWithDetails,
};
use crate::schema::{platform_blocked_profiles, platform_memberships, platform_moderators, platforms, profiles};
use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct PlatformQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    pub search: Option<String>,
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

    // Get the total count for pagination info
    let total_count = match platforms::table.count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    // Query platforms with pagination
    let platforms_result = platforms::table
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

    // Get the total count for pagination info (only approved platforms)
    let total_count = match platforms::table
        .filter(platforms::is_approved.eq(true))
        .count()
        .get_result::<i64>(&mut conn)
        .await
    {
        Ok(count) => count,
        Err(_) => 0,
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    // Query platforms with pagination, filtered by approval status
    let platforms_result = platforms::table
        .filter(platforms::is_approved.eq(true))
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
                    "profiles.owner_address = platform_memberships.profile_id OR profiles.username = platform_memberships.profile_id OR profiles.profile_id = platform_memberships.profile_id",
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
                    "profiles.owner_address = platform_memberships.profile_id OR profiles.username = platform_memberships.profile_id OR profiles.profile_id = platform_memberships.profile_id",
                )
            ),
        )
        .select((
            platform_memberships::profile_id,
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
                .map(|(profile_id, wallet_address, username, display_name, profile_photo, joined_at)| {
                    PlatformMember {
                        profile_id: profile_id.clone(),
                        wallet_address: wallet_address.unwrap_or_else(|| profile_id.clone()),
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
pub async fn check_platform_membership(
    Path((platform_id, profile_id)): Path<(String, String)>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    debug!(
        "Checking if profile {} is a member of platform {}",
        profile_id, platform_id
    );

    // Input validation
    if platform_id.trim().is_empty() || profile_id.trim().is_empty() {
        debug!("Invalid IDs: empty string");
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Platform ID and profile ID are required"
            })),
        );
    }

    // Basic length validation to prevent potential attacks
    if platform_id.len() > 256 || profile_id.len() > 256 {
        debug!("Invalid IDs: too long");
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Platform ID and profile ID must be 256 characters or less"
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

    // Resolve profile_id to wallet address if needed
    // profile_id in platform_memberships is the wallet address (owner_address)
    // Supports: wallet address (0x...), profile_id, or username
    let profile_address = if profile_id.starts_with("0x") {
        profile_id.clone()
    } else {
        // Try to resolve as profile_id first
        match profiles::table
            .filter(profiles::profile_id.eq(&profile_id))
            .select(profiles::owner_address)
            .first::<String>(&mut conn)
            .await
        {
            Ok(addr) => {
                debug!("Resolved profile_id {} to wallet address {}", profile_id, addr);
                addr
            }
            Err(diesel::result::Error::NotFound) => {
                // If profile_id not found, try resolving as username
                match profiles::table
                    .filter(profiles::username.eq(&profile_id))
                    .select(profiles::owner_address)
                    .first::<String>(&mut conn)
                    .await
                {
                    Ok(addr) => {
                        debug!("Resolved username {} to wallet address {}", profile_id, addr);
                        addr
                    }
                    Err(diesel::result::Error::NotFound) => {
                        // If neither found, return error
                        debug!("Profile ID or username not found: {}", profile_id);
                        return (
                            StatusCode::NOT_FOUND,
                            Json(serde_json::json!({
                                "error": format!("Profile not found: {}", profile_id)
                            })),
                        );
                    }
                    Err(e) => {
                        error!("Error resolving username: {}", e);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "error": format!("Failed to resolve username: {}", e)
                            })),
                        );
                    }
                }
            }
            Err(e) => {
                error!("Error resolving profile_id: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Failed to resolve profile ID: {}", e)
                    })),
                );
            }
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
        .filter(platform_memberships::profile_id.eq(&profile_address))
        .select(platform_memberships::joined_at)
        .first::<chrono::NaiveDateTime>(&mut conn)
        .await;

    match membership_result {
        Ok(joined_at) => {
            debug!(
                "Found membership: profile {} is a member of platform {}",
                profile_address, platform_id
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
                "No membership found: profile {} is not a member of platform {}",
                profile_address, platform_id
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
pub async fn get_profile_platforms(
    Path(profile_id): Path<String>,
    Query(query): Query<PlatformQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    debug!("Getting platforms for profile: {}", profile_id);

    // Input validation
    if profile_id.trim().is_empty() {
        debug!("Invalid profile_id: empty string");
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Profile ID is required"
            })),
        );
    }

    // Basic length validation to prevent potential attacks
    if profile_id.len() > 256 {
        debug!("Invalid profile_id: too long");
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Profile ID must be 256 characters or less"
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

    // Resolve profile_id to wallet address if needed
    // profile_id in platform_memberships is the wallet address (owner_address)
    // Supports: wallet address (if starts with "0x"), otherwise treats as profile_id
    let profile_address = if profile_id.starts_with("0x") {
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
            Ok(addr) => {
                debug!("Resolved profile_id {} to wallet address {}", profile_id, addr);
                addr
            }
            Err(diesel::result::Error::NotFound) => {
                // If profile_id not found, assume it's a wallet address (for legacy data)
                debug!("Profile ID not found, treating as wallet address: {}", profile_id);
                profile_id.clone()
            }
            Err(e) => {
                error!("Error resolving profile_id: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Failed to resolve profile ID: {}", e)
                    })),
                );
            }
        }
    };

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
        .filter(platform_memberships::profile_id.eq(&profile_address))
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
        .filter(platform_memberships::profile_id.eq(&profile_address))
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
        .load::<(i32, String, String, String, Option<String>, Option<String>, String, Option<String>, Option<String>, Option<serde_json::Value>, Option<serde_json::Value>, i16, Option<String>, Option<String>, NaiveDateTime, NaiveDateTime, bool, Option<NaiveDateTime>, Option<String>, Option<bool>, Option<String>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<i64>, NaiveDateTime)>(&mut conn)
        .await;

    match platforms_result {
        Ok(platforms_data) => {
            let mut platform_memberships = Vec::with_capacity(platforms_data.len());

            for (id, platform_id, name, tagline, description, logo, developer_address, terms_of_service, privacy_policy, platform_names, links, status, release_date, shutdown_date, created_at, updated_at, is_approved, approval_changed_at, approved_by, wants_dao_governance, governance_registry_id, delegate_count, delegate_term_epochs, max_votes_per_user, min_on_chain_age_days, proposal_submission_cost, quadratic_base_cost, quorum_votes, voting_period_epochs, treasury, version, joined_at) in platforms_data {
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
