// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

use crate::social::models::profile::NewProfile;

/// Helper function to deserialize strings as numbers
fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    // This will handle both string and numeric inputs
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber<T> {
        String(String),
        Number(T),
    }

    match StringOrNumber::<T>::deserialize(deserializer) {
        Ok(StringOrNumber::String(s)) => T::from_str(&s).map_err(serde::de::Error::custom),
        Ok(StringOrNumber::Number(n)) => Ok(n),
        Err(e) => Err(e),
    }
}

/// Helper function to deserialize optional strings as optional numbers
fn deserialize_optional_number_from_string<'de, T, D>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    T: FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    // This will handle both string and numeric inputs, and None values
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumberOrNone<T> {
        String(String),
        Number(T),
        None,
    }

    match StringOrNumberOrNone::<T>::deserialize(deserializer) {
        Ok(StringOrNumberOrNone::String(s)) => {
            if s.is_empty() {
                Ok(None)
            } else {
                match T::from_str(&s) {
                    Ok(val) => Ok(Some(val)),
                    Err(e) => Err(serde::de::Error::custom(e)),
                }
            }
        }
        Ok(StringOrNumberOrNone::Number(n)) => Ok(Some(n)),
        Ok(StringOrNumberOrNone::None) => Ok(None),
        Err(_) => Ok(None), // Treat errors as None
    }
}

/// Helper function for default timestamp
fn default_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Default function for u8 fields that should be 0
fn default_zero_u8() -> u8 {
    0
}

/// Default function for numeric fields that should be 0
fn default_zero() -> u64 {
    0
}

/// Event emitted when a profile is created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileCreatedEvent {
    /// ID of the profile - can come from multiple sources
    #[serde(rename = "profile_id", alias = "id", default)]
    pub profile_id: String,

    /// Owner's address - can be 'owner' or 'owner_address' in the event
    #[serde(rename = "owner_address", alias = "owner", default)]
    pub owner_address: String,

    /// Username - may not be present in the event
    #[serde(default)]
    pub username: Option<String>,

    /// Display name
    #[serde(rename = "display_name", default)]
    pub display_name: String,

    /// Profile photo URL - can come from multiple fields
    #[serde(
        rename = "profile_photo",
        alias = "profile_picture",
        alias = "avatar_url",
        default
    )]
    pub profile_photo: Option<String>,

    /// Cover photo URL
    #[serde(rename = "cover_photo", alias = "cover_url", default)]
    pub cover_photo: Option<String>,

    /// Bio - may be a string directly in the event
    #[serde(default)]
    pub bio: Option<String>,

    /// Timestamp of profile creation
    #[serde(
        rename = "created_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub created_at: u64,
}

impl ProfileCreatedEvent {
    /// Convert the event to a database model
    pub fn into_model(&self) -> Result<NewProfile> {
        // Always use the current time for database entries instead of blockchain epoch
        // Blockchain epoch values are small numbers (like 21) and not actual Unix timestamps
        let now = Utc::now().naive_utc();

        // Use username if available, otherwise generate a placeholder
        let username = match &self.username {
            Some(name) => name.clone(),
            None => format!(
                "user_{}",
                self.owner_address.chars().take(8).collect::<String>()
            ),
        };

        // Log all fields for debugging
        tracing::info!("Converting ProfileCreatedEvent to database model:");
        tracing::info!("  profile_id: {}", self.profile_id);
        tracing::info!("  username: {}", username);
        tracing::info!("  display_name: {}", self.display_name);
        tracing::info!("  bio: {:?}", self.bio);
        tracing::info!("  profile_photo: {:?}", self.profile_photo);
        tracing::info!("  cover_photo: {:?}", self.cover_photo);
        tracing::info!("  using current timestamp instead of blockchain epoch");

        // Always use the profile photo if it exists
        let profile_photo = self.profile_photo.clone();

        // Always use the cover photo if it exists
        let cover_photo = self.cover_photo.clone();

        Ok(NewProfile {
            owner_address: self.owner_address.clone(),
            username,
            display_name: Some(self.display_name.clone()),
            bio: self.bio.clone(),
            profile_photo,
            website: None,   // Not provided in profile creation event
            created_at: now, // Use current time for created_at
            updated_at: now, // Use current time for updated_at
            cover_photo,
            profile_id: Some(self.profile_id.clone()),
            // Initialize follower/following counts to 0
            followers_count: 0,
            following_count: 0,
            blocked_count: 0,
            // Initialize post count to 0
            post_count: 0,
            // Initialize minimum offer amount as None (not for sale)
            min_offer_amount: None,
            // Initialize all sensitive fields as None
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
            reservation_pool_address: None, // Will be set when reservation pool is created
            selected_badge_id: None, // Will be set when badge is selected
            paid_messaging_enabled: false, // Default to disabled
            paid_messaging_min_cost: None, // Default to no minimum cost
        })
    }
}

