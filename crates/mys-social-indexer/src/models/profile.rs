// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::schema::profiles;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profiles)]
pub struct Profile {
    pub id: i32,
    pub owner_address: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub website: Option<String>, // Website field from contract
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub cover_photo: Option<String>,
    pub profile_id: Option<String>,
    // Social graph statistics
    pub followers_count: i32,
    pub following_count: i32,
    // Blocking statistics
    pub blocked_count: i32,
    // Post count - number of top-level, non-deleted posts
    pub post_count: i32,
    // Minimum offer amount for profile sales (NULL = not for sale)
    pub min_offer_amount: Option<i64>,
    // Sensitive fields (all client-side encrypted)
    pub birthdate: Option<String>,
    pub current_location: Option<String>,
    pub raised_location: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub political_view: Option<String>,
    pub religion: Option<String>,
    pub education: Option<String>,
    pub primary_language: Option<String>,
    pub relationship_status: Option<String>,
    pub x_username: Option<String>,
    pub mastodon_username: Option<String>,
    pub facebook_username: Option<String>,
    pub reddit_username: Option<String>,
    pub github_username: Option<String>,
    pub instagram_username: Option<String>,
    // BlockList object address
    pub block_list_address: Option<String>,
    // Social proof token address
    pub social_proof_token_address: Option<String>,
    // Reservation pool object address
    pub reservation_pool_address: Option<String>,
    // Selected badge ID - the badge currently selected for display
    pub selected_badge_id: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub owner_address: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub website: Option<String>, // Website field from contract
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub cover_photo: Option<String>,
    pub profile_id: Option<String>,
    // Social graph statistics - initialize to 0
    #[serde(default)]
    pub followers_count: i32,
    #[serde(default)]
    pub following_count: i32,
    // Blocking statistics - initialize to 0
    #[serde(default)]
    pub blocked_count: i32,
    // Post count - initialize to 0
    #[serde(default)]
    pub post_count: i32,
    // Minimum offer amount for profile sales - initialize to None
    #[serde(default)]
    pub min_offer_amount: Option<i64>,
    // Sensitive fields (all client-side encrypted)
    pub birthdate: Option<String>,
    pub current_location: Option<String>,
    pub raised_location: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub political_view: Option<String>,
    pub religion: Option<String>,
    pub education: Option<String>,
    pub primary_language: Option<String>,
    pub relationship_status: Option<String>,
    pub x_username: Option<String>,
    pub mastodon_username: Option<String>,
    pub facebook_username: Option<String>,
    pub reddit_username: Option<String>,
    pub github_username: Option<String>,
    pub instagram_username: Option<String>,
    // BlockList object address
    pub block_list_address: Option<String>,
    // Social proof token address
    pub social_proof_token_address: Option<String>,
    // Reservation pool object address
    pub reservation_pool_address: Option<String>,
    // Selected badge ID - the badge currently selected for display
    pub selected_badge_id: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = profiles)]
pub struct UpdateProfile {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub website: Option<String>, // Website field from contract
    pub cover_photo: Option<String>,
    // Social graph statistics - optional for when they need to be updated
    pub followers_count: Option<i32>,
    pub following_count: Option<i32>,
    pub blocked_count: Option<i32>,
    // Post count - optional for when it needs to be updated
    pub post_count: Option<i32>,
    // Minimum offer amount for profile sales - optional for when it needs to be updated
    pub min_offer_amount: Option<i64>,
    // Sensitive fields (all client-side encrypted)
    pub birthdate: Option<String>,
    pub current_location: Option<String>,
    pub raised_location: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub political_view: Option<String>,
    pub religion: Option<String>,
    pub education: Option<String>,
    pub primary_language: Option<String>,
    pub relationship_status: Option<String>,
    pub x_username: Option<String>,
    pub mastodon_username: Option<String>,
    pub facebook_username: Option<String>,
    pub reddit_username: Option<String>,
    pub github_username: Option<String>,
    pub instagram_username: Option<String>,
    // BlockList object address
    pub block_list_address: Option<String>,
    // Social proof token address
    pub social_proof_token_address: Option<String>,
    // Selected badge ID - the badge currently selected for display
    pub selected_badge_id: Option<String>,
}
