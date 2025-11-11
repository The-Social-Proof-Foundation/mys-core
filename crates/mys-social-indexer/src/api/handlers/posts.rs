// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::deserialize::QueryableByName;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::db::DbPool;
use crate::db::query_types::CountResult;
use crate::schema::profiles;

// Query parameters for post listing
#[derive(Debug, Deserialize)]
pub struct PostQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub owner: Option<String>,
    pub profile_id: Option<String>,
    pub include_deleted: Option<bool>,
    pub platform_id: Option<String>,
}

// Query parameters for promotion listing
#[derive(Debug, Deserialize)]
pub struct PromotionQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub owner: Option<String>,
    pub active_only: Option<bool>,
    pub platform_id: Option<String>,
}

// Basic post information returned from queries
#[derive(Debug, Serialize, QueryableByName)]
pub struct PostBasic {
    #[diesel(sql_type = Text)]
    pub post_id: String,

    #[diesel(sql_type = Text)]
    pub owner: String,

    #[diesel(sql_type = Nullable<Text>)]
    pub profile_id: Option<String>,

    #[diesel(sql_type = Text)]
    pub content: String,

    #[diesel(sql_type = BigInt)]
    pub created_at: i64,

    #[diesel(sql_type = Nullable<BigInt>)]
    pub deleted_at: Option<i64>,

    #[diesel(sql_type = Bool)]
    pub removed_from_platform: bool,

    #[diesel(sql_type = BigInt)]
    pub reaction_count: i64,

    #[diesel(sql_type = BigInt)]
    pub comment_count: i64,

    #[diesel(sql_type = BigInt)]
    pub repost_count: i64,

    #[diesel(sql_type = BigInt)]
    pub tips_received: i64,

    #[diesel(sql_type = Nullable<Text>)]
    pub promotion_id: Option<String>,
}

// Response for a post with engagement stats
#[derive(Debug, Serialize)]
pub struct PostResponse {
    #[serde(flatten)]
    pub post: PostBasic,
    pub engagement_score: i64,
    pub trending_score: f64,
}

// Pagination info structure
#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
    pub total_pages: i64,
}

// API response structure with pagination
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub pagination: PaginationInfo,
}

// Comment data model
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct CommentInfo {
    #[diesel(sql_type = Text)]
    pub comment_id: String,

    #[diesel(sql_type = Text)]
    pub post_id: String,

    #[diesel(sql_type = Text)]
    pub owner: String,

    #[diesel(sql_type = Nullable<Text>)]
    pub profile_id: Option<String>,

    #[diesel(sql_type = Text)]
    pub content: String,

    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
}

// Reaction data model
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct ReactionInfo {
    #[diesel(sql_type = Text)]
    pub reaction_id: String,

    #[diesel(sql_type = Text)]
    pub object_id: String,

    #[diesel(sql_type = Bool)]
    pub is_post: bool,

    #[diesel(sql_type = Text)]
    pub owner: String,

    #[diesel(sql_type = Nullable<Text>)]
    pub profile_id: Option<String>,

    #[diesel(sql_type = Int2)]
    pub reaction_type: i16,

    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
}

// Repost data model
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct RepostInfo {
    #[diesel(sql_type = Text)]
    pub repost_id: String,

    #[diesel(sql_type = Text)]
    pub original_id: String,

    #[diesel(sql_type = Bool)]
    pub is_original_post: bool,

    #[diesel(sql_type = Text)]
    pub owner: String,

    #[diesel(sql_type = Nullable<Text>)]
    pub profile_id: Option<String>,

    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
}

// Post with engagement score for trending views
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PostWithEngagementInfo {
    #[diesel(sql_type = Text)]
    pub post_id: String,

    #[diesel(sql_type = Text)]
    pub owner: String,

    #[diesel(sql_type = Nullable<Text>)]
    pub profile_id: Option<String>,

    #[diesel(sql_type = Text)]
    pub content: String,

    #[diesel(sql_type = BigInt)]
    pub created_at: i64,

    #[diesel(sql_type = Nullable<BigInt>)]
    pub deleted_at: Option<i64>,

    #[diesel(sql_type = Bool)]
    pub removed_from_platform: bool,

    #[diesel(sql_type = BigInt)]
    pub reaction_count: i64,

    #[diesel(sql_type = BigInt)]
    pub comment_count: i64,

    #[diesel(sql_type = BigInt)]
    pub repost_count: i64,

    #[diesel(sql_type = BigInt)]
    pub tips_received: i64,

    #[diesel(sql_type = BigInt)]
    pub engagement_score: i64,

    #[diesel(sql_type = Float8)]
    pub trending_score: f64,

    #[diesel(sql_type = Nullable<Text>)]
    pub promotion_id: Option<String>,
}

