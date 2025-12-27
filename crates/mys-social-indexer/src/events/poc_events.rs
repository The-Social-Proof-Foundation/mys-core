// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;

// Import specific PoC event types
use crate::events::poc_event_types::{
    AnalysisSubmittedEvent, DisputeVoteCastEvent, PocBadgeIssuedEvent, PocConfigUpdatedEvent,
    PocDisputeResolvedEvent, PocDisputeSubmittedEvent, RevenueRedirectionActivatedEvent,
    TokenPoolSyncNeededEvent, VotingRewardClaimedEvent,
};

// Import PoC model types (will be created in Phase 3)
use crate::models::poc::{
    NewPocAnalysisResult, NewPocBadge, NewPocConfiguration, NewPocDispute, NewPocDisputeVote,
    NewPocRevenueRedirection,
};

// Model conversion impl for AnalysisSubmittedEvent
impl AnalysisSubmittedEvent {
    pub fn into_model(&self) -> Result<NewPocAnalysisResult> {
        Ok(NewPocAnalysisResult {
            post_id: self.post_id.clone(),
            media_type: self.media_type as i16,
            similarity_detected: self.similarity_detected,
            highest_similarity_score: self.highest_similarity_score as i64,
            oracle_address: self.oracle_address.clone(),
            original_creator: None,
            analysis_timestamp: self.timestamp as i64,
            transaction_id: "".to_string(),
            reasoning: self.reasoning.clone(),
            evidence_urls: self.evidence_urls.as_ref().map(|urls| serde_json::json!(urls)),
        })
    }
}

