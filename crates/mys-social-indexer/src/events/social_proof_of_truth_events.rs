// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{
    NewSpotBet, NewSpotEventLog, NewSpotPayout, NewSpotRecord, NewSpotRefund, NewSpotResolution,
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
    pub enable_flag: bool,
    pub confidence_threshold_bps: u64,
    pub resolution_window_epochs: u64,
    pub max_resolution_window_epochs: u64,
    pub payout_delay_epochs: u64,
    pub fee_bps: u64,
    pub fee_split_bps_platform: u64,
    pub platform_treasury: String,
    pub chain_treasury: String,
    pub oracle_address: String,
    pub max_single_bet: u64,
    pub timestamp: u64,
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