// Promoted post information
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PromotedPostInfo {
    #[diesel(sql_type = Text)]
    pub promotion_id: String,

    #[diesel(sql_type = Text)]
    pub post_id: String,

    #[diesel(sql_type = Text)]
    pub owner: String,

    #[diesel(sql_type = Text)]
    pub profile_id: String,

    #[diesel(sql_type = BigInt)]
    pub payment_per_view: i64,

    #[diesel(sql_type = BigInt)]
    pub total_budget: i64,

    #[diesel(sql_type = BigInt)]
    pub remaining_budget: i64,

    #[diesel(sql_type = Bool)]
    pub active: bool,

    #[diesel(sql_type = BigInt)]
    pub created_at: i64,

    #[diesel(sql_type = BigInt)]
    pub view_count: i64,

    #[diesel(sql_type = BigInt)]
    pub total_spent: i64,
}

// Promotion view information
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PromotionViewInfo {
    #[diesel(sql_type = Text)]
    pub post_id: String,

    #[diesel(sql_type = Text)]
    pub promotion_id: String,

    #[diesel(sql_type = Text)]
    pub viewer: String,

    #[diesel(sql_type = BigInt)]
    pub payment_amount: i64,

    #[diesel(sql_type = BigInt)]
    pub view_duration: i64,

    #[diesel(sql_type = Text)]
    pub platform_id: String,

    #[diesel(sql_type = BigInt)]
    pub timestamp: i64,
}

// Promotion statistics
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PromotionStats {
    #[diesel(sql_type = Text)]
    pub promotion_id: String,

    #[diesel(sql_type = BigInt)]
    pub total_views: i64,

    #[diesel(sql_type = BigInt)]
    pub unique_viewers: i64,

    #[diesel(sql_type = BigInt)]
    pub total_spent: i64,

    #[diesel(sql_type = BigInt)]
    pub avg_view_duration: i64,

    #[diesel(sql_type = Float8)]
    pub avg_payment_per_view: f64,

    #[diesel(sql_type = BigInt)]
    pub views_last_24h: i64,

    #[diesel(sql_type = BigInt)]
    pub views_last_7d: i64,
}

