// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Event emitted when a governance registry is created or updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRegistryEvent {
    pub registry_type: u8,
    pub delegate_count: u64,
    pub delegate_term_epochs: u64,
    pub proposal_submission_cost: u64,
    pub min_on_chain_age_days: u64,
    pub max_votes_per_user: u64,
    pub quadratic_base_cost: u64,
    pub voting_period_epochs: u64,
    pub quorum_votes: u64,
    pub updated_at: u64,
}

/// Event emitted when a delegate is nominated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegateNominatedEvent {
    pub address: String,
    pub profile_id: String,
    pub registry_type: u8,
    pub scheduled_term_start_epoch: u64,
    pub nomination_time: u64,
}

/// Event emitted when a delegate is voted on (upvoted or downvoted)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegateVotedEvent {
    pub target_address: String,
    pub voter_address: String,
    pub registry_type: u8,
    pub is_active_delegate: bool,
    pub upvote: bool,
    pub rated_at: u64,
}

/// Event emitted when a delegate is elected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegateElectedEvent {
    pub address: String,
    pub profile_id: String,
    pub registry_type: u8,
    pub term_start: u64,
    pub term_end: u64,
    pub upvotes: u64,
    pub downvotes: u64,
}

/// Event emitted when a proposal is submitted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalSubmittedEvent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposal_type: u8,
    pub reference_id: Option<String>,
    pub metadata_json: Option<serde_json::Value>,
    pub submitter: String,
    pub submission_time: u64,
    pub reward_pool: u64,
}

/// Event emitted when a delegate votes on a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegateVoteEvent {
    pub proposal_id: String,
    pub delegate_address: String,
    pub approve: bool,
    pub vote_time: u64,
    pub reason: Option<String>,
}

/// Event emitted when a community member votes on a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityVoteEvent {
    pub proposal_id: String,
    pub voter_address: String,
    pub vote_weight: u64,
    pub approve: bool,
    pub vote_time: u64,
    pub vote_cost: u64,
}

/// Event emitted when a proposal is approved for community voting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalApprovedForVotingEvent {
    pub proposal_id: String,
    pub voting_start_time: u64,
    pub voting_end_time: u64,
}

/// Event emitted when a proposal is rejected by delegates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalRejectedEvent {
    pub proposal_id: String,
    pub rejection_time: u64,
}

/// Event emitted when a proposal is rescinded by its owner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalRescindedEvent {
    pub proposal_id: String,
    pub submitter: String,
    pub rescind_time: u64,
    pub refund_amount: u64,
}

/// Event emitted when a proposal is approved after voting period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalApprovedEvent {
    pub proposal_id: String,
    pub approval_time: u64,
}

/// Event emitted when a proposal is rejected by community vote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalRejectedByCommunityEvent {
    pub proposal_id: String,
    pub rejection_time: u64,
}

/// Event emitted when a proposal is implemented
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalImplementedEvent {
    pub proposal_id: String,
    pub implementation_time: u64,
    pub implemented_description: String,
}

/// Event emitted when rewards are distributed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardsDistributedEvent {
    pub proposal_id: String,
    pub recipient_address: String,
    pub amount: u64,
    pub distribution_time: u64,
    pub distribution_type: String,
}

/// Event emitted when an anonymous vote is submitted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymousVoteEvent {
    pub proposal_id: String,
    pub voter: String,
    pub vote_time: u64,
    pub encrypted_vote_data: Vec<u8>,
}

/// Event emitted when vote decryption fails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteDecryptionFailedEvent {
    pub proposal_id: String,
    pub voter: String,
    pub failure_reason: String,
    pub timestamp: u64,
} 