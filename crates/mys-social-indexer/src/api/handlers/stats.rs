// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Serialize;
use tracing::{debug, error};

use crate::db::DbPool;
use crate::schema;

/// Response type for overall system statistics
#[derive(Debug, Serialize)]
pub struct SystemStatsResponse {
    /// Total number of social proof tokens
    pub social_proof_tokens: i64,

    /// Total number of posts
    pub total_posts: i64,

    /// Total number of comments
    pub total_comments: i64,

    /// Total number of reactions
    pub total_reactions: i64,

    /// Total number of social graph relationships (follows)
    pub total_social_relationships: i64,
}

/// Handler for getting overall system statistics
/// GET /stats/system
pub async fn get_system_stats(
    State(pool): State<DbPool>,
) -> Result<Json<SystemStatsResponse>, StatusCode> {
    debug!("Getting overall system statistics");

    let mut conn = pool.get().await.map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Count social proof tokens
    let social_proof_tokens = schema::spt_pools::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to count social proof tokens: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Count total posts
    let total_posts = schema::posts::table
        .filter(schema::posts::deleted_at.is_null()) // Only count non-deleted posts
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to count posts: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Count total comments
    let total_comments = schema::comments::table
        .filter(schema::comments::deleted_at.is_null()) // Only count non-deleted comments
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to count comments: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Count total reactions
    let total_reactions = schema::reactions::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to count reactions: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Count total social graph relationships
    let total_social_relationships = schema::social_graph_relationships::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to count social graph relationships: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let stats = SystemStatsResponse {
        social_proof_tokens,
        total_posts,
        total_comments,
        total_reactions,
        total_social_relationships,
    };

    debug!(
        "System stats: SPT={}, Posts={}, Comments={}, Reactions={}, Relationships={}",
        stats.social_proof_tokens,
        stats.total_posts,
        stats.total_comments,
        stats.total_reactions,
        stats.total_social_relationships
    );

    Ok(Json(stats))
}
