// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::social::events::event_utils::deserialize_u64_from_string;

use crate::social::models::insurance::{
    NewInsuranceConfig, NewInsuranceEventLog, NewInsuranceMarketExposure, NewInsurancePolicy,
    NewInsurancePolicyEvent, NewInsuranceUserExposure, NewInsuranceVault,
    NewInsuranceVaultTransaction,
};

// Matches social_contracts::insurance::ConfigInitializedEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInitializedEvent {
    pub admin: String,
    pub min_coverage_bps: u64,
    pub max_coverage_bps: u64, 
    pub max_duration_ms: u64,
    pub fee_bps: u64,
}

impl ConfigInitializedEvent {
    pub fn into_config_model(&self, timestamp_ms: u64, tx: String) -> Result<NewInsuranceConfig> {
        Ok(NewInsuranceConfig {
            updated_by: self.admin.clone(),
            enable_flag: false, // Config is initialized as disabled
            min_coverage_bps: self.min_coverage_bps as i64,
            max_coverage_bps: self.max_coverage_bps as i64,
            max_duration_ms: self.max_duration_ms as i64,
            fee_bps: self.fee_bps as i64,
            version: 1,
            timestamp_ms: timestamp_ms as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::insurance::UnderwriterVaultCreatedEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderwriterVaultCreatedEvent {
    pub vault_id: String,
    pub underwriter: String,
    pub base_rate_bps_per_day: u64,
    pub utilization_multiplier_bps: u64,
    pub max_exposure_per_market: u64,
    pub max_exposure_per_user: u64,
}

impl UnderwriterVaultCreatedEvent {
    pub fn into_vault_model(&self, tx: String) -> Result<NewInsuranceVault> {
        let now = Utc::now().naive_utc();
        Ok(NewInsuranceVault {
            vault_id: self.vault_id.clone(),
            underwriter: self.underwriter.clone(),
            capital_balance: 0, // Vault starts with zero capital
            reserved: 0,        // No reserves initially
            base_rate_bps_per_day: self.base_rate_bps_per_day as i64,
            utilization_multiplier_bps: self.utilization_multiplier_bps as i64,
            max_exposure_per_market: self.max_exposure_per_market as i64,
            max_exposure_per_user: self.max_exposure_per_user as i64,
            version: 1,
            created_at: now,
            updated_at: now,
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::insurance::UnderwriterVaultDepositedEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderwriterVaultDepositedEvent {
    pub vault_id: String,
    pub amount: u64,
    pub new_balance: u64,
}

impl UnderwriterVaultDepositedEvent {
    pub fn into_transaction_model(
        &self,
        timestamp_ms: u64,
        tx: String,
    ) -> Result<NewInsuranceVaultTransaction> {
        Ok(NewInsuranceVaultTransaction {
            vault_id: self.vault_id.clone(),
            transaction_type: "DEPOSIT".to_string(),
            amount: self.amount as i64,
            balance_after: self.new_balance as i64,
            timestamp_ms: timestamp_ms as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::insurance::UnderwriterVaultWithdrawnEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderwriterVaultWithdrawnEvent {
    pub vault_id: String,
    pub amount: u64,
    pub new_balance: u64,
}

impl UnderwriterVaultWithdrawnEvent {
    pub fn into_transaction_model(
        &self,
        timestamp_ms: u64,
        tx: String,
    ) -> Result<NewInsuranceVaultTransaction> {
        Ok(NewInsuranceVaultTransaction {
            vault_id: self.vault_id.clone(),
            transaction_type: "WITHDRAWAL".to_string(),
            amount: self.amount as i64,
            balance_after: self.new_balance as i64,
            timestamp_ms: timestamp_ms as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::insurance::CoveragePurchasedEvent
// NOTE: vault_id is not in the event but is needed. We'll need to query it from the policy
// or add it to the event in a future contract update.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoveragePurchasedEvent {
    pub policy_id: String,
    pub market_id: String,
    pub insured: String,
    pub option_id: u8,
    pub covered_amount: u64,
    pub coverage_bps: u64,
    pub premium_paid: u64,
    pub reserve_locked: u64,
    pub expiry_time_ms: u64,
    // vault_id is not in the contract event, but we need it
    // We'll query it from the database after policy creation or use a workaround
}

impl CoveragePurchasedEvent {
    pub fn into_policy_model(&self, start_time_ms: u64, vault_id: String, tx: String) -> Result<NewInsurancePolicy> {
        let now = Utc::now().naive_utc();
        Ok(NewInsurancePolicy {
            policy_id: self.policy_id.clone(),
            market_id: self.market_id.clone(),
            insured: self.insured.clone(),
            option_id: self.option_id as i16,
            covered_amount: self.covered_amount as i64,
            coverage_bps: self.coverage_bps as i64,
            premium_paid: self.premium_paid as i64,
            start_time_ms: start_time_ms as i64,
            expiry_time_ms: self.expiry_time_ms as i64,
            vault_id,
            status: 1, // STATUS_ACTIVE
            created_at: now,
            updated_at: now,
            transaction_id: tx,
        })
    }

    pub fn into_policy_event_model(
        &self,
        _vault_id: String,
        timestamp_ms: u64,
        tx: String,
    ) -> Result<NewInsurancePolicyEvent> {
        Ok(NewInsurancePolicyEvent {
            policy_id: self.policy_id.clone(),
            event_type: "PURCHASED".to_string(),
            market_id: self.market_id.clone(),
            insured: self.insured.clone(),
            option_id: self.option_id as i16,
            covered_amount: self.covered_amount as i64,
            coverage_bps: self.coverage_bps as i64,
            premium_paid: self.premium_paid as i64,
            reserve_locked: self.reserve_locked as i64,
            refunded_amount: None,
            fee_paid: None,
            payout: None,
            timestamp_ms: timestamp_ms as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }

    pub fn into_market_exposure_model(
        &self,
        vault_id: String,
        timestamp_ms: u64,
        tx: String,
    ) -> Result<NewInsuranceMarketExposure> {
        Ok(NewInsuranceMarketExposure {
            vault_id,
            market_id: self.market_id.clone(),
            option_id: self.option_id as i16,
            reserved_amount: self.reserve_locked as i64,
            timestamp_ms: timestamp_ms as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }

    pub fn into_user_exposure_model(
        &self,
        vault_id: String,
        timestamp_ms: u64,
        tx: String,
    ) -> Result<NewInsuranceUserExposure> {
        Ok(NewInsuranceUserExposure {
            vault_id,
            insured: self.insured.clone(),
            reserved_amount: self.reserve_locked as i64,
            timestamp_ms: timestamp_ms as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::insurance::CoverageCancelledEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageCancelledEvent {
    pub policy_id: String,
    pub insured: String,
    pub refunded_amount: u64,
    pub fee_paid: u64,
}

impl CoverageCancelledEvent {
    pub fn into_policy_event_model(
        &self,
        market_id: String,
        option_id: u8,
        covered_amount: u64,
        coverage_bps: u64,
        premium_paid: u64,
        reserve_locked: u64,
        _vault_id: String,
        timestamp_ms: u64,
        tx: String,
    ) -> Result<NewInsurancePolicyEvent> {
        Ok(NewInsurancePolicyEvent {
            policy_id: self.policy_id.clone(),
            event_type: "CANCELLED".to_string(),
            market_id,
            insured: self.insured.clone(),
            option_id: option_id as i16,
            covered_amount: covered_amount as i64,
            coverage_bps: coverage_bps as i64,
            premium_paid: premium_paid as i64,
            reserve_locked: reserve_locked as i64,
            refunded_amount: Some(self.refunded_amount as i64),
            fee_paid: Some(self.fee_paid as i64),
            payout: None,
            timestamp_ms: timestamp_ms as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::insurance::CoverageClaimedEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageClaimedEvent {
    pub policy_id: String,
    pub insured: String,
    pub payout: u64,
}

impl CoverageClaimedEvent {
    pub fn into_policy_event_model(
        &self,
        market_id: String,
        option_id: u8,
        covered_amount: u64,
        coverage_bps: u64,
        premium_paid: u64,
        reserve_locked: u64,
        _vault_id: String,
        timestamp_ms: u64,
        tx: String,
    ) -> Result<NewInsurancePolicyEvent> {
        Ok(NewInsurancePolicyEvent {
            policy_id: self.policy_id.clone(),
            event_type: "CLAIMED".to_string(),
            market_id,
            insured: self.insured.clone(),
            option_id: option_id as i16,
            covered_amount: covered_amount as i64,
            coverage_bps: coverage_bps as i64,
            premium_paid: premium_paid as i64,
            reserve_locked: reserve_locked as i64,
            refunded_amount: None,
            fee_paid: None,
            payout: Some(self.payout as i64),
            timestamp_ms: timestamp_ms as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::insurance::ConfigUpdatedEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdatedEvent {
    pub updated_by: String,
    pub enable_flag: bool,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub min_coverage_bps: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub max_coverage_bps: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub max_duration_ms: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub fee_bps: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

impl ConfigUpdatedEvent {
    /// Convert to database model using timestamp_ms from BlockchainEvent (in milliseconds)
    /// The database trigger will convert timestamp_ms to time: to_timestamp(timestamp_ms / 1000)
    pub fn into_config_model(&self, timestamp_ms: u64, tx: String) -> Result<NewInsuranceConfig> {
        Ok(NewInsuranceConfig {
            updated_by: self.updated_by.clone(),
            enable_flag: self.enable_flag,
            min_coverage_bps: self.min_coverage_bps as i64,
            max_coverage_bps: self.max_coverage_bps as i64,
            max_duration_ms: self.max_duration_ms as i64,
            fee_bps: self.fee_bps as i64,
            version: 1, // Increment version if needed, or keep same
            timestamp_ms: timestamp_ms as i64, // Use timestamp_ms from BlockchainEvent (milliseconds)
            time: Utc::now(), // Will be overridden by database trigger
            transaction_id: tx,
        })
    }
}

// Matches social_contracts::insurance::PolicyExpiredEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyExpiredEvent {
    pub policy_id: String,
    pub insured: String,
    pub market_id: String,
    pub vault_id: String,
    pub reserve_released: u64,
    pub expiry_time_ms: u64,
}

impl PolicyExpiredEvent {
    pub fn into_policy_event_model(
        &self,
        option_id: u8,
        covered_amount: u64,
        coverage_bps: u64,
        premium_paid: u64,
        timestamp_ms: u64,
        tx: String,
    ) -> Result<NewInsurancePolicyEvent> {
        Ok(NewInsurancePolicyEvent {
            policy_id: self.policy_id.clone(),
            event_type: "EXPIRED".to_string(),
            market_id: self.market_id.clone(),
            insured: self.insured.clone(),
            option_id: option_id as i16,
            covered_amount: covered_amount as i64,
            coverage_bps: coverage_bps as i64,
            premium_paid: premium_paid as i64,
            reserve_locked: self.reserve_released as i64,
            refunded_amount: None,
            fee_paid: None,
            payout: None,
            timestamp_ms: timestamp_ms as i64,
            time: Utc::now(),
            transaction_id: tx,
        })
    }
}

/// Helper function to create event log entry
pub fn new_insurance_event_log(
    event_type: &str,
    event_data: &Value,
    event_id: &str,
) -> NewInsuranceEventLog {
    NewInsuranceEventLog {
        event_type: event_type.to_string(),
        event_data: event_data.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
    }
}

// =============================================================================
// PROCESS FUNCTIONS FOR CHECKPOINT PROCESSOR
// =============================================================================

use anyhow::anyhow;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::social::db::DbConnection;
use crate::social::schema::{
    insurance_config, insurance_vaults, insurance_vault_transactions,
    insurance_policies, insurance_policy_events, insurance_events,
    insurance_market_exposures, insurance_user_exposures,
};

/// Process a ConfigInitializedEvent and insert into the database
pub async fn process_config_initialized_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    timestamp_ms: u64,
    tx: String,
) -> Result<()> {
    let event: ConfigInitializedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse ConfigInitializedEvent: {}", e))?;

    let config = event.into_config_model(timestamp_ms, tx.clone())?;

    diesel::insert_into(insurance_config::table)
        .values(&config)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance config: {}", e))?;

    // Log the event
    let log = new_insurance_event_log("ConfigInitializedEvent", data, event_id);
    diesel::insert_into(insurance_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance event log: {}", e))?;

    tracing::info!("Processed ConfigInitializedEvent by {}", event.admin);
    Ok(())
}

/// Process a ConfigUpdatedEvent and insert into the database
pub async fn process_config_updated_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    timestamp_ms: u64,
    tx: String,
) -> Result<()> {
    let event: ConfigUpdatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse ConfigUpdatedEvent: {}", e))?;

    let config = event.into_config_model(timestamp_ms, tx.clone())?;

    diesel::insert_into(insurance_config::table)
        .values(&config)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance config update: {}", e))?;

    // Log the event
    let log = new_insurance_event_log("ConfigUpdatedEvent", data, event_id);
    diesel::insert_into(insurance_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance event log: {}", e))?;

    tracing::info!("Processed ConfigUpdatedEvent by {}", event.updated_by);
    Ok(())
}

/// Process an UnderwriterVaultCreatedEvent and insert into the database
pub async fn process_vault_created_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    tx: String,
) -> Result<()> {
    let event: UnderwriterVaultCreatedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse UnderwriterVaultCreatedEvent: {}", e))?;

    let vault = event.into_vault_model(tx.clone())?;

    diesel::insert_into(insurance_vaults::table)
        .values(&vault)
        .on_conflict(insurance_vaults::vault_id)
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance vault: {}", e))?;

    // Log the event
    let log = new_insurance_event_log("UnderwriterVaultCreatedEvent", data, event_id);
    diesel::insert_into(insurance_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance event log: {}", e))?;

    tracing::info!("Processed UnderwriterVaultCreatedEvent: {} by {}", event.vault_id, event.underwriter);
    Ok(())
}

/// Process an UnderwriterVaultDepositedEvent and insert into the database
pub async fn process_vault_deposited_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    timestamp_ms: u64,
    tx: String,
) -> Result<()> {
    let event: UnderwriterVaultDepositedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse UnderwriterVaultDepositedEvent: {}", e))?;

    let transaction = event.into_transaction_model(timestamp_ms, tx.clone())?;

    // Insert transaction record
    diesel::insert_into(insurance_vault_transactions::table)
        .values(&transaction)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert vault transaction: {}", e))?;

    // Update vault balance
    diesel::update(insurance_vaults::table)
        .filter(insurance_vaults::vault_id.eq(&event.vault_id))
        .set((
            insurance_vaults::capital_balance.eq(event.new_balance as i64),
            insurance_vaults::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to update vault balance: {}", e))?;

    // Log the event
    let log = new_insurance_event_log("UnderwriterVaultDepositedEvent", data, event_id);
    diesel::insert_into(insurance_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance event log: {}", e))?;

    tracing::info!("Processed UnderwriterVaultDepositedEvent: {} deposited {} to vault {}",
        event.amount, event.vault_id, event.new_balance);
    Ok(())
}

/// Process an UnderwriterVaultWithdrawnEvent and insert into the database
pub async fn process_vault_withdrawn_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    timestamp_ms: u64,
    tx: String,
) -> Result<()> {
    let event: UnderwriterVaultWithdrawnEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse UnderwriterVaultWithdrawnEvent: {}", e))?;

    let transaction = event.into_transaction_model(timestamp_ms, tx.clone())?;

    // Insert transaction record
    diesel::insert_into(insurance_vault_transactions::table)
        .values(&transaction)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert vault transaction: {}", e))?;

    // Update vault balance
    diesel::update(insurance_vaults::table)
        .filter(insurance_vaults::vault_id.eq(&event.vault_id))
        .set((
            insurance_vaults::capital_balance.eq(event.new_balance as i64),
            insurance_vaults::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to update vault balance: {}", e))?;

    // Log the event
    let log = new_insurance_event_log("UnderwriterVaultWithdrawnEvent", data, event_id);
    diesel::insert_into(insurance_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance event log: {}", e))?;

    tracing::info!("Processed UnderwriterVaultWithdrawnEvent: {} withdrawn from vault {}",
        event.amount, event.vault_id);
    Ok(())
}

/// Process a CoveragePurchasedEvent and insert into the database
pub async fn process_coverage_purchased_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    timestamp_ms: u64,
    tx: String,
) -> Result<()> {
    let event: CoveragePurchasedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse CoveragePurchasedEvent: {}", e))?;

    // Use market_id as the vault identifier - in the insurance system, coverage is purchased
    // against a specific market which is backed by underwriter vaults
    let vault_id = data.get("vault_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| event.market_id.clone());

    // Insert policy
    let policy = event.into_policy_model(timestamp_ms, vault_id.clone(), tx.clone())?;
    diesel::insert_into(insurance_policies::table)
        .values(&policy)
        .on_conflict(insurance_policies::policy_id)
        .do_nothing()
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance policy: {}", e))?;

    // Insert policy event
    let policy_event = event.into_policy_event_model(vault_id.clone(), timestamp_ms, tx.clone())?;
    diesel::insert_into(insurance_policy_events::table)
        .values(&policy_event)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert policy event: {}", e))?;

    // Insert market exposure
    let market_exposure = event.into_market_exposure_model(vault_id.clone(), timestamp_ms, tx.clone())?;
    diesel::insert_into(insurance_market_exposures::table)
        .values(&market_exposure)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert market exposure: {}", e))?;

    // Insert user exposure
    let user_exposure = event.into_user_exposure_model(vault_id, timestamp_ms, tx.clone())?;
    diesel::insert_into(insurance_user_exposures::table)
        .values(&user_exposure)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert user exposure: {}", e))?;

    // Log the event
    let log = new_insurance_event_log("CoveragePurchasedEvent", data, event_id);
    diesel::insert_into(insurance_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance event log: {}", e))?;

    tracing::info!("Processed CoveragePurchasedEvent: policy {} for {} covering {}",
        event.policy_id, event.insured, event.covered_amount);
    Ok(())
}

/// Process a CoverageCancelledEvent and update the database
pub async fn process_coverage_cancelled_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    timestamp_ms: u64,
    tx: String,
) -> Result<()> {
    let event: CoverageCancelledEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse CoverageCancelledEvent: {}", e))?;

    // Update policy status to cancelled (status = 3)
    diesel::update(insurance_policies::table)
        .filter(insurance_policies::policy_id.eq(&event.policy_id))
        .set((
            insurance_policies::status.eq(3i16), // STATUS_CANCELLED
            insurance_policies::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to update policy status: {}", e))?;

    // Log the event
    let log = new_insurance_event_log("CoverageCancelledEvent", data, event_id);
    diesel::insert_into(insurance_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance event log: {}", e))?;

    tracing::info!("Processed CoverageCancelledEvent: policy {} cancelled, refunded {}",
        event.policy_id, event.refunded_amount);
    let _ = (timestamp_ms, tx); // Mark as used
    Ok(())
}

/// Process a CoverageClaimedEvent and update the database
pub async fn process_coverage_claimed_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    timestamp_ms: u64,
    tx: String,
) -> Result<()> {
    let event: CoverageClaimedEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse CoverageClaimedEvent: {}", e))?;

    // Update policy status to claimed (status = 2)
    diesel::update(insurance_policies::table)
        .filter(insurance_policies::policy_id.eq(&event.policy_id))
        .set((
            insurance_policies::status.eq(2i16), // STATUS_CLAIMED
            insurance_policies::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to update policy status: {}", e))?;

    // Log the event
    let log = new_insurance_event_log("CoverageClaimedEvent", data, event_id);
    diesel::insert_into(insurance_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance event log: {}", e))?;

    tracing::info!("Processed CoverageClaimedEvent: policy {} claimed, payout {}",
        event.policy_id, event.payout);
    let _ = (timestamp_ms, tx); // Mark as used
    Ok(())
}

/// Process a PolicyExpiredEvent and update the database
pub async fn process_policy_expired_event(
    conn: &mut DbConnection,
    data: &Value,
    event_id: &str,
    timestamp_ms: u64,
    tx: String,
) -> Result<()> {
    let event: PolicyExpiredEvent = serde_json::from_value(data.clone())
        .map_err(|e| anyhow!("Failed to parse PolicyExpiredEvent: {}", e))?;

    // Update policy status to expired (status = 4)
    diesel::update(insurance_policies::table)
        .filter(insurance_policies::policy_id.eq(&event.policy_id))
        .set((
            insurance_policies::status.eq(4i16), // STATUS_EXPIRED
            insurance_policies::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to update policy status: {}", e))?;

    // Log the event
    let log = new_insurance_event_log("PolicyExpiredEvent", data, event_id);
    diesel::insert_into(insurance_events::table)
        .values(&log)
        .execute(conn)
        .await
        .map_err(|e| anyhow!("Failed to insert insurance event log: {}", e))?;

    tracing::info!("Processed PolicyExpiredEvent: policy {} expired, released {}",
        event.policy_id, event.reserve_released);
    let _ = (timestamp_ms, tx); // Mark as used
    Ok(())
}

