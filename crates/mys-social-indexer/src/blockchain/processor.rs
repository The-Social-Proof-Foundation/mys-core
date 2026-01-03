// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::blockchain::collector::TransactionBatch;
use crate::blockchain::event_router::EventRouter;
use crate::blockchain::listener::BlockchainEvent;

/// Processed transaction batch ready for committing
#[derive(Debug, Clone)]
pub struct ProcessedTransactionBatch {
    /// Original transaction batch
    pub batch: TransactionBatch,
    /// Events that were routed to handlers
    pub routed_events: Vec<BlockchainEvent>,
    /// Handler processing results (for future use)
    pub handler_results: HashMap<String, ProcessingResult>,
}

/// Processing result for a handler
#[derive(Debug, Clone)]
pub enum ProcessingResult {
    Success,
    Failed(String),
    Skipped,
}

/// Processor that processes transaction batches and routes events to handlers
pub struct Processor {
    /// Event router for routing events to handlers
    event_router: Arc<tokio::sync::Mutex<EventRouter>>,
    /// Channel receiver for transaction batches from Collector
    batch_rx: mpsc::Receiver<TransactionBatch>,
    /// Channel sender for processed batches to Committer
    processed_tx: mpsc::Sender<ProcessedTransactionBatch>,
}

impl Processor {
    /// Create a new processor
    pub fn new(
        event_router: Arc<tokio::sync::Mutex<EventRouter>>,
        batch_rx: mpsc::Receiver<TransactionBatch>,
        processed_tx: mpsc::Sender<ProcessedTransactionBatch>,
    ) -> Self {
        Self {
            event_router,
            batch_rx,
            processed_tx,
        }
    }

    /// Start processing transaction batches
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting event processor");

        while let Some(batch) = self.batch_rx.recv().await {
            debug!(
                "Processing transaction batch: tx_digest={}, events={}",
                batch.tx_digest,
                batch.events.len()
            );

            // Process events in transaction order
            let mut routed_events = Vec::new();
            let mut handler_results = HashMap::new();

            for event in &batch.events {
                // Route event to handlers
                let mut router_guard = self.event_router.lock().await;
                match router_guard.route_event(event.clone()).await {
                    Ok(_) => {
                        routed_events.push(event.clone());
                        debug!(
                            "Successfully routed event: type={}, tx_digest={}",
                            event.event_type, event.tx_digest
                        );
                    }
                    Err(e) => {
                        error!(
                            "Failed to route event: type={}, tx_digest={}, error={}",
                            event.event_type, event.tx_digest, e
                        );
                        handler_results.insert(
                            event.event_type.clone(),
                            ProcessingResult::Failed(e.to_string()),
                        );
                    }
                }
            }

            // Create processed batch
            let processed_batch = ProcessedTransactionBatch {
                batch: batch.clone(),
                routed_events,
                handler_results,
            };

            // Send to committer
            if let Err(e) = self.processed_tx.send(processed_batch).await {
                error!("Failed to send processed batch to committer: {}", e);
                return Err(anyhow::anyhow!("Committer channel closed"));
            }

            debug!(
                "Processed transaction batch: tx_digest={}, routed_events={}",
                batch.tx_digest,
                processed_batch.routed_events.len()
            );
        }

        warn!("Processor batch receiver closed");
        Ok(())
    }
}

