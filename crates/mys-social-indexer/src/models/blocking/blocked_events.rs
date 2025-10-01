// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::schema::blocked_events;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Blocked event model - represents a complete audit trail of blocking events
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = blocked_events)]
pub struct BlockedEvent {
    pub id: i32,
    pub event_id: Option<String>,
    pub event_type: String,
    pub blocker_address: String,
    pub blocked_address: Option<String>,
    pub block_list_address: Option<String>,
    pub raw_event_data: Option<serde_json::Value>,
    pub processed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

/// DTO for inserting a new blocked event
#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = blocked_events)]
pub struct NewBlockedEvent {
    pub event_id: Option<String>,
    pub event_type: String,
    pub blocker_address: String,
    pub blocked_address: Option<String>,
    pub block_list_address: Option<String>,
    pub raw_event_data: Option<serde_json::Value>,
    pub processed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl NewBlockedEvent {
    /// Create a new block event
    pub fn new_block_event(
        event_id: Option<String>,
        blocker_address: String,
        blocked_address: String,
        block_list_address: Option<String>,
        raw_event_data: Option<serde_json::Value>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            event_id,
            event_type: "block".to_string(),
            blocker_address,
            blocked_address: Some(blocked_address),
            block_list_address,
            raw_event_data,
            processed_at: chrono::Utc::now().naive_utc(),
            created_at,
        }
    }

    /// Create a new unblock event
    pub fn new_unblock_event(
        event_id: Option<String>,
        blocker_address: String,
        blocked_address: String,
        block_list_address: Option<String>,
        raw_event_data: Option<serde_json::Value>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            event_id,
            event_type: "unblock".to_string(),
            blocker_address,
            blocked_address: Some(blocked_address),
            block_list_address,
            raw_event_data,
            processed_at: chrono::Utc::now().naive_utc(),
            created_at,
        }
    }

    /// Create a new block list created event
    pub fn new_block_list_created_event(
        event_id: Option<String>,
        blocker_address: String,
        block_list_address: String,
        raw_event_data: Option<serde_json::Value>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            event_id,
            event_type: "block_list_created".to_string(),
            blocker_address,
            blocked_address: None, // No specific blocked address for block list creation
            block_list_address: Some(block_list_address),
            raw_event_data,
            processed_at: chrono::Utc::now().naive_utc(),
            created_at,
        }
    }
}
