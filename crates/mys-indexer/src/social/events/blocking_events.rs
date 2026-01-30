// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde_json;
use tracing::{error, info, warn};

use crate::social::db::DbConnection;
use crate::social::events::platform_events::PlatformEventType;
use crate::social::events::profile_event_types::{BlockAddedEvent, BlockRemovedEvent};
use crate::social::models::blocking::{NewBlockedEvent, NewBlockedProfile};
use crate::social::models::blocking::{UserBlockEvent, UserUnblockEvent};
use crate::social::models::profile::NewProfile;
use crate::social::models::profile_events::NewProfileEvent;
use crate::social::models::platform::{NewPlatformBlockedProfile, NewPlatformEvent};
use crate::social::schema::{blocked_events, blocked_profiles, platform_blocked_profiles, platform_events, profile_events, profiles};


/// Ensure a profile exists for the given wallet address.
/// Creates a minimal profile if one doesn't exist, using any provided profile data.
/// This is idempotent - if a profile already exists, it does nothing.
async fn ensure_profile_exists(
    conn: &mut DbConnection,
    wallet_address: &str,
    existing_username: Option<&str>,
    existing_display_name: Option<&str>,
    existing_profile_photo: Option<&str>,
) -> Result<()> {
    use diesel::dsl::exists;
    use diesel::select;

    // Check if profile already exists
    let profile_exists = select(exists(
        profiles::table.filter(profiles::owner_address.eq(wallet_address)),
    ))
    .get_result::<bool>(conn)
    .await
    .unwrap_or(false);

    if profile_exists {
        info!(
            "Profile already exists for wallet address: {}",
            wallet_address
        );
        return Ok(());
    }

    // Generate username from address if not provided
    let username = existing_username.map(|s| s.to_string()).unwrap_or_else(|| {
        format!(
            "user_{}",
            wallet_address.chars().take(8).collect::<String>()
        )
    });

    let now = chrono::Utc::now().naive_utc();

    // Create minimal profile
    let new_profile = NewProfile {
        owner_address: wallet_address.to_string(),
        username: username.clone(),
        display_name: existing_display_name.map(|s| s.to_string()),
        bio: None,
        profile_photo: existing_profile_photo.map(|s| s.to_string()),
        website: None,
        created_at: now,
        updated_at: now,
        cover_photo: None,
        profile_id: None,
        followers_count: 0,
        following_count: 0,
        blocked_count: 0,
        post_count: 0,
        min_offer_amount: None,
        birthdate: None,
        current_location: None,
        raised_location: None,
        phone: None,
        email: None,
        gender: None,
        political_view: None,
        religion: None,
        education: None,
        primary_language: None,
        relationship_status: None,
        x_username: None,
        facebook_username: None,
        reddit_username: None,
        github_username: None,
        instagram_username: None,
        linkedin_username: None,
        twitch_username: None,
        social_proof_token_address: None,
        reservation_pool_address: None,
        selected_badge_id: None,
        paid_messaging_enabled: false,
        paid_messaging_min_cost: None,
    };

    // Insert profile, handling conflicts gracefully (idempotent)
    match diesel::insert_into(profiles::table)
        .values(&new_profile)
        .on_conflict(profiles::owner_address)
        .do_nothing()
        .execute(conn)
        .await
    {
        Ok(0) => {
            // Profile already exists (race condition handled)
            info!(
                "Profile already exists for wallet address (race condition): {}",
                wallet_address
            );
        }
        Ok(_) => {
            info!(
                "Created minimal profile for wallet address: {} with username: {}",
                wallet_address, username
            );
        }
        Err(e) => {
            // If it's a unique constraint violation on username, try with a modified username
            if e.to_string().contains("username") || e.to_string().contains("unique") {
                warn!(
                    "Username conflict for {}, trying with address-based username",
                    wallet_address
                );
                // Try again with a more unique username
                let unique_username = format!(
                    "user_{}_{}",
                    wallet_address.chars().take(8).collect::<String>(),
                    chrono::Utc::now().timestamp()
                );
                let mut retry_profile = new_profile;
                retry_profile.username = unique_username.clone();

                if let Err(retry_err) = diesel::insert_into(profiles::table)
                    .values(&retry_profile)
                    .on_conflict(profiles::owner_address)
                    .do_nothing()
                    .execute(conn)
                    .await
                {
                    error!(
                        "Failed to create profile for wallet address {} even after retry: {}",
                        wallet_address, retry_err
                    );
                    return Err(anyhow::anyhow!("Failed to create profile: {}", retry_err));
                } else {
                    info!(
                        "Created profile with unique username for wallet address: {}",
                        wallet_address
                    );
                }
            } else {
                error!(
                    "Failed to create profile for wallet address {}: {}",
                    wallet_address, e
                );
                return Err(anyhow::anyhow!("Failed to create profile: {}", e));
            }
        }
    }

    Ok(())
}

