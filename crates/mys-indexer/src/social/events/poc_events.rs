// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;

// Import specific PoC event types
use crate::social::events::poc_event_types::{
    AnalysisSubmittedEvent, DisputeVoteCastEvent, PocBadgeIssuedEvent, PocConfigUpdatedEvent,
    PocDisputeResolvedEvent, PocDisputeSubmittedEvent, RevenueRedirectionActivatedEvent,
    TokenPoolSyncNeededEvent, VotingRewardClaimedEvent,
};

// Import PoC model types (will be created in Phase 3)
use crate::social::models::poc::{
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
            status: crate::social::events::poc_event_types::DISPUTE_STATUS_VOTING as i16,
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
    /// Convert to database model using timestamp_ms from BlockchainEvent (in milliseconds)
    /// Uses oracle_address and dispute_protocol_fee directly from the event
    pub fn into_model(&self, timestamp_ms: u64) -> Result<NewPocConfiguration> {
        Ok(NewPocConfiguration {
            image_threshold: self.image_threshold as i64,
            video_threshold: self.video_threshold as i64,
            audio_threshold: self.audio_threshold as i64,
            revenue_redirect_percentage: self.revenue_redirect_percentage as i64,
            dispute_cost: self.dispute_cost as i64,
            dispute_protocol_fee: self.dispute_protocol_fee as i64,
            min_vote_stake: self.min_vote_stake as i64,
            max_vote_stake: self.max_vote_stake as i64,
            voting_duration_epochs: self.voting_duration_epochs as i64,
            max_reasoning_length: self.max_reasoning_length as i64,
            max_evidence_urls: self.max_evidence_urls as i64,
            max_votes_per_dispute: self.max_votes_per_dispute as i64,
            oracle_address: Some(self.oracle_address.clone()),
            updated_by: self.updated_by.clone(),
            updated_at: timestamp_ms as i64, // Use timestamp_ms from BlockchainEvent (milliseconds)
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
    use crate::social::events::poc_event_types::{
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

    if event.max_votes_per_dispute == 0 {
        return Err(PocEventError::ParseError(
            "max_votes_per_dispute must be greater than 0".to_string(),
        ));
    }

    Ok(())
}

// =============================================================================
// PROCESS FUNCTIONS FOR CHECKPOINT PROCESSOR
// =============================================================================

use anyhow::anyhow;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::social::db::DbConnection;
use crate::social::schema::{
    poc_analysis_results, poc_badges, poc_revenue_redirections,
    poc_disputes, poc_dispute_votes, poc_configuration,
};

/// Process an AnalysisSubmittedEvent and insert into the database
pub async fn process_analysis_submitted_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
    tx: String,
) -> Result<()> {
    let event: AnalysisSubmittedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse AnalysisSubmittedEvent: {}", e))?;

    // Validate the event
    validate_analysis_submitted_event(&event)
        .map_err(|e| anyhow!("Validation failed: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = tx;

    diesel::insert_into(poc_analysis_results::table)
        .values(&model)
        .on_conflict(poc_analysis_results::post_id)
        .do_update()
        .set((
            poc_analysis_results::media_type.eq(&model.media_type),
            poc_analysis_results::similarity_detected.eq(&model.similarity_detected),
            poc_analysis_results::highest_similarity_score.eq(&model.highest_similarity_score),
            poc_analysis_results::oracle_address.eq(&model.oracle_address),
            poc_analysis_results::original_creator.eq(&model.original_creator),
            poc_analysis_results::analysis_timestamp.eq(&model.analysis_timestamp),
            poc_analysis_results::transaction_id.eq(&model.transaction_id),
            poc_analysis_results::reasoning.eq(&model.reasoning),
            poc_analysis_results::evidence_urls.eq(&model.evidence_urls),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert PoC analysis result: {}", e))?;

    tracing::info!("Processed AnalysisSubmittedEvent for post {} (event: {})", event.post_id, event_id);
    Ok(())
}

/// Process a PocBadgeIssuedEvent and insert into the database
pub async fn process_poc_badge_issued_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
    tx: String,
) -> Result<()> {
    let event: PocBadgeIssuedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PocBadgeIssuedEvent: {}", e))?;

    // Validate the event
    validate_badge_issued_event(&event)
        .map_err(|e| anyhow!("Validation failed: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = tx;

    diesel::insert_into(poc_badges::table)
        .values(&model)
        .on_conflict(poc_badges::badge_id)
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert PoC badge: {}", e))?;

    tracing::info!("Processed PocBadgeIssuedEvent: badge {} for post {} (event: {})",
        event.badge_id, event.post_id, event_id);
    Ok(())
}

/// Process a RevenueRedirectionActivatedEvent and insert into the database
pub async fn process_revenue_redirection_activated_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
    tx: String,
) -> Result<()> {
    let event: RevenueRedirectionActivatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse RevenueRedirectionActivatedEvent: {}", e))?;

    // Validate the event
    validate_redirection_activated_event(&event)
        .map_err(|e| anyhow!("Validation failed: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = tx;

    diesel::insert_into(poc_revenue_redirections::table)
        .values(&model)
        .on_conflict(poc_revenue_redirections::redirection_id)
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert PoC revenue redirection: {}", e))?;

    tracing::info!("Processed RevenueRedirectionActivatedEvent: {} -> {} (event: {})",
        event.accused_post_id, event.original_post_id, event_id);
    Ok(())
}

/// Process a PocDisputeSubmittedEvent and insert into the database
pub async fn process_poc_dispute_submitted_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
    tx: String,
) -> Result<()> {
    let event: PocDisputeSubmittedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PocDisputeSubmittedEvent: {}", e))?;

    // Validate the event
    validate_dispute_submitted_event(&event)
        .map_err(|e| anyhow!("Validation failed: {}", e))?;

    // Extract evidence from raw data if present
    let evidence = data.get("evidence")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let mut model = event.into_model(evidence)?;
    model.transaction_id = tx;

    diesel::insert_into(poc_disputes::table)
        .values(&model)
        .on_conflict(poc_disputes::dispute_id)
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert PoC dispute: {}", e))?;

    tracing::info!("Processed PocDisputeSubmittedEvent: dispute {} for post {} (event: {})",
        event.dispute_id, event.post_id, event_id);
    Ok(())
}

/// Process a DisputeVoteCastEvent and insert into the database
pub async fn process_dispute_vote_cast_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
    tx: String,
) -> Result<()> {
    let event: DisputeVoteCastEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse DisputeVoteCastEvent: {}", e))?;

    // Validate the event
    validate_vote_cast_event(&event)
        .map_err(|e| anyhow!("Validation failed: {}", e))?;

    let mut model = event.into_model()?;
    model.transaction_id = tx;

    diesel::insert_into(poc_dispute_votes::table)
        .values(&model)
        .on_conflict((poc_dispute_votes::dispute_id, poc_dispute_votes::voter))
        .do_update()
        .set((
            poc_dispute_votes::vote_choice.eq(&model.vote_choice),
            poc_dispute_votes::stake_amount.eq(&model.stake_amount),
            poc_dispute_votes::voted_at.eq(&model.voted_at),
            poc_dispute_votes::reward_claimed.eq(&model.reward_claimed),
            poc_dispute_votes::reward_amount.eq(&model.reward_amount),
            poc_dispute_votes::transaction_id.eq(&model.transaction_id),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert PoC dispute vote: {}", e))?;

    tracing::info!("Processed DisputeVoteCastEvent: voter {} on dispute {} (event: {})",
        event.voter, event.dispute_id, event_id);
    Ok(())
}

/// Process a PocDisputeResolvedEvent and update the database
pub async fn process_poc_dispute_resolved_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
    _tx: String,
) -> Result<()> {
    let event: PocDisputeResolvedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PocDisputeResolvedEvent: {}", e))?;

    let (resolution, winning_side, total_winning_stake, total_losing_stake, resolved_at) =
        event.get_dispute_update_fields();

    // Update dispute status
    diesel::update(poc_disputes::table)
        .filter(poc_disputes::dispute_id.eq(&event.dispute_id))
        .set((
            poc_disputes::status.eq(resolution),
            poc_disputes::resolution.eq(Some(resolution)),
            poc_disputes::winning_side.eq(Some(winning_side)),
            poc_disputes::total_winning_stake.eq(Some(total_winning_stake)),
            poc_disputes::total_losing_stake.eq(Some(total_losing_stake)),
            poc_disputes::resolved_at.eq(Some(resolved_at)),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to update PoC dispute: {}", e))?;

    // If badge should be revoked
    if event.should_revoke_badge() {
        diesel::update(poc_badges::table)
            .filter(poc_badges::post_id.eq(&event.post_id))
            .set((
                poc_badges::revoked.eq(true),
                poc_badges::revoked_at.eq(Some(event.timestamp as i64)),
            ))
            .execute(conn)
            .await
            .map_err(|e| anyhow!("Failed to revoke PoC badge: {}", e))?;
    }

    // If redirection should be removed
    if event.should_remove_redirection() {
        diesel::update(poc_revenue_redirections::table)
            .filter(poc_revenue_redirections::accused_post_id.eq(&event.post_id))
            .set((
                poc_revenue_redirections::removed.eq(true),
                poc_revenue_redirections::removed_at.eq(Some(event.timestamp as i64)),
            ))
            .execute(conn)
            .await
            .map_err(|e| anyhow!("Failed to remove PoC redirection: {}", e))?;
    }

    tracing::info!("Processed PocDisputeResolvedEvent: dispute {} resolved (event: {})",
        event.dispute_id, event_id);
    Ok(())
}

/// Process a VotingRewardClaimedEvent and update the database
pub async fn process_voting_reward_claimed_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
    _tx: String,
) -> Result<()> {
    let event: VotingRewardClaimedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse VotingRewardClaimedEvent: {}", e))?;

    let (reward_claimed, reward_amount) = event.get_reward_update_fields();

    diesel::update(poc_dispute_votes::table)
        .filter(poc_dispute_votes::dispute_id.eq(&event.dispute_id))
        .filter(poc_dispute_votes::voter.eq(&event.voter))
        .set((
            poc_dispute_votes::reward_claimed.eq(reward_claimed),
            poc_dispute_votes::reward_amount.eq(Some(reward_amount)),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to update PoC vote reward: {}", e))?;

    tracing::info!("Processed VotingRewardClaimedEvent: voter {} claimed {} (event: {})",
        event.voter, event.reward_amount, event_id);
    Ok(())
}

/// Process a PocConfigUpdatedEvent and insert into the database
pub async fn process_poc_config_updated_event(
    conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
    timestamp_ms: u64,
    tx: String,
) -> Result<()> {
    let event: PocConfigUpdatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PocConfigUpdatedEvent: {}", e))?;

    // Validate the event
    validate_config_updated_event(&event)
        .map_err(|e| anyhow!("Validation failed: {}", e))?;

    let mut model = event.into_model(timestamp_ms)?;
    model.transaction_id = tx;

    diesel::insert_into(poc_configuration::table)
        .values(&model)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert PoC configuration: {}", e))?;

    tracing::info!("Processed PocConfigUpdatedEvent by {} (event: {})",
        event.updated_by, event_id);
    Ok(())
}

/// Process a TokenPoolSyncNeededEvent (just logs it for now)
pub async fn process_token_pool_sync_needed_event(
    _conn: &mut DbConnection,
    data: &serde_json::Value,
    event_id: &str,
    _tx: String,
) -> Result<()> {
    let event: TokenPoolSyncNeededEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse TokenPoolSyncNeededEvent: {}", e))?;

    // This event is informational - signals that SPT pool needs to sync PoC status
    tracing::info!("Processed TokenPoolSyncNeededEvent for post {} at {} (event: {})",
        event.get_post_id(), event.get_timestamp(), event_id);
    Ok(())
}
