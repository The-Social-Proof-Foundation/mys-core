// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use futures::StreamExt;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{interval, Duration};
use tracing::{debug, error, info, warn};

use mys_sdk::{rpc_types::EventFilter, MysClientBuilder};

use crate::config::Config;
use crate::db::Database;

/// Type for events received from the blockchain
#[derive(Debug, Clone)]
pub struct BlockchainEvent {
    /// Transaction digest
    pub tx_digest: String,
    /// Unique event ID (in format <digest>:<event_seq>)
    pub event_id: String,
    /// Event type
    pub event_type: String,
    /// Event data as JSON
    pub data: serde_json::Value,
    /// Timestamp from the blockchain
    pub timestamp_ms: u64,
    /// Checkpoint sequence number (if available)
    pub checkpoint_seq: Option<u64>,
    /// Event sequence within transaction
    pub event_seq: Option<u64>,
}

/// Listener that connects to the blockchain and processes events
pub struct BlockchainEventListener {
    /// Configuration
    config: Config,
    /// Event handler channels
    event_senders: Mutex<Vec<mpsc::Sender<BlockchainEvent>>>,
}

impl BlockchainEventListener {
    /// Create a new blockchain event listener
    pub fn new(config: Config, _db: Arc<Database>) -> Self {
        // Note: db parameter kept for API compatibility but not used
        Self {
            config,
            event_senders: Mutex::new(Vec::new()),
        }
    }

    /// Test basic connectivity to the RPC endpoint
    pub async fn test_connectivity(&self) -> Result<()> {
        info!("🔍 Testing connectivity to MySocial RPC endpoint...");
        info!("RPC URL: {}", self.config.blockchain.rpc_url);
        info!("WebSocket URL: {}", self.config.blockchain.ws_url);

        // Test basic HTTP connectivity first
        info!("Testing basic HTTP connectivity...");
        match reqwest::get(&self.config.blockchain.rpc_url).await {
            Ok(response) => {
                info!(
                    "✅ HTTP connection successful - Status: {}",
                    response.status()
                );

                // Log response headers for debugging
                for (key, value) in response.headers() {
                    debug!("Response header: {}: {:?}", key, value);
                }
            }
            Err(e) => {
                error!("❌ HTTP connection failed: {}", e);
                return Err(anyhow!("HTTP connectivity test failed: {}", e));
            }
        }

        // Test MySocial client connection
        info!("Testing MySocial client connection...");
        match MysClientBuilder::default()
            .build(&self.config.blockchain.rpc_url)
            .await
        {
            Ok(_client) => {
                info!("✅ MySocial client connection successful!");
                Ok(())
            }
            Err(e) => {
                error!("❌ MySocial client connection failed: {}", e);
                Err(anyhow!("MySocial client connectivity test failed: {}", e))
            }
        }
    }

    /// Register a new event handler
    pub async fn register_event_handler(&self, sender: mpsc::Sender<BlockchainEvent>) {
        let mut senders = self.event_senders.lock().await;
        senders.push(sender);
    }

