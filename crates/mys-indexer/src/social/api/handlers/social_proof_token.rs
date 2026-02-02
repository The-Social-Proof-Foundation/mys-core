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
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::error;

use crate::social::db::Database;
use crate::social::models::social_proof_token::{
    PopularTokenPool, SocialProofPriceAggregation, SocialProofTokenHolding,
    SocialProofTokenPoolWithDisplay, SocialProofTokenTransaction,
    SptReservation, SptReservationPoolWithDisplay, UserTokenHolding,
};

// Shared query parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl PaginationParams {
    fn get_page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(20).clamp(1, 100)
    }

    fn get_offset(&self) -> i64 {
        (self.get_page() - 1) * self.get_limit()
    }
}

// Time range parameters for price history
#[derive(Debug, Deserialize)]
pub struct TimeRangeParams {
    pub from: Option<i64>,        // Unix timestamp in seconds
    pub to: Option<i64>,          // Unix timestamp in seconds
    pub interval: Option<String>, // "hour", "day", "week", "month"
}

// Custom filter params for token pools
#[derive(Debug, Deserialize)]
pub struct TokenPoolFilterParams {
    pub token_type: Option<i16>,
    pub owner: Option<String>,
    pub sort_by: Option<String>,  // "created", "supply", "price"
    pub sort_dir: Option<String>, // "asc", "desc"
}

// Time period parameter for analytics
#[derive(Debug, Deserialize)]
pub struct TimePeriodParams {
    pub period: Option<String>, // "day", "week", "month"
}

// API response structure
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub pagination: Option<PaginationInfo>,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}

// Token performance analytics response
#[derive(Debug, Serialize)]
pub struct TokenPerformance {
    pub pool_id: String,
    pub name: String,
    pub symbol: String,
    pub price_change_percentage: f64,
    pub volume_change_percentage: f64,
    pub current_price: i64,
    pub previous_price: i64,
    pub current_volume: i64,
    pub previous_volume: i64,
}

// Portfolio performance analytics response
#[derive(Debug, Serialize)]
pub struct PortfolioPerformance {
    pub address: String,
    pub current_value: i64,
    pub initial_investment: i64,
    pub roi_percentage: f64,
    pub holdings: Vec<PortfolioHolding>,
    pub value_history: Vec<PortfolioValuePoint>,
}

#[derive(Debug, Serialize)]
pub struct PortfolioHolding {
    pub pool_id: String,
    pub name: String,
    pub symbol: String,
    pub amount: i64,
    pub current_value: i64,
    pub initial_value: i64,
    pub roi_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct PortfolioValuePoint {
    pub timestamp: i64,
    pub value: i64,
}

// Creator revenue dashboard response
#[derive(Debug, Serialize)]
pub struct CreatorRevenueReport {
    pub address: String,
    pub total_revenue: i64,
    pub token_pools: Vec<CreatorTokenRevenue>,
    pub revenue_by_period: Vec<RevenuePeriod>,
}

#[derive(Debug, Serialize)]
pub struct CreatorTokenRevenue {
    pub pool_id: String,
    pub name: String,
    pub symbol: String,
    pub total_revenue: i64,
    pub buy_revenue: i64,
    pub sell_revenue: i64,
    pub transactions_count: i64,
}

#[derive(Debug, Serialize)]
pub struct RevenuePeriod {
    pub period_start: i64,
    pub revenue: i64,
}

// Market sentiment response
#[derive(Debug, Serialize)]
pub struct MarketSentiment {
    pub overall_sentiment: f64, // -1.0 to 1.0 (bearish to bullish)
    pub buy_volume_24h: i64,
    pub sell_volume_24h: i64,
    pub transaction_count_24h: i64,
    pub unique_buyers_24h: i64,
    pub unique_sellers_24h: i64,
    pub volume_change_percentage: f64,
    pub price_momentum: Vec<MomentumIndicator>,
}

#[derive(Debug, Serialize)]
pub struct MomentumIndicator {
    pub token_type: i16,
    pub sentiment_score: f64,
    pub volume_change: f64,
}

// Token liquidity profile response
#[derive(Debug, Serialize)]
pub struct TokenLiquidityProfile {
    pub pool_id: String,
    pub name: String,
    pub symbol: String,
    pub total_volume_24h: i64,
    pub transaction_count_24h: i64,
    pub average_transaction_size: i64,
    pub largest_transaction: i64,
    pub unique_traders_count: i64,
    pub buy_sell_ratio: f64,
    pub volume_distribution: Vec<VolumeDistribution>,
}

#[derive(Debug, Serialize)]
pub struct VolumeDistribution {
    pub hour: i64,
    pub volume: i64,
}

// Count result for pagination queries
#[derive(diesel::QueryableByName)]
struct CountResult {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
}

// Custom type for SQL query results in get_user_spt_holdings
#[derive(diesel::QueryableByName)]
struct UserTokenHoldingRow {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub pool_id: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub amount: i64,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub symbol: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub name: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub current_price: i64,
}

/// Get social proof token pool by ID
pub async fn get_spt_pool_by_id(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<SocialProofTokenPoolWithDisplay>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Using raw SQL with diesel because it's a complex query with custom joins
    let query = diesel::sql_query(
        r#"
        WITH latest_profiles AS (
            SELECT DISTINCT ON (profile_id) *
            FROM profiles
            WHERE profile_id IS NOT NULL
            ORDER BY profile_id, updated_at DESC
        ),
        latest_posts AS (
            SELECT DISTINCT ON (post_id) *
            FROM posts
            ORDER BY post_id, time DESC
        )
        SELECT 
            p.id,
            p.pool_id,
            p.token_type,
            p.owner,
            p.associated_id,
            p.symbol,
            p.name,
            p.circulating_supply,
            p.base_price,
            p.quadratic_coefficient,
            p.created_at as created_at_epoch,
            p.time as created_at,
            p.transaction_id,
            COALESCE(ph.price, p.base_price) as current_price,
            CASE 
                WHEN p.token_type = 1 THEN prof.profile_photo
                WHEN p.token_type = 2 THEN 
                    CASE 
                        WHEN post.media_urls IS NOT NULL AND jsonb_typeof(post.media_urls) = 'array' AND jsonb_array_length(post.media_urls) > 0 THEN
                            CASE 
                                WHEN jsonb_typeof(post.media_urls->0) = 'string' THEN post.media_urls->>0
                                WHEN jsonb_typeof(post.media_urls->0) = 'object' THEN post.media_urls->0->>'url'
                                ELSE NULL
                            END
                        ELSE NULL
                    END
                ELSE NULL
            END as icon,
            CASE 
                WHEN p.token_type = 1 THEN 
                    CASE 
                        WHEN prof.profile_id IS NOT NULL THEN COALESCE(prof.display_name, prof.username)
                        ELSE 'Anonymous wallet'
                    END
                WHEN p.token_type = 2 THEN post.content
                ELSE NULL
            END as primary_label,
            CASE 
                WHEN p.token_type = 1 THEN prof.username
                ELSE NULL
            END as secondary_label
        FROM spt_pools p
        LEFT JOIN LATERAL (
            SELECT price
            FROM spt_price_history
            WHERE pool_id = p.pool_id
            ORDER BY time DESC
            LIMIT 1
        ) ph ON true
        LEFT JOIN latest_profiles prof ON 
            p.token_type = 1 AND 
            (CASE 
                WHEN p.associated_id LIKE 'profile_%' THEN SUBSTRING(p.associated_id FROM 9)
                ELSE p.associated_id
            END) = prof.profile_id
        LEFT JOIN latest_posts post ON 
            p.token_type = 2 AND 
            (CASE 
                WHEN p.associated_id LIKE 'post_%' THEN SUBSTRING(p.associated_id FROM 6)
                ELSE p.associated_id
            END) = post.post_id
        WHERE p.pool_id = $1
        ORDER BY p.time DESC
        LIMIT 1
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(id);

    let result = query
        .get_result::<SocialProofTokenPoolWithDisplay>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        data: result,
        pagination: None,
    }))
}

