// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
};
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;

// Query parameters for marketplace data listing
#[derive(Debug, Deserialize)]
pub struct MarketplaceQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub creator: Option<String>,
    pub media_type: Option<String>,
    pub tags: Option<String>,
    pub platform_id: Option<String>,
    pub min_price: Option<i64>,
    pub max_price: Option<i64>,
    pub data_quality: Option<String>,
    pub geographic_region: Option<String>,
    pub is_free: Option<bool>,
    pub sort_by: Option<String>, // "created_at", "price", "revenue", "popularity"
}

// Combined stats for marketplace data
#[derive(Debug, Serialize)]
pub struct MarketplaceStatsResponse {
    pub ip_id: String,
    pub owner: String,
    pub media_type: String,
    pub total_revenue: i64,
    pub purchase_count: i64,
    pub subscription_count: i64,
    pub access_count: i64,
    pub one_time_price: Option<i64>,
    pub subscription_price: Option<i64>,
    pub created_at: i64,
    pub last_updated: i64,
}

// Revenue analytics data
#[derive(Debug, Serialize, QueryableByName)]
pub struct DailyRevenue {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = BigInt)]
    pub daily_revenue: i64,
    #[diesel(sql_type = BigInt)]
    pub daily_transactions: i64,
}

// Access analytics data
#[derive(Debug, Serialize, QueryableByName)]
pub struct AccessAnalytics {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = Text)]
    pub access_type: String,
    #[diesel(sql_type = BigInt)]
    pub unique_users: i64,
    #[diesel(sql_type = BigInt)]
    pub total_accesses: i64,
}

// Basic marketplace data returned for list operations
#[derive(Debug, Serialize, QueryableByName)]
pub struct MarketplaceDataBasic {
    #[diesel(sql_type = Text)]
    pub ip_id: String,
    
    #[diesel(sql_type = Text)]
    pub owner: String,
    
    #[diesel(sql_type = Text)]
    pub media_type: String,
    
    #[diesel(sql_type = Jsonb)]
    pub tags: serde_json::Value,
    
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_id: Option<String>,
    
    #[diesel(sql_type = BigInt)]
    pub timestamp_start: i64,
    
    #[diesel(sql_type = Nullable<BigInt>)]
    pub timestamp_end: Option<i64>,
    
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    
    #[diesel(sql_type = BigInt)]
    pub last_updated: i64,
    
    #[diesel(sql_type = Nullable<BigInt>)]
    pub one_time_price: Option<i64>,
    
    #[diesel(sql_type = Nullable<BigInt>)]
    pub subscription_price: Option<i64>,
    
    #[diesel(sql_type = BigInt)]
    pub subscription_duration_days: i64,
    
    #[diesel(sql_type = Nullable<Text>)]
    pub geographic_region: Option<String>,
    
    #[diesel(sql_type = Nullable<Text>)]
    pub data_quality: Option<String>,
    
    #[diesel(sql_type = Nullable<BigInt>)]
    pub sample_size: Option<i64>,
    
    #[diesel(sql_type = Bool)]
    pub is_updating: bool,
    
    #[diesel(sql_type = Nullable<Text>)]
    pub update_frequency: Option<String>,
}

