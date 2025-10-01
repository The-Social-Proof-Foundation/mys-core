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
    pub is_yes: bool,
    pub escrow_amount: u64,
    pub amm_amount: u64,
}

impl SpotBetPlacedEvent {
    pub fn into_bet_model(&self, epoch: u64, tx: String) -> Result<NewSpotBet> {
        Ok(NewSpotBet {
            post_id: self.post_id.clone(),
            user_address: self.user.clone(),
            is_yes: self.is_yes,
            escrow_amount: self.escrow_amount as i64,
            amm_amount: self.amm_amount as i64,
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
        })
    }
}

// Matches social_contracts::social_proof_of_truth::SpotDaoRequiredEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotDaoRequiredEvent {
    pub post_id: String,
    pub confidence_bps: u64,
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
        total_yes_escrow: 0,
        total_no_escrow: 0,
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
