use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{Utc, NaiveDate};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use mys_data_ingestion_core::Worker;
use mys_types::full_checkpoint_content::CheckpointData;
use mys_types::event::{Event as MysEvent, EventID};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::db::{Database, DbConnection};
use crate::events::{
    parse_event,
    MODULE_PREFIX_PROFILE, MODULE_PREFIX_PLATFORM, MODULE_PREFIX_CONTENT,
    MODULE_PREFIX_BLOCK_LIST, MODULE_PREFIX_MYDATA, MODULE_PREFIX_FEE_DISTRIBUTION,
    MODULE_PREFIX_SOCIAL_GRAPH,
    ProfileCreatedEvent, ProfileUpdatedEvent, UsernameUpdatedEvent, UsernameRegisteredEvent, 
    PlatformCreatedEvent, ContentCreatedEvent, ContentInteractionEvent,
    EntityBlockedEvent, IPRegisteredEvent, LicenseGrantedEvent, ProofCreatedEvent,
    FeeModelCreatedEvent, FeesDistributedEvent, ProfileFollowEvent, ProfileJoinedPlatformEvent,
    FollowEvent, UnfollowEvent,
    PlatformBlockedProfileEvent, PlatformUnblockedProfileEvent, UserJoinedPlatformEvent, UserLeftPlatformEvent,
    SpotBetPlacedEvent as SpotBetPlacedEvt,
    SpotResolvedEvent as SpotResolvedEvt,
    SpotDaoRequiredEvent as SpotDaoRequiredEvt,
    SpotPayoutEvent as SpotPayoutEvt,
    SpotRefundEvent as SpotRefundEvt,
    TokensVestedEvent, TokensClaimedEvent,
    PostCreatedEvent,
};
use crate::models::profile::{NewProfile, NewProfilePlatformLink, UpdateProfile};
use crate::models::username::{NewUsername, UpdateUsername, NewUsernameHistory};
use crate::models::platform::NewPlatformBlockedProfile;
// These model imports will be added when we implement these features
//use crate::models::platform::NewPlatform;
//use crate::models::content::{NewContent, NewContentInteraction};
//use crate::models::block_list::NewBlock;
//use crate::models::intellectual_property::{NewIntellectualProperty, NewIPLicense, NewProofOfCreativity};
//use crate::models::fee_distribution::{NewFeeModel, NewFeeDistribution, NewFeeRecipient, NewFeeRecipientPayment};
use crate::models::statistics::{NewDailyStatistics, NewPlatformDailyStatistics};
use crate::models::indexer::NewIndexerProgress;
use crate::schema;
use crate::models::{
    NewSpotRecord, UpdateSpotRecord, NewSpotBet, NewSpotPayout, NewSpotRefund, NewSpotResolution,
    VestingWallet, UpdateVestingWallet,
};
use diesel::dsl::now;

/// Social indexer worker that processes blockchain events
pub struct SocialIndexerWorker {
    /// Database connection pool
    db: Arc<Database>,
    /// Worker ID
    worker_id: String,
}

impl SocialIndexerWorker {
    /// Create a new social indexer worker
    pub fn new(db: Arc<Database>, worker_id: String) -> Self {
        Self { db, worker_id }
    }
    
    /// Get a database connection from the pool
    async fn get_connection(&self) -> Result<DbConnection> {
        self.db.get_connection()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }
    
    /// Update worker progress
    async fn update_progress(&self, checkpoint_seq: u64) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now();
        
        let progress = NewIndexerProgress {
            id: self.worker_id.clone(),
            last_checkpoint_processed: checkpoint_seq as i64,
            last_processed_at: now,
        };
        
        diesel::insert_into(schema::indexer_progress::table)
            .values(&progress)
            .on_conflict(schema::indexer_progress::id)
            .do_update()
            .set(&progress)
            .execute(&mut conn)
            .await?;
            