/// Updated ProfileUpdatedEvent with all profile fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdatedEvent {
    /// ID of the profile
    #[serde(rename = "profile_id", alias = "id", default)]
    pub profile_id: String,

    /// Display name
    #[serde(rename = "display_name", default)]
    pub display_name: Option<String>,

    /// Username
    #[serde(default)]
    pub username: Option<String>,

    /// Owner's address
    #[serde(rename = "owner_address", alias = "owner", default)]
    pub owner_address: String,

    /// Profile photo URL
    #[serde(
        rename = "profile_photo",
        alias = "profile_picture",
        alias = "avatar_url",
        default
    )]
    pub profile_photo: Option<String>,

    /// Cover photo URL
    #[serde(rename = "cover_photo", alias = "cover_url", default)]
    pub cover_photo: Option<String>,

    /// Bio
    #[serde(rename = "bio", alias = "description", default)]
    pub bio: Option<String>,

    /// Update timestamp
    #[serde(
        rename = "updated_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub updated_at: u64,

    // All sensitive fields that are client-side encrypted
    #[serde(default)]
    pub birthdate: Option<String>,

    #[serde(default)]
    pub current_location: Option<String>,

    #[serde(default)]
    pub raised_location: Option<String>,

    #[serde(default)]
    pub phone: Option<String>,

    #[serde(default)]
    pub email: Option<String>,

    #[serde(default)]
    pub gender: Option<String>,

    #[serde(default)]
    pub political_view: Option<String>,

    #[serde(default)]
    pub religion: Option<String>,

    #[serde(default)]
    pub education: Option<String>,

    #[serde(default)]
    pub primary_language: Option<String>,

    #[serde(default)]
    pub relationship_status: Option<String>,

    #[serde(default)]
    pub x_username: Option<String>,

    #[serde(default)]
    pub facebook_username: Option<String>,

    #[serde(default)]
    pub reddit_username: Option<String>,

    #[serde(default)]
    pub github_username: Option<String>,

    #[serde(default)]
    pub instagram_username: Option<String>,

    #[serde(default)]
    pub linkedin_username: Option<String>,

    #[serde(default)]
    pub twitch_username: Option<String>,

    #[serde(default)]
    pub min_offer_amount: Option<u64>,
}

/// Event emitted when a username is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameUpdatedEvent {
    /// ID of the profile
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,
    /// Old username
    #[serde(rename = "old_username", default)]
    pub old_username: String,
    /// New username
    #[serde(rename = "new_username", default)]
    pub new_username: String,
    /// Owner's address
    #[serde(rename = "owner_address", alias = "owner", default)]
    pub owner_address: String,
}

/// Event emitted when a username is registered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameRegisteredEvent {
    /// ID of the profile
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,
    /// Username
    #[serde(default)]
    pub username: String,
    /// Owner's address
    #[serde(rename = "owner_address", alias = "owner", default)]
    pub owner_address: String,
    /// Expiration timestamp
    #[serde(
        rename = "expires_at",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    pub expires_at: u64,
    /// Registration timestamp
    #[serde(
        rename = "registered_at",
        default,
        deserialize_with = "deserialize_number_from_string"
    )]
    pub registered_at: u64,
}

/// Event emitted when a profile joins a platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileJoinedPlatformEvent {
    /// ID of the profile
    pub profile_id: String,
    /// ID of the platform
    pub platform_id: String,
    /// Timestamp of the join action
    #[serde(default, deserialize_with = "deserialize_optional_number_from_string")]
    pub joined_at: Option<u64>,
}

