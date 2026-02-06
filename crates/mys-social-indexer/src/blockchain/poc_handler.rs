// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use chrono::Utc;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Int2, Nullable, Text};
use diesel_async::RunQueryDsl;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::blockchain::listener::BlockchainEvent;
use crate::db::{Database, DbConnection};
use crate::events::event_utils::extract_event_fields;
use crate::events::poc_event_types::{
    AnalysisSubmittedEvent, DisputeVoteCastEvent, PocBadgeIssuedEvent, PocConfigUpdatedEvent,
    PocDisputeResolvedEvent, PocDisputeSubmittedEvent, RevenueRedirectionActivatedEvent,
    TokenPoolSyncNeededEvent, VotingRewardClaimedEvent,
};
use crate::events::poc_events::{
    validate_analysis_submitted_event, validate_badge_issued_event, validate_config_updated_event,
    validate_dispute_submitted_event, validate_redirection_activated_event,
    validate_vote_cast_event, PocEventError,
};
use crate::schema;

/// Handler for Proof of Creativity (PoC) blockchain events.
pub struct PocEventHandler {
    db: Arc<Database>,
    rx: mpsc::Receiver<BlockchainEvent>,
    worker_name: String,
}

impl PocEventHandler {
    pub fn new(
        db: Arc<Database>,
        rx: mpsc::Receiver<BlockchainEvent>,
        worker_name: String,
    ) -> Self {
        Self {
            db,
            rx,
            worker_name,
        }
    }

    async fn get_connection(&self) -> Result<DbConnection> {
        self.db
            .get_connection()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }

    fn parse_event<T: serde::de::DeserializeOwned>(value: &Value) -> Result<T> {
        let fields = extract_event_fields(value)?;
        serde_json::from_value::<T>(fields)
            .map_err(|e| anyhow!("Failed to deserialize PoC event payload: {}", e))
    }

    fn capture_evidence(fields: &Value) -> String {
        fields
            .get("evidence")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    }

