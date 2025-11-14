// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde_json::Value;
use tracing::{debug, error, info};

use crate::db::query_types::{DelegateVoteResult, ProposalTypeResult};
use crate::db::DbConnection;
use crate::events::event_utils::parse_json_event;
use crate::events::governance_event_types::*;
use crate::models::governance::*;
use crate::{
    GOVERNANCE_STATUS_APPROVED, GOVERNANCE_STATUS_COMMUNITY_VOTING, GOVERNANCE_STATUS_IMPLEMENTED,
    GOVERNANCE_STATUS_OWNER_RESCINDED, GOVERNANCE_STATUS_REJECTED, GOVERNANCE_STATUS_SUBMITTED,
    NOMINEE_STATUS_ELECTED, NOMINEE_STATUS_PENDING,
};

/// Process a governance registry creation/update event
pub async fn process_governance_registry_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing governance registry event");

    // Parse the event
    let registry_event = parse_json_event::<GovernanceRegistryEvent>(event)?;

    // Insert or update registry
    let new_registry = NewGovernanceRegistry {
        registry_type: registry_event.registry_type as i16,
        delegate_count: registry_event.delegate_count as i64,
        delegate_term_epochs: registry_event.delegate_term_epochs as i64,
        proposal_submission_cost: registry_event.proposal_submission_cost as i64,
        min_on_chain_age_days: registry_event.min_on_chain_age_days as i64,
        max_votes_per_user: registry_event.max_votes_per_user as i64,
        quadratic_base_cost: registry_event.quadratic_base_cost as i64,
        voting_period_epochs: registry_event.voting_period_epochs as i64,
        quorum_votes: registry_event.quorum_votes as i64,
        updated_at: registry_event.updated_at as i64,
        transaction_id: event_id.to_string(),
    };

    let result = diesel::insert_into(crate::schema::governance_registries::table)
        .values(&new_registry)
        .on_conflict(crate::schema::governance_registries::registry_type)
        .do_update()
        .set((
            crate::schema::governance_registries::delegate_count.eq(new_registry.delegate_count),
            crate::schema::governance_registries::delegate_term_epochs
                .eq(new_registry.delegate_term_epochs),
            crate::schema::governance_registries::proposal_submission_cost
                .eq(new_registry.proposal_submission_cost),
            crate::schema::governance_registries::min_on_chain_age_days
                .eq(new_registry.min_on_chain_age_days),
            crate::schema::governance_registries::max_votes_per_user
                .eq(new_registry.max_votes_per_user),
            crate::schema::governance_registries::quadratic_base_cost
                .eq(new_registry.quadratic_base_cost),
            crate::schema::governance_registries::voting_period_epochs
                .eq(new_registry.voting_period_epochs),
            crate::schema::governance_registries::quorum_votes.eq(new_registry.quorum_votes),
            crate::schema::governance_registries::updated_at.eq(new_registry.updated_at),
            crate::schema::governance_registries::transaction_id.eq(event_id.to_string()),
        ))
        .execute(conn)
        .await?;

    info!(
        "Processed governance registry event: {} rows affected",
        result
    );

    // Record this event in the governance_events table
    let governance_event = NewGovernanceEvent {
        event_type: "GovernanceRegistryEvent".to_string(),
        registry_type: registry_event.registry_type as i16,
        event_data: event.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
        anonymous_voting_related: None,
    };

    diesel::insert_into(crate::schema::governance_events::table)
        .values(&governance_event)
        .execute(conn)
        .await?;

    Ok(())
}

/// Process a delegate nomination event
pub async fn process_delegate_nominated_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing delegate nominated event");

    // Parse the event
    let nomination_event = parse_json_event::<DelegateNominatedEvent>(event)?;

    // Create new nominee record
    let new_nominee = NewNominatedDelegate {
        address: nomination_event.address.clone(),
        profile_id: nomination_event.profile_id.clone(),
        registry_type: nomination_event.registry_type as i16,
        upvotes: 0,
        downvotes: 0,
        scheduled_term_start_epoch: nomination_event.scheduled_term_start_epoch as i64,
        nomination_time: nomination_event.nomination_time as i64,
        status: NOMINEE_STATUS_PENDING as i16,
        transaction_id: event_id.to_string(),
    };

    // Insert the record, updating if the address already exists for this registry type
    let result = diesel::insert_into(crate::schema::nominated_delegates::table)
        .values(&new_nominee)
        .on_conflict((
            crate::schema::nominated_delegates::address,
            crate::schema::nominated_delegates::registry_type,
        ))
        .do_update()
        .set((
            crate::schema::nominated_delegates::profile_id.eq(new_nominee.profile_id.clone()),
            crate::schema::nominated_delegates::upvotes.eq(0),
            crate::schema::nominated_delegates::downvotes.eq(0),
            crate::schema::nominated_delegates::scheduled_term_start_epoch
                .eq(new_nominee.scheduled_term_start_epoch),
            crate::schema::nominated_delegates::nomination_time.eq(new_nominee.nomination_time),
            crate::schema::nominated_delegates::status.eq(NOMINEE_STATUS_PENDING as i16),
            crate::schema::nominated_delegates::transaction_id.eq(event_id.to_string()),
        ))
        .execute(conn)
        .await?;

    info!(
        "Processed delegate nomination event: {} rows affected",
        result
    );

    // Record this event in the governance_events table
    let governance_event = NewGovernanceEvent {
        event_type: "DelegateNominatedEvent".to_string(),
        registry_type: nomination_event.registry_type as i16,
        event_data: event.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
        anonymous_voting_related: None,
    };

    diesel::insert_into(crate::schema::governance_events::table)
        .values(&governance_event)
        .execute(conn)
        .await?;

    Ok(())
}

