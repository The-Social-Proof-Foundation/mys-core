// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use chrono::{DateTime, NaiveDateTime, Utc, Duration};
use anyhow::Result;
use bigdecimal::{BigDecimal, ToPrimitive};

use crate::db::DbPool;
use crate::models::{
    CreatorRevenueStats, PlatformRevenueStats, RevenueTimeSeriesPoint,
    RevenueLeaderboardEntry, 
    RevenueDashboard, RevenueSourceStats, SptRevenueStats,
    REVENUE_SOURCE_SUBSCRIPTION, REVENUE_SOURCE_MY_IP, REVENUE_SOURCE_SPT,
    REVENUE_SOURCE_TIPS, REVENUE_SOURCE_POSTS, format_myso_amount,
    calculate_percentage, calculate_growth_rate
};
use crate::models::revenue::RevenueBreakdown;
use crate::schema;

// ==============================================================================
// REQUEST STRUCTURES
// ==============================================================================

#[derive(Debug, Deserialize)]
pub struct RevenueQuery {
    pub creator_address: Option<String>,
    pub platform_address: Option<String>,
    pub revenue_source: Option<String>,
    pub revenue_type: Option<String>,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub period: Option<String>, // "hour", "day", "week", "month"
}

#[derive(Debug, Deserialize)]
pub struct LeaderboardQuery {
    pub revenue_source: Option<String>,
    pub period_days: Option<i64>,
    pub limit: Option<i64>,
    pub min_revenue: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ChartQuery {
    pub creator_address: Option<String>,
    pub revenue_source: Option<String>,
    pub period: Option<String>, // "hour", "day", "week", "month"
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub points: Option<i64>, // Number of data points to return
}

// ==============================================================================
// REVENUE ANALYTICS ENDPOINTS
// ==============================================================================

/// Get unified revenue dashboard (24-hour overview)
pub async fn get_revenue_dashboard(
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                }))
            )
        }
    };

    match build_revenue_dashboard(&mut conn).await {
        Ok(dashboard) => (
            StatusCode::OK,
            Json(serde_json::json!(dashboard))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to build dashboard: {}", e)
            }))
        ),
    }
}

/// Get revenue leaderboard
pub async fn get_revenue_leaderboard(
    Query(params): Query<LeaderboardQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(50).min(100);
    let period_days = params.period_days.unwrap_or(30);
    let min_revenue = params.min_revenue.unwrap_or(0);

    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                }))
            )
        }
    };

    match build_revenue_leaderboard(&mut conn, &params, limit, period_days, min_revenue).await {
        Ok(leaderboard) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "leaderboard": leaderboard,
                "period_days": period_days,
                "min_revenue": min_revenue,
                "limit": limit
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to build leaderboard: {}", e)
            }))
        ),
    }
}

/// Get revenue time series data for charts
pub async fn get_revenue_chart_data(
    Query(params): Query<ChartQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let period = params.period.as_deref().unwrap_or("day");
    let points = params.points.unwrap_or(30);
    let end_date = params.end_date.unwrap_or_else(|| Utc::now().naive_utc());
    let start_date = params.start_date.unwrap_or_else(|| {
        match period {
            "hour" => end_date - Duration::hours(points),
            "day" => end_date - Duration::days(points),
            "week" => end_date - Duration::weeks(points),
            "month" => end_date - Duration::days(points * 30),
            _ => end_date - Duration::days(points),
        }
    });

    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                }))
            )
        }
    };

    match build_revenue_chart_data(&mut conn, &params, period, start_date, end_date).await {
        Ok(chart_data) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "chart_data": chart_data,
                "period": period,
                "start_date": start_date,
                "end_date": end_date
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to build chart data: {}", e)
            }))
        ),
    }
}

/// Get creator revenue statistics
pub async fn get_creator_revenue_stats(
    Path(creator_address): Path<String>,
    Query(params): Query<RevenueQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                }))
            )
        }
    };

    match build_creator_revenue_stats(&mut conn, &creator_address, &params).await {
        Ok(stats) => (
            StatusCode::OK,
            Json(serde_json::json!(stats))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to build creator stats: {}", e)
            }))
        ),
    }
}

