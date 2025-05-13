// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::sql_types::*;
use diesel::pg::Pg;
use diesel::deserialize::{QueryableByName};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;

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
}

// Response for a post with engagement stats
#[derive(Debug, Serialize)]
pub struct PostResponse {
    #[serde(flatten)]
    pub post: PostBasic,
    pub engagement_score: i64,
    pub trending_score: f64,
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
}

// Get a post by ID
pub async fn get_post_by_id(
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
    
    // Use diesel sql_query instead of QueryDsl since there might be schema definition issues
    let query = "SELECT post_id, owner, profile_id, content, created_at, deleted_at, removed_from_platform, reaction_count, comment_count, repost_count, tips_received FROM posts WHERE post_id = $1";
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&post_id)
        .get_result::<PostBasic>(&mut conn)
        .await;
    
    match result {
        Ok(post) => Json(post).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Post not found").into_response()
        },
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
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
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

// List posts with pagination and filtering
pub async fn list_posts(
    State(pool): State<DbPool>, 
    Query(params): Query<PostQuery>
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

    // Build the query
    let limit = params.limit.unwrap_or(20).min(100); // Max 100 posts
    let offset = params.offset.unwrap_or(0);
    
    // Simplified query that just returns basic post info
    let query = "
        SELECT post_id, owner, profile_id, content, created_at, deleted_at, 
               removed_from_platform, reaction_count, comment_count, repost_count, tips_received
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
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
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
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
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
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
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
               removed_from_platform, reaction_count, comment_count, repost_count, tips_received
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
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

/// Get posts by a specific profile
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

    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let include_deleted = params.include_deleted.unwrap_or(false);
    
    let mut query = "
        SELECT 
            p.post_id, p.owner, p.profile_id, p.content, p.created_at, p.deleted_at, p.removed_from_platform, 
            p.reaction_count, p.comment_count, p.repost_count, p.tips_received,
            (p.reaction_count + p.comment_count * 2 + p.repost_count * 3 + p.tips_received) AS engagement_score,
            ((p.reaction_count + p.comment_count * 2 + p.repost_count * 3 + p.tips_received) / 
             (EXTRACT(EPOCH FROM NOW()) - p.created_at + 3600) * 10000) AS trending_score
        FROM 
            posts p
        WHERE 
            p.profile_id = $1
    ".to_string();
    
    if !include_deleted {
        query.push_str(" AND p.deleted_at IS NULL AND p.removed_from_platform = false");
    }
    
    // Filter by platform_id if provided
    if let Some(platform_id) = &params.platform_id {
        query.push_str(&format!("
            AND EXISTS (
                SELECT 1 FROM posts_moderation_events pme 
                WHERE pme.object_id = p.post_id AND pme.platform_id = '{}'
                AND pme.removed = false
            )", platform_id));
    }
    
    query.push_str(" ORDER BY p.created_at DESC LIMIT $2 OFFSET $3");
    
    let posts_result = diesel::sql_query(&query)
        .bind::<Text, _>(profile_id)
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
                    },
                    engagement_score: p.engagement_score,
                    trending_score: p.trending_score,
                })
                .collect();
                
            Json(post_responses).into_response()
        },
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
} 