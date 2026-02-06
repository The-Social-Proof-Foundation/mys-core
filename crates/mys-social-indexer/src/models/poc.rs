// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::sql_types::*;
use diesel::{Insertable, QueryableByName, Selectable};
use serde::{Deserialize, Serialize};

/// PoC badge model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::poc_badges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PocBadge {
    #[diesel(sql_type = Varchar)]
    pub badge_id: String,
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Int2)]
    pub media_type: i16,
    #[diesel(sql_type = Varchar)]
    pub issued_by: String,
    #[diesel(sql_type = Int8)]
    pub issued_at: i64,
    #[diesel(sql_type = Bool)]
    pub revoked: bool,
    #[diesel(sql_type = Nullable<Int8>)]
    pub revoked_at: Option<i64>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: DateTime<Utc>,
}

/// New PoC badge model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::poc_badges)]
pub struct NewPocBadge {
    pub badge_id: String,
    pub post_id: String,
    pub media_type: i16,
    pub issued_by: String,
    pub issued_at: i64,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub transaction_id: String,
}

/// PoC revenue redirection model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::poc_revenue_redirections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PocRevenueRedirection {
    #[diesel(sql_type = Varchar)]
    pub redirection_id: String,
    #[diesel(sql_type = Varchar)]
    pub accused_post_id: String,
    #[diesel(sql_type = Varchar)]
    pub original_post_id: String,
    #[diesel(sql_type = Int8)]
    pub redirect_percentage: i64,
    #[diesel(sql_type = Int8)]
    pub similarity_score: i64,
    #[diesel(sql_type = Int8)]
    pub created_at: i64,
    #[diesel(sql_type = Bool)]
    pub removed: bool,
    #[diesel(sql_type = Nullable<Int8>)]
    pub removed_at: Option<i64>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: DateTime<Utc>,
}

/// New PoC revenue redirection model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::poc_revenue_redirections)]
pub struct NewPocRevenueRedirection {
    pub redirection_id: String,
    pub accused_post_id: String,
    pub original_post_id: String,
    pub redirect_percentage: i64,
    pub similarity_score: i64,
    pub created_at: i64,
    pub removed: bool,
    pub removed_at: Option<i64>,
    pub transaction_id: String,
}

/// PoC analysis result model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::poc_analysis_results)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PocAnalysisResult {
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Int2)]
    pub media_type: i16,
    #[diesel(sql_type = Bool)]
    pub similarity_detected: bool,
    #[diesel(sql_type = Int8)]
    pub highest_similarity_score: i64,
    #[diesel(sql_type = Varchar)]
    pub oracle_address: String,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub original_creator: Option<String>,
    #[diesel(sql_type = Int8)]
    pub analysis_timestamp: i64,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = Nullable<Text>)]
    pub reasoning: Option<String>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub evidence_urls: Option<serde_json::Value>,
}

/// New PoC analysis result model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::poc_analysis_results)]
pub struct NewPocAnalysisResult {
    pub post_id: String,
    pub media_type: i16,
    pub similarity_detected: bool,
    pub highest_similarity_score: i64,
    pub oracle_address: String,
    pub original_creator: Option<String>,
    pub analysis_timestamp: i64,
    pub transaction_id: String,
    pub reasoning: Option<String>,
    pub evidence_urls: Option<serde_json::Value>,
}

