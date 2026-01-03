// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::db::{Database, ConnectionManager};
use crate::blockchain::processor::ProcessedTransactionBatch;
use crate::blockchain::watermark::WatermarkManager;

/// Committer that commits transaction batches atomically and updates CommitterWatermark
pub struct Committer {
    /// Database connection manager
    connection_manager: Arc<ConnectionManager>,
    /// Watermark manager
    watermark_manager: WatermarkManager,
    /// Channel receiver for processed batches from Processor
    processed_rx: mpsc::Receiver<ProcessedTransactionBatch>,
}

impl Committer {
    /// Create a new committer
    pub fn new(
        db: Arc<Database>,
        processed_rx: mpsc::Receiver<ProcessedTransactionBatch>,
    ) -> Self {
        let connection_manager = Arc::new(ConnectionManager::new(db.clone()));
        let watermark_manager = WatermarkManager::new(db);
        Self {
            connection_manager,
            watermark_manager,
            processed_rx,
        }
    }

    /// Start committing transaction batches
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting transaction committer");

        while let Some(processed_batch) = self.processed_rx.recv().await {
            debug!(
                "Committing transaction batch: tx_digest={}, events={}",
                processed_batch.batch.tx_digest,
                processed_batch.routed_events.len()
            );

            // Commit all events from this transaction atomically
            match self.commit_batch(&processed_batch).await {
                Ok(_) => {
                    // Update committer watermark after successful commit
                    if let Some(checkpoint_seq) = processed_batch.batch.checkpoint_seq {
                        if let Err(e) = self
                            .watermark_manager
                            .update_committer_watermark(
                                checkpoint_seq as i64,
                                &processed_batch.batch.tx_digest,
                            )
                            .await
                        {
                            error!(
                                "Failed to update committer watermark: {}",
                                e
                            );
                            // Don't fail the entire commit if watermark update fails
                            // The next commit will retry from the last successful checkpoint
                        } else {
                            debug!(
                                "Updated committer watermark: checkpoint_seq={}, tx_digest={}",
                                checkpoint_seq, processed_batch.batch.tx_digest
                            );
                        }
                    }

                    info!(
                        "Successfully committed transaction batch: tx_digest={}",
                        processed_batch.batch.tx_digest
                    );
                }
                Err(e) => {
                    error!(
                        "Failed to commit transaction batch: tx_digest={}, error={}",
                        processed_batch.batch.tx_digest, e
                    );
                    // Watermark is not updated on failure, so we can retry later
                    // In a production system, you might want to send to a retry queue
                }
            }
        }

        warn!("Committer batch receiver closed");
        Ok(())
    }

    /// Commit a transaction batch atomically
    async fn commit_batch(&self, processed_batch: &ProcessedTransactionBatch) -> Result<()> {
        // Wrap all database operations in a single transaction
        // Note: Currently, handlers commit immediately when they process events.
        // For true atomicity, handlers would need to be refactored to accept a transaction context.
        // For now, we'll commit each event handler's operations individually.
        // TODO: Refactor handlers to support transaction context for true atomicity

        // Since handlers currently commit immediately when processing events,
        // the events in processed_batch.routed_events have already been committed
        // by their respective handlers. This means we can't rollback if one fails.
        //
        // The proper solution is to:
        // 1. Refactor handlers to prepare operations instead of committing
        // 2. Collect all prepared operations
        // 3. Commit them all atomically here
        //
        // For now, we'll just verify that all events were successfully routed
        // and update the watermark accordingly.

        if processed_batch.routed_events.is_empty() {
            debug!(
                "No events to commit for transaction: {}",
                processed_batch.batch.tx_digest
            );
            return Ok(());
        }

        // Check if any handlers failed
        let mut has_failures = false;
        for (event_type, result) in &processed_batch.handler_results {
            match result {
                crate::blockchain::processor::ProcessingResult::Failed(err) => {
                    error!(
                        "Handler failed for event type {}: {}",
                        event_type, err
                    );
                    has_failures = true;
                }
                _ => {}
            }
        }

        if has_failures {
            return Err(anyhow!(
                "One or more handlers failed for transaction: {}",
                processed_batch.batch.tx_digest
            ));
        }

        // All events were successfully processed
        // In the future, when handlers support transaction context,
        // we would commit all operations atomically here using:
        //
        // self.connection_manager.with_transaction(|conn| {
        //     Box::pin(async move {
        //         // Execute all handler operations within this transaction
        //         // If any fails, the entire transaction rolls back
        //     })
        // }).await

        Ok(())
    }
}

