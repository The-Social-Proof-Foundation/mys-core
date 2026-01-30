// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::social::db::DbPool;
use crate::social::models::governance::*;
use crate::social::schema::{
    community_votes, delegate_ratings, delegate_votes, delegates, governance_events,
    governance_registries, nominated_delegates, proposals, reward_distributions,
};

// Query parameters
#[derive(Debug, Deserialize)]
pub struct ProposalListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub status: Option<i16>,
    pub proposal_type: Option<i16>,
    pub submitter: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DelegateListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub registry_type: Option<i16>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct NomineeListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub registry_type: Option<i16>,
    pub status: Option<i16>,
}

// Response types
#[derive(Debug, Serialize)]
pub struct ProposalDetail {
    #[serde(flatten)]
    pub proposal: Proposal,
    pub delegate_votes: Vec<DelegateVote>,
    pub community_votes_count: i64,
    pub reward_distributions: Vec<RewardDistribution>,
}

// Add QueryableByName structs for raw SQL queries
#[derive(Debug, QueryableByName)]
struct AnonymousVotingStatsRow {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    total_anonymous_votes: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    successfully_decrypted: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    failed_decryptions: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    anonymous_votes_for: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    anonymous_votes_against: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pending_decryption: i64,
}

#[derive(Debug, QueryableByName)]
struct AnonymousVotingTrendRow {
    #[diesel(sql_type = diesel::sql_types::Text)]
    day: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    total_votes: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    successful_decryptions: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    failed_decryptions: i64,
    #[diesel(sql_type = diesel::sql_types::Double)]
    success_rate: f64,
}

// ============= PROPOSALS =============

