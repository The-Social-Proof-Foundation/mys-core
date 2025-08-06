// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Represents the types of social graph events in the blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialGraphEventType {
    Follow,
    Unfollow,
}

/// Event details for follow actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowEventDetails {
    pub follower_address: String,
    pub following_address: String,
    pub timestamp: u64,
}

/// Event details for unfollow actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfollowEventDetails {
    pub follower_address: String,
    pub following_address: String,
    pub timestamp: u64,
} 