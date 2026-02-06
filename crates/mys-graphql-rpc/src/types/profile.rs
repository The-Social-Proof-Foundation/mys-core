// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::*;
use chrono::{DateTime as ChronoDateTime, Utc};
use mys_indexer::social::models::universal_user::{
    ReservationStatus as DbReservationStatus, SelectedBadgeInfo, SocialProofTokenInfo,
};

use super::big_int::BigInt;
use super::date_time::DateTime;

/// Reservation pool status
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ReservationStatus {
    /// Reservation pool exists and is active
    Active,
    /// Reservation pool exists and threshold met
    ThresholdMet,
    /// Reservation pool exists but inactive
    Inactive,
    /// No reservation pool
    None,
}

impl From<DbReservationStatus> for ReservationStatus {
    fn from(status: DbReservationStatus) -> Self {
        match status {
            DbReservationStatus::Active => ReservationStatus::Active,
            DbReservationStatus::ThresholdMet => ReservationStatus::ThresholdMet,
            DbReservationStatus::Inactive => ReservationStatus::Inactive,
            DbReservationStatus::None => ReservationStatus::None,
        }
    }
}

/// Social Proof Token information (includes reservation data)
#[derive(SimpleObject)]
pub(crate) struct SocialProofToken {
    /// SPT Pool ID
    pub pool_id: Option<String>,
    /// Token address (from profiles.social_proof_token_address)
    pub token_address: Option<String>,
    /// True if SPT pool exists and is active/trading
    pub is_active: bool,
    /// Reservation pool ID
    pub reservation_pool_id: Option<String>,
    /// Reservation percentage (0.0 if no reservation pool)
    pub reservation_percentage: f64,
    /// Reservation status
    pub reservation_status: ReservationStatus,
    /// Total amount reserved
    pub total_reserved: i64,
    /// Required threshold for reservation pool
    pub required_threshold: i64,
}

impl From<SocialProofTokenInfo> for SocialProofToken {
    fn from(info: SocialProofTokenInfo) -> Self {
        SocialProofToken {
            pool_id: info.pool_id,
            token_address: info.token_address,
            is_active: info.is_active,
            reservation_pool_id: info.reservation_pool_id,
            reservation_percentage: info.reservation_percentage,
            reservation_status: info.reservation_status.into(),
            total_reserved: info.total_reserved,
            required_threshold: info.required_threshold,
        }
    }
}

/// Selected badge information
#[derive(SimpleObject)]
pub(crate) struct SelectedBadge {
    /// Badge ID
    pub badge_id: String,
    /// Badge name
    pub badge_name: String,
    /// Badge icon URL
    pub badge_icon_url: Option<String>,
    /// Badge media URL
    pub badge_media_url: Option<String>,
    /// Platform ID that issued the badge
    pub platform_id: String,
    /// Badge type/tier
    pub badge_type: i32,
}

impl From<SelectedBadgeInfo> for SelectedBadge {
    fn from(info: SelectedBadgeInfo) -> Self {
        SelectedBadge {
            badge_id: info.badge_id,
            badge_name: info.badge_name,
            badge_icon_url: info.badge_icon_url,
            badge_media_url: info.badge_media_url,
            platform_id: info.platform_id,
            badge_type: info.badge_type as i32,
        }
    }
}

/// Profile type with all database fields plus enrichment data
#[derive(SimpleObject)]
pub(crate) struct Profile {
    /// Database ID
    pub id: i32,
    /// Wallet address (owner_address)
    pub owner_address: String,
    /// Username
    pub username: String,
    /// Display name
    pub display_name: Option<String>,
    /// Bio
    pub bio: Option<String>,
    /// Profile photo URL
    pub profile_photo: Option<String>,
    /// Website URL
    pub website: Option<String>,
    /// Cover photo URL
    pub cover_photo: Option<String>,
    /// Profile ID (on-chain object ID)
    pub profile_id: Option<String>,
    /// Number of followers
    pub followers_count: i32,
    /// Number of users being followed
    pub following_count: i32,
    /// Number of blocked users
    pub blocked_count: i32,
    /// Number of posts
    pub post_count: i32,
    /// Minimum offer amount for profile sales
    pub min_offer_amount: Option<BigInt>,
    /// Creation timestamp
    pub created_at: DateTime,
    /// Last update timestamp
    pub updated_at: DateTime,
    /// Paid messaging enabled flag
    pub paid_messaging_enabled: bool,
    /// Paid messaging minimum cost
    pub paid_messaging_min_cost: Option<BigInt>,
    /// X/Twitter username (encrypted)
    pub x_username: Option<String>,
    /// Facebook username (encrypted)
    pub facebook_username: Option<String>,
    /// Reddit username (encrypted)
    pub reddit_username: Option<String>,
    /// GitHub username (encrypted)
    pub github_username: Option<String>,
    /// Instagram username (encrypted)
    pub instagram_username: Option<String>,
    /// LinkedIn username (encrypted)
    pub linkedin_username: Option<String>,
    /// Twitch username (encrypted)
    pub twitch_username: Option<String>,
    /// Social proof token address (from database)
    pub social_proof_token_address: Option<String>,
    /// Reservation pool address (from database)
    pub reservation_pool_address: Option<String>,
    /// Selected badge ID (from database)
    pub selected_badge_id: Option<String>,
    /// Social Proof Token info (from enrichment)
    pub social_proof_token: Option<SocialProofToken>,
    /// Selected badge info (from enrichment)
    pub selected_badge: Option<SelectedBadge>,
}

/// Helper to convert database Profile to GraphQL Profile
impl Profile {
    pub fn from_db_profile(
        profile: &mys_indexer::social::models::Profile,
    ) -> Result<Self, crate::error::Error> {
        // Convert NaiveDateTime to DateTime<Utc> then to GraphQL DateTime
        let created_at = ChronoDateTime::<Utc>::from_naive_utc_and_offset(profile.created_at, Utc);
        let updated_at = ChronoDateTime::<Utc>::from_naive_utc_and_offset(profile.updated_at, Utc);

        Ok(Profile {
            id: profile.id,
            owner_address: profile.owner_address.clone(),
            username: profile.username.clone(),
            display_name: profile.display_name.clone(),
            bio: profile.bio.clone(),
            profile_photo: profile.profile_photo.clone(),
            website: profile.website.clone(),
            cover_photo: profile.cover_photo.clone(),
            profile_id: profile.profile_id.clone(),
            followers_count: profile.followers_count,
            following_count: profile.following_count,
            blocked_count: profile.blocked_count,
            post_count: profile.post_count,
            min_offer_amount: profile.min_offer_amount.map(BigInt::from),
            created_at: DateTime::from_chrono(created_at),
            updated_at: DateTime::from_chrono(updated_at),
            paid_messaging_enabled: profile.paid_messaging_enabled,
            paid_messaging_min_cost: profile.paid_messaging_min_cost.map(BigInt::from),
            x_username: profile.x_username.clone(),
            facebook_username: profile.facebook_username.clone(),
            reddit_username: profile.reddit_username.clone(),
            github_username: profile.github_username.clone(),
            instagram_username: profile.instagram_username.clone(),
            linkedin_username: profile.linkedin_username.clone(),
            twitch_username: profile.twitch_username.clone(),
            social_proof_token_address: profile.social_proof_token_address.clone(),
            reservation_pool_address: profile.reservation_pool_address.clone(),
            selected_badge_id: profile.selected_badge_id.clone(),
            social_proof_token: None,
            selected_badge: None,
        })
    }
}
