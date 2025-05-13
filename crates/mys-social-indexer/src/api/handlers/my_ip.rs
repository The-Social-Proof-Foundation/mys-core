// Copyright (c) MySocial Team
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

// Query parameters for license listing
#[derive(Debug, Deserialize)]
pub struct MyIPQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub creator: Option<String>,
    pub license_id: Option<String>,
    pub license_type: Option<i16>,
    pub license_state: Option<i16>,
    pub include_expired: Option<bool>,
}

// Combined stats for a license
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub license_id: String,
    pub name: String,
    pub creator: String,
    pub total_revenue: i64,
    pub transactions_count: i64,
    pub last_transaction: Option<String>,
    pub usage_count: i64,
}

// Revenue timeline data
#[derive(Debug, Serialize, QueryableByName)]
pub struct DailyRevenue {
    #[diesel(sql_type = Timestamp)]
    pub time_bucket: chrono::NaiveDateTime,
    
    #[diesel(sql_type = BigInt)]
    pub daily_revenue: i64,
    
    #[diesel(sql_type = BigInt)]
    pub daily_transactions: i64,
}

// Basic license info returned for list operations
#[derive(Debug, Serialize, QueryableByName)]
pub struct LicenseBasic {
    #[diesel(sql_type = Text)]
    pub license_id: String,
    
    #[diesel(sql_type = Text)]
    pub name: String,
    
    #[diesel(sql_type = Text)]
    pub creator: String,
    
    #[diesel(sql_type = Nullable<Text>)]
    pub description: Option<String>,
    
    #[diesel(sql_type = Int2)]
    pub license_type: i16,
    
    #[diesel(sql_type = Int2)]
    pub license_state: i16,
    
    #[diesel(sql_type = BigInt)]
    pub creation_time: i64,
    
    #[diesel(sql_type = Nullable<BigInt>)]
    pub expires_at: Option<i64>,
}

// Get a license by ID
pub async fn get_license_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
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

    let query = "SELECT license_id, name, creator, description, license_type, license_state, creation_time, expires_at FROM my_ip WHERE license_id = $1";
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&id)
        .get_result::<LicenseBasic>(&mut conn)
        .await;

    match result {
        Ok(license) => Json(license).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "License not found").into_response()
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

