// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Context, Result};
use diesel::result::Error as DieselError;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Nullable, SmallInt, Text};
use diesel_async::RunQueryDsl;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::social::blockchain::listener::BlockchainEvent;
use crate::social::db::{Database, DbConnection};
use crate::social::events::event_utils::extract_event_fields;
use diesel_async::AsyncPgConnection;
use crate::social::events::social_proof_of_truth_events::{
    SpotBetPlacedEvent, SpotBetWithdrawnEvent, SpotConfigUpdatedEvent, SpotDaoRequiredEvent,
    SpotPayoutEvent, SpotRecordCreatedEvent, SpotRefundEvent, SpotResolvedEvent,
};
use crate::social::models::social_proof_of_truth::{
    NewSocialProofOfTruthEvent, NewSpotBetWithdrawal, NewSpotEventLog,
    SpotConfig,
};
use crate::social::schema;

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

    /// Validate outcome value is within expected range
    /// Returns: (is_valid, is_draw, is_unapplicable)
    fn validate_outcome(outcome: u8) -> (bool, bool, bool) {
        match outcome {
            254 => (true, false, true),  // UNAPPLICABLE
            255 => (true, true, false),   // DRAW
            0..=9 => (true, false, false), // Valid option_id (0-9)
            _ => (false, false, false),   // Invalid
        }
    }


    /// Validate option_id exists in betting_options array (works with transaction connections)
    async fn validate_option_id_in_transaction(
        conn: &mut AsyncPgConnection,
        post_id: &str,
        option_id: u8,
    ) -> Result<(), DieselError> {
        use diesel::sql_types::Jsonb;
        #[derive(QueryableByName)]
        struct BettingOptionsRow {
            #[diesel(sql_type = Nullable<Jsonb>)]
            betting_options: Option<Value>,
        }

        let result: Result<BettingOptionsRow, _> = diesel::sql_query(
            "SELECT betting_options FROM spot_records WHERE post_id = $1"
        )
        .bind::<Text, _>(post_id)
        .get_result(conn)
        .await;

        match result {
            Ok(row) => {
                if let Some(betting_options) = row.betting_options {
                    if let Some(options_array) = betting_options.as_array() {
                        let option_id_u64 = option_id as u64;
                        if option_id_u64 >= options_array.len() as u64 {
                            return Err(DieselError::QueryBuilderError(format!(
                                "Option ID {} is out of range for betting_options array of length {}",
                                option_id,
                                options_array.len()
                            ).into()));
                        }
                    }
                } else {
                    warn!("betting_options is NULL for post_id: {}", post_id);
                }
            }
            Err(e) => {
                // Record doesn't exist - validation will be handled by spot_record_exists
                return Err(e);
            }
        }

        Ok(())
    }

    /// Check if spot_record exists (works with transaction connections)
    async fn spot_record_exists_in_transaction(
        conn: &mut AsyncPgConnection,
        post_id: &str,
    ) -> Result<bool, DieselError> {
        #[derive(QueryableByName)]
        struct ExistsRow {
            #[diesel(sql_type = BigInt)]
            #[allow(dead_code)]
            exists: i64,
        }

        let result: Result<ExistsRow, _> = diesel::sql_query(
            "SELECT 1 as exists FROM spot_records WHERE post_id = $1 LIMIT 1"
        )
        .bind::<Text, _>(post_id)
        .get_result(conn)
        .await;

        Ok(result.is_ok())
    }

    async fn ensure_spot_record_exists(
        conn: &mut DbConnection,
        post_id: &str,
        created_epoch: i64,
        transaction_id: &str,
    ) -> Result<()> {
        // Attempt to insert a placeholder record if it does not exist.
        let insert_sql = "INSERT INTO spot_records (post_id, status, outcome, amm_split_bps_used, betting_options, option_escrow, resolution_window_epochs, max_resolution_window_epochs, created_epoch, last_resolution_epoch, version, created_at, updated_at, transaction_id) \
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW(), $12) \
            ON CONFLICT (post_id) DO NOTHING";

        diesel::sql_query(insert_sql)
            .bind::<Text, _>(post_id)
            .bind::<SmallInt, _>(1) // STATUS_OPEN
            .bind::<Nullable<SmallInt>, _>(None::<i16>)
            .bind::<Integer, _>(3000)
            .bind::<Nullable<diesel::sql_types::Jsonb>, _>(Some(serde_json::json!([])))
            .bind::<Nullable<diesel::sql_types::Jsonb>, _>(Some(serde_json::json!({})))
            .bind::<Nullable<BigInt>, _>(None::<i64>)
            .bind::<Nullable<BigInt>, _>(None::<i64>)
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

    async fn log_spot_event_in_transaction(
        conn: &mut AsyncPgConnection,
        event_type: &str,
        post_id: &str,
        event_payload: &impl serde::Serialize,
        event_id: Option<String>,
    ) -> Result<(), DieselError> {
        if let Some(event_id) = event_id {
            let json = serde_json::to_value(event_payload)
                .map_err(|e| DieselError::QueryBuilderError(format!("JSON serialization error: {}", e).into()))?;
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

    async fn write_unified_row_in_transaction(
        conn: &mut AsyncPgConnection,
        event: &BlockchainEvent,
        event_type: &str,
        payload: NewSocialProofOfTruthEvent,
    ) -> Result<(), DieselError> {
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
        let parsed = Self::parse_event::<SpotBetPlacedEvent>(&event.data)
            .context("Failed to parse SpotBetPlacedEvent")?;
        let tx = event.tx_digest.clone();
        let timestamp_epoch = Self::timestamp_epoch(event);
        let time = Self::event_time(event);

        // Validate amount
        if parsed.amount == 0 {
            return Err(anyhow!("Bet amount must be greater than zero"));
        }

        let mut bet = parsed
            .into_bet_model(timestamp_epoch as u64, tx.clone())
            .map_err(|e| anyhow!("Failed to convert SpotBetPlacedEvent: {}", e))?;
        bet.time = time;

        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let post_id = parsed.post_id.clone();
        let user = parsed.user.clone();
        let option_id = parsed.option_id;
        let amount = parsed.amount;
        let parsed_clone = parsed.clone();
        let parsed_json = serde_json::to_value(&parsed_clone)?;

        // Wrap in transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Ensure spot record exists
                    let insert_sql = "INSERT INTO spot_records (post_id, status, outcome, amm_split_bps_used, betting_options, option_escrow, resolution_window_epochs, max_resolution_window_epochs, created_epoch, last_resolution_epoch, version, created_at, updated_at, transaction_id) \
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW(), $12) \
                        ON CONFLICT (post_id) DO NOTHING";

                    diesel::sql_query(insert_sql)
                        .bind::<Text, _>(&bet.post_id)
                        .bind::<SmallInt, _>(1) // STATUS_OPEN
                        .bind::<Nullable<SmallInt>, _>(None::<i16>)
                        .bind::<Integer, _>(3000)
                        .bind::<Nullable<diesel::sql_types::Jsonb>, _>(Some(serde_json::json!([])))
                        .bind::<Nullable<diesel::sql_types::Jsonb>, _>(Some(serde_json::json!({})))
                        .bind::<Nullable<BigInt>, _>(None::<i64>)
                        .bind::<Nullable<BigInt>, _>(None::<i64>)
                        .bind::<BigInt, _>(timestamp_epoch)
                        .bind::<Nullable<BigInt>, _>(None::<i64>)
                        .bind::<BigInt, _>(1)
                        .bind::<Text, _>(&tx)
                        .execute(&mut conn)
                        .await?;

                    // Validate option_id exists in betting_options
                    Self::validate_option_id_in_transaction(&mut conn, &post_id, option_id)
                        .await?;

                    // Insert bet
                    diesel::insert_into(schema::spot_bets::table)
                        .values(&bet)
                        .execute(&mut conn)
                        .await?;

                    // Update aggregated escrow amounts using JSONB option_escrow
                    let option_id_str = option_id.to_string();
                    let sql = "UPDATE spot_records SET option_escrow = jsonb_set(
                        COALESCE(option_escrow, '{}'::jsonb),
                        ARRAY[$1],
                        ((COALESCE((option_escrow->>$1)::bigint, 0) + $2)::text)::jsonb
                    ), updated_at = NOW() WHERE post_id = $3";

                    let rows_updated = diesel::sql_query(sql)
                        .bind::<Text, _>(&option_id_str)
                        .bind::<BigInt, _>(amount as i64)
                        .bind::<Text, _>(&post_id)
                        .execute(&mut conn)
                        .await?;

                    if rows_updated == 0 {
                        warn!(
                            "No rows updated for option_escrow on post_id: {}",
                            post_id
                        );
                    }

                    // Log the event
                    Self::log_spot_event_in_transaction(
                        &mut conn,
                        "SpotBetPlacedEvent",
                        &post_id,
                        &parsed_clone,
                        Some(event_id.clone()),
                    )
                    .await?;

                    // Write unified event
                    let unified = NewSocialProofOfTruthEvent {
                        event_type: "SpotBetPlacedEvent".to_string(),
                        post_id: post_id.clone(),
                        user_address: Some(user),
                        option_id: Some(option_id as i16),
                        escrow_amount: Some(amount as i64), // amount goes to escrow
                        amm_amount: Some(0), // No AMM in current contract
                        amount: Some(amount as i64),
                        outcome: None,
                        total_escrow: None,
                        fee_taken: None,
                        confidence_bps: None,
                        timestamp_epoch,
                        time: Utc::now(),
                        event_id: Some(event_id.clone()),
                        transaction_id: Some(tx_digest.clone()),
                        raw_event: Some(parsed_json),
                    };

                    let event_for_unified = BlockchainEvent {
                        event_type: "SpotBetPlacedEvent".to_string(),
                        event_id: event_id.clone(),
                        tx_digest: tx_digest.clone(),
                        timestamp_ms: event_timestamp_ms,
                        data: serde_json::Value::Null,
                    };

                    Self::write_unified_row_in_transaction(&mut conn, &event_for_unified, "SpotBetPlacedEvent", unified)
                        .await?;

                    Ok::<(), DieselError>(())
                })
            })
            .await
            .map_err(|e| anyhow!("Transaction failed for SpotBetPlacedEvent: {}", e))?;

        Ok(())
    }

    async fn handle_spot_resolved(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotResolvedEvent>(&event.data)
            .context("Failed to parse SpotResolvedEvent")?;
        let tx = event.tx_digest.clone();
        let timestamp_epoch = Self::timestamp_epoch(event);
        let time = Self::event_time(event);

        // Validate outcome
        let (is_valid, is_draw, is_unapplicable) = Self::validate_outcome(parsed.outcome);
        if !is_valid {
            return Err(anyhow!(
                "Invalid outcome value: {} (expected 0-9 for option_id, 254 for UNAPPLICABLE, or 255 for DRAW)",
                parsed.outcome
            ));
        }

        // Log outcome type for monitoring
        if is_draw {
            debug!("Processing DRAW resolution for post_id: {}", parsed.post_id);
        } else if is_unapplicable {
            debug!("Processing UNAPPLICABLE resolution for post_id: {}", parsed.post_id);
        } else {
            debug!("Processing resolution with winning option_id: {} for post_id: {}", parsed.outcome, parsed.post_id);
        }

        let mut resolution = parsed
            .into_resolution_model(timestamp_epoch as u64, tx.clone())
            .map_err(|e| anyhow!("Failed to convert SpotResolvedEvent: {}", e))?;
        resolution.time = time;

        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let post_id = parsed.post_id.clone();
        let outcome = parsed.outcome;
        let total_escrow = parsed.total_escrow;
        let fee_taken = parsed.fee_taken;
        let parsed_clone = parsed.clone();
        let parsed_json = serde_json::to_value(&parsed_clone)?;

        // Wrap in transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Verify spot record exists
                    if !Self::spot_record_exists_in_transaction(&mut conn, &post_id).await? {
                        return Err(DieselError::QueryBuilderError(format!("Spot record does not exist for post_id: {}", post_id).into()));
                    }

                    // Update spot record status and outcome
                    let rows_updated = diesel::sql_query(
                        "UPDATE spot_records SET status = $1, outcome = $2, last_resolution_epoch = $3, updated_at = NOW() WHERE post_id = $4"
                    )
                    .bind::<SmallInt, _>(3) // STATUS_RESOLVED
                    .bind::<Nullable<SmallInt>, _>(Some(resolution.outcome))
                    .bind::<BigInt, _>(resolution.resolved_epoch)
                    .bind::<Text, _>(&post_id)
                    .execute(&mut conn)
                    .await?;

                    if rows_updated == 0 {
                        return Err(DieselError::QueryBuilderError(format!("No spot record found to update for post_id: {}", post_id).into()));
                    }

                    // Insert resolution record
                    diesel::insert_into(schema::spot_resolutions::table)
                        .values(&resolution)
                        .execute(&mut conn)
                        .await?;

                    // Log the event
                    Self::log_spot_event_in_transaction(
                        &mut conn,
                        "SpotResolvedEvent",
                        &post_id,
                        &parsed_clone,
                        Some(event_id.clone()),
                    )
                    .await?;

                    // Write unified event
                    // Outcome interpretation: 0-9 = winning option_id, 255 = DRAW, 254 = UNAPPLICABLE
                    let unified = NewSocialProofOfTruthEvent {
                        event_type: "SpotResolvedEvent".to_string(),
                        post_id: post_id.clone(),
                        user_address: None,
                        option_id: None,
                        escrow_amount: None,
                        amm_amount: None,
                        amount: None,
                        outcome: Some(outcome as i16),
                        total_escrow: Some(total_escrow as i64),
                        fee_taken: Some(fee_taken as i64),
                        confidence_bps: None,
                        timestamp_epoch,
                        time: Utc::now(),
                        event_id: Some(event_id.clone()),
                        transaction_id: Some(tx_digest.clone()),
                        raw_event: Some(parsed_json),
                    };

                    let event_for_unified = BlockchainEvent {
                        event_type: "SpotResolvedEvent".to_string(),
                        event_id: event_id.clone(),
                        tx_digest: tx_digest.clone(),
                        timestamp_ms: event_timestamp_ms,
                        data: serde_json::Value::Null,
                    };

                    Self::write_unified_row_in_transaction(&mut conn, &event_for_unified, "SpotResolvedEvent", unified)
                        .await?;

                    Ok::<(), DieselError>(())
                })
            })
            .await
            .context("Transaction failed for SpotResolvedEvent")?;

        Ok(())
    }

    async fn handle_spot_bet_withdrawn(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotBetWithdrawnEvent>(&event.data)
            .context("Failed to parse SpotBetWithdrawnEvent")?;
        let tx = event.tx_digest.clone();
        let timestamp_epoch = Self::timestamp_epoch(event);
        let time = Self::event_time(event);

        // Validate amount
        if parsed.amount == 0 {
            return Err(anyhow!("Withdrawal amount must be greater than zero"));
        }

        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let post_id = parsed.post_id.clone();
        let user = parsed.user.clone();
        let option_id = parsed.option_id;
        let amount = parsed.amount;
        let fee_taken = parsed.fee_taken;
        let parsed_clone = parsed.clone();
        let parsed_json = serde_json::to_value(&parsed_clone)?;

        // Wrap in transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Verify spot record exists
                    if !Self::spot_record_exists_in_transaction(&mut conn, &post_id).await? {
                        return Err(DieselError::QueryBuilderError(format!("Spot record does not exist for post_id: {}", post_id).into()));
                    }

                    // Validate option_id exists in betting_options
                    Self::validate_option_id_in_transaction(&mut conn, &post_id, option_id)
                        .await?;

                    // Insert withdrawal record
                    let withdrawal = NewSpotBetWithdrawal {
                        post_id: post_id.clone(),
                        user_address: user.clone(),
                        option_id: option_id as i16,
                        amount: amount as i64,
                        fee_taken: fee_taken as i64,
                        timestamp_epoch,
                        time,
                        transaction_id: tx.clone(),
                    };

                    diesel::insert_into(schema::spot_bet_withdrawals::table)
                        .values(&withdrawal)
                        .execute(&mut conn)
                        .await?;

                    // Update option_escrow JSONB to subtract withdrawn amount from specific option
                    let option_id_str = option_id.to_string();
                    let sql = "UPDATE spot_records SET option_escrow = jsonb_set(
                        COALESCE(option_escrow, '{}'::jsonb),
                        ARRAY[$1],
                        GREATEST((COALESCE((option_escrow->>$1)::bigint, 0) - $2), 0)::text::jsonb
                    ), updated_at = NOW() WHERE post_id = $3";

                    let rows_updated = diesel::sql_query(sql)
                        .bind::<Text, _>(&option_id_str)
                        .bind::<BigInt, _>(amount as i64)
                        .bind::<Text, _>(&post_id)
                        .execute(&mut conn)
                        .await?;

                    if rows_updated == 0 {
                        warn!(
                            "No rows updated for option_escrow on post_id: {}",
                            post_id
                        );
                    }

                    // Delete the bet from spot_bets table - match by amount and timestamp to ensure we delete the correct bet
                    // This prevents deleting multiple bets if user has multiple bets on same option
                    let deleted_rows = diesel::sql_query(
                        "DELETE FROM spot_bets WHERE post_id = $1 AND user_address = $2 AND option_id = $3 AND escrow_amount = $4 AND timestamp_epoch <= $5 LIMIT 1"
                    )
                    .bind::<Text, _>(&post_id)
                    .bind::<Text, _>(&user)
                    .bind::<SmallInt, _>(option_id as i16)
                    .bind::<BigInt, _>(amount as i64)
                    .bind::<BigInt, _>(timestamp_epoch)
                    .execute(&mut conn)
                    .await?;

                    if deleted_rows == 0 {
                        warn!(
                            "No bet found to delete for post_id: {}, user: {}, option_id: {}, amount: {}",
                            post_id, user, option_id, amount
                        );
                    } else if deleted_rows > 1 {
                        warn!(
                            "Unexpected: deleted {} bets (expected 1) for post_id: {}, user: {}, option_id: {}",
                            deleted_rows, post_id, user, option_id
                        );
                    }

                    // Log the event
                    Self::log_spot_event_in_transaction(
                        &mut conn,
                        "SpotBetWithdrawnEvent",
                        &post_id,
                        &parsed_clone,
                        Some(event_id.clone()),
                    )
                    .await?;

                    // Write to unified table
                    let unified = NewSocialProofOfTruthEvent {
                        event_type: "SpotBetWithdrawnEvent".to_string(),
                        post_id: post_id.clone(),
                        user_address: Some(user),
                        option_id: Some(option_id as i16),
                        escrow_amount: None,
                        amm_amount: None,
                        amount: Some(amount as i64),
                        outcome: None,
                        total_escrow: None,
                        fee_taken: Some(fee_taken as i64),
                        confidence_bps: None,
                        timestamp_epoch,
                        time: Utc::now(),
                        event_id: Some(event_id.clone()),
                        transaction_id: Some(tx_digest.clone()),
                        raw_event: Some(parsed_json),
                    };

                    let event_for_unified = BlockchainEvent {
                        event_type: "SpotBetWithdrawnEvent".to_string(),
                        event_id: event_id.clone(),
                        tx_digest: tx_digest.clone(),
                        timestamp_ms: event_timestamp_ms,
                        data: serde_json::Value::Null,
                    };

                    Self::write_unified_row_in_transaction(&mut conn, &event_for_unified, "SpotBetWithdrawnEvent", unified)
                        .await?;

                    Ok::<(), DieselError>(())
                })
            })
            .await
            .context("Transaction failed for SpotBetWithdrawnEvent")?;

        Ok(())
    }

    async fn handle_spot_dao_required(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotDaoRequiredEvent>(&event.data)
            .context("Failed to parse SpotDaoRequiredEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let post_id = parsed.post_id.clone();
        let confidence_bps = parsed.confidence_bps;
        let parsed_clone = parsed.clone();
        let parsed_json = serde_json::to_value(&parsed_clone)?;

        // Verify spot record exists
        #[derive(QueryableByName)]
        struct CountRow {
            #[diesel(sql_type = BigInt)]
            count: i64,
        }

        let record_exists_result: Result<CountRow, _> = diesel::sql_query(
            "SELECT COUNT(*) as count FROM spot_records WHERE post_id = $1"
        )
        .bind::<Text, _>(&post_id)
        .get_result(&mut conn)
        .await;

        let record_exists = record_exists_result
            .map(|row| row.count > 0)
            .unwrap_or(false);

        if !record_exists {
            warn!("Spot record does not exist for post_id: {}, creating placeholder", post_id);
            // Create placeholder record if it doesn't exist
            Self::ensure_spot_record_exists(&mut conn, &post_id, Self::timestamp_epoch(event), &tx_digest).await?;
        }

        diesel::sql_query(
            "UPDATE spot_records SET status = 2, updated_at = NOW() WHERE post_id = $1",
        )
        .bind::<Text, _>(&post_id)
        .execute(&mut conn)
        .await
        .context("Failed to update spot record status to DAO_REQUIRED")?;

        Self::log_spot_event(
            &mut conn,
            "SpotDaoRequiredEvent",
            &post_id,
            &parsed_clone,
            Some(event_id.clone()),
        )
        .await
        .context("Failed to log spot event")?;

        let unified = NewSocialProofOfTruthEvent {
            event_type: "SpotDaoRequiredEvent".to_string(),
            post_id: post_id.clone(),
            user_address: None,
            option_id: None,
            escrow_amount: None,
            amm_amount: None,
            amount: None,
            outcome: None,
            total_escrow: None,
            fee_taken: None,
            confidence_bps: Some(confidence_bps as i64),
            timestamp_epoch: Self::timestamp_epoch(event),
            time: Utc::now(),
            event_id: Some(event_id.clone()),
            transaction_id: Some(tx_digest.clone()),
            raw_event: Some(parsed_json),
        };

        let event_for_unified = BlockchainEvent {
            event_type: "SpotDaoRequiredEvent".to_string(),
            event_id: event_id.clone(),
            tx_digest: tx_digest.clone(),
            timestamp_ms: event_timestamp_ms,
            data: serde_json::Value::Null,
        };

        Self::write_unified_row(&mut conn, &event_for_unified, "SpotDaoRequiredEvent", unified)
            .await
            .context("Failed to write unified event")?;

        Ok(())
    }

    async fn handle_spot_payout(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotPayoutEvent>(&event.data)
            .context("Failed to parse SpotPayoutEvent")?;
        let tx = event.tx_digest.clone();
        let timestamp_epoch = Self::timestamp_epoch(event);
        let time = Self::event_time(event);
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let parsed_clone = parsed.clone();
        let parsed_json = serde_json::to_value(&parsed_clone)?;

        let mut payout = parsed
            .into_model(timestamp_epoch as u64, tx.clone())
            .map_err(|e| anyhow!("Failed to convert SpotPayoutEvent: {}", e))?;
        payout.time = time;

        let mut conn = self.get_connection().await?;

        diesel::insert_into(schema::spot_payouts::table)
            .values(&payout)
            .execute(&mut conn)
            .await
            .context("Failed to insert payout record")?;

        let unified = NewSocialProofOfTruthEvent {
            event_type: "SpotPayoutEvent".to_string(),
            post_id: parsed.post_id.clone(),
            user_address: Some(parsed.user.clone()),
            option_id: None,
            escrow_amount: None,
            amm_amount: None,
            amount: Some(parsed.amount as i64),
            outcome: None,
            total_escrow: None,
            fee_taken: None,
            confidence_bps: None,
            timestamp_epoch,
            time: Utc::now(),
            event_id: Some(event_id.clone()),
            transaction_id: Some(tx_digest.clone()),
            raw_event: Some(parsed_json),
        };

        let event_for_unified = BlockchainEvent {
            event_type: "SpotPayoutEvent".to_string(),
            event_id: event_id.clone(),
            tx_digest: tx_digest.clone(),
            timestamp_ms: event_timestamp_ms,
            data: serde_json::Value::Null,
        };

        Self::write_unified_row(&mut conn, &event_for_unified, "SpotPayoutEvent", unified)
            .await
            .context("Failed to write unified event")?;

        Ok(())
    }

    async fn handle_spot_refund(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotRefundEvent>(&event.data)
            .context("Failed to parse SpotRefundEvent")?;
        let tx = event.tx_digest.clone();
        let timestamp_epoch = Self::timestamp_epoch(event);
        let time = Self::event_time(event);
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let parsed_clone = parsed.clone();
        let parsed_json = serde_json::to_value(&parsed_clone)?;

        let mut refund = parsed
            .into_model(timestamp_epoch as u64, tx.clone())
            .map_err(|e| anyhow!("Failed to convert SpotRefundEvent: {}", e))?;
        refund.time = time;

        let mut conn = self.get_connection().await?;

        diesel::insert_into(schema::spot_refunds::table)
            .values(&refund)
            .execute(&mut conn)
            .await
            .context("Failed to insert refund record")?;

        let unified = NewSocialProofOfTruthEvent {
            event_type: "SpotRefundEvent".to_string(),
            post_id: parsed.post_id.clone(),
            user_address: Some(parsed.user.clone()),
            option_id: None,
            escrow_amount: None,
            amm_amount: None,
            amount: Some(parsed.amount as i64),
            outcome: None,
            total_escrow: None,
            fee_taken: None,
            confidence_bps: None,
            timestamp_epoch,
            time: Utc::now(),
            event_id: Some(event_id.clone()),
            transaction_id: Some(tx_digest.clone()),
            raw_event: Some(parsed_json),
        };

        let event_for_unified = BlockchainEvent {
            event_type: "SpotRefundEvent".to_string(),
            event_id: event_id.clone(),
            tx_digest: tx_digest.clone(),
            timestamp_ms: event_timestamp_ms,
            data: serde_json::Value::Null,
        };

        Self::write_unified_row(&mut conn, &event_for_unified, "SpotRefundEvent", unified)
            .await
            .context("Failed to write unified event")?;

        Ok(())
    }

    async fn handle_spot_config_updated(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotConfigUpdatedEvent>(&event.data)
            .context("Failed to parse SpotConfigUpdatedEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let parsed_clone = parsed.clone();
        let parsed_json = serde_json::to_value(&parsed_clone)?;

        // Log the config update event
        // Note: Config updates don't have a post_id, so we use a special placeholder
        // since spot_events.post_id has a NOT NULL constraint
        const GLOBAL_CONFIG_POST_ID: &str = "GLOBAL_CONFIG";
        Self::log_spot_event(
            &mut conn,
            "SpotConfigUpdatedEvent",
            GLOBAL_CONFIG_POST_ID,
            &parsed_clone,
            Some(event_id.clone()),
        )
        .await
        .context("Failed to log spot event")?;

        // Fetch the latest config from database to use as fallback for missing values
        let latest_config = diesel::sql_query(
            "SELECT id, updated_by, enable_flag, confidence_threshold_bps, resolution_window_epochs, \
             max_resolution_window_epochs, payout_delay_ms, fee_bps, fee_split_bps_platform, \
             oracle_address, max_single_bet, version, timestamp_ms, \
             time, transaction_id \
             FROM spot_config ORDER BY time DESC LIMIT 1"
        )
        .get_result::<SpotConfig>(&mut conn)
        .await
        .ok(); // Use None if no previous config exists

        // Insert into spot_config table
        // Use values from event when present, fallback to latest DB config if missing
        let datetime = Self::event_time(event);
        let new_config = parsed.into_config_model(
            event_timestamp_ms,
            tx_digest.clone(),
            datetime,
            latest_config.as_ref(),
        );

        diesel::insert_into(schema::spot_config::table)
            .values(&new_config)
            .execute(&mut conn)
            .await
            .context("Failed to insert config record")?;

        // Create unified event entry
        // For config updates, we use a placeholder post_id since they're global
        let unified = NewSocialProofOfTruthEvent {
            event_type: "SpotConfigUpdatedEvent".to_string(),
            post_id: GLOBAL_CONFIG_POST_ID.to_string(),
            user_address: Some(parsed.updated_by.clone()),
            option_id: None,
            escrow_amount: None,
            amm_amount: None,
            amount: None,
            outcome: None,
            total_escrow: None,
            fee_taken: None,
            confidence_bps: parsed.confidence_threshold_bps.map(|v| v as i64).or_else(|| {
                latest_config.as_ref().map(|c| c.confidence_threshold_bps)
            }),
            timestamp_epoch: Self::timestamp_epoch(event),
            time: Utc::now(),
            event_id: Some(event_id.clone()),
            transaction_id: Some(tx_digest.clone()),
            raw_event: Some(parsed_json),
        };

        let event_for_unified = BlockchainEvent {
            event_type: "SpotConfigUpdatedEvent".to_string(),
            event_id: event_id.clone(),
            tx_digest: tx_digest.clone(),
            timestamp_ms: event_timestamp_ms,
            data: serde_json::Value::Null,
        };

        Self::write_unified_row(&mut conn, &event_for_unified, "SpotConfigUpdatedEvent", unified)
            .await
            .context("Failed to write unified event")?;

        Ok(())
    }

    async fn handle_spot_record_created(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<SpotRecordCreatedEvent>(&event.data)
            .context("Failed to parse SpotRecordCreatedEvent")?;
        let tx = event.tx_digest.clone();
        let timestamp_epoch = Self::timestamp_epoch(event);

        // Validate betting_options
        if parsed.betting_options.is_empty() {
            return Err(anyhow!("betting_options cannot be empty"));
        }
        if parsed.betting_options.len() > 10 {
            return Err(anyhow!("betting_options cannot exceed 10 options"));
        }
        if parsed.betting_options.len() < 2 {
            return Err(anyhow!("betting_options must have at least 2 options"));
        }

        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let post_id = parsed.post_id.clone();
        let created_epoch = parsed.created_epoch as i64;
        let betting_options = parsed.betting_options.clone();
        let parsed_clone = parsed.clone();
        let parsed_json = serde_json::to_value(&parsed_clone)?;

        // Wrap in transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Create or update the spot record
                    let betting_options_json = serde_json::to_value(&betting_options)
                        .map_err(|e| DieselError::QueryBuilderError(format!("JSON serialization error: {}", e).into()))?;
                    let option_escrow_json = serde_json::json!({});
                    let resolution_window = parsed_clone.resolution_window_epochs.map(|v| v as i64);
                    let max_resolution_window = parsed_clone.max_resolution_window_epochs.map(|v| v as i64);

                    let insert_sql = "INSERT INTO spot_records (post_id, status, outcome, amm_split_bps_used, betting_options, option_escrow, resolution_window_epochs, max_resolution_window_epochs, created_epoch, last_resolution_epoch, version, created_at, updated_at, transaction_id) \
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW(), $12) \
                        ON CONFLICT (post_id) DO UPDATE SET \
                            betting_options = EXCLUDED.betting_options, \
                            resolution_window_epochs = EXCLUDED.resolution_window_epochs, \
                            max_resolution_window_epochs = EXCLUDED.max_resolution_window_epochs, \
                            created_epoch = EXCLUDED.created_epoch, \
                            updated_at = NOW(), \
                            transaction_id = EXCLUDED.transaction_id";

                    diesel::sql_query(insert_sql)
                        .bind::<Text, _>(&post_id)
                        .bind::<SmallInt, _>(1) // STATUS_OPEN
                        .bind::<Nullable<SmallInt>, _>(None::<i16>)
                        .bind::<Integer, _>(3000) // Default AMM split
                        .bind::<Nullable<diesel::sql_types::Jsonb>, _>(Some(betting_options_json))
                        .bind::<Nullable<diesel::sql_types::Jsonb>, _>(Some(option_escrow_json))
                        .bind::<Nullable<BigInt>, _>(resolution_window)
                        .bind::<Nullable<BigInt>, _>(max_resolution_window)
                        .bind::<BigInt, _>(created_epoch)
                        .bind::<Nullable<BigInt>, _>(None::<i64>)
                        .bind::<BigInt, _>(1) // Version
                        .bind::<Text, _>(&tx)
                        .execute(&mut conn)
                        .await?;

                    // Log the record created event (includes record_id in raw_event)
                    Self::log_spot_event_in_transaction(
                        &mut conn,
                        "SpotRecordCreatedEvent",
                        &post_id,
                        &parsed_clone,
                        Some(event_id.clone()),
                    )
                    .await?;

                    // Create unified event entry
                    let unified = NewSocialProofOfTruthEvent {
                        event_type: "SpotRecordCreatedEvent".to_string(),
                        post_id: post_id.clone(),
                        user_address: None,
                        option_id: None,
                        escrow_amount: None,
                        amm_amount: None,
                        amount: None,
                        outcome: None,
                        total_escrow: None,
                        fee_taken: None,
                        confidence_bps: None,
                        timestamp_epoch,
                        time: Utc::now(),
                        event_id: Some(event_id.clone()),
                        transaction_id: Some(tx_digest.clone()),
                        raw_event: Some(parsed_json),
                    };

                    let event_for_unified = BlockchainEvent {
                        event_type: "SpotRecordCreatedEvent".to_string(),
                        event_id: event_id.clone(),
                        tx_digest: tx_digest.clone(),
                        timestamp_ms: event_timestamp_ms,
                        data: serde_json::Value::Null,
                    };

                    Self::write_unified_row_in_transaction(&mut conn, &event_for_unified, "SpotRecordCreatedEvent", unified)
                        .await?;

                    // Update post record with spot_id
                    diesel::update(schema::posts::table)
                        .filter(schema::posts::post_id.eq(&post_id))
                        .set(schema::posts::spot_id.eq(&parsed.record_id))
                        .execute(&mut conn)
                        .await?;

                    Ok::<(), DieselError>(())
                })
            })
            .await
            .context("Transaction failed for SpotRecordCreatedEvent")?;

        Ok(())
    }


    fn is_spot_event(event_type: &str) -> bool {
        event_type.contains("::social_proof_of_truth::")
            || event_type.ends_with("SpotBetPlacedEvent")
            || event_type.ends_with("SpotBetWithdrawnEvent")
            || event_type.ends_with("SpotResolvedEvent")
            || event_type.ends_with("SpotDaoRequiredEvent")
            || event_type.ends_with("SpotPayoutEvent")
            || event_type.ends_with("SpotRefundEvent")
            || event_type.ends_with("SpotConfigUpdatedEvent")
            || event_type.ends_with("SpotRecordCreatedEvent")
    }

    async fn update_progress(&self) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now().naive_utc();

        let progress = crate::social::models::indexer::NewIndexerProgress {
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
            } else if event.event_type.ends_with("SpotBetWithdrawnEvent") {
                self.handle_spot_bet_withdrawn(&event).await
            } else if event.event_type.ends_with("SpotResolvedEvent") {
                self.handle_spot_resolved(&event).await
            } else if event.event_type.ends_with("SpotDaoRequiredEvent") {
                self.handle_spot_dao_required(&event).await
            } else if event.event_type.ends_with("SpotPayoutEvent") {
                self.handle_spot_payout(&event).await
            } else if event.event_type.ends_with("SpotRefundEvent") {
                self.handle_spot_refund(&event).await
            } else if event.event_type.ends_with("SpotConfigUpdatedEvent") {
                self.handle_spot_config_updated(&event).await
            } else if event.event_type.ends_with("SpotRecordCreatedEvent") {
                self.handle_spot_record_created(&event).await
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