/// Process a delegate elected event
pub async fn process_delegate_elected_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing delegate elected event");

    // Parse the event
    let elected_event = parse_json_event::<DelegateElectedEvent>(event)?;

    // Begin transaction
    conn.build_transaction()
        .run(|tx_conn| {
            Box::pin(async move {
                // Update nominee status to elected
                diesel::update(crate::schema::nominated_delegates::table)
                    .filter(
                        crate::schema::nominated_delegates::address
                            .eq(&elected_event.address)
                            .and(
                                crate::schema::nominated_delegates::registry_type
                                    .eq(elected_event.registry_type as i16),
                            ),
                    )
                    .set((
                        crate::schema::nominated_delegates::status
                            .eq(NOMINEE_STATUS_ELECTED as i16),
                        crate::schema::nominated_delegates::upvotes
                            .eq(elected_event.upvotes as i64),
                        crate::schema::nominated_delegates::downvotes
                            .eq(elected_event.downvotes as i64),
                    ))
                    .execute(tx_conn)
                    .await?;

                // Create or update delegate record
                let now_unix = Utc::now().timestamp() as i64;
                let new_delegate = NewDelegate {
                    address: elected_event.address.clone(),
                    profile_id: elected_event.profile_id.clone(),
                    registry_type: elected_event.registry_type as i16,
                    upvotes: elected_event.upvotes as i64,
                    downvotes: elected_event.downvotes as i64,
                    proposals_reviewed: 0, // These start at 0 for new delegates
                    proposals_submitted: 0,
                    sided_winning_proposals: 0,
                    sided_losing_proposals: 0,
                    term_start: elected_event.term_start as i64,
                    term_end: elected_event.term_end as i64,
                    is_active: true,
                    created_at: now_unix,
                    updated_at: now_unix,
                    transaction_id: event_id.to_string(),
                };

                diesel::insert_into(crate::schema::delegates::table)
                    .values(&new_delegate)
                    .on_conflict((
                        crate::schema::delegates::address,
                        crate::schema::delegates::registry_type,
                    ))
                    .do_update()
                    .set((
                        crate::schema::delegates::profile_id.eq(new_delegate.profile_id.clone()),
                        crate::schema::delegates::upvotes.eq(new_delegate.upvotes),
                        crate::schema::delegates::downvotes.eq(new_delegate.downvotes),
                        crate::schema::delegates::term_start.eq(new_delegate.term_start),
                        crate::schema::delegates::term_end.eq(new_delegate.term_end),
                        crate::schema::delegates::is_active.eq(true),
                        crate::schema::delegates::updated_at.eq(now_unix),
                        crate::schema::delegates::transaction_id.eq(event_id.to_string()),
                    ))
                    .execute(tx_conn)
                    .await?;

                // Record this event in the governance_events table
                let governance_event = NewGovernanceEvent {
                    event_type: "DelegateElectedEvent".to_string(),
                    registry_type: elected_event.registry_type as i16,
                    event_data: event.clone(),
                    event_id: event_id.to_string(),
                    created_at: Utc::now(),
                    anonymous_voting_related: None,
                };

                diesel::insert_into(crate::schema::governance_events::table)
                    .values(&governance_event)
                    .execute(tx_conn)
                    .await?;

                info!("Processed delegate elected event successfully");

                Ok::<_, anyhow::Error>(())
            })
        })
        .await?;

    Ok(())
}

/// Process a delegate voted (rating) event
pub async fn process_delegate_voted_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing delegate voted event");

    // Parse the event
    let voted_event = parse_json_event::<DelegateVotedEvent>(event)?;

    // Begin transaction
    conn.build_transaction()
        .run(|tx_conn| {
            Box::pin(async move {
                // Create delegate rating record
                let new_rating = NewDelegateRating {
                    target_address: voted_event.target_address.clone(),
                    voter_address: voted_event.voter.clone(), // Use voter field
                    registry_type: voted_event.registry_type as i16,
                    is_active_delegate: voted_event.is_active_delegate,
                    upvote: voted_event.upvote,
                    rated_at: Utc::now().timestamp(), // Use current time since event doesn't have timestamp
                    transaction_id: event_id.to_string(),
                };

                // Insert or update rating
                diesel::insert_into(crate::schema::delegate_ratings::table)
                    .values(&new_rating)
                    .on_conflict((
                        crate::schema::delegate_ratings::target_address,
                        crate::schema::delegate_ratings::voter_address,
                        crate::schema::delegate_ratings::registry_type,
                    ))
                    .do_update()
                    .set((
                        crate::schema::delegate_ratings::upvote.eq(new_rating.upvote),
                        crate::schema::delegate_ratings::rated_at.eq(new_rating.rated_at),
                        crate::schema::delegate_ratings::transaction_id.eq(event_id.to_string()),
                    ))
                    .execute(tx_conn)
                    .await?;

                // Update vote counts using the new counts from the event
                // The event already contains the updated counts, so we use those directly
                if voted_event.is_active_delegate {
                    // Update delegate record with new vote counts
                    diesel::update(crate::schema::delegates::table)
                        .filter(
                            crate::schema::delegates::address
                                .eq(&voted_event.target_address)
                                .and(
                                    crate::schema::delegates::registry_type
                                        .eq(voted_event.registry_type as i16),
                                ),
                        )
                        .set((
                            crate::schema::delegates::upvotes.eq(voted_event.new_upvote_count as i64),
                            crate::schema::delegates::downvotes.eq(voted_event.new_downvote_count as i64),
                        ))
                        .execute(tx_conn)
                        .await?;
                } else {
                    // Update nominee record with new vote counts
                    diesel::update(crate::schema::nominated_delegates::table)
                        .filter(
                            crate::schema::nominated_delegates::address
                                .eq(&voted_event.target_address)
                                .and(
                                    crate::schema::nominated_delegates::registry_type
                                        .eq(voted_event.registry_type as i16),
                                ),
                        )
                        .set((
                            crate::schema::nominated_delegates::upvotes.eq(voted_event.new_upvote_count as i64),
                            crate::schema::nominated_delegates::downvotes.eq(voted_event.new_downvote_count as i64),
                        ))
                        .execute(tx_conn)
                        .await?;
                }

                // Record this event in the governance_events table
                let governance_event = NewGovernanceEvent {
                    event_type: "DelegateVotedEvent".to_string(),
                    registry_type: voted_event.registry_type as i16,
                    event_data: event.clone(),
                    event_id: event_id.to_string(),
                    created_at: Utc::now(),
                    anonymous_voting_related: None,
                };

                diesel::insert_into(crate::schema::governance_events::table)
                    .values(&governance_event)
                    .execute(tx_conn)
                    .await?;

                info!("Processed delegate voted event successfully");

                Ok::<_, anyhow::Error>(())
            })
        })
        .await?;

    Ok(())
}

