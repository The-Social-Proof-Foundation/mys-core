// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use tracing::{debug, info};
use chrono::{DateTime, Utc};

use crate::db::Database;
use crate::blockchain::chain_indexer::{
    BlockchainHandler, HandlerOptions, MysEventWithMetadata
};
use crate::events::social_proof_token_events::{
    SocialProofInitPoolEvent, SocialProofBuyEvent, SocialProofSellEvent,
    SocialProofStakeCreatedEvent, SocialProofStakeWithdrawnEvent,
    SocialProofThresholdMetEvent, ConfigUpdatedEvent,
};
use crate::models::social_proof_token::{
    NewSocialProofTokenPool, SocialProofTokenPool,
    NewSptStakePool, SptStakePool,
};

// Social proof token event handler implementation
pub struct SocialProofTokenHandler {
    db: Arc<Database>,
}

impl SocialProofTokenHandler {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
    
    // Helper to convert timestamp to DateTime
    fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(timestamp, 0)
            .unwrap_or_else(|| Utc::now())
    }
}

#[async_trait]
impl BlockchainHandler for SocialProofTokenHandler {
    async fn handle_event(&self, event: MysEventWithMetadata, _options: &HandlerOptions) -> Result<()> {
        let event_type = &event.event.type_;
        
        match event_type.as_str() {
            "social_proof_token::token_pool::InitPoolEvent" => {
                self.handle_init_pool_event(event).await
            },
            "social_proof_token::token_pool::BuyEvent" => {
                self.handle_buy_event(event).await
            },
            "social_proof_token::token_pool::SellEvent" => {
                self.handle_sell_event(event).await
            },
            "token_exchange::StakeCreatedEvent" => {
                self.handle_stake_created_event(event).await
            },
            "token_exchange::StakeWithdrawnEvent" => {
                self.handle_stake_withdrawn_event(event).await
            },
            "token_exchange::ThresholdMetEvent" => {
                self.handle_threshold_met_event(event).await
            },
            "token_exchange::ConfigUpdatedEvent" => {
                self.handle_config_updated_event(event).await
            },
            "token_exchange::EmergencyKillSwitchEvent" => {
                self.handle_emergency_kill_switch_event(event).await
            },
            _ => {
                debug!("Ignoring unhandled event type: {}", event_type);
                Ok(())
            }
        }
    }
}

impl SocialProofTokenHandler {
    // Handle token pool initialization events
    async fn handle_init_pool_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling init pool event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse the event
        let init_event = SocialProofInitPoolEvent::try_from(event.contents.clone())?;
        
        // Process the event data into model
        let mut token_pool = init_event.into_model(timestamp, transaction_id.clone())?;
        
        // Update with DateTime
        token_pool.time = datetime;
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // Insert the token pool
        diesel::insert_into(crate::schema::social_proof_token_pools::table)
            .values(&token_pool)
            .execute(&mut conn)
            .await?;
        