/// PoC dispute model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::poc_disputes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PocDispute {
    #[diesel(sql_type = Varchar)]
    pub dispute_id: String,
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Varchar)]
    pub disputer: String,
    #[diesel(sql_type = Int2)]
    pub dispute_type: i16,
    #[diesel(sql_type = Text)]
    pub evidence: String,
    #[diesel(sql_type = Int2)]
    pub status: i16,
    #[diesel(sql_type = Int8)]
    pub stake_amount: i64,
    #[diesel(sql_type = Int8)]
    pub voting_start_epoch: i64,
    #[diesel(sql_type = Int8)]
    pub voting_end_epoch: i64,
    #[diesel(sql_type = Nullable<Int2>)]
    pub resolution: Option<i16>,
    #[diesel(sql_type = Nullable<Int2>)]
    pub winning_side: Option<i16>,
    #[diesel(sql_type = Nullable<Int8>)]
    pub total_winning_stake: Option<i64>,
    #[diesel(sql_type = Nullable<Int8>)]
    pub total_losing_stake: Option<i64>,
    #[diesel(sql_type = Int8)]
    pub submitted_at: i64,
    #[diesel(sql_type = Nullable<Int8>)]
    pub resolved_at: Option<i64>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: DateTime<Utc>,
}

/// New PoC dispute model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::poc_disputes)]
pub struct NewPocDispute {
    pub dispute_id: String,
    pub post_id: String,
    pub disputer: String,
    pub dispute_type: i16,
    pub evidence: String,
    pub status: i16,
    pub stake_amount: i64,
    pub voting_start_epoch: i64,
    pub voting_end_epoch: i64,
    pub resolution: Option<i16>,
    pub winning_side: Option<i16>,
    pub total_winning_stake: Option<i64>,
    pub total_losing_stake: Option<i64>,
    pub submitted_at: i64,
    pub resolved_at: Option<i64>,
    pub transaction_id: String,
}

/// PoC dispute vote model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::poc_dispute_votes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PocDisputeVote {
    #[diesel(sql_type = Varchar)]
    pub dispute_id: String,
    #[diesel(sql_type = Varchar)]
    pub voter: String,
    #[diesel(sql_type = Int2)]
    pub vote_choice: i16,
    #[diesel(sql_type = Int8)]
    pub stake_amount: i64,
    #[diesel(sql_type = Int8)]
    pub voted_at: i64,
    #[diesel(sql_type = Bool)]
    pub reward_claimed: bool,
    #[diesel(sql_type = Nullable<Int8>)]
    pub reward_amount: Option<i64>,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: DateTime<Utc>,
}

/// New PoC dispute vote model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::poc_dispute_votes)]
pub struct NewPocDisputeVote {
    pub dispute_id: String,
    pub voter: String,
    pub vote_choice: i16,
    pub stake_amount: i64,
    pub voted_at: i64,
    pub reward_claimed: bool,
    pub reward_amount: Option<i64>,
    pub transaction_id: String,
}

/// PoC configuration model for database
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::poc_configuration)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PocConfiguration {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Int8)]
    pub image_threshold: i64,
    #[diesel(sql_type = Int8)]
    pub video_threshold: i64,
    #[diesel(sql_type = Int8)]
    pub audio_threshold: i64,
    #[diesel(sql_type = Int8)]
    pub revenue_redirect_percentage: i64,
    #[diesel(sql_type = Int8)]
    pub dispute_cost: i64,
    #[diesel(sql_type = Int8)]
    pub dispute_protocol_fee: i64,
    #[diesel(sql_type = Int8)]
    pub min_vote_stake: i64,
    #[diesel(sql_type = Int8)]
    pub max_vote_stake: i64,
    #[diesel(sql_type = Int8)]
    pub voting_duration_epochs: i64,
    #[diesel(sql_type = Int8)]
    pub max_reasoning_length: i64,
    #[diesel(sql_type = Int8)]
    pub max_evidence_urls: i64,
    #[diesel(sql_type = Int8)]
    pub max_votes_per_dispute: i64,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub oracle_address: Option<String>,
    #[diesel(sql_type = Varchar)]
    pub updated_by: String,
    #[diesel(sql_type = Int8)]
    pub updated_at: i64,
    #[diesel(sql_type = Varchar)]
    pub transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    pub time: DateTime<Utc>,
}