/// Process a proposal submitted event
pub async fn process_proposal_submitted_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing proposal submitted event");

    // Parse the event
    let proposal_event = parse_json_event::<ProposalSubmittedEvent>(event)?;

    // Create new proposal
    let new_proposal = NewProposal {
        id: proposal_event.id.clone(),
        title: proposal_event.title.clone(),
        description: proposal_event.description.clone(),
        proposal_type: proposal_event.proposal_type as i16,
        reference_id: proposal_event.reference_id.clone(),
        metadata_json: proposal_event.metadata_json.clone(),
        submitter: proposal_event.submitter.clone(),
        submission_time: proposal_event.submission_time as i64,
        status: GOVERNANCE_STATUS_SUBMITTED as i16,
        reward_pool: proposal_event.reward_pool as i64,
        transaction_id: event_id.to_string(),
    };

    // Insert the proposal
    let result = diesel::insert_into(crate::schema::proposals::table)
        .values(&new_proposal)
        .execute(conn)
        .await?;

    info!(
        "Processed proposal submitted event: {} rows affected",
        result
    );

    // Also update the delegate's proposals_submitted count
    diesel::update(crate::schema::delegates::table)
        .filter(
            crate::schema::delegates::address
                .eq(&proposal_event.submitter)
                .and(
                    crate::schema::delegates::registry_type.eq(proposal_event.proposal_type as i16),
                )
                .and(crate::schema::delegates::is_active.eq(true)),
        )
        .set(
            crate::schema::delegates::proposals_submitted
                .eq(crate::schema::delegates::proposals_submitted + 1),
        )
        .execute(conn)
        .await?;

    // Record this event in the governance_events table
    let governance_event = NewGovernanceEvent {
        event_type: "ProposalSubmittedEvent".to_string(),
        registry_type: proposal_event.proposal_type as i16,
        event_data: event.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
        anonymous_voting_related: None,
    };

    diesel::insert_into(crate::schema::governance_events::table)
        .values(&governance_event)
        .execute(conn)
        .await?;

    Ok(())
}

/// Process a delegate vote on proposal event
pub async fn process_delegate_vote_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing delegate vote event");

    // Parse the event
    let vote_event = parse_json_event::<DelegateVoteEvent>(event)?;

    // Begin transaction
    conn.build_transaction()
        .run(|tx_conn| {
            Box::pin(async move {
                // Insert the vote
                let new_vote = NewDelegateVote {
                    proposal_id: vote_event.proposal_id.clone(),
                    delegate_address: vote_event.delegate_address.clone(),
                    approve: vote_event.approve,
                    vote_time: vote_event.vote_time as i64,
                    reason: vote_event.reason.clone(),
                    transaction_id: event_id.to_string(),
                };

                diesel::insert_into(crate::schema::delegate_votes::table)
                    .values(&new_vote)
                    .on_conflict((
                        crate::schema::delegate_votes::proposal_id,
                        crate::schema::delegate_votes::delegate_address,
                    ))
                    .do_update()
                    .set((
                        crate::schema::delegate_votes::approve.eq(new_vote.approve),
                        crate::schema::delegate_votes::vote_time.eq(new_vote.vote_time),
                        crate::schema::delegate_votes::reason.eq(new_vote.reason.clone()),
                        crate::schema::delegate_votes::transaction_id.eq(event_id.to_string()),
                    ))
                    .execute(tx_conn)
                    .await?;

                // Update proposal vote counts
                if vote_event.approve {
                    diesel::update(crate::schema::proposals::table)
                        .filter(crate::schema::proposals::id.eq(&vote_event.proposal_id))
                        .set(
                            crate::schema::proposals::delegate_approval_count
                                .eq(crate::schema::proposals::delegate_approval_count + 1),
                        )
                        .execute(tx_conn)
                        .await?;
                } else {
                    diesel::update(crate::schema::proposals::table)
                        .filter(crate::schema::proposals::id.eq(&vote_event.proposal_id))
                        .set(
                            crate::schema::proposals::delegate_rejection_count
                                .eq(crate::schema::proposals::delegate_rejection_count + 1),
                        )
                        .execute(tx_conn)
                        .await?;
                }

                // Update delegate's proposals_reviewed count
                diesel::update(crate::schema::delegates::table)
                    .filter(
                        crate::schema::delegates::address
                            .eq(&vote_event.delegate_address)
                            .and(crate::schema::delegates::is_active.eq(true)),
                    )
                    .set(
                        crate::schema::delegates::proposals_reviewed
                            .eq(crate::schema::delegates::proposals_reviewed + 1),
                    )
                    .execute(tx_conn)
                    .await?;

                // Record this event in the governance_events table
                // We need to get the proposal type from the proposal
                let proposal_type_query =
                    diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
                        .bind::<diesel::sql_types::Text, _>(&vote_event.proposal_id)
                        .load::<ProposalTypeResult>(tx_conn)
                        .await?;

                if let Some(result) = proposal_type_query.get(0) {
                    let governance_event = NewGovernanceEvent {
                        event_type: "DelegateVoteEvent".to_string(),
                        registry_type: result.proposal_type,
                        event_data: event.clone(),
                        event_id: event_id.to_string(),
                        created_at: Utc::now(),
                        anonymous_voting_related: None,
                    };

                    diesel::insert_into(crate::schema::governance_events::table)
                        .values(&governance_event)
                        .execute(tx_conn)
                        .await?;
                } else {
                    error!(
                        "Failed to find proposal type for proposal ID: {}",
                        vote_event.proposal_id
                    );
                }

                info!("Processed delegate vote event successfully");

                Ok::<_, anyhow::Error>(())
            })
        })
        .await?;

    Ok(())
}