/// Process a profile block event
/// Detects if blocker is a platform address and routes to appropriate handler
pub async fn process_profile_block_event(
    conn: &mut DbConnection,
    event_data: &serde_json::Value,
) -> Result<()> {
    // Log the raw event data for debugging
    info!(
        "Processing profile block event (raw data): {:?}",
        event_data
    );

    // Parse the UserBlockEvent
    let block_event = match serde_json::from_value::<UserBlockEvent>(event_data.clone()) {
        Ok(evt) => {
            info!(
                "Successfully parsed UserBlockEvent: blocker={}, blocked={}",
                evt.blocker, evt.blocked
            );
            evt
        }
        Err(e) => {
            info!("Failed to parse UserBlockEvent: {}", e);

            // Extract directly from JSON
            let empty_map = serde_json::Map::new();
            let obj = event_data.as_object().unwrap_or(&empty_map);

            // Try to extract from fields container first
            if let Some(fields_obj) = obj.get("fields").and_then(|f| f.as_object()) {
                // Try to extract blocker and blocked
                let blocker = fields_obj
                    .get("blocker")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                let blocked = fields_obj
                    .get("blocked")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                UserBlockEvent { blocker, blocked }
            }
            // Try module-level properties directly
            else if obj.get("blocker").is_some() && obj.get("blocked").is_some() {
                UserBlockEvent {
                    blocker: obj
                        .get("blocker")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    blocked: obj
                        .get("blocked")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                }
            }
            // Try event container (may be nested)
            else if let Some(event_obj) = obj.get("event").and_then(|e| e.as_object()) {
                UserBlockEvent {
                    blocker: event_obj
                        .get("blocker")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    blocked: event_obj
                        .get("blocked")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                }
            }
            // As a last resort, try to parse raw JSON
            else {
                // Create a placeholder event
                UserBlockEvent {
                    blocker: "unknown".to_string(),
                    blocked: "unknown".to_string(),
                }
            }
        }
    };

    // Check if we have valid data
    if block_event.blocker.is_empty()
        || block_event.blocker == "unknown"
        || block_event.blocked.is_empty()
        || block_event.blocked == "unknown"
    {
        info!("Invalid block event data, skipping");
        return Ok(());
    }

    // Check if blocker is a platform address
    use crate::social::schema::platforms;
    let is_platform = match platforms::table
        .filter(platforms::platform_id.eq(&block_event.blocker))
        .select(platforms::platform_id)
        .first::<String>(conn)
        .await
    {
        Ok(_) => {
            info!("Detected platform blocking: platform {} blocked wallet {}", block_event.blocker, block_event.blocked);
            true
        }
        Err(diesel::NotFound) => {
            info!("User-to-user blocking: {} blocked {}", block_event.blocker, block_event.blocked);
            false
        }
        Err(e) => {
            warn!("Failed to check if blocker is platform: {}, treating as user blocking", e);
            false
        }
    };

    // Route to platform handler if blocker is a platform
    if is_platform {
        return process_platform_wallet_block_event(
            conn,
            &block_event.blocker,
            &block_event.blocked,
            event_data,
        )
        .await;
    }

    info!(
        "Processing user-to-user block event: {} blocked {}",
        block_event.blocker, block_event.blocked
    );

    let now = chrono::Utc::now().naive_utc();

    // 1. Insert into blocked_events for audit trail
    let blocked_event = NewBlockedEvent::new_block_event(
        None, // event_id - could be extracted from blockchain if available
        block_event.blocker.clone(),
        block_event.blocked.clone(),
        Some(event_data.clone()),
        now,
    );

    let event_result = diesel::insert_into(blocked_events::table)
        .values(&blocked_event)
        .execute(conn)
        .await;

    // 2. Fetch blocked user's profile data for rich information
    let (blocked_profile_id, blocked_username, blocked_display_name, blocked_profile_photo) = {
        use crate::social::schema::profiles;

        match profiles::table
            .filter(profiles::owner_address.eq(&block_event.blocked))
            .select((
                profiles::profile_id.nullable(),
                profiles::username,
                profiles::display_name.nullable(),
                profiles::profile_photo.nullable(),
            ))
            .first::<(Option<String>, String, Option<String>, Option<String>)>(conn)
            .await
        {
            Ok((profile_id, username, display_name, profile_photo)) => {
                info!(
                    "Found rich profile data for blocked user {}: username={}",
                    block_event.blocked, username
                );
                (profile_id, Some(username), display_name, profile_photo)
            }
            Err(e) => {
                info!("Could not find profile data for blocked user {}: {}", block_event.blocked, e);
                // Profile doesn't exist - will be created by ensure_profile_exists
                (None, None, None, None)
            }
        }
    };

    // Clone values before they're moved in the database update
    let blocked_display_name_clone = blocked_display_name.clone();
    let blocked_profile_photo_clone = blocked_profile_photo.clone();

    // Before upsert, check if an active block record already exists (to maintain accurate blocked_count)
    // NOTE: Currently unused but kept for potential future use in maintaining accurate blocked_count
    // let _existed_before = {
    //     use diesel::dsl::exists;
    //     diesel::select(exists(
    //         blocked_profiles::table
    //             .filter(blocked_profiles::blocker_address.eq(&block_event.blocker))
    //             .filter(blocked_profiles::blocked_address.eq(&block_event.blocked)),
    //     ))
    //     .get_result::<bool>(conn)
    //     .await
    //     .unwrap_or(false)
    // };

    // 3. Insert or update blocked_profiles for current state with rich data
    let new_blocked_profile = NewBlockedProfile::new(
        block_event.blocker.clone(),
        block_event.blocked.clone(),
        blocked_profile_id.clone(),
        blocked_username.clone().unwrap_or_else(|| block_event.blocked.clone()),
        blocked_display_name.clone(),
        blocked_profile_photo.clone(),
        now,
    );

    let profile_result = diesel::insert_into(blocked_profiles::table)
        .values(&new_blocked_profile)
        .on_conflict((
            blocked_profiles::blocker_address,
            blocked_profiles::blocked_address,
        ))
        .do_update()
        .set((
            blocked_profiles::last_blocked_at.eq(now),
            // Update rich profile data in case it has changed
            blocked_profiles::blocked_profile_id.eq(blocked_profile_id),
            blocked_profiles::blocked_username.eq(blocked_username.clone().unwrap_or_else(|| block_event.blocked.clone())),
            blocked_profiles::blocked_display_name.eq(blocked_display_name.clone()),
            blocked_profiles::blocked_profile_photo.eq(blocked_profile_photo.clone()),
            // Increment count only when re-blocking the same profile
            blocked_profiles::total_block_count.eq(blocked_profiles::total_block_count + 1_i32),
        ))
        .execute(conn)
        .await;

    // Log results
    match (event_result, profile_result) {
        (Ok(_), Ok(_)) => {
            info!("✅ Successfully wrote block event to production blocking system");

            // Ensure profiles exist for both blocker and blocked addresses
            // This ensures that follower/following counts are correctly updated when relationships are deleted
            info!(
                "Ensuring profiles exist for blocker={} and blocked={}",
                block_event.blocker, block_event.blocked
            );

            // Ensure blocker profile exists
            if let Err(e) = ensure_profile_exists(
                conn,
                &block_event.blocker,
                None, // No existing username data for blocker
                None, // No existing display_name data for blocker
                None, // No existing profile_photo data for blocker
            )
            .await
            {
                warn!(
                    "Failed to ensure profile exists for blocker {}: {}",
                    block_event.blocker, e
                );
                // Continue anyway - the relationship deletion will still work
            }

            // Ensure blocked profile exists, using any available profile data
            if let Err(e) = ensure_profile_exists(
                conn,
                &block_event.blocked,
                blocked_username.as_deref(), // Use fetched username if available (None if profile didn't exist)
                blocked_display_name_clone.as_deref(), // Use cloned display_name if available
                blocked_profile_photo_clone.as_deref(), // Use cloned profile_photo if available
            )
            .await
            {
                warn!(
                    "Failed to ensure profile exists for blocked {}: {}",
                    block_event.blocked, e
                );
                // Continue anyway - the relationship deletion will still work
            }

            // Note: blocked_count is now automatically updated by database triggers
            // when blocked_profiles table is updated (see update_blocked_count trigger)

            // Remove follow relationships in BOTH directions when blocking
            // This ensures blocking automatically unfollows in both directions:
            // 1. Blocker unfollows blocked user (if following)
            // 2. Blocked user unfollows blocker (if following)
            //
            // Smart contract uses wallet addresses for following/blocking.
            // Relationships are stored with wallet addresses, so we delete using wallet addresses.
            
            info!(
                "BLOCK EVENT: Severing follow relationships: blocker={}, blocked={}",
                block_event.blocker, block_event.blocked
            );
            
            // Remove blocker -> blocked relationship
            let blocker_to_blocked_deleted = diesel::delete(crate::social::schema::social_graph_relationships::table)
                .filter(crate::social::schema::social_graph_relationships::follower_address.eq(&block_event.blocker))
                .filter(crate::social::schema::social_graph_relationships::following_address.eq(&block_event.blocked))
                .execute(conn)
                .await;

            match blocker_to_blocked_deleted {
                Ok(deleted_count) => {
                    if deleted_count > 0 {
                        info!(
                            "BLOCK EVENT: Deleted {} relationship(s): {} -> {} (triggers will update counts)",
                            deleted_count, block_event.blocker, block_event.blocked
                        );

                        // Log the unfollow event for audit trail
                        let unfollow_event = crate::social::models::social_graph::NewSocialGraphEvent {
                            event_type: "unfollow_blocked".to_string(),
                            follower_address: block_event.blocker.clone(),
                            following_address: block_event.blocked.clone(),
                            created_at: now,
                            event_id: None,
                            raw_event_data: Some(serde_json::json!({
                                "reason": "blocked",
                                "direction": "blocker_to_blocked",
                                "blocker": block_event.blocker,
                                "blocked": block_event.blocked,
                                "deleted_count": deleted_count,
                            })),
                        };

                        if let Err(e) = diesel::insert_into(crate::social::schema::social_graph_events::table)
                            .values(&unfollow_event)
                            .execute(conn)
                            .await
                        {
                            warn!("Failed to log unfollow_blocked event: {}", e);
                        }
                    } else {
                        info!(
                            "BLOCK EVENT: No relationship found to delete: {} -> {} (may not have been following)",
                            block_event.blocker, block_event.blocked
                        );
                    }
                }
                Err(e) => {
                    warn!(
                        "BLOCK EVENT: Failed to remove follow relationship when blocking {} -> {}: {}",
                        block_event.blocker, block_event.blocked, e
                    );
                }
            }

            // Remove blocked -> blocker relationship (reverse direction)
            let blocked_to_blocker_deleted = diesel::delete(crate::social::schema::social_graph_relationships::table)
                .filter(crate::social::schema::social_graph_relationships::follower_address.eq(&block_event.blocked))
                .filter(crate::social::schema::social_graph_relationships::following_address.eq(&block_event.blocker))
                .execute(conn)
                .await;

            match blocked_to_blocker_deleted {
                Ok(deleted_count) => {
                    if deleted_count > 0 {
                        info!(
                            "BLOCK EVENT: Deleted {} reverse relationship(s): {} -> {} (triggers will update counts)",
                            deleted_count, block_event.blocked, block_event.blocker
                        );

                        // Log the unfollow event for audit trail
                        let unfollow_event = crate::social::models::social_graph::NewSocialGraphEvent {
                            event_type: "unfollow_blocked".to_string(),
                            follower_address: block_event.blocked.clone(),
                            following_address: block_event.blocker.clone(),
                            created_at: now,
                            event_id: None,
                            raw_event_data: Some(serde_json::json!({
                                "reason": "blocked",
                                "direction": "blocked_to_blocker",
                                "blocker": block_event.blocker,
                                "blocked": block_event.blocked,
                                "deleted_count": deleted_count,
                            })),
                        };

                        if let Err(e) = diesel::insert_into(crate::social::schema::social_graph_events::table)
                            .values(&unfollow_event)
                            .execute(conn)
                            .await
                        {
                            warn!("Failed to log unfollow_blocked event (reverse): {}", e);
                        }
                    } else {
                        info!(
                            "BLOCK EVENT: No reverse relationship found to delete: {} -> {} (may not have been following)",
                            block_event.blocked, block_event.blocker
                        );
                    }
                }
                Err(e) => {
                    warn!(
                        "BLOCK EVENT: Failed to remove reverse follow relationship when blocking {} -> {}: {}",
                        block_event.blocked, block_event.blocker, e
                    );
                }
            }

            // Create a profile_events entry to track in user history
            let block_timestamp = chrono::Utc::now().timestamp() as u64;

            // Create block added event for profile_events
            let profile_block_event = BlockAddedEvent {
                blocker_profile_id: block_event.blocker.clone(),
                blocked_profile_id: block_event.blocked.clone(),
                timestamp: block_timestamp,
            };

            // Create profile event for blocking
            let profile_event = NewProfileEvent::from_block_added(
                &profile_block_event,
                None, // No event ID available
            );

            // Insert into profile_events
            let event_result = diesel::insert_into(profile_events::table)
                .values(&profile_event)
                .execute(conn)
                .await;

            match event_result {
                Ok(_) => {
                    info!("Successfully created profile_events record for block event");
                }
                Err(e) => {
                    error!("Failed to insert block event into profile_events: {}", e);
                }
            }

            // Write to relay outbox for notifications
            let event_data = serde_json::json!({
                "blocker_address": block_event.blocker,
                "blocked_address": block_event.blocked,
            });
            if let Err(e) = crate::social::relay_outbox::write_notification_event(
                conn,
                "blocked.created",
                &event_data,
                None, // event_id not available in this context
                None, // transaction_id not available in this context
            )
            .await
            {
                warn!("Failed to write block event to outbox: {}", e);
            }
        }
        (Err(e), _) => {
            error!("Failed to insert into blocked_events table: {}", e);
            return Err(anyhow::anyhow!("Failed to write audit event: {}", e));
        }
        (_, Err(e)) => {
            error!("Failed to insert/update blocked_profiles table: {}", e);
            return Err(anyhow::anyhow!("Failed to update blocking state: {}", e));
        }
    }

    Ok(())
}

