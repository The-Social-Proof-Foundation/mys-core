// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use serde_json::Value;
use tracing::{debug, info};
use tokio::sync::mpsc;
use std::sync::Arc;

use crate::db::DbConnection;
use crate::db::Database;
use crate::blockchain::listener::BlockchainEvent;
use crate::PROFILE_MODULE_NAME;

/// ProfileEventListener handles all profile-related events from the blockchain
pub struct ProfileEventListener {
    db: Arc<Database>,
    receiver: mpsc::Receiver<BlockchainEvent>,
    worker_name: String,
}

impl ProfileEventListener {
    /// Create a new ProfileEventListener instance
    pub fn new(
        db: Arc<Database>,
        receiver: mpsc::Receiver<BlockchainEvent>,
        worker_name: String,
    ) -> Self {
        Self {
            db,
            receiver,
            worker_name,
        }
    }

    /// Start the profile event listener
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting profile event listener: {}", self.worker_name);
        
        while let Some(event) = self.receiver.recv().await {
            // Extract the module name from the event type
            // Example: 0x123::profile::ProfileCreatedEvent
            let parts: Vec<&str> = event.event_type.split("::").collect();
            if parts.len() < 2 {
                continue; // Skip malformed event types
            }
            
            let module_name = parts[1]; // Second part is the module name
            
            // Get the function/event name, which is the last part
            let function_name = parts.last().unwrap_or(&"")
                .replace("Event", ""); // Remove "Event" suffix if present
            
            let mut conn = self.db.get_connection().await?;
            self.process_event(&mut conn, module_name, &function_name, &event.data, &event.event_id).await?;
        }
        
        Ok(())
    }

    /// Process a profile event from the blockchain
    pub async fn process_event(
        &self,
        _conn: &mut DbConnection,
        module_name: &str,
        function_name: &str,
        _event_data: &Value,
        _event_id: &str,
    ) -> Result<()> {
        // Skip if this is not a profile module event
        if module_name != PROFILE_MODULE_NAME {
            return Ok(());
        }

        debug!("Processing profile event: {}", function_name);

        // Handle profile events by function name
        match function_name {
            "create_profile" | "ProfileCreated" => {
                // Handle profile creation
                debug!("Processing profile creation event");
                // Implementation would go here
            }
            "update_profile" | "ProfileUpdated" => {
                // Handle profile update
                debug!("Processing profile update event");
                // Implementation would go here
            }
            "register_username" | "UsernameRegistered" => {
                // Handle username registration
                debug!("Processing username registration event");
                // Implementation would go here
            }
            "update_username" | "UsernameUpdated" => {
                // Handle username update
                debug!("Processing username update event");
                // Implementation would go here
            }
            _ => {
                debug!("Unknown profile function: {}", function_name);
            }
        }

        info!("Processed profile event: {}", function_name);
        Ok(())
    }
} 