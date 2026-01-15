// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, SmallInt, Text};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;

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

#[derive(Debug, Deserialize)]
pub struct PolicyFilters {
    pub insured: Option<String>,
    pub market_id: Option<String>,
    pub vault_id: Option<String>,
    pub status: Option<i16>,
}

/// Get current insurance configuration
#[derive(QueryableByName, Serialize)]
pub struct InsuranceConfigInfo {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    pub enable_flag: bool,
    #[diesel(sql_type = BigInt)]
    pub min_coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_duration_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

pub async fn get_insurance_configuration(State(pool): State<DbPool>) -> impl IntoResponse {
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
        SELECT updated_by, enable_flag, min_coverage_bps, max_coverage_bps, max_duration_ms,
               fee_bps, version, timestamp_ms, time, transaction_id
        FROM insurance_config
        ORDER BY time DESC
        LIMIT 1
    ";
    match diesel::sql_query(sql)
        .get_result::<InsuranceConfigInfo>(&mut conn)
        .await
    {
        Ok(config) => Json(config).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Insurance configuration not found").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
            .into_response(),
    }
}

/// Get vault details
#[derive(QueryableByName, Serialize)]
pub struct VaultInfo {
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub underwriter: String,
    #[diesel(sql_type = BigInt)]
    pub capital_balance: i64,
    #[diesel(sql_type = BigInt)]
    pub reserved: i64,
    #[diesel(sql_type = BigInt)]
    pub base_rate_bps_per_day: i64,
    #[diesel(sql_type = BigInt)]
    pub utilization_multiplier_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub max_exposure_per_market: i64,
    #[diesel(sql_type = BigInt)]
    pub max_exposure_per_user: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_vault(State(pool): State<DbPool>, Path(vault_id): Path<String>) -> impl IntoResponse {
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
        SELECT vault_id, underwriter, capital_balance, reserved, base_rate_bps_per_day,
               utilization_multiplier_bps, max_exposure_per_market, max_exposure_per_user,
               version, created_at, updated_at
        FROM insurance_vaults
        WHERE vault_id = $1
    ";
    match diesel::sql_query(sql)
        .bind::<Text, _>(&vault_id)
        .get_result::<VaultInfo>(&mut conn)
        .await
    {
        Ok(vault) => Json(vault).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Vault not found").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
            .into_response(),
    }
}

/// List all vaults
#[derive(QueryableByName, Serialize)]
pub struct VaultRow {
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = Text)]
    pub underwriter: String,
    #[diesel(sql_type = BigInt)]
    pub capital_balance: i64,
    #[diesel(sql_type = BigInt)]
    pub reserved: i64,
}

pub async fn list_vaults(
    State(pool): State<DbPool>,
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
    let sql = "
        SELECT vault_id, underwriter, capital_balance, reserved
        FROM insurance_vaults
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    ";
    match diesel::sql_query(sql)
        .bind::<BigInt, _>(&p.limit())
        .bind::<BigInt, _>(&p.offset())
        .get_results::<VaultRow>(&mut conn)
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

/// Get policy details
#[derive(QueryableByName, Serialize)]
pub struct PolicyInfo {
    #[diesel(sql_type = Text)]
    pub policy_id: String,
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = Text)]
    pub insured: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub covered_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub premium_paid: i64,
    #[diesel(sql_type = BigInt)]
    pub start_time_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub expiry_time_ms: i64,
    #[diesel(sql_type = Text)]
    pub vault_id: String,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
}

pub async fn get_policy(
    State(pool): State<DbPool>,
    Path(policy_id): Path<String>,
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
    let sql = "
        SELECT policy_id, market_id, insured, option_id, covered_amount, coverage_bps,
               premium_paid, start_time_ms, expiry_time_ms, vault_id, status
        FROM insurance_policies
        WHERE policy_id = $1
    ";
    match diesel::sql_query(sql)
        .bind::<Text, _>(&policy_id)
        .get_result::<PolicyInfo>(&mut conn)
        .await
    {
        Ok(policy) => Json(policy).into_response(),
        Err(diesel::result::Error::NotFound) => {
            (StatusCode::NOT_FOUND, "Policy not found").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
            .into_response(),
    }
}

/// List policies with filters
#[derive(QueryableByName, Serialize)]
pub struct PolicyRow {
    #[diesel(sql_type = Text)]
    pub policy_id: String,
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = Text)]
    pub insured: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub covered_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub premium_paid: i64,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
}