/// Get marketplace data by IP ID
pub async fn get_marketplace_data_by_id(
    State(pool): State<DbPool>,
    Path(ip_id): Path<String>,
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
        SELECT ip_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end, 
               created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
               geographic_region, data_quality, sample_size, is_updating, update_frequency
        FROM my_ip_data 
        WHERE ip_id = $1
    ";
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&ip_id)
        .get_result::<MarketplaceDataBasic>(&mut conn)
        .await;

    match result {
        Ok(data) => Json(data).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Marketplace data not found").into_response()
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

/// List marketplace data with filtering and pagination
pub async fn list_marketplace_data(
    State(pool): State<DbPool>, 
    Query(params): Query<MarketplaceQuery>
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
    
    // Build dynamic SQL query based on filters
    let mut query = "
        SELECT ip_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
               created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
               geographic_region, data_quality, sample_size, is_updating, update_frequency
        FROM my_ip_data WHERE 1=1".to_string();
    
    // Apply filters
    if let Some(creator) = &params.creator {
        query.push_str(&format!(" AND owner = '{}'", creator));
    }
    
    if let Some(media_type) = &params.media_type {
        query.push_str(&format!(" AND media_type = '{}'", media_type));
    }
    
    if let Some(platform_id) = &params.platform_id {
        query.push_str(&format!(" AND platform_id = '{}'", platform_id));
    }
    
    if let Some(data_quality) = &params.data_quality {
        query.push_str(&format!(" AND data_quality = '{}'", data_quality));
    }
    
    if let Some(geographic_region) = &params.geographic_region {
        query.push_str(&format!(" AND geographic_region = '{}'", geographic_region));
    }
    
    if let Some(min_price) = params.min_price {
        query.push_str(&format!(" AND (one_time_price >= {} OR subscription_price >= {})", min_price, min_price));
    }
    
    if let Some(max_price) = params.max_price {
        query.push_str(&format!(" AND (one_time_price <= {} OR subscription_price <= {})", max_price, max_price));
    }
    
    if let Some(is_free) = params.is_free {
        if is_free {
            query.push_str(" AND one_time_price IS NULL AND subscription_price IS NULL");
        } else {
            query.push_str(" AND (one_time_price IS NOT NULL OR subscription_price IS NOT NULL)");
        }
    }
    
    if let Some(tags) = &params.tags {
        query.push_str(&format!(" AND tags @> '[\"{}\"]\\'", tags));
    }
    
    // Apply sorting
    let sort_clause = match params.sort_by.as_deref() {
        Some("price") => " ORDER BY COALESCE(one_time_price, subscription_price) DESC",
        Some("created_at") => " ORDER BY created_at DESC",
        Some("updated") => " ORDER BY last_updated DESC",
        _ => " ORDER BY created_at DESC", // Default sort
    };
    query.push_str(sort_clause);
    
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    
    let result = diesel::sql_query(&query)
        .load::<MarketplaceDataBasic>(&mut conn)
        .await;
    
    match result {
        Ok(data) => Json(data).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

/// Get purchases for a specific IP
pub async fn get_ip_purchases(
    State(pool): State<DbPool>,
    Path(ip_id): Path<String>,
    Query(params): Query<MarketplaceQuery>,
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
        SELECT id, ip_id, buyer, price, purchase_type, purchase_time, time, transaction_id
        FROM my_ip_purchases 
        WHERE ip_id = $1 
        ORDER BY purchase_time DESC 
        LIMIT $2 OFFSET $3
    ";
    
    #[derive(QueryableByName, Serialize)]
    struct PurchaseInfo {
        #[diesel(sql_type = Integer)]
        id: i32,
        #[diesel(sql_type = Text)]
        ip_id: String,
        #[diesel(sql_type = Text)]
        buyer: String,
        #[diesel(sql_type = BigInt)]
        price: i64,
        #[diesel(sql_type = Text)]
        purchase_type: String,
        #[diesel(sql_type = BigInt)]
        purchase_time: i64,
        #[diesel(sql_type = Timestamptz)]
        time: chrono::DateTime<chrono::Utc>,
        #[diesel(sql_type = Text)]
        transaction_id: String,
    }
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&ip_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PurchaseInfo>(&mut conn)
        .await;
        
    match result {
        Ok(purchases) => Json(purchases).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

/// Get subscriptions for a specific IP
pub async fn get_ip_subscriptions(
    State(pool): State<DbPool>,
    Path(ip_id): Path<String>,
    Query(params): Query<MarketplaceQuery>,
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
        SELECT id, ip_id, subscriber, subscription_start, subscription_end, price, time, transaction_id
        FROM my_ip_subscriptions 
        WHERE ip_id = $1 
        ORDER BY subscription_start DESC 
        LIMIT $2 OFFSET $3
    ";
    
    #[derive(QueryableByName, Serialize)]
    struct SubscriptionInfo {
        #[diesel(sql_type = Integer)]
        id: i32,
        #[diesel(sql_type = Text)]
        ip_id: String,
        #[diesel(sql_type = Text)]
        subscriber: String,
        #[diesel(sql_type = BigInt)]
        subscription_start: i64,
        #[diesel(sql_type = BigInt)]
        subscription_end: i64,
        #[diesel(sql_type = BigInt)]
        price: i64,
        #[diesel(sql_type = Timestamptz)]
        time: chrono::DateTime<chrono::Utc>,
        #[diesel(sql_type = Text)]
        transaction_id: String,
    }
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&ip_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SubscriptionInfo>(&mut conn)
        .await;
        
    match result {
        Ok(subscriptions) => Json(subscriptions).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

/// Get revenue analytics for a specific IP
pub async fn get_ip_revenue(
    State(pool): State<DbPool>,
    Path(ip_id): Path<String>,
    Query(params): Query<MarketplaceQuery>,
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

    let limit = params.limit.unwrap_or(30).min(100);
    let offset = params.offset.unwrap_or(0);
    
    let query = "
        SELECT id, ip_id, from_address, to_address, amount, revenue_type, revenue_time, time, transaction_id
        FROM my_ip_revenue 
        WHERE ip_id = $1 
        ORDER BY revenue_time DESC 
        LIMIT $2 OFFSET $3
    ";
    
    #[derive(QueryableByName, Serialize)]
    struct RevenueInfo {
        #[diesel(sql_type = Integer)]
        id: i32,
        #[diesel(sql_type = Text)]
        ip_id: String,
        #[diesel(sql_type = Text)]
        from_address: String,
        #[diesel(sql_type = Text)]
        to_address: String,
        #[diesel(sql_type = BigInt)]
        amount: i64,
        #[diesel(sql_type = Text)]
        revenue_type: String,
        #[diesel(sql_type = BigInt)]
        revenue_time: i64,
        #[diesel(sql_type = Timestamptz)]
        time: chrono::DateTime<chrono::Utc>,
        #[diesel(sql_type = Text)]
        transaction_id: String,
    }
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&ip_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<RevenueInfo>(&mut conn)
        .await;
        
    match result {
        Ok(revenue) => Json(revenue).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

/// Get access logs for a specific IP  
pub async fn get_ip_access_logs(
    State(pool): State<DbPool>,
    Path(ip_id): Path<String>,
    Query(params): Query<MarketplaceQuery>,
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

    let limit = params.limit.unwrap_or(50).min(200);
    let offset = params.offset.unwrap_or(0);
    
    let query = "
        SELECT id, ip_id, user_address, access_type, access_time, time, transaction_id
        FROM my_ip_access_logs 
        WHERE ip_id = $1 
        ORDER BY access_time DESC 
        LIMIT $2 OFFSET $3
    ";
    
    #[derive(QueryableByName, Serialize)]
    struct AccessLogInfo {
        #[diesel(sql_type = Integer)]
        id: i32,
        #[diesel(sql_type = Text)]
        ip_id: String,
        #[diesel(sql_type = Text)]
        user_address: String,
        #[diesel(sql_type = Text)]
        access_type: String,
        #[diesel(sql_type = BigInt)]
        access_time: i64,
        #[diesel(sql_type = Timestamptz)]
        time: chrono::DateTime<chrono::Utc>,
        #[diesel(sql_type = Text)]
        transaction_id: String,
    }
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&ip_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<AccessLogInfo>(&mut conn)
        .await;
        
    match result {
        Ok(access_logs) => Json(access_logs).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

/// Get marketplace data created by a specific creator
pub async fn get_creator_data(
    State(pool): State<DbPool>,
    Path(creator): Path<String>,
    Query(params): Query<MarketplaceQuery>,
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
        SELECT ip_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
               created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
               geographic_region, data_quality, sample_size, is_updating, update_frequency
        FROM my_ip_data 
        WHERE owner = $1 
        ORDER BY created_at DESC 
        LIMIT $2 OFFSET $3
    ";
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&creator)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MarketplaceDataBasic>(&mut conn)
        .await;
    
    match result {
        Ok(data) => Json(data).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

/// Get comprehensive stats for a specific marketplace data
pub async fn get_marketplace_stats(
    State(pool): State<DbPool>,
    Path(ip_id): Path<String>,
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

    // First get the marketplace data basic info
    let data_query = "
        SELECT ip_id, owner, media_type, one_time_price, subscription_price, created_at, last_updated
        FROM my_ip_data 
        WHERE ip_id = $1
    ";
    
    #[derive(QueryableByName)]
    struct DataInfo {
        #[diesel(sql_type = Text)]
        ip_id: String,
        #[diesel(sql_type = Text)]
        owner: String,
        #[diesel(sql_type = Text)]
        media_type: String,
        #[diesel(sql_type = Nullable<BigInt>)]
        one_time_price: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        subscription_price: Option<i64>,
        #[diesel(sql_type = BigInt)]
        created_at: i64,
        #[diesel(sql_type = BigInt)]
        last_updated: i64,
    }
    
    let data_info = match diesel::sql_query(data_query)
        .bind::<Text, _>(&ip_id)
        .get_result::<DataInfo>(&mut conn)
        .await
    {
        Ok(data) => data,
        Err(diesel::result::Error::NotFound) => {
            return (StatusCode::NOT_FOUND, "Marketplace data not found").into_response();
        },
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response();
        }
    };
    
    // Get aggregated stats
    let stats_query = "
        SELECT 
            COALESCE(SUM(r.amount), 0) as total_revenue,
            (SELECT COUNT(*) FROM my_ip_purchases p WHERE p.ip_id = $1) as purchase_count,
            (SELECT COUNT(*) FROM my_ip_subscriptions s WHERE s.ip_id = $1) as subscription_count,
            (SELECT COUNT(*) FROM my_ip_access_logs a WHERE a.ip_id = $1) as access_count
        FROM 
            my_ip_revenue r
        WHERE 
            r.ip_id = $1
    ";
    
    #[derive(QueryableByName)]
    struct StatsInfo {
        #[diesel(sql_type = BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = BigInt)]
        purchase_count: i64,
        #[diesel(sql_type = BigInt)]
        subscription_count: i64,
        #[diesel(sql_type = BigInt)]
        access_count: i64,
    }
    
    let stats_info = match diesel::sql_query(stats_query)
        .bind::<Text, _>(&ip_id)
        .get_result::<StatsInfo>(&mut conn)
        .await
    {
        Ok(stats) => stats,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error fetching stats: {}", e),
            )
                .into_response();
        }
    };
    
    // Combine all the stats
    let stats_response = MarketplaceStatsResponse {
        ip_id: data_info.ip_id,
        owner: data_info.owner,
        media_type: data_info.media_type,
        total_revenue: stats_info.total_revenue,
        purchase_count: stats_info.purchase_count,
        subscription_count: stats_info.subscription_count,
        access_count: stats_info.access_count,
        one_time_price: data_info.one_time_price,
        subscription_price: data_info.subscription_price,
        created_at: data_info.created_at,
        last_updated: data_info.last_updated,
    };
    
    Json(stats_response).into_response()
}

/// Get revenue timeline data using TimescaleDB time_bucket function
pub async fn get_revenue_timeline(
    State(pool): State<DbPool>,
    Path(ip_id): Path<String>,
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
    
    // TimescaleDB query with time_bucket aggregation
    let query = "
        SELECT 
            time_bucket('1 day', to_timestamp(revenue_time)) as day,
            SUM(amount) as daily_revenue,
            COUNT(*) as daily_transactions
        FROM 
            my_ip_revenue
        WHERE 
            ip_id = $1
        GROUP BY 
            day
        ORDER BY 
            day DESC
        LIMIT 30
    ";
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&ip_id)
        .load::<DailyRevenue>(&mut conn)
        .await;
        
    match result {
        Ok(timeline) => Json(timeline).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

/// Get access analytics using TimescaleDB time_bucket function
pub async fn get_access_analytics(
    State(pool): State<DbPool>,
    Path(ip_id): Path<String>,
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
    
    // TimescaleDB query with time_bucket aggregation
    let query = "
        SELECT 
            time_bucket('1 day', to_timestamp(access_time)) as day,
            access_type,
            COUNT(DISTINCT user_address) as unique_users,
            COUNT(*) as total_accesses
        FROM 
            my_ip_access_logs
        WHERE 
            ip_id = $1
        GROUP BY 
            day, access_type
        ORDER BY 
            day DESC, access_type
        LIMIT 100
    ";
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&ip_id)
        .load::<AccessAnalytics>(&mut conn)
        .await;
        
    match result {
        Ok(analytics) => Json(analytics).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

/// Get popular marketplace data (most purchases/revenue/access)
pub async fn get_popular_marketplace_data(
    State(pool): State<DbPool>,
    Query(params): Query<MarketplaceQuery>,
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
    
    // Join with revenue and purchase stats to sort by popularity
    let query = "
        SELECT DISTINCT
            d.ip_id, d.owner, d.media_type, d.tags, d.platform_id, d.timestamp_start, d.timestamp_end,
            d.created_at, d.last_updated, d.one_time_price, d.subscription_price, d.subscription_duration_days,
            d.geographic_region, d.data_quality, d.sample_size, d.is_updating, d.update_frequency
        FROM my_ip_data d
        LEFT JOIN my_ip_purchases p ON d.ip_id = p.ip_id
        LEFT JOIN my_ip_revenue r ON d.ip_id = r.ip_id
        LEFT JOIN my_ip_access_logs a ON d.ip_id = a.ip_id
        WHERE 
            (d.one_time_price IS NOT NULL OR d.subscription_price IS NOT NULL)
        GROUP BY d.ip_id, d.owner, d.media_type, d.tags, d.platform_id, d.timestamp_start, d.timestamp_end,
                 d.created_at, d.last_updated, d.one_time_price, d.subscription_price, d.subscription_duration_days,
                 d.geographic_region, d.data_quality, d.sample_size, d.is_updating, d.update_frequency
        ORDER BY 
            (COUNT(p.id) + COUNT(r.id) + COUNT(a.id)) DESC, 
            d.created_at DESC
        LIMIT $1 OFFSET $2
    ";
    
    let result = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MarketplaceDataBasic>(&mut conn)
        .await;
        
    match result {
        Ok(data) => Json(data).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
} 