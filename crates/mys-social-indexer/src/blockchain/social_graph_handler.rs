// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::blockchain::listener::BlockchainEvent;
use crate::db::{Database, DbConnection};
use crate::events::{FollowEvent, UnfollowEvent};
use crate::models::indexer::NewIndexerProgress;
use crate::schema;

/// SocialGraphEventHandler handles all social graph events from the blockchain
pub struct SocialGraphEventHandler {
    db: Arc<Database>,
    rx: mpsc::Receiver<BlockchainEvent>,
    worker_name: String,
}

impl SocialGraphEventHandler {
    /// Create a new SocialGraphEventHandler instance
    pub fn new(
        db: Arc<Database>,
        rx: mpsc::Receiver<BlockchainEvent>,
        worker_name: String,
    ) -> Self {
        Self {
            db,
            rx,
            worker_name,
        }
    }

    /// Get a database connection from the pool
    async fn get_connection(&self) -> Result<DbConnection> {
        self.db
            .get_connection()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }

    /// Update worker progress
    async fn update_progress(&self) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now().naive_utc();

        let progress = NewIndexerProgress {
            id: self.worker_name.clone(),
            last_checkpoint_processed: 0,
            last_processed_at: now,
        };

        diesel::insert_into(schema::indexer_progress::table)
            .values(&progress)
            .on_conflict(schema::indexer_progress::id)
            .do_update()
            .set((
                schema::indexer_progress::last_checkpoint_processed
                    .eq(progress.last_checkpoint_processed),
                schema::indexer_progress::last_processed_at.eq(progress.last_processed_at),
            ))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Process a follow event
    async fn process_follow_event(
        &self,
        event: &FollowEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        // Check if relationship already exists
        let exists = diesel::select(diesel::dsl::exists(
            schema::social_graph_relationships::table
                .filter(schema::social_graph_relationships::follower_address.eq(&event.follower))
                .filter(schema::social_graph_relationships::following_address.eq(&event.following)),
        ))
        .get_result::<bool>(&mut conn)
        .await?;

        if exists {
            info!(
                "Follow relationship already exists: {} -> {}",
                event.follower, event.following
            );
            return Ok(());
        }

        // Convert event to database model
        let relationship = event.into_relationship()?;

        // Insert the follow relationship
        diesel::insert_into(schema::social_graph_relationships::table)
            .values(&relationship)
            .execute(&mut conn)
            .await?;

        // Log the follow event to social_graph_events table
        let event_log = crate::models::social_graph::NewSocialGraphEvent {
            event_type: "follow".to_string(),
            follower_address: event.follower.clone(),
            following_address: event.following.clone(),
            created_at: relationship.created_at,
            event_id: blockchain_event.map(|e| e.event_id.clone()),
            raw_event_data: Some(serde_json::to_value(event)?),
        };

        diesel::insert_into(schema::social_graph_events::table)
            .values(&event_log)
            .execute(&mut conn)
            .await?;

        // Write to relay outbox for notifications
        let event_data = serde_json::json!({
            "follower_address": event.follower,
            "following_address": event.following,
        });
        if let Err(e) = crate::relay_outbox::write_notification_event(
            &mut conn,
            "follow.created",
            &event_data,
            blockchain_event.map(|e| e.event_id.as_str()),
            blockchain_event.map(|e| e.tx_digest.as_str()),
        )
        .await
        {
            warn!("Failed to write follow event to outbox: {}", e);
        }

        // Update the follower's following_count (+1)
        // First try to update by profile_id, then by owner_address if no rows affected
        let follower_updated = diesel::update(schema::profiles::table)
            .filter(schema::profiles::profile_id.eq(&event.follower))
            .set(schema::profiles::following_count.eq(schema::profiles::following_count + 1))
            .execute(&mut conn)
            .await?;

        // If no rows were updated by profile_id, try owner_address
        if follower_updated == 0 {
            diesel::update(schema::profiles::table)
                .filter(schema::profiles::owner_address.eq(&event.follower))
                .set(schema::profiles::following_count.eq(schema::profiles::following_count + 1))
                .execute(&mut conn)
                .await?;
        }

        // Update the followed profile's followers_count (+1)
        // First try to update by profile_id, then by owner_address if no rows affected
        let following_updated = diesel::update(schema::profiles::table)
            .filter(schema::profiles::profile_id.eq(&event.following))
            .set(schema::profiles::followers_count.eq(schema::profiles::followers_count + 1))
            .execute(&mut conn)
            .await?;

        // If no rows were updated by profile_id, try owner_address
        if following_updated == 0 {
            diesel::update(schema::profiles::table)
                .filter(schema::profiles::owner_address.eq(&event.following))
                .set(schema::profiles::followers_count.eq(schema::profiles::followers_count + 1))
                .execute(&mut conn)
                .await?;
        }

        Ok(())
    }

