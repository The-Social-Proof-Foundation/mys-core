// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::{
    spot_bets, spot_events, spot_payouts, spot_records, spot_refunds, spot_resolutions,
};

// =============================================================================
// Core SPoT record
// =============================================================================

#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = spot_records)]
pub struct SpotRecord {
    pub id: i32,
    pub post_id: String,
    pub status: i16,
    pub outcome: Option<i16>,
    pub amm_split_bps_used: i32,
    pub total_yes_escrow: i64,
    pub total_no_escrow: i64,
    pub created_epoch: i64,
    pub last_resolution_epoch: Option<i64>,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = spot_records)]
pub struct NewSpotRecord {
    pub post_id: String,
    pub status: i16,
    pub outcome: Option<i16>,
    pub amm_split_bps_used: i32,
    pub total_yes_escrow: i64,
    pub total_no_escrow: i64,
    pub created_epoch: i64,
    pub last_resolution_epoch: Option<i64>,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = spot_records)]
pub struct UpdateSpotRecord {
    pub status: Option<i16>,
    pub outcome: Option<Option<i16>>,
    pub total_yes_escrow: Option<i64>,
    pub total_no_escrow: Option<i64>,
    pub last_resolution_epoch: Option<i64>,
    pub updated_at: NaiveDateTime,
}

// =============================================================================
// Bets (hypertable by time)
// =============================================================================

#[derive(QueryableByName, Queryable, Debug)]
pub struct SpotBet {
    #[diesel(sql_type = diesel::sql_types::Int4)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub post_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub user_address: String,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub is_yes: bool,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub escrow_amount: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub amm_amount: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub timestamp_epoch: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = spot_bets)]
pub struct NewSpotBet {
    pub post_id: String,
    pub user_address: String,
    pub is_yes: bool,
    pub escrow_amount: i64,
    pub amm_amount: i64,
    pub timestamp_epoch: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

// =============================================================================
// Payouts and refunds (hypertables)
// =============================================================================

#[derive(Insertable, Debug)]
#[diesel(table_name = spot_payouts)]
pub struct NewSpotPayout {
    pub post_id: String,
    pub user_address: String,
    pub amount: i64,
    pub timestamp_epoch: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = spot_refunds)]
pub struct NewSpotRefund {
    pub post_id: String,
    pub user_address: String,
    pub amount: i64,
    pub timestamp_epoch: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

// =============================================================================
// Resolution summary
// =============================================================================

#[derive(Insertable, Debug)]
#[diesel(table_name = spot_resolutions)]
pub struct NewSpotResolution {
    pub post_id: String,
    pub outcome: i16,
    pub total_escrow: i64,
    pub fee_taken: i64,
    pub resolved_epoch: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub reasoning: String,
    pub evidence_urls: serde_json::Value,
}

// =============================================================================
// Event audit log
// =============================================================================

#[derive(Insertable, Debug)]
#[diesel(table_name = spot_events)]
pub struct NewSpotEventLog {
    pub event_type: String,
    pub post_id: String,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// =============================================================================
// Unified SPoT events table model
// =============================================================================

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::social_proof_of_truth)]
pub struct NewSocialProofOfTruthEvent {
    pub event_type: String,
    pub post_id: String,
    pub user_address: Option<String>,
    pub is_yes: Option<bool>,
    pub escrow_amount: Option<i64>,
    pub amm_amount: Option<i64>,
    pub amount: Option<i64>,
    pub outcome: Option<i16>,
    pub total_escrow: Option<i64>,
    pub fee_taken: Option<i64>,
    pub confidence_bps: Option<i64>,
    pub timestamp_epoch: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub event_id: Option<String>,
    pub transaction_id: Option<String>,
    pub raw_event: Option<serde_json::Value>,
}
