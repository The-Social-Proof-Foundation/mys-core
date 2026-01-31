// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Checkpoint-based social event processor
//!
//! This module implements the Worker trait from mys-data-ingestion-core to process
//! social events from checkpoints. It supports two deployment modes:
//!
//! 1. **Embedded Mode**: Registered as a worker in mys-indexer's IndexerExecutor
//! 2. **Standalone Mode**: Runs independently with CheckpointReader
//!
//! Events are filtered by package address and routed to appropriate handlers.

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, error, info};

use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use mys_data_ingestion_core::Worker;
use mys_rpc_api::CheckpointData;
use mys_types::base_types::ObjectID;

use crate::social::db::Database;
use crate::social::models::indexer::NewIndexerProgress;
use crate::social::schema;

/// Processes social events from checkpoint data
///
/// Implements the Worker trait for integration with mys-indexer's checkpoint pipeline.
pub struct SocialCheckpointProcessor {
    db: Arc<Database>,
    package_address: ObjectID,
}

impl SocialCheckpointProcessor {
    /// Create a new SocialCheckpointProcessor
    pub fn new(db: Arc<Database>, package_address: ObjectID) -> Self {
        Self { db, package_address }
    }

    /// Get the package address being monitored
    pub fn package_address(&self) -> &ObjectID {
        &self.package_address
    }
}

#[async_trait]
impl Worker for SocialCheckpointProcessor {
    type Result = ();

    async fn process_checkpoint(&self, data: &CheckpointData) -> Result<()> {
        let checkpoint_seq = data.checkpoint_summary.sequence_number;
        let mut event_count = 0;

        // Get a database connection for event processing
        let mut conn = self.db.get_connection().await?;

        // Process events from all transactions in this checkpoint
        for tx in &data.transactions {
            let tx_digest = tx.transaction.digest().to_string();

            if let Some(tx_events) = &tx.events {
                for (event_seq, event) in tx_events.data.iter().enumerate() {
                    // Filter by social package address
                    if event.package_id == self.package_address {
                        event_count += 1;

                        let module = event.type_.module.as_str();
                        let event_name = event.type_.name.as_str();
                        let event_id = format!("{}:{}", tx_digest, event_seq);

                        // Parse the BCS-encoded event contents to JSON
                        let event_data = match bcs::from_bytes::<serde_json::Value>(&event.contents) {
                            Ok(v) => v,
                            Err(e) => {
                                debug!("BCS parse failed for event {}::{}: {}, trying JSON", module, event_name, e);
                                match serde_json::from_slice(&event.contents) {
                                    Ok(v) => v,
                                    Err(_) => {
                                        debug!("Failed to parse event {}::{} contents", module, event_name);
                                        serde_json::Value::Null
                                    }
                                }
                            }
                        };

                        // Route to appropriate handler based on module
                        if let Err(e) = self.route_event(&mut conn, module, event_name, &event_data, &event_id).await {
                            error!(
                                "Failed to process event {}::{} in checkpoint {}: {}",
                                module, event_name, checkpoint_seq, e
                            );
                            // Continue processing other events
                        }
                    }
                }
            }
        }

        if event_count > 0 {
            info!(
                "Processed {} social events from checkpoint {}",
                event_count, checkpoint_seq
            );
        } else {
            debug!("No social events in checkpoint {}", checkpoint_seq);
        }

        // Update progress
        self.update_checkpoint_progress(checkpoint_seq).await?;

        Ok(())
    }
}