/// Process a community vote event
pub async fn process_community_vote_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing community vote event");

    // Parse the event
    let vote_event = parse_json_event::<CommunityVoteEvent>(event)?;

    // Begin transaction
    conn.build_transaction()
        .run(|tx_conn| {
            Box::pin(async move {
                // Insert the vote
                let new_vote = NewCommunityVote {
                    proposal_id: vote_event.proposal_id.clone(),
                    voter_address: vote_event.voter_address.clone(),
                    vote_weight: vote_event.vote_weight as i64,
                    approve: vote_event.approve,
                    vote_time: vote_event.vote_time as i64,
                    vote_cost: vote_event.vote_cost as i64,
                    transaction_id: event_id.to_string(),
                };

                diesel::insert_into(crate::schema::community_votes::table)
                    .values(&new_vote)
                    .on_conflict((
                        crate::schema::community_votes::proposal_id,
                        crate::schema::community_votes::voter_address,
                    ))
                    .do_update()
                    .set((
                        crate::schema::community_votes::vote_weight.eq(new_vote.vote_weight),
                        crate::schema::community_votes::approve.eq(new_vote.approve),
                        crate::schema::community_votes::vote_time.eq(new_vote.vote_time),
                        crate::schema::community_votes::vote_cost.eq(new_vote.vote_cost),
                        crate::schema::community_votes::transaction_id.eq(event_id.to_string()),
                    ))
                    .execute(tx_conn)
                    .await?;

                // Update proposal vote counts
                if vote_event.approve {
                    diesel::update(crate::schema::proposals::table)
                        .filter(crate::schema::proposals::id.eq(&vote_event.proposal_id))
                        .set(
                            crate::schema::proposals::community_votes_for
                                .eq(crate::schema::proposals::community_votes_for
                                    + vote_event.vote_weight as i64),
                        )
                        .execute(tx_conn)
                        .await?;
                } else {
                    diesel::update(crate::schema::proposals::table)
                        .filter(crate::schema::proposals::id.eq(&vote_event.proposal_id))
                        .set(
                            crate::schema::proposals::community_votes_against
                                .eq(crate::schema::proposals::community_votes_against
                                    + vote_event.vote_weight as i64),
                        )
                        .execute(tx_conn)
                        .await?;
                }

                // Record this event in the governance_events table
                // We need to get the proposal type from the proposal
                let proposal_type_query =
                    diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
                        .bind::<diesel::sql_types::Text, _>(&vote_event.proposal_id)
                        .load::<ProposalTypeResult>(tx_conn)
                        .await?;

                if let Some(result) = proposal_type_query.get(0) {
                    let governance_event = NewGovernanceEvent {
                        event_type: "CommunityVoteEvent".to_string(),
                        registry_type: result.proposal_type,
                        event_data: event.clone(),
                        event_id: event_id.to_string(),
                        created_at: Utc::now(),
                        anonymous_voting_related: None,
                    };

                    diesel::insert_into(crate::schema::governance_events::table)
                        .values(&governance_event)
                        .execute(tx_conn)
                        .await?;
                } else {
                    error!(
                        "Failed to find proposal type for proposal ID: {}",
                        vote_event.proposal_id
                    );
                }

                info!("Processed community vote event successfully");

                Ok::<_, anyhow::Error>(())
            })
        })
        .await?;

    Ok(())
}

/// Process a proposal approved for voting event
pub async fn process_proposal_approved_for_voting_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing proposal approved for voting event");

    // Parse the event
    let approved_event = parse_json_event::<ProposalApprovedForVotingEvent>(event)?;

    // Begin transaction
    conn.build_transaction()
        .run(|tx_conn| {
            Box::pin(async move {
                // Update proposal status
                diesel::update(crate::schema::proposals::table)
                    .filter(crate::schema::proposals::id.eq(&approved_event.proposal_id))
                    .set((
                        crate::schema::proposals::status
                            .eq(GOVERNANCE_STATUS_COMMUNITY_VOTING as i16),
                        crate::schema::proposals::voting_start_time
                            .eq(approved_event.voting_start_time as i64),
                        crate::schema::proposals::voting_end_time
                            .eq(approved_event.voting_end_time as i64),
                    ))
                    .execute(tx_conn)
                    .await?;

                // Record this event in the governance_events table
                // We need to get the proposal type from the proposal
                let proposal_type_query =
                    diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
                        .bind::<diesel::sql_types::Text, _>(&approved_event.proposal_id)
                        .load::<ProposalTypeResult>(tx_conn)
                        .await?;

                if let Some(result) = proposal_type_query.get(0) {
                    let governance_event = NewGovernanceEvent {
                        event_type: "ProposalApprovedForVotingEvent".to_string(),
                        registry_type: result.proposal_type,
                        event_data: event.clone(),
                        event_id: event_id.to_string(),
                        created_at: Utc::now(),
                        anonymous_voting_related: None,
                    };

                    diesel::insert_into(crate::schema::governance_events::table)
                        .values(&governance_event)
                        .execute(tx_conn)
                        .await?;
                } else {
                    error!(
                        "Failed to find proposal type for proposal ID: {}",
                        approved_event.proposal_id
                    );
                }

                info!("Processed proposal approved for voting event successfully");

                Ok::<_, anyhow::Error>(())
            })
        })
        .await?;

    Ok(())
}

