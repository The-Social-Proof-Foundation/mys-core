// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::social::events::event_utils::{deserialize_u64_from_string, deserialize_optional_u64_from_string};

use crate::social::models::{
    NewSpotBet, NewSpotEventLog, NewSpotPayout, NewSpotRecord, NewSpotRefund, NewSpotResolution,
    SpotConfig,
};

// Matches social_contracts::social_proof_of_truth::SpotBetPlacedEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotBetPlacedEvent {
    pub post_id: String,
    pub user: String,
    pub option_id: u8, // 0-indexed option ID (replaces is_yes)
    pub amount: u64, // Matches contract - all funds go to escrow
}

impl SpotBetPlacedEvent {
    pub fn into_bet_model(&self, epoch: u64, tx: String) -> Result<NewSpotBet> {
        Ok(NewSpotBet {
            post_id: self.post_id.clone(),
            user_address: self.user.clone(),
            option_id: self.option_id as i16,
            escrow_amount: self.amount as i64, // amount goes to escrow
            amm_amount: 0, // No AMM in current contract
            timestamp_epoch: epoch as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::social_proof_of_truth::SpotResolvedEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotResolvedEvent {
    pub post_id: String,
    pub outcome: u8,
    pub total_escrow: u64,
    pub fee_taken: u64,
    pub reasoning: String,
    pub evidence_urls: Vec<String>,
}

impl SpotResolvedEvent {
    pub fn into_resolution_model(&self, epoch: u64, tx: String) -> Result<NewSpotResolution> {
        Ok(NewSpotResolution {
            post_id: self.post_id.clone(),
            outcome: self.outcome as i16,
            total_escrow: self.total_escrow as i64,
            fee_taken: self.fee_taken as i64,
            resolved_epoch: epoch as i64,
            time: Utc::now(),
            transaction_id: tx,
            reasoning: self.reasoning.clone(),
            evidence_urls: serde_json::json!(self.evidence_urls),
        })
    }
}

// Matches social_contracts::social_proof_of_truth::SpotDaoRequiredEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotDaoRequiredEvent {
    pub post_id: String,
    pub confidence_bps: u64,
    pub reasoning: String,
}

// Matches social_contracts::social_proof_of_truth::SpotPayoutEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotPayoutEvent {
    pub post_id: String,
    pub user: String,
    pub amount: u64,
}

impl SpotPayoutEvent {
    pub fn into_model(&self, epoch: u64, tx: String) -> Result<NewSpotPayout> {
        Ok(NewSpotPayout {
            post_id: self.post_id.clone(),
            user_address: self.user.clone(),
            amount: self.amount as i64,
            timestamp_epoch: epoch as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::social_proof_of_truth::SpotRefundEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotRefundEvent {
    pub post_id: String,
    pub user: String,
    pub amount: u64,
}

impl SpotRefundEvent {
    pub fn into_model(&self, epoch: u64, tx: String) -> Result<NewSpotRefund> {
        Ok(NewSpotRefund {
            post_id: self.post_id.clone(),
            user_address: self.user.clone(),
            amount: self.amount as i64,
            timestamp_epoch: epoch as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::social_proof_of_truth::SpotConfigUpdatedEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotConfigUpdatedEvent {
    pub updated_by: String,
    pub enable_flag: bool, // Primary field being toggled - always required
    // Optional config fields - will fallback to latest DB config if missing
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub confidence_threshold_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub resolution_window_epochs: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub max_resolution_window_epochs: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub payout_delay_ms: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub fee_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub fee_split_bps_platform: Option<u64>,
    #[serde(default)]
    pub oracle_address: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub max_single_bet: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub version: Option<u64>,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

impl SpotConfigUpdatedEvent {
    /// Convert to database model using values from event when present, falling back to latest DB config if missing
    /// The enable_flag is always taken from the event (this is the primary field being toggled)
    pub fn into_config_model(
        &self,
        timestamp_ms: u64,
        transaction_id: String,
        time: chrono::DateTime<chrono::Utc>,
        latest_config: Option<&SpotConfig>,
    ) -> crate::social::models::social_proof_of_truth::NewSpotConfig {
        // Helper to get value from event or fallback to latest config
        let get_value = |event_val: Option<u64>, config_val: i64| -> i64 {
            event_val.map(|v| v as i64).unwrap_or(config_val)
        };

        let get_string_value = |event_val: Option<String>, config_val: &str| -> String {
            event_val.unwrap_or_else(|| config_val.to_string())
        };

        crate::social::models::social_proof_of_truth::NewSpotConfig {
            updated_by: self.updated_by.clone(),
            enable_flag: self.enable_flag, // Always use event value for enable_flag
            confidence_threshold_bps: get_value(
                self.confidence_threshold_bps,
                latest_config.map(|c| c.confidence_threshold_bps).unwrap_or(0),
            ),
            resolution_window_epochs: get_value(
                self.resolution_window_epochs,
                latest_config.map(|c| c.resolution_window_epochs).unwrap_or(0),
            ),
            max_resolution_window_epochs: get_value(
                self.max_resolution_window_epochs,
                latest_config.map(|c| c.max_resolution_window_epochs).unwrap_or(0),
            ),
            payout_delay_ms: get_value(
                self.payout_delay_ms,
                latest_config.map(|c| c.payout_delay_ms).unwrap_or(0),
            ),
            fee_bps: get_value(
                self.fee_bps,
                latest_config.map(|c| c.fee_bps).unwrap_or(0),
            ),
            fee_split_bps_platform: get_value(
                self.fee_split_bps_platform,
                latest_config.map(|c| c.fee_split_bps_platform).unwrap_or(0),
            ),
            oracle_address: get_string_value(
                self.oracle_address.clone(),
                latest_config.map(|c| c.oracle_address.as_str()).unwrap_or(""),
            ),
            max_single_bet: get_value(
                self.max_single_bet,
                latest_config.map(|c| c.max_single_bet).unwrap_or(0),
            ),
            version: get_value(
                self.version,
                latest_config.map(|c| c.version).unwrap_or(0),
            ),
            timestamp_ms: timestamp_ms as i64,
            time,
            transaction_id,
        }
    }
}

// Matches social_contracts::social_proof_of_truth::SpotRecordCreatedEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotRecordCreatedEvent {
    pub record_id: String,
    pub post_id: String,
    pub betting_options: Vec<String>, // Array of option labels (e.g., ["Option A", "Option B", "Option C"])
    pub resolution_window_epochs: Option<u64>, // Optional time window for resolution
    pub max_resolution_window_epochs: Option<u64>, // Optional max window
    pub created_epoch: u64,
}

// Matches social_contracts::social_proof_of_truth::SpotBetWithdrawnEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotBetWithdrawnEvent {
    pub post_id: String,
    pub user: String,
    pub option_id: u8, // 0-indexed option ID
    pub amount: u64, // Amount withdrawn
    pub fee_taken: u64, // Fee taken from withdrawal
}

// Helper to create initial record if needed
pub fn default_record_for_post(
    post_id: &str,
    amm_split_bps_used: i32,
    created_epoch: i64,
    version: i64,
    tx: String,
) -> NewSpotRecord {
    let now = Utc::now().naive_utc();
    NewSpotRecord {
        post_id: post_id.to_string(),
        status: 1, // STATUS_OPEN
        outcome: None,
        amm_split_bps_used,
        betting_options: Some(serde_json::json!([])), // Empty array, will be set by SpotRecordCreatedEvent
        option_escrow: Some(serde_json::json!({})), // Empty JSONB object
        resolution_window_epochs: None,
        max_resolution_window_epochs: None,
        created_epoch,
        last_resolution_epoch: None,
        version,
        created_at: now,
        updated_at: now,
        transaction_id: tx,
    }
}

pub fn new_event_log(
    event_type: &str,
    post_id: &str,
    event_data: &Value,
    event_id: &str,
) -> NewSpotEventLog {
    NewSpotEventLog {
        event_type: event_type.to_string(),
        post_id: post_id.to_string(),
        event_data: event_data.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
    }
}

// =============================================================================
// PROCESS FUNCTIONS FOR CHECKPOINT PROCESSOR
// =============================================================================

use anyhow::anyhow;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::social::db::DbConnection;
use crate::social::schema::{
    spot_bets, spot_resolutions, spot_payouts, spot_refunds,
    spot_events, spot_records, spot_config, spot_bet_withdrawals,
};

/// Process a SpotBetPlacedEvent and insert into the database
pub async fn process_spot_bet_placed_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    epoch: u64,
    tx: String,
) -> Result<()> {
    let event: SpotBetPlacedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse SpotBetPlacedEvent: {}", e))?;

    let bet = event.into_bet_model(epoch, tx.clone())?;

    diesel::insert_into(spot_bets::table)
        .values(&bet)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT bet: {}", e))?;

    // Log the event
    let log = new_event_log("SpotBetPlacedEvent", &event.post_id, data, event_id);
    diesel::insert_into(spot_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT event log: {}", e))?;

    tracing::info!("Processed SpotBetPlacedEvent: user {} bet {} on option {} for post {}",
        event.user, event.amount, event.option_id, event.post_id);
    Ok(())
}

/// Process a SpotResolvedEvent and insert into the database
pub async fn process_spot_resolved_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    epoch: u64,
    tx: String,
) -> Result<()> {
    let event: SpotResolvedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse SpotResolvedEvent: {}", e))?;

    let resolution = event.into_resolution_model(epoch, tx.clone())?;

    diesel::insert_into(spot_resolutions::table)
        .values(&resolution)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT resolution: {}", e))?;

    // Update the spot record status
    diesel::update(spot_records::table)
        .filter(spot_records::post_id.eq(&event.post_id))
        .set((
            spot_records::status.eq(2i16), // STATUS_RESOLVED
            spot_records::outcome.eq(Some(event.outcome as i16)),
            spot_records::last_resolution_epoch.eq(Some(epoch as i64)),
            spot_records::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to update SPOT record: {}", e))?;

    // Log the event
    let log = new_event_log("SpotResolvedEvent", &event.post_id, data, event_id);
    diesel::insert_into(spot_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT event log: {}", e))?;

    tracing::info!("Processed SpotResolvedEvent: post {} resolved with outcome {}",
        event.post_id, event.outcome);
    Ok(())
}

/// Process a SpotDaoRequiredEvent (just logs it)
pub async fn process_spot_dao_required_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    _epoch: u64,
    _tx: String,
) -> Result<()> {
    let event: SpotDaoRequiredEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse SpotDaoRequiredEvent: {}", e))?;

    // Log the event
    let log = new_event_log("SpotDaoRequiredEvent", &event.post_id, data, event_id);
    diesel::insert_into(spot_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT event log: {}", e))?;

    tracing::info!("Processed SpotDaoRequiredEvent: post {} requires DAO (confidence: {} bps)",
        event.post_id, event.confidence_bps);
    Ok(())
}

/// Process a SpotPayoutEvent and insert into the database
pub async fn process_spot_payout_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    epoch: u64,
    tx: String,
) -> Result<()> {
    let event: SpotPayoutEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse SpotPayoutEvent: {}", e))?;

    let payout = event.into_model(epoch, tx.clone())?;

    diesel::insert_into(spot_payouts::table)
        .values(&payout)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT payout: {}", e))?;

    // Log the event
    let log = new_event_log("SpotPayoutEvent", &event.post_id, data, event_id);
    diesel::insert_into(spot_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT event log: {}", e))?;

    tracing::info!("Processed SpotPayoutEvent: user {} received {} for post {}",
        event.user, event.amount, event.post_id);
    Ok(())
}

/// Process a SpotRefundEvent and insert into the database
pub async fn process_spot_refund_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    epoch: u64,
    tx: String,
) -> Result<()> {
    let event: SpotRefundEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse SpotRefundEvent: {}", e))?;

    let refund = event.into_model(epoch, tx.clone())?;

    diesel::insert_into(spot_refunds::table)
        .values(&refund)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT refund: {}", e))?;

    // Log the event
    let log = new_event_log("SpotRefundEvent", &event.post_id, data, event_id);
    diesel::insert_into(spot_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT event log: {}", e))?;

    tracing::info!("Processed SpotRefundEvent: user {} refunded {} for post {}",
        event.user, event.amount, event.post_id);
    Ok(())
}

/// Process a SpotConfigUpdatedEvent and insert into the database
pub async fn process_spot_config_updated_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    timestamp_ms: u64,
    tx: String,
) -> Result<()> {
    let event: SpotConfigUpdatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse SpotConfigUpdatedEvent: {}", e))?;

    // Get latest config for fallback values
    let latest_config: Option<crate::social::models::social_proof_of_truth::SpotConfig> = spot_config::table
        .order(spot_config::id.desc())
        .first(conn)
        .await
        .ok();

    let config = event.into_config_model(
        timestamp_ms,
        tx.clone(),
        Utc::now(),
        latest_config.as_ref(),
    );

    diesel::insert_into(spot_config::table)
        .values(&config)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT config: {}", e))?;

    // Log the event
    let log = new_event_log("SpotConfigUpdatedEvent", "", data, event_id);
    diesel::insert_into(spot_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT event log: {}", e))?;

    tracing::info!("Processed SpotConfigUpdatedEvent by {}", event.updated_by);
    Ok(())
}

/// Process a SpotRecordCreatedEvent and insert into the database
pub async fn process_spot_record_created_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    _epoch: u64,
    tx: String,
) -> Result<()> {
    let event: SpotRecordCreatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse SpotRecordCreatedEvent: {}", e))?;

    let record = default_record_for_post(
        &event.post_id,
        0, // amm_split_bps_used
        event.created_epoch as i64,
        1, // version
        tx.clone(),
    );

    // Update with actual values from event
    let betting_options = serde_json::json!(event.betting_options);

    diesel::insert_into(spot_records::table)
        .values(&record)
        .on_conflict(spot_records::post_id)
        .do_update()
        .set((
            spot_records::betting_options.eq(Some(betting_options)),
            spot_records::resolution_window_epochs.eq(event.resolution_window_epochs.map(|v| v as i64)),
            spot_records::max_resolution_window_epochs.eq(event.max_resolution_window_epochs.map(|v| v as i64)),
            spot_records::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT record: {}", e))?;

    // Log the event
    let log = new_event_log("SpotRecordCreatedEvent", &event.post_id, data, event_id);
    diesel::insert_into(spot_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT event log: {}", e))?;

    tracing::info!("Processed SpotRecordCreatedEvent: record {} for post {}",
        event.record_id, event.post_id);
    Ok(())
}

/// Process a SpotBetWithdrawnEvent and insert into the database
pub async fn process_spot_bet_withdrawn_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    epoch: u64,
    tx: String,
) -> Result<()> {
    let event: SpotBetWithdrawnEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse SpotBetWithdrawnEvent: {}", e))?;

    // Insert withdrawal record
    let withdrawal = crate::social::models::NewSpotBetWithdrawal {
        post_id: event.post_id.clone(),
        user_address: event.user.clone(),
        option_id: event.option_id as i16,
        amount: event.amount as i64,
        fee_taken: event.fee_taken as i64,
        timestamp_epoch: epoch as i64,
        time: Utc::now(),
        transaction_id: tx.clone(),
    };

    diesel::insert_into(spot_bet_withdrawals::table)
        .values(&withdrawal)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT bet withdrawal: {}", e))?;

    // Log the event
    let log = new_event_log("SpotBetWithdrawnEvent", &event.post_id, data, event_id);
    diesel::insert_into(spot_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert SPOT event log: {}", e))?;

    tracing::info!("Processed SpotBetWithdrawnEvent: user {} withdrew {} from post {}",
        event.user, event.amount, event.post_id);
    Ok(())
}
