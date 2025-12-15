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
use diesel::sql_types::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;

// Query parameters for PoC endpoints
#[derive(Debug, Deserialize)]
pub struct PocQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub post_id: Option<String>,
    pub oracle_id: Option<String>,
    pub media_type: Option<String>,
    pub status: Option<String>,
    pub from_time: Option<i64>,
    pub to_time: Option<i64>,
}

// PoC Badge response structure
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PocBadgeInfo {
    #[diesel(sql_type = Text)]
    pub badge_id: String,

    #[diesel(sql_type = Text)]
    pub post_id: String,

    #[diesel(sql_type = Text)]
    pub oracle_id: String,

    #[diesel(sql_type = Text)]
    pub media_type: String,

    #[diesel(sql_type = Float8)]
    pub authenticity_score: f64,

    #[diesel(sql_type = Float8)]
    pub creativity_score: f64,

    #[diesel(sql_type = Bool)]
    pub revoked: bool,

    #[diesel(sql_type = BigInt)]
    pub issued_at: i64,

    #[diesel(sql_type = Nullable<BigInt>)]
    pub revoked_at: Option<i64>,
}

// Revenue Redirection response structure
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct RevenueRedirectionInfo {
    #[diesel(sql_type = Text)]
    pub redirection_id: String,

    #[diesel(sql_type = Text)]
    pub accused_post_id: String,

    #[diesel(sql_type = Text)]
    pub original_post_id: String,

    #[diesel(sql_type = BigInt)]
    pub redirect_percentage: i64,

    #[diesel(sql_type = Bool)]
    pub removed: bool,

    #[diesel(sql_type = BigInt)]
    pub activated_at: i64,

    #[diesel(sql_type = Nullable<BigInt>)]
    pub removed_at: Option<i64>,
}

// PoC Analysis Result response structure
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PocAnalysisInfo {
    #[diesel(sql_type = Text)]
    pub analysis_id: String,

    #[diesel(sql_type = Text)]
    pub post_id: String,

    #[diesel(sql_type = Text)]
    pub oracle_id: String,

    #[diesel(sql_type = Text)]
    pub media_type: String,

    #[diesel(sql_type = Float8)]
    pub authenticity_score: f64,

    #[diesel(sql_type = Float8)]
    pub creativity_score: f64,

    #[diesel(sql_type = BigInt)]
    pub analyzed_at: i64,
}

// PoC Dispute response structure
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PocDisputeInfo {
    #[diesel(sql_type = Text)]
    pub dispute_id: String,

    #[diesel(sql_type = Text)]
    pub post_id: String,

    #[diesel(sql_type = Text)]
    pub challenger: String,

    #[diesel(sql_type = BigInt)]
    pub stake_amount: i64,

    #[diesel(sql_type = Text)]
    pub status: String,

    #[diesel(sql_type = Nullable<Text>)]
    pub resolution: Option<String>,

    #[diesel(sql_type = BigInt)]
    pub submitted_at: i64,

    #[diesel(sql_type = Nullable<BigInt>)]
    pub resolved_at: Option<i64>,
}

// Dispute vote response structure
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct DisputeVoteInfo {
    #[diesel(sql_type = Text)]
    pub dispute_id: String,

    #[diesel(sql_type = Text)]
    pub voter: String,

    #[diesel(sql_type = BigInt)]
    pub stake_amount: i64,

    #[diesel(sql_type = Text)]
    pub vote_choice: String,

    #[diesel(sql_type = Bool)]
    pub reward_claimed: bool,

    #[diesel(sql_type = BigInt)]
    pub voted_at: i64,
}

// PoC Analytics response structure
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PocAnalytics {
    #[diesel(sql_type = BigInt)]
    pub total_badges: i64,

    #[diesel(sql_type = BigInt)]
    pub active_badges: i64,

    #[diesel(sql_type = BigInt)]
    pub revoked_badges: i64,

    #[diesel(sql_type = BigInt)]
    pub total_disputes: i64,

    #[diesel(sql_type = BigInt)]
    pub resolved_disputes: i64,

    #[diesel(sql_type = Float8)]
    pub avg_authenticity_score: f64,

    #[diesel(sql_type = Float8)]
    pub avg_creativity_score: f64,
}

