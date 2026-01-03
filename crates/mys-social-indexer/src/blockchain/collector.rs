// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info};

use mys_sdk::{rpc_types::EventFilter, MysClientBuilder};

use crate::config::Config;
use crate::db::Database;
use crate::blockchain::listener::BlockchainEvent;
use crate::blockchain::watermark::WatermarkManager;

/// Transaction batch containing all events from a single transaction
#[derive(Debug, Clone)]
pub struct TransactionBatch {
    /// Transaction digest
    pub tx_digest: String,
    /// Checkpoint sequence number (if available)
    pub checkpoint_seq: Option<u64>,
    /// Events from this transaction, ordered by event_seq
    pub events: Vec<BlockchainEvent>,
    /// Timestamp of the transaction
    pub timestamp_ms: u64,
}

/// Collector that groups events by transaction and updates ReaderWatermark
pub struct Collector {
    /// Configuration
    config: Config,
    /// Database connection
    db: Arc<Database>,
    /// Watermark manager
    watermark_manager: WatermarkManager,
    /// Channel sender for transaction batches
    batch_tx: mpsc::Sender<TransactionBatch>,
}

impl Collector {
    /// Create a new collector
    pub fn new(
        config: Config,
        db: Arc<Database>,
        batch_tx: mpsc::Sender<TransactionBatch>,
    ) -> Self {
        let watermark_manager = WatermarkManager::new(db.clone());
        Self {
            config,
            db,
            watermark_manager,
            batch_tx,
        }
    }

    /// Start collecting events from the blockchain
    pub async fn start(&self) -> Result<()> {
        info!("Starting event collector");
        info!(
            "Attempting to connect to RPC: {}",
            self.config.blockchain.rpc_url
        );

        // Create MySocial client
        let client = MysClientBuilder::default()
            .build(&self.config.blockchain.rpc_url)
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to build MySocial client - RPC: {}, Error: {}",
                    self.config.blockchain.rpc_url,
                    e
                )
            })?;

        info!(
            "✅ Successfully connected to blockchain node: {}",
            self.config.blockchain.rpc_url
        );

        // Get the last committer watermark to resume from
        let last_committed_checkpoint = self
            .watermark_manager
            .get_committer_watermark()
            .await?
            .unwrap_or(0) as u64;

        info!(
            "Resuming from checkpoint: {}",
            last_committed_checkpoint
        );

        // Create event filter for all events
        let event_filter = EventFilter::All([]);

        // Create polling interval
        let mut interval = interval(Duration::from_millis(
            self.config.blockchain.poll_interval_ms,
        ));

        // Track consecutive errors
        let mut consecutive_errors = 0;
        const MAX_CONSECUTIVE_ERRORS: u32 = 5;
        let mut client = client;

        // Track last processed checkpoint for watermark updates
        let mut last_processed_checkpoint: u64 = last_committed_checkpoint;

        // Poll for events
        loop {
            interval.tick().await;

            match client
                .event_api()
                .query_events(
                    event_filter.clone(),
                    None,
                    Some(self.config.blockchain.batch_size),
                    true, // descending order to get newest first
                )
                .await
            {
                Ok(events) => {
                    consecutive_errors = 0;

                    if events.data.is_empty() {
                        debug!("No new events found");
                        continue;
                    }

                    // Group events by transaction digest
                    let mut tx_batches: HashMap<String, Vec<_>> = HashMap::new();

                    // Process events in reverse order (oldest to newest) and collect them
                    let events_vec: Vec<_> = events.data.into_iter().rev().collect();
                    
                    for event in events_vec {
                        let tx_digest = event.id.tx_digest.to_string();
                        tx_batches
                            .entry(tx_digest.clone())
                            .or_insert_with(Vec::new)
                            .push(event);
                    }

                    let batch_count = tx_batches.len();

                    // Create TransactionBatch for each transaction
                    for (tx_digest, mut events) in tx_batches {
                        // Sort events by event_seq within the transaction
                        events.sort_by_key(|e| e.id.event_seq);

                        // Convert to BlockchainEvent and create batch
                        let mut batch_events = Vec::new();
                        let mut batch_timestamp = 0u64;
                        let mut checkpoint_seq: Option<u64> = None;

                        for event in events {
                            let timestamp_ms = event.timestamp_ms.unwrap_or_else(|| {
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_millis() as u64
                            });

                            if batch_timestamp == 0 {
                                batch_timestamp = timestamp_ms;
                            }

                            // Try to extract checkpoint_seq from event if available
                            // Note: This may need adjustment based on actual event structure
                            if checkpoint_seq.is_none() {
                                // For now, we'll use a synthetic checkpoint based on timestamp
                                // In a full implementation, this would come from the checkpoint API
                                checkpoint_seq = Some(timestamp_ms / 1000); // Use seconds as checkpoint proxy
                            }

                            let event_id = format!("{}:{}", event.id.tx_digest, event.id.event_seq);
                            let parsed_data = event.parsed_json.clone();
                            let event_seq = event.id.event_seq;

                            let blockchain_event = BlockchainEvent {
                                tx_digest: event.id.tx_digest.to_string(),
                                event_id,
                                event_type: event.type_.to_string(),
                                data: parsed_data,
                                timestamp_ms,
                                checkpoint_seq,
                                event_seq: Some(event_seq),
                            };

                            batch_events.push(blockchain_event);
                        }

                        // Create transaction batch
                        let batch = TransactionBatch {
                            tx_digest: tx_digest.clone(),
                            checkpoint_seq,
                            events: batch_events,
                            timestamp_ms: batch_timestamp,
                        };

                        // Update reader watermark for this transaction
                        if let Some(checkpoint) = checkpoint_seq {
                            if let Err(e) = self
                                .watermark_manager
                                .update_reader_watermark(checkpoint as i64, &tx_digest)
                                .await
                            {
                                error!("Failed to update reader watermark: {}", e);
                            } else {
                                last_processed_checkpoint = checkpoint;
                            }
                        }

                        // Send batch to processor
                        if let Err(e) = self.batch_tx.send(batch).await {
                            error!("Failed to send transaction batch to processor: {}", e);
                            // Channel closed, exit
                            return Err(anyhow!("Processor channel closed"));
                        }
                    }

                    debug!(
                        "Collected and sent {} transaction batches",
                        batch_count
                    );
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    consecutive_errors += 1;

                    error!(
                        "Error querying events ({} consecutive): {}",
                        consecutive_errors, error_msg
                    );

                    if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                        error!(
                            "Reached max consecutive errors ({}). Recreating blockchain client.",
                            MAX_CONSECUTIVE_ERRORS
                        );

                        match MysClientBuilder::default()
                            .build(&self.config.blockchain.rpc_url)
                            .await
                        {
                            Ok(new_client) => {
                                client = new_client;
                                info!("Successfully recreated blockchain client.");
                                consecutive_errors = 0;
                            }
                            Err(e) => {
                                error!("Failed to recreate blockchain client: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}

