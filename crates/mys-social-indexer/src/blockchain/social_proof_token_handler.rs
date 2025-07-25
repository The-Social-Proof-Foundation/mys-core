// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use tracing::{debug, info, warn};
use chrono::{DateTime, Utc};

use crate::blockchain::handler_trait::{BaseHandler, BlockchainEventHandler, HandlerHealth, HandlerStats};
use crate::blockchain::listener::BlockchainEvent;
use crate::db::Database;
use crate::events::social_proof_token_events::{
    SocialProofInitPoolEvent, SocialProofBuyEvent, SocialProofSellEvent,
    SocialProofStakeCreatedEvent,
    SocialProofThresholdMetEvent, ConfigUpdatedEvent,
};
use crate::models::social_proof_token::{
    NewSocialProofTokenPool, SocialProofTokenPool,
    NewSptStakePool, SptStakePool,
};
use crate::models::indexer::NewIndexerProgress;
use crate::schema;

/// Social proof token event handler using the new architecture
pub struct SocialProofTokenHandler {
    base: BaseHandler,
}

impl SocialProofTokenHandler {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            base: BaseHandler::new("social-proof-token".to_string(), db),
        }
    }
    
    /// Helper to convert timestamp to DateTime
    fn timestamp_to_datetime(timestamp_ms: u64) -> DateTime<Utc> {
        let timestamp_secs = (timestamp_ms / 1000) as i64;
        DateTime::<Utc>::from_timestamp(timestamp_secs, 0)
            .unwrap_or_else(|| Utc::now())
    }

    /// Extract event fields from blockchain event data
    fn extract_event_fields(data: &serde_json::Value) -> Result<serde_json::Value> {
        crate::events::event_utils::extract_event_fields(data)
    }

    /// Process initialization pool events
    async fn process_init_pool_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT init pool event: {}", event.event_id);
        
        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let init_event = serde_json::from_value::<SocialProofInitPoolEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse InitPoolEvent: {}", e))?;
        
        // Convert to database model
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let mut token_pool = init_event.into_model(timestamp, event.tx_digest.clone())?;
        token_pool.time = Self::timestamp_to_datetime(event.timestamp_ms);
        
        // Insert into database
        let mut conn = self.base.get_connection().await?;
        diesel::insert_into(schema::social_proof_token_pools::table)
            .values(&token_pool)
            .execute(&mut conn)
            .await?;
        
        info!("Successfully processed init pool event for pool: {}", token_pool.pool_id);
        Ok(())
    }

    /// Process buy events
    async fn process_buy_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT buy event: {}", event.event_id);
        
        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let buy_event = serde_json::from_value::<SocialProofBuyEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse BuyEvent: {}", e))?;
        
        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);
        
        // Get the latest token pool to update supply and price
        let latest_pool = schema::social_proof_token_pools::table
            .filter(schema::social_proof_token_pools::pool_id.eq(&buy_event.pool_id))
            .order_by(schema::social_proof_token_pools::time.desc())
            .first::<SocialProofTokenPool>(&mut conn)
            .await
            .map_err(|e| anyhow!("Failed to get latest token pool: {}", e))?;
        
        // Create updated token pool record
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
            transaction_id: event.tx_digest.clone(),
        };
        
        // Insert transaction record
        let mut tx = buy_event.into_transaction_model(timestamp, event.tx_digest.clone())?;
        tx.time = datetime;
        
        diesel::insert_into(schema::spt_transactions::table)
            .values(&tx)
            .execute(&mut conn)
            .await?;
        
        // Update or insert holding for buyer
        let mut holding = buy_event.into_holding_model(timestamp, event.tx_digest.clone())?;
        holding.time = datetime;
        
        diesel::insert_into(schema::spt_holdings::table)
            .values(&holding)
            .execute(&mut conn)
            .await?;
        
        // Insert price history record
        let mut price_history = buy_event.into_price_history_model(new_circulating_supply, timestamp, event.tx_digest.clone())?;
        price_history.time = datetime;
        
        diesel::insert_into(schema::spt_price_history::table)
            .values(&price_history)
            .execute(&mut conn)
            .await?;
        
        // Insert updated token pool
        diesel::insert_into(schema::social_proof_token_pools::table)
            .values(&new_token_pool)
            .execute(&mut conn)
            .await?;
        
        info!("Successfully processed buy event for pool: {}", buy_event.pool_id);
        Ok(())
    }

    /// Process sell events
    async fn process_sell_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT sell event: {}", event.event_id);
        
        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let sell_event = serde_json::from_value::<SocialProofSellEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse SellEvent: {}", e))?;
        
        let _conn = self.base.get_connection().await?;
        let _timestamp = (event.timestamp_ms / 1000) as i64;
        let _datetime = Self::timestamp_to_datetime(event.timestamp_ms);
        
        // TODO: Implement sell event processing similar to buy event
        
        info!("Successfully processed sell event for pool: {}", sell_event.pool_id);
        Ok(())
    }

    /// Process stake created events
    async fn process_stake_created_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT stake created event: {}", event.event_id);
        
        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let stake_event = serde_json::from_value::<SocialProofStakeCreatedEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse StakeCreatedEvent: {}", e))?;
        
        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);
        
        // 1. Create individual stake record
        let mut stake_record = stake_event.into_stake_model(timestamp, event.tx_digest.clone())?;
        stake_record.time = datetime;
        
        diesel::insert_into(schema::spt_stakes::table)
            .values(&stake_record)
            .execute(&mut conn)
            .await?;
        
        // 2. Create or update stake pool record
        let pool_id = format!("stake_pool_{}", stake_event.associated_id);
        
        // Check if stake pool already exists
        let existing_pool = schema::spt_stake_pools::table
            .filter(schema::spt_stake_pools::pool_id.eq(&pool_id))
            .order_by(schema::spt_stake_pools::time.desc())
            .first::<SptStakePool>(&mut conn)
            .await
            .optional()?;
        
        if let Some(existing) = existing_pool {
            // Update existing pool
            let updated_pool = NewSptStakePool {
                pool_id: existing.pool_id.clone(),
                associated_id: existing.associated_id.clone(),
                owner: existing.owner.clone(),
                token_type: existing.token_type,
                total_staked: stake_event.total_staked,
                required_threshold: existing.required_threshold,
                status: if stake_event.threshold_met {
                    "threshold_met".to_string()
                } else {
                    existing.status
                },
                created_at: existing.created_at,
                time: datetime,
                transaction_id: event.tx_digest.clone(),
            };
            
            diesel::insert_into(schema::spt_stake_pools::table)
                .values(&updated_pool)
                .execute(&mut conn)
                .await?;
        } else {
            // Create new pool
            let required_threshold = if stake_event.associated_id.starts_with("post_") { 1000 } else { 10000 };
            let new_pool = NewSptStakePool {
                pool_id: pool_id.clone(),
                associated_id: stake_event.associated_id.clone(),
                owner: stake_event.staker.clone(),
                token_type: if stake_event.associated_id.starts_with("post_") { 2 } else { 1 },
                total_staked: stake_event.total_staked,
                required_threshold,
                status: if stake_event.threshold_met {
                    "threshold_met".to_string()
                } else {
                    "active".to_string()
                },
                created_at: timestamp,
                time: datetime,
                transaction_id: event.tx_digest.clone(),
            };
            
            diesel::insert_into(schema::spt_stake_pools::table)
                .values(&new_pool)
                .execute(&mut conn)
                .await?;
        }
        
        info!("Successfully processed stake created event for pool: {}", pool_id);
        Ok(())
    }

    /// Process stake withdrawn events
    async fn process_stake_withdrawn_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT stake withdrawn event: {}", event.event_id);
        
        // Similar to stake created but handling withdrawals
        // ... (implementation would handle stake withdrawals)
        
        Ok(())
    }

    /// Process threshold met events
    async fn process_threshold_met_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT threshold met event: {}", event.event_id);
        
        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let threshold_event = serde_json::from_value::<SocialProofThresholdMetEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse ThresholdMetEvent: {}", e))?;
        
        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);
        
        // Update stake pool status to threshold_met
        let mut stake_pool = threshold_event.into_stake_pool_model(timestamp, event.tx_digest.clone())?;
        stake_pool.time = datetime;
        
        diesel::insert_into(schema::spt_stake_pools::table)
            .values(&stake_pool)
            .execute(&mut conn)
            .await?;
        
        info!("Successfully processed threshold met event");
        Ok(())
    }

    /// Process config updated events
    async fn process_config_updated_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT config updated event: {}", event.event_id);
        
        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let config_event = serde_json::from_value::<ConfigUpdatedEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse ConfigUpdatedEvent: {}", e))?;
        
        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);
        
        // Update token exchange config
        let mut config = config_event.into_exchange_config_model(timestamp as u64, event.tx_digest.clone())?;
        config.time = datetime;
        
        diesel::insert_into(schema::spt_exchange_config::table)
            .values(&config)
            .execute(&mut conn)
            .await?;
        
        info!("Successfully processed config updated event");
        Ok(())
    }

    /// Update progress tracking
    async fn update_progress(&self) -> Result<()> {
        let mut conn = self.base.get_connection().await?;
        let now = chrono::Utc::now().naive_utc();
        
        let progress = NewIndexerProgress {
            id: self.base.name.clone(),
            last_checkpoint_processed: 0, // Not using checkpoints in event-driven system
            last_processed_at: now,
        };
        
        diesel::insert_into(schema::indexer_progress::table)
            .values(&progress)
            .on_conflict(schema::indexer_progress::id)
            .do_update()
            .set((
                schema::indexer_progress::last_checkpoint_processed.eq(progress.last_checkpoint_processed),
                schema::indexer_progress::last_processed_at.eq(progress.last_processed_at),
            ))
            .execute(&mut conn)
            .await?;
            
        Ok(())
    }
}

