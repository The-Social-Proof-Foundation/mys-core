// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn, trace};

use crate::db::{Database, DbConnection};
use crate::events::{FollowEvent, UnfollowEvent, parse_event};
use crate::schema;
use mys_types::event::Event as MysEvent;

use super::listener::BlockchainEvent;

/// Handlers for social graph related events
pub struct SocialGraphEventHandler {
    /// Database connection
    db: Arc<Database>,
    /// Event receiver channel
    rx: mpsc::Receiver<BlockchainEvent>,
}

impl SocialGraphEventHandler {
    /// Create a new social graph event handler
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
    
    /// Process a follow event - creates relationship and updates follow counts
    async fn process_follow_event(&self, event: &FollowEvent, blockchain_event: Option<&BlockchainEvent>) -> Result<()> {
        info!("Processing social graph FollowEvent");
        let mut conn = self.get_connection().await?;
        
        // Create a social graph event record for history/auditing
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        
        let event_id = blockchain_event.map(|e| e.event_id.clone());
        
        let social_graph_event = crate::models::social_graph::NewSocialGraphEvent {
            event_type: "follow".to_string(),
            follower_address: event.follower.clone(),
            following_address: event.following.clone(),
            created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now())
                .naive_utc(),
            event_id,
            raw_event_data: serde_json::to_value(event).ok(),
        };
        
        // Always insert the event record
        diesel::insert_into(schema::social_graph_events::table)
            .values(&social_graph_event)
            .execute(&mut conn)
            .await?;
            
        // Get profile IDs from addresses (using owner_address field)
        let follower_profile = match schema::profiles::table
            .filter(schema::profiles::owner_address.eq(&event.follower))
            .select((schema::profiles::id, schema::profiles::owner_address))
            .first::<(i32, String)>(&mut conn)
            .await {
            Ok(profile) => profile,
            Err(e) => {
                error!("Failed to find follower profile for address {}: {}", event.follower, e);
                return Ok(()); // Still return Ok since we recorded the event
            }
        };
            
        let following_profile = match schema::profiles::table
            .filter(schema::profiles::owner_address.eq(&event.following))
            .select((schema::profiles::id, schema::profiles::owner_address))
            .first::<(i32, String)>(&mut conn)
            .await {
            Ok(profile) => profile,
            Err(e) => {
                error!("Failed to find following profile for address {}: {}", event.following, e);
                return Ok(()); // Still return Ok since we recorded the event
            }
        };
        
        // Create relationship using addresses (matching the schema)
        let relationship = match event.into_relationship() {
            Ok(rel) => rel,
            Err(e) => {
                error!("Failed to create relationship: {}", e);
                return Ok(());
            }
        };
        
        // Check if relationship already exists using addresses (matching the schema)
        let existing = match schema::social_graph_relationships::table
            .filter(schema::social_graph_relationships::follower_address.eq(&event.follower))
            .filter(schema::social_graph_relationships::following_address.eq(&event.following))
            .count()
            .get_result::<i64>(&mut conn)
            .await {
            Ok(count) => count > 0,
            Err(e) => {
                error!("Failed to check existing relationship: {}", e);
                return Ok(());
            }
        };
            
        if existing {
            info!("Follow relationship already exists between {} and {}", 
                event.follower, event.following);
            return Ok(());
        }
            
        // Start a transaction for atomicity
        let result = conn.build_transaction()
            .run(|mut conn| Box::pin(async move {
                // Insert relationship
                diesel::insert_into(schema::social_graph_relationships::table)
                    .values(&relationship)
                    .execute(&mut conn)
                    .await?;
                    
                // Update follower's following count (increment)
                diesel::sql_query(format!(
                    "UPDATE profiles SET following_count = following_count + 1 WHERE id = {}", 
                    follower_profile.0
                ))
                .execute(&mut conn)
                .await?;
                
                // Update followed's followers count (increment)
                diesel::sql_query(format!(
                    "UPDATE profiles SET followers_count = followers_count + 1 WHERE id = {}", 
                    following_profile.0
                ))
                .execute(&mut conn)
                .await?;
                
                Result::<_, diesel::result::Error>::Ok(())
            }))
            .await;
            
