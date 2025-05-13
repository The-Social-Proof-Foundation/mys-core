// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents an event with metadata from MySocial blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysEvent {
    /// ID of the package that emitted the event
    pub package_id: String,
    /// Module where the event was defined
    pub transaction_module: String,
    /// Address of the sender who triggered the event
    pub sender: String,
    /// Type of the event
    pub type_: String,
    /// JSON contents of the event
    pub contents: Value,
}

/// Event metadata with additional context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysEventWithMetadata {
    /// The event itself
    pub event: MysEvent,
    /// Transaction digest where the event was emitted
    pub transaction_digest: String,
    /// Timestamp of the event (Unix timestamp in seconds)
    pub timestamp: i64,
    /// Sequence number for ordering
    pub sequence_number: u64,
}

/// Configuration options for a blockchain handler
#[derive(Debug, Clone)]
pub struct HandlerOptions {
    /// Whether to process historical events
    pub process_historical: bool,
    /// Whether to index in real-time
    pub real_time: bool,
    /// Max number of events to process in a batch
    pub batch_size: usize,
    /// Sleep time between batches (in milliseconds)
    pub sleep_time_ms: u64,
}

/// Trait for blockchain event handlers
#[async_trait]
pub trait BlockchainHandler: Send + Sync {
    /// Handle a blockchain event
    async fn handle_event(&self, event: MysEventWithMetadata, options: &HandlerOptions) -> Result<()>;
} 