/// Get platform revenue statistics
pub async fn get_platform_revenue_stats(
    Path(platform_address): Path<String>,
    Query(params): Query<RevenueQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                }))
            )
        }
    };

    match build_platform_revenue_stats(&mut conn, &platform_address, &params).await {
        Ok(stats) => (
            StatusCode::OK,
            Json(serde_json::json!(stats))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to build platform stats: {}", e)
            }))
        ),
    }
}

/// Get unified revenue records with filtering
pub async fn get_unified_revenue(
    Query(params): Query<RevenueQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);

    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                }))
            )
        }
    };

    // Build dynamic query
    let build_query = || {
        let mut query = schema::unified_revenue::table.into_boxed();
        
        if let Some(creator_address) = &params.creator_address {
            query = query.filter(schema::unified_revenue::creator_address.eq(creator_address));
        }
        
        if let Some(platform_address) = &params.platform_address {
            query = query.filter(schema::unified_revenue::platform_address.eq(platform_address));
        }
        
        if let Some(revenue_source) = &params.revenue_source {
            query = query.filter(schema::unified_revenue::revenue_source.eq(revenue_source));
        }
        
        if let Some(revenue_type) = &params.revenue_type {
            query = query.filter(schema::unified_revenue::revenue_type.eq(revenue_type));
        }
        
        if let Some(content_id) = &params.content_id {
            query = query.filter(schema::unified_revenue::content_id.eq(content_id));
        }
        
        if let Some(content_type) = &params.content_type {
            query = query.filter(schema::unified_revenue::content_type.eq(content_type));
        }
        
        if let Some(start_date) = &params.start_date {
            query = query.filter(schema::unified_revenue::time.ge(start_date));
        }
        
        if let Some(end_date) = &params.end_date {
            query = query.filter(schema::unified_revenue::time.le(end_date));
        }
        
        query
    };

    // Get total count and amount
    let total_count = match build_query().count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to count revenue records: {}", e)
                }))
            );
        }
    };

    let total_amount = match build_query()
        .select(diesel::dsl::sum(schema::unified_revenue::amount).nullable())
        .get_result::<Option<BigDecimal>>(&mut conn)
        .await
    {
        Ok(sum) => sum.and_then(|bd| bd.to_i64()).unwrap_or(0),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to aggregate revenue: {}", e)
                }))
            );
        }
    };

    // Get records with pagination
    let revenue_records = match build_query()
        .order_by(schema::unified_revenue::time.desc())
        .limit(limit)
        .offset(offset)
        .load::<crate::models::UnifiedRevenue>(&mut conn)
        .await
    {
        Ok(records) => records,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch revenue records: {}", e)
                }))
            );
        }
    };

    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "revenue_records": revenue_records,
            "total_count": total_count,
            "total_amount": total_amount,
            "total_amount_formatted": format_myso_amount(total_amount),
            "pagination": {
                "total": total_count,
                "limit": limit,
                "offset": offset,
                "page": (offset / limit) + 1,
                "total_pages": total_pages,
            }
        }))
    )
}

/// Get SPT revenue statistics for a pool
pub async fn get_spt_pool_revenue(
    Path(pool_id): Path<String>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                }))
            )
        }
    };

    match build_spt_pool_revenue_stats(&mut conn, &pool_id).await {
        Ok(Some(stats)) => (
            StatusCode::OK,
            Json(serde_json::json!(stats))
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "SPT pool not found or no revenue data"
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to build SPT stats: {}", e)
            }))
        ),
    }
}

// ==============================================================================
// HELPER FUNCTIONS
// ==============================================================================

