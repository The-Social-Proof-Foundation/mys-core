// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Universal user result structure returned by all endpoints that return user/profile arrays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalUserResult {
    /// Wallet address (owner_address)
    pub wallet_address: String,
    /// Username from profile
    pub username: Option<String>,
    /// Display name (fullname) from profile
    pub fullname: Option<String>,
    /// Profile photo URL
    pub profile_photo: Option<String>,
    
    /// Social Proof Token info (includes reservation data)
    pub social_proof_token: Option<SocialProofTokenInfo>,
    
    /// Selected badge info
    pub selected_badge: Option<SelectedBadgeInfo>,
}

/// Social Proof Token information (consolidates SPT pool and reservation pool data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialProofTokenInfo {
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

/// Reservation pool status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationStatus {
    /// Reservation pool exists and is active
    Active,
    /// Reservation pool exists and threshold met
    ThresholdMet,
    /// Reservation pool exists but inactive
    Inactive,
    /// No reservation pool
    None,
}

/// Selected badge information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedBadgeInfo {
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
    pub badge_type: i16,
}