// PoC Configuration response structure
#[derive(Debug, Clone, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct PocConfigInfo {
    #[diesel(sql_type = BigInt)]
    pub image_threshold: i64,

    #[diesel(sql_type = BigInt)]
    pub video_threshold: i64,

    #[diesel(sql_type = BigInt)]
    pub audio_threshold: i64,

    #[diesel(sql_type = BigInt)]
    pub revenue_redirect_percentage: i64,

    #[diesel(sql_type = BigInt)]
    pub dispute_cost: i64,

    #[diesel(sql_type = BigInt)]
    pub dispute_protocol_fee: i64,

    #[diesel(sql_type = BigInt)]
    pub min_vote_stake: i64,

    #[diesel(sql_type = BigInt)]
    pub max_vote_stake: i64,

    #[diesel(sql_type = BigInt)]
    pub voting_duration_epochs: i64,

    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
}

/// Get PoC badges with filtering and pagination
pub async fn get_poc_badges(
    State(pool): State<DbPool>,
    Query(params): Query<PocQuery>,
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

    // Build simple query with optional filters
    let query = if let Some(_post_id) = &params.post_id {
        "
            SELECT badge_id, post_id, oracle_id, media_type, authenticity_score, 
                   creativity_score, revoked, issued_at, revoked_at
            FROM poc_badges
            WHERE post_id = $1
            ORDER BY issued_at DESC
            LIMIT $2 OFFSET $3
        "
    } else if let Some(_oracle_id) = &params.oracle_id {
        "
            SELECT badge_id, post_id, oracle_id, media_type, authenticity_score, 
                   creativity_score, revoked, issued_at, revoked_at
            FROM poc_badges
            WHERE oracle_id = $1
            ORDER BY issued_at DESC
            LIMIT $2 OFFSET $3
        "
    } else if let Some(_media_type) = &params.media_type {
        "
            SELECT badge_id, post_id, oracle_id, media_type, authenticity_score, 
                   creativity_score, revoked, issued_at, revoked_at
            FROM poc_badges
            WHERE media_type = $1
            ORDER BY issued_at DESC
            LIMIT $2 OFFSET $3
        "
    } else {
        "
            SELECT badge_id, post_id, oracle_id, media_type, authenticity_score, 
                   creativity_score, revoked, issued_at, revoked_at
            FROM poc_badges
            ORDER BY issued_at DESC
            LIMIT $1 OFFSET $2
        "
    };

    let result = if let Some(_post_id) = &params.post_id {
        diesel::sql_query(query)
            .bind::<Text, _>(_post_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<PocBadgeInfo>(&mut conn)
            .await
    } else if let Some(_oracle_id) = &params.oracle_id {
        diesel::sql_query(query)
            .bind::<Text, _>(_oracle_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<PocBadgeInfo>(&mut conn)
            .await
    } else if let Some(_media_type) = &params.media_type {
        diesel::sql_query(query)
            .bind::<Text, _>(_media_type)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<PocBadgeInfo>(&mut conn)
            .await
    } else {
        diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<PocBadgeInfo>(&mut conn)
            .await
    };

    match result {
        Ok(badges) => Json(badges).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get PoC badge by ID
pub async fn get_poc_badge_by_id(
    State(pool): State<DbPool>,
    Path(badge_id): Path<String>,
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
        SELECT badge_id, post_id, oracle_id, media_type, authenticity_score, 
               creativity_score, revoked, issued_at, revoked_at
        FROM poc_badges 
        WHERE badge_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&badge_id)
        .get_result::<PocBadgeInfo>(&mut conn)
        .await;

    match result {
        Ok(badge) => Json(badge).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "PoC badge not found").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get revenue redirections with filtering and pagination
pub async fn get_revenue_redirections(
    State(pool): State<DbPool>,
    Query(params): Query<PocQuery>,
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
        SELECT redirection_id, accused_post_id, original_post_id, redirect_percentage, 
               removed, activated_at, removed_at
        FROM poc_revenue_redirections
        ORDER BY activated_at DESC
        LIMIT $1 OFFSET $2
    ";

    let result = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<RevenueRedirectionInfo>(&mut conn)
        .await;

    match result {
        Ok(redirections) => Json(redirections).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get PoC analysis results with filtering and pagination
pub async fn get_poc_analysis_results(
    State(pool): State<DbPool>,
    Query(params): Query<PocQuery>,
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
        SELECT analysis_id, post_id, oracle_id, media_type, authenticity_score, 
               creativity_score, analyzed_at
        FROM poc_analysis_results
        ORDER BY analyzed_at DESC
        LIMIT $1 OFFSET $2
    ";

    let result = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocAnalysisInfo>(&mut conn)
        .await;

    match result {
        Ok(analyses) => Json(analyses).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get PoC disputes with filtering and pagination
pub async fn get_poc_disputes(
    State(pool): State<DbPool>,
    Query(params): Query<PocQuery>,
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
        SELECT dispute_id, post_id, challenger, stake_amount, status, 
               resolution, submitted_at, resolved_at
        FROM poc_disputes
        ORDER BY submitted_at DESC
        LIMIT $1 OFFSET $2
    ";

    let result = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocDisputeInfo>(&mut conn)
        .await;

    match result {
        Ok(disputes) => Json(disputes).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get dispute by ID
pub async fn get_poc_dispute_by_id(
    State(pool): State<DbPool>,
    Path(dispute_id): Path<String>,
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
        SELECT dispute_id, post_id, challenger, stake_amount, status, 
               resolution, submitted_at, resolved_at
        FROM poc_disputes 
        WHERE dispute_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&dispute_id)
        .get_result::<PocDisputeInfo>(&mut conn)
        .await;

    match result {
        Ok(dispute) => Json(dispute).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "PoC dispute not found").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get votes for a specific dispute
pub async fn get_dispute_votes(
    State(pool): State<DbPool>,
    Path(dispute_id): Path<String>,
    Query(params): Query<PocQuery>,
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
        SELECT dispute_id, voter, stake_amount, vote_choice, reward_claimed, voted_at
        FROM poc_dispute_votes
        WHERE dispute_id = $1
        ORDER BY voted_at DESC
        LIMIT $2 OFFSET $3
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&dispute_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<DisputeVoteInfo>(&mut conn)
        .await;

    match result {
        Ok(votes) => Json(votes).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get PoC analytics and statistics
pub async fn get_poc_analytics(State(pool): State<DbPool>) -> Response {
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
            (SELECT COUNT(*) FROM poc_badges) as total_badges,
            (SELECT COUNT(*) FROM poc_badges WHERE revoked = false) as active_badges,
            (SELECT COUNT(*) FROM poc_badges WHERE revoked = true) as revoked_badges,
            (SELECT COUNT(*) FROM poc_disputes) as total_disputes,
            (SELECT COUNT(*) FROM poc_disputes WHERE status IN ('RESOLVED_UPHELD', 'RESOLVED_OVERTURNED')) as resolved_disputes,
            (SELECT COALESCE(AVG(authenticity_score), 0.0) FROM poc_badges) as avg_authenticity_score,
            (SELECT COALESCE(AVG(creativity_score), 0.0) FROM poc_badges) as avg_creativity_score
    ";

    let result = diesel::sql_query(query)
        .get_result::<PocAnalytics>(&mut conn)
        .await;

    match result {
        Ok(analytics) => Json(analytics).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get current PoC configuration
pub async fn get_poc_configuration(State(pool): State<DbPool>) -> Response {
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
        SELECT image_threshold, video_threshold, audio_threshold, revenue_redirect_percentage,
               dispute_cost, dispute_protocol_fee, min_vote_stake, max_vote_stake,
               voting_duration_epochs, updated_at
        FROM poc_configuration
        ORDER BY updated_at DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .load::<PocConfigInfo>(&mut conn)
        .await;

    match result {
        Ok(configs) => {
            if !configs.is_empty() {
                Json(&configs[0]).into_response()
            } else {
                // Return default configuration if none exists
                let default_config = PocConfigInfo {
                    image_threshold: 0,
                    video_threshold: 0,
                    audio_threshold: 0,
                    revenue_redirect_percentage: 0,
                    dispute_cost: 0,
                    dispute_protocol_fee: 0,
                    min_vote_stake: 0,
                    max_vote_stake: 0,
                    voting_duration_epochs: 0,
                    updated_at: 0,
                };
                Json(default_config).into_response()
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get PoC badges for a specific post
pub async fn get_post_poc_badges(
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
        SELECT badge_id, post_id, oracle_id, media_type, authenticity_score, 
               creativity_score, revoked, issued_at, revoked_at
        FROM poc_badges 
        WHERE post_id = $1
        ORDER BY issued_at DESC
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&post_id)
        .load::<PocBadgeInfo>(&mut conn)
        .await;

    match result {
        Ok(badges) => Json(badges).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

/// Get revenue redirections for a specific post
pub async fn get_post_revenue_redirections(
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
        SELECT redirection_id, accused_post_id, original_post_id, redirect_percentage, 
               removed, activated_at, removed_at
        FROM poc_revenue_redirections 
        WHERE accused_post_id = $1 OR original_post_id = $1
        ORDER BY activated_at DESC
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(&post_id)
        .bind::<Text, _>(&post_id)
        .load::<RevenueRedirectionInfo>(&mut conn)
        .await;

    match result {
        Ok(redirections) => Json(redirections).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}