async fn build_revenue_dashboard(conn: &mut diesel_async::AsyncPgConnection) -> Result<RevenueDashboard> {
    let now = Utc::now().naive_utc();
    let twenty_four_hours_ago = now - Duration::hours(24);

    // Use TimescaleDB view for 24h data
    let dashboard_query = r#"
        SELECT 
            revenue_source,
            SUM(revenue_5min) as total_revenue_24h,
            SUM(transactions_5min) as total_transactions_24h,
            COUNT(DISTINCT active_creators) as unique_creators_24h,
            COUNT(DISTINCT active_payers) as unique_payers_24h,
            MAX(max_transaction) as largest_transaction_24h
        FROM revenue_realtime_metrics
        WHERE bucket >= $1
        GROUP BY revenue_source
        ORDER BY total_revenue_24h DESC
    "#;

    #[derive(QueryableByName, Debug)]
    struct DashboardQueryResult {
        #[diesel(sql_type = diesel::sql_types::Text)]
        revenue_source: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_revenue_24h: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_transactions_24h: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_creators_24h: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_payers_24h: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        largest_transaction_24h: i64,
    }

    let dashboard_results: Vec<DashboardQueryResult> = diesel::sql_query(dashboard_query)
        .bind::<diesel::sql_types::Timestamp, _>(twenty_four_hours_ago)
        .load(conn)
        .await?;

    // Calculate totals
    let total_revenue_24h = dashboard_results.iter().map(|r| r.total_revenue_24h).sum();
    let total_transactions_24h = dashboard_results.iter().map(|r| r.total_transactions_24h).sum();
    let unique_creators_24h = dashboard_results.iter().map(|r| r.unique_creators_24h).max().unwrap_or(0);
    let unique_payers_24h = dashboard_results.iter().map(|r| r.unique_payers_24h).max().unwrap_or(0);
    let largest_transaction_24h = dashboard_results.iter().map(|r| r.largest_transaction_24h).max().unwrap_or(0);

    // Build revenue by source
    let revenue_by_source: Vec<RevenueSourceStats> = dashboard_results
        .into_iter()
        .map(|r| RevenueSourceStats {
            revenue_source: r.revenue_source,
            total_revenue: r.total_revenue_24h,
            transaction_count: r.total_transactions_24h,
            percentage_of_total: calculate_percentage(r.total_revenue_24h, total_revenue_24h),
            growth_rate: None, // Would calculate from historical data
        })
        .collect();

    // Get top creators (simplified)
    let top_creators = build_top_creators_leaderboard(conn, 10).await?;

    // Get recent trends (simplified)
    let recent_trends = build_recent_trends(conn, 24).await?;

    Ok(RevenueDashboard {
        total_revenue_24h,
        total_transactions_24h,
        unique_creators_24h,
        unique_payers_24h,
        largest_transaction_24h,
        revenue_by_source,
        top_creators,
        recent_trends,
    })
}

async fn build_revenue_leaderboard(
    conn: &mut diesel_async::AsyncPgConnection,
    params: &LeaderboardQuery,
    limit: i64,
    period_days: i64,
    min_revenue: i64,
) -> Result<Vec<RevenueLeaderboardEntry>> {
    // Use creator_revenue_summary view for leaderboard
    let mut leaderboard_query = "
        SELECT 
            creator_address,
            total_revenue,
            total_subscription_revenue,
            total_myip_revenue,
            total_spt_revenue,
            total_tips_revenue,
            total_transactions,
            total_unique_payers,
            ROW_NUMBER() OVER (ORDER BY total_revenue DESC) as rank
        FROM creator_revenue_summary
        WHERE total_revenue >= $1
    ".to_string();

    if let Some(revenue_source) = &params.revenue_source {
        match revenue_source.as_str() {
            "subscription" => leaderboard_query.push_str(" AND total_subscription_revenue > 0"),
            "my_ip" => leaderboard_query.push_str(" AND total_myip_revenue > 0"),
            "spt" => leaderboard_query.push_str(" AND total_spt_revenue > 0"),
            "tips" => leaderboard_query.push_str(" AND total_tips_revenue > 0"),
            _ => {}
        }
    }

    leaderboard_query.push_str(" ORDER BY total_revenue DESC LIMIT $2");

    #[derive(QueryableByName, Debug)]
    struct LeaderboardQueryResult {
        #[diesel(sql_type = diesel::sql_types::Text)]
        creator_address: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_subscription_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_myip_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_spt_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_tips_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_transactions: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_unique_payers: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        rank: i64,
    }

    let results: Vec<LeaderboardQueryResult> = diesel::sql_query(leaderboard_query)
        .bind::<diesel::sql_types::BigInt, _>(min_revenue)
        .bind::<diesel::sql_types::BigInt, _>(limit)
        .load(conn)
        .await?;

    let leaderboard = results
        .into_iter()
        .map(|r| RevenueLeaderboardEntry {
            rank: r.rank,
            creator_address: r.creator_address,
            total_revenue: r.total_revenue,
            revenue_breakdown: crate::models::revenue::RevenueBreakdown {
                subscription_revenue: r.total_subscription_revenue,
                myip_revenue: r.total_myip_revenue,
                spt_revenue: r.total_spt_revenue,
                tips_revenue: r.total_tips_revenue,
                posts_revenue: 0, // Not tracked separately yet
            },
            growth_rate: None, // Would calculate from historical data
            transaction_count: r.total_transactions,
            unique_payers: r.total_unique_payers,
        })
        .collect();

    Ok(leaderboard)
}

