// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Type of PoC event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PocEventType {
    AnalysisSubmitted,
    PocBadgeIssued,
    RevenueRedirectionActivated,
    PocDisputeSubmitted,
    DisputeVoteCast,
    PocDisputeResolved,
    VotingRewardClaimed,
    PocConfigUpdated,
    TokenPoolSyncNeeded,
}

/// Analysis submitted event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSubmittedEvent {
    pub post_id: String,
    pub media_type: u8,
    pub similarity_detected: bool,
    pub highest_similarity_score: u64,
    pub oracle_address: String,
    pub timestamp: u64,
}

/// PoC badge issued event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocBadgeIssuedEvent {
    pub badge_id: String,
    pub post_id: String,
    pub media_type: u8,
    pub issued_by: String,
    pub timestamp: u64,
}

/// Revenue redirection activated event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueRedirectionActivatedEvent {
    pub redirection_id: String,
    pub accused_post_id: String,
    pub original_post_id: String,
    pub redirect_percentage: u64,
    pub similarity_score: u64,
    pub timestamp: u64,
}

/// PoC dispute submitted event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocDisputeSubmittedEvent {
    pub dispute_id: String,
    pub post_id: String,
    pub disputer: String,
    pub dispute_type: u8,
    pub stake_amount: u64,
    pub voting_start_epoch: u64,
    pub voting_end_epoch: u64,
    pub timestamp: u64,
}

/// Dispute vote cast event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeVoteCastEvent {
    pub dispute_id: String,
    pub voter: String,
    pub vote_choice: u8,
    pub stake_amount: u64,
    pub total_uphold_stake: u64,
    pub total_overturn_stake: u64,
    pub timestamp: u64,
}

/// PoC dispute resolved event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocDisputeResolvedEvent {
    pub dispute_id: String,
    pub post_id: String,
    pub resolution: u8, // upheld or overturned
    pub winning_side: u8, // VOTE_UPHOLD or VOTE_OVERTURN
    pub total_winning_stake: u64,
    pub total_losing_stake: u64,
    pub badge_revoked: bool,
    pub redirection_removed: bool,
    pub timestamp: u64,
}

/// Voting reward claimed event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingRewardClaimedEvent {
    pub dispute_id: String,
    pub voter: String,
    pub original_stake: u64,
    pub reward_amount: u64,
    pub total_payout: u64,
    pub timestamp: u64,
}

/// PoC configuration updated event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocConfigUpdatedEvent {
    pub updated_by: String,
    pub image_threshold: u64,
    pub video_threshold: u64,
    pub audio_threshold: u64,
    pub revenue_redirect_percentage: u64,
    pub dispute_cost: u64,
    pub min_vote_stake: u64,
    pub max_vote_stake: u64,
    pub voting_duration_epochs: u64,
    pub timestamp: u64,
}

/// Token pool sync needed event from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPoolSyncNeededEvent {
    pub post_id: String,
    pub timestamp: u64,
}

// Constants matching the smart contract values
pub const MEDIA_TYPE_IMAGE: u8 = 1;
pub const MEDIA_TYPE_VIDEO: u8 = 2;
pub const MEDIA_TYPE_AUDIO: u8 = 3;

pub const DISPUTE_STATUS_VOTING: u8 = 1;
pub const DISPUTE_STATUS_RESOLVED_UPHELD: u8 = 2;
pub const DISPUTE_STATUS_RESOLVED_OVERTURNED: u8 = 3;

pub const VOTE_UPHOLD: u8 = 1;
pub const VOTE_OVERTURN: u8 = 2; 