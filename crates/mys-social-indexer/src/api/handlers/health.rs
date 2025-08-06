// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tokio::time::{timeout, Duration};
use tracing::{error, info, warn};
use chrono;
use crate::db::DbPool;

/// Health check endpoint with enhanced error handling for Railway deployments
pub async fn health_check(State(pool): State<DbPool>) -> impl IntoResponse {
    info!("Health check requested");
    
    // Add timeout to prevent hanging health checks
    let connection_timeout = Duration::from_secs(10);
    
    match timeout(connection_timeout, pool.get()).await {
        Ok(Ok(_conn)) => {
            info!("Health check passed: database connection successful");
            (
                StatusCode::OK,
                Json(json!({
                    "status": "healthy",
                    "message": "API server is running and database is accessible",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }))
            )
        },
        Ok(Err(pool_error)) => {
            error!("Health check failed: database connection error: {}", pool_error);
            warn!("This might be due to: 1) Database not ready, 2) Wrong DATABASE_URL, 3) Missing SSL/TLS config, 4) Network issues");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "status": "unhealthy",
                    "message": format!("Database connection failed: {}", pool_error),
                    "error_type": "database_connection_error",
                    "suggestions": [
                        "Check if DATABASE_URL is correctly set",
                        "Verify database is running and accessible",
                        "Check SSL/TLS configuration for cloud databases",
                        "Ensure firewall allows database connections"
                    ],
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }))
            )
        },
        Err(_timeout_error) => {
            error!("Health check failed: connection pool timeout after 10 seconds");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "status": "unhealthy",
                    "message": "Database connection timed out after 10 seconds",
                    "error_type": "connection_timeout",
                    "suggestions": [
                        "Database might be starting up",
                        "Network connectivity issues",
                        "Database server overloaded"
                    ],
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }))
            )
        }
    }
}