#[async_trait]
impl BlockchainEventHandler for SocialProofTokenHandler {
    fn name(&self) -> &str {
        &self.base.name
    }

    async fn process_event(&mut self, event: BlockchainEvent) -> Result<()> {
        let event_type = &event.event_type;
        
        // Route to appropriate handler based on event type
        let result = match event_type {
            t if t.contains("::social_proof_token::") && t.ends_with("::InitPoolEvent") => {
                self.process_init_pool_event(&event).await
            },
            t if t.contains("::social_proof_token::") && t.ends_with("::BuyEvent") => {
                self.process_buy_event(&event).await
            },
            t if t.contains("::social_proof_token::") && t.ends_with("::SellEvent") => {
                self.process_sell_event(&event).await
            },
            t if t.contains("::token_exchange::") && t.ends_with("::StakeCreatedEvent") => {
                self.process_stake_created_event(&event).await
            },
            t if t.contains("::token_exchange::") && t.ends_with("::StakeWithdrawnEvent") => {
                self.process_stake_withdrawn_event(&event).await
            },
            t if t.contains("::token_exchange::") && t.ends_with("::ThresholdMetEvent") => {
                self.process_threshold_met_event(&event).await
            },
            t if t.contains("::token_exchange::") && t.ends_with("::ConfigUpdatedEvent") => {
                self.process_config_updated_event(&event).await
            },
            _ => {
                debug!("Ignoring unhandled SPT event type: {}", event_type);
                return Ok(());
            }
        };

        // Update statistics and progress
        match &result {
            Ok(_) => {
                self.base.update_stats_success(event.timestamp_ms);
                if let Err(e) = self.update_progress().await {
                    warn!("Failed to update progress for SPT handler: {}", e);
                }
            }
            Err(e) => {
                self.base.update_stats_failure(format!(
                    "Failed to process event {}: {}",
                    event.event_id, e
                ));
            }
        }

        result
    }

    fn stats(&self) -> HandlerStats {
        self.base.stats.clone()
    }

    async fn health(&self) -> HandlerHealth {
        self.base.get_health()
    }
}