/// List proposals with optional filtering
pub async fn list_proposals(
    State(pool): State<DbPool>,
    params: axum::extract::Query<ProposalListParams>,
) -> Result<Json<Vec<Proposal>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let limit = params.limit.unwrap_or(20);
    let offset = params.offset.unwrap_or(0);

    let mut query = proposals::table
        .order_by(proposals::submission_time.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if let Some(status) = params.status {
        query = query.filter(proposals::status.eq(status));
    }

    if let Some(proposal_type) = params.proposal_type {
        query = query.filter(proposals::proposal_type.eq(proposal_type));
    }

    if let Some(ref submitter) = params.submitter {
        query = query.filter(proposals::submitter.eq(submitter));
    }

    let proposals_list = query.load::<Proposal>(&mut conn).await.map_err(|e| {
        error!("Error loading proposals: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(proposals_list))
}

/// Get proposal by ID with details (votes, distributions)
pub async fn get_proposal_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<ProposalDetail>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let proposal = proposals::table
        .filter(proposals::id.eq(&id))
        .first::<Proposal>(&mut conn)
        .await
        .map_err(|e| {
            if let diesel::result::Error::NotFound = e {
                StatusCode::NOT_FOUND
            } else {
                error!("Error loading proposal: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    // Get delegate votes for this proposal
    let delegate_votes = delegate_votes::table
        .filter(delegate_votes::proposal_id.eq(&id))
        .order_by(delegate_votes::vote_time.desc())
        .load::<DelegateVote>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading delegate votes: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Count community votes
    let community_votes_count = community_votes::table
        .filter(community_votes::proposal_id.eq(&id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error counting community votes: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get reward distributions
    let reward_distributions = reward_distributions::table
        .filter(reward_distributions::proposal_id.eq(&id))
        .order_by(reward_distributions::distribution_time.desc())
        .load::<RewardDistribution>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading reward distributions: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let proposal_detail = ProposalDetail {
        proposal,
        delegate_votes,
        community_votes_count,
        reward_distributions,
    };

    Ok(Json(proposal_detail))
}

// Get community votes for a proposal
pub async fn get_proposal_community_votes(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<Vec<CommunityVote>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let votes = community_votes::table
        .filter(community_votes::proposal_id.eq(&id))
        .order_by(community_votes::vote_time.desc())
        .load::<CommunityVote>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading community votes: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(votes))
}

// ============= DELEGATES =============

/// List delegates with optional filtering
pub async fn list_delegates(
    State(pool): State<DbPool>,
    params: axum::extract::Query<DelegateListParams>,
) -> Result<Json<Vec<Delegate>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    let mut query = delegates::table
        .select(Delegate::as_select())
        .order_by(delegates::upvotes.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if let Some(registry_type) = params.registry_type {
        query = query.filter(delegates::registry_type.eq(registry_type));
    }

    if let Some(is_active) = params.is_active {
        query = query.filter(delegates::is_active.eq(is_active));
    }

    let delegates_list = query.load::<Delegate>(&mut conn).await.map_err(|e| {
        error!("Error loading delegates: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(delegates_list))
}

/// Get delegate by address
pub async fn get_delegate_by_address(
    State(pool): State<DbPool>,
    Path(address): Path<String>,
) -> Result<Json<Delegate>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let delegate = delegates::table
        .select(Delegate::as_select())
        .filter(delegates::address.eq(&address))
        .first::<Delegate>(&mut conn)
        .await
        .map_err(|e| {
            if let diesel::result::Error::NotFound = e {
                StatusCode::NOT_FOUND
            } else {
                error!("Error loading delegate: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok(Json(delegate))
}

/// Get proposals reviewed by a delegate
pub async fn get_delegate_proposals(
    State(pool): State<DbPool>,
    Path(address): Path<String>,
) -> Result<Json<Vec<Proposal>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // First get all proposals this delegate voted on
    let proposal_ids = delegate_votes::table
        .filter(delegate_votes::delegate_address.eq(&address))
        .select(delegate_votes::proposal_id)
        .load::<String>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading delegate votes: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if proposal_ids.is_empty() {
        return Ok(Json(vec![]));
    }

    // Then get the actual proposals
    let proposals_list = proposals::table
        .filter(proposals::id.eq_any(proposal_ids))
        .order_by(proposals::submission_time.desc())
        .load::<Proposal>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading proposals: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(proposals_list))
}

/// Get delegate ratings
pub async fn get_delegate_ratings(
    State(pool): State<DbPool>,
    Path(address): Path<String>,
) -> Result<Json<Vec<DelegateRating>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let ratings = delegate_ratings::table
        .filter(delegate_ratings::target_address.eq(&address))
        .order_by(delegate_ratings::rated_at.desc())
        .load::<DelegateRating>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading delegate ratings: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ratings))
}

// ============= NOMINATED DELEGATES =============

/// List nominated delegates with optional filtering
pub async fn list_nominees(
    State(pool): State<DbPool>,
    params: axum::extract::Query<NomineeListParams>,
) -> Result<Json<Vec<NominatedDelegate>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    let mut query = nominated_delegates::table
        .select(NominatedDelegate::as_select())
        .order_by(nominated_delegates::upvotes.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if let Some(registry_type) = params.registry_type {
        query = query.filter(nominated_delegates::registry_type.eq(registry_type));
    }

    if let Some(status) = params.status {
        query = query.filter(nominated_delegates::status.eq(status));
    }

    let nominees_list = query
        .load::<NominatedDelegate>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading nominees: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(nominees_list))
}

// ============= GOVERNANCE REGISTRIES =============

/// List governance registries
pub async fn list_registries(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<GovernanceRegistry>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let registries = governance_registries::table
        .order_by(governance_registries::registry_type)
        .load::<GovernanceRegistry>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading registries: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(registries))
}

/// Get a specific registry by type
pub async fn get_registry_by_type(
    State(pool): State<DbPool>,
    Path(registry_type): Path<i16>,
) -> Result<Json<GovernanceRegistry>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let registry = governance_registries::table
        .filter(governance_registries::registry_type.eq(registry_type))
        .first::<GovernanceRegistry>(&mut conn)
        .await
        .map_err(|e| {
            if let diesel::result::Error::NotFound = e {
                StatusCode::NOT_FOUND
            } else {
                error!("Error loading registry: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok(Json(registry))
}

// ============= GOVERNANCE EVENTS =============

/// List governance events
pub async fn list_governance_events(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<GovernanceEvent>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let events = governance_events::table
        .order_by(governance_events::created_at.desc())
        .limit(100)
        .load::<GovernanceEvent>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading governance events: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(events))
}

// ============= ANONYMOUS VOTING ENDPOINTS =============

#[derive(Debug, Serialize)]
pub struct AnonymousVotingStats {
    pub total_anonymous_votes: i64,
    pub successfully_decrypted: i64,
    pub failed_decryptions: i64,
    pub anonymous_votes_for: i64,
    pub anonymous_votes_against: i64,
    pub pending_decryption: i64,
}

#[derive(Debug, Deserialize)]
pub struct AnonymousVotingParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub proposal_id: Option<String>,
    pub decryption_status: Option<i16>,
}

/// Get anonymous voting statistics for a proposal
pub async fn get_proposal_anonymous_stats(
    State(pool): State<DbPool>,
    Path(proposal_id): Path<String>,
) -> Result<Json<AnonymousVotingStats>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stats = diesel::sql_query(
        "
        SELECT 
            COUNT(*) as total_anonymous_votes,
            COUNT(*) FILTER (WHERE decrypted = true) as successfully_decrypted,
            COUNT(*) FILTER (WHERE decryption_status = $2) as failed_decryptions,
            COUNT(*) FILTER (WHERE decrypted_vote = 1) as anonymous_votes_for,
            COUNT(*) FILTER (WHERE decrypted_vote = 0) as anonymous_votes_against,
            COUNT(*) FILTER (WHERE decryption_status = $3) as pending_decryption
        FROM anonymous_votes 
        WHERE proposal_id = $1
    ",
    )
    .bind::<diesel::sql_types::Text, _>(&proposal_id)
    .bind::<diesel::sql_types::SmallInt, _>(crate::social::ANONYMOUS_VOTE_STATUS_FAILED as i16)
    .bind::<diesel::sql_types::SmallInt, _>(crate::social::ANONYMOUS_VOTE_STATUS_PENDING as i16)
    .get_result::<AnonymousVotingStatsRow>(&mut conn)
    .await
    .map_err(|e| {
        error!("Error loading anonymous voting stats: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = AnonymousVotingStats {
        total_anonymous_votes: stats.total_anonymous_votes,
        successfully_decrypted: stats.successfully_decrypted,
        failed_decryptions: stats.failed_decryptions,
        anonymous_votes_for: stats.anonymous_votes_for,
        anonymous_votes_against: stats.anonymous_votes_against,
        pending_decryption: stats.pending_decryption,
    };

    Ok(Json(response))
}

/// Get anonymous votes for a proposal
pub async fn get_proposal_anonymous_votes(
    State(pool): State<DbPool>,
    Path(proposal_id): Path<String>,
    params: axum::extract::Query<AnonymousVotingParams>,
) -> Result<Json<Vec<crate::social::models::governance::AnonymousVote>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    let mut query = crate::social::schema::anonymous_votes::table
        .filter(crate::social::schema::anonymous_votes::proposal_id.eq(&proposal_id))
        .order_by(crate::social::schema::anonymous_votes::time.desc())
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if let Some(status) = params.decryption_status {
        query = query.filter(crate::social::schema::anonymous_votes::decryption_status.eq(status));
    }

    let votes = query
        .load::<crate::social::models::governance::AnonymousVote>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading anonymous votes: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(votes))
}

/// Get decryption failures for a proposal
pub async fn get_proposal_decryption_failures(
    State(pool): State<DbPool>,
    Path(proposal_id): Path<String>,
) -> Result<Json<Vec<crate::social::models::governance::VoteDecryptionFailure>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let failures = crate::social::schema::vote_decryption_failures::table
        .filter(crate::social::schema::vote_decryption_failures::proposal_id.eq(&proposal_id))
        .order_by(crate::social::schema::vote_decryption_failures::time.desc())
        .limit(100) // Limit failures for performance
        .load::<crate::social::models::governance::VoteDecryptionFailure>(&mut conn)
        .await
        .map_err(|e| {
            error!("Error loading decryption failures: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(failures))
}

#[derive(Debug, Deserialize)]
pub struct TrendsParams {
    pub days: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct AnonymousVotingTrend {
    pub day: String,
    pub total_votes: i64,
    pub successful_decryptions: i64,
    pub failed_decryptions: i64,
    pub success_rate: f64,
}

/// Get anonymous voting trends over time
pub async fn get_anonymous_voting_trends(
    State(pool): State<DbPool>,
    Query(params): Query<TrendsParams>,
) -> Result<Json<Vec<AnonymousVotingTrend>>, StatusCode> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let days = params.days.unwrap_or(30);

    let trends = diesel::sql_query("
        SELECT 
            day::text,
            COALESCE(SUM(total_anonymous_votes), 0) as total_votes,
            COALESCE(SUM(successfully_decrypted), 0) as successful_decryptions,
            COALESCE(SUM(failed_decryptions), 0) as failed_decryptions,
            CASE 
                WHEN SUM(total_anonymous_votes) > 0 THEN 
                    ROUND((SUM(successfully_decrypted)::float / SUM(total_anonymous_votes) * 100)::numeric, 2)
                ELSE 0 
            END as success_rate
        FROM anonymous_voting_daily_stats 
        WHERE day >= NOW() - INTERVAL '$1 days'
        GROUP BY day
        ORDER BY day DESC
    ")
    .bind::<diesel::sql_types::Integer, _>(days)
    .get_results::<AnonymousVotingTrendRow>(&mut conn)
    .await
    .map_err(|e| {
        error!("Error loading anonymous voting trends: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = trends
        .into_iter()
        .map(|row| AnonymousVotingTrend {
            day: row.day,
            total_votes: row.total_votes,
            successful_decryptions: row.successful_decryptions,
            failed_decryptions: row.failed_decryptions,
            success_rate: row.success_rate,
        })
        .collect();

    Ok(Json(response))
}
