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
use chrono::{NaiveDateTime, Utc};
use anyhow::Result;
use bigdecimal::{BigDecimal, ToPrimitive};

use crate::db::DbPool;
use crate::models::subscription::*;
use crate::schema;

// ==============================================================================
// REQUEST STRUCTURES
// ==============================================================================

#[derive(Debug, Deserialize)]
pub struct SubscriptionQuery {
    pub subscriber: Option<String>,
    pub service_id: Option<String>,
    pub profile_owner: Option<String>,
    pub active_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub service_id: Option<String>,
    pub profile_owner: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub period: Option<String>, // "day", "week", "month"
}

#[derive(Debug, Deserialize)]
pub struct RevenueQuery {
    pub service_id: Option<String>,
    pub profile_owner: Option<String>,
    pub revenue_type: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ==============================================================================
// SUBSCRIPTION ENDPOINTS
// ==============================================================================

/// Get subscriptions with filtering and pagination
pub async fn get_subscriptions(
    Query(params): Query<SubscriptionQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(50).min(100); // Cap at 100
    let page = params.page.unwrap_or(1).max(1);
    let offset = params.offset.unwrap_or((page - 1) * limit);
    
    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                }))
            );
        }
    };
    
    // Build the query dynamically
    let build_query = || {
        let mut query = schema::profile_subscriptions::table.into_boxed();
        
        if let Some(subscriber) = &params.subscriber {
            query = query.filter(schema::profile_subscriptions::subscriber.eq(subscriber));
        }
        
        if let Some(service_id) = &params.service_id {
            query = query.filter(schema::profile_subscriptions::service_id.eq(service_id));
        }
        
        if params.active_only.unwrap_or(false) {
            let current_time = Utc::now().timestamp();
            query = query.filter(
                schema::profile_subscriptions::cancelled_at.is_null()
                    .and(schema::profile_subscriptions::expires_at.gt(current_time))
            );
        }
        
        query
    };
    
    // Get total count
    let total_count = match build_query().count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to count subscriptions: {}", e)
                }))
            );
        }
    };
    
    // Get subscriptions with pagination
    let subscriptions = match build_query()
        .order_by(schema::profile_subscriptions::time.desc())
        .limit(limit)
        .offset(offset)
        .load::<ProfileSubscription>(&mut conn)
        .await
    {
        Ok(subs) => subs,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch subscriptions: {}", e)
                }))
            );
        }
    };
    
    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;
    
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "subscriptions": subscriptions,
            "total_count": total_count,
            "pagination": {
                "total": total_count,
                "limit": limit,
                "offset": offset,
                "page": page,
                "total_pages": total_pages,
            }
        }))
    )
}

/// Get subscription services with filtering and pagination
pub async fn get_subscription_services(
    Query(params): Query<SubscriptionQuery>,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(50).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = params.offset.unwrap_or((page - 1) * limit);
    
    let mut conn = match db_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Database connection error: {}", e)
                }))
            );
        }
    };
    
    // Build the query dynamically
    let build_query = || {
        let mut query = schema::profile_subscription_services::table.into_boxed();
        
        if let Some(profile_owner) = &params.profile_owner {
            query = query.filter(schema::profile_subscription_services::profile_owner.eq(profile_owner));
        }
        
        if params.active_only.unwrap_or(false) {
            query = query.filter(schema::profile_subscription_services::active.eq(true));
        }
        
        query
    };
    
    // Get total count
    let total_count = match build_query().count().get_result::<i64>(&mut conn).await {
        Ok(count) => count,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to count services: {}", e)
                }))
            );
        }
    };
    
    // Get services with pagination
    let services = match build_query()
        .order_by(schema::profile_subscription_services::time.desc())
        .limit(limit)
        .offset(offset)
        .load::<ProfileSubscriptionService>(&mut conn)
        .await
    {
        Ok(services) => services,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch services: {}", e)
                }))
            );
        }
    };
    
    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;
    
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "services": services,
            "total_count": total_count,
            "pagination": {
                "total": total_count,
                "limit": limit,
                "offset": offset,
                "page": page,
                "total_pages": total_pages,
            }
        }))
    )
}