/// Event emitted when MYS tokens are vested
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokensVestedEvent {
    /// ID of the vesting wallet
    #[serde(rename = "wallet_id", default)]
    pub wallet_id: String,

    /// Address of the wallet owner
    #[serde(rename = "owner", default)]
    pub owner: String,

    /// Total amount of tokens vested
    #[serde(
        rename = "total_amount",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub total_amount: u64,

    /// Start time of vesting (in milliseconds)
    #[serde(
        rename = "start_time",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub start_time: u64,

    /// Duration of vesting period (in milliseconds)
    #[serde(
        rename = "duration",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub duration: u64,

    /// Curve factor for vesting schedule
    #[serde(
        rename = "curve_factor",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub curve_factor: u64,

    /// Timestamp when tokens were vested (in milliseconds)
    #[serde(
        rename = "vested_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub vested_at: u64,
}

impl TokensVestedEvent {
    /// Convert the event to vesting wallet and event models
    pub fn into_models(
        &self,
        transaction_id: String,
    ) -> (
        crate::social::models::NewVestingWallet,
        crate::social::models::NewVestingEvent,
    ) {
        let wallet = crate::social::models::NewVestingWallet::from_tokens_vested_event(
            self.wallet_id.clone(),
            self.owner.clone(),
            self.total_amount,
            self.start_time,
            self.duration,
            self.curve_factor,
            transaction_id.clone(),
            Some(self.vested_at),
        );

        let event = crate::social::models::NewVestingEvent::from_tokens_vested_event(
            self.wallet_id.clone(),
            self.owner.clone(),
            self.total_amount,
            self.start_time,
            self.duration,
            self.curve_factor,
            self.vested_at,
            transaction_id,
        );

        (wallet, event)
    }
}

/// Event emitted when vested tokens are claimed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokensClaimedEvent {
    /// ID of the vesting wallet
    #[serde(rename = "wallet_id", default)]
    pub wallet_id: String,

    /// Address of the wallet owner
    #[serde(rename = "owner", default)]
    pub owner: String,

    /// Amount of tokens claimed in this transaction
    #[serde(
        rename = "claimed_amount",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub claimed_amount: u64,

    /// Remaining balance after this claim
    #[serde(
        rename = "remaining_balance",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub remaining_balance: u64,

    /// Timestamp when tokens were claimed (in milliseconds)
    #[serde(
        rename = "claimed_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub claimed_at: u64,
}

impl TokensClaimedEvent {
    /// Convert the event to a database event model
    /// 
    /// Note: The wallet update is now handled separately in the processor
    /// to ensure cumulative claimed_amount is calculated correctly.
    pub fn into_models(
        &self,
        transaction_id: String,
    ) -> crate::social::models::NewVestingEvent {
        crate::social::models::NewVestingEvent::from_tokens_claimed_event(
            self.wallet_id.clone(),
            self.owner.clone(),
            self.claimed_amount,
            self.remaining_balance,
            self.claimed_at,
            transaction_id,
        )
    }
}

/// Event emitted when an offer is created for a profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileOfferCreatedEvent {
    /// ID of the profile
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,

    /// Address of the offeror
    #[serde(rename = "offeror", default)]
    pub offeror: String,

    /// Amount of the offer in MYS tokens
    #[serde(
        rename = "amount",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub amount: u64,

    /// Timestamp when the offer was created
    #[serde(
        rename = "created_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub created_at: u64,
}

/// Event emitted when an offer is accepted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileOfferAcceptedEvent {
    /// ID of the profile
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,

    /// Address of the offeror (new owner)
    #[serde(rename = "offeror", default)]
    pub offeror: String,

    /// Address of the previous owner
    #[serde(rename = "previous_owner", default)]
    pub previous_owner: String,

    /// Amount of the offer in MYS tokens
    #[serde(
        rename = "amount",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub amount: u64,

    /// Timestamp when the offer was accepted
    #[serde(
        rename = "accepted_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub accepted_at: u64,
}

/// Event emitted when an offer is rejected or revoked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileOfferRejectedEvent {
    /// ID of the profile
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,

    /// Address of the offeror
    #[serde(rename = "offeror", default)]
    pub offeror: String,

    /// Address of who rejected/revoked the offer
    #[serde(rename = "rejected_by", default)]
    pub rejected_by: String,

    /// Amount of the offer in MYS tokens
    #[serde(
        rename = "amount",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub amount: u64,

    /// Timestamp when the offer was rejected/revoked
    #[serde(
        rename = "rejected_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub rejected_at: u64,

    /// Whether this was a revocation (by offeror) vs rejection (by owner)
    #[serde(rename = "is_revoked", default)]
    pub is_revoked: bool,
}

