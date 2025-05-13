// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    delegates, governance_events, governance_registries, nominated_delegates, 
    proposals, delegate_ratings, delegate_votes, community_votes, reward_distributions,
};

/// Model for governance_registries table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = governance_registries)]
pub struct GovernanceRegistry {
    pub id: i32,
    pub registry_type: i16,
    pub delegate_count: i64,
    pub delegate_term_epochs: i64,
    pub proposal_submission_cost: i64,
    pub min_on_chain_age_days: i64,
    pub max_votes_per_user: i64,
    pub quadratic_base_cost: i64,
    pub voting_period_epochs: i64,
    pub quorum_votes: i64,
    pub updated_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for creation of new governance registry
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = governance_registries)]
pub struct NewGovernanceRegistry {
    pub registry_type: i16,
    pub delegate_count: i64,
    pub delegate_term_epochs: i64,
    pub proposal_submission_cost: i64,
    pub min_on_chain_age_days: i64,
    pub max_votes_per_user: i64,
    pub quadratic_base_cost: i64,
    pub voting_period_epochs: i64,
    pub quorum_votes: i64,
    pub updated_at: i64,
    pub transaction_id: String,
}

/// Model for delegates table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = delegates)]
#[diesel(primary_key(id, time))]
pub struct Delegate {
    pub id: i32,
    pub address: String,
    pub profile_id: String,
    pub registry_type: i16,
    pub upvotes: i64,
    pub downvotes: i64,
    pub proposals_reviewed: i64,
    pub proposals_submitted: i64,
    pub sided_winning_proposals: i64,
    pub sided_losing_proposals: i64,
    pub term_start: i64,
    pub term_end: i64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for creation of new delegate
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegates)]
pub struct NewDelegate {
    pub address: String,
    pub profile_id: String,
    pub registry_type: i16,
    pub upvotes: i64,
    pub downvotes: i64,
    pub proposals_reviewed: i64,
    pub proposals_submitted: i64,
    pub sided_winning_proposals: i64,
    pub sided_losing_proposals: i64,
    pub term_start: i64,
    pub term_end: i64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub transaction_id: String,
}

/// Model for nominated_delegates table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = nominated_delegates)]
#[diesel(primary_key(id, time))]
pub struct NominatedDelegate {
    pub id: i32,
    pub address: String,
    pub profile_id: String,
    pub registry_type: i16,
    pub upvotes: i64,
    pub downvotes: i64,
    pub scheduled_term_start_epoch: i64,
    pub nomination_time: i64,
    pub status: i16,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for creation of new nominated delegate
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = nominated_delegates)]
pub struct NewNominatedDelegate {
    pub address: String,
    pub profile_id: String,
    pub registry_type: i16,
    pub upvotes: i64,
    pub downvotes: i64,
    pub scheduled_term_start_epoch: i64,
    pub nomination_time: i64,
    pub status: i16,
    pub transaction_id: String,
}

/// Model for proposals table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = proposals)]
#[diesel(primary_key(id, time))]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposal_type: i16,
    pub reference_id: Option<String>,
    pub metadata_json: Option<serde_json::Value>,
    pub submitter: String,
    pub submission_time: i64,
    pub delegate_approval_count: i64,
    pub delegate_rejection_count: i64,
    pub community_votes_for: i64,
    pub community_votes_against: i64,
    pub status: i16,
    pub voting_start_time: Option<i64>,
    pub voting_end_time: Option<i64>,
    pub reward_pool: i64,
    pub implemented_description: Option<String>,
    pub implementation_time: Option<i64>,
    pub rescind_time: Option<i64>,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for creation of new proposal
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = proposals)]
pub struct NewProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposal_type: i16,
    pub reference_id: Option<String>,
    pub metadata_json: Option<serde_json::Value>,
    pub submitter: String,
    pub submission_time: i64,
    pub status: i16,
    pub reward_pool: i64,
    pub transaction_id: String,
}

/// Model for delegate_ratings table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = delegate_ratings)]
#[diesel(primary_key(id, time))]
pub struct DelegateRating {
    pub id: i32,
    pub target_address: String,
    pub voter_address: String,
    pub registry_type: i16,
    pub is_active_delegate: bool,
    pub upvote: bool,
    pub rated_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for creation of new delegate rating
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegate_ratings)]
pub struct NewDelegateRating {
    pub target_address: String,
    pub voter_address: String,
    pub registry_type: i16,
    pub is_active_delegate: bool,
    pub upvote: bool,
    pub rated_at: i64,
    pub transaction_id: String,
}

/// Model for delegate_votes table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = delegate_votes)]
#[diesel(primary_key(id, time))]
pub struct DelegateVote {
    pub id: i32,
    pub proposal_id: String,
    pub delegate_address: String,
    pub approve: bool,
    pub vote_time: i64,
    pub reason: Option<String>,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for creation of new delegate vote
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegate_votes)]
pub struct NewDelegateVote {
    pub proposal_id: String,
    pub delegate_address: String,
    pub approve: bool,
    pub vote_time: i64,
    pub reason: Option<String>,
    pub transaction_id: String,
}

/// Model for community_votes table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = community_votes)]
#[diesel(primary_key(id, time))]
pub struct CommunityVote {
    pub id: i32,
    pub proposal_id: String,
    pub voter_address: String,
    pub vote_weight: i64,
    pub approve: bool,
    pub vote_time: i64,
    pub vote_cost: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for creation of new community vote
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = community_votes)]
pub struct NewCommunityVote {
    pub proposal_id: String,
    pub voter_address: String,
    pub vote_weight: i64,
    pub approve: bool,
    pub vote_time: i64,
    pub vote_cost: i64,
    pub transaction_id: String,
}

/// Model for reward_distributions table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = reward_distributions)]
#[diesel(primary_key(id, time))]
pub struct RewardDistribution {
    pub id: i32,
    pub proposal_id: String,
    pub recipient_address: String,
    pub amount: i64,
    pub distribution_time: i64,
    pub distribution_type: Option<String>,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for creation of new reward distribution
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = reward_distributions)]
pub struct NewRewardDistribution {
    pub proposal_id: String,
    pub recipient_address: String,
    pub amount: i64,
    pub distribution_time: i64,
    pub distribution_type: Option<String>,
    pub transaction_id: String,
}

/// Model for governance_events table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = governance_events)]
pub struct GovernanceEvent {
    pub id: i32,
    pub event_type: String,
    pub registry_type: i16,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: DateTime<Utc>,
}

/// Model for creation of new governance event
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = governance_events)]
pub struct NewGovernanceEvent {
    pub event_type: String,
    pub registry_type: i16,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: DateTime<Utc>,
} 