/// Get subscription revenue records
pub async fn get_subscription_revenue(
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
            );
        }
    };
    
    // Build the query dynamically
    let build_query = || {
        let mut query = schema::subscription_revenue::table.into_boxed();
        
        if let Some(service_id) = &params.service_id {
            query = query.filter(schema::subscription_revenue::service_id.eq(service_id));
        }
        
        if let Some(revenue_type) = &params.revenue_type {
            query = query.filter(schema::subscription_revenue::revenue_type.eq(revenue_type));
        }
        
        if let Some(start_date) = &params.start_date {
            query = query.filter(schema::subscription_revenue::time.ge(start_date));
        }
        
        if let Some(end_date) = &params.end_date {
            query = query.filter(schema::subscription_revenue::time.le(end_date));
        }
        
        query
    };
    
    // Get total count first
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

    // Get total amount separately
    let total_amount = match build_query()
        .select(diesel::dsl::sum(schema::subscription_revenue::amount).nullable())
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
    
    // Get revenue records with pagination
    let revenue_records = match build_query()
        .order_by(schema::subscription_revenue::time.desc())
        .limit(limit)
        .offset(offset)
        .load::<SubscriptionRevenue>(&mut conn)
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

/// Check subscription access for a user to specific content
pub async fn check_subscription_access(
    Path((subscriber, content_id)): Path<(String, String)>,
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
            );
        }
    };
    
    // Check if the user has an active subscription for content requiring subscription
    let current_time = Utc::now().timestamp();
    
    // First, check if the content requires a subscription
    let post_info = match schema::posts::table
        .filter(schema::posts::post_id.eq(&content_id))
        .select((
            schema::posts::requires_subscription,
            schema::posts::subscription_service_id,
        ))
        .first::<(Option<bool>, Option<String>)>(&mut conn)
        .await
    {
        Ok(info) => info,
        Err(diesel::result::Error::NotFound) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Content not found"
                }))
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to check content: {}", e)
                }))
            );
        }
    };
    
    let (requires_subscription_opt, service_id_opt) = post_info;
    let requires_subscription = requires_subscription_opt.unwrap_or(false);
    
    // If content doesn't require subscription, grant access
    if !requires_subscription {
        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "has_access": true,
                "subscription": null,
                "service": null,
                "access_expires_at": null
            }))
        );
    }
    
    // If content requires subscription but no service is specified, deny access
    let service_id = match service_id_opt {
        Some(id) => id,
        None => {
            return (
                StatusCode::OK,
                Json(serde_json::json!({
                    "has_access": false,
                    "subscription": null,
                    "service": null,
                    "access_expires_at": null
                }))
            );
        }
    };
    
    // Check for active subscription
    let subscription_result = schema::profile_subscriptions::table
        .filter(schema::profile_subscriptions::subscriber.eq(&subscriber))
        .filter(schema::profile_subscriptions::service_id.eq(&service_id))
        .filter(schema::profile_subscriptions::cancelled_at.is_null())
        .filter(schema::profile_subscriptions::expires_at.gt(current_time))
        .first::<ProfileSubscription>(&mut conn)
        .await;
    
    match subscription_result {
        Ok(subscription) => {
            // Get service details
            let service = match schema::profile_subscription_services::table
                .filter(schema::profile_subscription_services::service_id.eq(&service_id))
                .first::<ProfileSubscriptionService>(&mut conn)
                .await
            {
                Ok(service) => Some(service),
                Err(_) => None,
            };
            
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "has_access": true,
                    "subscription": subscription,
                    "service": service,
                    "access_expires_at": subscription.expires_at
                }))
            )
        }
        Err(_) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "has_access": false,
                    "subscription": null,
                    "service": null,
                    "access_expires_at": null
                }))
            )
        }
    }
}

/// Get subscription status for a specific subscription
pub async fn get_subscription_status(
    Path(subscription_id): Path<String>,
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
            );
        }
    };
    
    // Get subscription with service info
    let subscription_info = match schema::profile_subscriptions::table
        .inner_join(schema::profile_subscription_services::table.on(
            schema::profile_subscriptions::service_id.eq(schema::profile_subscription_services::service_id)
        ))
        .filter(schema::profile_subscriptions::subscription_id.eq(&subscription_id))
        .select((
            schema::profile_subscriptions::all_columns,
            schema::profile_subscription_services::monthly_fee,
        ))
        .first::<(ProfileSubscription, i64)>(&mut conn)
        .await
    {
        Ok(info) => info,
        Err(diesel::result::Error::NotFound) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Subscription not found"
                }))
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch subscription: {}", e)
                }))
            );
        }
    };
    
    let (subscription, monthly_fee) = subscription_info;
    let current_time = Utc::now().timestamp();
    
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "subscription_id": subscription.subscription_id,
            "is_active": subscription.is_active(current_time),
            "expires_at": subscription.expires_at,
            "days_remaining": subscription.days_until_expiration(current_time),
            "status": subscription.status(current_time),
            "can_auto_renew": subscription.can_auto_renew(monthly_fee)
        }))
    )
}