/// Event emitted when a fee is collected from a profile sale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSaleFeeEvent {
    /// ID of the profile
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,

    /// Address of the offeror (buyer)
    #[serde(rename = "offeror", default)]
    pub offeror: String,

    /// Address of the previous owner (seller)
    #[serde(rename = "previous_owner", default)]
    pub previous_owner: String,

    /// Total sale amount in MYS tokens
    #[serde(
        rename = "sale_amount",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub sale_amount: u64,

    /// Fee amount collected in MYS tokens
    #[serde(
        rename = "fee_amount",
        default = "default_zero",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub fee_amount: u64,

    /// Address that received the fee
    #[serde(rename = "fee_recipient", default)]
    pub fee_recipient: String,

    /// Timestamp when the fee was collected
    #[serde(
        rename = "timestamp",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub timestamp: u64,
}

/// Event emitted when a badge is assigned to a profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeAssignedEvent {
    /// ID of the profile receiving the badge
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,

    /// Badge identifier
    #[serde(rename = "badge_id", default)]
    pub badge_id: String,

    /// Badge name
    #[serde(rename = "name", default)]
    pub name: String,

    /// Badge description (optional, may not be in older events)
    #[serde(rename = "description", default)]
    pub description: Option<String>,

    /// Media URL for the badge (optional, may not be in older events)
    #[serde(rename = "media_url", default)]
    pub media_url: Option<String>,

    /// Icon URL for the badge (optional, may not be in older events)
    #[serde(rename = "icon_url", default)]
    pub icon_url: Option<String>,

    /// Platform ID that issued the badge
    #[serde(rename = "platform_id", default)]
    pub platform_id: String,

    /// Admin/moderator who assigned the badge
    #[serde(rename = "assigned_by", default)]
    pub assigned_by: String,

    /// Timestamp when assigned
    #[serde(
        rename = "assigned_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub assigned_at: u64,

    /// Badge type/tier (1-100)
    #[serde(
        rename = "badge_type",
        default = "default_zero_u8",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub badge_type: u8,
}

/// Event emitted when a badge is revoked from a profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeRevokedEvent {
    /// ID of the profile losing the badge
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,

    /// Badge identifier
    #[serde(rename = "badge_id", default)]
    pub badge_id: String,

    /// Platform ID that issued the badge
    #[serde(rename = "platform_id", default)]
    pub platform_id: String,

    /// Admin/moderator who revoked the badge
    #[serde(rename = "revoked_by", default)]
    pub revoked_by: String,

    /// Timestamp when revoked
    #[serde(
        rename = "revoked_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub revoked_at: u64,
}

/// Event emitted when a profile owner selects a badge to display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeSelectedEvent {
    /// ID of the profile
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,

    /// Badge identifier that was selected
    #[serde(rename = "badge_id", default)]
    pub badge_id: String,

    /// Owner who selected the badge
    #[serde(rename = "selected_by", default)]
    pub selected_by: String,

    /// Timestamp when selected
    #[serde(
        rename = "selected_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub selected_at: u64,
}

/// Event emitted when a vesting wallet is deleted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingWalletDeletedEvent {
    /// ID of the vesting wallet
    #[serde(rename = "wallet_id", default)]
    pub wallet_id: String,

    /// Owner of the wallet
    #[serde(rename = "owner", default)]
    pub owner: String,

    /// Timestamp when deleted
    #[serde(
        rename = "deleted_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub deleted_at: u64,
}

/// Event emitted when paid messaging settings are updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidMessagingSettingsUpdatedEvent {
    /// ID of the profile
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,

    /// Owner of the profile
    #[serde(rename = "owner", default)]
    pub owner: String,

    /// Whether paid messaging is enabled
    #[serde(default)]
    pub enabled: bool,

    /// Minimum cost for messaging (in MYS tokens)
    #[serde(
        rename = "min_cost",
        default,
        deserialize_with = "deserialize_optional_number_from_string"
    )]
    pub min_cost: Option<u64>,

    /// Timestamp when settings were updated
    #[serde(
        rename = "updated_at",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub updated_at: u64,
}