/// List social proof token pools with pagination and filtering
pub async fn list_spt_pools(
    State(db): State<Arc<Database>>,
    Query(pagination): Query<PaginationParams>,
    Query(filters): Query<TokenPoolFilterParams>,
) -> Result<Json<ApiResponse<Vec<SocialProofTokenPoolWithDisplay>>>, StatusCode> {
    let limit = pagination.get_limit();
    let offset = pagination.get_offset();

    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Determine the sort field and direction
    let sort_field = match filters.sort_by.as_deref() {
        Some("created") => "p.time",
        Some("supply") => "p.circulating_supply",
        Some("price") => "current_price",
        _ => "p.time", // Default sort by time
    };

    let sort_dir = match filters.sort_dir.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC", // Default sort descending
    };

    // Common CTEs for latest profiles and posts
    let common_ctes = r#"
        latest_profiles AS (
            SELECT DISTINCT ON (profile_id) *
            FROM profiles
            WHERE profile_id IS NOT NULL
            ORDER BY profile_id, updated_at DESC
        ),
        latest_posts AS (
            SELECT DISTINCT ON (post_id) *
            FROM posts
            ORDER BY post_id, time DESC
        )
    "#;

    // Build and execute the query based on filter conditions
    let token_pools = match (filters.token_type, &filters.owner) {
        (Some(token_type), Some(owner)) => {
            // Both token_type and owner filters
            diesel::sql_query(&format!(
                r#"
                WITH latest_pools AS (
                    SELECT DISTINCT ON (pool_id) *
                    FROM spt_pools
                    WHERE token_type = $1 AND owner = $2
                    ORDER BY pool_id, time DESC
                ),
                {}
                SELECT 
                    p.id,
                    p.pool_id,
                    p.token_type,
                    p.owner,
                    p.associated_id,
                    p.symbol,
                    p.name,
                    p.circulating_supply,
                    p.base_price,
                    p.quadratic_coefficient,
                    p.created_at as created_at_epoch,
                    p.time as created_at,
                    p.transaction_id,
                    COALESCE(ph.price, p.base_price) as current_price,
                    CASE 
                        WHEN p.token_type = 1 THEN prof.profile_photo
                        WHEN p.token_type = 2 THEN 
                            CASE 
                                WHEN post.media_urls IS NOT NULL AND jsonb_typeof(post.media_urls) = 'array' AND jsonb_array_length(post.media_urls) > 0 THEN
                                    CASE 
                                        WHEN jsonb_typeof(post.media_urls->0) = 'string' THEN post.media_urls->>0
                                        WHEN jsonb_typeof(post.media_urls->0) = 'object' THEN post.media_urls->0->>'url'
                                        ELSE NULL
                                    END
                                ELSE NULL
                            END
                        ELSE NULL
                    END as icon,
                    CASE 
                        WHEN p.token_type = 1 THEN COALESCE(prof.display_name, prof.username)
                        WHEN p.token_type = 2 THEN post.content
                        ELSE NULL
                    END as primary_label,
                    CASE 
                        WHEN p.token_type = 1 THEN prof.username
                        ELSE NULL
                    END as secondary_label
                FROM latest_pools p
                LEFT JOIN LATERAL (
                    SELECT price
                    FROM spt_price_history
                    WHERE pool_id = p.pool_id
                    ORDER BY time DESC
                    LIMIT 1
                ) ph ON true
                LEFT JOIN latest_profiles prof ON 
                    p.token_type = 1 AND 
                    (CASE 
                        WHEN p.associated_id LIKE 'profile_%' THEN SUBSTRING(p.associated_id FROM 9)
                        ELSE p.associated_id
                    END) = prof.profile_id
                LEFT JOIN latest_posts post ON 
                    p.token_type = 2 AND 
                    (CASE 
                        WHEN p.associated_id LIKE 'post_%' THEN SUBSTRING(p.associated_id FROM 6)
                        ELSE p.associated_id
                    END) = post.post_id
                ORDER BY {} {}
                LIMIT $3 OFFSET $4
                "#,
                common_ctes, sort_field, sort_dir
            ))
            .bind::<diesel::sql_types::SmallInt, _>(token_type)
            .bind::<diesel::sql_types::Text, _>(owner)
            .bind::<diesel::sql_types::BigInt, _>(limit)
            .bind::<diesel::sql_types::BigInt, _>(offset)
            .load::<SocialProofTokenPoolWithDisplay>(&mut conn)
            .await
        }
        (Some(token_type), None) => {
            // Only token_type filter
            diesel::sql_query(&format!(
                r#"
                WITH latest_pools AS (
                    SELECT DISTINCT ON (pool_id) *
                    FROM spt_pools
                    WHERE token_type = $1
                    ORDER BY pool_id, time DESC
                ),
                {}
                SELECT 
                    p.id,
                    p.pool_id,
                    p.token_type,
                    p.owner,
                    p.associated_id,
                    p.symbol,
                    p.name,
                    p.circulating_supply,
                    p.base_price,
                    p.quadratic_coefficient,
                    p.created_at as created_at_epoch,
                    p.time as created_at,
                    p.transaction_id,
                    COALESCE(ph.price, p.base_price) as current_price,
                    CASE 
                        WHEN p.token_type = 1 THEN prof.profile_photo
                        WHEN p.token_type = 2 THEN 
                            CASE 
                                WHEN post.media_urls IS NOT NULL AND jsonb_typeof(post.media_urls) = 'array' AND jsonb_array_length(post.media_urls) > 0 THEN
                                    CASE 
                                        WHEN jsonb_typeof(post.media_urls->0) = 'string' THEN post.media_urls->>0
                                        WHEN jsonb_typeof(post.media_urls->0) = 'object' THEN post.media_urls->0->>'url'
                                        ELSE NULL
                                    END
                                ELSE NULL
                            END
                        ELSE NULL
                    END as icon,
                    CASE 
                        WHEN p.token_type = 1 THEN COALESCE(prof.display_name, prof.username)
                        WHEN p.token_type = 2 THEN post.content
                        ELSE NULL
                    END as primary_label,
                    CASE 
                        WHEN p.token_type = 1 THEN prof.username
                        ELSE NULL
                    END as secondary_label
                FROM latest_pools p
                LEFT JOIN LATERAL (
                    SELECT price
                    FROM spt_price_history
                    WHERE pool_id = p.pool_id
                    ORDER BY time DESC
                    LIMIT 1
                ) ph ON true
                LEFT JOIN latest_profiles prof ON 
                    p.token_type = 1 AND 
                    (CASE 
                        WHEN p.associated_id LIKE 'profile_%' THEN SUBSTRING(p.associated_id FROM 9)
                        ELSE p.associated_id
                    END) = prof.profile_id
                LEFT JOIN latest_posts post ON 
                    p.token_type = 2 AND 
                    (CASE 
                        WHEN p.associated_id LIKE 'post_%' THEN SUBSTRING(p.associated_id FROM 6)
                        ELSE p.associated_id
                    END) = post.post_id
                ORDER BY {} {}
                LIMIT $2 OFFSET $3
                "#,
                common_ctes, sort_field, sort_dir
            ))
            .bind::<diesel::sql_types::SmallInt, _>(token_type)
            .bind::<diesel::sql_types::BigInt, _>(limit)
            .bind::<diesel::sql_types::BigInt, _>(offset)
            .load::<SocialProofTokenPoolWithDisplay>(&mut conn)
            .await
        }
        (None, Some(owner)) => {
            // Only owner filter
            diesel::sql_query(&format!(
                r#"
                WITH latest_pools AS (
                    SELECT DISTINCT ON (pool_id) *
                    FROM spt_pools
                    WHERE owner = $1
                    ORDER BY pool_id, time DESC
                ),
                {}
                SELECT 
                    p.id,
                    p.pool_id,
                    p.token_type,
                    p.owner,
                    p.associated_id,
                    p.symbol,
                    p.name,
                    p.circulating_supply,
                    p.base_price,
                    p.quadratic_coefficient,
                    p.created_at as created_at_epoch,
                    p.time as created_at,
                    p.transaction_id,
                    COALESCE(ph.price, p.base_price) as current_price,
                    CASE 
                        WHEN p.token_type = 1 THEN prof.profile_photo
                        WHEN p.token_type = 2 THEN 
                            CASE 
                                WHEN post.media_urls IS NOT NULL AND jsonb_typeof(post.media_urls) = 'array' AND jsonb_array_length(post.media_urls) > 0 THEN
                                    CASE 
                                        WHEN jsonb_typeof(post.media_urls->0) = 'string' THEN post.media_urls->>0
                                        WHEN jsonb_typeof(post.media_urls->0) = 'object' THEN post.media_urls->0->>'url'
                                        ELSE NULL
                                    END
                                ELSE NULL
                            END
                        ELSE NULL
                    END as icon,
                    CASE 
                        WHEN p.token_type = 1 THEN COALESCE(prof.display_name, prof.username)
                        WHEN p.token_type = 2 THEN post.content
                        ELSE NULL
                    END as primary_label,
                    CASE 
                        WHEN p.token_type = 1 THEN prof.username
                        ELSE NULL
                    END as secondary_label
                FROM latest_pools p
                LEFT JOIN LATERAL (
                    SELECT price
                    FROM spt_price_history
                    WHERE pool_id = p.pool_id
                    ORDER BY time DESC
                    LIMIT 1
                ) ph ON true
                LEFT JOIN latest_profiles prof ON 
                    p.token_type = 1 AND 
                    (CASE 
                        WHEN p.associated_id LIKE 'profile_%' THEN SUBSTRING(p.associated_id FROM 9)
                        ELSE p.associated_id
                    END) = prof.profile_id
                LEFT JOIN latest_posts post ON 
                    p.token_type = 2 AND 
                    (CASE 
                        WHEN p.associated_id LIKE 'post_%' THEN SUBSTRING(p.associated_id FROM 6)
                        ELSE p.associated_id
                    END) = post.post_id
                ORDER BY {} {}
                LIMIT $2 OFFSET $3
                "#,
                common_ctes, sort_field, sort_dir
            ))
            .bind::<diesel::sql_types::Text, _>(owner)
            .bind::<diesel::sql_types::BigInt, _>(limit)
            .bind::<diesel::sql_types::BigInt, _>(offset)
            .load::<SocialProofTokenPoolWithDisplay>(&mut conn)
            .await
        }
        (None, None) => {
            // No filters
            diesel::sql_query(&format!(
                r#"
                WITH latest_pools AS (
                    SELECT DISTINCT ON (pool_id) *
                    FROM spt_pools
                    ORDER BY pool_id, time DESC
                ),
                {}
                SELECT 
                    p.id,
                    p.pool_id,
                    p.token_type,
                    p.owner,
                    p.associated_id,
                    p.symbol,
                    p.name,
                    p.circulating_supply,
                    p.base_price,
                    p.quadratic_coefficient,
                    p.created_at as created_at_epoch,
                    p.time as created_at,
                    p.transaction_id,
                    COALESCE(ph.price, p.base_price) as current_price,
                    CASE 
                        WHEN p.token_type = 1 THEN prof.profile_photo
                        WHEN p.token_type = 2 THEN 
                            CASE 
                                WHEN post.media_urls IS NOT NULL AND jsonb_typeof(post.media_urls) = 'array' AND jsonb_array_length(post.media_urls) > 0 THEN
                                    CASE 
                                        WHEN jsonb_typeof(post.media_urls->0) = 'string' THEN post.media_urls->>0
                                        WHEN jsonb_typeof(post.media_urls->0) = 'object' THEN post.media_urls->0->>'url'
                                        ELSE NULL
                                    END
                                ELSE NULL
                            END
                        ELSE NULL
                    END as icon,
                    CASE 
                        WHEN p.token_type = 1 THEN COALESCE(prof.display_name, prof.username)
                        WHEN p.token_type = 2 THEN post.content
                        ELSE NULL
                    END as primary_label,
                    CASE 
                        WHEN p.token_type = 1 THEN prof.username
                        ELSE NULL
                    END as secondary_label
                FROM latest_pools p
                LEFT JOIN LATERAL (
                    SELECT price
                    FROM spt_price_history
                    WHERE pool_id = p.pool_id
                    ORDER BY time DESC
                    LIMIT 1
                ) ph ON true
                LEFT JOIN latest_profiles prof ON 
                    p.token_type = 1 AND 
                    (CASE 
                        WHEN p.associated_id LIKE 'profile_%' THEN SUBSTRING(p.associated_id FROM 9)
                        ELSE p.associated_id
                    END) = prof.profile_id
                LEFT JOIN latest_posts post ON 
                    p.token_type = 2 AND 
                    (CASE 
                        WHEN p.associated_id LIKE 'post_%' THEN SUBSTRING(p.associated_id FROM 6)
                        ELSE p.associated_id
                    END) = post.post_id
                ORDER BY {} {}
                LIMIT $1 OFFSET $2
                "#,
                common_ctes, sort_field, sort_dir
            ))
            .bind::<diesel::sql_types::BigInt, _>(limit)
            .bind::<diesel::sql_types::BigInt, _>(offset)
            .load::<SocialProofTokenPoolWithDisplay>(&mut conn)
            .await
        }
    }
    .map_err(|e| {
        error!("Database error in main query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Count total for pagination
    let total_count = diesel::sql_query(
        "
        SELECT COUNT(DISTINCT pool_id) as count
        FROM spt_pools p
    ",
    )
    .get_result::<CountResult>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error in count query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total = total_count.count;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(ApiResponse {
        data: token_pools,
        pagination: Some(PaginationInfo {
            page: pagination.get_page(),
            limit,
            total,
            total_pages,
        }),
    }))
}

/// Get social proof token pool by associated ID (profile or post ID)
pub async fn get_spt_pool_by_associated_id(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<SocialProofTokenPoolWithDisplay>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let query = diesel::sql_query(
        r#"
        WITH latest_profiles AS (
            SELECT DISTINCT ON (profile_id) *
            FROM profiles
            WHERE profile_id IS NOT NULL
            ORDER BY profile_id, updated_at DESC
        ),
        latest_posts AS (
            SELECT DISTINCT ON (post_id) *
            FROM posts
            ORDER BY post_id, time DESC
        )
        SELECT 
            p.id,
            p.pool_id,
            p.token_type,
            p.owner,
            p.associated_id,
            p.symbol,
            p.name,
            p.circulating_supply,
            p.base_price,
            p.quadratic_coefficient,
            p.created_at as created_at_epoch,
            p.time as created_at,
            p.transaction_id,
            COALESCE(ph.price, p.base_price) as current_price,
            CASE 
                WHEN p.token_type = 1 THEN prof.profile_photo
                WHEN p.token_type = 2 THEN 
                    CASE 
                        WHEN post.media_urls IS NOT NULL AND jsonb_typeof(post.media_urls) = 'array' AND jsonb_array_length(post.media_urls) > 0 THEN
                            CASE 
                                WHEN jsonb_typeof(post.media_urls->0) = 'string' THEN post.media_urls->>0
                                WHEN jsonb_typeof(post.media_urls->0) = 'object' THEN post.media_urls->0->>'url'
                                ELSE NULL
                            END
                        ELSE NULL
                    END
                ELSE NULL
            END as icon,
            CASE 
                WHEN p.token_type = 1 THEN 
                    CASE 
                        WHEN prof.profile_id IS NOT NULL THEN COALESCE(prof.display_name, prof.username)
                        ELSE 'Anonymous wallet'
                    END
                WHEN p.token_type = 2 THEN post.content
                ELSE NULL
            END as primary_label,
            CASE 
                WHEN p.token_type = 1 THEN prof.username
                ELSE NULL
            END as secondary_label
        FROM spt_pools p
        LEFT JOIN LATERAL (
            SELECT price
            FROM spt_price_history
            WHERE pool_id = p.pool_id
            ORDER BY time DESC
            LIMIT 1
        ) ph ON true
        LEFT JOIN latest_profiles prof ON 
            p.token_type = 1 AND 
            (CASE 
                WHEN p.associated_id LIKE 'profile_%' THEN SUBSTRING(p.associated_id FROM 9)
                ELSE p.associated_id
            END) = prof.profile_id
        LEFT JOIN latest_posts post ON 
            p.token_type = 2 AND 
            (CASE 
                WHEN p.associated_id LIKE 'post_%' THEN SUBSTRING(p.associated_id FROM 6)
                ELSE p.associated_id
            END) = post.post_id
        WHERE p.associated_id = $1
        ORDER BY p.time DESC
        LIMIT 1
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(id);

    let result = query
        .get_result::<SocialProofTokenPoolWithDisplay>(&mut conn)
        .await
        .optional()
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match result {
        Some(token_pool) => Ok(Json(ApiResponse {
            data: token_pool,
            pagination: None,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Get transactions for a token pool
pub async fn get_spt_transactions(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SocialProofTokenTransaction>>>, StatusCode> {
    let limit = pagination.get_limit();
    let offset = pagination.get_offset();

    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get transactions
    let transactions = diesel::sql_query(
        r#"
        SELECT *
        FROM spt_transactions
        WHERE pool_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(id.clone())
    .bind::<diesel::sql_types::BigInt, _>(limit)
    .bind::<diesel::sql_types::BigInt, _>(offset)
    .load::<SocialProofTokenTransaction>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Count total for pagination
    let total_count = diesel::sql_query(
        r#"
        SELECT COUNT(*) as count
        FROM spt_transactions
        WHERE pool_id = $1
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(id)
    .get_result::<CountResult>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error in count query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total = total_count.count;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(ApiResponse {
        data: transactions,
        pagination: Some(PaginationInfo {
            page: pagination.get_page(),
            limit,
            total,
            total_pages,
        }),
    }))
}

/// Get token holdings for a token pool
pub async fn get_spt_holdings(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SocialProofTokenHolding>>>, StatusCode> {
    let limit = pagination.get_limit();
    let offset = pagination.get_offset();

    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get holdings
    let holdings = diesel::sql_query(
        r#"
        WITH latest_holdings AS (
            SELECT DISTINCT ON (holder_address) *
            FROM spt_holdings
            WHERE pool_id = $1
            ORDER BY holder_address, time DESC
        )
        SELECT *
        FROM latest_holdings
        WHERE amount > 0
        ORDER BY amount DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(id.clone())
    .bind::<diesel::sql_types::BigInt, _>(limit)
    .bind::<diesel::sql_types::BigInt, _>(offset)
    .load::<SocialProofTokenHolding>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Count total for pagination
    let total_count = diesel::sql_query(
        r#"
        WITH latest_holdings AS (
            SELECT DISTINCT ON (holder_address) *
            FROM spt_holdings
            WHERE pool_id = $1
            ORDER BY holder_address, time DESC
        )
        SELECT COUNT(*) as count
        FROM latest_holdings
        WHERE amount > 0
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(id)
    .get_result::<CountResult>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error in count query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total = total_count.count;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(ApiResponse {
        data: holdings,
        pagination: Some(PaginationInfo {
            page: pagination.get_page(),
            limit,
            total,
            total_pages,
        }),
    }))
}

/// Get price history for a token pool
pub async fn get_spt_price_history(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    Query(time_range): Query<TimeRangeParams>,
) -> Result<Json<ApiResponse<Vec<SocialProofPriceAggregation>>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Determine the interval to use for bucketing
    let interval = match time_range.interval.as_deref() {
        Some("hour") => "1 hour",
        Some("day") => "1 day",
        Some("week") => "1 week",
        Some("month") => "1 month",
        _ => "1 hour", // Default to hourly
    };

    // Execute query with parameters directly
    let price_history = if let (Some(from), Some(to)) = (time_range.from, time_range.to) {
        let from_timestamp = chrono::DateTime::<chrono::Utc>::from_timestamp(from, 0)
            .expect("Invalid from timestamp");
        let to_timestamp =
            chrono::DateTime::<chrono::Utc>::from_timestamp(to, 0).expect("Invalid to timestamp");

        diesel::sql_query(&format!(
            r#"
            SELECT 
                pool_id,
                time_bucket('{}', time) AS bucket,
                FIRST(price, time) AS open,
                MAX(price) AS high,
                MIN(price) AS low,
                LAST(price, time) AS close,
                LAST(circulating_supply, time) AS circulating_supply
            FROM spt_price_history
            WHERE pool_id = $1
              AND time >= $2
              AND time <= $3
            GROUP BY pool_id, bucket
            ORDER BY bucket ASC
            "#,
            interval
        ))
        .bind::<diesel::sql_types::Text, _>(&id)
        .bind::<diesel::sql_types::Timestamptz, _>(from_timestamp)
        .bind::<diesel::sql_types::Timestamptz, _>(to_timestamp)
        .load::<SocialProofPriceAggregation>(&mut conn)
        .await
    } else if let Some(from) = time_range.from {
        let from_timestamp = chrono::DateTime::<chrono::Utc>::from_timestamp(from, 0)
            .expect("Invalid from timestamp");

        diesel::sql_query(&format!(
            r#"
            SELECT 
                pool_id,
                time_bucket('{}', time) AS bucket,
                FIRST(price, time) AS open,
                MAX(price) AS high,
                MIN(price) AS low,
                LAST(price, time) AS close,
                LAST(circulating_supply, time) AS circulating_supply
            FROM spt_price_history
            WHERE pool_id = $1
              AND time >= $2
            GROUP BY pool_id, bucket
            ORDER BY bucket ASC
            "#,
            interval
        ))
        .bind::<diesel::sql_types::Text, _>(&id)
        .bind::<diesel::sql_types::Timestamptz, _>(from_timestamp)
        .load::<SocialProofPriceAggregation>(&mut conn)
        .await
    } else if let Some(to) = time_range.to {
        let to_timestamp =
            chrono::DateTime::<chrono::Utc>::from_timestamp(to, 0).expect("Invalid to timestamp");

        diesel::sql_query(&format!(
            r#"
            SELECT 
                pool_id,
                time_bucket('{}', time) AS bucket,
                FIRST(price, time) AS open,
                MAX(price) AS high,
                MIN(price) AS low,
                LAST(price, time) AS close,
                LAST(circulating_supply, time) AS circulating_supply
            FROM spt_price_history
            WHERE pool_id = $1
              AND time <= $2
            GROUP BY pool_id, bucket
            ORDER BY bucket ASC
            "#,
            interval
        ))
        .bind::<diesel::sql_types::Text, _>(&id)
        .bind::<diesel::sql_types::Timestamptz, _>(to_timestamp)
        .load::<SocialProofPriceAggregation>(&mut conn)
        .await
    } else {
        diesel::sql_query(&format!(
            r#"
            SELECT 
                pool_id,
                time_bucket('{}', time) AS bucket,
                FIRST(price, time) AS open,
                MAX(price) AS high,
                MIN(price) AS low,
                LAST(price, time) AS close,
                LAST(circulating_supply, time) AS circulating_supply
            FROM spt_price_history
            WHERE pool_id = $1
            GROUP BY pool_id, bucket
            ORDER BY bucket ASC
            "#,
            interval
        ))
        .bind::<diesel::sql_types::Text, _>(&id)
        .load::<SocialProofPriceAggregation>(&mut conn)
        .await
    }
    .map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ApiResponse {
        data: price_history,
        pagination: None,
    }))
}

/// Get active reservation pools
pub async fn get_spt_reservation_pools(
    State(db): State<Arc<Database>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SptReservationPoolWithDisplay>>>, StatusCode> {
    let limit = pagination.get_limit();
    let offset = pagination.get_offset();

    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get active reservation pools with display fields from profiles/posts
    let reservation_pools = diesel::sql_query(
        r#"
        WITH latest_reservation_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_reservation_pools
            ORDER BY pool_id, time DESC
        ),
        latest_profiles AS (
            SELECT DISTINCT ON (profile_id) *
            FROM profiles
            WHERE profile_id IS NOT NULL
            ORDER BY profile_id, updated_at DESC
        ),
        latest_posts AS (
            SELECT DISTINCT ON (post_id) *
            FROM posts
            ORDER BY post_id, time DESC
        )
        SELECT 
            rp.id,
            rp.pool_id,
            rp.associated_id,
            rp.token_type,
            rp.owner,
            rp.total_reserved,
            rp.required_threshold,
            rp.status,
            rp.created_at as created_at_epoch,
            rp.time as created_at,
            rp.transaction_id,
            CASE 
                WHEN rp.token_type = 1 THEN prof.profile_photo
                WHEN rp.token_type = 2 THEN 
                    CASE 
                        WHEN post.media_urls IS NOT NULL AND jsonb_typeof(post.media_urls) = 'array' AND jsonb_array_length(post.media_urls) > 0 THEN
                            CASE 
                                WHEN jsonb_typeof(post.media_urls->0) = 'string' THEN post.media_urls->>0
                                WHEN jsonb_typeof(post.media_urls->0) = 'object' THEN post.media_urls->0->>'url'
                                ELSE NULL
                            END
                        ELSE NULL
                    END
                ELSE NULL
            END as icon,
            CASE 
                WHEN rp.token_type = 1 THEN 
                    CASE 
                        WHEN prof.profile_id IS NOT NULL THEN COALESCE(prof.display_name, prof.username)
                        ELSE 'Anonymous wallet'
                    END
                WHEN rp.token_type = 2 THEN post.content
                ELSE NULL
            END as primary_label,
            CASE 
                WHEN rp.token_type = 1 THEN prof.username
                ELSE NULL
            END as secondary_label
        FROM latest_reservation_pools rp
        LEFT JOIN latest_profiles prof ON 
            rp.token_type = 1 AND 
            (CASE 
                WHEN rp.associated_id LIKE 'profile_%' THEN SUBSTRING(rp.associated_id FROM 9)
                ELSE rp.associated_id
            END) = prof.profile_id
        LEFT JOIN latest_posts post ON 
            rp.token_type = 2 AND 
            (CASE 
                WHEN rp.associated_id LIKE 'post_%' THEN SUBSTRING(rp.associated_id FROM 6)
                ELSE rp.associated_id
            END) = post.post_id
        WHERE rp.status = 'active' OR rp.status = 'threshold_met'
        ORDER BY rp.total_reserved DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind::<diesel::sql_types::BigInt, _>(limit)
    .bind::<diesel::sql_types::BigInt, _>(offset)
    .load::<SptReservationPoolWithDisplay>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Count total for pagination
    let total_count = diesel::sql_query(
        r#"
        WITH latest_reservation_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_reservation_pools
            ORDER BY pool_id, time DESC
        )
        SELECT COUNT(*) as count
        FROM latest_reservation_pools
        WHERE status = 'active' OR status = 'threshold_met'
        "#,
    )
    .get_result::<CountResult>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error in count query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total = total_count.count;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(ApiResponse {
        data: reservation_pools,
        pagination: Some(PaginationInfo {
            page: pagination.get_page(),
            limit,
            total,
            total_pages,
        }),
    }))
}

/// Get reservation pool by ID
/// Enhanced reservation pool response with fee breakdowns
#[derive(Debug, Serialize)]
pub struct EnhancedReservationPool {
    #[serde(flatten)]
    pub pool: SptReservationPoolWithDisplay,
    pub total_fees_paid: i64,
    pub total_creator_fees: i64,
    pub total_platform_fees: i64,
    pub total_treasury_fees: i64,
    pub reservation_count: i64,
    pub unique_reservers: i64,
}

pub async fn get_spt_reservation_pool_by_id(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<EnhancedReservationPool>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Handle both old format (reservation_pool_0x...) and new format (0x...)
    // Also support lookup by associated_id (profile_0x... or post_0x...)
    let pool_id = if id.starts_with("reservation_pool_") {
        id.strip_prefix("reservation_pool_").unwrap().to_string()
    } else {
        id.clone()
    };

    // Get the reservation pool with display fields
    let pool_result = diesel::sql_query(
        r#"
        WITH latest_reservation_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_reservation_pools
            WHERE pool_id = $1 OR associated_id = $1
            ORDER BY pool_id, time DESC
        ),
        latest_profiles AS (
            SELECT DISTINCT ON (profile_id) *
            FROM profiles
            WHERE profile_id IS NOT NULL
            ORDER BY profile_id, updated_at DESC
        ),
        latest_posts AS (
            SELECT DISTINCT ON (post_id) *
            FROM posts
            ORDER BY post_id, time DESC
        )
        SELECT 
            rp.id,
            rp.pool_id,
            rp.associated_id,
            rp.token_type,
            rp.owner,
            rp.total_reserved,
            rp.required_threshold,
            rp.status,
            rp.created_at as created_at_epoch,
            rp.time as created_at,
            rp.transaction_id,
            CASE 
                WHEN rp.token_type = 1 THEN prof.profile_photo
                WHEN rp.token_type = 2 THEN 
                    CASE 
                        WHEN post.media_urls IS NOT NULL AND jsonb_typeof(post.media_urls) = 'array' AND jsonb_array_length(post.media_urls) > 0 THEN
                            CASE 
                                WHEN jsonb_typeof(post.media_urls->0) = 'string' THEN post.media_urls->>0
                                WHEN jsonb_typeof(post.media_urls->0) = 'object' THEN post.media_urls->0->>'url'
                                ELSE NULL
                            END
                        ELSE NULL
                    END
                ELSE NULL
            END as icon,
            CASE 
                WHEN rp.token_type = 1 THEN 
                    CASE 
                        WHEN prof.profile_id IS NOT NULL THEN COALESCE(prof.display_name, prof.username)
                        ELSE 'Anonymous wallet'
                    END
                WHEN rp.token_type = 2 THEN post.content
                ELSE NULL
            END as primary_label,
            CASE 
                WHEN rp.token_type = 1 THEN prof.username
                ELSE NULL
            END as secondary_label
        FROM latest_reservation_pools rp
        LEFT JOIN latest_profiles prof ON 
            rp.token_type = 1 AND 
            (CASE 
                WHEN rp.associated_id LIKE 'profile_%' THEN SUBSTRING(rp.associated_id FROM 9)
                ELSE rp.associated_id
            END) = prof.profile_id
        LEFT JOIN latest_posts post ON 
            rp.token_type = 2 AND 
            (CASE 
                WHEN rp.associated_id LIKE 'post_%' THEN SUBSTRING(rp.associated_id FROM 6)
                ELSE rp.associated_id
            END) = post.post_id
        ORDER BY rp.time DESC
        LIMIT 1
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(pool_id.clone())
    .get_result::<SptReservationPoolWithDisplay>(&mut conn)
    .await
    .optional()
    .map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let pool = match pool_result {
        Some(p) => p,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Get fee totals and reservation stats
    #[derive(Debug, QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct FeeStats {
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_fees: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_creator_fees: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_platform_fees: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_treasury_fees: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        reservation_count: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_reservers: i64,
    }

    let stats = diesel::sql_query(
        r#"
        WITH latest_reservations AS (
            SELECT DISTINCT ON (reserver_address) *
            FROM spt_reservations
            WHERE pool_id = $1
            ORDER BY reserver_address, time DESC
        )
        SELECT 
            COALESCE(SUM(COALESCE(fee_amount, 0)), 0) as total_fees,
            COALESCE(SUM(COALESCE(creator_fee, 0)), 0) as total_creator_fees,
            COALESCE(SUM(COALESCE(platform_fee, 0)), 0) as total_platform_fees,
            COALESCE(SUM(COALESCE(treasury_fee, 0)), 0) as total_treasury_fees,
            COUNT(*) as reservation_count,
            COUNT(DISTINCT reserver_address) as unique_reservers
        FROM latest_reservations
        WHERE amount > 0
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(pool.pool_id.clone())
    .get_result::<FeeStats>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error getting fee stats: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let enhanced = EnhancedReservationPool {
        pool,
        total_fees_paid: stats.total_fees,
        total_creator_fees: stats.total_creator_fees,
        total_platform_fees: stats.total_platform_fees,
        total_treasury_fees: stats.total_treasury_fees,
        reservation_count: stats.reservation_count,
        unique_reservers: stats.unique_reservers,
    };

    Ok(Json(ApiResponse {
        data: enhanced,
        pagination: None,
    }))
}

/// Get reservations for a pool
pub async fn get_spt_reservations_by_pool(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SptReservation>>>, StatusCode> {
    let limit = pagination.get_limit();
    let offset = pagination.get_offset();

    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Handle both old format (reservation_pool_0x...) and new format (0x...)
    // Also support lookup by associated_id - need to resolve to actual pool_id
    let pool_id = if id.starts_with("reservation_pool_") {
        id.strip_prefix("reservation_pool_").unwrap().to_string()
    } else {
        id.clone()
    };

    // If it looks like an associated_id (profile_0x... or post_0x...), look up the pool first
    let actual_pool_id = if pool_id.starts_with("profile_") || pool_id.starts_with("post_") {
        // Look up the pool by associated_id to get the actual pool_id
        #[derive(Debug, QueryableByName)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        struct PoolIdResult {
            #[diesel(sql_type = diesel::sql_types::Text)]
            pool_id: String,
        }

        let pool_result = diesel::sql_query(
            r#"
            SELECT pool_id
            FROM spt_reservation_pools
            WHERE associated_id = $1
            ORDER BY time DESC
            LIMIT 1
            "#,
        )
        .bind::<diesel::sql_types::Text, _>(pool_id.clone())
        .get_result::<PoolIdResult>(&mut conn)
        .await
        .optional()
        .map_err(|e| {
            error!("Database error looking up pool: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        match pool_result {
            Some(result) => result.pool_id,
            None => return Err(StatusCode::NOT_FOUND),
        }
    } else {
        pool_id
    };

    // Get reservations - latest per reserver
    let reservations = diesel::sql_query(
        r#"
        WITH latest_reservations AS (
            SELECT DISTINCT ON (reserver_address) *
            FROM spt_reservations
            WHERE pool_id = $1
            ORDER BY reserver_address, time DESC
        )
        SELECT *
        FROM latest_reservations
        WHERE amount > 0
        ORDER BY amount DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(actual_pool_id.clone())
    .bind::<diesel::sql_types::BigInt, _>(limit)
    .bind::<diesel::sql_types::BigInt, _>(offset)
    .load::<SptReservation>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Count total for pagination
    let total_count = diesel::sql_query(
        r#"
        WITH latest_reservations AS (
            SELECT DISTINCT ON (reserver_address) *
            FROM spt_reservations
            WHERE pool_id = $1
            ORDER BY reserver_address, time DESC
        )
        SELECT COUNT(*) as count
        FROM latest_reservations
        WHERE amount > 0
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(actual_pool_id)
    .get_result::<CountResult>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error in count query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total = total_count.count;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(ApiResponse {
        data: reservations,
        pagination: Some(PaginationInfo {
            page: pagination.get_page(),
            limit,
            total,
            total_pages,
        }),
    }))
}

/// Query parameters for user holdings endpoint
#[derive(Debug, Deserialize)]
pub struct UserHoldingsParams {
    pub include_reservations: Option<bool>,
}

/// Reservation info for user holdings
#[derive(Debug, Serialize)]
pub struct UserReservationInfo {
    pub pool_id: String,
    pub associated_id: String,
    pub amount: i64,
    pub fee_amount: Option<i64>,
    pub creator_fee: Option<i64>,
    pub platform_fee: Option<i64>,
    pub treasury_fee: Option<i64>,
    pub reserved_at: i64,
}

/// Enhanced user token holdings with optional reservations
#[derive(Debug, Serialize)]
pub struct EnhancedUserTokenHoldings {
    pub holder_address: String,
    pub holdings: Vec<UserTokenHolding>,
    pub total_value: i64,
    pub reservations: Option<Vec<UserReservationInfo>>,
    pub total_reservation_value: Option<i64>,
}

/// Get token holdings for a user
pub async fn get_user_spt_holdings(
    State(db): State<Arc<Database>>,
    Path(address): Path<String>,
    Query(params): Query<UserHoldingsParams>,
) -> Result<Json<ApiResponse<EnhancedUserTokenHoldings>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Using raw SQL as this requires complex selects and joins
    let holdings_rows = diesel::sql_query(
        r#"
        WITH latest_holdings AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_holdings
            WHERE holder_address = $1
            ORDER BY pool_id, time DESC
        ),
        pool_info AS (
            SELECT DISTINCT ON (pool_id) p.*, 
                   COALESCE(ph.price, p.base_price) as current_price
            FROM spt_pools p
            LEFT JOIN LATERAL (
                SELECT price
                FROM spt_price_history
                WHERE pool_id = p.pool_id
                ORDER BY time DESC
                LIMIT 1
            ) ph ON true
            ORDER BY pool_id, p.time DESC
        )
        SELECT 
            h.pool_id, h.amount, 
            p.symbol, p.name, p.current_price
        FROM latest_holdings h
        JOIN pool_info p ON h.pool_id = p.pool_id
        WHERE h.amount > 0
        ORDER BY h.amount * p.current_price DESC
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(address.clone())
    .load::<UserTokenHoldingRow>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Convert to UserTokenHolding objects
    let mut total_value: i64 = 0;
    let mut user_holdings: Vec<UserTokenHolding> = Vec::new();

    for row in holdings_rows {
        let value = row.current_price * row.amount;
        total_value += value;

        user_holdings.push(UserTokenHolding {
            pool_id: row.pool_id,
            symbol: row.symbol,
            name: row.name,
            amount: row.amount,
            current_price: row.current_price,
            value,
        });
    }

    // Get reservations if requested
    let (reservations, total_reservation_value) = if params.include_reservations.unwrap_or(false) {
        #[derive(Debug, QueryableByName)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        struct ReservationRow {
            #[diesel(sql_type = diesel::sql_types::Text)]
            pool_id: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            associated_id: String,
            #[diesel(sql_type = diesel::sql_types::BigInt)]
            amount: i64,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
            fee_amount: Option<i64>,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
            creator_fee: Option<i64>,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
            platform_fee: Option<i64>,
            #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
            treasury_fee: Option<i64>,
            #[diesel(sql_type = diesel::sql_types::BigInt)]
            reserved_at: i64,
        }

        let reservation_rows = diesel::sql_query(
            r#"
            WITH latest_reservations AS (
                SELECT DISTINCT ON (r.pool_id, r.reserver_address) 
                    r.pool_id,
                    r.reserver_address,
                    r.amount,
                    r.fee_amount,
                    r.creator_fee,
                    r.platform_fee,
                    r.treasury_fee,
                    r.reserved_at,
                    rp.associated_id
                FROM spt_reservations r
                LEFT JOIN spt_reservation_pools rp ON r.pool_id = rp.pool_id
                WHERE r.reserver_address = $1
                ORDER BY r.pool_id, r.reserver_address, r.time DESC
            )
            SELECT 
                pool_id,
                COALESCE(associated_id, '') as associated_id,
                amount,
                fee_amount,
                creator_fee,
                platform_fee,
                treasury_fee,
                reserved_at
            FROM latest_reservations
            WHERE amount > 0
            ORDER BY amount DESC
            "#,
        )
        .bind::<diesel::sql_types::Text, _>(address.clone())
        .load::<ReservationRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error getting reservations: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let reservations: Vec<UserReservationInfo> = reservation_rows
            .into_iter()
            .map(|r| UserReservationInfo {
                pool_id: r.pool_id,
                associated_id: r.associated_id,
                amount: r.amount,
                fee_amount: r.fee_amount,
                creator_fee: r.creator_fee,
                platform_fee: r.platform_fee,
                treasury_fee: r.treasury_fee,
                reserved_at: r.reserved_at,
            })
            .collect();

        let total_reservation_value = reservations.iter().map(|r| r.amount).sum();

        (Some(reservations), Some(total_reservation_value))
    } else {
        (None, None)
    };

    // Create the response
    let result = EnhancedUserTokenHoldings {
        holder_address: address,
        holdings: user_holdings,
        total_value,
        reservations,
        total_reservation_value,
    };

    Ok(Json(ApiResponse {
        data: result,
        pagination: None,
    }))
}

/// Get all reserved tokens for a user across all pools
pub async fn get_user_spt_reservations(
    State(db): State<Arc<Database>>,
    Path(address): Path<String>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<UserReservationInfo>>>, StatusCode> {
    let limit = pagination.get_limit();
    let offset = pagination.get_offset();

    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    #[derive(Debug, QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct ReservationRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        pool_id: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        associated_id: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        amount: i64,
        #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
        fee_amount: Option<i64>,
        #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
        creator_fee: Option<i64>,
        #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
        platform_fee: Option<i64>,
        #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
        treasury_fee: Option<i64>,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        reserved_at: i64,
    }

    // Get reservations - latest per pool for this user
    let reservation_rows = diesel::sql_query(
        r#"
        WITH latest_reservations AS (
            SELECT DISTINCT ON (r.pool_id, r.reserver_address) 
                r.pool_id,
                r.reserver_address,
                r.amount,
                r.fee_amount,
                r.creator_fee,
                r.platform_fee,
                r.treasury_fee,
                r.reserved_at,
                rp.associated_id
            FROM spt_reservations r
            LEFT JOIN spt_reservation_pools rp ON r.pool_id = rp.pool_id
            WHERE r.reserver_address = $1
            ORDER BY r.pool_id, r.reserver_address, r.time DESC
        )
        SELECT 
            pool_id,
            COALESCE(associated_id, '') as associated_id,
            amount,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
            reserved_at
        FROM latest_reservations
        WHERE amount > 0
        ORDER BY amount DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(address.clone())
    .bind::<diesel::sql_types::BigInt, _>(limit)
    .bind::<diesel::sql_types::BigInt, _>(offset)
    .load::<ReservationRow>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error getting reservations: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Convert to UserReservationInfo
    let reservations: Vec<UserReservationInfo> = reservation_rows
        .into_iter()
        .map(|r| UserReservationInfo {
            pool_id: r.pool_id,
            associated_id: r.associated_id,
            amount: r.amount,
            fee_amount: r.fee_amount,
            creator_fee: r.creator_fee,
            platform_fee: r.platform_fee,
            treasury_fee: r.treasury_fee,
            reserved_at: r.reserved_at,
        })
        .collect();

    // Count total for pagination
    let total_count = diesel::sql_query(
        r#"
        WITH latest_reservations AS (
            SELECT DISTINCT ON (r.pool_id, r.reserver_address) 
                r.pool_id,
                r.reserver_address,
                r.amount
            FROM spt_reservations r
            WHERE r.reserver_address = $1
            ORDER BY r.pool_id, r.reserver_address, r.time DESC
        )
        SELECT COUNT(*) as count
        FROM latest_reservations
        WHERE amount > 0
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(address)
    .get_result::<CountResult>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error in count query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total = total_count.count;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(ApiResponse {
        data: reservations,
        pagination: Some(PaginationInfo {
            page: pagination.get_page(),
            limit,
            total,
            total_pages,
        }),
    }))
}

/// Get popular token pools
pub async fn get_popular_tokens(
    State(db): State<Arc<Database>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<PopularTokenPool>>>, StatusCode> {
    let limit = pagination.get_limit();
    let offset = pagination.get_offset();

    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get popular tokens from the view
    let tokens = diesel::sql_query(
        r#"
        SELECT * FROM popular_token_pools
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind::<diesel::sql_types::BigInt, _>(limit)
    .bind::<diesel::sql_types::BigInt, _>(offset)
    .load::<PopularTokenPool>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Count total for pagination - this is a view so count is the total rows
    let total_count = diesel::sql_query(
        r#"
        SELECT COUNT(*) as count
        FROM popular_token_pools
        "#,
    )
    .get_result::<CountResult>(&mut conn)
    .await
    .map_err(|e| {
        error!("Database error in count query: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let total = total_count.count;
    let total_pages = (total + limit - 1) / limit;

    Ok(Json(ApiResponse {
        data: tokens,
        pagination: Some(PaginationInfo {
            page: pagination.get_page(),
            limit,
            total,
            total_pages,
        }),
    }))
}

/// Get token performance analytics based on time period
pub async fn get_top_performing_tokens(
    State(db): State<Arc<Database>>,
    Query(params): Query<TimePeriodParams>,
) -> Result<Json<ApiResponse<Vec<TokenPerformance>>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Determine time period for comparison
    let period_interval = match params.period.as_deref() {
        Some("day") => "1 day",
        Some("week") => "7 days",
        Some("month") => "30 days",
        _ => "1 day", // Default to daily comparison
    };

    // Execute query to get top performing tokens
    let query = diesel::sql_query(&format!(
        r#"
        WITH current_prices AS (
            SELECT DISTINCT ON (ph.pool_id) 
                ph.pool_id, 
                ph.price as current_price,
                p.name,
                p.symbol,
                p.token_type,
                SUM(t.mys_amount) OVER (PARTITION BY ph.pool_id) as current_volume
            FROM spt_price_history ph
            JOIN spt_pools p ON ph.pool_id = p.pool_id
            LEFT JOIN spt_transactions t ON ph.pool_id = t.pool_id 
                                        AND t.time > NOW() - INTERVAL '{}'
            WHERE ph.time = (
                SELECT MAX(time) FROM spt_price_history 
                WHERE pool_id = ph.pool_id
            )
        ),
        previous_prices AS (
            SELECT DISTINCT ON (ph.pool_id) 
                ph.pool_id, 
                ph.price as previous_price,
                SUM(t.mys_amount) OVER (PARTITION BY ph.pool_id) as previous_volume
            FROM spt_price_history ph
            LEFT JOIN spt_transactions t ON ph.pool_id = t.pool_id 
                                        AND t.time BETWEEN NOW() - INTERVAL '{}' * 2 AND NOW() - INTERVAL '{}'
            WHERE ph.time BETWEEN NOW() - INTERVAL '{}' * 2 AND NOW() - INTERVAL '{}'
        )
        SELECT 
            c.pool_id,
            c.name,
            c.symbol,
            c.current_price,
            p.previous_price,
            COALESCE(c.current_volume, 0) as current_volume,
            COALESCE(p.previous_volume, 0) as previous_volume,
            CASE 
                WHEN p.previous_price = 0 THEN 0
                ELSE (c.current_price - p.previous_price) * 100.0 / p.previous_price 
            END as price_change_percentage,
            CASE 
                WHEN COALESCE(p.previous_volume, 0) = 0 THEN 0
                ELSE (COALESCE(c.current_volume, 0) - COALESCE(p.previous_volume, 0)) * 100.0 / COALESCE(p.previous_volume, 1)
            END as volume_change_percentage
        FROM current_prices c
        LEFT JOIN previous_prices p ON c.pool_id = p.pool_id
        ORDER BY price_change_percentage DESC
        LIMIT 50
        "#,
        period_interval, period_interval, period_interval, period_interval, period_interval
    ));

    #[derive(diesel::QueryableByName)]
    struct TokenPerformanceRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        pool_id: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        name: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        symbol: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        current_price: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        previous_price: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        current_volume: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        previous_volume: i64,
        #[diesel(sql_type = diesel::sql_types::Double)]
        price_change_percentage: f64,
        #[diesel(sql_type = diesel::sql_types::Double)]
        volume_change_percentage: f64,
    }

    let performance_rows = query
        .load::<TokenPerformanceRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Convert to response format
    let performances = performance_rows
        .into_iter()
        .map(|row| TokenPerformance {
            pool_id: row.pool_id,
            name: row.name,
            symbol: row.symbol,
            price_change_percentage: row.price_change_percentage,
            volume_change_percentage: row.volume_change_percentage,
            current_price: row.current_price,
            previous_price: row.previous_price,
            current_volume: row.current_volume,
            previous_volume: row.previous_volume,
        })
        .collect();

    Ok(Json(ApiResponse {
        data: performances,
        pagination: None,
    }))
}

/// Get user's token portfolio performance
pub async fn get_user_portfolio_performance(
    State(db): State<Arc<Database>>,
    Path(address): Path<String>,
    Query(time_range): Query<TimeRangeParams>,
) -> Result<Json<ApiResponse<PortfolioPerformance>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get current holdings with value
    let holdings_query = diesel::sql_query(
        r#"
        WITH latest_holdings AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_holdings
            WHERE holder_address = $1
            ORDER BY pool_id, time DESC
        ),
        initial_transactions AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_transactions
            WHERE sender = $1 AND transaction_type = 'BUY'
            ORDER BY pool_id, time ASC
        ),
        current_prices AS (
            SELECT DISTINCT ON (pool_id) pool_id, price
            FROM spt_price_history
            ORDER BY pool_id, time DESC
        ),
        pool_info AS (
            SELECT DISTINCT ON (pool_id) pool_id, name, symbol
            FROM spt_pools
            ORDER BY pool_id, time DESC
        )
        SELECT 
            h.pool_id,
            p.name,
            p.symbol,
            h.amount,
            h.amount * cp.price as current_value,
            COALESCE(it.price * h.amount, 0) as initial_value,
            CASE 
                WHEN COALESCE(it.price * h.amount, 0) = 0 THEN 0
                ELSE ((h.amount * cp.price) - (it.price * h.amount)) * 100.0 / (it.price * h.amount)
            END as roi_percentage
        FROM latest_holdings h
        JOIN pool_info p ON h.pool_id = p.pool_id
        JOIN current_prices cp ON h.pool_id = cp.pool_id
        LEFT JOIN initial_transactions it ON h.pool_id = it.pool_id
        WHERE h.amount > 0
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(address.clone());

    #[derive(diesel::QueryableByName)]
    struct PortfolioHoldingRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        pool_id: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        name: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        symbol: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        amount: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        current_value: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        initial_value: i64,
        #[diesel(sql_type = diesel::sql_types::Double)]
        roi_percentage: f64,
    }

    let holding_rows = holdings_query
        .load::<PortfolioHoldingRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let holdings = holding_rows
        .into_iter()
        .map(|row| PortfolioHolding {
            pool_id: row.pool_id,
            name: row.name,
            symbol: row.symbol,
            amount: row.amount,
            current_value: row.current_value,
            initial_value: row.initial_value,
            roi_percentage: row.roi_percentage,
        })
        .collect::<Vec<_>>();

    // Get portfolio value over time
    let from_date = time_range
        .from
        .map(|ts| {
            chrono::DateTime::<chrono::Utc>::from_timestamp(ts, 0)
                .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(30))
        })
        .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(30));

    let to_date = time_range
        .to
        .map(|ts| {
            chrono::DateTime::<chrono::Utc>::from_timestamp(ts, 0)
                .unwrap_or_else(|| chrono::Utc::now())
        })
        .unwrap_or_else(|| chrono::Utc::now());

    // Determine interval based on time range
    let interval = if (to_date - from_date).num_days() > 30 {
        "1 day"
    } else if (to_date - from_date).num_days() > 7 {
        "4 hours"
    } else {
        "1 hour"
    };

    let value_history_query = diesel::sql_query(
        r#"
        WITH time_points AS (
            SELECT generate_series(
                $2::TIMESTAMPTZ,
                $3::TIMESTAMPTZ,
                $4::INTERVAL
            ) as point_time
        ),
        holdings_over_time AS (
            SELECT 
                tp.point_time,
                h.pool_id,
                h.amount,
                COALESCE(ph.price, 0) as price
            FROM time_points tp
            CROSS JOIN (
                SELECT DISTINCT pool_id 
                FROM spt_holdings 
                WHERE holder_address = $1
            ) distinct_pools
            LEFT JOIN LATERAL (
                SELECT * 
                FROM spt_holdings
                WHERE holder_address = $1 
                  AND pool_id = distinct_pools.pool_id
                  AND time <= tp.point_time
                ORDER BY time DESC
                LIMIT 1
            ) h ON true
            LEFT JOIN LATERAL (
                SELECT price
                FROM spt_price_history
                WHERE pool_id = distinct_pools.pool_id
                  AND time <= tp.point_time
                ORDER BY time DESC
                LIMIT 1
            ) ph ON true
            WHERE h.amount > 0
        )
        SELECT 
            EXTRACT(EPOCH FROM point_time)::BIGINT as timestamp,
            SUM(COALESCE(amount * price, 0)) as value
        FROM holdings_over_time
        GROUP BY point_time
        ORDER BY point_time
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(address.clone())
    .bind::<diesel::sql_types::Timestamptz, _>(from_date)
    .bind::<diesel::sql_types::Timestamptz, _>(to_date)
    .bind::<diesel::sql_types::Text, _>(interval);

    #[derive(diesel::QueryableByName)]
    struct ValueHistoryRow {
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        timestamp: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        value: i64,
    }

    let value_history_rows = value_history_query
        .load::<ValueHistoryRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let value_history = value_history_rows
        .into_iter()
        .map(|row| PortfolioValuePoint {
            timestamp: row.timestamp,
            value: row.value,
        })
        .collect::<Vec<_>>();

    // Calculate overall portfolio metrics
    let current_value: i64 = holdings.iter().map(|h| h.current_value).sum();
    let initial_investment: i64 = holdings.iter().map(|h| h.initial_value).sum();
    let roi_percentage = if initial_investment > 0 {
        ((current_value as f64 - initial_investment as f64) / initial_investment as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(ApiResponse {
        data: PortfolioPerformance {
            address,
            current_value,
            initial_investment,
            roi_percentage,
            holdings,
            value_history,
        },
        pagination: None,
    }))
}

/// Get creator revenue dashboard
pub async fn get_creator_revenue_streams(
    State(db): State<Arc<Database>>,
    Path(address): Path<String>,
    Query(time_range): Query<TimeRangeParams>,
) -> Result<Json<ApiResponse<CreatorRevenueReport>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Set up time range parameters
    let from_timestamp = time_range
        .from
        .map(|ts| {
            chrono::DateTime::<chrono::Utc>::from_timestamp(ts, 0)
                .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(30))
        })
        .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(30));

    let to_timestamp = time_range
        .to
        .map(|ts| {
            chrono::DateTime::<chrono::Utc>::from_timestamp(ts, 0)
                .unwrap_or_else(|| chrono::Utc::now())
        })
        .unwrap_or_else(|| chrono::Utc::now());

    // Get revenue by token pool
    let token_revenue_query = diesel::sql_query(
        r#"
        WITH token_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_pools
            WHERE owner = $1
            ORDER BY pool_id, time DESC
        ),
        buy_transactions AS (
            SELECT 
                pool_id, 
                SUM(creator_fee) as buy_revenue,
                COUNT(*) as buy_count
            FROM spt_transactions
            WHERE transaction_type = 'BUY'
              AND pool_id IN (SELECT pool_id FROM token_pools)
              AND time >= $2
              AND time <= $3
            GROUP BY pool_id
        ),
        sell_transactions AS (
            SELECT 
                pool_id, 
                SUM(creator_fee) as sell_revenue,
                COUNT(*) as sell_count
            FROM spt_transactions
            WHERE transaction_type = 'SELL'
              AND pool_id IN (SELECT pool_id FROM token_pools)
              AND time >= $2
              AND time <= $3
            GROUP BY pool_id
        )
        SELECT 
            tp.pool_id,
            tp.name,
            tp.symbol,
            COALESCE(bt.buy_revenue, 0) as buy_revenue,
            COALESCE(st.sell_revenue, 0) as sell_revenue,
            COALESCE(bt.buy_revenue, 0) + COALESCE(st.sell_revenue, 0) as total_revenue,
            COALESCE(bt.buy_count, 0) + COALESCE(st.sell_count, 0) as transactions_count
        FROM token_pools tp
        LEFT JOIN buy_transactions bt ON tp.pool_id = bt.pool_id
        LEFT JOIN sell_transactions st ON tp.pool_id = st.pool_id
        ORDER BY total_revenue DESC
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(address.clone())
    .bind::<diesel::sql_types::Timestamptz, _>(from_timestamp)
    .bind::<diesel::sql_types::Timestamptz, _>(to_timestamp);

    #[derive(diesel::QueryableByName)]
    struct TokenRevenueRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        pool_id: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        name: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        symbol: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        buy_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        sell_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        transactions_count: i64,
    }

    let token_revenue_rows = token_revenue_query
        .load::<TokenRevenueRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let token_pools = token_revenue_rows
        .into_iter()
        .map(|row| CreatorTokenRevenue {
            pool_id: row.pool_id,
            name: row.name,
            symbol: row.symbol,
            total_revenue: row.total_revenue,
            buy_revenue: row.buy_revenue,
            sell_revenue: row.sell_revenue,
            transactions_count: row.transactions_count,
        })
        .collect::<Vec<_>>();

    // Determine appropriate interval based on date range
    let days_difference = (to_timestamp - from_timestamp).num_days();
    let interval = if days_difference > 90 {
        "1 week"
    } else if days_difference > 30 {
        "1 day"
    } else {
        "4 hours"
    };

    // Get revenue by time period
    let period_revenue_query = diesel::sql_query(
        r#"
        WITH time_periods AS (
            SELECT generate_series(
                $2::TIMESTAMPTZ,
                $3::TIMESTAMPTZ,
                $4::INTERVAL
            ) as period_start
        ),
        creator_pools AS (
            SELECT DISTINCT pool_id
            FROM spt_pools
            WHERE owner = $1
        ),
        period_revenues AS (
            SELECT 
                p.period_start,
                SUM(t.creator_fee) as revenue
            FROM time_periods p
            LEFT JOIN spt_transactions t ON 
                t.time >= p.period_start AND 
                t.time < p.period_start + ($4::INTERVAL) AND
                t.pool_id IN (SELECT pool_id FROM creator_pools)
            GROUP BY p.period_start
            ORDER BY p.period_start
        )
        SELECT 
            EXTRACT(EPOCH FROM period_start)::BIGINT as period_start,
            revenue
        FROM period_revenues
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(address.clone())
    .bind::<diesel::sql_types::Timestamptz, _>(from_timestamp)
    .bind::<diesel::sql_types::Timestamptz, _>(to_timestamp)
    .bind::<diesel::sql_types::Text, _>(interval);

    #[derive(diesel::QueryableByName)]
    struct PeriodRevenueRow {
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        period_start: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        revenue: i64,
    }

    let period_revenue_rows = period_revenue_query
        .load::<PeriodRevenueRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let revenue_by_period = period_revenue_rows
        .into_iter()
        .map(|row| RevenuePeriod {
            period_start: row.period_start,
            revenue: row.revenue,
        })
        .collect::<Vec<_>>();

    // Calculate total revenue
    let total_revenue: i64 = token_pools.iter().map(|t| t.total_revenue).sum();

    Ok(Json(ApiResponse {
        data: CreatorRevenueReport {
            address,
            total_revenue,
            token_pools,
            revenue_by_period,
        },
        pagination: None,
    }))
}

/// Get market sentiment indicators
pub async fn get_market_sentiment(
    State(db): State<Arc<Database>>,
) -> Result<Json<ApiResponse<MarketSentiment>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get overall market metrics for last 24 hours
    let market_metrics_query = diesel::sql_query(
        r#"
        WITH current_volume AS (
            SELECT 
                SUM(CASE WHEN transaction_type = 'BUY' THEN mys_amount ELSE 0 END) as buy_volume,
                SUM(CASE WHEN transaction_type = 'SELL' THEN mys_amount ELSE 0 END) as sell_volume,
                COUNT(*) as transaction_count,
                COUNT(DISTINCT CASE WHEN transaction_type = 'BUY' THEN sender END) as unique_buyers,
                COUNT(DISTINCT CASE WHEN transaction_type = 'SELL' THEN sender END) as unique_sellers
            FROM spt_transactions
            WHERE time > NOW() - INTERVAL '24 hours'
        ),
        previous_volume AS (
            SELECT 
                SUM(mys_amount) as total_volume
            FROM spt_transactions
            WHERE time BETWEEN NOW() - INTERVAL '48 hours' AND NOW() - INTERVAL '24 hours'
        )
        SELECT 
            c.buy_volume,
            c.sell_volume,
            c.transaction_count,
            c.unique_buyers,
            c.unique_sellers,
            CASE 
                WHEN COALESCE(p.total_volume, 0) = 0 THEN 0
                ELSE ((c.buy_volume + c.sell_volume) - COALESCE(p.total_volume, 0)) * 100.0 / COALESCE(p.total_volume, 1)
            END as volume_change_percentage,
            CASE
                WHEN (c.buy_volume + c.sell_volume) = 0 THEN 0
                ELSE (c.buy_volume - c.sell_volume) * 1.0 / (c.buy_volume + c.sell_volume)
            END as sentiment_score
        FROM current_volume c
        CROSS JOIN previous_volume p
        "#,
    );

    #[derive(diesel::QueryableByName)]
    struct MarketMetricsRow {
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        buy_volume: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        sell_volume: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        transaction_count: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_buyers: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_sellers: i64,
        #[diesel(sql_type = diesel::sql_types::Double)]
        volume_change_percentage: f64,
        #[diesel(sql_type = diesel::sql_types::Double)]
        sentiment_score: f64,
    }

    let market_metrics = market_metrics_query
        .get_result::<MarketMetricsRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get sentiment by token type
    let token_type_sentiment_query = diesel::sql_query(
        r#"
        WITH token_type_metrics AS (
            SELECT 
                p.token_type,
                SUM(CASE WHEN t.transaction_type = 'BUY' THEN t.mys_amount ELSE 0 END) as buy_volume,
                SUM(CASE WHEN t.transaction_type = 'SELL' THEN t.mys_amount ELSE 0 END) as sell_volume,
                SUM(t.mys_amount) as current_volume
            FROM spt_transactions t
            JOIN spt_pools p ON t.pool_id = p.pool_id
            WHERE t.time > NOW() - INTERVAL '24 hours'
            GROUP BY p.token_type
        ),
        previous_volumes AS (
            SELECT 
                p.token_type,
                SUM(t.mys_amount) as previous_volume
            FROM spt_transactions t
            JOIN spt_pools p ON t.pool_id = p.pool_id
            WHERE t.time BETWEEN NOW() - INTERVAL '48 hours' AND NOW() - INTERVAL '24 hours'
            GROUP BY p.token_type
        )
        SELECT 
            c.token_type,
            CASE
                WHEN (c.buy_volume + c.sell_volume) = 0 THEN 0
                ELSE (c.buy_volume - c.sell_volume) * 1.0 / (c.buy_volume + c.sell_volume)
            END as sentiment_score,
            CASE 
                WHEN COALESCE(p.previous_volume, 0) = 0 THEN 0
                ELSE (c.current_volume - COALESCE(p.previous_volume, 0)) * 100.0 / COALESCE(p.previous_volume, 1)
            END as volume_change
        FROM token_type_metrics c
        LEFT JOIN previous_volumes p ON c.token_type = p.token_type
        "#,
    );

    #[derive(diesel::QueryableByName)]
    struct TokenTypeSentimentRow {
        #[diesel(sql_type = diesel::sql_types::SmallInt)]
        token_type: i16,
        #[diesel(sql_type = diesel::sql_types::Double)]
        sentiment_score: f64,
        #[diesel(sql_type = diesel::sql_types::Double)]
        volume_change: f64,
    }

    let token_type_sentiment_rows = token_type_sentiment_query
        .load::<TokenTypeSentimentRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let price_momentum = token_type_sentiment_rows
        .into_iter()
        .map(|row| MomentumIndicator {
            token_type: row.token_type,
            sentiment_score: row.sentiment_score,
            volume_change: row.volume_change,
        })
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse {
        data: MarketSentiment {
            overall_sentiment: market_metrics.sentiment_score,
            buy_volume_24h: market_metrics.buy_volume,
            sell_volume_24h: market_metrics.sell_volume,
            transaction_count_24h: market_metrics.transaction_count,
            unique_buyers_24h: market_metrics.unique_buyers,
            unique_sellers_24h: market_metrics.unique_sellers,
            volume_change_percentage: market_metrics.volume_change_percentage,
            price_momentum,
        },
        pagination: None,
    }))
}

/// Get token liquidity profile
pub async fn get_token_liquidity_profile(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<TokenLiquidityProfile>>, StatusCode> {
    // Get a connection from the pool
    let mut conn = db.get_connection().await.map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get token pool info
    let pool_query = diesel::sql_query(
        r#"
        SELECT name, symbol
        FROM spt_pools
        WHERE pool_id = $1
        ORDER BY time DESC
        LIMIT 1
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(id.clone());

    #[derive(diesel::QueryableByName)]
    struct TokenPoolInfoRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        name: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        symbol: String,
    }

    let pool_info = pool_query
        .get_result::<TokenPoolInfoRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get liquidity metrics
    let liquidity_metrics_query = diesel::sql_query(
        r#"
        WITH transaction_metrics AS (
            SELECT 
                SUM(mys_amount) as total_volume,
                COUNT(*) as transaction_count,
                MAX(mys_amount) as largest_transaction,
                COUNT(DISTINCT sender) as unique_traders_count,
                SUM(CASE WHEN transaction_type = 'BUY' THEN mys_amount ELSE 0 END) as buy_volume,
                SUM(CASE WHEN transaction_type = 'SELL' THEN mys_amount ELSE 0 END) as sell_volume
            FROM spt_transactions
            WHERE pool_id = $1
              AND time > NOW() - INTERVAL '24 hours'
        )
        SELECT 
            total_volume,
            transaction_count,
            CASE WHEN transaction_count = 0 THEN 0 ELSE total_volume / transaction_count END as average_transaction_size,
            largest_transaction,
            unique_traders_count,
            CASE 
                WHEN (buy_volume + sell_volume) = 0 THEN 0
                ELSE buy_volume * 1.0 / NULLIF(sell_volume, 0)
            END as buy_sell_ratio
        FROM transaction_metrics
        "#
    )
    .bind::<diesel::sql_types::Text, _>(id.clone());

    #[derive(diesel::QueryableByName)]
    struct LiquidityMetricsRow {
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_volume: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        transaction_count: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        average_transaction_size: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        largest_transaction: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_traders_count: i64,
        #[diesel(sql_type = diesel::sql_types::Double)]
        buy_sell_ratio: f64,
    }

    let liquidity_metrics = liquidity_metrics_query
        .get_result::<LiquidityMetricsRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get hourly volume distribution
    let volume_distribution_query = diesel::sql_query(
        r#"
        SELECT 
            EXTRACT(EPOCH FROM time_bucket('1 hour', time))::BIGINT as hour,
            SUM(mys_amount) as volume
        FROM spt_transactions
        WHERE pool_id = $1
          AND time > NOW() - INTERVAL '24 hours'
        GROUP BY hour
        ORDER BY hour
        "#,
    )
    .bind::<diesel::sql_types::Text, _>(id.clone());

    #[derive(diesel::QueryableByName)]
    struct VolumeDistributionRow {
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        hour: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        volume: i64,
    }

    let volume_distribution_rows = volume_distribution_query
        .load::<VolumeDistributionRow>(&mut conn)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let volume_distribution = volume_distribution_rows
        .into_iter()
        .map(|row| VolumeDistribution {
            hour: row.hour,
            volume: row.volume,
        })
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse {
        data: TokenLiquidityProfile {
            pool_id: id,
            name: pool_info.name,
            symbol: pool_info.symbol,
            total_volume_24h: liquidity_metrics.total_volume,
            transaction_count_24h: liquidity_metrics.transaction_count,
            average_transaction_size: liquidity_metrics.average_transaction_size,
            largest_transaction: liquidity_metrics.largest_transaction,
            unique_traders_count: liquidity_metrics.unique_traders_count,
            buy_sell_ratio: liquidity_metrics.buy_sell_ratio,
            volume_distribution,
        },
        pagination: None,
    }))
}

/// SPT Configuration response structure
#[derive(Debug, Serialize, QueryableByName)]
#[diesel(check_for_backend(Pg))]
pub struct SptConfigInfo {
    #[diesel(sql_type = Text)]
    pub updated_by: String,

    #[diesel(sql_type = BigInt)]
    pub post_threshold: i64,

    #[diesel(sql_type = BigInt)]
    pub profile_threshold: i64,

    #[diesel(sql_type = BigInt)]
    pub max_individual_reservation_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub total_fee_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub creator_fee_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub platform_fee_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub treasury_fee_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub trading_creator_fee_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub trading_platform_fee_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub trading_treasury_fee_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub reservation_creator_fee_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub reservation_platform_fee_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub reservation_treasury_fee_bps: i64,

    #[diesel(sql_type = BigInt)]
    pub max_reservers_per_pool: i64,

    #[diesel(sql_type = BigInt)]
    pub base_price: i64,

    #[diesel(sql_type = BigInt)]
    pub quadratic_coefficient: i64,

    #[diesel(sql_type = BigInt)]
    pub max_hold_percent_bps: i64,

    #[diesel(sql_type = Bool)]
    pub trading_enabled: bool,

    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,

    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,

    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// Get current social proof tokens configuration
pub async fn get_spt_configuration(State(db): State<Arc<Database>>) -> Response {
    let mut conn = match db.get_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Database connection error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                })),
            )
                .into_response();
        }
    };

    let query = "
        SELECT 
            updated_by,
            post_threshold,
            profile_threshold,
            max_individual_reservation_bps,
            total_fee_bps,
            creator_fee_bps,
            platform_fee_bps,
            treasury_fee_bps,
            trading_creator_fee_bps,
            trading_platform_fee_bps,
            trading_treasury_fee_bps,
            reservation_creator_fee_bps,
            reservation_platform_fee_bps,
            reservation_treasury_fee_bps,
            max_reservers_per_pool,
            base_price,
            quadratic_coefficient,
            max_hold_percent_bps,
            trading_enabled,
            updated_at,
            time,
            transaction_id
        FROM spt_exchange_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<SptConfigInfo>(&mut conn)
        .await;

    match result {
        Ok(config) => {
            // Query current treasury address from ecosystem_treasury table
            let treasury_address = match crate::social::models::get_current_treasury_address(&mut conn).await {
                Ok(addr) => addr,
                Err(e) => {
                    error!("Failed to get current treasury address: {}", e);
                    // Return error if treasury not found
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("Failed to get treasury address: {}", e)
                        })),
                    )
                        .into_response();
                }
            };

            // Add treasury address to response
            let mut config_json = match serde_json::to_value(&config) {
                Ok(json) => json,
                Err(e) => {
                    error!("Failed to serialize config: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("Failed to serialize config: {}", e)
                        })),
                    )
                        .into_response();
                }
            };

            if let Some(obj) = config_json.as_object_mut() {
                obj.insert("ecosystem_treasury".to_string(), serde_json::Value::String(treasury_address));
            }

            Json(config_json).into_response()
        },
        Err(diesel::result::Error::NotFound) => {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Social proof tokens configuration not found"
                })),
            )
                .into_response()
        }
        Err(e) => {
            error!("Database error getting SPT configuration: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database error: {}", e)
                })),
            )
                .into_response()
        }
    }
}