// ==============================================================================
// ANALYTICS ENDPOINTS
// ==============================================================================

/// Get comprehensive subscription analytics
pub async fn get_subscription_analytics(
    Query(params): Query<AnalyticsQuery>,
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
            );
        }
    };
    
    // Set default time range if not provided
    let end_date = params.end_date.unwrap_or_else(|| Utc::now().naive_utc());
    let start_date = params.start_date.unwrap_or_else(|| {
        end_date - chrono::Duration::days(30) // Default to 30 days
    });
    
    // This would be a complex analytics query - simplified for example
    match calculate_subscription_analytics(&mut conn, &params, start_date, end_date).await {
        Ok(analytics) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "analytics": analytics,
                "period_start": start_date,
                "period_end": end_date
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to calculate analytics: {}", e)
            }))
        ),
    }
}

/// Get service performance metrics
pub async fn get_service_performance(
    Query(params): Query<AnalyticsQuery>,
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
            );
        }
    };
    
    match calculate_service_performance(&mut conn, &params).await {
        Ok(services) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "total_count": services.len() as i64,
                "services": services
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to calculate service performance: {}", e)
            }))
        ),
    }
}

/// Get subscriber summary
pub async fn get_subscriber_summary(
    Path(subscriber): Path<String>,
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
            );
        }
    };
    
    match calculate_subscriber_summary(&mut conn, &subscriber).await {
        Ok(summary) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "summary": summary
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to calculate subscriber summary: {}", e)
            }))
        ),
    }
}

// ==============================================================================
// HELPER FUNCTIONS FOR ANALYTICS
// ==============================================================================

async fn calculate_subscription_analytics(
    conn: &mut diesel_async::AsyncPgConnection,
    params: &AnalyticsQuery,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
) -> Result<SubscriptionAnalytics> {
    // This is a simplified implementation - in a real system, this would use
    // the TimescaleDB continuous aggregates for better performance
    
    // Helper function to build base query
    let build_base_query = || {
        let mut query = schema::profile_subscriptions::table.into_boxed();
        if let Some(service_id) = &params.service_id {
            query = query.filter(schema::profile_subscriptions::service_id.eq(service_id));
        }
        query = query.filter(schema::profile_subscriptions::time.between(start_date, end_date));
        query
    };
    
    // Get basic metrics with separate queries
    let total_subscriptions = build_base_query().count().get_result::<i64>(conn).await?;
    
    let active_subscriptions = build_base_query()
        .filter(schema::profile_subscriptions::cancelled_at.is_null())
        .count()
        .get_result::<i64>(conn)
        .await?;
    
    let cancelled_subscriptions = build_base_query()
        .filter(schema::profile_subscriptions::cancelled_at.is_not_null())
        .count()
        .get_result::<i64>(conn)
        .await?;
    
    // Calculate churn rate
    let churn_rate = if total_subscriptions > 0 {
        cancelled_subscriptions as f64 / total_subscriptions as f64
    } else {
        0.0
    };
    
    // Calculate total revenue from subscription_revenue table
    let total_revenue = if let Some(service_id) = &params.service_id {
        // Revenue for specific service
        schema::subscription_revenue::table
            .filter(schema::subscription_revenue::service_id.eq(service_id))
            .filter(schema::subscription_revenue::time.between(start_date, end_date))
            .select(diesel::dsl::sum(schema::subscription_revenue::amount))
            .first::<Option<BigDecimal>>(conn)
            .await?
            .map(|bd| bd.to_string().parse::<i64>().unwrap_or(0))
            .unwrap_or(0)
    } else {
        // Total revenue across all services
        schema::subscription_revenue::table
            .filter(schema::subscription_revenue::time.between(start_date, end_date))
            .select(diesel::dsl::sum(schema::subscription_revenue::amount))
            .first::<Option<BigDecimal>>(conn)
            .await?
            .map(|bd| bd.to_string().parse::<i64>().unwrap_or(0))
            .unwrap_or(0)
    };
    
    // Calculate monthly recurring revenue (simplified)
    let monthly_recurring_revenue = total_revenue / 30; // Rough approximation
    
    // This is a simplified analytics calculation
    // In production, you'd use the TimescaleDB continuous aggregates
    Ok(SubscriptionAnalytics {
        service_id: params.service_id.clone().unwrap_or_else(|| "all".to_string()),
        total_revenue,
        active_subscriptions,
        cancelled_subscriptions,
        monthly_recurring_revenue,
        churn_rate,
        average_subscription_duration: 30.0, // Placeholder
        total_renewals: 0, // Would calculate from renewal events
        auto_renewal_rate: 0.0, // Would calculate from renewal data
        refund_rate: 0.0, // Would calculate from refund data
        growth_metrics: vec![], // Would calculate time-series data
    })
}