impl SocialCheckpointProcessor {
    /// Route an event to the appropriate handler based on module
    async fn route_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        module: &str,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        match module {
            // Governance events - have standalone processing functions
            "governance" => {
                self.handle_governance_event(conn, event_name, data, event_id).await?;
            }
            // Blocking events - have standalone processing functions
            "block_list" | "blocking" => {
                self.handle_blocking_event(conn, event_name, data).await?;
            }
            // MyData events - have standalone processing functions
            "mydata" | "my_ip" => {
                self.handle_mydata_event(conn, event_name, data, event_id).await?;
            }
            // Profile events
            "profile" => {
                self.handle_profile_event(conn, event_name, data, event_id).await?;
            }
            // Social graph events
            "social_graph" => {
                self.handle_social_graph_event(conn, event_name, data, event_id).await?;
            }
            // Platform events
            "platform" => {
                self.handle_platform_event(conn, event_name, data, event_id).await?;
            }
            // Post/comment/reaction events
            "post" | "comment" | "reaction" | "repost" | "tip" => {
                self.handle_post_event(conn, event_name, data, event_id).await?;
            }
            // Subscription events
            "subscription" | "profile_subscription" => {
                self.handle_subscription_event(conn, event_name, data, event_id).await?;
            }
            // Insurance events
            "insurance" => {
                self.handle_insurance_event(conn, event_name, data, event_id).await?;
            }
            // Proof of Creativity events
            "poc" | "proof_of_creativity" => {
                self.handle_poc_event(conn, event_name, data, event_id).await?;
            }
            // Social Proof of Truth events
            "social_proof_of_truth" | "spot" => {
                self.handle_spot_event(conn, event_name, data, event_id).await?;
            }
            // Social Proof Token events
            "social_proof_tokens" | "spt" => {
                debug!("Processing SPT event: {}::{}", module, event_name);
                self.handle_spt_event(conn, event_name, data, event_id).await?;
            }
            // Upgrade/migration events (system events - logged for audit)
            "upgrade" => {
                self.handle_upgrade_event(event_name, data, event_id).await?;
            }
            _ => {
                debug!("Unknown module: {}", module);
            }
        }