    async fn handle_analysis_submitted(&self, event: &BlockchainEvent) -> Result<()> {
        let raw_fields = extract_event_fields(&event.data)?;
        let parsed: AnalysisSubmittedEvent =
            serde_json::from_value(raw_fields.clone()).map_err(|e| anyhow!(e))?;
        validate_analysis_submitted_event(&parsed)?;

        let mut model = parsed
            .into_model()
            .map_err(|e| anyhow!("Failed to convert AnalysisSubmittedEvent: {}", e))?;
        model.transaction_id = event.tx_digest.clone();

        let mut conn = self.get_connection().await?;
        diesel::insert_into(schema::poc_analysis_results::table)
            .values(&model)
            .execute(&mut conn)
            .await?;

        // Update post record with PoC metadata fields
        let evidence_urls_json = parsed.evidence_urls.as_ref()
            .map(|urls| serde_json::to_value(urls).unwrap_or(serde_json::json!(null)));

        diesel::update(schema::posts::table)
            .filter(schema::posts::post_id.eq(&parsed.post_id))
            .set((
                schema::posts::poc_reasoning.eq(parsed.reasoning.as_ref()),
                schema::posts::poc_evidence_urls.eq(&evidence_urls_json),
                schema::posts::poc_similarity_score.eq(Some(parsed.highest_similarity_score as i64)),
                schema::posts::poc_media_type.eq(Some(parsed.media_type as i16)),
                schema::posts::poc_oracle_address.eq(Some(&parsed.oracle_address)),
                schema::posts::poc_analyzed_at.eq(Some(parsed.timestamp as i64)),
            ))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    async fn handle_badge_issued(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<PocBadgeIssuedEvent>(&event.data)?;
        validate_badge_issued_event(&parsed)?;

        let mut model = parsed
            .into_model()
            .map_err(|e| anyhow!("Failed to convert PocBadgeIssuedEvent: {}", e))?;
        model.transaction_id = event.tx_digest.clone();

        let mut conn = self.get_connection().await?;
        diesel::insert_into(schema::poc_badges::table)
            .values(&model)
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    async fn handle_revenue_redirection(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<RevenueRedirectionActivatedEvent>(&event.data)?;
        validate_redirection_activated_event(&parsed)?;

        let mut model = parsed
            .into_model()
            .map_err(|e| anyhow!("Failed to convert RevenueRedirectionActivatedEvent: {}", e))?;
        model.transaction_id = event.tx_digest.clone();

        let mut conn = self.get_connection().await?;
        diesel::insert_into(schema::poc_revenue_redirections::table)
            .values(&model)
            .execute(&mut conn)
            .await?;

        // Update post record with revenue redirection fields
        // Note: accused_post_id is the post that will redirect revenue
        diesel::update(schema::posts::table)
            .filter(schema::posts::post_id.eq(&parsed.accused_post_id))
            .set((
                schema::posts::revenue_redirect_to.eq(Some(&parsed.original_post_id)),
                schema::posts::revenue_redirect_percentage.eq(Some(parsed.redirect_percentage as i64)),
            ))
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    async fn handle_dispute_submitted(&self, event: &BlockchainEvent) -> Result<()> {
        let fields = extract_event_fields(&event.data)?;
        let parsed: PocDisputeSubmittedEvent = serde_json::from_value(fields.clone())?;
        validate_dispute_submitted_event(&parsed)?;

        let evidence = Self::capture_evidence(&fields);
        let mut model = parsed
            .into_model(evidence)
            .map_err(|e| anyhow!("Failed to convert PocDisputeSubmittedEvent: {}", e))?;
        model.transaction_id = event.tx_digest.clone();

        let mut conn = self.get_connection().await?;
        diesel::insert_into(schema::poc_disputes::table)
            .values(&model)
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    async fn handle_vote_cast(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<DisputeVoteCastEvent>(&event.data)?;
        validate_vote_cast_event(&parsed)?;

        let mut model = parsed
            .into_model()
            .map_err(|e| anyhow!("Failed to convert DisputeVoteCastEvent: {}", e))?;
        model.transaction_id = event.tx_digest.clone();

        let mut conn = self.get_connection().await?;
        diesel::insert_into(schema::poc_dispute_votes::table)
            .values(&model)
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    async fn handle_dispute_resolved(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<PocDisputeResolvedEvent>(&event.data)?;
        let (resolution, winning_side, total_winning, total_losing, resolved_at) =
            parsed.get_dispute_update_fields();

        let mut conn = self.get_connection().await?;

        let update_sql = "UPDATE poc_disputes SET status = $1, resolution = $2, winning_side = $3, total_winning_stake = $4, total_losing_stake = $5, resolved_at = $6, transaction_id = $7, time = NOW() \
            WHERE dispute_id = $8 AND time = (SELECT time FROM poc_disputes WHERE dispute_id = $8 ORDER BY time DESC LIMIT 1)";

        diesel::sql_query(update_sql)
            .bind::<Int2, _>(resolution)
            .bind::<Nullable<Int2>, _>(Some(resolution))
            .bind::<Nullable<Int2>, _>(Some(winning_side))
            .bind::<Nullable<BigInt>, _>(Some(total_winning as i64))
            .bind::<Nullable<BigInt>, _>(Some(total_losing as i64))
            .bind::<Nullable<BigInt>, _>(Some(resolved_at as i64))
            .bind::<Text, _>(&event.tx_digest)
            .bind::<Text, _>(&parsed.dispute_id)
            .execute(&mut conn)
            .await?;

        if parsed.should_revoke_badge() {
            let revoke_sql = "UPDATE poc_badges SET revoked = TRUE, revoked_at = $1, transaction_id = $2, time = NOW() \
                WHERE post_id = $3 AND time = (SELECT time FROM poc_badges WHERE post_id = $3 ORDER BY time DESC LIMIT 1)";
            diesel::sql_query(revoke_sql)
                .bind::<Nullable<BigInt>, _>(Some(resolved_at as i64))
                .bind::<Text, _>(&event.tx_digest)
                .bind::<Text, _>(&parsed.post_id)
                .execute(&mut conn)
                .await?;
        }

        if parsed.should_remove_redirection() {
            let remove_sql = "UPDATE poc_revenue_redirections SET removed = TRUE, removed_at = $1, transaction_id = $2, time = NOW() \
                WHERE accused_post_id = $3 AND time = (SELECT time FROM poc_revenue_redirections WHERE accused_post_id = $3 ORDER BY time DESC LIMIT 1)";
            diesel::sql_query(remove_sql)
                .bind::<Nullable<BigInt>, _>(Some(resolved_at as i64))
                .bind::<Text, _>(&event.tx_digest)
                .bind::<Text, _>(&parsed.post_id)
                .execute(&mut conn)
                .await?;
        }

        Ok(())
    }

    async fn handle_reward_claimed(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<VotingRewardClaimedEvent>(&event.data)?;

        let mut conn = self.get_connection().await?;
        let update_sql = "UPDATE poc_dispute_votes SET reward_claimed = $1, reward_amount = $2, transaction_id = $3, time = NOW() \
            WHERE dispute_id = $4 AND voter = $5 AND time = (SELECT time FROM poc_dispute_votes WHERE dispute_id = $4 AND voter = $5 ORDER BY time DESC LIMIT 1)";

        diesel::sql_query(update_sql)
            .bind::<Bool, _>(true)
            .bind::<Nullable<BigInt>, _>(Some(parsed.reward_amount as i64))
            .bind::<Text, _>(&event.tx_digest)
            .bind::<Text, _>(&parsed.dispute_id)
            .bind::<Text, _>(&parsed.voter)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    async fn handle_config_updated(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<PocConfigUpdatedEvent>(&event.data)?;
        validate_config_updated_event(&parsed)?;

        // Use timestamp_ms from BlockchainEvent (in milliseconds) for correct timestamp
        // oracle_address and dispute_protocol_fee are now included in the event
        let mut model = parsed
            .into_model(event.timestamp_ms)
            .map_err(|e| anyhow!("Failed to convert PocConfigUpdatedEvent: {}", e))?;
        model.transaction_id = event.tx_digest.clone();

        let mut conn = self.get_connection().await?;
        diesel::insert_into(schema::poc_configuration::table)
            .values(&model)
            .execute(&mut conn)
            .await?;
        Ok(())
    }

    async fn update_progress(&self) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now().naive_utc();

        let progress = crate::models::indexer::NewIndexerProgress {
            id: self.worker_name.clone(),
            last_checkpoint_processed: 0,
            last_processed_at: now,
        };

        diesel::insert_into(schema::indexer_progress::table)
            .values(&progress)
            .on_conflict(schema::indexer_progress::id)
            .do_update()
            .set((
                schema::indexer_progress::last_checkpoint_processed
                    .eq(progress.last_checkpoint_processed),
                schema::indexer_progress::last_processed_at.eq(progress.last_processed_at),
            ))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    fn is_poc_event(event_type: &str) -> bool {
        event_type.contains("::poc::")
            || event_type.contains("::proof_of_creativity::")
            || event_type.ends_with("AnalysisSubmittedEvent")
            || event_type.ends_with("PocBadgeIssuedEvent")
            || event_type.ends_with("PoCBadgeIssuedEvent") // Handle both casings
            || event_type.ends_with("RevenueRedirectionActivatedEvent")
            || event_type.ends_with("PocDisputeSubmittedEvent")
            || event_type.ends_with("PoCDisputeSubmittedEvent") // Handle both casings
            || event_type.ends_with("DisputeVoteCastEvent")
            || event_type.ends_with("PocDisputeResolvedEvent")
            || event_type.ends_with("PoCDisputeResolvedEvent") // Handle both casings
            || event_type.ends_with("VotingRewardClaimedEvent")
            || event_type.ends_with("PocConfigUpdatedEvent")
            || event_type.ends_with("PoCConfigUpdatedEvent") // Handle both casings
            || event_type.ends_with("TokenPoolSyncNeededEvent")
    }

    async fn handle_token_pool_sync_needed(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<TokenPoolSyncNeededEvent>(&event.data)?;
        info!(
            "TokenPoolSyncNeededEvent received for post_id={} at epoch={}",
            parsed.post_id, parsed.timestamp
        );
        Ok(())
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting PoC event handler");

        while let Some(event) = self.rx.recv().await {
            if !Self::is_poc_event(&event.event_type) {
                continue;
            }

            info!("Processing PoC event: {}", event.event_type);

            let result = if event.event_type.ends_with("AnalysisSubmittedEvent") {
                self.handle_analysis_submitted(&event).await
            } else if event.event_type.ends_with("PocBadgeIssuedEvent") 
                || event.event_type.ends_with("PoCBadgeIssuedEvent") {
                self.handle_badge_issued(&event).await
            } else if event.event_type.ends_with("RevenueRedirectionActivatedEvent") {
                self.handle_revenue_redirection(&event).await
            } else if event.event_type.ends_with("PocDisputeSubmittedEvent")
                || event.event_type.ends_with("PoCDisputeSubmittedEvent") {
                self.handle_dispute_submitted(&event).await
            } else if event.event_type.ends_with("DisputeVoteCastEvent") {
                self.handle_vote_cast(&event).await
            } else if event.event_type.ends_with("PocDisputeResolvedEvent")
                || event.event_type.ends_with("PoCDisputeResolvedEvent") {
                self.handle_dispute_resolved(&event).await
            } else if event.event_type.ends_with("VotingRewardClaimedEvent") {
                self.handle_reward_claimed(&event).await
            } else if event.event_type.ends_with("PocConfigUpdatedEvent")
                || event.event_type.ends_with("PoCConfigUpdatedEvent") {
                self.handle_config_updated(&event).await
            } else if event.event_type.ends_with("TokenPoolSyncNeededEvent") {
                self.handle_token_pool_sync_needed(&event).await
            } else {
                // Unhandled PoC event
                warn!(
                    "Received unhandled PoC event: {} (event_id: {})",
                    event.event_type, event.event_id
                );
                Ok(())
            };

            if let Err(err) = result {
                match err.downcast_ref::<PocEventError>() {
                    Some(validation_err) => {
                        warn!(
                            "Validation failed for PoC event {}: {}",
                            event.event_type, validation_err
                        );
                    }
                    None => {
                        error!("Failed to process PoC event {}: {}", event.event_type, err);
                    }
                }
            } else if let Err(progress_err) = self.update_progress().await {
                warn!("Failed to update PoC handler progress: {}", progress_err);
            }
        }

        warn!("PoC event handler channel closed");
        Ok(())
    }
}