        if let Err(e) = result {
            error!("Failed to process follow event transaction: {}", e);
            return Err(anyhow::anyhow!("Transaction failed: {}", e));
        } else {
            info!("Processed follow event: {} is now following {}", 
                event.follower, event.following);
        }
            
        Ok(())
    }
    
    /// Process an unfollow event - removes relationship and updates follow counts
    async fn process_unfollow_event(&self, event: &UnfollowEvent, blockchain_event: Option<&BlockchainEvent>) -> Result<()> {
        info!("Processing social graph UnfollowEvent");
        let mut conn = self.get_connection().await?;
        
        // Create a social graph event record for history/auditing
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        
        let event_id = blockchain_event.map(|e| e.event_id.clone());
        
        let social_graph_event = crate::models::social_graph::NewSocialGraphEvent {
            event_type: "unfollow".to_string(),
            follower_address: event.follower.clone(),
            following_address: event.unfollowed.clone(),
            created_at: chrono::DateTime::from_timestamp(now.as_secs() as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now())
                .naive_utc(),
            event_id,
            raw_event_data: serde_json::to_value(event).ok(),
        };
        
        // Always insert the event record
        diesel::insert_into(schema::social_graph_events::table)
            .values(&social_graph_event)
            .execute(&mut conn)
            .await?;
            
        // Get profile IDs from addresses (using owner_address field)
        let follower_profile = match schema::profiles::table
            .filter(schema::profiles::owner_address.eq(&event.follower))
            .select((schema::profiles::id, schema::profiles::owner_address))
            .first::<(i32, String)>(&mut conn)
            .await {
            Ok(profile) => profile,
            Err(e) => {
                error!("Failed to find follower profile for address {}: {}", event.follower, e);
                return Ok(()); // Still return Ok since we recorded the event
            }
        };
            
        let unfollowed_profile = match schema::profiles::table
            .filter(schema::profiles::owner_address.eq(&event.unfollowed))
            .select((schema::profiles::id, schema::profiles::owner_address))
            .first::<(i32, String)>(&mut conn)
            .await {
            Ok(profile) => profile,
            Err(e) => {
                error!("Failed to find unfollowed profile for address {}: {}", event.unfollowed, e);
                return Ok(()); // Still return Ok since we recorded the event
            }
        };
        
        // Check if relationship exists (using addresses since that's what the schema uses)
        let relationship_exists = match schema::social_graph_relationships::table
            .filter(schema::social_graph_relationships::follower_address.eq(&event.follower))
            .filter(schema::social_graph_relationships::following_address.eq(&event.unfollowed))
            .count()
            .get_result::<i64>(&mut conn)
            .await {
            Ok(count) => count > 0,
            Err(e) => {
                error!("Failed to check existing relationship: {}", e);
                return Ok(());
            }
        };
        
        if !relationship_exists {
            info!("Follow relationship does not exist between {} and {}", 
                event.follower, event.unfollowed);
            return Ok(());
        }
            
        // Start a transaction for atomicity
        let result = conn.build_transaction()
            .run(|mut conn| Box::pin(async move {
                // Delete the relationship using addresses
                diesel::delete(schema::social_graph_relationships::table
                    .filter(schema::social_graph_relationships::follower_address.eq(&event.follower))
                    .filter(schema::social_graph_relationships::following_address.eq(&event.unfollowed)))
                    .execute(&mut conn)
                    .await?;
                    
                // Update follower's following count (decrement with safety)
                diesel::sql_query(format!(
                    "UPDATE profiles SET following_count = GREATEST(0, following_count - 1) WHERE id = {}", 
                    follower_profile.0
                ))
                .execute(&mut conn)
                .await?;
                
                // Update unfollowed's followers count (decrement with safety)
                diesel::sql_query(format!(
                    "UPDATE profiles SET followers_count = GREATEST(0, followers_count - 1) WHERE id = {}", 
                    unfollowed_profile.0
                ))
                .execute(&mut conn)
                .await?;
                
                Result::<_, diesel::result::Error>::Ok(())
            }))
            .await;
            
        if let Err(e) = result {
            error!("Failed to process unfollow event transaction: {}", e);
            return Err(anyhow::anyhow!("Transaction failed: {}", e));
        } else {
            info!("Processed unfollow event: {} unfollowed {}", 
                event.follower, event.unfollowed);
        }
            
        Ok(())
    }
    
    /// Process raw blockchain events
    async fn process_event(&self, event: BlockchainEvent) -> Result<()> {
        info!("🚨 SOCIAL GRAPH: Examining event: {}", event.event_type);
        info!("🚨 SOCIAL GRAPH: Event ID: {}", event.event_id);
        
        // TEMPORARY DEBUG: Log ALL events to see what we're receiving
        info!("🔍 SOCIAL GRAPH DEBUG: ALL EVENTS - Type: {}", event.event_type);
        if event.event_type.to_lowercase().contains("follow") || 
           event.event_type.to_lowercase().contains("social") ||
           event.event_type.to_lowercase().contains("graph") {
            info!("🔍 SOCIAL GRAPH DEBUG: POTENTIAL MATCH - Type: {}", event.event_type);
            info!("🔍 SOCIAL GRAPH DEBUG: POTENTIAL MATCH - Data: {}", serde_json::to_string_pretty(&event.data).unwrap_or_default());
        }
        
        // TEMPORARY: More permissive filtering - catch social graph events from ANY package
        let event_parts: Vec<&str> = event.event_type.split("::").collect();
        let is_social_graph_event = if event_parts.len() >= 3 {
            let module_name = event_parts[1];
            let event_name = event_parts[2];
            
            // Accept if module is social_graph OR event name contains Follow
            module_name == "social_graph" || 
            event_name.contains("Follow") ||
            event_name == "FollowEvent" ||
            event_name == "UnfollowEvent"
        } else {
            // Fallback to original filtering
            event.event_type.contains("::social_graph::") || 
            event.event_type.contains("::FollowEvent") || 
            event.event_type.contains("::UnfollowEvent") ||
            event.event_type.ends_with("FollowEvent") ||
            event.event_type.ends_with("UnfollowEvent") ||
            event.event_type.to_lowercase().contains("follow")
        };
        
        if is_social_graph_event {
            info!("🚨 SOCIAL GRAPH: Processing social graph event: {}", event.event_type);
            info!("🚨 SOCIAL GRAPH: Full event data: {}", serde_json::to_string_pretty(&event.data).unwrap_or_default());
            
            if event.event_type.ends_with("::FollowEvent") || event.event_type.ends_with("FollowEvent") {
                info!("🚨 SOCIAL GRAPH: Attempting to parse FollowEvent");
                match crate::events::event_utils::parse_json_event_with_fields::<FollowEvent>(&event.data) {
                    Ok(follow_event) => {
                        info!("🚨 SOCIAL GRAPH: Successfully parsed FollowEvent: {} -> {}", &follow_event.follower, &follow_event.following);
                        if let Err(e) = self.process_follow_event(&follow_event, Some(&event)).await {
                            error!("Failed to process follow event: {}", e);
                        }
                    },
                    Err(e) => {
                        error!("🚨 SOCIAL GRAPH: Failed to parse follow event: {}", e);
                        error!("Event data that failed to parse: {}", serde_json::to_string_pretty(&event.data).unwrap_or_default());
                    }
                }
            } else if event.event_type.ends_with("::UnfollowEvent") || event.event_type.ends_with("UnfollowEvent") {
                info!("🚨 SOCIAL GRAPH: Attempting to parse UnfollowEvent");
                match crate::events::event_utils::parse_json_event_with_fields::<UnfollowEvent>(&event.data) {
                    Ok(unfollow_event) => {
                        info!("🚨 SOCIAL GRAPH: Successfully parsed UnfollowEvent: {} -> {}", &unfollow_event.follower, &unfollow_event.unfollowed);
                        if let Err(e) = self.process_unfollow_event(&unfollow_event, Some(&event)).await {
                            error!("Failed to process unfollow event: {}", e);
                        }
                    },
                    Err(e) => {
                        error!("🚨 SOCIAL GRAPH: Failed to parse unfollow event: {}", e);
                        error!("Event data that failed to parse: {}", serde_json::to_string_pretty(&event.data).unwrap_or_default());
                    }
                }
            } else {
                info!("🚨 SOCIAL GRAPH: Social graph event type not specifically handled: {}", event.event_type);
            }
        } else {
            // Log that we're skipping non-social-graph events (but only at debug level to avoid spam)
            debug!("Social graph handler skipping non-social-graph event: {}", event.event_type);
        }
        
        Ok(())
    }
    
    /// Start listening for social graph events
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting social graph event handler");
        
        while let Some(event) = self.rx.recv().await {
            info!("🚨 SOCIAL GRAPH HANDLER: Received event: {}", event.event_type);
            info!("🚨 SOCIAL GRAPH HANDLER: Event ID: {}", event.event_id);
            info!("🚨 SOCIAL GRAPH HANDLER: Event data: {}", serde_json::to_string_pretty(&event.data).unwrap_or_default());
            
            if let Err(e) = self.process_event(event).await {
                error!("Error processing event: {}", e);
            }
        }
        
        warn!("Social graph event handler channel closed");
        Ok(())
    }
}