        Ok(())
    }

    /// Handle governance events using existing processing functions
    async fn handle_governance_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::governance_events::*;

        match event_name {
            "GovernanceRegistryCreatedEvent" => {
                process_governance_registry_created_event(conn, data, event_id).await?;
            }
            "DelegateNominatedEvent" => {
                process_delegate_nominated_event(conn, data, event_id).await?;
            }
            "DelegateElectedEvent" => {
                process_delegate_elected_event(conn, data, event_id).await?;
            }
            "DelegateVotedEvent" => {
                process_delegate_voted_event(conn, data, event_id).await?;
            }
            "ProposalSubmittedEvent" => {
                process_proposal_submitted_event(conn, data, event_id).await?;
            }
            "DelegateVoteEvent" => {
                process_delegate_vote_event(conn, data, event_id).await?;
            }
            "CommunityVoteEvent" => {
                process_community_vote_event(conn, data, event_id).await?;
            }
            "ProposalApprovedForVotingEvent" => {
                process_proposal_approved_for_voting_event(conn, data, event_id).await?;
            }
            "ProposalRejectedEvent" => {
                process_proposal_rejected_event(conn, data, event_id).await?;
            }
            "ProposalRescindedEvent" => {
                process_proposal_rescinded_event(conn, data, event_id).await?;
            }
            "ProposalRejectedByCommunityEvent" => {
                process_proposal_rejected_by_community_event(conn, data, event_id).await?;
            }
            "ProposalApprovedEvent" => {
                process_proposal_approved_event(conn, data, event_id).await?;
            }
            "ProposalImplementedEvent" => {
                process_proposal_implemented_event(conn, data, event_id).await?;
            }
            "RewardsDistributedEvent" => {
                process_rewards_distributed_event(conn, data, event_id).await?;
            }
            "AnonymousVoteEvent" => {
                process_anonymous_vote_event(conn, data, event_id).await?;
            }
            "VoteDecryptionFailedEvent" => {
                process_vote_decryption_failed_event(conn, data, event_id).await?;
            }
            "GovernanceParametersUpdatedEvent" => {
                process_governance_parameters_updated_event(conn, data, event_id).await?;
            }
            _ => {
                debug!("Unhandled governance event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle blocking events using existing processing functions
    async fn handle_blocking_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
    ) -> Result<()> {
        use crate::social::events::blocking_events::*;

        match event_name {
            "UserBlockEvent" | "ProfileBlockEvent" => {
                process_profile_block_event(conn, data).await?;
            }
            "UserUnblockEvent" | "ProfileUnblockEvent" => {
                process_profile_unblock_event(conn, data).await?;
            }
            _ => {
                debug!("Unhandled blocking event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle mydata events using existing processing functions
    async fn handle_mydata_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::mydata_events::*;

        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        match event_name {
            "MyDataRegisteredEvent" | "IPRegisteredEvent" => {
                process_mydata_registered_event(conn, data, event_id).await?;
            }
            "MyDataUnregisteredEvent" | "IPUnregisteredEvent" => {
                process_mydata_unregistered_event(conn, data, event_id).await?;
            }
            "MyDataCreatedEvent" | "DataCreatedEvent" => {
                process_mydata_created_event(conn, data, event_id).await?;
            }
            "PurchaseEvent" | "DataPurchasedEvent" => {
                process_mydata_purchase_event(conn, data, event_id).await?;
            }
            "AccessGrantedEvent" | "DataAccessGrantedEvent" => {
                process_mydata_access_granted_event(conn, data, event_id).await?;
            }
            "MyDataConfigUpdatedEvent" | "ConfigUpdatedEvent" => {
                process_mydata_config_updated_event(conn, data, event_id, timestamp_ms).await?;
            }
            _ => {
                debug!("Unhandled mydata event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle profile events
    async fn handle_profile_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::profile_events::*;

        match event_name {
            "ProfileCreatedEvent" => {
                process_profile_created_event(conn, data, event_id).await?;
            }
            "ProfileUpdatedEvent" => {
                process_profile_updated_event(conn, data, event_id).await?;
            }
            "UsernameRegisteredEvent" => {
                process_username_registered_event(conn, data, event_id).await?;
            }
            "UsernameUpdatedEvent" => {
                process_username_updated_event(conn, data, event_id).await?;
            }
            "BadgeAssignedEvent" => {
                process_badge_assigned_event(conn, data, event_id).await?;
            }
            "BadgeRevokedEvent" => {
                process_badge_revoked_event(conn, data, event_id).await?;
            }
            "BadgeSelectedEvent" => {
                process_badge_selected_event(conn, data, event_id).await?;
            }
            "PaidMessagingSettingsUpdatedEvent" => {
                process_paid_messaging_settings_updated_event(conn, data, event_id).await?;
            }
            _ => {
                debug!("Unhandled profile event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle social graph events
    async fn handle_social_graph_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::social_graph_events::*;

        match event_name {
            "FollowEvent" | "UserFollowedEvent" => {
                process_follow_event(conn, data, event_id).await?;
            }
            "UnfollowEvent" | "UserUnfollowedEvent" => {
                process_unfollow_event(conn, data, event_id).await?;
            }
            _ => {
                debug!("Unhandled social graph event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle platform events
    async fn handle_platform_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::platform_events::*;

        match event_name {
            "PlatformCreatedEvent" => {
                process_platform_created_event(conn, data, event_id).await?;
            }
            "PlatformUpdatedEvent" => {
                process_platform_updated_event(conn, data, event_id).await?;
            }
            "PlatformApprovalChangedEvent" | "ApprovalChangedEvent" => {
                process_platform_approval_changed_event(conn, data, event_id).await?;
            }
            "ModeratorAddedEvent" => {
                process_moderator_added_event(conn, data, event_id).await?;
            }
            "ModeratorRemovedEvent" => {
                process_moderator_removed_event(conn, data, event_id).await?;
            }
            "PlatformBlockedProfileEvent" => {
                process_platform_blocked_profile_event(conn, data, event_id).await?;
            }
            "PlatformUnblockedProfileEvent" => {
                process_platform_unblocked_profile_event(conn, data, event_id).await?;
            }
            "UserJoinedPlatformEvent" => {
                process_user_joined_platform_event(conn, data, event_id).await?;
            }
            "UserLeftPlatformEvent" => {
                process_user_left_platform_event(conn, data, event_id).await?;
            }
            "TokenAirdropEvent" => {
                process_token_airdrop_event(conn, data, event_id).await?;
            }
            "PlatformDeletedEvent" => {
                process_platform_deleted_event(conn, data, event_id).await?;
            }
            _ => {
                debug!("Unhandled platform event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle post/comment/reaction events
    async fn handle_post_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::post_events::*;

        match event_name {
            "PostCreatedEvent" => {
                process_post_created_event(conn, data, event_id).await?;
            }
            "CommentCreatedEvent" => {
                process_comment_created_event(conn, data, event_id).await?;
            }
            "ReactionEvent" | "ReactionAddedEvent" => {
                process_reaction_event(conn, data, event_id).await?;
            }
            "ReactionRemovedEvent" | "RemoveReactionEvent" => {
                process_remove_reaction_event(conn, data, event_id).await?;
            }
            "RepostEvent" | "RepostCreatedEvent" => {
                process_repost_event(conn, data, event_id).await?;
            }
            "TipEvent" | "TipSentEvent" => {
                process_tip_event(conn, data, event_id).await?;
            }
            "ModerationEvent" | "ContentModerationEvent" => {
                process_moderation_event(conn, data, event_id).await?;
            }
            "ReportEvent" | "ContentReportEvent" => {
                process_report_event(conn, data, event_id).await?;
            }
            "DeletionEvent" | "ContentDeletedEvent" | "PostDeletedEvent" | "CommentDeletedEvent" => {
                process_deletion_event(conn, data, event_id).await?;
            }
            "ContentUpdateEvent" | "PostUpdatedEvent" | "CommentUpdatedEvent" => {
                process_content_update_event(conn, data, event_id).await?;
            }
            _ => {
                debug!("Unhandled post event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle subscription events
    async fn handle_subscription_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::subscription_events::*;

        match event_name {
            "ProfileSubscriptionServiceCreatedEvent" => {
                process_subscription_service_created_event(conn, data, event_id).await?;
            }
            "ProfileSubscriptionCreatedEvent" => {
                process_subscription_created_event(conn, data, event_id).await?;
            }
            "ProfileSubscriptionRenewedEvent" => {
                process_subscription_renewed_event(conn, data, event_id).await?;
            }
            "ProfileSubscriptionCancelledEvent" => {
                process_subscription_cancelled_event(conn, data, event_id).await?;
            }
            "ProfileSubscriptionUpdatedEvent" => {
                process_subscription_updated_event(conn, data, event_id).await?;
            }
            "RenewalBalanceFundedEvent" => {
                process_renewal_balance_funded_event(conn, data, event_id).await?;
            }
            "ProfileSubscriptionServiceDeactivatedEvent" => {
                process_subscription_service_deactivated_event(conn, data, event_id).await?;
            }
            _ => {
                debug!("Unhandled subscription event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle insurance events
    async fn handle_insurance_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::insurance_events::*;

        // Use current time for timestamp_ms if not in event
        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let tx = event_id.split(':').next().unwrap_or(event_id).to_string();

        match event_name {
            "ConfigInitializedEvent" => {
                process_config_initialized_event(conn, data, event_id, timestamp_ms, tx).await?;
            }
            "ConfigUpdatedEvent" => {
                process_config_updated_event(conn, data, event_id, timestamp_ms, tx).await?;
            }
            "UnderwriterVaultCreatedEvent" => {
                process_vault_created_event(conn, data, event_id, tx).await?;
            }
            "UnderwriterVaultDepositedEvent" => {
                process_vault_deposited_event(conn, data, event_id, timestamp_ms, tx).await?;
            }
            "UnderwriterVaultWithdrawnEvent" => {
                process_vault_withdrawn_event(conn, data, event_id, timestamp_ms, tx).await?;
            }
            "CoveragePurchasedEvent" => {
                process_coverage_purchased_event(conn, data, event_id, timestamp_ms, tx).await?;
            }
            "CoverageCancelledEvent" => {
                process_coverage_cancelled_event(conn, data, event_id, timestamp_ms, tx).await?;
            }
            "CoverageClaimedEvent" => {
                process_coverage_claimed_event(conn, data, event_id, timestamp_ms, tx).await?;
            }
            "PolicyExpiredEvent" => {
                process_policy_expired_event(conn, data, event_id, timestamp_ms, tx).await?;
            }
            _ => {
                debug!("Unhandled insurance event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle Proof of Creativity (PoC) events
    async fn handle_poc_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::poc_events::*;

        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let tx = event_id.split(':').next().unwrap_or(event_id).to_string();

        match event_name {
            "AnalysisSubmittedEvent" => {
                process_analysis_submitted_event(conn, data, event_id, tx).await?;
            }
            "PocBadgeIssuedEvent" | "BadgeIssuedEvent" => {
                process_poc_badge_issued_event(conn, data, event_id, tx).await?;
            }
            "RevenueRedirectionActivatedEvent" => {
                process_revenue_redirection_activated_event(conn, data, event_id, tx).await?;
            }
            "PocDisputeSubmittedEvent" | "DisputeSubmittedEvent" => {
                process_poc_dispute_submitted_event(conn, data, event_id, tx).await?;
            }
            "DisputeVoteCastEvent" | "VoteCastEvent" => {
                process_dispute_vote_cast_event(conn, data, event_id, tx).await?;
            }
            "PocDisputeResolvedEvent" | "DisputeResolvedEvent" => {
                process_poc_dispute_resolved_event(conn, data, event_id, tx).await?;
            }
            "VotingRewardClaimedEvent" | "RewardClaimedEvent" => {
                process_voting_reward_claimed_event(conn, data, event_id, tx).await?;
            }
            "PocConfigUpdatedEvent" | "ConfigUpdatedEvent" => {
                process_poc_config_updated_event(conn, data, event_id, timestamp_ms, tx).await?;
            }
            "TokenPoolSyncNeededEvent" => {
                process_token_pool_sync_needed_event(conn, data, event_id, tx).await?;
            }
            _ => {
                debug!("Unhandled PoC event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle Social Proof of Truth (SPOT) events
    async fn handle_spot_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::social_proof_of_truth_events::*;

        // Use current epoch (in practice, this would come from checkpoint data)
        let epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let timestamp_ms = epoch * 1000;
        let tx = event_id.split(':').next().unwrap_or(event_id).to_string();

        match event_name {
            "SpotBetPlacedEvent" | "BetPlacedEvent" => {
                process_spot_bet_placed_event(conn, data, event_id, epoch, tx).await?;
            }
            "SpotResolvedEvent" | "ResolvedEvent" => {
                process_spot_resolved_event(conn, data, event_id, epoch, tx).await?;
            }
            "SpotDaoRequiredEvent" | "DaoRequiredEvent" => {
                process_spot_dao_required_event(conn, data, event_id, epoch, tx).await?;
            }
            "SpotPayoutEvent" | "PayoutEvent" => {
                process_spot_payout_event(conn, data, event_id, epoch, tx).await?;
            }
            "SpotRefundEvent" | "RefundEvent" => {
                process_spot_refund_event(conn, data, event_id, epoch, tx).await?;
            }
            "SpotConfigUpdatedEvent" | "ConfigUpdatedEvent" => {
                process_spot_config_updated_event(conn, data, event_id, timestamp_ms, tx).await?;
            }
            "SpotRecordCreatedEvent" | "RecordCreatedEvent" => {
                process_spot_record_created_event(conn, data, event_id, epoch, tx).await?;
            }
            "SpotBetWithdrawnEvent" | "BetWithdrawnEvent" => {
                process_spot_bet_withdrawn_event(conn, data, event_id, epoch, tx).await?;
            }
            _ => {
                debug!("Unhandled SPOT event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle upgrade/migration events (system events - logged for audit purposes)
    async fn handle_upgrade_event(
        &self,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        match event_name {
            "UpgradeEvent" => {
                let package_id = data.get("package_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                let version = data.get("version").and_then(|v| v.as_u64()).unwrap_or(0);
                tracing::info!("Processed UpgradeEvent: package {} upgraded to version {} (event: {})",
                    package_id, version, event_id);
            }
            "ObjectMigratedEvent" => {
                let object_id = data.get("object_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                let object_type = data.get("object_type").and_then(|v| v.as_str()).unwrap_or("unknown");
                let old_version = data.get("old_version").and_then(|v| v.as_u64()).unwrap_or(0);
                let new_version = data.get("new_version").and_then(|v| v.as_u64()).unwrap_or(0);
                let migrated_by = data.get("migrated_by").and_then(|v| v.as_str()).unwrap_or("unknown");
                tracing::info!("Processed ObjectMigratedEvent: {} ({}) migrated from v{} to v{} by {} (event: {})",
                    object_id, object_type, old_version, new_version, migrated_by, event_id);
            }
            _ => {
                debug!("Unhandled upgrade event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Handle Social Proof Token (SPT) events
    async fn handle_spt_event(
        &self,
        conn: &mut crate::social::db::DbConnection,
        event_name: &str,
        data: &serde_json::Value,
        event_id: &str,
    ) -> Result<()> {
        use crate::social::events::social_proof_token_events::*;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let tx = event_id.split(':').next().unwrap_or(event_id).to_string();

        match event_name {
            "TokenPoolCreatedEvent" | "PoolCreatedEvent" => {
                info!("Processing TokenPoolCreatedEvent (event_id: {})", event_id);
                process_token_pool_created_event(conn, data, event_id, timestamp, tx).await?;
            }
            "TokenBoughtEvent" | "BuyEvent" => {
                process_token_bought_event(conn, data, event_id, timestamp, tx).await?;
            }
            "TokenSoldEvent" | "SellEvent" => {
                process_token_sold_event(conn, data, event_id, timestamp, tx).await?;
            }
            "ReservationPoolCreatedEvent" => {
                process_reservation_pool_created_event(conn, data, event_id, timestamp, tx).await?;
            }
            "ReservationCreatedEvent" => {
                process_reservation_created_event(conn, data, event_id, timestamp, tx).await?;
            }
            "ReservationWithdrawnEvent" => {
                process_reservation_withdrawn_event(conn, data, event_id, timestamp, tx).await?;
            }
            "ThresholdMetEvent" => {
                process_threshold_met_event(conn, data, event_id, timestamp, tx).await?;
            }
            "ConfigUpdatedEvent" => {
                process_spt_config_updated_event(conn, data, event_id, timestamp, tx).await?;
            }
            "EmergencyKillSwitchEvent" => {
                process_emergency_kill_switch_event(conn, data, event_id, timestamp, tx).await?;
            }
            "SocialProofInitPoolEvent" | "InitPoolEvent" => {
                process_social_proof_init_pool_event(conn, data, event_id, timestamp as i64, tx).await?;
            }
            "SocialProofBuyEvent" => {
                process_social_proof_buy_event(conn, data, event_id, timestamp as i64, tx).await?;
            }
            "SocialProofSellEvent" => {
                process_social_proof_sell_event(conn, data, event_id, timestamp as i64, tx).await?;
            }
            _ => {
                debug!("Unhandled SPT event: {}", event_name);
            }
        }
        Ok(())
    }

    /// Update the checkpoint progress in the database
    async fn update_checkpoint_progress(&self, checkpoint_seq: u64) -> Result<()> {
        let mut conn = self.db.get_connection().await?;
        let now = Utc::now().naive_utc();

        let progress = NewIndexerProgress {
            id: "social_checkpoint_processor".to_string(),
            last_checkpoint_processed: checkpoint_seq as i64,
            last_processed_at: now,
        };

        diesel::insert_into(schema::indexer_progress::table)
            .values(&progress)
            .on_conflict(schema::indexer_progress::id)
            .do_update()
            .set((
                schema::indexer_progress::last_checkpoint_processed.eq(checkpoint_seq as i64),
                schema::indexer_progress::last_processed_at.eq(now),
            ))
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}