    /// Process a blockchain event and forward it to all registered handlers
    async fn process_event(&self, event: BlockchainEvent) {
        // SUPER IMPORTANT: Log every single event type that comes through the system
        // This will help us identify if events are being received at all
        tracing::info!(
            "🔍 GLOBAL EVENT TRACKER: Received event type: {}",
            event.event_type
        );

        // CRITICAL: Package address debugging - extract package address from event type
        let event_parts: Vec<&str> = event.event_type.split("::").collect();
        if event_parts.len() >= 3 {
            let package_addr = event_parts[0];
            let module_name = event_parts[1];
            let event_name = event_parts[2];

            tracing::info!(
                "🔍 EVENT BREAKDOWN: Package={}, Module={}, Event={}",
                package_addr,
                module_name,
                event_name
            );

            // Log our configured package address vs actual
            let configured_addr = crate::get_mysocial_package_address();
            tracing::info!("🔍 CONFIGURED PACKAGE: {}", configured_addr);
            tracing::info!("🔍 ACTUAL PACKAGE: {}", package_addr);

            // Check for social graph events specifically
            if module_name == "social_graph" || event_name.contains("Follow") {
                tracing::error!("🚨🚨🚨 SOCIAL GRAPH EVENT FOUND: {}", event.event_type);
                tracing::error!("🚨 Package: {}", package_addr);
                tracing::error!("🚨 Module: {}", module_name);
                tracing::error!("🚨 Event: {}", event_name);
                tracing::error!(
                    "🚨 Full Data: {}",
                    serde_json::to_string_pretty(&event.data).unwrap_or_default()
                );

                // Check if package matches our configured address
                if package_addr == configured_addr {
                    tracing::error!("✅ Package address MATCHES configured address");
                } else {
                    tracing::error!("❌ Package address DOES NOT MATCH configured address!");
                    tracing::error!("❌ Expected: {}", configured_addr);
                    tracing::error!("❌ Actual: {}", package_addr);
                }
            }
        }

        // CRITICAL: Enhanced social graph event detection with case-insensitive matching
        let event_type_lower = event.event_type.to_lowercase();
        if event_type_lower.contains("follow") || event_type_lower.contains("unfollow") {
            tracing::error!(
                "🚨🚨🚨 FOLLOW/UNFOLLOW EVENT DETECTED: {}",
                event.event_type
            );
            tracing::error!(
                "🚨 EVENT DATA: {}",
                serde_json::to_string_pretty(&event.data).unwrap_or_default()
            );
            tracing::error!(
                "🚨 FULL EVENT: tx_digest={}, event_id={}, timestamp={}",
                event.tx_digest,
                event.event_id,
                event.timestamp_ms
            );
        }

        // Enhanced social graph event detection
        if event.event_type.contains("FollowEvent")
            || event.event_type.contains("UnfollowEvent")
            || event.event_type.contains("social_graph")
            || event.event_type.contains("SocialGraph")
        {
            tracing::error!("🚨 SOCIAL GRAPH EVENT DETECTED: {}", event.event_type);
            tracing::error!(
                "🚨 FULL EVENT DATA: {}",
                serde_json::to_string_pretty(&event.data).unwrap_or_default()
            );
        }

        // Specifically log any event that might be related to blocking
        if event.event_type.contains("block_list")
            || event.event_type.contains("BlockProfile")
            || event.event_type.contains("BlockList")
            || event.event_type.contains("Unblock")
            || event.event_type.contains("blocker")
            || event.event_type.contains("blocked")
        {
            tracing::info!("🚨 POTENTIAL BLOCK EVENT FOUND: {}", event.event_type);
            tracing::info!(
                "🚨 EVENT DATA: {}",
                serde_json::to_string_pretty(&event.data).unwrap_or_default()
            );

            // Check for module_name field in event data for more reliable detection
            if let Some(obj) = event.data.as_object() {
                if let Some(fields) = obj.get("fields").and_then(|f| f.as_object()) {
                    if let Some(module_name) = fields.get("module_name") {
                        tracing::info!("🔍 Event has module_name field: {}", module_name);
                    }
                }
            }
        }

        // Log ALL events that could possibly be social graph related
        if event.event_type.contains("::profile::")
            || event.event_type.contains("::social_graph::")
            || event.event_type.contains("::social::")
            || event.event_type.contains("Follow")
            || event.event_type.contains("follow")
            || event.event_type.contains("Unfollow")
            || event.event_type.contains("unfollow")
            || event.event_type.contains("::platform::")
            || event.event_type.contains("::Platform")
            || event.event_type.contains("::block_list::")
            || event.event_type.contains("BlockProfileEvent")
        {
            tracing::info!(
                "🔍 POTENTIAL SOCIAL EVENT DETECTED - Event type: {}",
                event.event_type
            );
            tracing::info!(
                "🔍 Event structure analysis - Top level data: {}",
                serde_json::to_string_pretty(&event.data).unwrap_or_default()
            );
        }

        // Log ALL events to help debug what's actually coming through
        tracing::debug!(
            "📋 ALL EVENTS: type='{}', tx_digest='{}', event_id='{}'",
            event.event_type,
            event.tx_digest,
            event.event_id
        );

        let senders = self.event_senders.lock().await;
        for sender in senders.iter() {
            if let Err(e) = sender.send(event.clone()).await {
                error!("Failed to send event to handler: {}", e);
            }
        }
    }

