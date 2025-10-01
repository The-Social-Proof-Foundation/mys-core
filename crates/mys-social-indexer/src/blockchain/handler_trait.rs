// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::blockchain::listener::BlockchainEvent;
use crate::db::Database;

/// Health status of a handler
#[derive(Debug, Clone, PartialEq)]
pub enum HandlerHealth {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

/// Statistics for a handler
#[derive(Debug, Clone, Default)]
pub struct HandlerStats {
    pub events_processed: u64,
    pub events_failed: u64,
    pub last_processed_timestamp: Option<u64>,
    pub processing_errors: Vec<String>,
}

/// Standard trait that all blockchain event handlers must implement
#[async_trait]
pub trait BlockchainEventHandler: Send + Sync {
    /// Get the handler name (for logging and metrics)
    fn name(&self) -> &str;

    /// Process a single blockchain event
    async fn process_event(&mut self, event: BlockchainEvent) -> Result<()>;

    /// Get current handler health status
    async fn health(&self) -> HandlerHealth {
        HandlerHealth::Healthy
    }

    /// Get handler statistics
    fn stats(&self) -> HandlerStats {
        HandlerStats::default()
    }

    /// Start the handler's main processing loop
    async fn start(&mut self, mut receiver: mpsc::Receiver<BlockchainEvent>) -> Result<()> {
        info!("Starting blockchain event handler: {}", self.name());

        let mut stats = HandlerStats::default();

        while let Some(event) = receiver.recv().await {
            // Update last processed timestamp
            stats.last_processed_timestamp = Some(event.timestamp_ms);

            // Process the event
            match self.process_event(event.clone()).await {
                Ok(_) => {
                    stats.events_processed += 1;
                    // Clear errors on success
                    if !stats.processing_errors.is_empty() {
                        stats.processing_errors.clear();
                    }
                }
                Err(e) => {
                    stats.events_failed += 1;
                    let error_msg = format!(
                        "Failed to process event {} (type: {}): {}",
                        event.event_id, event.event_type, e
                    );
                    error!("{}", error_msg);

                    // Keep only last 10 errors to prevent memory bloat
                    stats.processing_errors.push(error_msg);
                    if stats.processing_errors.len() > 10 {
                        stats.processing_errors.remove(0);
                    }
                }
            }

            // Log progress every 100 events
            if stats.events_processed % 100 == 0 && stats.events_processed > 0 {
                info!(
                    "Handler {} processed {} events ({} failed)",
                    self.name(),
                    stats.events_processed,
                    stats.events_failed
                );
            }
        }

        warn!("Handler {} stopped - channel closed", self.name());
        Ok(())
    }

    /// Shutdown the handler gracefully
    async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down handler: {}", self.name());
        Ok(())
    }
}

/// Base handler struct that provides common functionality
pub struct BaseHandler {
    pub name: String,
    pub db: Arc<Database>,
    pub stats: HandlerStats,
}

impl BaseHandler {
    pub fn new(name: String, db: Arc<Database>) -> Self {
        Self {
            name,
            db,
            stats: HandlerStats::default(),
        }
    }

    /// Get a database connection with proper error handling
    pub async fn get_connection(&self) -> Result<crate::db::DbConnection> {
        self.db.get_connection().await.map_err(|e| {
            anyhow::anyhow!(
                "Failed to get database connection for handler {}: {}",
                self.name,
                e
            )
        })
    }

    /// Update handler statistics
    pub fn update_stats_success(&mut self, timestamp: u64) {
        self.stats.events_processed += 1;
        self.stats.last_processed_timestamp = Some(timestamp);

        // Clear errors on successful processing
        if !self.stats.processing_errors.is_empty() {
            self.stats.processing_errors.clear();
        }
    }

    /// Update handler statistics for failure
    pub fn update_stats_failure(&mut self, error: String) {
        self.stats.events_failed += 1;
        self.stats.processing_errors.push(error);

        // Keep only last 10 errors
        if self.stats.processing_errors.len() > 10 {
            self.stats.processing_errors.remove(0);
        }
    }

    /// Get handler health based on error rate
    pub fn get_health(&self) -> HandlerHealth {
        let total_events = self.stats.events_processed + self.stats.events_failed;

        if total_events == 0 {
            return HandlerHealth::Healthy;
        }

        let error_rate = self.stats.events_failed as f64 / total_events as f64;

        if error_rate > 0.5 {
            HandlerHealth::Unhealthy(format!(
                "High error rate: {:.1}% ({}/{})",
                error_rate * 100.0,
                self.stats.events_failed,
                total_events
            ))
        } else if error_rate > 0.1 {
            HandlerHealth::Degraded(format!(
                "Elevated error rate: {:.1}% ({}/{})",
                error_rate * 100.0,
                self.stats.events_failed,
                total_events
            ))
        } else {
            HandlerHealth::Healthy
        }
    }
}

/// Helper trait for handlers that need to update progress tracking
#[async_trait]
pub trait ProgressTracker {
    /// Update the handler's progress in the database
    async fn update_progress(&self) -> Result<()>;
}

/// Macro to implement common handler boilerplate
#[macro_export]
macro_rules! impl_handler_boilerplate {
    ($handler_type:ty, $handler_name:expr) => {
        #[async_trait]
        impl crate::blockchain::handler_trait::BlockchainEventHandler for $handler_type {
            fn name(&self) -> &str {
                $handler_name
            }

            fn stats(&self) -> crate::blockchain::handler_trait::HandlerStats {
                self.base.stats.clone()
            }

            async fn health(&self) -> crate::blockchain::handler_trait::HandlerHealth {
                self.base.get_health()
            }
        }
    };
}

/// Helper function to spawn a handler task
pub fn spawn_handler_task<H>(
    mut handler: H,
    receiver: mpsc::Receiver<BlockchainEvent>,
) -> tokio::task::JoinHandle<Result<()>>
where
    H: BlockchainEventHandler + 'static,
{
    tokio::spawn(async move { handler.start(receiver).await })
}
