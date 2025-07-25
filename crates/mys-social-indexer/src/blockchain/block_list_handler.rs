// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use anyhow::{anyhow, Result};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
use serde_json;

use crate::db::{Database, DbConnection};
use crate::events::blocking_events::{
    process_profile_block_event,
    process_profile_unblock_event,
};

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
        Self {
            db,
            rx,
        }
    }
    
    /// Get a database connection from the pool
    async fn get_connection(&self) -> Result<DbConnection> {
        self.db.get_connection()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }

    /// Process raw blockchain events
    async fn process_event(&self, event: BlockchainEvent) -> Result<()> {
        debug!("BlockList handler examining event: {}", event.event_type);
        
        // Only process events from the block_list module, but exclude BlockListCreatedEvent 
        // since that's handled by profile_handler to avoid duplicate processing
        if !event.event_type.contains("::block_list::") {
            // Not from block_list module, skip it
            return Ok(());
        }
        
        // Skip BlockListCreatedEvent as it's handled by profile_handler
        if event.event_type.contains("BlockListCreatedEvent") {
            debug!("Skipping BlockListCreatedEvent - handled by profile_handler");
            return Ok(());
        }
        
        // Log the raw event data for debugging
        info!("BlockList handler received event: {}", event.event_type);
        info!("Event data: {}", serde_json::to_string_pretty(&event.data).unwrap_or_default());
        
        // Get a database connection
        let mut conn = self.get_connection().await?;
        
        // Process based on specific event type - use more flexible matching
        let event_type_lower = event.event_type.to_lowercase();
        
        if event_type_lower.contains("blockprofileevent") || 
           event_type_lower.contains("userblockevent") ||
           event_type_lower.contains("profileblockevent") {
            info!("Processing profile block event");
            if let Err(e) = process_profile_block_event(&mut conn, &event.data).await {
                error!("Failed to process profile block event: {}", e);
                return Err(e);
            }
        } else if event_type_lower.contains("unblockprofileevent") || 
                  event_type_lower.contains("userunblockevent") ||
                  event_type_lower.contains("profileunblockevent") {
            info!("Processing profile unblock event");
            if let Err(e) = process_profile_unblock_event(&mut conn, &event.data).await {
                error!("Failed to process profile unblock event: {}", e);
                return Err(e);
            }
        } else if event_type_lower.contains("platformblockedprofileevent") ||
                  event_type_lower.contains("platformblockevent") {
            info!("Processing platform block event");
            if let Err(e) = crate::events::blocking_events::process_platform_block_event(&mut conn, &event.data).await {
                error!("Failed to process platform block event: {}", e);
                return Err(e);
            }
        } else if event_type_lower.contains("platformunblockedprofileevent") ||
                  event_type_lower.contains("platformunblockevent") {
            info!("Processing platform unblock event");
            if let Err(e) = crate::events::blocking_events::process_platform_unblock_event(&mut conn, &event.data).await {
                error!("Failed to process platform unblock event: {}", e);
                return Err(e);
            }
        } else {
            // Unknown block_list event type - log more details for debugging
            warn!("Unknown block_list event type: {}", event.event_type);
            warn!("Event data: {}", serde_json::to_string_pretty(&event.data).unwrap_or_default());
            
            // Still try to process as a generic blocking event if it has the right fields
            if event.data.as_object().map_or(false, |obj| {
                (obj.contains_key("blocker") && obj.contains_key("blocked")) ||
                (obj.contains_key("platform_id") && obj.contains_key("profile_id"))
            }) {
                info!("Attempting to process unknown event as generic blocking event");
                // Try as profile block first
                if event.data.get("blocker").is_some() && event.data.get("blocked").is_some() {
                    if let Err(e) = process_profile_block_event(&mut conn, &event.data).await {
                        warn!("Failed to process as profile block event: {}", e);
                    }
                }
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