/// Process a profile unblock event
/// Detects if blocker is a platform address and routes to appropriate handler
pub async fn process_profile_unblock_event(
    conn: &mut DbConnection,
    event_data: &serde_json::Value,
) -> Result<()> {
    // Log the raw event data for debugging
    info!(
        "Processing profile unblock event (raw data): {:?}",
        event_data
    );

    // Try to parse the event data
    let unblock_event = match serde_json::from_value::<UserUnblockEvent>(event_data.clone()) {
        Ok(evt) => {
            info!(
                "Successfully parsed unblock event: blocker={}, unblocked={}",
                evt.blocker, evt.unblocked
            );
            evt
        }
        Err(e) => {
            info!("Failed to parse UserUnblockEvent: {}", e);

            // Extract directly from JSON
            let empty_map = serde_json::Map::new();
            let obj = event_data.as_object().unwrap_or(&empty_map);

            // Try to extract from fields container
            if let Some(fields_obj) = obj.get("fields").and_then(|f| f.as_object()) {
                // Try to extract blocker and unblocked
                let blocker = fields_obj
                    .get("blocker")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                let unblocked = fields_obj
                    .get("unblocked")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                UserUnblockEvent { blocker, unblocked }
            } else {
                // Try root-level fields directly
                let blocker = obj
                    .get("blocker")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                let unblocked = obj
                    .get("unblocked")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                UserUnblockEvent { blocker, unblocked }
            }
        }
    };

    // Check if all required fields are present
    if unblock_event.blocker.is_empty() || unblock_event.unblocked.is_empty() {
        info!("Missing required fields in unblock event, skipping");
        return Ok(());
    }

    // Check if blocker is a platform address
    use crate::social::schema::platforms;
    let is_platform = match platforms::table
        .filter(platforms::platform_id.eq(&unblock_event.blocker))
        .select(platforms::platform_id)
        .first::<String>(conn)
        .await
    {
        Ok(_) => {
            info!("Detected platform unblocking: platform {} unblocked wallet {}", unblock_event.blocker, unblock_event.unblocked);
            true
        }
        Err(diesel::NotFound) => {
            info!("User-to-user unblocking: {} unblocked {}", unblock_event.blocker, unblock_event.unblocked);
            false
        }
        Err(e) => {
            warn!("Failed to check if blocker is platform: {}, treating as user unblocking", e);
            false
        }
    };

    // Route to platform handler if blocker is a platform
    if is_platform {
        return process_platform_wallet_unblock_event(
            conn,
            &unblock_event.blocker,
            &unblock_event.unblocked,
            event_data,
        )
        .await;
    }

    info!(
        "Processing user-to-user unblock event: {} unblocked {}",
        unblock_event.blocker, unblock_event.unblocked
    );

    let now = chrono::Utc::now().naive_utc();

    // 1. Insert into blocked_events for audit trail
    let blocked_event = NewBlockedEvent::new_unblock_event(
        None, // event_id - could be extracted from blockchain if available
        unblock_event.blocker.clone(),
        unblock_event.unblocked.clone(),
        Some(event_data.clone()),
        now,
    );

    let event_result = diesel::insert_into(blocked_events::table)
        .values(&blocked_event)
        .execute(conn)
        .await;

    // 2. Delete from blocked_profiles for current state
    let profile_result = diesel::delete(blocked_profiles::table)
        .filter(blocked_profiles::blocker_address.eq(unblock_event.blocker.clone()))
        .filter(blocked_profiles::blocked_address.eq(unblock_event.unblocked.clone()))
        .execute(conn)
        .await;

    // Log results
    match (event_result, profile_result) {
        (Ok(_), Ok(deleted_rows)) => {
            info!(
                "✅ Successfully processed unblock in production system: {} records deleted",
                deleted_rows
            );

            // Create a profile_events entry to track in user history
            let unblock_timestamp = chrono::Utc::now().timestamp() as u64;

            // Create block removed event for profile_events
            let profile_unblock_event = BlockRemovedEvent {
                blocker_profile_id: unblock_event.blocker.clone(),
                blocked_profile_id: unblock_event.unblocked.clone(),
                timestamp: unblock_timestamp,
            };

            // Create profile event for unblocking
            let profile_event = NewProfileEvent::from_block_removed(
                &profile_unblock_event,
                None, // No event ID available
            );

            // Insert into profile_events
            let event_result = diesel::insert_into(profile_events::table)
                .values(&profile_event)
                .execute(conn)
                .await;

            match event_result {
                Ok(_) => {
                    info!("Successfully created profile_events record for unblock event");
                }
                Err(e) => {
                    error!("Failed to insert unblock event into profile_events: {}", e);
                }
            }

            // Write to relay outbox for notifications
            let event_data = serde_json::json!({
                "blocker_address": unblock_event.blocker,
                "unblocked_address": unblock_event.unblocked,
            });
            if let Err(e) = crate::social::relay_outbox::write_notification_event(
                conn,
                "unblocked.created",
                &event_data,
                None, // event_id not available in this context
                None, // transaction_id not available in this context
            )
            .await
            {
                warn!("Failed to write unblock event to outbox: {}", e);
            }

            // Note: blocked_count is now automatically updated by database triggers
            // when blocked_profiles table is updated (see update_blocked_count trigger)
        }
        (Err(e), _) => {
            error!("Failed to insert into blocked_events table: {}", e);
            return Err(anyhow::anyhow!("Failed to write audit event: {}", e));
        }
        (_, Err(e)) => {
            error!("Failed to delete from blocked_profiles table: {}", e);
            return Err(anyhow::anyhow!("Failed to update blocking state: {}", e));
        }
    }

    Ok(())
}

