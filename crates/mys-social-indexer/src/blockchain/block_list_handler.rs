// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::db::{Database, DbConnection};
use crate::events::blocking_events::{process_profile_block_event, process_profile_unblock_event};

use super::listener::BlockchainEvent;

/// Handler for block list related blockchain events
pub struct BlockListEventHandler {
    /// Database connection
    db: Arc<Database>,
    /// Event receiver channel
    rx: mpsc::Receiver<BlockchainEvent>,
}

impl BlockListEventHandler {
    /// Create a new block list event handler
    pub fn new(db: Arc<Database>, rx: mpsc::Receiver<BlockchainEvent>, _worker_id: String) -> Self {
        Self { db, rx }
    }

    /// Get a database connection from the pool
    async fn get_connection(&self) -> Result<DbConnection> {
        self.db
            .get_connection()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }

    /// Process raw blockchain events
    async fn process_event(&self, event: BlockchainEvent) -> Result<()> {
        debug!("BlockList handler examining event: {}", event.event_type);

        // Only process events from block_list module (user-to-user blocking)
        // Platform blocking events are handled by platform_handler.rs
        if !event.event_type.contains("::block_list::") {
            return Ok(());
        }

        info!("Processing block_list event: {}", event.event_type);

        let mut event_handled = false;
        let mut conn = self.get_connection().await?;

        // Handle UserBlockEvent
        if event.event_type.ends_with("::UserBlockEvent") {
            info!("Processing UserBlockEvent");
            if let Err(e) = process_profile_block_event(&mut conn, &event.data).await {
                error!("Failed to process UserBlockEvent: {}", e);
                return Err(e);
            }
            return Ok(());
        }
        // Handle UserUnblockEvent
        else if event.event_type.ends_with("::UserUnblockEvent") {
            info!("Processing UserUnblockEvent");
            if let Err(e) = process_profile_unblock_event(&mut conn, &event.data).await {
                error!("Failed to process UserUnblockEvent: {}", e);
                return Err(e);
            }
            return Ok(());
        }
        // Fallback: try flexible matching for backwards compatibility
        else {
            let event_type_lower = event.event_type.to_lowercase();

            if event_type_lower.contains("userblockevent") {
                event_handled = true;
                info!("Processing UserBlockEvent (flexible match)");
                if let Err(e) = process_profile_block_event(&mut conn, &event.data).await {
                    error!("Failed to process UserBlockEvent: {}", e);
                    return Err(e);
                }
            } else if event_type_lower.contains("userunblockevent") {
                event_handled = true;
                info!("Processing UserUnblockEvent (flexible match)");
                if let Err(e) = process_profile_unblock_event(&mut conn, &event.data).await {
                    error!("Failed to process UserUnblockEvent: {}", e);
                    return Err(e);
                }
            } else {
                // Unknown block_list event type - log more details for debugging
                warn!("Unknown block_list event type: {} (event_id: {})", event.event_type, event.event_id);
                warn!(
                    "Event data: {}",
                    serde_json::to_string_pretty(&event.data).unwrap_or_default()
                );

                // Still try to process as a generic blocking event if it has the right fields
                let fields = crate::events::event_utils::extract_event_fields(&event.data)
                    .unwrap_or_else(|_| event.data.clone());
                if fields.as_object().map_or(false, |obj| {
                    (obj.contains_key("blocker") && obj.contains_key("blocked"))
                        || (obj.contains_key("blocker") && obj.contains_key("unblocked"))
                }) {
                    info!("Attempting to process unknown event as generic blocking event");

                    // Try as profile block event
                    if fields.get("blocker").is_some() && fields.get("blocked").is_some() {
                        event_handled = true;
                        info!("Processing unknown event as UserBlockEvent");
                        if let Err(e) = process_profile_block_event(&mut conn, &fields).await {
                            warn!("Failed to process as UserBlockEvent: {}", e);
                        }
                    }
                    // Try as profile unblock event
                    else if fields.get("blocker").is_some() && fields.get("unblocked").is_some() {
                        event_handled = true;
                        info!("Processing unknown event as UserUnblockEvent");
                        if let Err(e) = process_profile_unblock_event(&mut conn, &fields).await {
                            warn!("Failed to process as UserUnblockEvent: {}", e);
                        }
                    }
                }
            }

            // Warn if we received a block_list event but didn't handle it
            if !event_handled {
                warn!(
                    "Received unhandled block_list event: {} (event_id: {})",
                    event.event_type, event.event_id
                );
            }
        }

        Ok(())
    }

    /// Start listening for block list events
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting block list event handler");

        while let Some(event) = self.rx.recv().await {
            debug!("Received event: {:?}", event.event_type);

            if let Err(e) = self.process_event(event).await {
                error!("Error processing event: {}", e);
            }
        }

        warn!("Block list event handler channel closed");
        Ok(())
    }
}