    /// Process an unfollow event
    async fn process_unfollow_event(
        &self,
        event: &UnfollowEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        // Delete the follow relationship
        let deleted_count = diesel::delete(schema::social_graph_relationships::table)
            .filter(schema::social_graph_relationships::follower_address.eq(&event.follower))
            .filter(schema::social_graph_relationships::following_address.eq(&event.unfollowed))
            .execute(&mut conn)
            .await?;

        if deleted_count == 0 {
            info!(
                "No follow relationship to remove: {} -> {}",
                event.follower, event.unfollowed
            );
            return Ok(());
        }

        // Log the unfollow event to social_graph_events table
        let event_log = crate::models::social_graph::NewSocialGraphEvent {
            event_type: "unfollow".to_string(),
            follower_address: event.follower.clone(),
            following_address: event.unfollowed.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            event_id: blockchain_event.map(|e| e.event_id.clone()),
            raw_event_data: Some(serde_json::to_value(event)?),
        };

        diesel::insert_into(schema::social_graph_events::table)
            .values(&event_log)
            .execute(&mut conn)
            .await?;

        // Write to relay outbox for notifications
        let event_data = serde_json::json!({
            "follower_address": event.follower,
            "unfollowed_address": event.unfollowed,
        });
        if let Err(e) = crate::relay_outbox::write_notification_event(
            &mut conn,
            "unfollow.created",
            &event_data,
            blockchain_event.map(|e| e.event_id.as_str()),
            blockchain_event.map(|e| e.tx_digest.as_str()),
        )
        .await
        {
            warn!("Failed to write unfollow event to outbox: {}", e);
        }

        // Update the follower's following_count (-1)
        // First try to update by profile_id, then by owner_address if no rows affected
        let follower_updated = diesel::update(schema::profiles::table)
            .filter(
                schema::profiles::profile_id
                    .eq(&event.follower)
                    .and(schema::profiles::following_count.gt(0)),
            )
            .set(schema::profiles::following_count.eq(schema::profiles::following_count - 1))
            .execute(&mut conn)
            .await?;

        // If no rows were updated by profile_id, try owner_address
        if follower_updated == 0 {
            diesel::update(schema::profiles::table)
                .filter(
                    schema::profiles::owner_address
                        .eq(&event.follower)
                        .and(schema::profiles::following_count.gt(0)),
                )
                .set(schema::profiles::following_count.eq(schema::profiles::following_count - 1))
                .execute(&mut conn)
                .await?;
        }

        // Update the unfollowed profile's followers_count (-1)
        // First try to update by profile_id, then by owner_address if no rows affected
        let unfollowed_updated = diesel::update(schema::profiles::table)
            .filter(
                schema::profiles::profile_id
                    .eq(&event.unfollowed)
                    .and(schema::profiles::followers_count.gt(0)),
            )
            .set(schema::profiles::followers_count.eq(schema::profiles::followers_count - 1))
            .execute(&mut conn)
            .await?;

        // If no rows were updated by profile_id, try owner_address
        if unfollowed_updated == 0 {
            diesel::update(schema::profiles::table)
                .filter(
                    schema::profiles::owner_address
                        .eq(&event.unfollowed)
                        .and(schema::profiles::followers_count.gt(0)),
                )
                .set(schema::profiles::followers_count.eq(schema::profiles::followers_count - 1))
                .execute(&mut conn)
                .await?;
        }

        Ok(())
    }

    /// Start the social graph event handler
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting social graph event handler: {}", self.worker_name);

        while let Some(event) = self.rx.recv().await {
            debug!("Received blockchain event: {:?}", event);

            // Check if this is a social graph event
            if event.event_type.contains("::social_graph::") {
                info!("Processing social graph event: {}", event.event_type);

                let mut event_handled = false;

                // Handle follow event
                if event.event_type.ends_with("::FollowEvent") {
                    event_handled = true;
                    // Extract fields from JSON and parse as FollowEvent
                    match crate::events::event_utils::extract_event_fields(&event.data).and_then(
                        |fields| {
                            serde_json::from_value::<FollowEvent>(fields)
                                .map_err(|e| anyhow::anyhow!(e))
                        },
                    ) {
                        Ok(follow_event) => {
                            if let Err(e) =
                                self.process_follow_event(&follow_event, Some(&event)).await
                            {
                                error!("Failed to process follow event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse follow event: {}", e);
                        }
                    }
                }

                // Handle unfollow event
                if event.event_type.ends_with("::UnfollowEvent") {
                    event_handled = true;
                    // Extract fields from JSON and parse as UnfollowEvent
                    match crate::events::event_utils::extract_event_fields(&event.data).and_then(
                        |fields| {
                            serde_json::from_value::<UnfollowEvent>(fields)
                                .map_err(|e| anyhow::anyhow!(e))
                        },
                    ) {
                        Ok(unfollow_event) => {
                            if let Err(e) = self
                                .process_unfollow_event(&unfollow_event, Some(&event))
                                .await
                            {
                                error!("Failed to process unfollow event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse unfollow event: {}", e);
                        }
                    }
                }

                // Warn if we received a social_graph event but didn't handle it
                if !event_handled {
                    warn!(
                        "Received unhandled social_graph event: {} (event_id: {})",
                        event.event_type, event.event_id
                    );
                }

                // Update progress after processing the event
                if let Err(e) = self.update_progress().await {
                    error!("Failed to update progress: {}", e);
                }
            }
        }

        warn!("Social graph event handler channel closed");
        Ok(())
    }
}