async fn calculate_service_performance(
    conn: &mut diesel_async::AsyncPgConnection,
    params: &AnalyticsQuery,
) -> Result<Vec<ServicePerformance>> {
    // This is a simplified implementation
    let mut query = schema::profile_subscription_services::table.into_boxed();
    
    if let Some(profile_owner) = &params.profile_owner {
        query = query.filter(schema::profile_subscription_services::profile_owner.eq(profile_owner));
    }
    
    let services = query.load::<ProfileSubscriptionService>(conn).await?;
    
    let mut performance_metrics = Vec::new();
    
    for service in services {
        let performance = ServicePerformance {
            service_id: service.service_id.clone(),
            profile_owner: service.profile_owner.clone(),
            profile_id: service.profile_id.clone(),
            monthly_fee: service.monthly_fee,
            total_subscribers: service.subscriber_count,
            active_subscribers: service.subscriber_count, // Simplified
            total_revenue: service.expected_monthly_revenue(),
            monthly_recurring_revenue: service.expected_monthly_revenue(),
            churn_rate: 0.0, // Would calculate from historical data
            average_lifetime_value: 0.0, // Would calculate from historical data
            conversion_rate: 0.0, // Would calculate from conversion data
        };
        
        performance_metrics.push(performance);
    }
    
    Ok(performance_metrics)
}

async fn calculate_subscriber_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    subscriber: &str,
) -> Result<SubscriberSummary> {
    // Get active subscriptions for the subscriber
    let current_time = Utc::now().timestamp();
    
    let active_subs = schema::profile_subscriptions::table
        .inner_join(schema::profile_subscription_services::table.on(
            schema::profile_subscriptions::service_id.eq(schema::profile_subscription_services::service_id)
        ))
        .filter(schema::profile_subscriptions::subscriber.eq(subscriber))
        .filter(schema::profile_subscriptions::cancelled_at.is_null())
        .filter(schema::profile_subscriptions::expires_at.gt(current_time))
        .select((
            schema::profile_subscriptions::all_columns,
            schema::profile_subscription_services::profile_owner,
            schema::profile_subscription_services::monthly_fee,
        ))
        .load::<(ProfileSubscription, String, i64)>(conn)
        .await?;
    
    let active_subscriptions: Vec<ActiveSubscription> = active_subs
        .into_iter()
        .map(|(sub, profile_owner, monthly_fee)| ActiveSubscription {
            subscription_id: sub.subscription_id,
            service_id: sub.service_id,
            profile_owner,
            monthly_fee,
            expires_at: sub.expires_at,
            auto_renew: sub.auto_renew,
            renewal_count: sub.renewal_count,
        })
        .collect();
    
    // Calculate total spent (simplified)
    let total_spent: i64 = 0; // Would use proper aggregation with cast
    
    // Calculate total refunds (simplified)
    let total_refunds: i64 = 0; // Would use proper aggregation with cast
    
    // Get total subscription count
    let subscription_count = schema::profile_subscriptions::table
        .filter(schema::profile_subscriptions::subscriber.eq(subscriber))
        .count()
        .get_result::<i64>(conn)
        .await?;
    
    Ok(SubscriberSummary {
        subscriber: subscriber.to_string(),
        active_subscriptions,
        total_spent,
        total_refunds,
        subscription_count,
        average_duration: 30.0, // Would calculate from historical data
    })
} 