// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json;
use std::sync::Arc;
use tracing::error;

use crate::db::Database;
use crate::models::{get_current_treasury_address, get_treasury_history};

/// Get current ecosystem treasury address
pub async fn get_current_treasury(State(db): State<Arc<Database>>) -> Response {
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

    match get_current_treasury_address(&mut conn).await {
        Ok(address) => Json(serde_json::json!({
            "treasury_address": address
        }))
        .into_response(),
        Err(e) => {
            error!("Failed to get current treasury address: {}", e);
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("Treasury address not found: {}", e)
                })),
            )
                .into_response()
        }
    }
}

/// Get treasury update history
pub async fn get_treasury_history_endpoint(
    State(db): State<Arc<Database>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Response {
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

    let limit = params
        .get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(100);

    match get_treasury_history(&mut conn, limit).await {
        Ok(history) => Json(serde_json::json!({
            "history": history,
            "count": history.len()
        }))
        .into_response(),
        Err(e) => {
            error!("Failed to get treasury history: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to get treasury history: {}", e)
                })),
            )
                .into_response()
        }
    }
}