/// Process a proposal rejected event
pub async fn process_proposal_rejected_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing proposal rejected event");

    // Parse the event
    let rejected_event = parse_json_event::<ProposalRejectedEvent>(event)?;

    // Begin transaction
    conn.build_transaction()
        .run(|tx_conn| {
            Box::pin(async move {
                // Update proposal status
                diesel::update(crate::schema::proposals::table)
                    .filter(crate::schema::proposals::id.eq(&rejected_event.proposal_id))
                    .set(crate::schema::proposals::status.eq(GOVERNANCE_STATUS_REJECTED as i16))
                    .execute(tx_conn)
                    .await?;

                // Update delegate stats
                // This is more complex as we need to find all delegates who voted and update their stats
                // First, find all delegates who voted on this proposal
                let delegate_votes = diesel::sql_query(
                    "
                    SELECT dv.delegate_address, dv.approve, p.submitter
                    FROM delegate_votes dv
                    JOIN proposals p ON dv.proposal_id = p.id
                    WHERE dv.proposal_id = $1
                ",
                )
                .bind::<diesel::sql_types::Text, _>(&rejected_event.proposal_id)
                .load::<DelegateVoteResult>(tx_conn)
                .await?;

                // Update each delegate's win/loss count
                for vote in &delegate_votes {
                    if !vote.approve {
                        // For rejected proposals
                        diesel::update(crate::schema::delegates::table)
                            .filter(crate::schema::delegates::address.eq(&vote.delegate_address))
                            .set(
                                crate::schema::delegates::sided_winning_proposals
                                    .eq(crate::schema::delegates::sided_winning_proposals + 1),
                            )
                            .execute(tx_conn)
                            .await?;
                    } else {
                        diesel::update(crate::schema::delegates::table)
                            .filter(crate::schema::delegates::address.eq(&vote.delegate_address))
                            .set(
                                crate::schema::delegates::sided_losing_proposals
                                    .eq(crate::schema::delegates::sided_losing_proposals + 1),
                            )
                            .execute(tx_conn)
                            .await?;
                    }
                }

                // Record this event in the governance_events table
                // We need to get the proposal type from the proposal
                let proposal_type_query =
                    diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
                        .bind::<diesel::sql_types::Text, _>(&rejected_event.proposal_id)
                        .load::<ProposalTypeResult>(tx_conn)
                        .await?;

                if let Some(result) = proposal_type_query.get(0) {
                    let governance_event = NewGovernanceEvent {
                        event_type: "ProposalRejectedEvent".to_string(),
                        registry_type: result.proposal_type,
                        event_data: event.clone(),
                        event_id: event_id.to_string(),
                        created_at: Utc::now(),
                        anonymous_voting_related: None,
                    };

                    diesel::insert_into(crate::schema::governance_events::table)
                        .values(&governance_event)
                        .execute(tx_conn)
                        .await?;
                } else {
                    error!(
                        "Failed to find proposal type for proposal ID: {}",
                        rejected_event.proposal_id
                    );
                }

                info!("Processed proposal rejected event successfully");

                Ok::<_, anyhow::Error>(())
            })
        })
        .await?;

    Ok(())
}

/// Process a proposal rescinded event
pub async fn process_proposal_rescinded_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing proposal rescinded event");

    // Parse the event
    let rescinded_event = parse_json_event::<ProposalRescindedEvent>(event)?;

    // Begin transaction
    conn.build_transaction()
        .run(|tx_conn| {
            Box::pin(async move {
                // Update proposal status
                diesel::update(crate::schema::proposals::table)
                    .filter(
                        crate::schema::proposals::id
                            .eq(&rescinded_event.proposal_id)
                            .and(
                                crate::schema::proposals::submitter.eq(&rescinded_event.submitter),
                            ),
                    )
                    .set((
                        crate::schema::proposals::status
                            .eq(GOVERNANCE_STATUS_OWNER_RESCINDED as i16),
                        crate::schema::proposals::rescind_time
                            .eq(rescinded_event.rescind_time as i64),
                        crate::schema::proposals::reward_pool.eq(0),
                    ))
                    .execute(tx_conn)
                    .await?;

                // Create refund record
                let new_refund = NewRewardDistribution {
                    proposal_id: rescinded_event.proposal_id.clone(),
                    recipient_address: rescinded_event.submitter.clone(),
                    amount: rescinded_event.refund_amount as i64,
                    distribution_time: rescinded_event.rescind_time as i64,
                    distribution_type: Some("rescind_refund".to_string()),
                    transaction_id: event_id.to_string(),
                };

                diesel::insert_into(crate::schema::reward_distributions::table)
                    .values(&new_refund)
                    .execute(tx_conn)
                    .await?;

                // Record this event in the governance_events table
                // We need to get the proposal type from the proposal
                let proposal_type_query =
                    diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
                        .bind::<diesel::sql_types::Text, _>(&rescinded_event.proposal_id)
                        .load::<ProposalTypeResult>(tx_conn)
                        .await?;

                if let Some(result) = proposal_type_query.get(0) {
                    let governance_event = NewGovernanceEvent {
                        event_type: "ProposalRescindedEvent".to_string(),
                        registry_type: result.proposal_type,
                        event_data: event.clone(),
                        event_id: event_id.to_string(),
                        created_at: Utc::now(),
                        anonymous_voting_related: None,
                    };

                    diesel::insert_into(crate::schema::governance_events::table)
                        .values(&governance_event)
                        .execute(tx_conn)
                        .await?;
                } else {
                    error!(
                        "Failed to find proposal type for proposal ID: {}",
                        rescinded_event.proposal_id
                    );
                }

                info!("Processed proposal rescinded event successfully");

                Ok::<_, anyhow::Error>(())
            })
        })
        .await?;

    Ok(())
}