        info!("Processed init pool event for pool ID: {}", token_pool.pool_id);
        Ok(())
    }
    
    // Handle token buy events
    async fn handle_buy_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling buy event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse the event
        let buy_event = SocialProofBuyEvent::try_from(event.contents.clone())?;
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // 1. Get the latest token pool to update supply and price
        let latest_pool = crate::schema::social_proof_token_pools::table
            .filter(crate::schema::social_proof_token_pools::pool_id.eq(&buy_event.pool_id))
            .order_by(crate::schema::social_proof_token_pools::time.desc())
            .first::<SocialProofTokenPool>(&mut conn)
            .await
            .map_err(|e| anyhow!("Failed to get latest token pool: {}", e))?;
        
        // 2. Create updated token pool record
        let new_circulating_supply = latest_pool.circulating_supply + buy_event.amount;
        let new_token_pool = NewSocialProofTokenPool {
            pool_id: latest_pool.pool_id.clone(),
            owner: latest_pool.owner.clone(),
            name: latest_pool.name.clone(),
            symbol: latest_pool.symbol.clone(),
            token_type: latest_pool.token_type,
            associated_id: latest_pool.associated_id.clone(),
            base_price: latest_pool.base_price,
            quadratic_coefficient: latest_pool.quadratic_coefficient,
            circulating_supply: new_circulating_supply,
            created_at: latest_pool.created_at,
            time: datetime,
            transaction_id: transaction_id.clone(),
        };
        
        // 3. Insert transaction record
        let mut tx = buy_event.into_transaction_model(timestamp, transaction_id.clone())?;
        tx.time = datetime;
        
        diesel::insert_into(crate::schema::spt_transactions::table)
            .values(&tx)
            .execute(&mut conn)
            .await?;
        
        // 4. Update or insert holding for buyer
        let mut holding = buy_event.into_holding_model(timestamp, transaction_id.clone())?;
        holding.time = datetime;
        
        diesel::insert_into(crate::schema::spt_holdings::table)
            .values(&holding)
            .execute(&mut conn)
            .await?;
        
        // 5. Insert price history record
        let mut price_history = buy_event.into_price_history_model(new_circulating_supply, timestamp, transaction_id.clone())?;
        price_history.time = datetime;
        
        diesel::insert_into(crate::schema::spt_price_history::table)
            .values(&price_history)
            .execute(&mut conn)
            .await?;
        
        // 6. Insert updated token pool
        diesel::insert_into(crate::schema::social_proof_token_pools::table)
            .values(&new_token_pool)
            .execute(&mut conn)
            .await?;
        
        // 7. Create SPT revenue record for swap fees
        if buy_event.creator_fee > 0 || buy_event.platform_fee > 0 || buy_event.treasury_fee > 0 {
            // Use the latest pool to get creator and platform addresses
            let creator_address = latest_pool.owner.clone();
            // For platform address, use a default or get from config - for now use a placeholder
            let platform_address = "platform_address".to_string(); // TODO: Get from config
            let treasury_address = "treasury_address".to_string(); // TODO: Get from config
            
            let spt_revenue = buy_event.create_spt_revenue(
                creator_address.clone(),
                platform_address.clone(),
                treasury_address.clone(),
                timestamp,
                transaction_id.clone(),
            )?;
            
            diesel::insert_into(crate::schema::spt_revenue::table)
                .values(&spt_revenue)
                .execute(&mut conn)
                .await?;
            
            // 8. Create unified revenue records for each fee type
            let unified_revenue_records = buy_event.create_unified_revenue_records(
                creator_address,
                platform_address,
                treasury_address,
                timestamp,
                transaction_id.clone(),
            )?;
            
            for record in unified_revenue_records {
                diesel::insert_into(crate::schema::unified_revenue::table)
                    .values(&record)
                    .execute(&mut conn)
                    .await?;
            }
        }
        
        info!("Processed buy event with revenue tracking for pool ID: {}", buy_event.pool_id);
        Ok(())
    }
    
    // Handle token sell events
    async fn handle_sell_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling sell event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse the event
        let sell_event = SocialProofSellEvent::try_from(event.contents.clone())?;
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // 1. Get the latest token pool to update supply and price
        let latest_pool = crate::schema::social_proof_token_pools::table
            .filter(crate::schema::social_proof_token_pools::pool_id.eq(&sell_event.pool_id))
            .order_by(crate::schema::social_proof_token_pools::time.desc())
            .first::<SocialProofTokenPool>(&mut conn)
            .await
            .map_err(|e| anyhow!("Failed to get latest token pool: {}", e))?;
        
        // 2. Create updated token pool record
        let new_circulating_supply = latest_pool.circulating_supply - sell_event.amount;
        let new_token_pool = NewSocialProofTokenPool {
            pool_id: latest_pool.pool_id.clone(),
            owner: latest_pool.owner.clone(),
            name: latest_pool.name.clone(),
            symbol: latest_pool.symbol.clone(),
            token_type: latest_pool.token_type,
            associated_id: latest_pool.associated_id.clone(),
            base_price: latest_pool.base_price,
            quadratic_coefficient: latest_pool.quadratic_coefficient,
            circulating_supply: new_circulating_supply,
            created_at: latest_pool.created_at,
            time: datetime,
            transaction_id: transaction_id.clone(),
        };
        
        // 3. Insert transaction record
        let mut tx = sell_event.into_transaction_model(timestamp, transaction_id.clone())?;
        tx.time = datetime;
        
        diesel::insert_into(crate::schema::spt_transactions::table)
            .values(&tx)
            .execute(&mut conn)
            .await?;
        
        // 4. Update or insert holding for seller
        let mut holding = sell_event.into_holding_model(timestamp, transaction_id.clone())?;
        holding.time = datetime;
        
        diesel::insert_into(crate::schema::spt_holdings::table)
            .values(&holding)
            .execute(&mut conn)
            .await?;
        
        // 5. Insert price history record
        let mut price_history = sell_event.into_price_history_model(new_circulating_supply, timestamp, transaction_id.clone())?;
        price_history.time = datetime;
        
        diesel::insert_into(crate::schema::spt_price_history::table)
            .values(&price_history)
            .execute(&mut conn)
            .await?;
        
        // 6. Insert updated token pool
        diesel::insert_into(crate::schema::social_proof_token_pools::table)
            .values(&new_token_pool)
            .execute(&mut conn)
            .await?;
        
        // 7. Create SPT revenue record for swap fees
        if sell_event.creator_fee > 0 || sell_event.platform_fee > 0 || sell_event.treasury_fee > 0 {
            // Use the latest pool to get creator and platform addresses
            let creator_address = latest_pool.owner.clone();
            // For platform address, use a default or get from config - for now use a placeholder
            let platform_address = "platform_address".to_string(); // TODO: Get from config
            let treasury_address = "treasury_address".to_string(); // TODO: Get from config
            
            let spt_revenue = sell_event.create_spt_revenue(
                creator_address.clone(),
                platform_address.clone(),
                treasury_address.clone(),
                timestamp,
                transaction_id.clone(),
            )?;
            
            diesel::insert_into(crate::schema::spt_revenue::table)
                .values(&spt_revenue)
                .execute(&mut conn)
                .await?;
            
            // 8. Create unified revenue records for each fee type
            let unified_revenue_records = sell_event.create_unified_revenue_records(
                creator_address,
                platform_address,
                treasury_address,
                timestamp,
                transaction_id.clone(),
            )?;
            
            for record in unified_revenue_records {
                diesel::insert_into(crate::schema::unified_revenue::table)
                    .values(&record)
                    .execute(&mut conn)
                    .await?;
            }
        }
        
        info!("Processed sell event with revenue tracking for pool ID: {}", sell_event.pool_id);
        Ok(())
    }
    
    // Handle stake created events
    async fn handle_stake_created_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling stake created event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse the event
        let stake_event = SocialProofStakeCreatedEvent::try_from(event.contents.clone())?;
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // 1. Create or update stake record
        let mut stake_record = stake_event.into_stake_model(timestamp, transaction_id.clone())?;
        stake_record.time = datetime;
        
        diesel::insert_into(crate::schema::spt_stakes::table)
            .values(&stake_record)
            .execute(&mut conn)
            .await?;
        
        // 2. Create or update stake pool record
        let pool_id = format!("stake_pool_{}", stake_event.associated_id);
        
        // Check if stake pool already exists
        let existing_pool = crate::schema::spt_stake_pools::table
            .filter(crate::schema::spt_stake_pools::pool_id.eq(&pool_id))
            .order_by(crate::schema::spt_stake_pools::time.desc())
            .first::<SptStakePool>(&mut conn)
            .await
            .optional()?;
        
        let stake_pool = if let Some(existing) = existing_pool {
            // Update existing pool
            NewSptStakePool {
                pool_id: existing.pool_id,
                associated_id: existing.associated_id,
                token_type: existing.token_type,
                owner: existing.owner,
                total_staked: stake_event.total_staked,
                required_threshold: existing.required_threshold,
                status: if stake_event.threshold_met {
                    "threshold_met".to_string()
                } else {
                    existing.status
                },
                created_at: existing.created_at,
                time: datetime,
                transaction_id: transaction_id.clone(),
            }
        } else {
            // Create new pool - we'll need to determine threshold from config
            let required_threshold = if stake_event.token_type == 1 { 10_000_000_000_000 } else { 1_000_000_000_000 }; // Default thresholds
            
            NewSptStakePool {
                pool_id: pool_id.clone(),
                associated_id: stake_event.associated_id.clone(),
                token_type: stake_event.token_type,
                owner: stake_event.staker.clone(), // Temporary, should be actual owner
                total_staked: stake_event.total_staked,
                required_threshold,
                status: if stake_event.threshold_met {
                    "threshold_met".to_string()
                } else {
                    "active".to_string()
                },
                created_at: stake_event.staked_at,
                time: datetime,
                transaction_id: transaction_id.clone(),
            }
        };
        
        diesel::insert_into(crate::schema::spt_stake_pools::table)
            .values(&stake_pool)
            .execute(&mut conn)
            .await?;
        
        info!("Processed stake created event for pool ID: {}", pool_id);
        Ok(())
    }
    
    // Handle stake withdrawn events  
    async fn handle_stake_withdrawn_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling stake withdrawn event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse the event
        let withdraw_event = SocialProofStakeWithdrawnEvent::try_from(event.contents.clone())?;
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // 1. Create withdrawal record (amount = 0 indicates full withdrawal)
        let mut stake_record = withdraw_event.into_stake_model(timestamp, transaction_id.clone())?;
        stake_record.time = datetime;
        
        diesel::insert_into(crate::schema::spt_stakes::table)
            .values(&stake_record)
            .execute(&mut conn)
            .await?;
        
        // 2. Update stake pool with new total
        let pool_id = format!("stake_pool_{}", withdraw_event.associated_id);
        
        let existing_pool = crate::schema::spt_stake_pools::table
            .filter(crate::schema::spt_stake_pools::pool_id.eq(&pool_id))
            .order_by(crate::schema::spt_stake_pools::time.desc())
            .first::<SptStakePool>(&mut conn)
            .await
            .map_err(|e| anyhow!("Failed to find stake pool: {}", e))?;
        
        let updated_stake_pool = NewSptStakePool {
            pool_id: existing_pool.pool_id,
            associated_id: existing_pool.associated_id,
            token_type: existing_pool.token_type,
            owner: existing_pool.owner,
            total_staked: withdraw_event.total_staked,
            required_threshold: existing_pool.required_threshold,
            status: if withdraw_event.total_staked >= existing_pool.required_threshold {
                "threshold_met".to_string()
            } else {
                "active".to_string()
            },
            created_at: existing_pool.created_at,
            time: datetime,
            transaction_id: transaction_id.clone(),
        };
        
        diesel::insert_into(crate::schema::spt_stake_pools::table)
            .values(&updated_stake_pool)
            .execute(&mut conn)
            .await?;
        
        info!("Processed stake withdrawn event for pool ID: {}", pool_id);
        Ok(())
    }
    
    // Handle threshold met events
    async fn handle_threshold_met_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling threshold met event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse the event
        let threshold_event = SocialProofThresholdMetEvent::try_from(event.contents.clone())?;
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // Update stake pool status to threshold_met
        let mut stake_pool = threshold_event.into_stake_pool_model(timestamp, transaction_id.clone())?;
        stake_pool.time = datetime;
        
        diesel::insert_into(crate::schema::spt_stake_pools::table)
            .values(&stake_pool)
            .execute(&mut conn)
            .await?;
        
        info!("Processed threshold met event for pool ID: {}", stake_pool.pool_id);
        Ok(())
    }
    
    // Handle exchange config updated events
    async fn handle_config_updated_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling config updated event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse the event
        let config_event = ConfigUpdatedEvent::try_from(event.contents.clone())?;
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // Create exchange config record
        let mut exchange_config = config_event.into_exchange_config_model(timestamp as u64, transaction_id.clone())?;
        exchange_config.time = datetime;
        
        diesel::insert_into(crate::schema::spt_exchange_config::table)
            .values(&exchange_config)
            .execute(&mut conn)
            .await?;
        
        info!("Processed config updated event by: {}", config_event.updated_by);
        Ok(())
    }
    
    // Handle emergency kill switch events
    async fn handle_emergency_kill_switch_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling emergency kill switch event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse and validate the event data with enhanced error handling
        let event_data = &event.contents;
        
        // Extract required fields with detailed error messages
        let admin = event_data["admin"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing admin field in kill switch event {}", transaction_id))?;
        let trading_halted = event_data["trading_halted"]
            .as_bool()
            .ok_or_else(|| anyhow!("Missing trading_halted field in kill switch event {}", transaction_id))?;
        let reason = event_data["reason"]
            .as_str()
            .unwrap_or("No reason provided");
        let event_timestamp = event_data["timestamp"]
            .as_u64()
            .unwrap_or(timestamp as u64) as i64;
            
        // Enhanced validation
        if admin.is_empty() {
            return Err(anyhow!("Admin address cannot be empty in kill switch event {}", transaction_id));
        }
        
        if admin.len() > 255 {
            return Err(anyhow!("Admin address too long (max 255 characters) in event {}", transaction_id));
        }
        
        if reason.len() > 512 {
            return Err(anyhow!("Reason text too long (max 512 characters) in event {}", transaction_id));
        }
        
        if transaction_id.len() > 255 {
            return Err(anyhow!("Transaction ID too long (max 255 characters): {}", transaction_id));
        }
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // Clone transaction_id for error handler (before it gets moved into closure)
        let tx_id_clone = transaction_id.clone();
        
        // Use serializable transaction to ensure atomicity and prevent race conditions
        conn.build_transaction()
            .serializable()
            .run(|tx_conn| {
                Box::pin(async move {
                    // Check if this is a duplicate event by checking the transaction_id
                    let existing_count = diesel::sql_query(
                        "SELECT COUNT(*) as count FROM token_exchange_config 
                         WHERE transaction_id = $1"
                    )
                    .bind::<diesel::sql_types::Text, _>(&transaction_id)
                    .get_result::<crate::db::query_types::CountResult>(tx_conn)
                    .await?
                    .count;
                    
                    if existing_count > 0 {
                        info!("Duplicate kill switch event detected for transaction {}, skipping", transaction_id);
                        return Ok(());
                    }
                    
                    // Check if this is a duplicate based on admin, timestamp, and state
                    let duplicate_check = diesel::sql_query(
                        "SELECT COUNT(*) as count FROM token_exchange_config 
                         WHERE admin_address = $1 AND timestamp_ms = $2 AND trading_halted = $3"
                    )
                    .bind::<diesel::sql_types::Text, _>(admin)
                    .bind::<diesel::sql_types::BigInt, _>(event_timestamp)
                    .bind::<diesel::sql_types::Bool, _>(trading_halted)
                    .get_result::<crate::db::query_types::CountResult>(tx_conn)
                    .await?
                    .count;
                    
                    if duplicate_check > 0 {
                        info!("Duplicate kill switch state change detected, skipping");
                        return Ok(());
                    }
                    
                    // Create new config entry
                    let new_config = crate::models::token_exchange::NewTokenExchangeConfig {
                        trading_halted,
                        admin_address: admin.to_string(),
                        reason: reason.to_string(),
                        timestamp_ms: event_timestamp,
                        updated_at: datetime,
                        transaction_id: transaction_id.clone(),
                    };
                    
                    // Insert config record with proper error handling
                    match diesel::insert_into(crate::schema::token_exchange_config::table)
                        .values(&new_config)
                        .execute(tx_conn)
                        .await {
                        Ok(rows_affected) => {
                            if rows_affected != 1 {
                                return Err(diesel::result::Error::RollbackTransaction);
                            }
                        }
                        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
                            info!("Kill switch config already exists for transaction {}, skipping", transaction_id);
                            return Ok(());
                        }
                        Err(e) => return Err(e),
                    }
                    
                    // Insert event history record with proper error handling
                    match diesel::sql_query(
                        "INSERT INTO token_exchange_events (event_type, event_data, event_id, created_at) 
                         VALUES ($1, $2, $3, $4)"
                    )
                    .bind::<diesel::sql_types::Text, _>("EmergencyKillSwitchEvent")
                    .bind::<diesel::sql_types::Jsonb, _>(&event.contents)
                    .bind::<diesel::sql_types::Text, _>(&transaction_id)
                    .bind::<diesel::sql_types::Timestamptz, _>(datetime)
                    .execute(tx_conn)
                    .await {
                        Ok(_) => {},
                        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
                            info!("Kill switch event already exists for transaction {}, skipping event record", transaction_id);
                        }
                        Err(e) => {
                            // Log but don't fail the transaction for event history
                            tracing::warn!("Failed to insert event history for {}: {}", transaction_id, e);
                        }
                    }
                    
                    info!("Successfully processed emergency kill switch event: trading_halted={}, admin={}, reason=\"{}\"", 
                          trading_halted, admin, reason);
                    
                    Ok::<_, diesel::result::Error>(())
                })
            })
            .await
            .map_err(|e| anyhow!("Transaction failed for kill switch event {}: {}", tx_id_clone, e))?;
        
        Ok(())
    }
} 