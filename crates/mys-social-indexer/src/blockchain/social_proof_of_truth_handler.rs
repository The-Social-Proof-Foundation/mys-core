// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Nullable, SmallInt, Text};
use diesel_async::RunQueryDsl;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::blockchain::listener::BlockchainEvent;
use crate::db::{Database, DbConnection};
use crate::events::event_utils::extract_event_fields;
use crate::events::social_proof_of_truth_events::{
    SpotBetPlacedEvent, SpotDaoRequiredEvent, SpotPayoutEvent, SpotRefundEvent, SpotResolvedEvent,
};
use crate::models::social_proof_of_truth::{NewSocialProofOfTruthEvent, NewSpotEventLog};
use crate::schema;

/// Handler for Social Proof of Truth (SPoT) blockchain events.
pub struct SocialProofOfTruthEventHandler {
    db: Arc<Database>,
    rx: mpsc::Receiver<BlockchainEvent>,
    worker_name: String,
}

impl SocialProofOfTruthEventHandler {
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

    fn timestamp_epoch(event: &BlockchainEvent) -> i64 {
        (event.timestamp_ms as i64).saturating_div(1000)
    }

    fn event_time(event: &BlockchainEvent) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp((event.timestamp_ms / 1000) as i64, 0)
            .unwrap_or_else(|| Utc::now())
    }

    fn parse_event<T: serde::de::DeserializeOwned>(value: &Value) -> Result<T> {
        let fields = extract_event_fields(value)?;
        serde_json::from_value::<T>(fields)
            .map_err(|e| anyhow!("Failed to deserialize SPoT event payload: {}", e))
    }

    async fn ensure_spot_record_exists(
        conn: &mut DbConnection,
        post_id: &str,
        created_epoch: i64,
        transaction_id: &str,
    ) -> Result<()> {
        // Attempt to insert a placeholder record if it does not exist.
        let insert_sql = "INSERT INTO spot_records (post_id, status, outcome, amm_split_bps_used, total_yes_escrow, total_no_escrow, created_epoch, last_resolution_epoch, version, created_at, updated_at, transaction_id) \
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW(), $10) \
            ON CONFLICT (post_id) DO NOTHING";

        diesel::sql_query(insert_sql)
            .bind::<Text, _>(post_id)
            .bind::<SmallInt, _>(1) // STATUS_OPEN
            .bind::<Nullable<SmallInt>, _>(None::<i16>)
            .bind::<Integer, _>(3000)
            .bind::<BigInt, _>(0)
            .bind::<BigInt, _>(0)
            .bind::<BigInt, _>(created_epoch)
            .bind::<Nullable<BigInt>, _>(None::<i64>)
            .bind::<BigInt, _>(1)
            .bind::<Text, _>(transaction_id)
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn log_spot_event(
        conn: &mut DbConnection,
        event_type: &str,
        post_id: &str,
        event_payload: &impl serde::Serialize,
        event_id: Option<String>,
    ) -> Result<()> {
        if let Some(event_id) = event_id {
            let json = serde_json::to_value(event_payload)?;
            diesel::insert_into(schema::spot_events::table)
                .values(&NewSpotEventLog {
                    event_type: event_type.to_string(),
                    post_id: post_id.to_string(),
                    event_data: json,
                    event_id,
                    created_at: Utc::now(),
                })
                .execute(conn)
                .await?;
        }

        Ok(())
    }

    async fn write_unified_row(
        conn: &mut DbConnection,
        event: &BlockchainEvent,
        event_type: &str,
        payload: NewSocialProofOfTruthEvent,
    ) -> Result<()> {
        let mut row = payload;
        row.event_type = event_type.to_string();
        row.timestamp_epoch = Self::timestamp_epoch(event);
        row.time = Utc::now();
        row.event_id = Some(event.event_id.clone());
        row.transaction_id = Some(event.tx_digest.clone());
        diesel::insert_into(schema::social_proof_of_truth::table)
            .values(&row)
            .execute(conn)
            .await?;
        Ok(())
    }

    async fn handle_spot_bet_placed(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotBetPlacedEvent>(&event.data)?;
        let tx = event.tx_digest.clone();
        let timestamp_epoch = Self::timestamp_epoch(event);
        let time = Self::event_time(event);

        let mut bet = parsed
            .into_bet_model(timestamp_epoch as u64, tx.clone())
            .map_err(|e| anyhow!("Failed to convert SpotBetPlacedEvent: {}", e))?;
        bet.time = time;

        let mut conn = self.get_connection().await?;
        Self::ensure_spot_record_exists(&mut conn, &bet.post_id, timestamp_epoch, &tx).await?;

        diesel::insert_into(schema::spot_bets::table)
            .values(&bet)
            .execute(&mut conn)
            .await?;

        // Update aggregated escrow amounts.
        if parsed.amount > 0 {
            let sql = if parsed.is_yes {
                "UPDATE spot_records SET total_yes_escrow = total_yes_escrow + $1, updated_at = NOW() WHERE post_id = $2"
            } else {
                "UPDATE spot_records SET total_no_escrow = total_no_escrow + $1, updated_at = NOW() WHERE post_id = $2"
            };

            diesel::sql_query(sql)
                .bind::<BigInt, _>(parsed.amount as i64)
                .bind::<Text, _>(&parsed.post_id)
                .execute(&mut conn)
                .await?;
        }

        Self::log_spot_event(
            &mut conn,
            "SpotBetPlacedEvent",
            &parsed.post_id,
            &parsed,
            Some(event.event_id.clone()),
        )
        .await?;

        let unified = NewSocialProofOfTruthEvent {
            event_type: "SpotBetPlacedEvent".to_string(),
            post_id: parsed.post_id.clone(),
            user_address: Some(parsed.user.clone()),
            is_yes: Some(parsed.is_yes),
            escrow_amount: Some(parsed.amount as i64), // amount goes to escrow
            amm_amount: Some(0), // No AMM in current contract
            amount: Some(parsed.amount as i64),
            outcome: None,
            total_escrow: None,
            fee_taken: None,
            confidence_bps: None,
            timestamp_epoch,
            time: Utc::now(),
            event_id: None,
            transaction_id: None,
            raw_event: Some(serde_json::to_value(&parsed)?),
        };

        self.write_unified(&mut conn, event, "SpotBetPlacedEvent", unified)
            .await
    }

    async fn handle_spot_resolved(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotResolvedEvent>(&event.data)?;
        let tx = event.tx_digest.clone();
        let timestamp_epoch = Self::timestamp_epoch(event);
        let time = Self::event_time(event);

        let mut resolution = parsed
            .into_resolution_model(timestamp_epoch as u64, tx.clone())
            .map_err(|e| anyhow!("Failed to convert SpotResolvedEvent: {}", e))?;
        resolution.time = time;

        let mut conn = self.get_connection().await?;

        diesel::sql_query(
            "UPDATE spot_records SET status = $1, outcome = $2, last_resolution_epoch = $3, updated_at = NOW() WHERE post_id = $4",
        )
        .bind::<SmallInt, _>(3) // STATUS_RESOLVED
        .bind::<Nullable<SmallInt>, _>(Some(resolution.outcome))
        .bind::<BigInt, _>(resolution.resolved_epoch)
        .bind::<Text, _>(&parsed.post_id)
        .execute(&mut conn)
        .await?;

        diesel::insert_into(schema::spot_resolutions::table)
            .values(&resolution)
            .execute(&mut conn)
            .await?;

        Self::log_spot_event(
            &mut conn,
            "SpotResolvedEvent",
            &parsed.post_id,
            &parsed,
            Some(event.event_id.clone()),
        )
        .await?;

        let unified = NewSocialProofOfTruthEvent {
            event_type: "SpotResolvedEvent".to_string(),
            post_id: parsed.post_id.clone(),
            user_address: None,
            is_yes: None,
            escrow_amount: None,
            amm_amount: None,
            amount: None,
            outcome: Some(parsed.outcome as i16),
            total_escrow: Some(parsed.total_escrow as i64),
            fee_taken: Some(parsed.fee_taken as i64),
            confidence_bps: None,
            timestamp_epoch,
            time: Utc::now(),
            event_id: None,
            transaction_id: None,
            raw_event: Some(serde_json::to_value(&parsed)?),
        };

        self.write_unified(&mut conn, event, "SpotResolvedEvent", unified)
            .await
    }

    async fn handle_spot_dao_required(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotDaoRequiredEvent>(&event.data)?;
        let mut conn = self.get_connection().await?;

        diesel::sql_query(
            "UPDATE spot_records SET status = 2, updated_at = NOW() WHERE post_id = $1",
        )
        .bind::<Text, _>(&parsed.post_id)
        .execute(&mut conn)
        .await?;

        Self::log_spot_event(
            &mut conn,
            "SpotDaoRequiredEvent",
            &parsed.post_id,
            &parsed,
            Some(event.event_id.clone()),
        )
        .await?;

        let unified = NewSocialProofOfTruthEvent {
            event_type: "SpotDaoRequiredEvent".to_string(),
            post_id: parsed.post_id.clone(),
            user_address: None,
            is_yes: None,
            escrow_amount: None,
            amm_amount: None,
            amount: None,
            outcome: None,
            total_escrow: None,
            fee_taken: None,
            confidence_bps: Some(parsed.confidence_bps as i64),
            timestamp_epoch: Self::timestamp_epoch(event),
            time: Utc::now(),
            event_id: None,
            transaction_id: None,
            raw_event: Some(serde_json::to_value(&parsed)?),
        };

        self.write_unified(&mut conn, event, "SpotDaoRequiredEvent", unified)
            .await
    }

    async fn handle_spot_payout(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotPayoutEvent>(&event.data)?;
        let tx = event.tx_digest.clone();
        let timestamp_epoch = Self::timestamp_epoch(event);
        let time = Self::event_time(event);

        let mut payout = parsed
            .into_model(timestamp_epoch as u64, tx.clone())
            .map_err(|e| anyhow!("Failed to convert SpotPayoutEvent: {}", e))?;
        payout.time = time;

        let mut conn = self.get_connection().await?;

        diesel::insert_into(schema::spot_payouts::table)
            .values(&payout)
            .execute(&mut conn)
            .await?;

        let unified = NewSocialProofOfTruthEvent {
            event_type: "SpotPayoutEvent".to_string(),
            post_id: parsed.post_id.clone(),
            user_address: Some(parsed.user.clone()),
            is_yes: None,
            escrow_amount: None,
            amm_amount: None,
            amount: Some(parsed.amount as i64),
            outcome: None,
            total_escrow: None,
            fee_taken: None,
            confidence_bps: None,
            timestamp_epoch,
            time: Utc::now(),
            event_id: None,
            transaction_id: None,
            raw_event: Some(serde_json::to_value(&parsed)?),
        };

        self.write_unified(&mut conn, event, "SpotPayoutEvent", unified)
            .await
    }

    async fn handle_spot_refund(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotRefundEvent>(&event.data)?;
        let tx = event.tx_digest.clone();
        let timestamp_epoch = Self::timestamp_epoch(event);
        let time = Self::event_time(event);

        let mut refund = parsed
            .into_model(timestamp_epoch as u64, tx.clone())
            .map_err(|e| anyhow!("Failed to convert SpotRefundEvent: {}", e))?;
        refund.time = time;

        let mut conn = self.get_connection().await?;

        diesel::insert_into(schema::spot_refunds::table)
            .values(&refund)
            .execute(&mut conn)
            .await?;

        let unified = NewSocialProofOfTruthEvent {
            event_type: "SpotRefundEvent".to_string(),
            post_id: parsed.post_id.clone(),
            user_address: Some(parsed.user.clone()),
            is_yes: None,
            escrow_amount: None,
            amm_amount: None,
            amount: Some(parsed.amount as i64),
            outcome: None,
            total_escrow: None,
            fee_taken: None,
            confidence_bps: None,
            timestamp_epoch,
            time: Utc::now(),
            event_id: None,
            transaction_id: None,
            raw_event: Some(serde_json::to_value(&parsed)?),
        };

        self.write_unified(&mut conn, event, "SpotRefundEvent", unified)
            .await
    }

    async fn write_unified(
        &self,
        conn: &mut DbConnection,
        event: &BlockchainEvent,
        event_type: &str,
        payload: NewSocialProofOfTruthEvent,
    ) -> Result<()> {
        SocialProofOfTruthEventHandler::write_unified_row(conn, event, event_type, payload).await
    }

    fn is_spot_event(event_type: &str) -> bool {
        event_type.contains("::social_proof_of_truth::")
            || event_type.ends_with("SpotBetPlacedEvent")
            || event_type.ends_with("SpotResolvedEvent")
            || event_type.ends_with("SpotDaoRequiredEvent")
            || event_type.ends_with("SpotPayoutEvent")
            || event_type.ends_with("SpotRefundEvent")
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

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting Social Proof of Truth event handler");

        while let Some(event) = self.rx.recv().await {
            debug!("Processing SPoT event: {}", event.event_type);

            if !Self::is_spot_event(&event.event_type) {
                continue;
            }

            let result = if event.event_type.ends_with("SpotBetPlacedEvent") {
                self.handle_spot_bet_placed(&event).await
            } else if event.event_type.ends_with("SpotResolvedEvent") {
                self.handle_spot_resolved(&event).await
            } else if event.event_type.ends_with("SpotDaoRequiredEvent") {
                self.handle_spot_dao_required(&event).await
            } else if event.event_type.ends_with("SpotPayoutEvent") {
                self.handle_spot_payout(&event).await
            } else if event.event_type.ends_with("SpotRefundEvent") {
                self.handle_spot_refund(&event).await
            } else {
                warn!("Received unhandled SPoT event: {} (event_id: {})", event.event_type, event.event_id);
                Ok(())
            };

            if let Err(err) = result {
                error!("Failed to process SPoT event {}: {}", event.event_type, err);
            } else if let Err(e) = self.update_progress().await {
                warn!("Failed to update SPoT handler progress: {}", e);
            }
        }

        warn!("Social Proof of Truth event handler channel closed");
        Ok(())
    }
}