/// Process a proposal rejected by community event
pub async fn process_proposal_rejected_by_community_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing proposal rejected by community event");

    // Parse the event
    let rejected_event = parse_json_event::<ProposalRejectedByCommunityEvent>(event)?;

    // Begin transaction
    conn.build_transaction()
        .run(|tx_conn| {
            Box::pin(async move {
                // Update proposal status
                diesel::update(crate::schema::proposals::table)
                    .filter(crate::schema::proposals::id.eq(&rejected_event.proposal_id))
                    .set((
                        crate::schema::proposals::status.eq(GOVERNANCE_STATUS_REJECTED as i16),
                        crate::schema::proposals::community_votes_for.eq(rejected_event.votes_for as i64),
                        crate::schema::proposals::community_votes_against.eq(rejected_event.votes_against as i64),
                    ))
                    .execute(tx_conn)
                    .await?;

                // Update delegate stats - delegates who voted against win, delegates who voted for lose
                let delegate_votes = diesel::sql_query(
                    "
                    SELECT dv.delegate_address, dv.approve, p.submitter
                    FROM delegate_votes dv
                    JOIN proposals p ON dv.proposal_id = p.id
                    WHERE dv.proposal_id = $1
                ",
                )
                .bind::<diesel::sql_types::Text, _>(&rejected_event.proposal_id)
                .load::<DelegateVoteResult>(tx_conn)
                .await?;

                // Update each delegate's win/loss count
                for vote in &delegate_votes {
                    if !vote.approve {
                        // Delegate voted against - they win since proposal was rejected
                        diesel::update(crate::schema::delegates::table)
                            .filter(crate::schema::delegates::address.eq(&vote.delegate_address))
                            .set(
                                crate::schema::delegates::sided_winning_proposals
                                    .eq(crate::schema::delegates::sided_winning_proposals + 1),
                            )
                            .execute(tx_conn)
                            .await?;
                    } else {
                        // Delegate voted for - they lose since proposal was rejected
                        diesel::update(crate::schema::delegates::table)
                            .filter(crate::schema::delegates::address.eq(&vote.delegate_address))
                            .set(
                                crate::schema::delegates::sided_losing_proposals
                                    .eq(crate::schema::delegates::sided_losing_proposals + 1),
                            )
                            .execute(tx_conn)
                            .await?;
                    }
                }

                // Record this event in the governance_events table
                let proposal_type_query =
                    diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
                        .bind::<diesel::sql_types::Text, _>(&rejected_event.proposal_id)
                        .load::<ProposalTypeResult>(tx_conn)
                        .await?;

                if let Some(result) = proposal_type_query.get(0) {
                    let governance_event = NewGovernanceEvent {
                        event_type: "ProposalRejectedByCommunityEvent".to_string(),
                        registry_type: result.proposal_type,
                        event_data: event.clone(),
                        event_id: event_id.to_string(),
                        created_at: Utc::now(),
                        anonymous_voting_related: None,
                    };

                    diesel::insert_into(crate::schema::governance_events::table)
                        .values(&governance_event)
                        .execute(tx_conn)
                        .await?;
                } else {
                    error!(
                        "Failed to find proposal type for proposal ID: {}",
                        rejected_event.proposal_id
                    );
                }

                info!("Processed proposal rejected by community event successfully");

                Ok::<_, anyhow::Error>(())
            })
        })
        .await?;

    Ok(())
}

/// Process a proposal approved event
pub async fn process_proposal_approved_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing proposal approved event");

    // Parse the event
    let approved_event = parse_json_event::<ProposalApprovedEvent>(event)?;

    // Begin transaction
    conn.build_transaction()
        .run(|tx_conn| {
            Box::pin(async move {
                // Update proposal status
                diesel::update(crate::schema::proposals::table)
                    .filter(crate::schema::proposals::id.eq(&approved_event.proposal_id))
                    .set((
                        crate::schema::proposals::status.eq(GOVERNANCE_STATUS_APPROVED as i16),
                        crate::schema::proposals::community_votes_for.eq(approved_event.votes_for as i64),
                        crate::schema::proposals::community_votes_against.eq(approved_event.votes_against as i64),
                    ))
                    .execute(tx_conn)
                    .await?;

                // Update delegate stats
                // This is more complex as we need to find all delegates who voted and update their stats
                // First, find all delegates who voted on this proposal
                let delegate_votes = diesel::sql_query(
                    "
                    SELECT dv.delegate_address, dv.approve, p.submitter
                    FROM delegate_votes dv
                    JOIN proposals p ON dv.proposal_id = p.id
                    WHERE dv.proposal_id = $1
                ",
                )
                .bind::<diesel::sql_types::Text, _>(&approved_event.proposal_id)
                .load::<DelegateVoteResult>(tx_conn)
                .await?;

                // Update each delegate's win/loss count
                for vote in &delegate_votes {
                    if vote.approve {
                        diesel::update(crate::schema::delegates::table)
                            .filter(crate::schema::delegates::address.eq(&vote.delegate_address))
                            .set(
                                crate::schema::delegates::sided_winning_proposals
                                    .eq(crate::schema::delegates::sided_winning_proposals + 1),
                            )
                            .execute(tx_conn)
                            .await?;
                    } else {
                        diesel::update(crate::schema::delegates::table)
                            .filter(crate::schema::delegates::address.eq(&vote.delegate_address))
                            .set(
                                crate::schema::delegates::sided_losing_proposals
                                    .eq(crate::schema::delegates::sided_losing_proposals + 1),
                            )
                            .execute(tx_conn)
                            .await?;
                    }
                }

                // Record this event in the governance_events table
                // We need to get the proposal type from the proposal
                let proposal_type_query =
                    diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
                        .bind::<diesel::sql_types::Text, _>(&approved_event.proposal_id)
                        .load::<ProposalTypeResult>(tx_conn)
                        .await?;

                if let Some(result) = proposal_type_query.get(0) {
                    let governance_event = NewGovernanceEvent {
                        event_type: "ProposalApprovedEvent".to_string(),
                        registry_type: result.proposal_type,
                        event_data: event.clone(),
                        event_id: event_id.to_string(),
                        created_at: Utc::now(),
                        anonymous_voting_related: None,
                    };

                    diesel::insert_into(crate::schema::governance_events::table)
                        .values(&governance_event)
                        .execute(tx_conn)
                        .await?;
                } else {
                    error!(
                        "Failed to find proposal type for proposal ID: {}",
                        approved_event.proposal_id
                    );
                }

                info!("Processed proposal approved event successfully");

                Ok::<_, anyhow::Error>(())
            })
        })
        .await?;

    Ok(())
}