        Ok(())
    }
    
    /// Process a profile created event
    async fn process_profile_created(&self, event: &ProfileCreatedEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing ProfileCreatedEvent: profile_id={}, username={:?}", 
              event.profile_id, event.username);
        
        // Convert event to database model
        let new_profile = event.into_model()?;
        
        // Insert the profile
        let result = diesel::insert_into(schema::profiles::table)
            .values(&new_profile)
            .on_conflict(schema::profiles::id)
            .do_update()
            .set(&new_profile)
            .returning(schema::profiles::id) // Return the profile ID
            .get_result::<i32>(&mut conn)
            .await?;
            
        let profile_id = result; // This is the newly created profile's ID
        
        // If the profile has a username, add it to the usernames table
        if let Some(username) = &event.username {
            info!("Profile has username: {}, adding to usernames table", username);
            
            // Check if the username already exists in the usernames table
            let username_exists = schema::usernames::table
                .filter(schema::usernames::profile_id.eq(profile_id))
                .filter(schema::usernames::username.eq(username))
                .first::<crate::models::username::Username>(&mut conn)
                .await.is_ok();
                
            if !username_exists {
                // Use current time instead of blockchain epoch
                // Blockchain epoch values are small numbers and not actual Unix timestamps
                let now = Utc::now().naive_utc();
                info!("Using current timestamp for username registration: {}", now);
                
                // Create a new username record
                let new_username = NewUsername {
                    profile_id,
                    username: username.clone(),
                    registered_at: now,
                    updated_at: now,
                };
                
                // Insert the username
                info!("Inserting username record into usernames table");
                match diesel::insert_into(schema::usernames::table)
                    .values(&new_username)
                    .execute(&mut conn)
                    .await {
                    Ok(rows) => info!("Successfully inserted {} username record(s) for: {}", rows, username),
                    Err(e) => error!("Failed to insert username record: {}", e)
                };
                
                // Verify the insertion worked
                info!("Verifying username insertion");
                match schema::usernames::table
                    .filter(schema::usernames::profile_id.eq(profile_id))
                    .filter(schema::usernames::username.eq(username))
                    .first::<crate::models::username::Username>(&mut conn)
                    .await {
                    Ok(username_rec) => info!("Verified username record exists: id={}, username={}", username_rec.id, username_rec.username),
                    Err(e) => error!("Username record not found after insertion: {}", e)
                }
            } else {
                info!("Username already exists in usernames table for this profile");
            }
        } else {
            info!("Profile doesn't have a username, skipping usernames table insertion");
        }
            
        // Update daily statistics
        self.update_daily_stats(|stats| {
            stats.new_profiles_count += 1;
        }).await?;
        
        info!("Processed profile created: {}", event.profile_id);
        Ok(())
    }
    
    /// Process a profile updated event
    async fn process_profile_updated(&self, event: &ProfileUpdatedEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Find the profile by profile_id
        let profile = schema::profiles::table
            .filter(schema::profiles::profile_id.eq(&event.profile_id))
            .first::<crate::models::profile::Profile>(&mut conn)
            .await?;
        
        // Log all fields for debugging
        info!("Processing ProfileUpdatedEvent:");
        info!("  profile_id: {}", event.profile_id);
        info!("  display_name: {:?}", event.display_name);
        info!("  bio: {:?}", event.bio);
        info!("  profile_photo: {:?}", event.profile_photo);
        info!("  cover_photo: {:?}", event.cover_photo);
        info!("  website: {:?}", event.website);
        
        // For existing profile in database:
        info!("Existing profile in database:");
        info!("  id: {}", profile.id);
        info!("  display_name: {:?}", profile.display_name);
        info!("  bio: {:?}", profile.bio);
        info!("  profile_photo: {:?}", profile.profile_photo);
        info!("  cover_photo: {:?}", profile.cover_photo);
        info!("  website: {:?}", profile.website);
        
        // Create an update model - use existing values when event doesn't provide them
        // Use website field from event if provided, otherwise keep existing
        
        // Use current time instead of blockchain epoch
        // Blockchain epoch values are small numbers and not actual Unix timestamps
        let now = Utc::now().naive_utc();
        info!("Using current timestamp instead of blockchain epoch: {}", now);
        
        let update = UpdateProfile {
            display_name: event.display_name.clone(),
            bio: if event.bio.is_some() { event.bio.clone() } else { profile.bio.clone() },
            profile_photo: if event.profile_photo.is_some() { event.profile_photo.clone() } else { profile.profile_photo.clone() },
            website: event.website.clone(),  // Use new website field from event
            cover_photo: if event.cover_photo.is_some() { event.cover_photo.clone() } else { profile.cover_photo.clone() },
            // Keep existing counts unchanged during profile updates
            followers_count: None,
            following_count: None,
            blocked_count: None,
            post_count: None,
            min_offer_amount: event.min_offer_amount.map(|v| v as i64),
            birthdate: event.birthdate.clone(),
            current_location: event.current_location.clone(),
            raised_location: event.raised_location.clone(),
            phone: event.phone.clone(),
            email: event.email.clone(),
            gender: event.gender.clone(),
            political_view: event.political_view.clone(),
            religion: event.religion.clone(),
            education: event.education.clone(),
            primary_language: event.primary_language.clone(),
            relationship_status: event.relationship_status.clone(),
            x_username: event.x_username.clone(),
            mastodon_username: event.mastodon_username.clone(),
            facebook_username: event.facebook_username.clone(),
            reddit_username: event.reddit_username.clone(),
            github_username: event.github_username.clone(),
            instagram_username: event.instagram_username.clone(),
            social_proof_token_address: event.social_proof_token_address.clone(),
            selected_badge_id: event.selected_badge_id.clone(),
            paid_messaging_enabled: None, // Not updated via ProfileUpdatedEvent
            paid_messaging_min_cost: None, // Not updated via ProfileUpdatedEvent
        };
        
        info!("Updating profile with:");
        info!("  display_name: {:?}", update.display_name);
        info!("  bio: {:?}", update.bio);
        info!("  profile_photo: {:?}", update.profile_photo);
        info!("  website: {:?}", update.website);
        info!("  cover_photo: {:?}", update.cover_photo);
        
        // Update the profile
        diesel::update(schema::profiles::table.find(profile.id))
            .set(&update)
            .execute(&mut conn)
            .await?;
            
        info!("Processed profile updated: {}", event.profile_id);
        Ok(())
    }
    
    /// Process a username updated event
    async fn process_username_updated(&self, event: &UsernameUpdatedEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Find the profile by profile_id
        let profile = schema::profiles::table
            .filter(schema::profiles::profile_id.eq(&event.profile_id))
            .first::<crate::models::profile::Profile>(&mut conn)
            .await?;
        
        // Update the profile table's username column (for backward compatibility)
        diesel::update(schema::profiles::table.find(profile.id))
            .set(schema::profiles::username.eq(&event.new_username))
            .execute(&mut conn)
            .await?;
        
        // Check if the username exists in the usernames table
        let username_result = schema::usernames::table
            .filter(schema::usernames::profile_id.eq(profile.id))
            .first::<crate::models::username::Username>(&mut conn)
            .await;
            
        let now = Utc::now().naive_utc();
        
        // If the username record exists, update it
        if let Ok(username) = username_result {
            // Update the username in the usernames table
            diesel::update(schema::usernames::table.find(username.id))
                .set(UpdateUsername {
                    username: Some(event.new_username.clone()),
                    updated_at: Some(now),
                })
                .execute(&mut conn)
                .await?;
        } else {
            // If username doesn't exist, create a new record
            let new_username = NewUsername {
                profile_id: profile.id,
                username: event.new_username.clone(),
                registered_at: now,
                updated_at: now,
            };
            
            diesel::insert_into(schema::usernames::table)
                .values(&new_username)
                .execute(&mut conn)
                .await?;
        }
        
        // Create a history record of the username change
        let history_record = NewUsernameHistory {
            profile_id: profile.id,
            old_username: event.old_username.clone(),
            new_username: event.new_username.clone(),
            changed_at: now,
        };
        
        diesel::insert_into(schema::username_history::table)
            .values(&history_record)
            .execute(&mut conn)
            .await?;
            
        info!("Processed username updated: {} -> {}", event.old_username, event.new_username);
        Ok(())
    }
    
    /// Process a username registered event
    async fn process_username_registered(&self, event: &UsernameRegisteredEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        info!("Processing UsernameRegisteredEvent: {:?}", event);
        
        // Find the profile by profile_id
        let profile_result = schema::profiles::table
            .filter(schema::profiles::profile_id.eq(&event.profile_id))
            .first::<crate::models::profile::Profile>(&mut conn)
            .await;
        
        match profile_result {
            Ok(profile) => {
                info!("Found profile with ID: {} for username: {}", profile.id, event.username);
                
                // Update the profile table's username column (for backward compatibility)
                diesel::update(schema::profiles::table.find(profile.id))
                    .set(schema::profiles::username.eq(&event.username))
                    .execute(&mut conn)
                    .await?;
                    
                // Check if the username already exists in the usernames table
                let username_exists = schema::usernames::table
                    .filter(schema::usernames::profile_id.eq(profile.id))
                    .filter(schema::usernames::username.eq(&event.username))
                    .first::<crate::models::username::Username>(&mut conn)
                    .await.is_ok();
                
                // Use current time instead of blockchain epoch
                let now = Utc::now().naive_utc();
                info!("Using current timestamp for username registration: {}", now);
                    
                // Only insert if it doesn't exist
                if !username_exists {
                    info!("Username doesn't exist in the usernames table, inserting new record");
                    
                    let new_username = NewUsername {
                        profile_id: profile.id,
                        username: event.username.clone(),
                        registered_at: now,
                        updated_at: now,
                    };
                    
                    // Insert the username
                    let result = diesel::insert_into(schema::usernames::table)
                        .values(&new_username)
                        .execute(&mut conn)
                        .await;
                        
                    match result {
                        Ok(_) => info!("Successfully inserted username record"),
                        Err(e) => error!("Failed to insert username record: {}", e)
                    }
                    
                    // Verify the username was inserted correctly
                    match schema::usernames::table
                        .filter(schema::usernames::profile_id.eq(profile.id))
                        .filter(schema::usernames::username.eq(&event.username))
                        .first::<crate::models::username::Username>(&mut conn)
                        .await {
                        Ok(username) => info!("Verified username record exists: id={}, username={}", username.id, username.username),
                        Err(e) => error!("Failed to verify username record: {}", e)
                    }
                } else {
                    info!("Username already exists in the usernames table, skipping insertion");
                }
            },
            Err(_) => {
                // Profile doesn't exist yet, likely because events are processed out of order
                warn!("Profile not found for profile_id: {}. UsernameRegisteredEvent will be handled when profile is created", event.profile_id);
                
                // Try to find a profile with a matching username
                let profile_by_username = schema::profiles::table
                    .filter(schema::profiles::username.eq(&event.username))
                    .first::<crate::models::profile::Profile>(&mut conn)
                    .await;
                
                if let Ok(profile) = profile_by_username {
                    info!("Found profile with username: {}, using that instead", event.username);
                    
                    // Create a new username record
                    let now = Utc::now().naive_utc();
                    let new_username = NewUsername {
                        profile_id: profile.id,
                        username: event.username.clone(),
                        registered_at: now,
                        updated_at: now,
                    };
                    
                    // Try to insert the username for this profile
                    match diesel::insert_into(schema::usernames::table)
                        .values(&new_username)
                        .on_conflict_do_nothing()
                        .execute(&mut conn)
                        .await {
                        Ok(_) => info!("Created username record for existing profile with matching username"),
                        Err(e) => error!("Failed to create username record: {}", e)
                    }
                } else {
                    warn!("No profile found with username {}. Event will be processed when profile is created", event.username);
                }
            }
        }
        
        info!("Processed username registered: {} for profile {}", event.username, event.profile_id);
        Ok(())
    }
    
    /// Process a tokens vested event
    async fn process_tokens_vested(&self, event: &TokensVestedEvent, transaction_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!(
            "Processing TokensVestedEvent: wallet_id={}, owner={}, amount={}",
            event.wallet_id, event.owner, event.total_amount
        );

        // Convert event to database models
        let (new_wallet, new_event) = event.into_models(transaction_id.to_string());

        // Check if wallet already exists to preserve claimed_amount
        let existing_wallet_result = schema::vesting_wallets::table
            .filter(schema::vesting_wallets::wallet_id.eq(&event.wallet_id))
            .first::<VestingWallet>(&mut conn)
            .await;

        let (claimed_amount, remaining_balance) = match existing_wallet_result {
            Ok(existing_wallet) => {
                // Wallet exists - preserve claimed_amount and recalculate remaining_balance
                let new_total = new_wallet.total_amount;
                let preserved_claimed = existing_wallet.claimed_amount;
                let recalculated_remaining = new_total - preserved_claimed;

                // Validate the recalculation
                if recalculated_remaining < 0 {
                    return Err(anyhow!(
                        "Invalid vesting state: new total_amount ({}) is less than existing claimed_amount ({}) for wallet {}",
                        new_total,
                        preserved_claimed,
                        event.wallet_id
                    ));
                }

                info!(
                    "Updating existing wallet {}: preserving claimed_amount={}, recalculating remaining_balance={} (new_total={})",
                    event.wallet_id, preserved_claimed, recalculated_remaining, new_total
                );

                (preserved_claimed, recalculated_remaining)
            }
            Err(diesel::NotFound) => {
                // New wallet - use default values
                info!(
                    "Creating new wallet {}: claimed_amount=0, remaining_balance={}",
                    event.wallet_id, new_wallet.total_amount
                );
                (new_wallet.claimed_amount, new_wallet.remaining_balance)
            }
            Err(e) => {
                return Err(anyhow!("Failed to check existing wallet {}: {}", event.wallet_id, e));
            }
        };

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
                schema::vesting_wallets::claimed_amount.eq(claimed_amount),
                schema::vesting_wallets::remaining_balance.eq(remaining_balance),
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
    async fn process_tokens_claimed(&self, event: &TokensClaimedEvent, transaction_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;

        info!("Processing TokensClaimedEvent: wallet_id={}, owner={}, claimed_amount={}, remaining_balance={}", 
              event.wallet_id, event.owner, event.claimed_amount, event.remaining_balance);

        // Fetch the existing wallet to get total_amount for calculating cumulative claimed_amount
        let wallet = schema::vesting_wallets::table
            .filter(schema::vesting_wallets::wallet_id.eq(&event.wallet_id))
            .first::<VestingWallet>(&mut conn)
            .await
            .map_err(|e| {
                anyhow!("Failed to fetch vesting wallet {}: {}", event.wallet_id, e)
            })?;

        // Calculate cumulative claimed_amount from total_amount and remaining_balance
        // This ensures the invariant: claimed_amount + remaining_balance = total_amount
        let total_claimed_amount = wallet.total_amount - (event.remaining_balance as i64);
        
        // Validate the calculation
        if total_claimed_amount < 0 {
            return Err(anyhow!(
                "Invalid vesting state: remaining_balance ({}) exceeds total_amount ({}) for wallet {}",
                event.remaining_balance,
                wallet.total_amount,
                event.wallet_id
            ));
        }

        info!(
            "Calculated cumulative claimed_amount: {} (total_amount: {}, remaining_balance: {})",
            total_claimed_amount, wallet.total_amount, event.remaining_balance
        );

        // Create wallet update with cumulative claimed_amount
        let wallet_update = UpdateVestingWallet::from_tokens_claimed(
            total_claimed_amount as u64,
            event.remaining_balance,
            Some(event.claimed_at),
        );

        // Convert event to database event model (for event history)
        let new_event = event.into_models(transaction_id.to_string());

        // Update the vesting wallet with cumulative claimed amount and remaining balance
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
            "✅ Successfully processed tokens claimed: wallet_id={}, incremental_amount={}, cumulative_claimed={}, remaining_balance={}",
            event.wallet_id, event.claimed_amount, total_claimed_amount, event.remaining_balance
        );
        Ok(())
    }
    
    // Private data update functionality has been removed
    // All sensitive fields are now stored directly in the profile
    
    /// Process a profile follow event
    async fn process_profile_follow(&self, event: &ProfileFollowEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Check if relationship already exists to avoid duplicates
        let exists = diesel::select(diesel::dsl::exists(
            schema::social_graph_relationships::table
                .filter(schema::social_graph_relationships::follower_address.eq(&event.follower_id))
                .filter(schema::social_graph_relationships::following_address.eq(&event.following_id))
        ))
        .get_result::<bool>(&mut conn)
        .await?;
        
        if exists {
            info!("Profile follow relationship already exists: {} -> {}", event.follower_id, event.following_id);
            return Ok(());
        }
        
        // Create new follow relationship using existing social_graph_relationships table
        let relationship = crate::models::social_graph::NewSocialGraphRelationship {
            follower_address: event.follower_id.clone(),
            following_address: event.following_id.clone(),
            created_at: if let Some(timestamp) = event.followed_at {
                chrono::DateTime::from_timestamp(timestamp as i64, 0)
                    .unwrap_or(chrono::Utc::now())
                    .naive_utc()
            } else {
                chrono::Utc::now().naive_utc()
            },
        };
        
        // Insert the follow relationship
        diesel::insert_into(schema::social_graph_relationships::table)
            .values(&relationship)
            .execute(&mut conn)
            .await?;
            
        // Log the event
        let event_log = crate::models::social_graph::NewSocialGraphEvent {
            event_type: "profile_follow".to_string(),
            follower_address: event.follower_id.clone(),
            following_address: event.following_id.clone(),
            created_at: relationship.created_at,
            event_id: None, // ProfileFollowEvent doesn't have blockchain event ID
            raw_event_data: Some(serde_json::to_value(event)?),
        };
        
        diesel::insert_into(schema::social_graph_events::table)
            .values(&event_log)
            .execute(&mut conn)
            .await?;
            
        // Count updates are now handled automatically by database triggers
        // when the relationship is inserted into social_graph_relationships
            
        info!("Processed profile follow: {} -> {}", event.follower_id, event.following_id);
        Ok(())
    }
    
    /// Process a platform created event
    async fn process_platform_created(&self, event: &PlatformCreatedEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Convert event to database model
        let new_platform = event.into_model()?;
        
        // Insert the platform
        diesel::insert_into(schema::platforms::table)
            .values(&new_platform)
            .on_conflict(schema::platforms::id)
            .do_update()
            .set(&new_platform)
            .execute(&mut conn)
            .await?;
            
        info!("Processed platform created: {}", event.platform_id);
        Ok(())
    }
    
    /// Process a profile joined platform event
    async fn process_profile_joined_platform(&self, event: &ProfileJoinedPlatformEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Create join record
        let joined_at = Utc::now(); // Use current time if event doesn't provide it
        let link = NewProfilePlatformLink {
            profile_id: event.profile_id.clone(),
            platform_id: event.platform_id.clone(),
            joined_at,
            last_active_at: Some(joined_at),
        };
        
        // Insert the platform join
        diesel::insert_into(schema::profile_platform_links::table)
            .values(&link)
            .on_conflict((schema::profile_platform_links::profile_id, schema::profile_platform_links::platform_id))
            .do_update()
            .set(&link)
            .execute(&mut conn)
            .await?;
            
        // Update platform user counts
        diesel::update(schema::platforms::table.find(&event.platform_id))
            .set((
                schema::platforms::total_users_count.eq(schema::platforms::total_users_count + 1),
                schema::platforms::active_users_count.eq(schema::platforms::active_users_count + 1),
                schema::platforms::last_activity_at.eq(joined_at),
            ))
            .execute(&mut conn)
            .await?;
            
        // Update profile platforms joined count
        diesel::update(schema::profiles::table.find(&event.profile_id))
            .set((
                schema::profiles::platforms_joined.eq(schema::profiles::platforms_joined + 1),
                schema::profiles::last_activity_at.eq(joined_at),
            ))
            .execute(&mut conn)
            .await?;
            
        // Update platform daily statistics
        self.update_platform_daily_stats(&event.platform_id, |stats| {
            stats.new_users_count += 1;
            stats.active_users_count += 1;
        }).await?;
        
        info!("Processed profile joined platform: {} -> {}", event.profile_id, event.platform_id);
        Ok(())
    }
    
    /// Process a content created event
    async fn process_content_created(&self, event: &ContentCreatedEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Convert event to database model
        let new_content = event.into_model()?;
        
        // Insert the content
        diesel::insert_into(schema::content::table)
            .values(&new_content)
            .on_conflict(schema::content::id)
            .do_update()
            .set(&new_content)
            .execute(&mut conn)
            .await?;
            
        // Update profile content count
        diesel::update(schema::profiles::table.find(&event.creator_id))
            .set((
                schema::profiles::content_count.eq(schema::profiles::content_count + 1),
                schema::profiles::last_activity_at.eq(new_content.created_at),
            ))
            .execute(&mut conn)
            .await?;
            
        // Update platform content count
        diesel::update(schema::platforms::table.find(&event.platform_id))
            .set((
                schema::platforms::content_count.eq(schema::platforms::content_count + 1),
                schema::platforms::last_activity_at.eq(new_content.created_at),
            ))
            .execute(&mut conn)
            .await?;
            
        // If this is a comment/reply, increment the comment count on the parent
        if let Some(parent_id) = &event.parent_id {
            diesel::update(schema::content::table.find(parent_id))
                .set(schema::content::comment_count.eq(schema::content::comment_count + 1))
                .execute(&mut conn)
                .await?;
        }
            
        // Update daily statistics
        self.update_daily_stats(|stats| {
            stats.new_content_count += 1;
        }).await?;
        
        // Update platform daily statistics
        self.update_platform_daily_stats(&event.platform_id, |stats| {
            stats.content_created_count += 1;
        }).await?;
        
        info!("Processed content created: {}", event.content_id);
        Ok(())
    }
    
    /// Process a content interaction event
    async fn process_content_interaction(&self, event: &ContentInteractionEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Convert event to database model
        let new_interaction = event.into_model()?;
        
        // Insert the interaction
        diesel::insert_into(schema::content_interactions::table)
            .values(&new_interaction)
            .on_conflict((
                schema::content_interactions::profile_id, 
                schema::content_interactions::content_id,
                schema::content_interactions::interaction_type
            ))
            .do_update()
            .set(&new_interaction)
            .execute(&mut conn)
            .await?;
            
        // Update content metrics based on interaction type
        match event.interaction_type.as_str() {
            "like" => {
                diesel::update(schema::content::table.find(&event.content_id))
                    .set(schema::content::like_count.eq(schema::content::like_count + 1))
                    .execute(&mut conn)
                    .await?;
            },
            "view" => {
                diesel::update(schema::content::table.find(&event.content_id))
                    .set(schema::content::view_count.eq(schema::content::view_count + 1))
                    .execute(&mut conn)
                    .await?;
            },
            "share" => {
                diesel::update(schema::content::table.find(&event.content_id))
                    .set(schema::content::share_count.eq(schema::content::share_count + 1))
                    .execute(&mut conn)
                    .await?;
            },
            _ => {}
        }
            
        // Update user last activity
        diesel::update(schema::profiles::table.find(&event.profile_id))
            .set(schema::profiles::last_activity_at.eq(new_interaction.created_at))
            .execute(&mut conn)
            .await?;
            
        // Get platform ID from content
        let content = schema::content::table
            .find(&event.content_id)
            .select(schema::content::platform_id)
            .first::<String>(&mut conn)
            .await?;
            
        // Update daily statistics
        self.update_daily_stats(|stats| {
            stats.total_interactions_count += 1;
        }).await?;
        
        // Update platform daily statistics
        self.update_platform_daily_stats(&content, |stats| {
            stats.total_interactions_count += 1;
        }).await?;
        
        info!("Processed content interaction: {} -> {}: {}", 
            event.profile_id, event.content_id, event.interaction_type);
        Ok(())
    }
    
    /// Process an entity blocked event
    async fn process_entity_blocked(&self, event: &EntityBlockedEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Convert event to database model
        let new_block = event.into_model()?;
        
        // Insert the block
        diesel::insert_into(schema::blocks::table)
            .values(&new_block)
            .on_conflict((schema::blocks::blocker_id, schema::blocks::blocked_id))
            .do_update()
            .set(&new_block)
            .execute(&mut conn)
            .await?;
            
        info!("Processed entity blocked: {} blocked {}", event.blocker_id, event.blocked_id);
        Ok(())
    }
    
    /// Process an IP registration event
    async fn process_ip_registered(&self, event: &IPRegisteredEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Convert event to database model
        let new_ip = event.into_model(None, None)?;
        
        // Insert the IP registration
        diesel::insert_into(schema::intellectual_property::table)
            .values(&new_ip)
            .on_conflict(schema::intellectual_property::id)
            .do_update()
            .set(&new_ip)
            .execute(&mut conn)
            .await?;
            
        // Update daily statistics
        self.update_daily_stats(|stats| {
            stats.new_ip_registrations_count += 1;
        }).await?;
        
        // If this IP is for content, mark the content as having IP
        diesel::update(schema::content::table.find(&event.ip_id))
            .set(schema::content::has_ip_registered.eq(true))
            .execute(&mut conn)
            .await
            .ok(); // Ignore errors, content might not exist
            
        info!("Processed IP registered: {}", event.ip_id);
        Ok(())
    }
    
    /// Process a license granted event
    async fn process_license_granted(&self, event: &LicenseGrantedEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Convert event to database model
        let new_license = event.into_model(None)?;
        
        // Insert the license
        diesel::insert_into(schema::ip_licenses::table)
            .values(&new_license)
            .on_conflict(schema::ip_licenses::id)
            .do_update()
            .set(&new_license)
            .execute(&mut conn)
            .await?;
            
        // Update IP metrics
        diesel::update(schema::intellectual_property::table.find(&event.ip_id))
            .set((
                schema::intellectual_property::total_licenses_count.eq(
                    schema::intellectual_property::total_licenses_count + 1
                ),
                schema::intellectual_property::active_licenses_count.eq(
                    schema::intellectual_property::active_licenses_count + 1
                ),
                schema::intellectual_property::total_revenue.eq(
                    schema::intellectual_property::total_revenue + event.payment_amount as i64
                ),
            ))
            .execute(&mut conn)
            .await?;
            
        // Update daily statistics
        self.update_daily_stats(|stats| {
            stats.new_licenses_count += 1;
        }).await?;
        
        info!("Processed license granted: {} for IP {}", event.license_id, event.ip_id);
        Ok(())
    }
    
    /// Process a fee distribution event
    async fn process_fee_distribution(&self, event: &FeesDistributedEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        
        // Convert event to database model
        let new_distribution = event.into_model()?;
        
        // Insert the fee distribution
        let result = diesel::insert_into(schema::fee_distributions::table)
            .values(&new_distribution)
            .returning(schema::fee_distributions::id)
            .get_result::<i32>(&mut conn)
            .await?;
            
        let distribution_id = result;
            
        // Update daily statistics
        self.update_daily_stats(|stats| {
            stats.total_fees_distributed += event.total_fee_amount as i64;
        }).await?;
        
        info!("Processed fee distribution: {} for model {}", distribution_id, event.fee_model_id);
        Ok(())
    }
    
    /// Update daily statistics
    async fn update_daily_stats<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut NewDailyStatistics),
    {
        let mut conn = self.get_connection().await?;
        let today = Utc::now().date_naive();
        
        // Try to load existing stats for today
        let existing_stats = schema::daily_statistics::table
            .find(today)
            .first::<crate::models::statistics::DailyStatistics>(&mut conn)
            .await
            .ok();
            
        // Create new stats or update existing
        let mut stats = match existing_stats {
            Some(existing) => NewDailyStatistics {
                date: existing.date,
                new_profiles_count: existing.new_profiles_count,
                active_profiles_count: existing.active_profiles_count,
                new_content_count: existing.new_content_count,
                total_interactions_count: existing.total_interactions_count,
                new_ip_registrations_count: existing.new_ip_registrations_count,
                new_licenses_count: existing.new_licenses_count,
                total_fees_distributed: existing.total_fees_distributed,
            },
            None => NewDailyStatistics {
                date: today,
                new_profiles_count: 0,
                active_profiles_count: 0,
                new_content_count: 0,
                total_interactions_count: 0,
                new_ip_registrations_count: 0,
                new_licenses_count: 0,
                total_fees_distributed: 0,
            },
        };
        
        // Apply updates to stats
        updater(&mut stats);
        
        // Insert or update stats
        diesel::insert_into(schema::daily_statistics::table)
            .values(&stats)
            .on_conflict(schema::daily_statistics::date)
            .do_update()
            .set(&stats)
            .execute(&mut conn)
            .await?;
            
        Ok(())
    }
    
    /// Update platform daily statistics
    async fn update_platform_daily_stats<F>(&self, platform_id: &str, updater: F) -> Result<()>
    where
        F: FnOnce(&mut NewPlatformDailyStatistics),
    {
        let mut conn = self.get_connection().await?;
        let today = Utc::now().date_naive();
        
        // Try to load existing stats for today and platform
        let existing_stats = schema::platform_daily_statistics::table
            .find((platform_id, today))
            .first::<crate::models::statistics::PlatformDailyStatistics>(&mut conn)
            .await
            .ok();
            
        // Create new stats or update existing
        let mut stats = match existing_stats {
            Some(existing) => NewPlatformDailyStatistics {
                platform_id: existing.platform_id,
                date: existing.date,
                active_users_count: existing.active_users_count,
                new_users_count: existing.new_users_count,
                content_created_count: existing.content_created_count,
                total_interactions_count: existing.total_interactions_count,
            },
            None => NewPlatformDailyStatistics {
                platform_id: platform_id.to_string(),
                date: today,
                active_users_count: 0,
                new_users_count: 0,
                content_created_count: 0,
                total_interactions_count: 0,
            },
        };
        
        // Apply updates to stats
        updater(&mut stats);
        
        // Insert or update stats
        diesel::insert_into(schema::platform_daily_statistics::table)
            .values(&stats)
            .on_conflict((schema::platform_daily_statistics::platform_id, schema::platform_daily_statistics::date))
            .do_update()
            .set(&stats)
            .execute(&mut conn)
            .await?;
            
        Ok(())
    }

    /// Process a platform blocked profile event
    async fn process_platform_blocked_profile(&self, event: &PlatformBlockedProfileEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now().naive_utc();
        
        // Create new blocked profile record
        let new_blocked_profile = NewPlatformBlockedProfile {
            platform_id: event.platform_id.clone(),
            profile_id: event.profile_id.clone(),
            blocked_by: event.blocked_by.clone(),
            created_at: now,
        };
        
        // Insert the blocked profile record
        diesel::insert_into(schema::platform_blocked_profiles::table)
            .values(&new_blocked_profile)
            .execute(&mut conn)
            .await?;
            
        info!("Processed platform blocked profile: platform={}, profile={}", 
              event.platform_id, event.profile_id);
        Ok(())
    }
    
    /// Process a platform unblocked profile event
    async fn process_platform_unblocked_profile(&self, event: &PlatformUnblockedProfileEvent) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now().naive_utc();
        
        // Delete the blocked profile record
        diesel::delete(schema::platform_blocked_profiles::table)
            .filter(schema::platform_blocked_profiles::platform_id.eq(&event.platform_id))
            .filter(schema::platform_blocked_profiles::profile_id.eq(&event.profile_id))
            .execute(&mut conn)
            .await?;
            
        info!("Processed platform unblocked profile: platform={}, profile={}", 
              event.platform_id, event.profile_id);
        Ok(())
    }
    
    /// Process a user joined platform event
    async fn process_user_joined_platform(&self, event: &UserJoinedPlatformEvent, event_id: Option<String>) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now().naive_utc();
        
        // Create a profile event for platform join
        let platform_join_event = crate::events::profile_event_types::PlatformJoinedEvent {
            profile_id: event.profile_id.clone(),
            platform_id: event.platform_id.clone(),
            timestamp: Utc::now().timestamp() as u64,
        };
        
        let profile_event = crate::models::profile_events::NewProfileEvent::from_platform_joined(
            &platform_join_event,
            event_id
        );
        
        // Insert the profile event record
        diesel::insert_into(schema::profile_events::table)
            .values(&profile_event)
            .execute(&mut conn)
            .await?;
            
        info!("Processed user joined platform: platform={}, profile={}", 
              event.platform_id, event.profile_id);
        Ok(())
    }
    
    /// Process a user left platform event
    async fn process_user_left_platform(&self, event: &UserLeftPlatformEvent, event_id: Option<String>) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now().naive_utc();
        
        // Create a profile event for platform leave
        let platform_left_event = crate::events::profile_event_types::PlatformLeftEvent {
            profile_id: event.profile_id.clone(),
            platform_id: event.platform_id.clone(),
            timestamp: Utc::now().timestamp() as u64,
        };
        
        let profile_event = crate::models::profile_events::NewProfileEvent::from_platform_left(
            &platform_left_event,
            event_id
        );
        
        // Insert the profile event record
        diesel::insert_into(schema::profile_events::table)
            .values(&profile_event)
            .execute(&mut conn)
            .await?;
            
        // Delete the platform membership record
        let deleted_count = diesel::delete(schema::platform_memberships::table)
            .filter(schema::platform_memberships::platform_id.eq(&event.platform_id))
            .filter(schema::platform_memberships::profile_id.eq(&event.profile_id))
            .execute(&mut conn)
            .await?;
            
        if deleted_count > 0 {
            info!("Deleted platform membership: platform={}, profile={}", 
                  event.platform_id, event.profile_id);
        } else {
            warn!("No platform membership found to delete: platform={}, profile={}", 
                  event.platform_id, event.profile_id);
        }
        
        info!("Processed user left platform: platform={}, profile={}", 
              event.platform_id, event.profile_id);
        Ok(())
    }
}

