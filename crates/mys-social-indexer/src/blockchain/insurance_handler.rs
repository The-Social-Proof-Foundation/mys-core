// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Context, Result};
use diesel::result::Error as DieselError;
use chrono::Utc;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, SmallInt, Text};
use diesel_async::RunQueryDsl;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::blockchain::listener::BlockchainEvent;
use crate::db::{Database, DbConnection};
use crate::events::event_utils::extract_event_fields;
use diesel_async::AsyncPgConnection;
use crate::events::insurance_events::{
    ConfigInitializedEvent, CoverageCancelledEvent, CoverageClaimedEvent,
    CoveragePurchasedEvent, new_insurance_event_log, PolicyExpiredEvent,
    UnderwriterVaultCreatedEvent, UnderwriterVaultDepositedEvent,
    UnderwriterVaultWithdrawnEvent,
};
use crate::events::InsuranceConfigUpdatedEvent;
use crate::events::insurance_event_types::{
    EVENT_COVERAGE_CANCELLED, EVENT_COVERAGE_CLAIMED, EVENT_COVERAGE_PURCHASED,
    EVENT_CONFIG_INITIALIZED, EVENT_CONFIG_UPDATED, EVENT_POLICY_EXPIRED,
    EVENT_VAULT_CREATED, EVENT_VAULT_DEPOSITED, EVENT_VAULT_WITHDRAWN,
    STATUS_CANCELLED, STATUS_CLAIMED, STATUS_EXPIRED,
};
// Note: NewInsurance* types are used through conversion methods (into_*_model)
// but Rust's unused import checker doesn't track indirect usage
#[allow(unused_imports)]
use crate::models::insurance::{
    NewInsuranceConfig, NewInsuranceEventLog, NewInsuranceMarketExposure, NewInsurancePolicy,
    NewInsurancePolicyEvent, NewInsuranceUserExposure, NewInsuranceVault,
    NewInsuranceVaultTransaction,
};
use crate::models::insurance::{UpdateInsurancePolicy, UpdateInsuranceVault};
use crate::schema;

/// Handler for Insurance blockchain events.
pub struct InsuranceEventHandler {
    db: Arc<Database>,
    rx: mpsc::Receiver<BlockchainEvent>,
    worker_name: String,
}