/// Event emitted when the ecosystem treasury address is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemTreasuryUpdatedEvent {
    /// Address of the admin who updated the treasury
    #[serde(rename = "updated_by", default)]
    pub updated_by: String,

    /// New treasury address
    #[serde(rename = "new_treasury_address", default)]
    pub new_treasury_address: String,

    /// Timestamp when the treasury was updated (in milliseconds)
    #[serde(
        rename = "timestamp",
        default = "default_timestamp",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub timestamp: u64,
}

// =============================================================================
// PROCESS FUNCTIONS FOR CHECKPOINT PROCESSOR
// =============================================================================

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::social::db::DbConnection;
use crate::social::schema::profiles;

/// Process a ProfileCreatedEvent and insert into the database
pub async fn process_profile_created_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    _event_id: &str,
) -> Result<()> {
    let event: ProfileCreatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow::anyhow!("Failed to parse ProfileCreatedEvent: {}", e))?;

    let model = event.into_model()?;

    diesel::insert_into(profiles::table)
        .values(&model)
        .on_conflict(profiles::owner_address)
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to insert profile: {}", e))?;

    tracing::info!("Processed ProfileCreatedEvent for profile_id: {}", event.profile_id);
    Ok(())
}

/// Process a ProfileUpdatedEvent and update the database
pub async fn process_profile_updated_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    _event_id: &str,
) -> Result<()> {
    let event: ProfileUpdatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow::anyhow!("Failed to parse ProfileUpdatedEvent: {}", e))?;

    let now = Utc::now().naive_utc();

    // Build update query based on available fields
    diesel::update(profiles::table)
        .filter(profiles::profile_id.eq(&event.profile_id))
        .set((
            profiles::display_name.eq(&event.display_name),
            profiles::bio.eq(&event.bio),
            profiles::profile_photo.eq(&event.profile_photo),
            profiles::cover_photo.eq(&event.cover_photo),
            profiles::updated_at.eq(now),
            // Update sensitive fields if provided
            profiles::birthdate.eq(&event.birthdate),
            profiles::current_location.eq(&event.current_location),
            profiles::raised_location.eq(&event.raised_location),
            profiles::phone.eq(&event.phone),
            profiles::email.eq(&event.email),
            profiles::gender.eq(&event.gender),
            profiles::political_view.eq(&event.political_view),
            profiles::religion.eq(&event.religion),
            profiles::education.eq(&event.education),
            profiles::primary_language.eq(&event.primary_language),
            profiles::relationship_status.eq(&event.relationship_status),
            profiles::x_username.eq(&event.x_username),
            profiles::facebook_username.eq(&event.facebook_username),
            profiles::reddit_username.eq(&event.reddit_username),
            profiles::github_username.eq(&event.github_username),
            profiles::instagram_username.eq(&event.instagram_username),
            profiles::linkedin_username.eq(&event.linkedin_username),
            profiles::twitch_username.eq(&event.twitch_username),
            profiles::min_offer_amount.eq(event.min_offer_amount.map(|v| v as i64)),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to update profile: {}", e))?;

    tracing::info!("Processed ProfileUpdatedEvent for profile_id: {}", event.profile_id);
    Ok(())
}

/// Process a UsernameRegisteredEvent and update the database
pub async fn process_username_registered_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    _event_id: &str,
) -> Result<()> {
    let event: UsernameRegisteredEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow::anyhow!("Failed to parse UsernameRegisteredEvent: {}", e))?;

    let now = Utc::now().naive_utc();

    // Update the profile's username
    diesel::update(profiles::table)
        .filter(profiles::profile_id.eq(&event.profile_id))
        .set((
            profiles::username.eq(&event.username),
            profiles::updated_at.eq(now),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to update username: {}", e))?;

    tracing::info!("Processed UsernameRegisteredEvent for profile_id: {} with username: {}",
        event.profile_id, event.username);
    Ok(())
}

/// Process a UsernameUpdatedEvent and update the database
pub async fn process_username_updated_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    _event_id: &str,
) -> Result<()> {
    let event: UsernameUpdatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow::anyhow!("Failed to parse UsernameUpdatedEvent: {}", e))?;

    let now = Utc::now().naive_utc();

    // Update the profile's username
    diesel::update(profiles::table)
        .filter(profiles::profile_id.eq(&event.profile_id))
        .set((
            profiles::username.eq(&event.new_username),
            profiles::updated_at.eq(now),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to update username: {}", e))?;

    tracing::info!("Processed UsernameUpdatedEvent for profile_id: {} from {} to {}",
        event.profile_id, event.old_username, event.new_username);
    Ok(())
}

/// Process a BadgeAssignedEvent and insert into the database
pub async fn process_badge_assigned_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
) -> Result<()> {
    use crate::social::schema::profile_badges;
    use crate::social::models::profile_extras::NewProfileBadge;

    let event: BadgeAssignedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow::anyhow!("Failed to parse BadgeAssignedEvent: {}", e))?;

    let badge = NewProfileBadge {
        profile_id: event.profile_id.clone(),
        badge_id: event.badge_id.clone(),
        badge_name: event.name.clone(),
        badge_description: event.description.clone(),
        badge_media_url: event.media_url.clone(),
        badge_icon_url: event.icon_url.clone(),
        platform_id: event.platform_id.clone(),
        assigned_by: event.assigned_by.clone(),
        assigned_at: event.assigned_at as i64,
        revoked: false,
        revoked_at: None,
        revoked_by: None,
        badge_type: event.badge_type as i16,
        transaction_id: event_id.to_string(),
        time: Utc::now().naive_utc(),
    };

    diesel::insert_into(profile_badges::table)
        .values(&badge)
        .on_conflict_do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to insert badge: {}", e))?;

    tracing::info!("Processed BadgeAssignedEvent for profile_id: {} badge_id: {}",
        event.profile_id, event.badge_id);
    Ok(())
}