/// Process a proposal implemented event
pub async fn process_proposal_implemented_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing proposal implemented event");

    // Parse the event
    let implemented_event = parse_json_event::<ProposalImplementedEvent>(event)?;

    // Update proposal status
    let result = diesel::update(crate::schema::proposals::table)
        .filter(crate::schema::proposals::id.eq(&implemented_event.proposal_id))
        .set((
            crate::schema::proposals::status.eq(GOVERNANCE_STATUS_IMPLEMENTED as i16),
            crate::schema::proposals::implementation_time
                .eq(implemented_event.implementation_time as i64),
            crate::schema::proposals::implemented_description
                .eq(&implemented_event.implemented_description),
        ))
        .execute(conn)
        .await?;

    info!(
        "Processed proposal implemented event: {} rows affected",
        result
    );

    // Record this event in the governance_events table
    // We need to get the proposal type from the proposal
    let proposal_type_query =
        diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
            .bind::<diesel::sql_types::Text, _>(&implemented_event.proposal_id)
            .load::<ProposalTypeResult>(conn)
            .await?;

    if let Some(result) = proposal_type_query.get(0) {
        let governance_event = NewGovernanceEvent {
            event_type: "ProposalImplementedEvent".to_string(),
            registry_type: result.proposal_type,
            event_data: event.clone(),
            event_id: event_id.to_string(),
            created_at: Utc::now(),
            anonymous_voting_related: None,
        };

        diesel::insert_into(crate::schema::governance_events::table)
            .values(&governance_event)
            .execute(conn)
            .await?;
    } else {
        error!(
            "Failed to find proposal type for proposal ID: {}",
            implemented_event.proposal_id
        );
    }

    Ok(())
}

/// Process a rewards distributed event
pub async fn process_rewards_distributed_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing rewards distributed event");

    // Parse the event
    let rewards_event = parse_json_event::<RewardsDistributedEvent>(event)?;

    // Note: The RewardsDistributedEvent contains aggregate data (total_reward, recipient_count)
    // but the database model expects individual recipient records. Since we don't have
    // individual recipient addresses from the event, we create an aggregate record
    // with a placeholder recipient_address indicating this is an aggregate distribution.
    let new_distribution = NewRewardDistribution {
        proposal_id: rewards_event.proposal_id.clone(),
        recipient_address: format!("aggregate_{}", rewards_event.proposal_id), // Aggregate placeholder
        amount: rewards_event.total_reward as i64,
        distribution_time: rewards_event.distribution_time as i64,
        distribution_type: Some(format!("aggregate_{}_recipients", rewards_event.recipient_count)),
        transaction_id: event_id.to_string(),
    };

    let result = diesel::insert_into(crate::schema::reward_distributions::table)
        .values(&new_distribution)
        .execute(conn)
        .await?;

    info!(
        "Processed rewards distributed event: {} rows affected",
        result
    );

    // Record this event in the governance_events table
    // We need to get the proposal type from the proposal
    let proposal_type_query =
        diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
            .bind::<diesel::sql_types::Text, _>(&rewards_event.proposal_id)
            .load::<ProposalTypeResult>(conn)
            .await?;

    if let Some(result) = proposal_type_query.get(0) {
        let governance_event = NewGovernanceEvent {
            event_type: "RewardsDistributedEvent".to_string(),
            registry_type: result.proposal_type,
            event_data: event.clone(),
            event_id: event_id.to_string(),
            created_at: Utc::now(),
            anonymous_voting_related: None,
        };

        diesel::insert_into(crate::schema::governance_events::table)
            .values(&governance_event)
            .execute(conn)
            .await?;
    } else {
        error!(
            "Failed to find proposal type for proposal ID: {}",
            rewards_event.proposal_id
        );
    }

    Ok(())
}