/// New PoC configuration model for insertion
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::poc_configuration)]
pub struct NewPocConfiguration {
    pub image_threshold: i64,
    pub video_threshold: i64,
    pub audio_threshold: i64,
    pub revenue_redirect_percentage: i64,
    pub dispute_cost: i64,
    pub dispute_protocol_fee: i64,
    pub min_vote_stake: i64,
    pub max_vote_stake: i64,
    pub voting_duration_epochs: i64,
    pub max_reasoning_length: i64,
    pub max_evidence_urls: i64,
    pub max_votes_per_dispute: i64,
    pub oracle_address: Option<String>,
    pub updated_by: String,
    pub updated_at: i64,
    pub transaction_id: String,
}

// Analytics and reporting structures

/// Daily PoC statistics from continuous aggregate
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PocDailyStats {
    #[diesel(sql_type = Timestamptz)]
    pub day: DateTime<Utc>,
    #[diesel(sql_type = Int8)]
    pub badges_issued: i64,
    #[diesel(sql_type = Int8)]
    pub redirections_created: i64,
    #[diesel(sql_type = Int8)]
    pub disputes_submitted: i64,
    #[diesel(sql_type = Int8)]
    pub votes_cast: i64,
}

/// Hourly PoC statistics from continuous aggregate
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PocHourlyStats {
    #[diesel(sql_type = Timestamptz)]
    pub hour: DateTime<Utc>,
    #[diesel(sql_type = Int8)]
    pub badges_issued_hourly: i64,
    #[diesel(sql_type = Nullable<Float8>)]
    pub avg_similarity_score: Option<f64>,
}

/// PoC analytics data for complex queries
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PocAnalytics {
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub badge_id: Option<String>,
    #[diesel(sql_type = Bool)]
    pub has_badge: bool,
    #[diesel(sql_type = Bool)]
    pub has_redirection: bool,
    #[diesel(sql_type = Nullable<Int8>)]
    pub redirect_percentage: Option<i64>,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub original_creator: Option<String>,
    #[diesel(sql_type = Nullable<Int8>)]
    pub similarity_score: Option<i64>,
    #[diesel(sql_type = Int8)]
    pub created_at: i64,
}

/// Revenue redirection data with post information
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RevenueRedirectionWithPost {
    #[diesel(sql_type = Varchar)]
    pub redirection_id: String,
    #[diesel(sql_type = Varchar)]
    pub accused_post_id: String,
    #[diesel(sql_type = Varchar)]
    pub original_post_id: String,
    #[diesel(sql_type = Int8)]
    pub redirect_percentage: i64,
    #[diesel(sql_type = Int8)]
    pub similarity_score: i64,
    #[diesel(sql_type = Int8)]
    pub created_at: i64,
    #[diesel(sql_type = Bool)]
    pub removed: bool,
    #[diesel(sql_type = Varchar)]
    pub accused_post_owner: String,
    #[diesel(sql_type = Varchar)]
    pub original_post_owner: String,
}

/// Dispute with voting summary
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DisputeWithVotingSummary {
    #[diesel(sql_type = Varchar)]
    pub dispute_id: String,
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Varchar)]
    pub disputer: String,
    #[diesel(sql_type = Int2)]
    pub dispute_type: i16,
    #[diesel(sql_type = Int2)]
    pub status: i16,
    #[diesel(sql_type = Int8)]
    pub stake_amount: i64,
    #[diesel(sql_type = Int8)]
    pub voting_start_epoch: i64,
    #[diesel(sql_type = Int8)]
    pub voting_end_epoch: i64,
    #[diesel(sql_type = Int8)]
    pub submitted_at: i64,
    #[diesel(sql_type = Nullable<Int8>)]
    pub resolved_at: Option<i64>,
    #[diesel(sql_type = Int8)]
    pub total_votes: i64,
    #[diesel(sql_type = Int8)]
    pub uphold_votes: i64,
    #[diesel(sql_type = Int8)]
    pub overturn_votes: i64,
    #[diesel(sql_type = Int8)]
    pub total_uphold_stake: i64,
    #[diesel(sql_type = Int8)]
    pub total_overturn_stake: i64,
}