// Model conversion impl for PocBadgeIssuedEvent
impl PocBadgeIssuedEvent {
    pub fn into_model(&self) -> Result<NewPocBadge> {
        Ok(NewPocBadge {
            badge_id: self.badge_id.clone(),
            post_id: self.post_id.clone(),
            media_type: self.media_type as i16,
            issued_by: self.issued_by.clone(),
            issued_at: self.timestamp as i64,
            revoked: false,
            revoked_at: None,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }
}

// Model conversion impl for RevenueRedirectionActivatedEvent
impl RevenueRedirectionActivatedEvent {
    pub fn into_model(&self) -> Result<NewPocRevenueRedirection> {
        Ok(NewPocRevenueRedirection {
            redirection_id: self.redirection_id.clone(),
            accused_post_id: self.accused_post_id.clone(),
            original_post_id: self.original_post_id.clone(),
            redirect_percentage: self.redirect_percentage as i64,
            similarity_score: self.similarity_score as i64,
            created_at: self.timestamp as i64,
            removed: false,
            removed_at: None,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }
}

// Model conversion impl for PocDisputeSubmittedEvent
impl PocDisputeSubmittedEvent {
    pub fn into_model(&self, evidence: String) -> Result<NewPocDispute> {
        Ok(NewPocDispute {
            dispute_id: self.dispute_id.clone(),
            post_id: self.post_id.clone(),
            disputer: self.disputer.clone(),
            dispute_type: self.dispute_type as i16,
            evidence,
            status: crate::events::poc_event_types::DISPUTE_STATUS_VOTING as i16,
            stake_amount: self.stake_amount as i64,
            voting_start_epoch: self.voting_start_epoch as i64,
            voting_end_epoch: self.voting_end_epoch as i64,
            resolution: None,
            winning_side: None,
            total_winning_stake: None,
            total_losing_stake: None,
            submitted_at: self.timestamp as i64,
            resolved_at: None,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }
}

// Model conversion impl for DisputeVoteCastEvent
impl DisputeVoteCastEvent {
    pub fn into_model(&self) -> Result<NewPocDisputeVote> {
        Ok(NewPocDisputeVote {
            dispute_id: self.dispute_id.clone(),
            voter: self.voter.clone(),
            vote_choice: self.vote_choice as i16,
            stake_amount: self.stake_amount as i64,
            voted_at: self.timestamp as i64,
            reward_claimed: false,
            reward_amount: None,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }
}

// Model conversion impl for PocDisputeResolvedEvent
impl PocDisputeResolvedEvent {
    pub fn get_dispute_update_fields(&self) -> (i16, i16, i64, i64, i64) {
        (
            self.resolution as i16,
            self.winning_side as i16,
            self.total_winning_stake as i64,
            self.total_losing_stake as i64,
            self.timestamp as i64,
        )
    }

    pub fn should_revoke_badge(&self) -> bool {
        self.badge_revoked
    }

    pub fn should_remove_redirection(&self) -> bool {
        self.redirection_removed
    }
}

// Model conversion impl for VotingRewardClaimedEvent
impl VotingRewardClaimedEvent {
    pub fn get_reward_update_fields(&self) -> (bool, i64) {
        (true, self.reward_amount as i64)
    }
}

// Model conversion impl for PocConfigUpdatedEvent
impl PocConfigUpdatedEvent {
    pub fn into_model(&self) -> Result<NewPocConfiguration> {
        Ok(NewPocConfiguration {
            image_threshold: self.image_threshold as i64,
            video_threshold: self.video_threshold as i64,
            audio_threshold: self.audio_threshold as i64,
            revenue_redirect_percentage: self.revenue_redirect_percentage as i64,
            dispute_cost: self.dispute_cost as i64,
            dispute_protocol_fee: 0, // Not included in this event, will use default
            min_vote_stake: self.min_vote_stake as i64,
            max_vote_stake: self.max_vote_stake as i64,
            voting_duration_epochs: self.voting_duration_epochs as i64,
            max_reasoning_length: self.max_reasoning_length as i64,
            max_evidence_urls: self.max_evidence_urls as i64,
            updated_by: self.updated_by.clone(),
            updated_at: self.timestamp as i64,
            transaction_id: "".to_string(), // Will be set by handler
        })
    }
}

// Model conversion impl for TokenPoolSyncNeededEvent
impl TokenPoolSyncNeededEvent {
    pub fn get_post_id(&self) -> &str {
        &self.post_id
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// Utility functions for PoC event validation and parsing
pub mod validation {
    use crate::events::poc_event_types::{
        DISPUTE_STATUS_RESOLVED_OVERTURNED, DISPUTE_STATUS_RESOLVED_UPHELD, DISPUTE_STATUS_VOTING,
        MEDIA_TYPE_AUDIO, MEDIA_TYPE_IMAGE, MEDIA_TYPE_VIDEO, VOTE_OVERTURN, VOTE_UPHOLD,
    };

    /// Validate media type
    pub fn is_valid_media_type(media_type: u8) -> bool {
        matches!(
            media_type,
            MEDIA_TYPE_IMAGE | MEDIA_TYPE_VIDEO | MEDIA_TYPE_AUDIO
        )
    }

    /// Validate vote choice
    pub fn is_valid_vote_choice(vote_choice: u8) -> bool {
        matches!(vote_choice, VOTE_UPHOLD | VOTE_OVERTURN)
    }

    /// Validate dispute status
    pub fn is_valid_dispute_status(status: u8) -> bool {
        matches!(
            status,
            DISPUTE_STATUS_VOTING
                | DISPUTE_STATUS_RESOLVED_UPHELD
                | DISPUTE_STATUS_RESOLVED_OVERTURNED
        )
    }

    /// Validate similarity score (0-100 as percentage)
    pub fn is_valid_similarity_score(score: u64) -> bool {
        score <= 100
    }

    /// Validate redirect percentage (0-100)
    pub fn is_valid_redirect_percentage(percentage: u64) -> bool {
        percentage <= 100
    }

    /// Validate threshold value (0-100)
    pub fn is_valid_threshold(threshold: u64) -> bool {
        threshold <= 100
    }
}

/// Error handling for PoC events
#[derive(Debug, thiserror::Error)]
pub enum PocEventError {
    #[error("Invalid media type: {0}")]
    InvalidMediaType(u8),

    #[error("Invalid vote choice: {0}")]
    InvalidVoteChoice(u8),

    #[error("Invalid dispute status: {0}")]
    InvalidDisputeStatus(u8),

    #[error("Invalid similarity score: {0} (must be 0-100)")]
    InvalidSimilarityScore(u64),

    #[error("Invalid redirect percentage: {0} (must be 0-100)")]
    InvalidRedirectPercentage(u64),

    #[error("Invalid threshold: {0} (must be 0-100)")]
    InvalidThreshold(u64),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Event parsing error: {0}")]
    ParseError(String),
}

/// Comprehensive event validation
pub fn validate_analysis_submitted_event(
    event: &AnalysisSubmittedEvent,
) -> Result<(), PocEventError> {
    if !validation::is_valid_media_type(event.media_type) {
        return Err(PocEventError::InvalidMediaType(event.media_type));
    }

    if !validation::is_valid_similarity_score(event.highest_similarity_score) {
        return Err(PocEventError::InvalidSimilarityScore(
            event.highest_similarity_score,
        ));
    }

    if event.post_id.is_empty() {
        return Err(PocEventError::MissingField("post_id".to_string()));
    }

    if event.oracle_address.is_empty() {
        return Err(PocEventError::MissingField("oracle_address".to_string()));
    }

    Ok(())
}

pub fn validate_badge_issued_event(event: &PocBadgeIssuedEvent) -> Result<(), PocEventError> {
    if !validation::is_valid_media_type(event.media_type) {
        return Err(PocEventError::InvalidMediaType(event.media_type));
    }

    if event.badge_id.is_empty() {
        return Err(PocEventError::MissingField("badge_id".to_string()));
    }

    if event.post_id.is_empty() {
        return Err(PocEventError::MissingField("post_id".to_string()));
    }

    if event.issued_by.is_empty() {
        return Err(PocEventError::MissingField("issued_by".to_string()));
    }

    Ok(())
}

pub fn validate_redirection_activated_event(
    event: &RevenueRedirectionActivatedEvent,
) -> Result<(), PocEventError> {
    if !validation::is_valid_redirect_percentage(event.redirect_percentage) {
        return Err(PocEventError::InvalidRedirectPercentage(
            event.redirect_percentage,
        ));
    }

    if !validation::is_valid_similarity_score(event.similarity_score) {
        return Err(PocEventError::InvalidSimilarityScore(
            event.similarity_score,
        ));
    }

    if event.redirection_id.is_empty() {
        return Err(PocEventError::MissingField("redirection_id".to_string()));
    }

    if event.accused_post_id.is_empty() {
        return Err(PocEventError::MissingField("accused_post_id".to_string()));
    }

    if event.original_post_id.is_empty() {
        return Err(PocEventError::MissingField("original_post_id".to_string()));
    }

    Ok(())
}

pub fn validate_dispute_submitted_event(
    event: &PocDisputeSubmittedEvent,
) -> Result<(), PocEventError> {
    if event.dispute_id.is_empty() {
        return Err(PocEventError::MissingField("dispute_id".to_string()));
    }

    if event.post_id.is_empty() {
        return Err(PocEventError::MissingField("post_id".to_string()));
    }

    if event.disputer.is_empty() {
        return Err(PocEventError::MissingField("disputer".to_string()));
    }

    if event.voting_start_epoch >= event.voting_end_epoch {
        return Err(PocEventError::ParseError(
            "Invalid voting epoch range".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_vote_cast_event(event: &DisputeVoteCastEvent) -> Result<(), PocEventError> {
    if !validation::is_valid_vote_choice(event.vote_choice) {
        return Err(PocEventError::InvalidVoteChoice(event.vote_choice));
    }

    if event.dispute_id.is_empty() {
        return Err(PocEventError::MissingField("dispute_id".to_string()));
    }

    if event.voter.is_empty() {
        return Err(PocEventError::MissingField("voter".to_string()));
    }

    if event.stake_amount == 0 {
        return Err(PocEventError::ParseError(
            "Stake amount must be greater than 0".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_config_updated_event(event: &PocConfigUpdatedEvent) -> Result<(), PocEventError> {
    if !validation::is_valid_threshold(event.image_threshold) {
        return Err(PocEventError::InvalidThreshold(event.image_threshold));
    }

    if !validation::is_valid_threshold(event.video_threshold) {
        return Err(PocEventError::InvalidThreshold(event.video_threshold));
    }

    if !validation::is_valid_threshold(event.audio_threshold) {
        return Err(PocEventError::InvalidThreshold(event.audio_threshold));
    }

    if !validation::is_valid_redirect_percentage(event.revenue_redirect_percentage) {
        return Err(PocEventError::InvalidRedirectPercentage(
            event.revenue_redirect_percentage,
        ));
    }

    if event.updated_by.is_empty() {
        return Err(PocEventError::MissingField("updated_by".to_string()));
    }

    if event.min_vote_stake > event.max_vote_stake {
        return Err(PocEventError::ParseError(
            "Min vote stake cannot be greater than max vote stake".to_string(),
        ));
    }

    if event.max_reasoning_length == 0 {
        return Err(PocEventError::ParseError(
            "max_reasoning_length must be greater than 0".to_string(),
        ));
    }

    if event.max_evidence_urls == 0 {
        return Err(PocEventError::ParseError(
            "max_evidence_urls must be greater than 0".to_string(),
        ));
    }

    Ok(())
}
