// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::blockchain::listener::BlockchainEvent;
use crate::db::{Database, DbConnection};
use crate::events::blocking_events;
use crate::events::profile_event_types::ProfileEventType;
use crate::events::profile_events::{
    ProfileCreatedEvent, ProfileUpdatedEvent, TokensClaimedEvent, TokensVestedEvent,
};
use crate::models::indexer::NewIndexerProgress;
use crate::models::profile_events::NewProfileEvent;
use crate::schema;
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
            last_checkpoint_processed: 0, // Not tracking specific checkpoints in event-driven system
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

    /// Try to manually parse a profile event from the raw JSON data
    async fn try_manual_profile_parse(&self, data: &serde_json::Value) -> Result<()> {
        info!(
            "Manually extracting profile data from: {}",
            serde_json::to_string_pretty(data).unwrap_or_default()
        );

        // Try different JSON paths to extract the profile data
        let profile = if let Some(obj) = data.as_object() {
            // Look for fields container in Move event structure
            let fields_container = if let Some(fields) = obj.get("fields") {
                fields.as_object()
            } else {
                Some(obj)
            };

            let fields = fields_container.unwrap_or(obj);

            // Log all available fields for debugging
            info!("Available fields: {:?}", fields.keys().collect::<Vec<_>>());

            // Extract the fields we need
            let profile_id = fields
                .get("profile_id")
                .and_then(|v| v.as_str())
                .or_else(|| obj.get("profile_id").and_then(|v| v.as_str()))
                .unwrap_or("unknown")
                .to_string();

            let owner = fields
                .get("owner_address")
                .or(fields.get("owner"))
                .and_then(|v| v.as_str())
                .or_else(|| {
                    obj.get("owner_address")
                        .or(obj.get("owner"))
                        .and_then(|v| v.as_str())
                })
                .unwrap_or("unknown")
                .to_string();

            let username = fields
                .get("username")
                .and_then(|v| v.as_str())
                .or_else(|| obj.get("username").and_then(|v| v.as_str()))
                .unwrap_or("unknown")
                .to_string();

            let display_name = fields
                .get("display_name")
                .and_then(|v| v.as_str())
                .or_else(|| obj.get("display_name").and_then(|v| v.as_str()))
                .map(|s| s.to_string());

            // Look for bio in various locations and formats
            let bio_value = fields.get("bio").or_else(|| obj.get("bio"));
            let bio = if let Some(b) = bio_value {
                if b.is_string() {
                    Some(b.as_str().unwrap().to_string())
                } else if b.is_object() {
                    // Try to extract string value from complex object
                    b.get("vec")
                        .and_then(|v| v.get(0))
                        .and_then(|s| s.get("string"))
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string())
                        .or_else(|| Some("".to_string())) // Default to empty if we can't extract
                } else {
                    None
                }
            } else {
                None
            };

            // Extract profile photo with similar logic
            let profile_photo_value = fields
                .get("profile_picture")
                .or(fields.get("profile_photo"))
                .or(fields.get("avatar_url"))
                .or_else(|| obj.get("profile_picture"))
                .or_else(|| obj.get("profile_photo"))
                .or_else(|| obj.get("avatar_url"));

            let profile_photo = if let Some(p) = profile_photo_value {
                if p.is_string() {
                    Some(p.as_str().unwrap().to_string())
                } else if p.is_object() {
                    // Extract URL from complex object
                    p.get("fields")
                        .and_then(|f| f.get("url"))
                        .and_then(|u| u.get("vec"))
                        .and_then(|v| v.get(0))
                        .and_then(|s| s.get("string"))
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string())
                } else {
                    None
                }
            } else {
                None
            };

            // Extract cover photo with similar logic
            let cover_photo_value = fields.get("cover_photo").or_else(|| obj.get("cover_photo"));

            let cover_photo = if let Some(c) = cover_photo_value {
                if c.is_string() {
                    Some(c.as_str().unwrap().to_string())
                } else if c.is_object() {
                    // Extract URL from complex object
                    c.get("fields")
                        .and_then(|f| f.get("url"))
                        .and_then(|u| u.get("vec"))
                        .and_then(|v| v.get(0))
                        .and_then(|s| s.get("string"))
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string())
                } else {
                    None
                }
            } else {
                None
            };

            // Log extracted fields
            info!("Extracted profile_id: {}", profile_id);
            info!("Extracted owner: {}", owner);
            info!("Extracted username: {}", username);
            info!("Extracted display_name: {:?}", display_name);
            info!("Extracted bio: {:?}", bio);
            info!("Extracted profile_photo: {:?}", profile_photo);
            info!("Extracted cover_photo: {:?}", cover_photo);

            // Create event manually
            ProfileCreatedEvent {
                profile_id,
                owner_address: owner,
                username: Some(username),
                display_name: display_name.unwrap_or_default(),
                bio,
                profile_photo,
                cover_photo,
                created_at: chrono::Utc::now().timestamp() as u64,
            }
        } else {
            return Err(anyhow!("Data is not an object"));
        };

        // Process the manually constructed profile
        info!("Manually parsed profile: {:?}", profile);
        self.process_profile_created(&profile).await
    }

    /// Process a profile created event
    async fn process_profile_created(&self, event: &ProfileCreatedEvent) -> Result<()> {
        self.process_profile_created_with_context(event, None).await
    }

    /// Process a profile created event with blockchain context
    async fn process_profile_created_with_context(
        &self,
        event: &ProfileCreatedEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!(
            "Processing ProfileCreatedEvent: profile_id={}, username={:?}",
            event.profile_id, event.username
        );

        // Convert event to database model
        let new_profile = event.into_model()?;

        // Insert the profile
        diesel::insert_into(schema::profiles::table)
            .values(&new_profile)
            .on_conflict(schema::profiles::username)
            .do_update()
            .set((
                schema::profiles::owner_address.eq(&new_profile.owner_address),
                schema::profiles::display_name.eq(&new_profile.display_name),
                schema::profiles::bio.eq(&new_profile.bio),
                schema::profiles::profile_photo.eq(&new_profile.profile_photo),
                schema::profiles::website.eq(&new_profile.website),
                schema::profiles::updated_at.eq(&new_profile.updated_at),
                schema::profiles::cover_photo.eq(&new_profile.cover_photo),
                schema::profiles::profile_id.eq(&new_profile.profile_id),
                schema::profiles::post_count.eq(&new_profile.post_count),
                // Sensitive fields
                schema::profiles::birthdate.eq(&new_profile.birthdate),
                schema::profiles::current_location.eq(&new_profile.current_location),
                schema::profiles::raised_location.eq(&new_profile.raised_location),
                schema::profiles::phone.eq(&new_profile.phone),
                schema::profiles::email.eq(&new_profile.email),
                schema::profiles::gender.eq(&new_profile.gender),
                schema::profiles::political_view.eq(&new_profile.political_view),
                schema::profiles::religion.eq(&new_profile.religion),
                schema::profiles::education.eq(&new_profile.education),
                schema::profiles::primary_language.eq(&new_profile.primary_language),
                schema::profiles::relationship_status.eq(&new_profile.relationship_status),
                schema::profiles::x_username.eq(&new_profile.x_username),
                schema::profiles::mastodon_username.eq(&new_profile.mastodon_username),
                schema::profiles::facebook_username.eq(&new_profile.facebook_username),
                schema::profiles::reddit_username.eq(&new_profile.reddit_username),
                schema::profiles::github_username.eq(&new_profile.github_username),
            ))
            .execute(&mut conn)
            .await?;

        // Log the profile created event to profile_events table
        let profile_event = NewProfileEvent::from_blockchain_event(
            ProfileEventType::ProfileCreated.to_str(),
            event.profile_id.clone(),
            serde_json::to_value(event)?,
            blockchain_event.map(|e| e.event_id.clone()),
            Some(event.created_at as u64),
        );

        diesel::insert_into(schema::profile_events::table)
            .values(&profile_event)
            .execute(&mut conn)
            .await?;

        info!(
            "✅ Successfully processed and logged profile created: {}",
            event.profile_id
        );
        Ok(())
    }

    /// Process a profile updated event
    async fn process_profile_updated(&self, event: &ProfileUpdatedEvent) -> Result<()> {
        self.process_profile_updated_with_context(event, None).await
    }

    /// Process a profile updated event with blockchain context
    async fn process_profile_updated_with_context(
        &self,
        event: &ProfileUpdatedEvent,
        blockchain_event: Option<&BlockchainEvent>,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!(
            "Processing ProfileUpdatedEvent: profile_id={}",
            event.profile_id
        );
        info!(
            "Update fields: display_name={:?}, bio={:?}, profile_photo={:?}, cover_photo={:?}",
            event.display_name, event.bio, event.profile_photo, event.cover_photo
        );

        // Find the profile by profile_id
        let existing_profile = schema::profiles::table
            .filter(schema::profiles::profile_id.eq(&event.profile_id))
            .first::<crate::models::profile::Profile>(&mut conn)
            .await
            .map_err(|e| anyhow!("Profile not found with ID {}: {}", event.profile_id, e))?;

        // Update profile fields with new values, keeping existing values for fields not being updated
        let updated_at = chrono::Utc::now().naive_utc();

        diesel::update(schema::profiles::table)
            .filter(schema::profiles::profile_id.eq(&event.profile_id))
            .set((
                // Update display_name if provided
                schema::profiles::display_name.eq(event
                    .display_name
                    .as_ref()
                    .unwrap_or(&existing_profile.display_name.unwrap_or_default())),
                // Update bio if provided
                schema::profiles::bio.eq(event.bio.as_ref().or(existing_profile.bio.as_ref())),
                // Update profile_photo if provided
                schema::profiles::profile_photo.eq(event
                    .profile_photo
                    .as_ref()
                    .or(existing_profile.profile_photo.as_ref())),
                // Update cover_photo if provided
                schema::profiles::cover_photo.eq(event
                    .cover_photo
                    .as_ref()
                    .or(existing_profile.cover_photo.as_ref())),
                // Update username if provided
                schema::profiles::username.eq(event
                    .username
                    .as_ref()
                    .unwrap_or(&existing_profile.username)),
                // Always update the timestamp
                schema::profiles::updated_at.eq(updated_at),
                // Update sensitive fields if provided
                schema::profiles::birthdate.eq(event
                    .birthdate
                    .as_ref()
                    .or(existing_profile.birthdate.as_ref())),
                schema::profiles::current_location.eq(event
                    .current_location
                    .as_ref()
                    .or(existing_profile.current_location.as_ref())),
                schema::profiles::raised_location.eq(event
                    .raised_location
                    .as_ref()
                    .or(existing_profile.raised_location.as_ref())),
                schema::profiles::phone
                    .eq(event.phone.as_ref().or(existing_profile.phone.as_ref())),
                schema::profiles::email
                    .eq(event.email.as_ref().or(existing_profile.email.as_ref())),
                schema::profiles::gender
                    .eq(event.gender.as_ref().or(existing_profile.gender.as_ref())),
                schema::profiles::political_view.eq(event
                    .political_view
                    .as_ref()
                    .or(existing_profile.political_view.as_ref())),
                schema::profiles::religion.eq(event
                    .religion
                    .as_ref()
                    .or(existing_profile.religion.as_ref())),
                schema::profiles::education.eq(event
                    .education
                    .as_ref()
                    .or(existing_profile.education.as_ref())),
                schema::profiles::primary_language.eq(event
                    .primary_language
                    .as_ref()
                    .or(existing_profile.primary_language.as_ref())),
                schema::profiles::relationship_status.eq(event
                    .relationship_status
                    .as_ref()
                    .or(existing_profile.relationship_status.as_ref())),
                schema::profiles::x_username.eq(event
                    .x_username
                    .as_ref()
                    .or(existing_profile.x_username.as_ref())),
                schema::profiles::mastodon_username.eq(event
                    .mastodon_username
                    .as_ref()
                    .or(existing_profile.mastodon_username.as_ref())),
                schema::profiles::facebook_username.eq(event
                    .facebook_username
                    .as_ref()
                    .or(existing_profile.facebook_username.as_ref())),
                schema::profiles::reddit_username.eq(event
                    .reddit_username
                    .as_ref()
                    .or(existing_profile.reddit_username.as_ref())),
                schema::profiles::github_username.eq(event
                    .github_username
                    .as_ref()
                    .or(existing_profile.github_username.as_ref())),
                schema::profiles::min_offer_amount.eq(event
                    .min_offer_amount
                    .map(|v| v as i64)
                    .or(existing_profile.min_offer_amount)),
            ))
            .execute(&mut conn)
            .await?;

        // Log the profile updated event to profile_events table
        let profile_event = NewProfileEvent::from_blockchain_event(
            ProfileEventType::ProfileUpdated.to_str(),
            event.profile_id.clone(),
            serde_json::to_value(event)?,
            blockchain_event.map(|e| e.event_id.clone()),
            Some(event.updated_at as u64),
        );

        diesel::insert_into(schema::profile_events::table)
            .values(&profile_event)
            .execute(&mut conn)
            .await?;

        info!(
            "✅ Successfully processed and logged profile updated: {}",
            event.profile_id
        );
        Ok(())
    }

    /// Process a tokens vested event
    async fn process_tokens_vested(
        &self,
        event: &TokensVestedEvent,
        transaction_id: &str,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!(
            "Processing TokensVestedEvent: wallet_id={}, owner={}, amount={}",
            event.wallet_id, event.owner, event.total_amount
        );

        // Convert event to database models
        let (new_wallet, new_event) = event.into_models(transaction_id.to_string());

        // Insert the vesting wallet (or update if it already exists)
        diesel::insert_into(schema::vesting_wallets::table)
            .values(&new_wallet)
            .on_conflict(schema::vesting_wallets::wallet_id)
            .do_update()
            .set((
                schema::vesting_wallets::owner_address.eq(&new_wallet.owner_address),
                schema::vesting_wallets::total_amount.eq(&new_wallet.total_amount),
                schema::vesting_wallets::start_time.eq(&new_wallet.start_time),
                schema::vesting_wallets::duration.eq(&new_wallet.duration),
                schema::vesting_wallets::curve_factor.eq(&new_wallet.curve_factor),
                schema::vesting_wallets::updated_at.eq(&new_wallet.updated_at),
                schema::vesting_wallets::transaction_id.eq(&new_wallet.transaction_id),
            ))
            .execute(&mut conn)
            .await?;

        // Insert the vesting event
        diesel::insert_into(schema::vesting_events::table)
            .values(&new_event)
            .execute(&mut conn)
            .await?;

        info!(
            "✅ Successfully processed tokens vested: wallet_id={}",
            event.wallet_id
        );
        Ok(())
    }

    /// Process a tokens claimed event
    async fn process_tokens_claimed(
        &self,
        event: &TokensClaimedEvent,
        transaction_id: &str,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!("Processing TokensClaimedEvent: wallet_id={}, owner={}, claimed_amount={}, remaining_balance={}", 
              event.wallet_id, event.owner, event.claimed_amount, event.remaining_balance);

        // Convert event to database models
        let (wallet_update, new_event) = event.into_models(transaction_id.to_string());

        // Update the vesting wallet with new claimed amount and remaining balance
        diesel::update(
            schema::vesting_wallets::table
                .filter(schema::vesting_wallets::wallet_id.eq(&event.wallet_id)),
        )
        .set(&wallet_update)
        .execute(&mut conn)
        .await?;

        // Insert the claim event
        diesel::insert_into(schema::vesting_events::table)
            .values(&new_event)
            .execute(&mut conn)
            .await?;

        info!(
            "✅ Successfully processed tokens claimed: wallet_id={}, amount={}",
            event.wallet_id, event.claimed_amount
        );
        Ok(())
    }

    /// Process platform block event
    async fn process_platform_block_event(&self, event_data: &serde_json::Value) -> Result<()> {
        let mut conn = self.get_connection().await?;
        blocking_events::process_platform_block_event(&mut conn, event_data).await
    }

    /// Process platform unblock event
    async fn process_platform_unblock_event(&self, event_data: &serde_json::Value) -> Result<()> {
        let mut conn = self.get_connection().await?;
        blocking_events::process_platform_unblock_event(&mut conn, event_data).await
    }

    /// Start the profile event listener
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting profile event listener: {}", self.worker_name);

        while let Some(event) = self.receiver.recv().await {
            debug!("Received blockchain event: {:?}", event);

            // Check if this is a profile event
            if event.event_type.contains("::profile::") {
                info!("Processing profile event: {}", event.event_type);

                // Handle profile created event
                if event.event_type.ends_with("::ProfileCreatedEvent") {
                    // Log the raw event data for debugging
                    info!(
                        "Profile created event detected with data: {}",
                        serde_json::to_string_pretty(&event.data).unwrap_or_default()
                    );

                    // Extract fields from JSON and parse as ProfileCreatedEvent
                    match crate::events::event_utils::extract_event_fields(&event.data).and_then(
                        |fields| {
                            serde_json::from_value::<ProfileCreatedEvent>(fields)
                                .map_err(|e| anyhow::anyhow!(e))
                        },
                    ) {
                        Ok(profile_event) => {
                            info!(
                                "Successfully parsed profile created event: {:?}",
                                profile_event
                            );
                            if let Err(e) = self
                                .process_profile_created_with_context(&profile_event, Some(&event))
                                .await
                            {
                                error!("Failed to process profile created event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize profile created event: {}", e);

                            // Try to parse the profile event struct manually
                            info!("Attempting manual profile event parsing...");
                            if let Err(parse_err) = self.try_manual_profile_parse(&event.data).await
                            {
                                error!("Manual parsing also failed: {}", parse_err);
                            }
                        }
                    }
                }
                // Handle profile updated event
                else if event.event_type.ends_with("::ProfileUpdatedEvent") {
                    // Log the raw event data for debugging
                    info!(
                        "Profile updated event detected with data: {}",
                        serde_json::to_string_pretty(&event.data).unwrap_or_default()
                    );

                    // Extract fields from JSON and parse as ProfileUpdatedEvent
                    match crate::events::event_utils::extract_event_fields(&event.data).and_then(
                        |fields| {
                            serde_json::from_value::<ProfileUpdatedEvent>(fields)
                                .map_err(|e| anyhow::anyhow!(e))
                        },
                    ) {
                        Ok(profile_event) => {
                            info!(
                                "Successfully parsed profile updated event: {:?}",
                                profile_event
                            );
                            if let Err(e) = self
                                .process_profile_updated_with_context(&profile_event, Some(&event))
                                .await
                            {
                                error!("Failed to process profile updated event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize profile updated event: {}", e);
                        }
                    }
                }
                // Handle tokens vested event
                else if event.event_type.ends_with("::TokensVestedEvent") {
                    info!(
                        "Tokens vested event detected with data: {}",
                        serde_json::to_string_pretty(&event.data).unwrap_or_default()
                    );

                    // Extract fields from JSON and parse as TokensVestedEvent
                    match crate::events::event_utils::extract_event_fields(&event.data).and_then(
                        |fields| {
                            serde_json::from_value::<TokensVestedEvent>(fields)
                                .map_err(|e| anyhow::anyhow!(e))
                        },
                    ) {
                        Ok(vesting_event) => {
                            info!(
                                "Successfully parsed tokens vested event: {:?}",
                                vesting_event
                            );
                            if let Err(e) = self
                                .process_tokens_vested(&vesting_event, &event.tx_digest)
                                .await
                            {
                                error!("Failed to process tokens vested event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize tokens vested event: {}", e);
                        }
                    }
                }
                // Handle tokens claimed event
                else if event.event_type.ends_with("::TokensClaimedEvent") {
                    info!(
                        "Tokens claimed event detected with data: {}",
                        serde_json::to_string_pretty(&event.data).unwrap_or_default()
                    );

                    // Extract fields from JSON and parse as TokensClaimedEvent
                    match crate::events::event_utils::extract_event_fields(&event.data).and_then(
                        |fields| {
                            serde_json::from_value::<TokensClaimedEvent>(fields)
                                .map_err(|e| anyhow::anyhow!(e))
                        },
                    ) {
                        Ok(claim_event) => {
                            info!(
                                "Successfully parsed tokens claimed event: {:?}",
                                claim_event
                            );
                            if let Err(e) = self
                                .process_tokens_claimed(&claim_event, &event.tx_digest)
                                .await
                            {
                                error!("Failed to process tokens claimed event: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize tokens claimed event: {}", e);
                        }
                    }
                }

                // Update progress after processing the event
                if let Err(e) = self.update_progress().await {
                    error!("Failed to update progress: {}", e);
                }
            }
            // Platform blocking events are handled by platform_handler.rs to avoid duplication
            // UserBlockEvent and UserUnblockEvent are handled by block_list_handler.rs to avoid duplication
            // BlockListCreatedEvent is handled by block_list_handler.rs to avoid duplication
        }

        warn!("Profile event listener channel closed");
        Ok(())
    }

    /// Process a profile event from the blockchain (legacy method for backward compatibility)
    pub async fn process_event(
        &self,
        _conn: &mut DbConnection,
        module_name: &str,
        function_name: &str,
        event_data: &Value,
        _event_id: &str,
    ) -> Result<()> {
        // Skip if this is not a profile module event
        if module_name != PROFILE_MODULE_NAME {
            return Ok(());
        }

        info!(
            "Processing profile event: {} (legacy method)",
            function_name
        );

        // Handle profile events by function name
        match function_name {
            "create_profile" | "ProfileCreated" => {
                info!("Processing profile creation event via legacy method");

                // Try to parse as ProfileCreatedEvent
                match crate::events::event_utils::extract_event_fields(event_data).and_then(
                    |fields| {
                        serde_json::from_value::<ProfileCreatedEvent>(fields)
                            .map_err(|e| anyhow::anyhow!(e))
                    },
                ) {
                    Ok(profile_event) => {
                        if let Err(e) = self.process_profile_created(&profile_event).await {
                            error!("Failed to process profile created event: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse profile created event: {}", e);
                        // Try manual parsing
                        if let Err(parse_err) = self.try_manual_profile_parse(event_data).await {
                            error!("Manual parsing also failed: {}", parse_err);
                        }
                    }
                }
            }
            "update_profile" | "ProfileUpdated" => {
                info!("Processing profile update event via legacy method");

                // Try to parse as ProfileUpdatedEvent
                match crate::events::event_utils::extract_event_fields(event_data).and_then(
                    |fields| {
                        serde_json::from_value::<ProfileUpdatedEvent>(fields)
                            .map_err(|e| anyhow::anyhow!(e))
                    },
                ) {
                    Ok(profile_event) => {
                        if let Err(e) = self.process_profile_updated(&profile_event).await {
                            error!("Failed to process profile updated event: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse profile updated event: {}", e);
                        // Log the full event data for debugging
                        error!(
                            "Event data: {}",
                            serde_json::to_string_pretty(event_data).unwrap_or_default()
                        );
                    }
                }
            }
            "register_username" | "UsernameRegistered" => {
                debug!("Processing username registration event (not yet implemented)");
            }
            "update_username" | "UsernameUpdated" => {
                debug!("Processing username update event (not yet implemented)");
            }
            _ => {
                debug!("Unknown profile function: {}", function_name);
            }
        }

        info!("Processed profile event: {}", function_name);
        Ok(())
    }
}