/// Process a platform wallet block event (from UserBlockEvent where blocker is a platform)
/// Stores in platform_blocked_profiles table using wallet_address
pub async fn process_platform_wallet_block_event(
    conn: &mut DbConnection,
    platform_id: &str,
    blocked_wallet_address: &str,
    event_data: &serde_json::Value,
) -> Result<()> {
    info!(
        "Processing platform wallet block event: platform {} blocked wallet {}",
        platform_id, blocked_wallet_address
    );

    let now = chrono::Utc::now().naive_utc();

    // 1. Insert into blocked_events for audit trail (same as user blocking)
    let blocked_event = NewBlockedEvent::new_block_event(
        None, // event_id - could be extracted from blockchain if available
        platform_id.to_string(),
        blocked_wallet_address.to_string(),
        Some(event_data.clone()),
        now,
    );

    let event_result = diesel::insert_into(blocked_events::table)
        .values(&blocked_event)
        .execute(conn)
        .await;

    // 2. Insert or update platform_blocked_profiles table using wallet_address
    let new_platform_blocked = NewPlatformBlockedProfile {
        platform_id: platform_id.to_string(),
        wallet_address: blocked_wallet_address.to_string(),
        blocked_by: platform_id.to_string(), // Platform acts as blocker
        created_at: now,
    };

    let platform_block_result = diesel::insert_into(platform_blocked_profiles::table)
        .values(&new_platform_blocked)
        .on_conflict((
            platform_blocked_profiles::platform_id,
            platform_blocked_profiles::wallet_address,
        ))
        .do_update()
        .set(platform_blocked_profiles::created_at.eq(now))
        .execute(conn)
        .await;

    // Log results
    match (event_result, platform_block_result) {
        (Ok(_), Ok(_)) => {
            info!("✅ Successfully wrote platform wallet block event");

            // Create a profile_events entry to track in user history
            let block_timestamp = chrono::Utc::now().timestamp() as u64;

            // Create block added event for profile_events with platform block metadata
            let profile_event = NewProfileEvent::from_blockchain_event(
                "BlockAdded",
                blocked_wallet_address.to_string(),
                serde_json::json!({
                    "platform_id": platform_id,
                    "blocked_by": platform_id,
                    "timestamp": block_timestamp,
                    "is_platform_block": true
                }),
                None, // No event ID available
                Some(block_timestamp),
            );

            // Insert into profile_events
            let event_result = diesel::insert_into(profile_events::table)
                .values(&profile_event)
                .execute(conn)
                .await;

            match event_result {
                Ok(_) => {
                    info!("Successfully created profile_events record for platform block event");
                }
                Err(e) => {
                    error!("Failed to insert platform block event into profile_events: {}", e);
                }
            }

            // Create platform_events entry for platform events endpoint
            let platform_event_data = serde_json::json!({
                "platform_id": platform_id,
                "profile_id": blocked_wallet_address,
                "blocked_by": platform_id
            });

            let platform_event = NewPlatformEvent {
                event_type: PlatformEventType::ProfileBlocked.to_str().to_string(),
                platform_id: platform_id.to_string(),
                event_data: platform_event_data,
                event_id: None,
                created_at: now,
                reasoning: None,
            };

            let platform_event_result = diesel::insert_into(platform_events::table)
                .values(&platform_event)
                .execute(conn)
                .await;

            match platform_event_result {
                Ok(_) => {
                    info!("Successfully created platform_events record for platform block event");
                }
                Err(e) => {
                    error!("Failed to insert platform block event into platform_events: {}", e);
                }
            }
        }
        (Err(e), _) => {
            error!("Failed to insert into blocked_events table: {}", e);
            return Err(anyhow::anyhow!("Failed to write audit event: {}", e));
        }
        (_, Err(e)) => {
            error!("Failed to insert/update platform_blocked_profiles table: {}", e);
            return Err(anyhow::anyhow!("Failed to update platform blocking state: {}", e));
        }
    }

    Ok(())
}

