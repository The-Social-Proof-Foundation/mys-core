// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use anyhow::Result;
use mys_sdk::MysClientBuilder;
use mys_types::base_types::MysAddress;
use serde_json;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, warn};

use crate::social::db::Database;
use crate::social::models::{get_current_treasury_details, get_treasury_history};

/// Get current ecosystem treasury details
/// Returns full treasury information including address, updated_by, timestamp, time, transaction_id, and balance
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

    let treasury = match get_current_treasury_details(&mut conn).await {
        Ok(treasury) => treasury,
        Err(e) => {
            error!("Failed to get current treasury details: {}", e);
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("Treasury details not found: {}", e)
                })),
            )
                .into_response();
        }
    };

    // Query balance from blockchain
    let balance = match query_treasury_balance(&treasury.treasury_address).await {
        Ok(balance) => Some(balance),
        Err(e) => {
            warn!("Failed to query treasury balance: {}", e);
            None // Don't fail the request if balance query fails
        }
    };

    let mut response = serde_json::json!({
        "treasury_address": treasury.treasury_address,
        "updated_by": treasury.updated_by,
        "timestamp_ms": treasury.timestamp_ms,
        "time": treasury.time.timestamp(),
        "transaction_id": treasury.transaction_id
    });

    if let Some(balance) = balance {
        response["balance"] = serde_json::json!({
            "total_balance": balance.total_balance,
            "coin_type": balance.coin_type,
            "coin_object_count": balance.coin_object_count
        });
    }

    Json(response).into_response()
}

/// Query the MYS coin balance for a given address
async fn query_treasury_balance(address: &str) -> Result<mys_json_rpc_types::Balance> {
    // Get RPC URL from environment or use default
    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "http://fullnode.testnet.mysocial.network:9000".to_string());

    // Create MySocial client
    let client = MysClientBuilder::default()
        .build(&rpc_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create MySocial client: {}", e))?;

    // Parse address
    let mys_address = MysAddress::from_str(address)
        .map_err(|e| anyhow::anyhow!("Invalid address format: {}", e))?;

    // Query balance for MYS coin type (default)
    let balance = client
        .coin_read_api()
        .get_balance(mys_address, None)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to query balance: {}", e))?;

    Ok(balance)
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

