// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::social::db::DbPool;

#[derive(Debug, Deserialize)]
pub struct PageParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
impl PageParams {
    fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
    fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).clamp(1, 100)
    }
    fn offset(&self) -> i64 {
        (self.page() - 1) * self.limit()
    }
}

#[derive(Debug, Serialize)]
pub struct SpotRecordResponse {
    pub post_id: String,
    pub status: i16,
    pub outcome: Option<i16>,
    pub betting_options: Vec<String>,
    pub option_escrow: std::collections::HashMap<String, i64>,
    pub resolution_window_epochs: Option<i64>,
    pub max_resolution_window_epochs: Option<i64>,
    pub created_epoch: i64,
    pub last_resolution_epoch: Option<i64>,
}

#[derive(QueryableByName, Serialize)]
pub struct SpotBetRow {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub post_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub user_address: String,
    #[diesel(sql_type = diesel::sql_types::SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub escrow_amount: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub amm_amount: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub timestamp_epoch: i64,
}

pub async fn get_spot_record(
    State(pool): State<DbPool>,
    Path(post_id): Path<String>,
) -> impl IntoResponse {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
                .into_response()
        }
    };
    let sql = "SELECT post_id, status, outcome, betting_options, option_escrow, resolution_window_epochs, max_resolution_window_epochs, created_epoch, last_resolution_epoch FROM spot_records WHERE post_id = $1";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type=diesel::sql_types::Text)]
        post_id: String,
        #[diesel(sql_type=diesel::sql_types::SmallInt)]
        status: i16,
        #[diesel(sql_type=diesel::sql_types::Nullable<diesel::sql_types::SmallInt>)]
        outcome: Option<i16>,
        #[diesel(sql_type=diesel::sql_types::Nullable<diesel::sql_types::Jsonb>)]
        betting_options: Option<serde_json::Value>,
        #[diesel(sql_type=diesel::sql_types::Nullable<diesel::sql_types::Jsonb>)]
        option_escrow: Option<serde_json::Value>,
        #[diesel(sql_type=diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
        resolution_window_epochs: Option<i64>,
        #[diesel(sql_type=diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
        max_resolution_window_epochs: Option<i64>,
        #[diesel(sql_type=diesel::sql_types::BigInt)]
        created_epoch: i64,
        #[diesel(sql_type=diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
        last_resolution_epoch: Option<i64>,
    }
    match diesel::sql_query(sql)
        .bind::<diesel::sql_types::Text, _>(&post_id)
        .get_result::<Row>(&mut conn)
        .await
    {
        Ok(r) => {
            // Parse JSONB fields
            let betting_options: Vec<String> = r.betting_options
                .and_then(|v| serde_json::from_value::<Vec<String>>(v).ok())
                .unwrap_or_default();
            
            let option_escrow: std::collections::HashMap<String, i64> = r.option_escrow
                .and_then(|v| serde_json::from_value::<std::collections::HashMap<String, i64>>(v).ok())
                .unwrap_or_default();

            Json(SpotRecordResponse {
                post_id: r.post_id,
                status: r.status,
                outcome: r.outcome,
                betting_options,
                option_escrow,
                resolution_window_epochs: r.resolution_window_epochs,
                max_resolution_window_epochs: r.max_resolution_window_epochs,
                created_epoch: r.created_epoch,
                last_resolution_epoch: r.last_resolution_epoch,
            })
            .into_response()
        },
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "SPoT record not found").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
            .into_response(),
    }
}

pub async fn list_spot_bets(
    State(pool): State<DbPool>,
    Path(post_id): Path<String>,
    Query(p): Query<PageParams>,
) -> impl IntoResponse {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
                .into_response()
        }
    };
    let sql = "SELECT post_id, user_address, option_id, escrow_amount, amm_amount, timestamp_epoch FROM spot_bets WHERE post_id = $1 ORDER BY time DESC LIMIT $2 OFFSET $3";
    match diesel::sql_query(sql)
        .bind::<diesel::sql_types::Text, _>(&post_id)
        .bind::<diesel::sql_types::BigInt, _>(&p.limit())
        .bind::<diesel::sql_types::BigInt, _>(&p.offset())
        .get_results::<SpotBetRow>(&mut conn)
        .await
    {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
            .into_response(),
    }
}

#[derive(QueryableByName, Serialize)]
pub struct TransferRow {
    #[diesel(sql_type=diesel::sql_types::Text)]
    pub user_address: String,
    #[diesel(sql_type=diesel::sql_types::BigInt)]
    pub amount: i64,
    #[diesel(sql_type=diesel::sql_types::BigInt)]
    pub timestamp_epoch: i64,
}

pub async fn list_spot_payouts(
    State(pool): State<DbPool>,
    Path(post_id): Path<String>,
    Query(p): Query<PageParams>,
) -> impl IntoResponse {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
                .into_response()
        }
    };
    let sql = "SELECT user_address, amount, timestamp_epoch FROM spot_payouts WHERE post_id = $1 ORDER BY time DESC LIMIT $2 OFFSET $3";
    match diesel::sql_query(sql)
        .bind::<diesel::sql_types::Text, _>(&post_id)
        .bind::<diesel::sql_types::BigInt, _>(&p.limit())
        .bind::<diesel::sql_types::BigInt, _>(&p.offset())
        .get_results::<TransferRow>(&mut conn)
        .await
    {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
            .into_response(),
    }
}

pub async fn list_spot_refunds(
    State(pool): State<DbPool>,
    Path(post_id): Path<String>,
    Query(p): Query<PageParams>,
) -> impl IntoResponse {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
                .into_response()
        }
    };
    let sql = "SELECT user_address, amount, timestamp_epoch FROM spot_refunds WHERE post_id = $1 ORDER BY time DESC LIMIT $2 OFFSET $3";
    match diesel::sql_query(sql)
        .bind::<diesel::sql_types::Text, _>(&post_id)
        .bind::<diesel::sql_types::BigInt, _>(&p.limit())
        .bind::<diesel::sql_types::BigInt, _>(&p.offset())
        .get_results::<TransferRow>(&mut conn)
        .await
    {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
            .into_response(),
    }
}

/// Get current Social Proof of Truth configuration
#[derive(QueryableByName, Serialize)]
pub struct SpotConfigInfo {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub updated_by: String,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub enable_flag: bool,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub confidence_threshold_bps: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub resolution_window_epochs: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub max_resolution_window_epochs: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub payout_delay_ms: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub fee_bps: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub fee_split_bps_platform: i64,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub oracle_address: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub max_single_bet: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub version: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

pub async fn get_spot_configuration(State(pool): State<DbPool>) -> impl IntoResponse {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
                .into_response()
        }
    };
    let sql = "
        SELECT updated_by, enable_flag, confidence_threshold_bps, resolution_window_epochs,
               max_resolution_window_epochs, payout_delay_ms, fee_bps, fee_split_bps_platform,
               oracle_address, max_single_bet, version, timestamp_ms,
               time, transaction_id
        FROM spot_config
        ORDER BY time DESC
        LIMIT 1
    ";
    match diesel::sql_query(sql)
        .get_result::<SpotConfigInfo>(&mut conn)
        .await
    {
        Ok(config) => Json(config).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "SPoT configuration not found").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
            .into_response(),
    }
}