// List licenses with filtering
pub async fn list_licenses(
    State(pool): State<DbPool>, 
    Query(params): Query<MyIPQuery>
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

    // Build the query based on parameters
    let limit = params.limit.unwrap_or(20).min(100); // Max 100 licenses
    let offset = params.offset.unwrap_or(0);
    
    // Build SQL query with basic fields directly
    let mut sql_query = "SELECT license_id, name, creator, description, license_type, license_state, creation_time, expires_at FROM my_ip WHERE 1=1".to_string();
    
    // Apply filters
    if let Some(creator) = &params.creator {
        sql_query.push_str(&format!(" AND creator = '{}'", creator));
    }
    
    if let Some(license_id) = &params.license_id {
        sql_query.push_str(&format!(" AND license_id = '{}'", license_id));
    }
    
    if let Some(license_type) = params.license_type {
        sql_query.push_str(&format!(" AND license_type = {}", license_type));
    }
    
    if let Some(license_state) = params.license_state {
        sql_query.push_str(&format!(" AND license_state = {}", license_state));
    }
    
    // By default, don't include expired licenses
    if !params.include_expired.unwrap_or(false) {
        sql_query.push_str(" AND (license_state != 1 AND (expires_at IS NULL OR expires_at > EXTRACT(EPOCH FROM NOW())::BIGINT))");
    }
    
    // Order by creation time (newest first)
    sql_query.push_str(" ORDER BY creation_time DESC LIMIT $1 OFFSET $2");
    
    let result = diesel::sql_query(&sql_query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<LicenseBasic>(&mut conn)
        .await;
    
    match result {
        Ok(licenses) => Json(licenses).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

// Get events for a specific license
pub async fn get_license_events(
    State(pool): State<DbPool>,
    Path(license_id): Path<String>,
    Query(params): Query<MyIPQuery>,
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
    
    // For now, just return the raw IDs since we're troubleshooting TimescaleDB integration
    let query = "SELECT event_id::text FROM my_ip_events WHERE license_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3";
    
    // Define a simple queryable struct for event IDs
    #[derive(QueryableByName, Serialize)]
    struct EventId {
        #[diesel(sql_type = Text)]
        event_id: String,
    }
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&license_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<EventId>(&mut conn)
        .await;
        
    match result {
        Ok(events) => Json(events).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

// Get grants for a specific license
pub async fn get_license_grants(
    State(pool): State<DbPool>,
    Path(license_id): Path<String>,
    Query(params): Query<MyIPQuery>,
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
    
    // For now, just return the raw IDs since we're troubleshooting TimescaleDB integration
    let query = "SELECT grant_id::text FROM my_ip_grants WHERE license_id = $1 ORDER BY grant_time DESC LIMIT $2 OFFSET $3";
    
    // Define a simple queryable struct for grant IDs
    #[derive(QueryableByName, Serialize)]
    struct GrantId {
        #[diesel(sql_type = Text)]
        grant_id: String,
    }
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&license_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<GrantId>(&mut conn)
        .await;
        
    match result {
        Ok(grants) => Json(grants).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

// Get revenue for a specific license
pub async fn get_license_revenue(
    State(pool): State<DbPool>,
    Path(license_id): Path<String>,
    Query(params): Query<MyIPQuery>,
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
    
    // For now, just return basic revenue info since we're troubleshooting TimescaleDB integration
    let query = "SELECT amount, revenue_time FROM my_ip_revenue WHERE license_id = $1 ORDER BY revenue_time DESC LIMIT $2 OFFSET $3";
    
    // Define a simple queryable struct for revenue
    #[derive(QueryableByName, Serialize)]
    struct RevenueEntry {
        #[diesel(sql_type = BigInt)]
        amount: i64,
        
        #[diesel(sql_type = BigInt)]
        revenue_time: i64,
    }
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&license_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<RevenueEntry>(&mut conn)
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

// Get licenses created by a specific creator
pub async fn get_creator_licenses(
    State(pool): State<DbPool>,
    Path(creator): Path<String>,
    Query(params): Query<MyIPQuery>,
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
    
    // Build SQL query with basic fields directly
    let mut query = "SELECT license_id, name, creator, description, license_type, license_state, creation_time, expires_at FROM my_ip WHERE creator = $1".to_string();
    
    // By default, don't include expired licenses
    if !params.include_expired.unwrap_or(false) {
        query.push_str(" AND (license_state != 1 AND (expires_at IS NULL OR expires_at > EXTRACT(EPOCH FROM NOW())::BIGINT))");
    }
    
    // Order by creation time (newest first)
    query.push_str(" ORDER BY creation_time DESC LIMIT $2 OFFSET $3");
    
    let result = diesel::sql_query(&query)
        .bind::<Text, _>(&creator)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<LicenseBasic>(&mut conn)
        .await;
    
    match result {
        Ok(licenses) => Json(licenses).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
}

// Get posts that use a specific license
pub async fn get_license_posts(
    State(pool): State<DbPool>,
    Path(license_id): Path<String>,
    Query(params): Query<MyIPQuery>,
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
    
    // For now, just return basic post info since we're troubleshooting TimescaleDB integration
    let query = "
        SELECT p.post_id, p.owner, p.created_at FROM posts p
        JOIN license_usage lu ON p.post_id = lu.object_id AND lu.object_type = 'post'
        WHERE lu.license_id = $1 AND p.deleted_at IS NULL AND p.removed_from_platform = false
        ORDER BY p.created_at DESC
        LIMIT $2 OFFSET $3
    ";

    // Define a simple queryable struct for post info
    #[derive(QueryableByName, Serialize)]
    struct PostInfo {
        #[diesel(sql_type = Text)]
        post_id: String,
        
        #[diesel(sql_type = Text)]
        owner: String,
        
        #[diesel(sql_type = BigInt)]
        created_at: i64,
    }
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&license_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PostInfo>(&mut conn)
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

// Get stats for a specific license
pub async fn get_license_stats(
    State(pool): State<DbPool>,
    Path(license_id): Path<String>,
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

    // First get the license basic info
    let query = "SELECT license_id, name, creator FROM my_ip WHERE license_id = $1";
    
    // Define a simple queryable struct for minimal license info
    #[derive(QueryableByName, Serialize)]
    struct LicenseInfo {
        #[diesel(sql_type = Text)]
        license_id: String,
        
        #[diesel(sql_type = Text)]
        name: String,
        
        #[diesel(sql_type = Text)]
        creator: String,
    }
    
    let license_result = diesel::sql_query(query)
        .bind::<Text, _>(&license_id)
        .get_result::<LicenseInfo>(&mut conn)
        .await;
    
    let license_info = match license_result {
        Ok(license) => license,
        Err(diesel::result::Error::NotFound) => {
            return (StatusCode::NOT_FOUND, "License not found").into_response();
        },
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response();
        }
    };
    
    // Get total revenue stats
    let revenue_stats_query = "
        SELECT 
            COALESCE(SUM(amount), 0) as total_revenue,
            COUNT(*) as transactions_count,
            MAX(revenue_time) as last_transaction
        FROM 
            my_ip_revenue
        WHERE 
            license_id = $1
    ";
    
    // Define a simple queryable struct for revenue stats
    #[derive(QueryableByName)]
    struct RevStats {
        #[diesel(sql_type = BigInt)]
        total_revenue: i64,
        
        #[diesel(sql_type = BigInt)]
        transactions_count: i64,
        
        #[diesel(sql_type = Nullable<BigInt>)]
        last_transaction: Option<i64>,
    }
    
    let revenue_stats_result = diesel::sql_query(revenue_stats_query)
        .bind::<Text, _>(&license_id)
        .get_result::<RevStats>(&mut conn)
        .await;
    
    let rev_stats = match revenue_stats_result {
        Ok(stats) => stats,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error fetching revenue stats: {}", e),
            )
                .into_response();
        }
    };
    
    // Get usage count
    let usage_count_query = "
        SELECT COUNT(*) as usage_count
        FROM license_usage
        WHERE license_id = $1
    ";
    
    #[derive(QueryableByName)]
    struct UsageCount {
        #[diesel(sql_type = BigInt)]
        usage_count: i64,
    }
    
    let usage_count_result = diesel::sql_query(usage_count_query)
        .bind::<Text, _>(&license_id)
        .get_result::<UsageCount>(&mut conn)
        .await;
    
    let usage_count = match usage_count_result {
        Ok(count) => count.usage_count,
        Err(_) => 0, // Default to 0 if there's an error
    };
    
    // Format the timestamp from epoch to string if available
    let last_transaction_str = rev_stats.last_transaction.map(|ts| {
        // Use DateTime::from_timestamp instead of deprecated from_timestamp_opt
        chrono::DateTime::from_timestamp(ts, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Invalid timestamp".to_string())
    });
    
    // Combine all the stats
    let stats_response = StatsResponse {
        license_id: license_info.license_id,
        name: license_info.name,
        creator: license_info.creator,
        total_revenue: rev_stats.total_revenue,
        transactions_count: rev_stats.transactions_count,
        last_transaction: last_transaction_str,
        usage_count,
    };
    
    Json(stats_response).into_response()
}

// Get revenue timeline data
pub async fn get_revenue_timeline(
    State(pool): State<DbPool>,
    Path(license_id): Path<String>,
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
    
    // This query uses TimescaleDB's time_bucket function to group revenue by day
    let query = "
        SELECT 
            time_bucket('1 day', to_timestamp(revenue_time)) as time_bucket,
            SUM(amount) as daily_revenue,
            COUNT(*) as daily_transactions
        FROM 
            my_ip_revenue
        WHERE 
            license_id = $1
        GROUP BY 
            time_bucket
        ORDER BY 
            time_bucket DESC
        LIMIT 30
    ";
    
    let result = diesel::sql_query(query)
        .bind::<Text, _>(&license_id)
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

// Get popular licenses
pub async fn get_popular_licenses(
    State(pool): State<DbPool>,
    Query(params): Query<MyIPQuery>,
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
    
    // Join with license_usage to count usage and sort by popularity
    let query = "
        SELECT m.license_id, m.name, m.creator, m.description, m.license_type, m.license_state, m.creation_time, m.expires_at, COUNT(lu.license_id) as usage_count
        FROM my_ip m
        LEFT JOIN license_usage lu ON m.license_id = lu.license_id
        WHERE 
            (m.expires_at IS NULL OR m.expires_at > EXTRACT(EPOCH FROM NOW())::BIGINT)
            AND m.license_state != 1
        GROUP BY m.license_id, m.id
        ORDER BY usage_count DESC, m.creation_time DESC
        LIMIT $1 OFFSET $2
    ";
    
    // Define a struct to hold the result with usage count
    #[derive(Debug, QueryableByName, Serialize)]
    struct LicenseWithUsage {
        #[diesel(sql_type = Text)]
        license_id: String,
        
        #[diesel(sql_type = Text)]
        name: String,
        
        #[diesel(sql_type = Text)]
        creator: String,
        
        #[diesel(sql_type = Nullable<Text>)]
        description: Option<String>,
        
        #[diesel(sql_type = Int2)]
        license_type: i16,
        
        #[diesel(sql_type = Int2)]
        license_state: i16,
        
        #[diesel(sql_type = BigInt)]
        creation_time: i64,
        
        #[diesel(sql_type = Nullable<BigInt>)]
        expires_at: Option<i64>,
        
        #[diesel(sql_type = BigInt)]
        usage_count: i64,
    }
    
    let result = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<LicenseWithUsage>(&mut conn)
        .await;
        
    match result {
        Ok(licenses) => Json(licenses).into_response(),
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
                .into_response()
        }
    }
} 