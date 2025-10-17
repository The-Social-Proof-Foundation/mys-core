// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::data_types::PgTimestamp;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

use mys_indexer_builder::{Task, LIVE_TASK_TARGET_CHECKPOINT};

use crate::schema::{
    bridge_treasury_balances, bridge_treasury_events, governance_actions, mys_error_transactions,
    mys_progress_store, progress_store, token_transfer, token_transfer_data,
};

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug)]
#[diesel(table_name = progress_store, primary_key(task_name))]
pub struct ProgressStore {
    pub task_name: String,
    pub checkpoint: i64,
    pub target_checkpoint: i64,
    pub timestamp: Option<PgTimestamp>,
}

impl From<ProgressStore> for Task {
    fn from(value: ProgressStore) -> Self {
        Self {
            task_name: value.task_name,
            start_checkpoint: value.checkpoint as u64,
            target_checkpoint: value.target_checkpoint as u64,
            // Ok to unwrap, timestamp is defaulted to now() in database
            timestamp: value.timestamp.expect("Timestamp not set").0 as u64,
            is_live_task: value.target_checkpoint == LIVE_TASK_TARGET_CHECKPOINT,
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug)]
#[diesel(table_name = mys_progress_store, primary_key(txn_digest))]
pub struct MysProgressStore {
    pub id: i32, // Dummy value
    pub txn_digest: Vec<u8>,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug)]
#[diesel(table_name = token_transfer, primary_key(chain_id, nonce))]
pub struct TokenTransfer {
    pub chain_id: i32,
    pub nonce: i64,
    pub status: String,
    pub block_height: i64,
    pub timestamp_ms: i64,
    pub txn_hash: Vec<u8>,
    pub txn_sender: Vec<u8>,
    pub gas_usage: i64,
    pub data_source: String,
    pub is_finalized: bool,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug)]
#[diesel(table_name = token_transfer_data, primary_key(chain_id, nonce))]
pub struct TokenTransferData {
    pub chain_id: i32,
    pub nonce: i64,
    pub block_height: i64,
    pub timestamp_ms: i64,
    pub txn_hash: Vec<u8>,
    pub sender_address: Vec<u8>,
    pub destination_chain: i32,
    pub recipient_address: Vec<u8>,
    pub token_id: i32,
    pub amount: i64,
    pub is_finalized: bool,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug)]
#[diesel(table_name = mys_error_transactions, primary_key(txn_digest))]
pub struct MysErrorTransactions {
    pub txn_digest: Vec<u8>,
    pub sender_address: Vec<u8>,
    pub timestamp_ms: i64,
    pub failure_status: String,
    pub cmd_idx: Option<i64>,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug)]
#[diesel(table_name = governance_actions, primary_key(txn_digest))]
pub struct GovernanceAction {
    pub nonce: Option<i64>,
    pub data_source: String,
    pub txn_digest: Vec<u8>,
    pub sender_address: Vec<u8>,
    pub timestamp_ms: i64,
    pub action: String,
    pub data: serde_json::Value,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, Clone)]
#[diesel(table_name = bridge_treasury_balances, primary_key(id))]
pub struct BridgeTreasuryBalance {
    pub id: i32,
    pub token_type: String,
    pub token_id: i32,
    pub total_locked: i64,
    pub total_unlocked: i64,
    pub net_balance: i64,
    pub last_updated_block: i64,
    pub last_updated_timestamp: i64,
    pub created_at: Option<PgTimestamp>,
    pub updated_at: Option<PgTimestamp>,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, Clone)]
#[diesel(table_name = bridge_treasury_events, primary_key(id))]
pub struct BridgeTreasuryEvent {
    pub id: i32,
    pub token_type: String,
    pub token_id: i32,
    pub event_type: String,
    pub amount: i64,
    pub tx_digest: String,
    pub block_height: i64,
    pub timestamp_ms: i64,
    pub sender_address: Option<Vec<u8>>,
    pub created_at: Option<PgTimestamp>,
}
