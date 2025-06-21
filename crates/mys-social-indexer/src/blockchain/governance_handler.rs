// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use serde_json::Value;
use tracing::{debug, info};
use tokio::sync::mpsc;
use std::sync::Arc;

use crate::db::DbConnection;
use crate::db::Database;
use crate::blockchain::listener::BlockchainEvent;
use crate::events::governance_events::*;
use crate::GOVERNANCE_MODULE_NAME;

/// GovernanceEventHandler handles all governance-related events from the blockchain
pub struct GovernanceEventHandler {
    db: Arc<Database>,
    receiver: mpsc::Receiver<BlockchainEvent>,
    worker_name: String,
}

impl GovernanceEventHandler {
    /// Create a new GovernanceEventHandler instance
    pub fn new(
        db: Arc<Database>,
        receiver: mpsc::Receiver<BlockchainEvent>,
        worker_name: String,
    ) -> Self {
        Self {
            db,
            receiver,
            worker_name,
        }
    }

    /// Start the governance event handler
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting governance event handler: {}", self.worker_name);
        
        while let Some(event) = self.receiver.recv().await {
            // Extract the module name from the event type
            // Example: 0x123::governance::ProposalSubmittedEvent
            let parts: Vec<&str> = event.event_type.split("::").collect();
            if parts.len() < 2 {
                continue; // Skip malformed event types
            }
            
            let module_name = parts[1]; // Second part is the module name
            
            // Get the function/event name, which is the last part
            let function_name = parts.last().unwrap_or(&"")
                .replace("Event", ""); // Remove "Event" suffix if present
            
            let mut conn = self.db.get_connection().await?;
            self.process_event(&mut conn, module_name, &function_name, &event.data, &event.event_id).await?;
        }
        
        Ok(())
    }

    /// Process a governance event from the blockchain
    pub async fn process_event(
        &self,
        conn: &mut DbConnection,
        module_name: &str,
        function_name: &str,
        event_data: &Value,
        event_id: &str,
    ) -> Result<()> {
        // Skip if this is not a governance module event
        if module_name != GOVERNANCE_MODULE_NAME {
            return Ok(());
        }

        debug!("Processing governance event: {}", function_name);

        // Match the function name to determine the event type
        match function_name {
            "create_registry" | "update_registry" | "GovernanceRegistry" => {
                process_governance_registry_event(conn, event_data, event_id).await?;
            }
            "nominate_delegate" | "DelegateNominated" => {
                process_delegate_nominated_event(conn, event_data, event_id).await?;
            }
            "rate_delegate" | "DelegateVoted" => {
                process_delegate_voted_event(conn, event_data, event_id).await?;
            }
            "elect_delegate" | "DelegateElected" => {
                process_delegate_elected_event(conn, event_data, event_id).await?;
            }
            "submit_proposal" | "ProposalSubmitted" => {
                process_proposal_submitted_event(conn, event_data, event_id).await?;
            }
            "delegate_vote" | "DelegateVote" => {
                process_delegate_vote_event(conn, event_data, event_id).await?;
            }
            "community_vote" | "CommunityVote" => {
                process_community_vote_event(conn, event_data, event_id).await?;
            }
            "approve_for_voting" | "ProposalApprovedForVoting" => {
                process_proposal_approved_for_voting_event(conn, event_data, event_id).await?;
            }
            "reject_proposal" | "ProposalRejected" => {
                process_proposal_rejected_event(conn, event_data, event_id).await?;
            }
            "rescind_proposal" | "ProposalRescinded" => {
                process_proposal_rescinded_event(conn, event_data, event_id).await?;
            }
            "approve_proposal" | "ProposalApproved" => {
                process_proposal_approved_event(conn, event_data, event_id).await?;
            }
            "implement_proposal" | "ProposalImplemented" => {
                process_proposal_implemented_event(conn, event_data, event_id).await?;
            }
            "distribute_rewards" | "RewardsDistributed" => {
                process_rewards_distributed_event(conn, event_data, event_id).await?;
            }
            "community_vote_anonymous" | "AnonymousVote" => {
                process_anonymous_vote_event(conn, event_data, event_id).await?;
            }
            "VoteDecryptionFailed" => {
                process_vote_decryption_failed_event(conn, event_data, event_id).await?;
            }
            _ => {
                debug!("Unknown governance function: {}", function_name);
            }
        }

        info!("Processed governance event: {}", function_name);
        Ok(())
    }
} 