pub async fn list_policies(
    State(pool): State<DbPool>,
    Query(filters): Query<PolicyFilters>,
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

    // Build SQL query with conditional filters using string interpolation for simplicity
    // Note: In production, consider using a query builder library for better safety
    let mut sql = String::from(
        "SELECT policy_id, market_id, insured, option_id, covered_amount, premium_paid, status
         FROM insurance_policies WHERE 1=1",
    );

    if let Some(ref insured) = filters.insured {
        sql.push_str(&format!(" AND insured = '{}'", insured.replace("'", "''")));
    }
    if let Some(ref market_id) = filters.market_id {
        sql.push_str(&format!(" AND market_id = '{}'", market_id.replace("'", "''")));
    }
    if let Some(ref vault_id) = filters.vault_id {
        sql.push_str(&format!(" AND vault_id = '{}'", vault_id.replace("'", "''")));
    }
    if let Some(status) = filters.status {
        sql.push_str(&format!(" AND status = {}", status));
    }

    sql.push_str(&format!(" ORDER BY created_at DESC LIMIT {} OFFSET {}", p.limit(), p.offset()));

    let query = diesel::sql_query(&sql);

    match query.get_results::<PolicyRow>(&mut conn).await {
        Ok(rows) => Json(rows).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Query error: {}", e),
        )
            .into_response(),
    }
}

/// Get vault transaction history
#[derive(QueryableByName, Serialize)]
pub struct VaultTransactionRow {
    #[diesel(sql_type = Text)]
    pub transaction_type: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub balance_after: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
}

pub async fn list_vault_transactions(
    State(pool): State<DbPool>,
    Path(vault_id): Path<String>,
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
    let sql = "
        SELECT transaction_type, amount, balance_after, timestamp_ms
        FROM insurance_vault_transactions
        WHERE vault_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    match diesel::sql_query(sql)
        .bind::<Text, _>(&vault_id)
        .bind::<BigInt, _>(&p.limit())
        .bind::<BigInt, _>(&p.offset())
        .get_results::<VaultTransactionRow>(&mut conn)
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

/// Get vault exposure analytics
#[derive(QueryableByName, Serialize)]
pub struct VaultExposureRow {
    #[diesel(sql_type = Text)]
    pub market_id: String,
    #[diesel(sql_type = SmallInt)]
    pub option_id: i16,
    #[diesel(sql_type = BigInt)]
    pub total_exposure: i64,
}

pub async fn get_vault_exposures(
    State(pool): State<DbPool>,
    Path(vault_id): Path<String>,
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
    let sql = "
        SELECT market_id, option_id, SUM(reserved_amount) as total_exposure
        FROM insurance_market_exposures
        WHERE vault_id = $1
        GROUP BY market_id, option_id
        ORDER BY total_exposure DESC
    ";
    match diesel::sql_query(sql)
        .bind::<Text, _>(&vault_id)
        .get_results::<VaultExposureRow>(&mut conn)
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

/// Get policies for a specific SPoT market
pub async fn list_market_policies(
    State(pool): State<DbPool>,
    Path(market_id): Path<String>,
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
    let sql = "
        SELECT policy_id, market_id, insured, option_id, covered_amount, premium_paid, status
        FROM insurance_policies
        WHERE market_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    match diesel::sql_query(sql)
        .bind::<Text, _>(&market_id)
        .bind::<BigInt, _>(&p.limit())
        .bind::<BigInt, _>(&p.offset())
        .get_results::<PolicyRow>(&mut conn)
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