/// Oracle performance analytics
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OraclePerformance {
    #[diesel(sql_type = Varchar)]
    pub oracle_address: String,
    #[diesel(sql_type = Int8)]
    pub total_analyses: i64,
    #[diesel(sql_type = Int8)]
    pub badges_issued: i64,
    #[diesel(sql_type = Int8)]
    pub redirections_created: i64,
    #[diesel(sql_type = Float8)]
    pub avg_similarity_score: f64,
    #[diesel(sql_type = Int8)]
    pub disputed_decisions: i64,
    #[diesel(sql_type = Int8)]
    pub upheld_disputes: i64,
    #[diesel(sql_type = Int8)]
    pub overturned_disputes: i64,
    #[diesel(sql_type = Float8)]
    pub accuracy_rate: f64,
}

/// Revenue impact analysis
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RevenueImpactAnalysis {
    #[diesel(sql_type = Varchar)]
    pub post_id: String,
    #[diesel(sql_type = Varchar)]
    pub post_owner: String,
    #[diesel(sql_type = Int8)]
    pub total_tips_received: i64,
    #[diesel(sql_type = Int8)]
    pub redirected_amount: i64,
    #[diesel(sql_type = Int8)]
    pub retained_amount: i64,
    #[diesel(sql_type = Float8)]
    pub redirect_percentage: f64,
    #[diesel(sql_type = Nullable<Varchar>)]
    pub original_creator: Option<String>,
    #[diesel(sql_type = Int8)]
    pub similarity_score: i64,
}

// Helper functions for model relationships and queries

impl PocBadge {
    /// Check if the badge is currently active (not revoked)
    pub fn is_active(&self) -> bool {
        !self.revoked
    }

    /// Get media type as string
    pub fn media_type_string(&self) -> &'static str {
        match self.media_type {
            1 => "image",
            2 => "video",
            3 => "audio",
            _ => "unknown",
        }
    }
}

impl PocRevenueRedirection {
    /// Check if the redirection is currently active (not removed)
    pub fn is_active(&self) -> bool {
        !self.removed
    }

    /// Calculate the actual redirect amount from a given tip
    pub fn calculate_redirect_amount(&self, tip_amount: i64) -> i64 {
        (tip_amount * self.redirect_percentage) / 100
    }

    /// Calculate the amount retained by the accused post owner
    pub fn calculate_retained_amount(&self, tip_amount: i64) -> i64 {
        tip_amount - self.calculate_redirect_amount(tip_amount)
    }
}

impl PocDispute {
    /// Check if the dispute is currently in voting phase
    pub fn is_voting_active(&self, current_epoch: i64) -> bool {
        self.status == 1 && // DISPUTE_STATUS_VOTING
        current_epoch >= self.voting_start_epoch &&
        current_epoch <= self.voting_end_epoch
    }

    /// Check if the dispute is resolved
    pub fn is_resolved(&self) -> bool {
        self.status == 2 || self.status == 3 // RESOLVED_UPHELD or RESOLVED_OVERTURNED
    }

    /// Get dispute status as string
    pub fn status_string(&self) -> &'static str {
        match self.status {
            1 => "voting",
            2 => "resolved_upheld",
            3 => "resolved_overturned",
            _ => "unknown",
        }
    }
}

impl PocDisputeVote {
    /// Get vote choice as string
    pub fn vote_choice_string(&self) -> &'static str {
        match self.vote_choice {
            1 => "uphold",
            2 => "overturn",
            _ => "unknown",
        }
    }

    /// Check if the vote is on the winning side
    pub fn is_winning_vote(&self, winning_side: i16) -> bool {
        self.vote_choice == winning_side
    }
}

impl PocConfiguration {
    /// Get the latest configuration (highest id)
    pub fn is_latest(&self, other_id: i32) -> bool {
        self.id >= other_id
    }