/// Process an anonymous vote event  
pub async fn process_anonymous_vote_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing anonymous vote event");

    // Parse the event
    let vote_event =
        parse_json_event::<crate::events::governance_event_types::AnonymousVoteEvent>(event)?;

    // Create new anonymous vote record
    let new_vote = crate::models::governance::NewAnonymousVote {
        proposal_id: vote_event.proposal_id.clone(),
        voter_address: vote_event.voter.clone(),
        encrypted_vote_data: Some(vote_event.encrypted_vote_data.clone()),
        submitted_at: vote_event.vote_time as i64,
        decryption_status: crate::ANONYMOUS_VOTE_STATUS_PENDING as i16,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };

    // Insert the vote record
    let result = diesel::insert_into(crate::schema::anonymous_votes::table)
        .values(&new_vote)
        .execute(conn)
        .await?;

    info!("Processed anonymous vote event: {} rows affected", result);

    // Update the anonymous voters count on the proposal
    diesel::sql_query(
        "
        UPDATE proposals 
        SET anonymous_voters_count = COALESCE(anonymous_voters_count, 0) + 1
        WHERE id = $1
    ",
    )
    .bind::<diesel::sql_types::Text, _>(&vote_event.proposal_id)
    .execute(conn)
    .await?;

    // Record this event in the governance_events table
    let proposal_type_query =
        diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
            .bind::<diesel::sql_types::Text, _>(&vote_event.proposal_id)
            .load::<crate::db::query_types::ProposalTypeResult>(conn)
            .await?;

    if let Some(result) = proposal_type_query.get(0) {
        let governance_event = NewGovernanceEvent {
            event_type: "AnonymousVoteEvent".to_string(),
            registry_type: result.proposal_type,
            event_data: event.clone(),
            event_id: event_id.to_string(),
            created_at: Utc::now(),
            anonymous_voting_related: Some(true),
        };

        diesel::insert_into(crate::schema::governance_events::table)
            .values(&governance_event)
            .execute(conn)
            .await?;
    } else {
        error!(
            "Failed to find proposal type for proposal ID: {}",
            vote_event.proposal_id
        );
    }

    Ok(())
}

/// Process a vote decryption failed event
pub async fn process_vote_decryption_failed_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing vote decryption failed event");

    // Parse the event
    let failure_event = parse_json_event::<
        crate::events::governance_event_types::VoteDecryptionFailedEvent,
    >(event)?;

    // Create decryption failure record
    let new_failure = crate::models::governance::NewVoteDecryptionFailure {
        proposal_id: failure_event.proposal_id.clone(),
        voter_address: failure_event.voter.clone(),
        failure_reason: failure_event.failure_reason.clone(),
        attempted_at: failure_event.timestamp as i64,
        encrypted_vote_length: None,
        transaction_id: event_id.to_string(),
    };

    let result = diesel::insert_into(crate::schema::vote_decryption_failures::table)
        .values(&new_failure)
        .execute(conn)
        .await?;

    info!(
        "Processed vote decryption failed event: {} rows affected",
        result
    );

    // Record this event in the governance_events table
    let proposal_type_query =
        diesel::sql_query("SELECT proposal_type FROM proposals WHERE id = $1")
            .bind::<diesel::sql_types::Text, _>(&failure_event.proposal_id)
            .load::<crate::db::query_types::ProposalTypeResult>(conn)
            .await?;

    if let Some(result) = proposal_type_query.get(0) {
        let governance_event = NewGovernanceEvent {
            event_type: "VoteDecryptionFailedEvent".to_string(),
            registry_type: result.proposal_type,
            event_data: event.clone(),
            event_id: event_id.to_string(),
            created_at: Utc::now(),
            anonymous_voting_related: Some(true),
        };

        diesel::insert_into(crate::schema::governance_events::table)
            .values(&governance_event)
            .execute(conn)
            .await?;
    } else {
        error!(
            "Failed to find proposal type for proposal ID: {}",
            failure_event.proposal_id
        );
    }

    Ok(())
}

/// Process a governance parameters updated event
pub async fn process_governance_parameters_updated_event(
    conn: &mut DbConnection,
    event: &Value,
    event_id: &str,
) -> Result<()> {
    debug!("Processing governance parameters updated event");

    // Parse the event
    let params_event = parse_json_event::<GovernanceParametersUpdatedEvent>(event)?;

    // Insert or update registry with the new parameters
    let new_registry = NewGovernanceRegistry {
        registry_type: params_event.registry_type as i16,
        delegate_count: params_event.delegate_count as i64,
        delegate_term_epochs: params_event.delegate_term_epochs as i64,
        proposal_submission_cost: params_event.proposal_submission_cost as i64,
        min_on_chain_age_days: params_event.min_on_chain_age_days as i64,
        max_votes_per_user: params_event.max_votes_per_user as i64,
        quadratic_base_cost: params_event.quadratic_base_cost as i64,
        voting_period_epochs: params_event.voting_period_epochs as i64,
        quorum_votes: params_event.quorum_votes as i64,
        updated_at: params_event.timestamp as i64,
        transaction_id: event_id.to_string(),
    };

    let result = diesel::insert_into(crate::schema::governance_registries::table)
        .values(&new_registry)
        .on_conflict(crate::schema::governance_registries::registry_type)
        .do_update()
        .set((
            crate::schema::governance_registries::delegate_count.eq(new_registry.delegate_count),
            crate::schema::governance_registries::delegate_term_epochs
                .eq(new_registry.delegate_term_epochs),
            crate::schema::governance_registries::proposal_submission_cost
                .eq(new_registry.proposal_submission_cost),
            crate::schema::governance_registries::min_on_chain_age_days
                .eq(new_registry.min_on_chain_age_days),
            crate::schema::governance_registries::max_votes_per_user
                .eq(new_registry.max_votes_per_user),
            crate::schema::governance_registries::quadratic_base_cost
                .eq(new_registry.quadratic_base_cost),
            crate::schema::governance_registries::voting_period_epochs
                .eq(new_registry.voting_period_epochs),
            crate::schema::governance_registries::quorum_votes.eq(new_registry.quorum_votes),
            crate::schema::governance_registries::updated_at.eq(new_registry.updated_at),
            crate::schema::governance_registries::transaction_id.eq(event_id.to_string()),
        ))
        .execute(conn)
        .await?;

    info!(
        "Processed governance parameters updated event: {} rows affected",
        result
    );

    // Record this event in the governance_events table
    // This preserves the updated_by field in the event data
    let governance_event = NewGovernanceEvent {
        event_type: "GovernanceParametersUpdatedEvent".to_string(),
        registry_type: params_event.registry_type as i16,
        event_data: event.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
        anonymous_voting_related: None,
    };

    diesel::insert_into(crate::schema::governance_events::table)
        .values(&governance_event)
        .execute(conn)
        .await?;

    Ok(())
}