async fn build_revenue_chart_data(
    conn: &mut diesel_async::AsyncPgConnection,
    params: &ChartQuery,
    period: &str,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
) -> Result<Vec<RevenueTimeSeriesPoint>> {
    let time_bucket = match period {
        "hour" => "1 hour",
        "day" => "1 day",
        "week" => "1 week",
        "month" => "1 month",
        _ => "1 day",
    };

    let chart_data_query = format!(
        "
        SELECT 
            time_bucket('{}', time) as bucket,
            revenue_source,
            SUM(amount) as total_revenue,
            COUNT(*) as transaction_count,
            COUNT(DISTINCT creator_address) as unique_creators,
            COUNT(DISTINCT payer_address) as unique_payers
        FROM unified_revenue
        WHERE time BETWEEN $1 AND $2
        {}
        GROUP BY bucket, revenue_source ORDER BY bucket ASC
        ",
        time_bucket,
        if params.creator_address.is_some() { " AND creator_address = $3" } else { "" }
    );
    
    #[derive(QueryableByName, Debug)]
    struct ChartQueryResult {
        #[diesel(sql_type = diesel::sql_types::Timestamp)]
        bucket: NaiveDateTime,
        #[diesel(sql_type = diesel::sql_types::Text)]
        revenue_source: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        transaction_count: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_creators: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_payers: i64,
    }
    
    let results: Vec<ChartQueryResult> = if let Some(creator_address) = &params.creator_address {
        diesel::sql_query(chart_data_query)
            .bind::<diesel::sql_types::Timestamp, _>(start_date)
            .bind::<diesel::sql_types::Timestamp, _>(end_date)
            .bind::<diesel::sql_types::Text, _>(creator_address)
            .load(conn)
            .await?
    } else {
        diesel::sql_query(chart_data_query)
            .bind::<diesel::sql_types::Timestamp, _>(start_date)
            .bind::<diesel::sql_types::Timestamp, _>(end_date)
            .load(conn)
            .await?
    };

    let chart_data = results
        .into_iter()
        .map(|r| RevenueTimeSeriesPoint {
            timestamp: DateTime::from_naive_utc_and_offset(r.bucket, Utc),
            revenue_source: r.revenue_source,
            total_revenue: r.total_revenue,
            transaction_count: r.transaction_count,
            unique_creators: r.unique_creators,
            unique_payers: r.unique_payers,
        })
        .collect();

    Ok(chart_data)
}

async fn build_creator_revenue_stats(
    conn: &mut diesel_async::AsyncPgConnection,
    creator_address: &str,
    params: &RevenueQuery,
) -> Result<CreatorRevenueStats> {
    // Use creator_revenue_summary view
    let stats_query = "
        SELECT 
            creator_address,
            total_revenue,
            total_subscription_revenue,
            total_myip_revenue,
            total_spt_revenue,
            total_tips_revenue,
            total_transactions,
            total_unique_payers,
            largest_single_transaction,
            active_days,
            last_revenue_date
        FROM creator_revenue_summary
        WHERE creator_address = $1
    ";

    #[derive(QueryableByName, Debug)]
    struct CreatorStatsResult {
        #[diesel(sql_type = diesel::sql_types::Text)]
        creator_address: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_subscription_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_myip_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_spt_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_tips_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_transactions: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_unique_payers: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        largest_single_transaction: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        active_days: i64,
        #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Date>)]
        last_revenue_date: Option<chrono::NaiveDate>,
    }

    let result: CreatorStatsResult = diesel::sql_query(stats_query)
        .bind::<diesel::sql_types::Text, _>(creator_address)
        .get_result(conn)
        .await?;

    Ok(CreatorRevenueStats {
        creator_address: result.creator_address,
        total_revenue: result.total_revenue,
        subscription_revenue: result.total_subscription_revenue,
        myip_revenue: result.total_myip_revenue,
        spt_revenue: result.total_spt_revenue,
        tips_revenue: result.total_tips_revenue,
        posts_revenue: 0, // Not tracked separately yet
        total_transactions: result.total_transactions,
        unique_payers: result.total_unique_payers,
        largest_transaction: result.largest_single_transaction,
        active_days: result.active_days,
        last_revenue_date: result.last_revenue_date.map(|d| DateTime::from_naive_utc_and_offset(d.and_hms_opt(0, 0, 0).unwrap(), Utc)),
        revenue_rank: None, // Would need to calculate ranking
    })
}