    /// Validate that thresholds are within valid ranges (0-100)
    pub fn validate_thresholds(&self) -> bool {
        self.image_threshold >= 0
            && self.image_threshold <= 100
            && self.video_threshold >= 0
            && self.video_threshold <= 100
            && self.audio_threshold >= 0
            && self.audio_threshold <= 100
            && self.revenue_redirect_percentage >= 0
            && self.revenue_redirect_percentage <= 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_poc_badge_creation() {
        let badge = PocBadge {
            badge_id: "test_badge_001".to_string(),
            post_id: "test_post_001".to_string(),
            media_type: 1, // image
            issued_by: "test_oracle_001".to_string(),
            issued_at: Utc::now().timestamp(),
            revoked: false,
            revoked_at: None,
            time: Utc::now(),
            transaction_id: "test_tx_001".to_string(),
        };

        assert_eq!(badge.badge_id, "test_badge_001");
        assert_eq!(badge.post_id, "test_post_001");
        assert_eq!(badge.issued_by, "test_oracle_001");
        assert_eq!(badge.media_type, 1);
        assert_eq!(badge.media_type_string(), "image");
        assert!(!badge.revoked);
        assert!(badge.revoked_at.is_none());
        assert!(badge.is_active());
    }

    #[test]
    fn test_poc_revenue_redirection_creation() {
        let redirection = PocRevenueRedirection {
            redirection_id: "test_redirection_001".to_string(),
            accused_post_id: "test_accused_post_001".to_string(),
            original_post_id: "test_original_post_001".to_string(),
            redirect_percentage: 50,
            similarity_score: 85,
            created_at: Utc::now().timestamp(),
            removed: false,
            removed_at: None,
            time: Utc::now(),
            transaction_id: "test_tx_002".to_string(),
        };

        assert_eq!(redirection.redirection_id, "test_redirection_001");
        assert_eq!(redirection.accused_post_id, "test_accused_post_001");
        assert_eq!(redirection.original_post_id, "test_original_post_001");
        assert_eq!(redirection.redirect_percentage, 50);
        assert_eq!(redirection.similarity_score, 85);
        assert!(!redirection.removed);
        assert!(redirection.removed_at.is_none());
        assert!(redirection.is_active());

        // Test calculation methods
        assert_eq!(redirection.calculate_redirect_amount(1000), 500);
        assert_eq!(redirection.calculate_retained_amount(1000), 500);
    }

    #[test]
    fn test_poc_dispute_creation() {
        let dispute = PocDispute {
            dispute_id: "test_dispute_001".to_string(),
            post_id: "test_post_001".to_string(),
            disputer: "test_disputer_001".to_string(),
            dispute_type: 1,
            evidence: "Test evidence for dispute".to_string(),
            status: 1, // VOTING
            stake_amount: 1000,
            voting_start_epoch: 100,
            voting_end_epoch: 200,
            resolution: None,
            winning_side: None,
            total_winning_stake: None,
            total_losing_stake: None,
            submitted_at: Utc::now().timestamp(),
            resolved_at: None,
            time: Utc::now(),
            transaction_id: "test_tx_003".to_string(),
        };

        assert_eq!(dispute.dispute_id, "test_dispute_001");
        assert_eq!(dispute.post_id, "test_post_001");
        assert_eq!(dispute.disputer, "test_disputer_001");
        assert_eq!(dispute.stake_amount, 1000);
        assert_eq!(dispute.status, 1);
        assert_eq!(dispute.status_string(), "voting");
        assert!(dispute.resolution.is_none());
        assert!(dispute.resolved_at.is_none());
        assert!(!dispute.is_resolved());
        assert!(dispute.is_voting_active(150)); // current epoch between start and end
        assert!(!dispute.is_voting_active(250)); // current epoch after end
    }

    #[test]
    fn test_poc_dispute_vote_creation() {
        let vote = PocDisputeVote {
            dispute_id: "test_dispute_001".to_string(),
            voter: "test_voter_001".to_string(),
            vote_choice: 1, // UPHOLD
            stake_amount: 500,
            voted_at: Utc::now().timestamp(),
            reward_claimed: false,
            reward_amount: None,
            time: Utc::now(),
            transaction_id: "test_tx_004".to_string(),
        };

        assert_eq!(vote.dispute_id, "test_dispute_001");
        assert_eq!(vote.voter, "test_voter_001");
        assert_eq!(vote.stake_amount, 500);
        assert_eq!(vote.vote_choice, 1);
        assert_eq!(vote.vote_choice_string(), "uphold");
        assert!(!vote.reward_claimed);
        assert!(vote.reward_amount.is_none());
        assert!(vote.is_winning_vote(1)); // UPHOLD
        assert!(!vote.is_winning_vote(2)); // OVERTURN
    }

    #[test]
    fn test_poc_configuration_creation() {
        let config = PocConfiguration {
            id: 1,
            image_threshold: 80,
            video_threshold: 75,
            audio_threshold: 85,
            revenue_redirect_percentage: 50,
            dispute_cost: 100,
            dispute_protocol_fee: 10,
            min_vote_stake: 50,
            max_vote_stake: 1000,
            voting_duration_epochs: 7, // 1 week
            max_reasoning_length: 5000,
            max_evidence_urls: 10,
            max_votes_per_dispute: 10000,
            oracle_address: Some("0x1234".to_string()),
            updated_by: "test_admin_001".to_string(),
            updated_at: Utc::now().timestamp(),
            time: Utc::now(),
            transaction_id: "test_tx_005".to_string(),
        };

        assert_eq!(config.id, 1);
        assert_eq!(config.image_threshold, 80);
        assert_eq!(config.video_threshold, 75);
        assert_eq!(config.audio_threshold, 85);
        assert_eq!(config.revenue_redirect_percentage, 50);
        assert_eq!(config.dispute_cost, 100);
        assert_eq!(config.updated_by, "test_admin_001");
        assert!(config.validate_thresholds());
        assert!(config.is_latest(1)); // config id 1 >= 1 is true
        assert!(!config.is_latest(2)); // config id 1 >= 2 is false
    }

    #[test]
    fn test_poc_analysis_result_creation() {
        let analysis = PocAnalysisResult {
            post_id: "test_post_001".to_string(),
            media_type: 2, // video
            similarity_detected: true,
            highest_similarity_score: 85,
            oracle_address: "test_oracle_001".to_string(),
            original_creator: Some("test_creator_001".to_string()),
            analysis_timestamp: Utc::now().timestamp(),
            time: Utc::now(),
            transaction_id: "test_tx_006".to_string(),
            reasoning: None,
            evidence_urls: None,
        };

        assert_eq!(analysis.post_id, "test_post_001");
        assert_eq!(analysis.media_type, 2);
        assert_eq!(analysis.oracle_address, "test_oracle_001");
        assert!(analysis.similarity_detected);
        assert_eq!(analysis.highest_similarity_score, 85);
        assert!(analysis.original_creator.is_some());
        assert_eq!(analysis.original_creator.unwrap(), "test_creator_001");
    }

    #[test]
    fn test_media_type_conversions() {
        // Test media type string conversions
        let image_badge = PocBadge {
            badge_id: "test_badge_001".to_string(),
            post_id: "test_post_001".to_string(),
            media_type: 1,
            issued_by: "test_oracle_001".to_string(),
            issued_at: Utc::now().timestamp(),
            revoked: false,
            revoked_at: None,
            time: Utc::now(),
            transaction_id: "test_tx_001".to_string(),
        };

        let video_badge = PocBadge {
            media_type: 2,
            ..image_badge.clone()
        };

        let audio_badge = PocBadge {
            media_type: 3,
            ..image_badge.clone()
        };

        assert_eq!(image_badge.media_type_string(), "image");
        assert_eq!(video_badge.media_type_string(), "video");
        assert_eq!(audio_badge.media_type_string(), "audio");
    }

    #[test]
    fn test_dispute_status_values() {
        // Test dispute status values using numeric constants
        let voting_dispute = PocDispute {
            dispute_id: "test_dispute_001".to_string(),
            post_id: "test_post_001".to_string(),
            disputer: "test_disputer_001".to_string(),
            dispute_type: 1,
            evidence: "Test evidence".to_string(),
            status: 1, // VOTING
            stake_amount: 1000,
            voting_start_epoch: 100,
            voting_end_epoch: 200,
            resolution: None,
            winning_side: None,
            total_winning_stake: None,
            total_losing_stake: None,
            submitted_at: Utc::now().timestamp(),
            resolved_at: None,
            time: Utc::now(),
            transaction_id: "test_tx_007".to_string(),
        };

        let upheld_dispute = PocDispute {
            status: 2, // RESOLVED_UPHELD
            resolution: Some(2),
            resolved_at: Some(Utc::now().timestamp()),
            ..voting_dispute.clone()
        };

        let overturned_dispute = PocDispute {
            status: 3, // RESOLVED_OVERTURNED
            resolution: Some(3),
            resolved_at: Some(Utc::now().timestamp()),
            ..voting_dispute.clone()
        };

        assert_eq!(voting_dispute.status_string(), "voting");
        assert_eq!(upheld_dispute.status_string(), "resolved_upheld");
        assert_eq!(overturned_dispute.status_string(), "resolved_overturned");

        assert!(!voting_dispute.is_resolved());
        assert!(upheld_dispute.is_resolved());
        assert!(overturned_dispute.is_resolved());
    }

    #[test]
    fn test_vote_choice_values() {
        // Test vote choice values using numeric constants
        let uphold_vote = PocDisputeVote {
            dispute_id: "test_dispute_001".to_string(),
            voter: "test_voter_001".to_string(),
            vote_choice: 1, // UPHOLD
            stake_amount: 500,
            voted_at: Utc::now().timestamp(),
            reward_claimed: false,
            reward_amount: None,
            time: Utc::now(),
            transaction_id: "test_tx_008".to_string(),
        };

        let overturn_vote = PocDisputeVote {
            vote_choice: 2, // OVERTURN
            ..uphold_vote.clone()
        };

        assert_eq!(uphold_vote.vote_choice_string(), "uphold");
        assert_eq!(overturn_vote.vote_choice_string(), "overturn");

        assert!(uphold_vote.is_winning_vote(1)); // UPHOLD wins
        assert!(!uphold_vote.is_winning_vote(2)); // OVERTURN wins
        assert!(!overturn_vote.is_winning_vote(1)); // UPHOLD wins
        assert!(overturn_vote.is_winning_vote(2)); // OVERTURN wins
    }

    #[test]
    fn test_redirection_calculations() {
        let redirection = PocRevenueRedirection {
            redirection_id: "test_redirection_001".to_string(),
            accused_post_id: "test_accused_post_001".to_string(),
            original_post_id: "test_original_post_001".to_string(),
            redirect_percentage: 75, // 75% redirect
            similarity_score: 90,
            created_at: Utc::now().timestamp(),
            removed: false,
            removed_at: None,
            time: Utc::now(),
            transaction_id: "test_tx_009".to_string(),
        };

        // Test calculations with different tip amounts
        assert_eq!(redirection.calculate_redirect_amount(1000), 750);
        assert_eq!(redirection.calculate_retained_amount(1000), 250);

        assert_eq!(redirection.calculate_redirect_amount(500), 375);
        assert_eq!(redirection.calculate_retained_amount(500), 125);

        assert_eq!(redirection.calculate_redirect_amount(0), 0);
        assert_eq!(redirection.calculate_retained_amount(0), 0);
    }
}