/// Process a BadgeRevokedEvent and update the database
pub async fn process_badge_revoked_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    _event_id: &str,
) -> Result<()> {
    use crate::social::schema::profile_badges;

    let event: BadgeRevokedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow::anyhow!("Failed to parse BadgeRevokedEvent: {}", e))?;

    diesel::update(profile_badges::table)
        .filter(profile_badges::profile_id.eq(&event.profile_id))
        .filter(profile_badges::badge_id.eq(&event.badge_id))
        .set((
            profile_badges::revoked.eq(true),
            profile_badges::revoked_at.eq(Some(event.revoked_at as i64)),
            profile_badges::revoked_by.eq(Some(&event.revoked_by)),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to revoke badge: {}", e))?;

    tracing::info!("Processed BadgeRevokedEvent for profile_id: {} badge_id: {}",
        event.profile_id, event.badge_id);
    Ok(())
}

/// Process a BadgeSelectedEvent and update the profile's selected badge
pub async fn process_badge_selected_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    _event_id: &str,
) -> Result<()> {
    let event: BadgeSelectedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow::anyhow!("Failed to parse BadgeSelectedEvent: {}", e))?;

    let now = Utc::now().naive_utc();

    diesel::update(profiles::table)
        .filter(profiles::profile_id.eq(&event.profile_id))
        .set((
            profiles::selected_badge_id.eq(Some(&event.badge_id)),
            profiles::updated_at.eq(now),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to update selected badge: {}", e))?;

    tracing::info!("Processed BadgeSelectedEvent for profile_id: {} badge_id: {}",
        event.profile_id, event.badge_id);
    Ok(())
}

/// Process a PaidMessagingSettingsUpdatedEvent and update the profile
pub async fn process_paid_messaging_settings_updated_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    _event_id: &str,
) -> Result<()> {
    let event: PaidMessagingSettingsUpdatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow::anyhow!("Failed to parse PaidMessagingSettingsUpdatedEvent: {}", e))?;

    let now = Utc::now().naive_utc();

    diesel::update(profiles::table)
        .filter(profiles::profile_id.eq(&event.profile_id))
        .set((
            profiles::paid_messaging_enabled.eq(event.enabled),
            profiles::paid_messaging_min_cost.eq(event.min_cost.map(|v| v as i64)),
            profiles::updated_at.eq(now),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to update paid messaging settings: {}", e))?;

    tracing::info!("Processed PaidMessagingSettingsUpdatedEvent for profile_id: {}",
        event.profile_id);
    Ok(())
}