// Get a post by ID
pub async fn get_post_by_id(State(pool): State<DbPool>, Path(post_id): Path<String>) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    // Use diesel sql_query instead of QueryDsl since there might be schema definition issues
    let query = "SELECT post_id, owner, profile_id, content, created_at, deleted_at, removed_from_platform, reaction_count, comment_count, repost_count, tips_received, promotion_id FROM posts WHERE post_id = $1";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&post_id)
        .get_result::<PostBasic>(&mut conn)
        .await;

    match result {
        Ok(post) => Json(post).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Post not found").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

// Get comments for a post with pagination
pub async fn get_post_comments(
    State(pool): State<DbPool>,
    Path(post_id): Path<String>,
    Query(params): Query<PostQuery>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    // Use direct SQL query
    let query = "
        SELECT comment_id, post_id, owner, profile_id, content, created_at FROM comments 
        WHERE post_id = $1
        ORDER BY created_at DESC 
        LIMIT $2 OFFSET $3
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<CommentInfo>(&mut conn)
        .await;

    match result {
        Ok(comments) => Json(comments).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

// List posts with pagination and filtering
pub async fn list_posts(State(pool): State<DbPool>, Query(params): Query<PostQuery>) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    // Build the query
    let limit = params.limit.unwrap_or(20).min(100); // Max 100 posts
    let offset = params.offset.unwrap_or(0);

    // Simplified query that just returns basic post info
    let query = "
        SELECT post_id, owner, profile_id, content, created_at, deleted_at, 
               removed_from_platform, reaction_count, comment_count, repost_count, tips_received, promotion_id
        FROM posts 
        WHERE deleted_at IS NULL 
        ORDER BY created_at DESC 
        LIMIT $1 OFFSET $2
    ";

    let result = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PostBasic>(&mut conn)
        .await;

    match result {
        Ok(posts) => Json(posts).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get reactions for a specific post
pub async fn get_post_reactions(
    State(pool): State<DbPool>,
    Path(post_id): Path<String>,
    Query(params): Query<PostQuery>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    // Use a SQL query instead of the ORM to avoid type issues
    let query = "
        SELECT reaction_id, object_id, is_post, owner, profile_id, reaction_type, created_at 
        FROM reactions
        WHERE object_id = $1 AND is_post = true
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ReactionInfo>(&mut conn)
        .await;

    match result {
        Ok(reactions) => Json(reactions).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get reposts for a specific post
pub async fn get_post_reposts(
    State(pool): State<DbPool>,
    Path(post_id): Path<String>,
    Query(params): Query<PostQuery>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    // Use a SQL query instead of the ORM to avoid type issues
    let query = "
        SELECT repost_id, original_id, is_original_post, owner, profile_id, created_at
        FROM reposts
        WHERE original_id = $1 AND is_original_post = true
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<RepostInfo>(&mut conn)
        .await;

    match result {
        Ok(reposts) => Json(reposts).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get trending posts based on engagement - simplified to avoid diesel issues
pub async fn get_trending_posts(
    State(pool): State<DbPool>,
    Query(params): Query<PostQuery>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    // Simplified query - just get posts ordered by created_at
    let query = "
        SELECT post_id, owner, profile_id, content, created_at, deleted_at, 
               removed_from_platform, reaction_count, comment_count, repost_count, tips_received, promotion_id
        FROM posts 
        WHERE deleted_at IS NULL AND removed_from_platform = false
        ORDER BY (reaction_count + comment_count * 2 + repost_count * 3) DESC, created_at DESC
        LIMIT $1 OFFSET $2
    ";

    let result = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PostBasic>(&mut conn)
        .await;

    match result {
        Ok(posts) => Json(posts).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get posts by a specific profile
/// Supports profile_id, wallet address (0x...), or username
pub async fn get_profile_posts(
    State(pool): State<DbPool>,
    Path(profile_id): Path<String>,
    Query(params): Query<PostQuery>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    // Resolve profile_id, wallet address, or username to the actual profile_id used in posts table
    let (resolved_profile_id, is_wallet_address) = if profile_id.starts_with("0x") {
        // It's a wallet address - resolve to profile_id
        debug!("Resolving wallet address to profile_id: {}", profile_id);
        match profiles::table
            .filter(profiles::owner_address.eq(&profile_id))
            .select(profiles::profile_id.nullable())
            .first::<Option<String>>(&mut conn)
            .await
        {
            Ok(Some(pid)) => {
                debug!("Resolved wallet address {} to profile_id {}", profile_id, pid);
                (pid, false) // Resolved to profile_id, not a wallet address anymore
            }
            Ok(None) | Err(diesel::result::Error::NotFound) => {
                // No profile_id found, will query by owner field
                debug!("Wallet address not found in profiles, will query by owner field: {}", profile_id);
                (profile_id.clone(), true) // Keep as wallet address to query by owner
            }
            Err(e) => {
                error!("Error resolving wallet address: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": format!("Failed to resolve wallet address: {}", e)
                    })),
                )
                    .into_response();
            }
        }
    } else {
        // Try to resolve as profile_id first
        match profiles::table
            .filter(profiles::profile_id.eq(&profile_id))
            .select(profiles::profile_id.nullable())
            .first::<Option<String>>(&mut conn)
            .await
        {
            Ok(Some(pid)) => {
                debug!("Found profile_id: {}", pid);
                (pid, false)
            }
            Ok(None) => {
                // Profile found but no profile_id - use input as-is
                debug!("Profile found but no profile_id, using input as-is: {}", profile_id);
                (profile_id.clone(), false)
            }
            Err(diesel::result::Error::NotFound) => {
                // If profile_id not found, try resolving as username
                debug!("Profile ID not found, trying username: {}", profile_id);
                match profiles::table
                    .filter(profiles::username.eq(&profile_id))
                    .select(profiles::profile_id.nullable())
                    .first::<Option<String>>(&mut conn)
                    .await
                {
                    Ok(Some(pid)) => {
                        debug!("Resolved username {} to profile_id {}", profile_id, pid);
                        (pid, false)
                    }
                    Ok(None) => {
                        // Username found but no profile_id - use username as-is
                        debug!("Username found but no profile_id, using username: {}", profile_id);
                        (profile_id.clone(), false)
                    }
                    Err(diesel::result::Error::NotFound) => {
                        // Neither profile_id nor username found - use as-is (might be a legacy profile_id)
                        debug!("Profile ID and username not found, using as-is: {}", profile_id);
                        (profile_id.clone(), false)
                    }
                    Err(e) => {
                        error!("Error resolving username: {}", e);
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "error": format!("Failed to resolve username: {}", e)
                            })),
                        )
                            .into_response();
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
                )
                    .into_response();
            }
        }
    };

    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let include_deleted = params.include_deleted.unwrap_or(false);

    // Build query - use profile_id if resolved, otherwise query by owner field for wallet addresses
    let where_clause = if is_wallet_address {
        "p.owner = $1"
    } else {
        "p.profile_id = $1"
    };

    let mut query = format!(
        "
        SELECT 
            p.post_id, p.owner, p.profile_id, p.content, p.created_at, p.deleted_at, p.removed_from_platform, 
            p.reaction_count, p.comment_count, p.repost_count, p.tips_received,
            (p.reaction_count + p.comment_count * 2 + p.repost_count * 3 + p.tips_received) AS engagement_score,
            ((p.reaction_count + p.comment_count * 2 + p.repost_count * 3 + p.tips_received) / 
             (EXTRACT(EPOCH FROM NOW()) - p.created_at + 3600) * 10000) AS trending_score,
            p.promotion_id
        FROM 
            posts p
        WHERE 
            {}
        ",
        where_clause
    );

    if !include_deleted {
        query.push_str(" AND p.deleted_at IS NULL AND p.removed_from_platform = false");
    }

    // Filter by platform_id if provided
    if let Some(platform_id) = &params.platform_id {
        query.push_str(&format!(
            "
            AND EXISTS (
                SELECT 1 FROM posts_moderation_events pme 
                WHERE pme.object_id = p.post_id AND pme.platform_id = '{}'
                AND pme.removed = false
            )",
            platform_id
        ));
    }

    query.push_str(" ORDER BY p.created_at DESC LIMIT $2 OFFSET $3");

    // Build count query with same WHERE conditions
    let mut count_query = format!(
        "
        SELECT COUNT(*) as count
        FROM posts p
        WHERE {}
        ",
        where_clause
    );

    if !include_deleted {
        count_query.push_str(" AND p.deleted_at IS NULL AND p.removed_from_platform = false");
    }

    // Filter by platform_id if provided
    if let Some(platform_id) = &params.platform_id {
        count_query.push_str(&format!(
            "
            AND EXISTS (
                SELECT 1 FROM posts_moderation_events pme 
                WHERE pme.object_id = p.post_id AND pme.platform_id = '{}'
                AND pme.removed = false
            )",
            platform_id
        ));
    }

    // Get total count for pagination
    let count_result = diesel::sql_query(&count_query)
        .bind::<Text, _>(&resolved_profile_id)
        .get_result::<CountResult>(&mut conn)
        .await;

    let total = match count_result {
        Ok(result) => result.count,
        Err(e) => {
            error!("Error getting post count: {}", e);
            // If count fails, still return posts but with total = 0
            0
        }
    };

    let posts_result = diesel::sql_query(&query)
        .bind::<Text, _>(resolved_profile_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PostWithEngagementInfo>(&mut conn)
        .await;

    match posts_result {
        Ok(posts_with_engagement) => {
            // Convert the posts to PostResponse
            let post_responses: Vec<PostResponse> = posts_with_engagement
                .into_iter()
                .map(|p| PostResponse {
                    post: PostBasic {
                        post_id: p.post_id,
                        owner: p.owner,
                        profile_id: p.profile_id,
                        content: p.content,
                        created_at: p.created_at,
                        deleted_at: p.deleted_at,
                        removed_from_platform: p.removed_from_platform,
                        reaction_count: p.reaction_count,
                        comment_count: p.comment_count,
                        repost_count: p.repost_count,
                        tips_received: p.tips_received,
                        promotion_id: p.promotion_id,
                    },
                    engagement_score: p.engagement_score,
                    trending_score: p.trending_score,
                })
                .collect();

            // Calculate total pages
            let total_pages = if total == 0 {
                0
            } else {
                (total + limit - 1) / limit
            };

            // Return response with pagination metadata
            Json(ApiResponse {
                data: post_responses,
                pagination: PaginationInfo {
                    limit,
                    offset,
                    total,
                    total_pages,
                },
            })
            .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get promoted posts with optional filtering
pub async fn get_promoted_posts(
    State(pool): State<DbPool>,
    Query(params): Query<PromotionQuery>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let active_only = params.active_only.unwrap_or(true);

    let mut query = "
        SELECT DISTINCT
            pp.promotion_id,
            pp.post_id,
            pp.owner,
            pp.profile_id,
            pp.payment_per_view,
            pp.total_budget,
            pp.remaining_budget,
            pp.active,
            pp.created_at,
            COUNT(DISTINCT pv.viewer) AS view_count,
            COALESCE(SUM(pv.payment_amount), 0) AS total_spent
        FROM promoted_posts pp
        LEFT JOIN promotion_views pv ON pp.promotion_id = pv.promotion_id
        WHERE 1=1
    "
    .to_string();

    if active_only {
        query.push_str(" AND pp.active = true AND pp.remaining_budget > 0");
    }

    if let Some(owner) = &params.owner {
        query.push_str(&format!(" AND pp.owner = '{}'", owner));
    }

    if let Some(platform_id) = &params.platform_id {
        query.push_str(&format!(
            " AND EXISTS (
            SELECT 1 FROM promotion_views pv2 
            WHERE pv2.promotion_id = pp.promotion_id 
            AND pv2.platform_id = '{}'
        )",
            platform_id
        ));
    }

    query.push_str(
        "
        GROUP BY pp.promotion_id, pp.post_id, pp.owner, pp.profile_id, 
                 pp.payment_per_view, pp.total_budget, pp.remaining_budget, 
                 pp.active, pp.created_at
        ORDER BY pp.created_at DESC
        LIMIT $1 OFFSET $2
    ",
    );

    let result = diesel::sql_query(&query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PromotedPostInfo>(&mut conn)
        .await;

    match result {
        Ok(promoted_posts) => Json(promoted_posts).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get promotion details for a specific post
pub async fn get_post_promotion(
    State(pool): State<DbPool>,
    Path(post_id): Path<String>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    let query = "
        SELECT 
            pp.promotion_id,
            pp.post_id,
            pp.owner,
            pp.profile_id,
            pp.payment_per_view,
            pp.total_budget,
            pp.remaining_budget,
            pp.active,
            pp.created_at,
            COUNT(DISTINCT pv.viewer) AS view_count,
            COALESCE(SUM(pv.payment_amount), 0) AS total_spent
        FROM promoted_posts pp
        LEFT JOIN promotion_views pv ON pp.promotion_id = pv.promotion_id
        WHERE pp.post_id = $1
        GROUP BY pp.promotion_id, pp.post_id, pp.owner, pp.profile_id, 
                 pp.payment_per_view, pp.total_budget, pp.remaining_budget, 
                 pp.active, pp.created_at
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&post_id)
        .get_result::<PromotedPostInfo>(&mut conn)
        .await;

    match result {
        Ok(promotion) => Json(promotion).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "No promotion found for this post").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get promotion views for a specific promotion
pub async fn get_promotion_views(
    State(pool): State<DbPool>,
    Path(promotion_id): Path<String>,
    Query(params): Query<PostQuery>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    let query = "
        SELECT 
            post_id,
            promotion_id,
            viewer,
            payment_amount,
            view_duration,
            platform_id,
            timestamp
        FROM promotion_views
        WHERE promotion_id = $1
        ORDER BY timestamp DESC
        LIMIT $2 OFFSET $3
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&promotion_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PromotionViewInfo>(&mut conn)
        .await;

    match result {
        Ok(views) => Json(views).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

// Time bucket analytics for promotions
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PromotionTimeBucket {
    #[diesel(sql_type = Timestamptz)]
    pub bucket: chrono::DateTime<chrono::Utc>,

    #[diesel(sql_type = BigInt)]
    pub view_count: i64,

    #[diesel(sql_type = BigInt)]
    pub total_payments: i64,

    #[diesel(sql_type = Float8)]
    pub avg_view_duration: f64,
}

// Performance metrics from materialized views
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PromotionPerformance {
    #[diesel(sql_type = Text)]
    pub promotion_id: String,

    #[diesel(sql_type = Text)]
    pub post_id: String,

    #[diesel(sql_type = BigInt)]
    pub total_views: i64,

    #[diesel(sql_type = BigInt)]
    pub unique_viewers: i64,

    #[diesel(sql_type = Float8)]
    pub views_per_hour: f64,

    #[diesel(sql_type = Float8)]
    pub budget_utilization_percent: f64,

    #[diesel(sql_type = Float8)]
    pub actual_cost_per_view: f64,
}

/// Get promotion statistics
pub async fn get_promotion_stats(
    State(pool): State<DbPool>,
    Path(promotion_id): Path<String>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    // Enhanced query using TimescaleDB time_bucket for better time-based analytics
    let query = "
        WITH current_stats AS (
            SELECT 
                COUNT(*) AS total_views,
                COUNT(DISTINCT viewer) AS unique_viewers,
                COALESCE(SUM(payment_amount), 0) AS total_spent,
                COALESCE(AVG(view_duration), 0) AS avg_view_duration,
                COALESCE(AVG(payment_amount), 0.0) AS avg_payment_per_view
            FROM promotion_views
            WHERE promotion_id = $1
        ),
        time_based_stats AS (
            SELECT 
                COUNT(CASE WHEN time >= NOW() - INTERVAL '24 hours' THEN 1 END) AS views_last_24h,
                COUNT(CASE WHEN time >= NOW() - INTERVAL '7 days' THEN 1 END) AS views_last_7d
            FROM promotion_views
            WHERE promotion_id = $1
        )
        SELECT 
            $1::text AS promotion_id,
            cs.total_views,
            cs.unique_viewers,
            cs.total_spent,
            cs.avg_view_duration,
            cs.avg_payment_per_view,
            ts.views_last_24h,
            ts.views_last_7d
        FROM current_stats cs, time_based_stats ts
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&promotion_id)
        .get_result::<PromotionStats>(&mut conn)
        .await;

    match result {
        Ok(stats) => Json(stats).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get promotion view analytics over time using TimescaleDB time_bucket
pub async fn get_promotion_time_analytics(
    State(pool): State<DbPool>,
    Path(promotion_id): Path<String>,
    Query(params): Query<PostQuery>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    // Use time_bucket for efficient time-series aggregation
    let query = "
        SELECT 
            time_bucket('1 hour', time) AS bucket,
            COUNT(*) AS view_count,
            SUM(payment_amount) AS total_payments,
            AVG(view_duration)::FLOAT8 AS avg_view_duration
        FROM promotion_views
        WHERE promotion_id = $1
        AND time >= NOW() - INTERVAL '7 days'
        GROUP BY bucket
        ORDER BY bucket DESC
        LIMIT $2
    ";

    let limit = params.limit.unwrap_or(168).min(168); // Max 7 days of hourly data

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&promotion_id)
        .bind::<BigInt, _>(limit)
        .load::<PromotionTimeBucket>(&mut conn)
        .await;

    match result {
        Ok(buckets) => Json(buckets).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get top performing promotions using materialized views
pub async fn get_top_performing_promotions(
    State(pool): State<DbPool>,
    Query(params): Query<PostQuery>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    let limit = params.limit.unwrap_or(20).min(100);

    // Query the pre-computed view for better performance
    let query = "
        SELECT 
            promotion_id,
            post_id,
            total_views,
            unique_viewers,
            views_per_hour,
            budget_utilization_percent,
            actual_cost_per_view
        FROM promotion_performance
        WHERE budget_utilization_percent < 100
        ORDER BY views_per_hour DESC
        LIMIT $1
    ";

    let result = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .load::<PromotionPerformance>(&mut conn)
        .await;

    match result {
        Ok(promotions) => Json(promotions).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get promotion views from continuous aggregate for better performance
pub async fn get_promotion_hourly_stats(
    State(pool): State<DbPool>,
    Path(promotion_id): Path<String>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    // Query the continuous aggregate instead of raw data
    let query = "
        SELECT 
            bucket,
            view_count,
            total_payments,
            avg_view_duration::FLOAT8 AS avg_view_duration
        FROM promotion_views_hourly
        WHERE promotion_id = $1
        AND bucket >= NOW() - INTERVAL '7 days'
        ORDER BY bucket DESC
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&promotion_id)
        .load::<PromotionTimeBucket>(&mut conn)
        .await;

    match result {
        Ok(stats) => Json(stats).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get overall promotion spending trends from continuous aggregate
pub async fn get_promotion_spending_trends(
    State(pool): State<DbPool>,
    Query(params): Query<PostQuery>,
) -> Response {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", e),
            )
                .into_response();
        }
    };

    // Query the daily spending continuous aggregate
    let query = "
        SELECT 
            bucket AS bucket,
            total_views AS view_count,
            total_spending AS total_payments,
            COALESCE(avg_payment_per_view, 0)::FLOAT8 AS avg_view_duration
        FROM promotion_spending_daily
        WHERE bucket >= NOW() - INTERVAL '30 days'
        ORDER BY bucket DESC
        LIMIT $1
    ";

    let limit = params.limit.unwrap_or(30).min(90);

    let result = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .load::<PromotionTimeBucket>(&mut conn)
        .await;

    match result {
        Ok(trends) => Json(trends).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}