// ============================================================================
// RESERVATION POOL INFO FOR PROFILES
// ============================================================================

/// Reservation pool information for a profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationPoolInfo {
    pub claimed_percentage: f64,
    pub is_active: bool,
    pub total_reserved: i64,
    pub required_threshold: i64,
    pub pool_id: Option<String>,
}

impl Default for ReservationPoolInfo {
    fn default() -> Self {
        Self {
            claimed_percentage: 0.0,
            is_active: false,
            total_reserved: 0,
            required_threshold: 0,
            pool_id: None,
        }
    }
}

/// Get reservation pool information for multiple profiles by wallet address
/// Returns a HashMap mapping owner_address -> ReservationPoolInfo
pub async fn get_reservation_pool_info_for_profiles(
    wallet_addresses: Vec<String>,
    conn: &mut diesel_async::AsyncPgConnection,
) -> Result<HashMap<String, ReservationPoolInfo>, diesel::result::Error> {
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    if wallet_addresses.is_empty() {
        return Ok(HashMap::new());
    }

    // Get latest exchange config for profile_threshold
    let profile_threshold = crate::social::schema::spt_exchange_config::table
        .order_by(crate::social::schema::spt_exchange_config::time.desc())
        .select(crate::social::schema::spt_exchange_config::profile_threshold)
        .first::<i64>(conn)
        .await
        .unwrap_or(10000); // Fallback to 10000 if config doesn't exist

    // Build associated_id values: 'profile_' || owner_address
    let associated_ids: Vec<String> = wallet_addresses
        .iter()
        .map(|addr| format!("profile_{}", addr))
        .collect();

    // Query for latest reservation pools using raw SQL for better performance
    let query = diesel::sql_query(
        r#"
        WITH latest_pools AS (
            SELECT DISTINCT ON (associated_id) 
                associated_id,
                pool_id,
                total_reserved,
                required_threshold,
                status,
                time
            FROM spt_reservation_pools
            WHERE associated_id = ANY($1::TEXT[])
            ORDER BY associated_id, time DESC
        )
        SELECT 
            associated_id,
            pool_id,
            total_reserved,
            required_threshold,
            status
        FROM latest_pools
        "#,
    )
    .bind::<diesel::sql_types::Array<diesel::sql_types::Text>, _>(&associated_ids);

    #[derive(QueryableByName)]
    struct PoolRow {
        #[diesel(sql_type = Text)]
        associated_id: String,
        #[diesel(sql_type = Text)]
        pool_id: String,
        #[diesel(sql_type = BigInt)]
        total_reserved: i64,
        #[diesel(sql_type = BigInt)]
        required_threshold: i64,
        #[diesel(sql_type = Text)]
        status: String,
    }

    let pools: Vec<PoolRow> = query.load::<PoolRow>(conn).await?;

    // Build HashMap: extract owner_address from associated_id and calculate percentage
    let mut result = HashMap::new();
    for pool in pools {
        // Extract owner_address from associated_id (remove 'profile_' prefix)
        if let Some(owner_address) = pool.associated_id.strip_prefix("profile_") {
            let claimed_percentage = if profile_threshold > 0 {
                (pool.total_reserved as f64 / profile_threshold as f64) * 100.0
            } else {
                0.0
            };

            let is_active = pool.status == "active" && pool.total_reserved < pool.required_threshold;

            result.insert(
                owner_address.to_string(),
                ReservationPoolInfo {
                    claimed_percentage: claimed_percentage.min(100.0).max(0.0),
                    is_active,
                    total_reserved: pool.total_reserved,
                    required_threshold: pool.required_threshold,
                    pool_id: Some(pool.pool_id),
                },
            );
        }
    }

    Ok(result)
}