/// Handle specific event types from MysEvent
pub async fn handle_event(db: &Arc<Database>, event: &MysEvent, transaction_id: &str) -> Result<()> {
    let event_type = &event.type_.to_string(); // Convert StructTag to String
    
    if event_type.ends_with("::FollowEvent") {
        let parsed_event = parse_event::<FollowEvent>(event)
            .map_err(|e| anyhow!("Failed to parse FollowEvent: {}", e))?;
        
        // Create a temporary BlockchainEvent to use with existing handlers
        let blockchain_event = BlockchainEvent {
            event_type: event_type.clone(), // Using the string version
            event_id: transaction_id.to_string(),
            data: serde_json::to_value(event).unwrap_or_default(),
            timestamp_ms: 0, // Not used in this context
            tx_digest: transaction_id.to_string(),
        };
        
        // Create a handler instance just for this event
        let handler = SocialGraphEventHandler::new(
            db.clone(), 
            mpsc::channel(1).1, // Dummy channel that won't be used
            "direct_handler".to_string()
        );
        
        handler.process_follow_event(&parsed_event, Some(&blockchain_event)).await?;
    } else if event_type.ends_with("::UnfollowEvent") {
        let parsed_event = parse_event::<UnfollowEvent>(event)
            .map_err(|e| anyhow!("Failed to parse UnfollowEvent: {}", e))?;
        
        // Create a temporary BlockchainEvent to use with existing handlers
        let blockchain_event = BlockchainEvent {
            event_type: event_type.clone(), // Using the string version
            event_id: transaction_id.to_string(),
            data: serde_json::to_value(event).unwrap_or_default(),
            timestamp_ms: 0, // Not used in this context
            tx_digest: transaction_id.to_string(),
        };
        
        // Create a handler instance just for this event
        let handler = SocialGraphEventHandler::new(
            db.clone(), 
            mpsc::channel(1).1, // Dummy channel that won't be used
            "direct_handler".to_string()
        );
        
        handler.process_unfollow_event(&parsed_event, Some(&blockchain_event)).await?;
    }
    
    Ok(())
}