#[async_trait]
impl Worker for SocialIndexerWorker {
    type Result = ();

    async fn process_checkpoint(&self, checkpoint: &CheckpointData) -> Result<()> {
        let checkpoint_seq = checkpoint.checkpoint_summary.sequence_number;
        info!("Processing checkpoint: {}", checkpoint_seq);
        
        // Process each transaction in the checkpoint
        for transaction in &checkpoint.transactions {
            // Process each event in the transaction
            for event in &transaction.events {
                let type_str = &event.type_;
                
                // Log all events for debugging with the EXACT type string
                info!("🚨 WORKER: Processing event of type: {}", type_str);
                info!("📊 WORKER: Raw event data: {}", serde_json::to_string_pretty(event).unwrap_or_default());
                
                // Process events by module
                match type_str {
                    // Profile events
                    t if t.starts_with(MODULE_PREFIX_PROFILE) && t.ends_with("ProfileCreatedEvent") => {
                        // Log the raw event for better debugging
                        info!("Raw ProfileCreatedEvent data: {}", serde_json::to_string_pretty(&event).unwrap_or_default());
                        
                        match parse_event::<ProfileCreatedEvent>(event) {
                            Ok(event) => {
                                info!("Successfully parsed ProfileCreatedEvent with fields:");
                                info!("  profile_id: {}", event.profile_id);
                                info!("  owner_address: {}", event.owner_address);
                                info!("  username: {:?}", event.username);
                                info!("  display_name: {}", event.display_name);
                                info!("  bio: {:?}", event.bio);
                                info!("  profile_photo: {:?}", event.profile_photo);
                                info!("  cover_photo: {:?}", event.cover_photo);
                                
                                if let Err(e) = self.process_profile_created(&event).await {
                                    error!("Failed to process ProfileCreatedEvent: {}", e);
                                }
                            },
                            Err(e) => {
                                error!("Failed to parse ProfileCreatedEvent: {}", e);
                                // Log full event for debugging
                                error!("Event data: {}", serde_json::to_string_pretty(event).unwrap_or_default());
                            }
                        }
                    },
                    t if t.starts_with(MODULE_PREFIX_PROFILE) && t.ends_with("ProfileUpdatedEvent") => {
                        // Log the raw event for better debugging
                        info!("Raw ProfileUpdatedEvent data: {}", serde_json::to_string_pretty(&event).unwrap_or_default());
                        
                        match parse_event::<ProfileUpdatedEvent>(event) {
                            Ok(event) => {
                                info!("Successfully parsed ProfileUpdatedEvent with fields:");
                                info!("  profile_id: {}", event.profile_id);
                                info!("  owner_address: {}", event.owner_address);
                                info!("  username: {:?}", event.username);
                                info!("  display_name: {:?}", event.display_name);
                                info!("  bio: {:?}", event.bio);
                                info!("  profile_photo: {:?}", event.profile_photo);
                                info!("  cover_photo: {:?}", event.cover_photo);
                                
                                if let Err(e) = self.process_profile_updated(&event).await {
                                    error!("Failed to process ProfileUpdatedEvent: {}", e);
                                }
                            },
                            Err(e) => {
                                error!("Failed to parse ProfileUpdatedEvent: {}", e);
                                // Log full event for debugging
                                error!("Event data: {}", serde_json::to_string_pretty(event).unwrap_or_default());
                            }
                        }
                    },
                    t if t.starts_with(MODULE_PREFIX_PROFILE) && t.ends_with("UsernameUpdatedEvent") => {
                        if let Ok(event) = parse_event::<UsernameUpdatedEvent>(event) {
                            if let Err(e) = self.process_username_updated(&event).await {
                                error!("Failed to process UsernameUpdatedEvent: {}", e);
                            }
                        }
                    },
                    t if t.starts_with(MODULE_PREFIX_PROFILE) && t.ends_with("UsernameRegisteredEvent") => {
                        info!("Found a UsernameRegisteredEvent: {}", serde_json::to_string_pretty(event).unwrap_or_default());
                        match parse_event::<UsernameRegisteredEvent>(event) {
                            Ok(event) => {
                                info!("Successfully parsed UsernameRegisteredEvent: profile_id={}, username={}", 
                                       event.profile_id, event.username);
                                
                                if let Err(e) = self.process_username_registered(&event).await {
                                    error!("Failed to process UsernameRegisteredEvent: {}", e);
                                }
                            },
                            Err(e) => {
                                error!("Failed to parse UsernameRegisteredEvent: {}", e);
                                // Dump the full event for debugging
                                error!("Raw event data: {}", serde_json::to_string_pretty(event).unwrap_or_default());
                            }
                        }
                    },
                    t if t.starts_with(MODULE_PREFIX_PROFILE) && t.ends_with("TokensVestedEvent") => {
                        info!(
                            "Tokens vested event detected with data: {}",
                            serde_json::to_string_pretty(&event).unwrap_or_default()
                        );

                        match parse_event::<TokensVestedEvent>(event) {
                            Ok(vesting_event) => {
                                info!(
                                    "Successfully parsed tokens vested event: {:?}",
                                    vesting_event
                                );
                                let tx_id = event.tx_digest.clone().unwrap_or_default();
                                if let Err(e) = self
                                    .process_tokens_vested(&vesting_event, &tx_id)
                                    .await
                                {
                                    error!("Failed to process tokens vested event: {}", e);
                                }
                            }
                            Err(e) => {
                                error!("Failed to deserialize tokens vested event: {}", e);
                            }
                        }
                    },
                    t if t.starts_with(MODULE_PREFIX_PROFILE) && t.ends_with("TokensClaimedEvent") => {
                        info!(
                            "Tokens claimed event detected with data: {}",
                            serde_json::to_string_pretty(&event).unwrap_or_default()
                        );

                        match parse_event::<TokensClaimedEvent>(event) {
                            Ok(claim_event) => {
                                info!(
                                    "Successfully parsed tokens claimed event: {:?}",
                                    claim_event
                                );
                                let tx_id = event.tx_digest.clone().unwrap_or_default();
                                if let Err(e) = self
                                    .process_tokens_claimed(&claim_event, &tx_id)
                                    .await
                                {
                                    error!("Failed to process tokens claimed event: {}", e);
                                }
                            }
                            Err(e) => {
                                error!("Failed to deserialize tokens claimed event: {}", e);
                            }
                        }
                    },
                    // Private data update functionality has been removed
                    // All sensitive fields are now stored directly in the profile
                    t if t.starts_with(MODULE_PREFIX_SOCIAL_GRAPH) && t.ends_with("ProfileFollowEvent") => {
                        if let Ok(event) = parse_event::<ProfileFollowEvent>(event) {
                            if let Err(e) = self.process_profile_follow(&event).await {
                                error!("Failed to process ProfileFollowEvent: {}", e);
                            }
                        }
                    },
                    
                    // Social Graph events from social_graph module
                    t if t.starts_with(MODULE_PREFIX_SOCIAL_GRAPH) && t.ends_with("FollowEvent") => {
                        info!("🚨 WORKER: FollowEvent detected - should be handled by SocialGraphEventHandler");
                        info!("🚨 WORKER: Event type: {}", type_str);
                        info!("🚨 WORKER: Raw event data: {}", serde_json::to_string_pretty(event).unwrap_or_default());
                    },
                    
                    t if t.starts_with(MODULE_PREFIX_SOCIAL_GRAPH) && t.ends_with("UnfollowEvent") => {
                        info!("🚨 WORKER: UnfollowEvent detected - should be handled by SocialGraphEventHandler");
                        info!("🚨 WORKER: Event type: {}", type_str);
                        info!("🚨 WORKER: Raw event data: {}", serde_json::to_string_pretty(event).unwrap_or_default());
                    },
                    
                    // Platform events
                    t if t.starts_with(MODULE_PREFIX_PLATFORM) => {
                        match type_str {
                            t if t.ends_with("PlatformBlockedProfileEvent") => {
                                match parse_event::<PlatformBlockedProfileEvent>(event) {
                                    Ok(event) => self.process_platform_blocked_profile(&event).await?,
                                    Err(e) => error!("Failed to parse PlatformBlockedProfileEvent: {}", e),
                                }
                            }
                            t if t.ends_with("PlatformUnblockedProfileEvent") => {
                                match parse_event::<PlatformUnblockedProfileEvent>(event) {
                                    Ok(event) => self.process_platform_unblocked_profile(&event).await?,
                                    Err(e) => error!("Failed to parse PlatformUnblockedProfileEvent: {}", e),
                                }
                            }
                            t if t.ends_with("UserJoinedPlatformEvent") => {
                                match parse_event::<UserJoinedPlatformEvent>(event) {
                                    Ok(parsed_event) => {
                                        // Extract event ID using EventID - look for appropriate fields
                                        let event_id = if let Some(tx_digest) = &event.tx_digest {
                                            // EventID includes both transaction digest and event sequence
                                            let event_id_struct = EventID {
                                                tx_digest: tx_digest.clone(),
                                                event_seq: event.event_num,
                                            };
                                            
                                            // Convert EventID to string representation
                                            Some(event_id_struct.to_string())
                                        } else {
                                            None
                                        };
                                        
                                        info!("Processing UserJoinedPlatformEvent with event_id: {:?}", event_id);
                                        self.process_user_joined_platform(&parsed_event, event_id).await?
                                    },
                                    Err(e) => error!("Failed to parse UserJoinedPlatformEvent: {}", e),
                                }
                            }
                            t if t.ends_with("UserLeftPlatformEvent") => {
                                match parse_event::<UserLeftPlatformEvent>(event) {
                                    Ok(parsed_event) => {
                                        // Extract event ID using EventID - look for appropriate fields
                                        let event_id = if let Some(tx_digest) = &event.tx_digest {
                                            // EventID includes both transaction digest and event sequence
                                            let event_id_struct = EventID {
                                                tx_digest: tx_digest.clone(),
                                                event_seq: event.event_num,
                                            };
                                            
                                            // Convert EventID to string representation
                                            Some(event_id_struct.to_string())
                                        } else {
                                            None
                                        };
                                        
                                        info!("Processing UserLeftPlatformEvent with event_id: {:?}", event_id);
                                        self.process_user_left_platform(&parsed_event, event_id).await?
                                    },
                                    Err(e) => error!("Failed to parse UserLeftPlatformEvent: {}", e),
                                }
                            }
                            _ => {
                                debug!("Unhandled platform event type: {}", type_str);
                            }
                        }
                    },
                    
                    // Post events - check for ::post:: module specifically
                    t if t.contains("::post::") && t.ends_with("PostCreatedEvent") => {
                        info!("🚨 WORKER: PostCreatedEvent detected - routing to post handler");
                        
                        // Extract transaction digest from event, with fallback
                        let tx_digest = event.tx_digest.clone().unwrap_or_else(|| {
                            warn!("No transaction digest found in event for PostCreatedEvent, using empty string");
                            String::new()
                        });
                        
                        info!("📝 WORKER: Processing PostCreatedEvent");
                        info!("📝 WORKER: Event type: {}", type_str);
                        info!("📝 WORKER: Transaction digest: {}", tx_digest);
                        info!("📝 WORKER: Event data: {}", serde_json::to_string_pretty(event).unwrap_or_default());
                        
                        // Use the handler to process post events
                        match crate::blockchain::handler::handle_event(&self.db, event, &tx_digest).await {
                            Ok(_) => {
                                info!("✅ WORKER: Successfully processed PostCreatedEvent");
                            }
                            Err(e) => {
                                error!("❌ WORKER: Failed to process PostCreatedEvent");
                                error!("❌ WORKER: Error: {}", e);
                                error!("❌ WORKER: Error chain: {:?}", e);
                                error!("❌ WORKER: Event type: {}", type_str);
                                error!("❌ WORKER: Transaction digest: {}", tx_digest);
                            }
                        }
                    },
                    // Content events
                    t if t.starts_with(MODULE_PREFIX_CONTENT) && t.ends_with("ContentCreatedEvent") => {
                        if let Ok(event) = parse_event::<ContentCreatedEvent>(event) {
                            if let Err(e) = self.process_content_created(&event).await {
                                error!("Failed to process ContentCreatedEvent: {}", e);
                            }
                        }
                    },
                    t if t.starts_with(MODULE_PREFIX_CONTENT) && t.ends_with("ContentInteractionEvent") => {
                        if let Ok(event) = parse_event::<ContentInteractionEvent>(event) {
                            if let Err(e) = self.process_content_interaction(&event).await {
                                error!("Failed to process ContentInteractionEvent: {}", e);
                            }
                        }
                    },
                    
                    // Block list events are now handled by block_list_handler.rs
                    // Note: UserBlockEvent is handled directly in blockchain/events.rs
                    // Handle only things not covered in blockchain/events.rs
                    t if t.starts_with(MODULE_PREFIX_BLOCK_LIST) && t.ends_with("EntityBlockedEvent") => {
                        if let Ok(event) = parse_event::<EntityBlockedEvent>(event) {
                            if let Err(e) = self.process_entity_blocked(&event).await {
                                error!("Failed to process EntityBlockedEvent: {}", e);
                            }
                        }
                    },
                    
                    // IP events
                    t if t.starts_with(MODULE_PREFIX_MYDATA) && t.ends_with("IPRegisteredEvent") => {
                        if let Ok(event) = parse_event::<IPRegisteredEvent>(event) {
                            if let Err(e) = self.process_ip_registered(&event).await {
                                error!("Failed to process IPRegisteredEvent: {}", e);
                            }
                        }
                    },
                    t if t.starts_with(MODULE_PREFIX_MYDATA) && t.ends_with("LicenseGrantedEvent") => {
                        if let Ok(event) = parse_event::<LicenseGrantedEvent>(event) {
                            if let Err(e) = self.process_license_granted(&event).await {
                                error!("Failed to process LicenseGrantedEvent: {}", e);
                            }
                        }
                    },
                    
                    // Fee distribution events
                    t if t.starts_with(MODULE_PREFIX_FEE_DISTRIBUTION) && t.ends_with("FeesDistributedEvent") => {
                        if let Ok(event) = parse_event::<FeesDistributedEvent>(event) {
                            if let Err(e) = self.process_fee_distribution(&event).await {
                                error!("Failed to process FeesDistributedEvent: {}", e);
                            }
                        }
                    },
                    
                    // Ignore other events
                    _ => {}
                }
            }
        }
        
        // Update worker progress
        self.update_progress(checkpoint_seq).await?;
        
        info!("Processed checkpoint: {}", checkpoint_seq);
        Ok(())
    }
}
                    // SPoT events (social_proof_of_truth)
                    t if t.ends_with("SpotBetPlacedEvent") => {
                        // Parse and persist bet; ensure record exists
                        match parse_event::<SpotBetPlacedEvt>(event) {
                            Ok(parsed) => {
                                let event_id = if let Some(tx_digest) = &event.tx_digest {
                                    let event_id_struct = EventID { tx_digest: tx_digest.clone(), event_seq: event.event_num };
                                    Some(event_id_struct.to_string())
                                } else { None };

                                let tx = event.tx_digest.clone().unwrap_or_default();
                                let ts_secs: i64 = (checkpoint.checkpoint_summary.timestamp_ms as i64) / 1000;

                                let bet: NewSpotBet = parsed.into_bet_model(ts_secs as u64, tx.clone())?;

                                let mut conn = self.get_connection().await?;

                                // Upsert record if missing
                                let now_ts = chrono::Utc::now().naive_utc();
                                let betting_options_json = serde_json::json!(["Yes", "No"]);
                                let option_escrow_json = serde_json::json!({});
                                
                                // Insert record if not exists
                                diesel::sql_query("INSERT INTO spot_records (post_id, status, outcome, amm_split_bps_used, betting_options, option_escrow, resolution_window_epochs, max_resolution_window_epochs, created_epoch, last_resolution_epoch, version, created_at, updated_at, transaction_id) \
                                                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW(), $12) \
                                                   ON CONFLICT (post_id) DO NOTHING")
                                    .bind::<diesel::sql_types::Text, _>(&parsed.post_id)
                                    .bind::<diesel::sql_types::SmallInt, _>(&1i16) // STATUS_OPEN
                                    .bind::<diesel::sql_types::Nullable<diesel::sql_types::SmallInt>, _>(&None::<i16>)
                                    .bind::<diesel::sql_types::Integer, _>(&3000i32)
                                    .bind::<diesel::sql_types::Nullable<diesel::sql_types::Jsonb>, _>(&Some(betting_options_json))
                                    .bind::<diesel::sql_types::Nullable<diesel::sql_types::Jsonb>, _>(&Some(option_escrow_json))
                                    .bind::<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, _>(&None::<i64>)
                                    .bind::<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, _>(&None::<i64>)
                                    .bind::<diesel::sql_types::BigInt, _>(&(ts_secs as i64))
                                    .bind::<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, _>(&None::<i64>)
                                    .bind::<diesel::sql_types::BigInt, _>(&1i64)
                                    .bind::<diesel::sql_types::Text, _>(&tx)
                                    .execute(&mut conn)
                                    .await?;

                                // Insert bet row
                                diesel::insert_into(schema::spot_bets::table)
                                    .values(&bet)
                                    .execute(&mut conn)
                                    .await?;

                                // Update escrow aggregates using JSONB option_escrow
                                if parsed.amount > 0 {
                                    let option_id_str = parsed.option_id.to_string();
                                    diesel::sql_query("UPDATE spot_records SET option_escrow = jsonb_set(
                                        COALESCE(option_escrow, '{}'::jsonb),
                                        ARRAY[$1],
                                        ((COALESCE((option_escrow->>$1)::bigint, 0) + $2)::text)::jsonb
                                    ), updated_at = NOW() WHERE post_id = $3")
                                        .bind::<diesel::sql_types::Text, _>(&option_id_str)
                                        .bind::<diesel::sql_types::BigInt, _>(&(parsed.amount as i64))
                                        .bind::<diesel::sql_types::Text, _>(&parsed.post_id)
                                        .execute(&mut conn).await?;
                                }

                                // Log event
                                if let Some(eid) = event_id {
                                    let json = serde_json::to_value(&parsed)?;
                                    diesel::insert_into(schema::spot_events::table)
                                        .values(&crate::models::NewSpotEventLog {
                                            event_type: "SpotBetPlacedEvent".to_string(),
                                            post_id: parsed.post_id.clone(),
                                            event_data: json,
                                            event_id: eid,
                                            created_at: chrono::Utc::now(),
                                        })
                                        .execute(&mut conn)
                                        .await?;
                                }

                                // Unified SPoT event row
                                let unified = crate::models::NewSocialProofOfTruthEvent {
                                    event_type: "SpotBetPlacedEvent".to_string(),
                                    post_id: parsed.post_id.clone(),
                                    user_address: Some(parsed.user.clone()),
                                    option_id: Some(parsed.option_id as i16),
                                    escrow_amount: Some(parsed.amount as i64),
                                    amm_amount: Some(0i64), // No AMM in current contract
                                    amount: Some(parsed.amount as i64),
                                    outcome: None,
                                    total_escrow: None,
                                    fee_taken: None,
                                    confidence_bps: None,
                                    timestamp_epoch: ts_secs,
                                    time: chrono::Utc::now(),
                                    event_id: event_id.clone(),
                                    transaction_id: event.tx_digest.clone(),
                                    raw_event: Some(serde_json::to_value(&parsed)?),
                                };
                                diesel::insert_into(schema::social_proof_of_truth::table)
                                    .values(&unified)
                                    .execute(&mut conn)
                                    .await?;
                            }
                            Err(e) => error!("Failed to parse SpotBetPlacedEvent: {}", e),
                        }
                    },

                    t if t.ends_with("SpotResolvedEvent") => {
                        match parse_event::<SpotResolvedEvt>(event) {
                            Ok(parsed) => {
                                let event_id = if let Some(tx_digest) = &event.tx_digest {
                                    let event_id_struct = EventID { tx_digest: tx_digest.clone(), event_seq: event.event_num };
                                    Some(event_id_struct.to_string())
                                } else { None };

                                let tx = event.tx_digest.clone().unwrap_or_default();
                                let ts_secs: i64 = (checkpoint.checkpoint_summary.timestamp_ms as i64) / 1000;

                                let resolution: NewSpotResolution = parsed.into_resolution_model(ts_secs as u64, tx.clone())?;
                                let mut conn = self.get_connection().await?;

                                // Update record status/outcome
                                diesel::sql_query("UPDATE spot_records SET status = $1, outcome = $2, last_resolution_epoch = $3, updated_at = NOW() WHERE post_id = $4")
                                    .bind::<diesel::sql_types::SmallInt, _>(3) // STATUS_RESOLVED
                                    .bind::<diesel::sql_types::Nullable<diesel::sql_types::SmallInt>, _>(Some(resolution.outcome))
                                    .bind::<diesel::sql_types::BigInt, _>(resolution.resolved_epoch)
                                    .bind::<diesel::sql_types::Text, _>(&parsed.post_id)
                                    .execute(&mut conn).await?;

                                // Insert summary
                                diesel::insert_into(schema::spot_resolutions::table)
                                    .values(&resolution)
                                    .execute(&mut conn)
                                    .await?;

                                // Log event
                                if let Some(eid) = event_id {
                                    let json = serde_json::to_value(&parsed)?;
                                    diesel::insert_into(schema::spot_events::table)
                                        .values(&crate::models::NewSpotEventLog {
                                            event_type: "SpotResolvedEvent".to_string(),
                                            post_id: parsed.post_id.clone(),
                                            event_data: json,
                                            event_id: eid,
                                            created_at: chrono::Utc::now(),
                                        })
                                        .execute(&mut conn)
                                        .await?;
                                }

                                // Unified SPoT event row
                                let unified = crate::models::NewSocialProofOfTruthEvent {
                                    event_type: "SpotResolvedEvent".to_string(),
                                    post_id: parsed.post_id.clone(),
                                    user_address: None,
                                    option_id: None,
                                    escrow_amount: None,
                                    amm_amount: None,
                                    amount: None,
                                    outcome: Some(parsed.outcome as i16),
                                    total_escrow: Some(parsed.total_escrow as i64),
                                    fee_taken: Some(parsed.fee_taken as i64),
                                    confidence_bps: None,
                                    timestamp_epoch: ts_secs,
                                    time: chrono::Utc::now(),
                                    event_id: event_id.clone(),
                                    transaction_id: event.tx_digest.clone(),
                                    raw_event: Some(serde_json::to_value(&parsed)?),
                                };
                                diesel::insert_into(schema::social_proof_of_truth::table)
                                    .values(&unified)
                                    .execute(&mut conn)
                                    .await?;
                            }
                            Err(e) => error!("Failed to parse SpotResolvedEvent: {}", e),
                        }
                    },

                    t if t.ends_with("SpotDaoRequiredEvent") => {
                        match parse_event::<SpotDaoRequiredEvt>(event) {
                            Ok(parsed) => {
                                let event_id = if let Some(tx_digest) = &event.tx_digest {
                                    let event_id_struct = EventID { tx_digest: tx_digest.clone(), event_seq: event.event_num };
                                    Some(event_id_struct.to_string())
                                } else { None };
                                let mut conn = self.get_connection().await?;
                                // Set status DAO_REQUIRED = 2
                                diesel::sql_query("UPDATE spot_records SET status = 2, updated_at = NOW() WHERE post_id = $1")
                                    .bind::<diesel::sql_types::Text, _>(&parsed.post_id)
                                    .execute(&mut conn).await?;
                                if let Some(eid) = event_id {
                                    let json = serde_json::to_value(&parsed)?;
                                    diesel::insert_into(schema::spot_events::table)
                                        .values(&crate::models::NewSpotEventLog {
                                            event_type: "SpotDaoRequiredEvent".to_string(),
                                            post_id: parsed.post_id.clone(),
                                            event_data: json,
                                            event_id: eid,
                                            created_at: chrono::Utc::now(),
                                        })
                                        .execute(&mut conn)
                                        .await?;
                                }

                                // Unified SPoT event row
                                let unified = crate::models::NewSocialProofOfTruthEvent {
                                    event_type: "SpotDaoRequiredEvent".to_string(),
                                    post_id: parsed.post_id.clone(),
                                    user_address: None,
                                    option_id: None,
                                    escrow_amount: None,
                                    amm_amount: None,
                                    amount: None,
                                    outcome: None,
                                    total_escrow: None,
                                    fee_taken: None,
                                    confidence_bps: Some(parsed.confidence_bps as i64),
                                    timestamp_epoch: (checkpoint.checkpoint_summary.timestamp_ms as i64) / 1000,
                                    time: chrono::Utc::now(),
                                    event_id: None,
                                    transaction_id: event.tx_digest.clone(),
                                    raw_event: Some(serde_json::to_value(&parsed)?),
                                };
                                diesel::insert_into(schema::social_proof_of_truth::table)
                                    .values(&unified)
                                    .execute(&mut conn)
                                    .await?;
                            }
                            Err(e) => error!("Failed to parse SpotDaoRequiredEvent: {}", e),
                        }
                    },

                    t if t.ends_with("SpotPayoutEvent") => {
                        match parse_event::<SpotPayoutEvt>(event) {
                            Ok(parsed) => {
                                let tx = event.tx_digest.clone().unwrap_or_default();
                                let ts_secs: i64 = (checkpoint.checkpoint_summary.timestamp_ms as i64) / 1000;
                                let payout: NewSpotPayout = parsed.into_model(ts_secs as u64, tx.clone())?;
                                let mut conn = self.get_connection().await?;
                                diesel::insert_into(schema::spot_payouts::table)
                                    .values(&payout)
                                    .execute(&mut conn)
                                    .await?;

                                // Unified SPoT event row
                                let unified = crate::models::NewSocialProofOfTruthEvent {
                                    event_type: "SpotPayoutEvent".to_string(),
                                    post_id: parsed.post_id.clone(),
                                    user_address: Some(parsed.user.clone()),
                                    option_id: None,
                                    escrow_amount: None,
                                    amm_amount: None,
                                    amount: Some(parsed.amount as i64),
                                    outcome: None,
                                    total_escrow: None,
                                    fee_taken: None,
                                    confidence_bps: None,
                                    timestamp_epoch: ts_secs,
                                    time: chrono::Utc::now(),
                                    event_id: None,
                                    transaction_id: event.tx_digest.clone(),
                                    raw_event: Some(serde_json::to_value(&parsed)?),
                                };
                                diesel::insert_into(schema::social_proof_of_truth::table)
                                    .values(&unified)
                                    .execute(&mut conn)
                                    .await?;
                            }
                            Err(e) => error!("Failed to parse SpotPayoutEvent: {}", e),
                        }
                    },

                    t if t.ends_with("SpotRefundEvent") => {
                        match parse_event::<SpotRefundEvt>(event) {
                            Ok(parsed) => {
                                let tx = event.tx_digest.clone().unwrap_or_default();
                                let ts_secs: i64 = (checkpoint.checkpoint_summary.timestamp_ms as i64) / 1000;
                                let refund: NewSpotRefund = parsed.into_model(ts_secs as u64, tx.clone())?;
                                let mut conn = self.get_connection().await?;
                                diesel::insert_into(schema::spot_refunds::table)
                                    .values(&refund)
                                    .execute(&mut conn)
                                    .await?;

                                // Unified SPoT event row
                                let unified = crate::models::NewSocialProofOfTruthEvent {
                                    event_type: "SpotRefundEvent".to_string(),
                                    post_id: parsed.post_id.clone(),
                                    user_address: Some(parsed.user.clone()),
                                    option_id: None,
                                    escrow_amount: None,
                                    amm_amount: None,
                                    amount: Some(parsed.amount as i64),
                                    outcome: None,
                                    total_escrow: None,
                                    fee_taken: None,
                                    confidence_bps: None,
                                    timestamp_epoch: ts_secs,
                                    time: chrono::Utc::now(),
                                    event_id: None,
                                    transaction_id: event.tx_digest.clone(),
                                    raw_event: Some(serde_json::to_value(&parsed)?),
                                };
                                diesel::insert_into(schema::social_proof_of_truth::table)
                                    .values(&unified)
                                    .execute(&mut conn)
                                    .await?;
                            }
                            Err(e) => error!("Failed to parse SpotRefundEvent: {}", e),
                        }
                    },