/// Process a platform wallet unblock event (from UserUnblockEvent where blocker is a platform)
/// Removes from platform_blocked_profiles table using wallet_address
pub async fn process_platform_wallet_unblock_event(
    conn: &mut DbConnection,
    platform_id: &str,
    unblocked_wallet_address: &str,
    event_data: &serde_json::Value,
) -> Result<()> {
    info!(
        "Processing platform wallet unblock event: platform {} unblocked wallet {}",
        platform_id, unblocked_wallet_address
    );

    let now = chrono::Utc::now().naive_utc();

    // 1. Insert into blocked_events for audit trail
    let blocked_event = NewBlockedEvent::new_unblock_event(
        None, // event_id - could be extracted from blockchain if available
        platform_id.to_string(),
        unblocked_wallet_address.to_string(),
        Some(event_data.clone()),
        now,
    );

    let event_result = diesel::insert_into(blocked_events::table)
        .values(&blocked_event)
        .execute(conn)
        .await;

    // 2. Delete from platform_blocked_profiles for current state
    let platform_unblock_result = diesel::delete(platform_blocked_profiles::table)
        .filter(platform_blocked_profiles::platform_id.eq(platform_id))
        .filter(platform_blocked_profiles::wallet_address.eq(unblocked_wallet_address))
        .execute(conn)
        .await;

    // Log results
    match (event_result, platform_unblock_result) {
        (Ok(_), Ok(deleted_rows)) => {
            info!(
                "✅ Successfully processed platform wallet unblock: {} records deleted",
                deleted_rows
            );

            // Create a profile_events entry to track in user history
            let unblock_timestamp = chrono::Utc::now().timestamp() as u64;

            // Create block removed event for profile_events with platform unblock metadata
            let profile_event = NewProfileEvent::from_blockchain_event(
                "BlockRemoved",
                unblocked_wallet_address.to_string(),
                serde_json::json!({
                    "platform_id": platform_id,
                    "unblocked_by": platform_id,
                    "timestamp": unblock_timestamp,
                    "is_platform_block": true
                }),
                None, // No event ID available
                Some(unblock_timestamp),
            );

            // Insert into profile_events
            let event_result = diesel::insert_into(profile_events::table)
                .values(&profile_event)
                .execute(conn)
                .await;

            match event_result {
                Ok(_) => {
                    info!("Successfully created profile_events record for platform unblock event");
                }
                Err(e) => {
                    error!("Failed to insert platform unblock event into profile_events: {}", e);
                }
            }

            // Create platform_events entry for platform events endpoint
            let platform_event_data = serde_json::json!({
                "platform_id": platform_id,
                "profile_id": unblocked_wallet_address,
                "unblocked_by": platform_id
            });

            let platform_event = NewPlatformEvent {
                event_type: PlatformEventType::ProfileUnblocked.to_str().to_string(),
                platform_id: platform_id.to_string(),
                event_data: platform_event_data,
                event_id: None,
                created_at: now,
                reasoning: None,
            };

            let platform_event_result = diesel::insert_into(platform_events::table)
                .values(&platform_event)
                .execute(conn)
                .await;

            match platform_event_result {
                Ok(_) => {
                    info!("Successfully created platform_events record for platform unblock event");
                }
                Err(e) => {
                    error!("Failed to insert platform unblock event into platform_events: {}", e);
                }
            }
        }
        (Err(e), _) => {
            error!("Failed to insert into blocked_events table: {}", e);
            return Err(anyhow::anyhow!("Failed to write audit event: {}", e));
        }
        (_, Err(e)) => {
            error!("Failed to delete from platform_blocked_profiles table: {}", e);
            return Err(anyhow::anyhow!("Failed to update platform blocking state: {}", e));
        }
    }

    Ok(())
}