    /// Start the blockchain event listener using websocket
    pub async fn start_ws_listener(&self) -> Result<()> {
        info!("Starting blockchain event listener using WebSocket");
        info!(
            "Attempting to connect to RPC: {}",
            self.config.blockchain.rpc_url
        );
        info!(
            "Attempting to connect to WebSocket: {}",
            self.config.blockchain.ws_url
        );

        // Create MySocial client with WebSocket support
        let client = MysClientBuilder::default()
            .ws_url(&self.config.blockchain.ws_url)
            .build(&self.config.blockchain.rpc_url)
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to build MySocial client - RPC: {}, WS: {}, Error: {}",
                    self.config.blockchain.rpc_url,
                    self.config.blockchain.ws_url,
                    e
                )
            })?;

        info!(
            "✅ Successfully connected to blockchain node: {}",
            self.config.blockchain.ws_url
        );

        // Get the MySocial package address to monitor
        let package_address = crate::get_mysocial_package_address();
        info!("Monitoring events for package: {}", package_address);

        // Create event filter for all events
        // This will capture all events - we'll filter by package and module in our handlers
        let event_filter = EventFilter::All([]);

        // Subscribe to events
        let mut event_stream = client.event_api().subscribe_event(event_filter).await?;
        info!("Successfully subscribed to blockchain events");

        // Process events as they arrive
        while let Some(event_result) = event_stream.next().await {
            match event_result {
                Ok(event) => {
                    debug!("Received event: {:?}", event);

                    // Get timestamp with fallback
                    let timestamp_ms = event.timestamp_ms.unwrap_or_else(|| {
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64
                    });

                    // CRITICAL: Enhanced social graph event detection
                    let event_type_str = event.type_.to_string();
                    if event_type_str.contains("FollowEvent")
                        || event_type_str.contains("UnfollowEvent")
                        || event_type_str.contains("social_graph")
                    {
                        tracing::error!("🚨 SOCIAL GRAPH EVENT DETECTED: {}", event_type_str);
                        tracing::error!(
                            "🚨 FULL EVENT DATA: {}",
                            serde_json::to_string_pretty(&event).unwrap_or_default()
                        );
                    }

                    // Log the raw event for debugging
                    tracing::debug!("Raw blockchain event: {:?}", event);

                    // Get the parsed JSON data
                    let parsed_data = event.parsed_json.clone();

                    // Log the complete raw event structure for detailed debugging
                    tracing::info!(
                        "Complete raw blockchain event JSON: {}",
                        serde_json::to_string_pretty(&event).unwrap_or_default()
                    );
                    tracing::info!(
                        "Parsed JSON data: {}",
                        serde_json::to_string_pretty(&parsed_data).unwrap_or_default()
                    );

                    // Log all events that might be relevant
                    if event.type_.to_string().contains("::profile::")
                        || event.type_.to_string().contains("::social_graph::")
                        || event.type_.to_string().contains("::FollowEvent")
                        || event.type_.to_string().contains("::UnfollowEvent")
                        || event.type_.to_string().contains("::platform::")
                        || event.type_.to_string().contains("::Platform")
                        || event.type_.to_string().contains("::block_list::")
                        || event.type_.to_string().contains("BlockProfileEvent")
                    {
                        tracing::info!("SOCIAL/PLATFORM EVENT DETECTED - Analyzing structure...");

                        // Log the event type
                        tracing::info!("Event type: {}", event.type_);

                        // Try to look into the parsed_json structure
                        if let Some(obj) = parsed_data.as_object() {
                            tracing::info!("Top-level keys: {:?}", obj.keys().collect::<Vec<_>>());

                            // Check if this contains a Move object with fields
                            if let Some(fields) = obj.get("fields") {
                                tracing::info!(
                                    "Move object fields found: {}",
                                    serde_json::to_string_pretty(fields).unwrap_or_default()
                                );

                                // Look specifically for content fields
                                if let Some(content) = obj.get("content") {
                                    tracing::info!(
                                        "Content section found: {}",
                                        serde_json::to_string_pretty(content).unwrap_or_default()
                                    );

                                    // Try to extract fields from content section
                                    if let Some(content_obj) = content.as_object() {
                                        if let Some(content_fields) = content_obj.get("fields") {
                                            tracing::info!(
                                                "Content fields section found: {}",
                                                serde_json::to_string_pretty(content_fields)
                                                    .unwrap_or_default()
                                            );
                                        }
                                    }
                                }

                                // Look for specific fields we need
                                tracing::info!("Looking for specific fields...");
                                for field_name in ["bio", "profile_picture", "cover_photo"] {
                                    if let Some(field_value) = obj.get(field_name) {
                                        tracing::info!(
                                            "Found '{}' at top level: {}",
                                            field_name,
                                            field_value
                                        );
                                    } else if let Some(fields_obj) = fields.as_object() {
                                        if let Some(field_value) = fields_obj.get(field_name) {
                                            tracing::info!(
                                                "Found '{}' in fields section: {}",
                                                field_name,
                                                field_value
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Generate event ID
                    let event_id = format!("{}:{}", event.id.tx_digest, event.id.event_seq);

                    // Convert to blockchain event
                    let blockchain_event = BlockchainEvent {
                        tx_digest: event.id.tx_digest.to_string(),
                        event_id,
                        event_type: event.type_.to_string(),
                        data: parsed_data,
                        timestamp_ms,
                        checkpoint_seq: None, // Not available in websocket listener
                        event_seq: Some(event.id.event_seq),
                    };

                    // Process the event
                    self.process_event(blockchain_event).await;
                }
                Err(e) => {
                    error!("Error receiving event: {}", e);
                }
            }
        }

        warn!("Event stream ended unexpectedly");
        Ok(())
    }

    /// Start the blockchain event listener using polling
    pub async fn start_polling_listener(&self) -> Result<()> {
        info!("Starting blockchain event listener using polling");
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
                    "Failed to build MySocial client for polling - RPC: {}, Error: {}",
                    self.config.blockchain.rpc_url,
                    e
                )
            })?;

        info!(
            "✅ Successfully connected to blockchain node for polling: {}",
            self.config.blockchain.rpc_url
        );

        // Get the MySocial package address to monitor
        let package_address = crate::get_mysocial_package_address();
        info!("Monitoring events for package: {}", package_address);

        // Create event filter for all events
        // This will capture all events - we'll filter by package and module in our handlers
        let event_filter = EventFilter::All([]);

        // Create polling interval
        let mut interval = interval(Duration::from_millis(
            self.config.blockchain.poll_interval_ms,
        ));

        // Track seen event IDs to prevent duplicate processing
        // Using event ID (tx_digest:event_seq) instead of timestamp to handle
        // multiple events from the same transaction correctly
        let mut seen_event_ids: HashSet<String> = HashSet::new();
        
        // Track the last seen event timestamp for initial filtering (to avoid processing very old events)
        // But we still use event IDs for precise deduplication
        let mut last_seen_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as u64;

        // Track consecutive errors to detect stuck state
        let mut consecutive_errors = 0;
        const MAX_CONSECUTIVE_ERRORS: u32 = 5;
        
        // Use a mutable reference to client so we can recreate it
        let mut client = client;

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
                    
                    // Log all event types returned by API query for debugging
                    let event_types: Vec<String> = events.data.iter().map(|e| e.type_.to_string()).collect();
                    debug!("API returned {} events: {:?}", events.data.len(), event_types);
                    
                    // Process events in reverse order (oldest to newest)
                    for event in events.data.into_iter().rev() {
                        // Generate the event ID first (before any filtering)
                        let event_id = format!("{}:{}", event.id.tx_digest, event.id.event_seq);
                        
                        // Skip events we've already seen (using event ID for precise deduplication)
                        if seen_event_ids.contains(&event_id) {
                            debug!("Skipping already processed event: {}", event_id);
                            continue;
                        }

                        // Get the timestamp for initial filtering (skip very old events)
                        // Only skip if timestamp is significantly older (more than 1 second) to allow
                        // multiple events from the same transaction with the same timestamp
                        let event_timestamp = event.timestamp_ms.unwrap_or(0);
                        if event_timestamp < last_seen_timestamp.saturating_sub(1000) {
                            // Only skip events that are more than 1 second older than last seen
                            // This allows events with the same timestamp to be processed
                            // Still mark as seen to prevent reprocessing
                            seen_event_ids.insert(event_id);
                            continue;
                        }

                        debug!("Processing event: {:?}", event);

                        // Get timestamp with fallback
                        let timestamp_ms = event.timestamp_ms.unwrap_or_else(|| {
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis() as u64
                        });

                        // Update the last seen timestamp
                        last_seen_timestamp = timestamp_ms;
                        
                        // Mark this event as seen BEFORE processing to prevent duplicates
                        // if the same event appears in a future batch
                        seen_event_ids.insert(event_id.clone());

                        // CRITICAL: Enhanced social graph event detection
                        let event_type_str = event.type_.to_string();
                        if event_type_str.contains("FollowEvent")
                            || event_type_str.contains("UnfollowEvent")
                            || event_type_str.contains("social_graph")
                        {
                            tracing::error!(
                                "🚨 POLLING: SOCIAL GRAPH EVENT DETECTED: {}",
                                event_type_str
                            );
                            tracing::error!(
                                "🚨 POLLING: FULL EVENT DATA: {}",
                                serde_json::to_string_pretty(&event).unwrap_or_default()
                            );
                        }

                        // Log the raw event for debugging
                        tracing::debug!("Raw blockchain event: {:?}", event);

                        // Get the parsed JSON data
                        let parsed_data = event.parsed_json.clone();

                        // Debug log for block profile events
                        if event.type_.to_string().contains("BlockProfileEvent") {
                            tracing::info!(
                                "!!! CRITICAL DEBUG: FOUND BlockProfileEvent in RAW STREAM: {}",
                                event.type_
                            );
                            tracing::info!(
                                "!!! CRITICAL DEBUG: BlockProfileEvent DATA: {}",
                                serde_json::to_string_pretty(&parsed_data).unwrap_or_default()
                            );
                        }

                        // Convert to blockchain event
                        let blockchain_event = BlockchainEvent {
                            tx_digest: event.id.tx_digest.to_string(),
                            event_id,
                            event_type: event.type_.to_string(),
                            data: parsed_data,
                            timestamp_ms,
                            checkpoint_seq: None, // Not available in polling listener
                            event_seq: Some(event.id.event_seq),
                        };

                        // Process the event
                        self.process_event(blockchain_event).await;
                    }
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    consecutive_errors += 1;
                    
                    error!("Error querying events ({} consecutive): {}", consecutive_errors, error_msg);
                    
                    // If we get a stale transaction events digest error, reset timestamp
                    // to skip past the problematic transaction and continue processing
                    if error_msg.contains("Could not find the referenced transaction events") {
                        warn!("Stale transaction events digest detected, resetting timestamp to skip past it");
                        // Reset to current time minus a small buffer to avoid missing recent events
                        last_seen_timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64 - 60000; // 1 minute ago
                        
                        // If we've hit this error multiple times, recreate the client to reset internal state
                        if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                            warn!("Too many consecutive errors, recreating client to reset state");
                            
                            // Recreate the client
                            match MysClientBuilder::default()
                                .build(&self.config.blockchain.rpc_url)
                                .await
                            {
                                Ok(new_client) => {
                                    info!("Successfully recreated client, continuing polling");
                                    client = new_client;
                                    consecutive_errors = 0; // Reset error counter
                                    // Wait a bit before retrying to give the node time to clear stale state
                                    tokio::time::sleep(Duration::from_secs(5)).await;
                                }
                                Err(e) => {
                                    error!("Failed to recreate client: {}", e);
                                    // Wait longer before retrying
                                    tokio::time::sleep(Duration::from_secs(30)).await;
                                }
                            }
                        } else {
                            // For fewer errors, wait with exponential backoff
                            let backoff_secs = (consecutive_errors as u64).min(10);
                            tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
                        }
                    }
                }
            }
        }
    }

    /// Start the blockchain event listener using the preferred method
    pub async fn start(&self) -> Result<()> {
        // Try WebSocket first, fall back to polling if that fails
        match self.start_ws_listener().await {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!(
                    "WebSocket connection failed, falling back to polling: {}",
                    e
                );
                self.start_polling_listener().await
            }
        }
    }
}

/// Allow cloning BlockchainEvent