impl InsuranceEventHandler {
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
            .map_err(|e| anyhow!("Failed to deserialize insurance event payload: {}", e))
    }

    async fn log_insurance_event(
        conn: &mut DbConnection,
        event_type: &str,
        event_payload: &impl serde::Serialize,
        event_id: Option<String>,
    ) -> Result<()> {
        if let Some(event_id) = event_id {
            let json = serde_json::to_value(event_payload)?;
            diesel::insert_into(schema::insurance_events::table)
                .values(&new_insurance_event_log(event_type, &json, &event_id))
                .execute(conn)
                .await?;
        }
        Ok(())
    }

    async fn log_insurance_event_in_transaction(
        conn: &mut AsyncPgConnection,
        event_type: &str,
        event_payload: &impl serde::Serialize,
        event_id: Option<String>,
    ) -> Result<(), DieselError> {
        if let Some(event_id) = event_id {
            let json = serde_json::to_value(event_payload)
                .map_err(|e| DieselError::QueryBuilderError(format!("JSON serialization error: {}", e).into()))?;
            diesel::insert_into(schema::insurance_events::table)
                .values(&new_insurance_event_log(event_type, &json, &event_id))
                .execute(conn)
                .await?;
        }
        Ok(())
    }

    async fn handle_config_initialized(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<ConfigInitializedEvent>(&event.data)
            .context("Failed to parse ConfigInitializedEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let parsed_clone = parsed.clone();

        let config = parsed
            .into_config_model(event_timestamp_ms, tx_digest.clone())
            .map_err(|e| anyhow!("Failed to convert ConfigInitializedEvent: {}", e))?;

        diesel::insert_into(schema::insurance_config::table)
            .values(&config)
            .execute(&mut conn)
            .await
            .context("Failed to insert insurance config")?;

        Self::log_insurance_event(
            &mut conn,
            EVENT_CONFIG_INITIALIZED,
            &parsed_clone,
            Some(event_id.clone()),
        )
        .await
        .context("Failed to log insurance event")?;

        Ok(())
    }

    async fn handle_config_updated(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<InsuranceConfigUpdatedEvent>(&event.data)
            .context("Failed to parse ConfigUpdatedEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let parsed_clone = parsed.clone();

        // Use timestamp_ms from BlockchainEvent (in milliseconds) for correct timestamp
        // The database trigger will convert it: to_timestamp(timestamp_ms / 1000)
        let config = parsed
            .into_config_model(event_timestamp_ms, tx_digest.clone())
            .map_err(|e| anyhow!("Failed to convert ConfigUpdatedEvent: {}", e))?;

        diesel::insert_into(schema::insurance_config::table)
            .values(&config)
            .execute(&mut conn)
            .await
            .context("Failed to insert insurance config update")?;

        Self::log_insurance_event(
            &mut conn,
            EVENT_CONFIG_UPDATED,
            &parsed_clone,
            Some(event_id.clone()),
        )
        .await
        .context("Failed to log insurance event")?;

        Ok(())
    }

    async fn handle_vault_created(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<UnderwriterVaultCreatedEvent>(&event.data)
            .context("Failed to parse UnderwriterVaultCreatedEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let parsed_clone = parsed.clone();

        let vault = parsed
            .into_vault_model(tx_digest.clone())
            .map_err(|e| anyhow!("Failed to convert UnderwriterVaultCreatedEvent: {}", e))?;

        diesel::insert_into(schema::insurance_vaults::table)
            .values(&vault)
            .execute(&mut conn)
            .await
            .context("Failed to insert insurance vault")?;

        Self::log_insurance_event(
            &mut conn,
            EVENT_VAULT_CREATED,
            &parsed_clone,
            Some(event_id.clone()),
        )
        .await
        .context("Failed to log insurance event")?;

        Ok(())
    }

    async fn handle_vault_deposited(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<UnderwriterVaultDepositedEvent>(&event.data)
            .context("Failed to parse UnderwriterVaultDepositedEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let vault_id = parsed.vault_id.clone();
        let new_balance = parsed.new_balance;
        let parsed_clone = parsed.clone();

        // Wrap in transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Verify vault exists
                    #[derive(QueryableByName)]
                    struct VaultExistsRow {
                        #[diesel(sql_type = BigInt)]
                        #[allow(dead_code)]
                        exists: i64,
                    }

                    let result: Result<VaultExistsRow, _> = diesel::sql_query(
                        "SELECT 1 as exists FROM insurance_vaults WHERE vault_id = $1 LIMIT 1"
                    )
                    .bind::<Text, _>(&vault_id)
                    .get_result(&mut conn)
                    .await;

                    if result.is_err() {
                        return Err(DieselError::QueryBuilderError(
                            format!("Vault does not exist: {}", vault_id).into()
                        ));
                    }

                    // Update vault balance
                    diesel::update(schema::insurance_vaults::table)
                        .filter(schema::insurance_vaults::vault_id.eq(&vault_id))
                        .set((
                            schema::insurance_vaults::capital_balance.eq(new_balance as i64),
                            schema::insurance_vaults::updated_at.eq(Utc::now().naive_utc()),
                        ))
                        .execute(&mut conn)
                        .await?;

                    // Create transaction record
                    let transaction = parsed_clone
                        .into_transaction_model(event_timestamp_ms, tx_digest.clone())
                        .map_err(|e| DieselError::QueryBuilderError(
                            format!("Failed to convert UnderwriterVaultDepositedEvent: {}", e).into()
                        ))?;

                    diesel::insert_into(schema::insurance_vault_transactions::table)
                        .values(&transaction)
                        .execute(&mut conn)
                        .await?;

                    // Log the event
                    Self::log_insurance_event_in_transaction(
                        &mut conn,
                        EVENT_VAULT_DEPOSITED,
                        &parsed_clone,
                        Some(event_id.clone()),
                    )
                    .await?;

                    Ok::<(), DieselError>(())
                })
            })
            .await
            .map_err(|e| anyhow!("Transaction failed for UnderwriterVaultDepositedEvent: {}", e))?;

        Ok(())
    }

    async fn handle_vault_withdrawn(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<UnderwriterVaultWithdrawnEvent>(&event.data)
            .context("Failed to parse UnderwriterVaultWithdrawnEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let vault_id = parsed.vault_id.clone();
        let new_balance = parsed.new_balance;
        let parsed_clone = parsed.clone();

        // Wrap in transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Verify vault exists
                    #[derive(QueryableByName)]
                    struct VaultExistsRow {
                        #[diesel(sql_type = BigInt)]
                        #[allow(dead_code)]
                        exists: i64,
                    }

                    let result: Result<VaultExistsRow, _> = diesel::sql_query(
                        "SELECT 1 as exists FROM insurance_vaults WHERE vault_id = $1 LIMIT 1"
                    )
                    .bind::<Text, _>(&vault_id)
                    .get_result(&mut conn)
                    .await;

                    if result.is_err() {
                        return Err(DieselError::QueryBuilderError(
                            format!("Vault does not exist: {}", vault_id).into()
                        ));
                    }

                    // Update vault balance using UpdateInsuranceVault model
                    let vault_update = UpdateInsuranceVault {
                        capital_balance: Some(new_balance as i64),
                        reserved: None,
                        updated_at: Some(Utc::now().naive_utc()),
                    };
                    diesel::update(schema::insurance_vaults::table)
                        .filter(schema::insurance_vaults::vault_id.eq(&vault_id))
                        .set(&vault_update)
                        .execute(&mut conn)
                        .await?;

                    // Create transaction record
                    let transaction = parsed_clone
                        .into_transaction_model(event_timestamp_ms, tx_digest.clone())
                        .map_err(|e| DieselError::QueryBuilderError(
                            format!("Failed to convert UnderwriterVaultWithdrawnEvent: {}", e).into()
                        ))?;

                    diesel::insert_into(schema::insurance_vault_transactions::table)
                        .values(&transaction)
                        .execute(&mut conn)
                        .await?;

                    // Log the event
                    Self::log_insurance_event_in_transaction(
                        &mut conn,
                        EVENT_VAULT_WITHDRAWN,
                        &parsed_clone,
                        Some(event_id.clone()),
                    )
                    .await?;

                    Ok::<(), DieselError>(())
                })
            })
            .await
            .map_err(|e| anyhow!("Transaction failed for UnderwriterVaultWithdrawnEvent: {}", e))?;

        Ok(())
    }

    async fn handle_coverage_purchased(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<CoveragePurchasedEvent>(&event.data)
            .context("Failed to parse CoveragePurchasedEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let parsed_clone = parsed.clone();

        // Wrap in transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // CoveragePurchasedEvent doesn't include vault_id, so we must query for it.
                    // We use a heuristic: find a vault with sufficient capital, preferring recently updated ones.
                    #[derive(QueryableByName)]
                    struct VaultIdRow {
                        #[diesel(sql_type = Text)]
                        vault_id: String,
                    }

                    let vault_id = {
                        // First, try to find a vault with sufficient capital
                        let vault_result: Result<VaultIdRow, _> = diesel::sql_query(
                            "SELECT vault_id FROM insurance_vaults 
                             WHERE capital_balance >= $1 
                             ORDER BY updated_at DESC LIMIT 1"
                        )
                        .bind::<BigInt, _>(parsed_clone.reserve_locked as i64)
                        .get_result(&mut conn)
                        .await;

                        match vault_result {
                            Ok(row) => row.vault_id,
                            Err(_) => {
                                // Fallback: use any vault (should rarely happen in production)
                                let any_vault_result: Result<VaultIdRow, _> = diesel::sql_query(
                                    "SELECT vault_id FROM insurance_vaults ORDER BY updated_at DESC LIMIT 1"
                                )
                                .get_result(&mut conn)
                                .await;
                                
                                match any_vault_result {
                                    Ok(row) => {
                                        warn!(
                                            "No vault with sufficient capital found. Using vault {} for policy {}",
                                            row.vault_id, parsed_clone.policy_id
                                        );
                                        row.vault_id
                                    }
                                    Err(_) => {
                                        return Err(DieselError::QueryBuilderError(
                                            format!("No vault found for coverage purchase. Policy ID: {}", parsed_clone.policy_id).into()
                                        ));
                                    }
                                }
                            }
                        }
                    };

                    // Create policy
                    let start_time_ms = event_timestamp_ms; // Use event timestamp as start time
                    let policy = parsed_clone
                        .into_policy_model(start_time_ms, vault_id.clone(), tx_digest.clone())
                        .map_err(|e| DieselError::QueryBuilderError(
                            format!("Failed to convert CoveragePurchasedEvent: {}", e).into()
                        ))?;

                    diesel::insert_into(schema::insurance_policies::table)
                        .values(&policy)
                        .execute(&mut conn)
                        .await?;

                    // Update vault: add premium to capital_balance and reserve_locked to reserved
                    #[derive(QueryableByName)]
                    struct VaultBalanceRow {
                        #[diesel(sql_type = BigInt)]
                        capital_balance: i64,
                        #[diesel(sql_type = BigInt)]
                        reserved: i64,
                    }
                    let current_vault: Result<VaultBalanceRow, _> = diesel::sql_query(
                        "SELECT capital_balance, reserved FROM insurance_vaults WHERE vault_id = $1"
                    )
                    .bind::<Text, _>(&vault_id)
                    .get_result(&mut conn)
                    .await;
                    
                    let (new_capital, new_reserved) = match current_vault {
                        Ok(row) => (
                            row.capital_balance + parsed_clone.premium_paid as i64,
                            row.reserved + parsed_clone.reserve_locked as i64,
                        ),
                        Err(_) => (
                            parsed_clone.premium_paid as i64,
                            parsed_clone.reserve_locked as i64,
                        ),
                    };
                    
                    let vault_update = UpdateInsuranceVault {
                        capital_balance: Some(new_capital),
                        reserved: Some(new_reserved),
                        updated_at: Some(Utc::now().naive_utc()),
                    };
                    diesel::update(schema::insurance_vaults::table)
                        .filter(schema::insurance_vaults::vault_id.eq(&vault_id))
                        .set(&vault_update)
                        .execute(&mut conn)
                        .await?;

                    // Create policy event record
                    let policy_event = parsed_clone
                        .into_policy_event_model(vault_id.clone(), event_timestamp_ms, tx_digest.clone())
                        .map_err(|e| DieselError::QueryBuilderError(
                            format!("Failed to convert CoveragePurchasedEvent to policy event: {}", e).into()
                        ))?;

                    diesel::insert_into(schema::insurance_policy_events::table)
                        .values(&policy_event)
                        .execute(&mut conn)
                        .await?;

                    // Create market exposure record
                    let market_exposure = parsed_clone
                        .into_market_exposure_model(vault_id.clone(), event_timestamp_ms, tx_digest.clone())
                        .map_err(|e| DieselError::QueryBuilderError(
                            format!("Failed to convert CoveragePurchasedEvent to market exposure: {}", e).into()
                        ))?;

                    diesel::insert_into(schema::insurance_market_exposures::table)
                        .values(&market_exposure)
                        .execute(&mut conn)
                        .await?;

                    // Create user exposure record
                    let user_exposure = parsed_clone
                        .into_user_exposure_model(vault_id.clone(), event_timestamp_ms, tx_digest.clone())
                        .map_err(|e| DieselError::QueryBuilderError(
                            format!("Failed to convert CoveragePurchasedEvent to user exposure: {}", e).into()
                        ))?;

                    diesel::insert_into(schema::insurance_user_exposures::table)
                        .values(&user_exposure)
                        .execute(&mut conn)
                        .await?;

                    // Log the event
                    Self::log_insurance_event_in_transaction(
                        &mut conn,
                        EVENT_COVERAGE_PURCHASED,
                        &parsed_clone,
                        Some(event_id.clone()),
                    )
                    .await?;

                    Ok::<(), DieselError>(())
                })
            })
            .await
            .map_err(|e| anyhow!("Transaction failed for CoveragePurchasedEvent: {}", e))?;

        Ok(())
    }

    async fn handle_coverage_cancelled(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<CoverageCancelledEvent>(&event.data)
            .context("Failed to parse CoverageCancelledEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let policy_id = parsed.policy_id.clone();
        let parsed_clone = parsed.clone();

        // Wrap in transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Get policy details
                    #[derive(QueryableByName)]
                    struct PolicyRow {
                        #[diesel(sql_type = Text)]
                        market_id: String,
                        #[diesel(sql_type = SmallInt)]
                        option_id: i16,
                        #[diesel(sql_type = BigInt)]
                        covered_amount: i64,
                        #[diesel(sql_type = BigInt)]
                        coverage_bps: i64,
                        #[diesel(sql_type = BigInt)]
                        premium_paid: i64,
                        #[diesel(sql_type = Text)]
                        vault_id: String,
                    }

                    let policy_result: Result<PolicyRow, _> = diesel::sql_query(
                        "SELECT market_id, option_id, covered_amount, coverage_bps, premium_paid, vault_id FROM insurance_policies WHERE policy_id = $1"
                    )
                    .bind::<Text, _>(&policy_id)
                    .get_result(&mut conn)
                    .await;

                    let policy = match policy_result {
                        Ok(p) => p,
                        Err(_) => {
                            return Err(DieselError::QueryBuilderError(
                                format!("Policy not found: {}", policy_id).into()
                            ));
                        }
                    };

                    // Calculate reserve_locked (covered_amount * coverage_bps / 10000)
                    let reserve_locked = (policy.covered_amount as i128 * policy.coverage_bps as i128) / 10000;

                    // Update policy status using UpdateInsurancePolicy model
                    let policy_update = UpdateInsurancePolicy {
                        status: Some(STATUS_CANCELLED as i16),
                        updated_at: Some(Utc::now().naive_utc()),
                    };
                    diesel::update(schema::insurance_policies::table)
                        .filter(schema::insurance_policies::policy_id.eq(&policy_id))
                        .set(&policy_update)
                        .execute(&mut conn)
                        .await?;

                    // Update vault: subtract refund + fee from capital_balance and reserve from reserved
                    let total_refund = parsed_clone.refunded_amount + parsed_clone.fee_paid;
                    #[derive(QueryableByName)]
                    struct VaultBalanceRow {
                        #[diesel(sql_type = BigInt)]
                        capital_balance: i64,
                        #[diesel(sql_type = BigInt)]
                        reserved: i64,
                    }
                    let current_vault: Result<VaultBalanceRow, _> = diesel::sql_query(
                        "SELECT capital_balance, reserved FROM insurance_vaults WHERE vault_id = $1"
                    )
                    .bind::<Text, _>(&policy.vault_id)
                    .get_result(&mut conn)
                    .await;
                    
                    let (new_capital, new_reserved) = match current_vault {
                        Ok(row) => (
                            (row.capital_balance - total_refund as i64).max(0),
                            (row.reserved - reserve_locked as i64).max(0),
                        ),
                        Err(_) => (0, 0),
                    };
                    
                    let vault_update = UpdateInsuranceVault {
                        capital_balance: Some(new_capital),
                        reserved: Some(new_reserved),
                        updated_at: Some(Utc::now().naive_utc()),
                    };
                    diesel::update(schema::insurance_vaults::table)
                        .filter(schema::insurance_vaults::vault_id.eq(&policy.vault_id))
                        .set(&vault_update)
                        .execute(&mut conn)
                        .await?;

                    // Clone values before moving them
                    let market_id = policy.market_id.clone();
                    let vault_id = policy.vault_id.clone();
                    let insured = parsed_clone.insured.clone();

                    // Create policy event record
                    let policy_event = parsed_clone
                        .into_policy_event_model(
                            market_id.clone(),
                            policy.option_id as u8,
                            policy.covered_amount as u64,
                            policy.coverage_bps as u64,
                            policy.premium_paid as u64,
                            reserve_locked as u64,
                            vault_id.clone(),
                            event_timestamp_ms,
                            tx_digest.clone(),
                        )
                        .map_err(|e| DieselError::QueryBuilderError(
                            format!("Failed to convert CoverageCancelledEvent: {}", e).into()
                        ))?;

                    diesel::insert_into(schema::insurance_policy_events::table)
                        .values(&policy_event)
                        .execute(&mut conn)
                        .await?;

                    // Create negative exposure records (to reduce exposure)
                    let market_exposure = NewInsuranceMarketExposure {
                        vault_id: vault_id.clone(),
                        market_id: market_id.clone(),
                        option_id: policy.option_id,
                        reserved_amount: -(reserve_locked as i64),
                        timestamp_ms: event_timestamp_ms as i64,
                        time: Utc::now(),
                        transaction_id: tx_digest.clone(),
                    };

                    diesel::insert_into(schema::insurance_market_exposures::table)
                        .values(&market_exposure)
                        .execute(&mut conn)
                        .await?;

                    let user_exposure = NewInsuranceUserExposure {
                        vault_id,
                        insured,
                        reserved_amount: -(reserve_locked as i64),
                        timestamp_ms: event_timestamp_ms as i64,
                        time: Utc::now(),
                        transaction_id: tx_digest.clone(),
                    };

                    diesel::insert_into(schema::insurance_user_exposures::table)
                        .values(&user_exposure)
                        .execute(&mut conn)
                        .await?;

                    // Log the event
                    Self::log_insurance_event_in_transaction(
                        &mut conn,
                        EVENT_COVERAGE_CANCELLED,
                        &parsed_clone,
                        Some(event_id.clone()),
                    )
                    .await?;

                    Ok::<(), DieselError>(())
                })
            })
            .await
            .map_err(|e| anyhow!("Transaction failed for CoverageCancelledEvent: {}", e))?;

        Ok(())
    }

    async fn handle_coverage_claimed(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<CoverageClaimedEvent>(&event.data)
            .context("Failed to parse CoverageClaimedEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let policy_id = parsed.policy_id.clone();
        let parsed_clone = parsed.clone();

        // Wrap in transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Get policy details
                    #[derive(QueryableByName)]
                    struct PolicyRow {
                        #[diesel(sql_type = Text)]
                        market_id: String,
                        #[diesel(sql_type = SmallInt)]
                        option_id: i16,
                        #[diesel(sql_type = BigInt)]
                        covered_amount: i64,
                        #[diesel(sql_type = BigInt)]
                        coverage_bps: i64,
                        #[diesel(sql_type = BigInt)]
                        premium_paid: i64,
                        #[diesel(sql_type = Text)]
                        vault_id: String,
                    }

                    let policy_result: Result<PolicyRow, _> = diesel::sql_query(
                        "SELECT market_id, option_id, covered_amount, coverage_bps, premium_paid, vault_id FROM insurance_policies WHERE policy_id = $1"
                    )
                    .bind::<Text, _>(&policy_id)
                    .get_result(&mut conn)
                    .await;

                    let policy = match policy_result {
                        Ok(p) => p,
                        Err(_) => {
                            return Err(DieselError::QueryBuilderError(
                                format!("Policy not found: {}", policy_id).into()
                            ));
                        }
                    };

                    // Calculate reserve_locked (covered_amount * coverage_bps / 10000)
                    let reserve_locked = (policy.covered_amount as i128 * policy.coverage_bps as i128) / 10000;

                    // Update policy status using UpdateInsurancePolicy model
                    let policy_update = UpdateInsurancePolicy {
                        status: Some(STATUS_CLAIMED as i16),
                        updated_at: Some(Utc::now().naive_utc()),
                    };
                    diesel::update(schema::insurance_policies::table)
                        .filter(schema::insurance_policies::policy_id.eq(&policy_id))
                        .set(&policy_update)
                        .execute(&mut conn)
                        .await?;

                    // Update vault reserved amount (subtract reserve) and capital (subtract payout) using UpdateInsuranceVault model
                    #[derive(QueryableByName)]
                    struct VaultBalanceRow {
                        #[diesel(sql_type = BigInt)]
                        reserved: i64,
                        #[diesel(sql_type = BigInt)]
                        capital_balance: i64,
                    }
                    let current_vault: Result<VaultBalanceRow, _> = diesel::sql_query(
                        "SELECT reserved, capital_balance FROM insurance_vaults WHERE vault_id = $1"
                    )
                    .bind::<Text, _>(&policy.vault_id)
                    .get_result(&mut conn)
                    .await;
                    
                    let (new_reserved, new_capital) = match current_vault {
                        Ok(row) => (
                            (row.reserved - reserve_locked as i64).max(0),
                            (row.capital_balance - parsed_clone.payout as i64).max(0),
                        ),
                        Err(_) => (0, 0),
                    };
                    
                    let vault_update = UpdateInsuranceVault {
                        capital_balance: Some(new_capital),
                        reserved: Some(new_reserved),
                        updated_at: Some(Utc::now().naive_utc()),
                    };
                    diesel::update(schema::insurance_vaults::table)
                        .filter(schema::insurance_vaults::vault_id.eq(&policy.vault_id))
                        .set(&vault_update)
                        .execute(&mut conn)
                        .await?;

                    // Clone values before moving them
                    let market_id = policy.market_id.clone();
                    let vault_id = policy.vault_id.clone();
                    let insured = parsed_clone.insured.clone();

                    // Create policy event record
                    let policy_event = parsed_clone
                        .into_policy_event_model(
                            market_id.clone(),
                            policy.option_id as u8,
                            policy.covered_amount as u64,
                            policy.coverage_bps as u64,
                            policy.premium_paid as u64,
                            reserve_locked as u64,
                            vault_id.clone(),
                            event_timestamp_ms,
                            tx_digest.clone(),
                        )
                        .map_err(|e| DieselError::QueryBuilderError(
                            format!("Failed to convert CoverageClaimedEvent: {}", e).into()
                        ))?;

                    diesel::insert_into(schema::insurance_policy_events::table)
                        .values(&policy_event)
                        .execute(&mut conn)
                        .await?;

                    // Create negative exposure records (to reduce exposure)
                    let market_exposure = NewInsuranceMarketExposure {
                        vault_id: vault_id.clone(),
                        market_id: market_id.clone(),
                        option_id: policy.option_id,
                        reserved_amount: -(reserve_locked as i64),
                        timestamp_ms: event_timestamp_ms as i64,
                        time: Utc::now(),
                        transaction_id: tx_digest.clone(),
                    };

                    diesel::insert_into(schema::insurance_market_exposures::table)
                        .values(&market_exposure)
                        .execute(&mut conn)
                        .await?;

                    let user_exposure = NewInsuranceUserExposure {
                        vault_id,
                        insured,
                        reserved_amount: -(reserve_locked as i64),
                        timestamp_ms: event_timestamp_ms as i64,
                        time: Utc::now(),
                        transaction_id: tx_digest.clone(),
                    };

                    diesel::insert_into(schema::insurance_user_exposures::table)
                        .values(&user_exposure)
                        .execute(&mut conn)
                        .await?;

                    // Log the event
                    Self::log_insurance_event_in_transaction(
                        &mut conn,
                        EVENT_COVERAGE_CLAIMED,
                        &parsed_clone,
                        Some(event_id.clone()),
                    )
                    .await?;

                    Ok::<(), DieselError>(())
                })
            })
            .await
            .map_err(|e| anyhow!("Transaction failed for CoverageClaimedEvent: {}", e))?;

        Ok(())
    }

    async fn handle_policy_expired(&self, event: &BlockchainEvent) -> Result<()> {
        let parsed = Self::parse_event::<PolicyExpiredEvent>(&event.data)
            .context("Failed to parse PolicyExpiredEvent")?;
        let mut conn = self.get_connection().await?;
        let event_id = event.event_id.clone();
        let tx_digest = event.tx_digest.clone();
        let event_timestamp_ms = event.timestamp_ms;
        let policy_id = parsed.policy_id.clone();
        let parsed_clone = parsed.clone();

        // Wrap in transaction for atomicity
        conn.build_transaction()
            .run(|mut conn| {
                Box::pin(async move {
                    // Get policy details
                    #[derive(QueryableByName)]
                    struct PolicyRow {
                        #[diesel(sql_type = Text)]
                        market_id: String,
                        #[diesel(sql_type = SmallInt)]
                        option_id: i16,
                        #[diesel(sql_type = BigInt)]
                        covered_amount: i64,
                        #[diesel(sql_type = BigInt)]
                        coverage_bps: i64,
                        #[diesel(sql_type = BigInt)]
                        premium_paid: i64,
                        #[diesel(sql_type = Text)]
                        vault_id: String,
                    }

                    let policy_result: Result<PolicyRow, _> = diesel::sql_query(
                        "SELECT market_id, option_id, covered_amount, coverage_bps, premium_paid, vault_id FROM insurance_policies WHERE policy_id = $1"
                    )
                    .bind::<Text, _>(&policy_id)
                    .get_result(&mut conn)
                    .await;

                    let policy = match policy_result {
                        Ok(p) => p,
                        Err(_) => {
                            return Err(DieselError::QueryBuilderError(
                                format!("Policy not found: {}", policy_id).into()
                            ));
                        }
                    };

                    // Verify vault_id matches
                    if policy.vault_id != parsed_clone.vault_id {
                        return Err(DieselError::QueryBuilderError(
                            format!("Vault ID mismatch for policy {}: expected {}, got {}", 
                                policy_id, parsed_clone.vault_id, policy.vault_id).into()
                        ));
                    }

                    // Update policy status using UpdateInsurancePolicy model
                    let policy_update = UpdateInsurancePolicy {
                        status: Some(STATUS_EXPIRED as i16),
                        updated_at: Some(Utc::now().naive_utc()),
                    };
                    diesel::update(schema::insurance_policies::table)
                        .filter(schema::insurance_policies::policy_id.eq(&policy_id))
                        .set(&policy_update)
                        .execute(&mut conn)
                        .await?;

                    // Update vault reserved amount (subtract reserve_released) using UpdateInsuranceVault model
                    #[derive(QueryableByName)]
                    struct VaultReservedRow {
                        #[diesel(sql_type = BigInt)]
                        reserved: i64,
                    }
                    let current_reserved: Result<VaultReservedRow, _> = diesel::sql_query(
                        "SELECT reserved FROM insurance_vaults WHERE vault_id = $1"
                    )
                    .bind::<Text, _>(&policy.vault_id)
                    .get_result(&mut conn)
                    .await;
                    
                    let new_reserved = match current_reserved {
                        Ok(row) => (row.reserved - parsed_clone.reserve_released as i64).max(0),
                        Err(_) => 0,
                    };
                    
                    let vault_update = UpdateInsuranceVault {
                        capital_balance: None,
                        reserved: Some(new_reserved),
                        updated_at: Some(Utc::now().naive_utc()),
                    };
                    diesel::update(schema::insurance_vaults::table)
                        .filter(schema::insurance_vaults::vault_id.eq(&policy.vault_id))
                        .set(&vault_update)
                        .execute(&mut conn)
                        .await?;

                    // Clone values before moving them
                    let market_id = policy.market_id.clone();
                    let vault_id = policy.vault_id.clone();
                    let insured = parsed_clone.insured.clone();

                    // Create policy event record
                    let policy_event = parsed_clone
                        .into_policy_event_model(
                            policy.option_id as u8,
                            policy.covered_amount as u64,
                            policy.coverage_bps as u64,
                            policy.premium_paid as u64,
                            event_timestamp_ms,
                            tx_digest.clone(),
                        )
                        .map_err(|e| DieselError::QueryBuilderError(
                            format!("Failed to convert PolicyExpiredEvent: {}", e).into()
                        ))?;

                    diesel::insert_into(schema::insurance_policy_events::table)
                        .values(&policy_event)
                        .execute(&mut conn)
                        .await?;

                    // Create negative exposure records (to reduce exposure)
                    let market_exposure = NewInsuranceMarketExposure {
                        vault_id: vault_id.clone(),
                        market_id: market_id.clone(),
                        option_id: policy.option_id,
                        reserved_amount: -(parsed_clone.reserve_released as i64),
                        timestamp_ms: event_timestamp_ms as i64,
                        time: Utc::now(),
                        transaction_id: tx_digest.clone(),
                    };

                    diesel::insert_into(schema::insurance_market_exposures::table)
                        .values(&market_exposure)
                        .execute(&mut conn)
                        .await?;

                    let user_exposure = NewInsuranceUserExposure {
                        vault_id,
                        insured,
                        reserved_amount: -(parsed_clone.reserve_released as i64),
                        timestamp_ms: event_timestamp_ms as i64,
                        time: Utc::now(),
                        transaction_id: tx_digest.clone(),
                    };

                    diesel::insert_into(schema::insurance_user_exposures::table)
                        .values(&user_exposure)
                        .execute(&mut conn)
                        .await?;

                    // Log the event
                    Self::log_insurance_event_in_transaction(
                        &mut conn,
                        EVENT_POLICY_EXPIRED,
                        &parsed_clone,
                        Some(event_id.clone()),
                    )
                    .await?;

                    Ok::<(), DieselError>(())
                })
            })
            .await
            .map_err(|e| anyhow!("Transaction failed for PolicyExpiredEvent: {}", e))?;

        Ok(())
    }

    fn is_insurance_event(event_type: &str) -> bool {
        event_type.contains("::insurance::")
            || event_type.ends_with(EVENT_CONFIG_INITIALIZED)
            || event_type.ends_with(EVENT_CONFIG_UPDATED)
            || event_type.ends_with(EVENT_VAULT_CREATED)
            || event_type.ends_with(EVENT_VAULT_DEPOSITED)
            || event_type.ends_with(EVENT_VAULT_WITHDRAWN)
            || event_type.ends_with(EVENT_COVERAGE_PURCHASED)
            || event_type.ends_with(EVENT_COVERAGE_CANCELLED)
            || event_type.ends_with(EVENT_COVERAGE_CLAIMED)
            || event_type.ends_with(EVENT_POLICY_EXPIRED)
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
        info!("Starting Insurance event handler");

        while let Some(event) = self.rx.recv().await {
            debug!("Processing Insurance event: {}", event.event_type);

            if !Self::is_insurance_event(&event.event_type) {
                continue;
            }

            let result = if event.event_type.ends_with(EVENT_CONFIG_INITIALIZED) {
                self.handle_config_initialized(&event).await
            } else if event.event_type.ends_with(EVENT_CONFIG_UPDATED) {
                self.handle_config_updated(&event).await
            } else if event.event_type.ends_with(EVENT_VAULT_CREATED) {
                self.handle_vault_created(&event).await
            } else if event.event_type.ends_with(EVENT_VAULT_DEPOSITED) {
                self.handle_vault_deposited(&event).await
            } else if event.event_type.ends_with(EVENT_VAULT_WITHDRAWN) {
                self.handle_vault_withdrawn(&event).await
            } else if event.event_type.ends_with(EVENT_COVERAGE_PURCHASED) {
                self.handle_coverage_purchased(&event).await
            } else if event.event_type.ends_with(EVENT_COVERAGE_CANCELLED) {
                self.handle_coverage_cancelled(&event).await
            } else if event.event_type.ends_with(EVENT_COVERAGE_CLAIMED) {
                self.handle_coverage_claimed(&event).await
            } else if event.event_type.ends_with(EVENT_POLICY_EXPIRED) {
                self.handle_policy_expired(&event).await
            } else {
                warn!("Received unhandled Insurance event: {} (event_id: {})", event.event_type, event.event_id);
                Ok(())
            };

            if let Err(err) = result {
                error!("Failed to process Insurance event {}: {}", event.event_type, err);
            } else if let Err(e) = self.update_progress().await {
                warn!("Failed to update Insurance handler progress: {}", e);
            }
        }

        warn!("Insurance event handler channel closed");
        Ok(())
    }
}