async fn build_platform_revenue_stats(
    conn: &mut diesel_async::AsyncPgConnection,
    platform_address: &str,
    _params: &RevenueQuery,
) -> Result<PlatformRevenueStats> {
    // Use platform_revenue_summary view
    let stats_query = "
        SELECT 
            platform_address,
            total_revenue,
            total_subscription_revenue,
            total_myip_revenue,
            total_spt_revenue,
            total_transactions,
            total_creators,
            total_payers,
            avg_transaction_amount,
            active_months,
            last_active_month
        FROM platform_revenue_summary
        WHERE platform_address = $1
    ";

    #[derive(QueryableByName, Debug)]
    struct PlatformStatsResult {
        #[diesel(sql_type = diesel::sql_types::Text)]
        platform_address: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_subscription_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_myip_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_spt_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_transactions: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_creators: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_payers: i64,
        #[diesel(sql_type = diesel::sql_types::Double)]
        avg_transaction_amount: f64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        active_months: i64,
        #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Date>)]
        last_active_month: Option<chrono::NaiveDate>,
    }

    let result: PlatformStatsResult = diesel::sql_query(stats_query)
        .bind::<diesel::sql_types::Text, _>(platform_address)
        .get_result(conn)
        .await?;

    Ok(PlatformRevenueStats {
        platform_address: result.platform_address,
        total_revenue: result.total_revenue,
        subscription_revenue: result.total_subscription_revenue,
        myip_revenue: result.total_myip_revenue,
        spt_revenue: result.total_spt_revenue,
        total_transactions: result.total_transactions,
        unique_creators: result.total_creators,
        unique_payers: result.total_payers,
        avg_transaction_amount: result.avg_transaction_amount,
        active_months: result.active_months,
        last_active_month: result.last_active_month.map(|d| DateTime::from_naive_utc_and_offset(d.and_hms_opt(0, 0, 0).unwrap(), Utc)),
    })
}

async fn build_spt_pool_revenue_stats(
    conn: &mut diesel_async::AsyncPgConnection,
    pool_id: &str,
) -> Result<Option<SptRevenueStats>> {
    // Use SPT revenue table directly for detailed stats
    let stats_query = "
        SELECT 
            pool_id,
            creator_address,
            SUM(total_fee) as total_fees,
            SUM(creator_fee) as creator_fees,
            SUM(platform_fee) as platform_fees,
            SUM(treasury_fee) as treasury_fees,
            SUM(mys_amount) as total_volume,
            SUM(token_amount) as total_tokens,
            COUNT(*) as transaction_count,
            COUNT(DISTINCT trader) as unique_traders,
            AVG(token_price::FLOAT) as avg_price,
            MAX(token_price) as max_price,
            MIN(token_price) as min_price,
            SUM(CASE WHEN transaction_type = 'buy' THEN mys_amount ELSE 0 END) as buy_volume,
            SUM(CASE WHEN transaction_type = 'sell' THEN mys_amount ELSE 0 END) as sell_volume
        FROM spt_revenue 
        WHERE pool_id = $1
        GROUP BY pool_id, creator_address
    ";

    #[derive(QueryableByName, Debug)]
    struct SptStatsResult {
        #[diesel(sql_type = diesel::sql_types::Text)]
        pool_id: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        creator_address: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_fees: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        creator_fees: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        platform_fees: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        treasury_fees: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_volume: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_tokens: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        transaction_count: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_traders: i64,
        #[diesel(sql_type = diesel::sql_types::Double)]
        avg_price: f64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        max_price: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        min_price: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        buy_volume: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        sell_volume: i64,
    }

    let result: Option<SptStatsResult> = diesel::sql_query(stats_query)
        .bind::<diesel::sql_types::Text, _>(pool_id)
        .get_result(conn)
        .await
        .optional()?;

    Ok(result.map(|r| SptRevenueStats {
        pool_id: r.pool_id,
        creator_address: r.creator_address,
        total_fees: r.total_fees,
        creator_fees: r.creator_fees,
        platform_fees: r.platform_fees,
        treasury_fees: r.treasury_fees,
        total_volume: r.total_volume,
        total_tokens: r.total_tokens,
        transaction_count: r.transaction_count,
        unique_traders: r.unique_traders,
        avg_price: r.avg_price,
        max_price: r.max_price,
        min_price: r.min_price,
        buy_volume: r.buy_volume,
        sell_volume: r.sell_volume,
        net_flow: r.buy_volume - r.sell_volume,
    }))
}

async fn build_top_creators_leaderboard(
    conn: &mut diesel_async::AsyncPgConnection,
    limit: i64,
) -> Result<Vec<RevenueLeaderboardEntry>> {
    // Simplified top creators query
    let query = "
        SELECT 
            creator_address,
            total_revenue,
            total_subscription_revenue,
            total_myip_revenue,
            total_spt_revenue,
            total_tips_revenue,
            total_transactions,
            total_unique_payers,
            ROW_NUMBER() OVER (ORDER BY total_revenue DESC) as rank
        FROM creator_revenue_summary
        ORDER BY total_revenue DESC
        LIMIT $1
    ";

    #[derive(QueryableByName, Debug)]
    struct TopCreatorResult {
        #[diesel(sql_type = diesel::sql_types::Text)]
        creator_address: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_subscription_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_myip_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_spt_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_tips_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_transactions: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_unique_payers: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        rank: i64,
    }

    let results: Vec<TopCreatorResult> = diesel::sql_query(query)
        .bind::<diesel::sql_types::BigInt, _>(limit)
        .load(conn)
        .await?;

    let leaderboard = results
        .into_iter()
        .map(|r| RevenueLeaderboardEntry {
            rank: r.rank,
            creator_address: r.creator_address,
            total_revenue: r.total_revenue,
            revenue_breakdown: crate::models::revenue::RevenueBreakdown {
                subscription_revenue: r.total_subscription_revenue,
                myip_revenue: r.total_myip_revenue,
                spt_revenue: r.total_spt_revenue,
                tips_revenue: r.total_tips_revenue,
                posts_revenue: 0,
            },
            growth_rate: None,
            transaction_count: r.total_transactions,
            unique_payers: r.total_unique_payers,
        })
        .collect();

    Ok(leaderboard)
}

async fn build_recent_trends(
    conn: &mut diesel_async::AsyncPgConnection,
    hours: i64,
) -> Result<Vec<RevenueTimeSeriesPoint>> {
    let start_time = Utc::now().naive_utc() - Duration::hours(hours);

    let query = "
        SELECT 
            bucket,
            revenue_source,
            revenue_5min as total_revenue,
            transactions_5min as transaction_count,
            active_creators as unique_creators,
            active_payers as unique_payers
        FROM revenue_realtime_metrics
        WHERE bucket >= $1
        ORDER BY bucket ASC
    ";

    #[derive(QueryableByName, Debug)]
    struct TrendResult {
        #[diesel(sql_type = diesel::sql_types::Timestamp)]
        bucket: NaiveDateTime,
        #[diesel(sql_type = diesel::sql_types::Text)]
        revenue_source: String,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        transaction_count: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_creators: i64,
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        unique_payers: i64,
    }

    let results: Vec<TrendResult> = diesel::sql_query(query)
        .bind::<diesel::sql_types::Timestamp, _>(start_time)
        .load(conn)
        .await?;

    let trends = results
        .into_iter()
        .map(|r| RevenueTimeSeriesPoint {
            timestamp: DateTime::from_naive_utc_and_offset(r.bucket, Utc),
            revenue_source: r.revenue_source,
            total_revenue: r.total_revenue,
            transaction_count: r.transaction_count,
            unique_creators: r.unique_creators,
            unique_payers: r.unique_payers,
        })
        .collect();

    Ok(trends)
} 