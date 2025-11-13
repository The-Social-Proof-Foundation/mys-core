// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use tracing::{info, warn};

use crate::blockchain::handler_trait::{
    BaseHandler, BlockchainEventHandler, HandlerHealth, HandlerStats,
};
use crate::blockchain::listener::BlockchainEvent;
use crate::db::Database;
use crate::events::social_proof_token_events::{
    ConfigUpdatedEvent, EmergencyKillSwitchEvent, PocRedirectionUpdatedEvent,
    PostPoolAutoInitializedEvent, ReservationPoolCreatedEvent, SocialProofBuyEvent,
    SocialProofInitPoolEvent, SocialProofReservationCreatedEvent, SocialProofReservationWithdrawnEvent,
    SocialProofSellEvent, SocialProofThresholdMetEvent, TokenBoughtEvent, TokenSoldEvent,
    TokensAddedEvent, TokenPoolCreatedEvent,
};
use crate::models::indexer::NewIndexerProgress;
use crate::models::social_proof_token::{
    NewSocialProofTokenPool, NewSptReservationPool, SocialProofTokenPool, SptReservationPool,
};
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
        DateTime::<Utc>::from_timestamp(timestamp_secs, 0).unwrap_or_else(|| Utc::now())
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

        // Update progress tracking
        self.update_progress().await?;

        info!(
            "Successfully processed init pool event for pool: {}, owner: {}, token_type: {}, supply: {}",
            token_pool.pool_id, token_pool.owner, token_pool.token_type, token_pool.circulating_supply
        );
        Ok(())
    }

    /// Process token bought events (from smart contract)
    async fn process_token_bought_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT token bought event: {}", event.event_id);

        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let buy_event = serde_json::from_value::<TokenBoughtEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse TokenBoughtEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        // Get the latest token pool to update supply and price
        let latest_pool = schema::social_proof_token_pools::table
            .filter(schema::social_proof_token_pools::pool_id.eq(&buy_event.id))
            .order_by(schema::social_proof_token_pools::time.desc())
            .first::<SocialProofTokenPool>(&mut conn)
            .await
            .map_err(|e| anyhow!("Failed to get latest token pool: {}", e))?;

        // Create updated token pool record
        let new_circulating_supply = latest_pool.circulating_supply + buy_event.amount as i64;
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
        let mut tx = buy_event.into_transaction_model(timestamp as u64, event.tx_digest.clone())?;
        tx.time = datetime;

        diesel::insert_into(schema::spt_transactions::table)
            .values(&tx)
            .execute(&mut conn)
            .await?;

        // Update or insert holding for buyer
        let mut holding = buy_event.into_holding_model(timestamp as u64, event.tx_digest.clone())?;
        holding.time = datetime;

        diesel::insert_into(schema::spt_holdings::table)
            .values(&holding)
            .execute(&mut conn)
            .await?;

        // Insert price history record
        let mut price_history = buy_event.create_price_history(
            new_circulating_supply,
            timestamp as u64,
            event.tx_digest.clone(),
        )?;
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

        // Update progress tracking
        self.update_progress().await?;

        info!(
            "Successfully processed token bought event for pool: {}, buyer: {}, amount: {}, new supply: {}",
            buy_event.id, buy_event.buyer, buy_event.amount, new_circulating_supply
        );
        Ok(())
    }

    /// Process token sold events (from smart contract)
    async fn process_token_sold_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT token sold event: {}", event.event_id);

        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let sell_event = serde_json::from_value::<TokenSoldEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse TokenSoldEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        // Get the latest token pool to update supply and price
        let latest_pool = schema::social_proof_token_pools::table
            .filter(schema::social_proof_token_pools::pool_id.eq(&sell_event.id))
            .order_by(schema::social_proof_token_pools::time.desc())
            .first::<SocialProofTokenPool>(&mut conn)
            .await
            .map_err(|e| anyhow!("Failed to get latest token pool for sell event: {}", e))?;

        // Calculate new circulating supply (reduce by sold amount)
        let new_circulating_supply = latest_pool
            .circulating_supply
            .saturating_sub(sell_event.amount as i64);

        // Create updated token pool record
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
        let mut tx = sell_event.into_transaction_model(timestamp as u64, event.tx_digest.clone())?;
        tx.time = datetime;

        diesel::insert_into(schema::spt_transactions::table)
            .values(&tx)
            .execute(&mut conn)
            .await?;

        // Update holding for seller (reduce their holding amount)
        let mut holding = sell_event.into_holding_model(timestamp as u64, event.tx_digest.clone())?;
        holding.time = datetime;

        diesel::insert_into(schema::spt_holdings::table)
            .values(&holding)
            .execute(&mut conn)
            .await?;

        // Insert price history record with new supply
        let mut price_history = sell_event.create_price_history(
            new_circulating_supply,
            timestamp as u64,
            event.tx_digest.clone(),
        )?;
        price_history.time = datetime;

        diesel::insert_into(schema::spt_price_history::table)
            .values(&price_history)
            .execute(&mut conn)
            .await?;

        // Insert updated token pool with reduced supply
        diesel::insert_into(schema::social_proof_token_pools::table)
            .values(&new_token_pool)
            .execute(&mut conn)
            .await?;

        // Update progress tracking
        self.update_progress().await?;

        info!(
            "Successfully processed token sold event for pool: {}, seller: {}, amount: {}, new supply: {}",
            sell_event.id, sell_event.seller, sell_event.amount, new_circulating_supply
        );
        Ok(())
    }

    /// Process buy events (legacy format)
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
        let mut price_history = buy_event.into_price_history_model(
            new_circulating_supply,
            timestamp,
            event.tx_digest.clone(),
        )?;
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

        // Update progress tracking
        self.update_progress().await?;

        info!(
            "Successfully processed buy event for pool: {}, buyer: {}, amount: {}, new supply: {}",
            buy_event.pool_id, buy_event.buyer, buy_event.amount, new_circulating_supply
        );
        Ok(())
    }

    /// Process sell events
    async fn process_sell_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT sell event: {}", event.event_id);

        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let sell_event = serde_json::from_value::<SocialProofSellEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse SellEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        // Get the latest token pool to update supply and price
        let latest_pool = schema::social_proof_token_pools::table
            .filter(schema::social_proof_token_pools::pool_id.eq(&sell_event.pool_id))
            .order_by(schema::social_proof_token_pools::time.desc())
            .first::<SocialProofTokenPool>(&mut conn)
            .await
            .map_err(|e| anyhow!("Failed to get latest token pool for sell event: {}", e))?;

        // Calculate new circulating supply (reduce by sold amount)
        let new_circulating_supply = latest_pool
            .circulating_supply
            .saturating_sub(sell_event.amount);

        // Create updated token pool record
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

        // Insert transaction record (sell transaction with negative amount or specific sell type)
        let mut tx = sell_event.into_transaction_model(timestamp, event.tx_digest.clone())?;
        tx.time = datetime;

        diesel::insert_into(schema::spt_transactions::table)
            .values(&tx)
            .execute(&mut conn)
            .await?;

        // Update holding for seller (reduce their holding amount)
        let mut holding = sell_event.into_holding_model(timestamp, event.tx_digest.clone())?;
        holding.time = datetime;

        diesel::insert_into(schema::spt_holdings::table)
            .values(&holding)
            .execute(&mut conn)
            .await?;

        // Insert price history record with new supply
        let mut price_history = sell_event.into_price_history_model(
            new_circulating_supply,
            timestamp,
            event.tx_digest.clone(),
        )?;
        price_history.time = datetime;

        diesel::insert_into(schema::spt_price_history::table)
            .values(&price_history)
            .execute(&mut conn)
            .await?;

        // Insert updated token pool with reduced supply
        diesel::insert_into(schema::social_proof_token_pools::table)
            .values(&new_token_pool)
            .execute(&mut conn)
            .await?;

        // Update progress tracking
        self.update_progress().await?;

        info!(
            "Successfully processed sell event for pool: {}, seller: {}, amount: {}, new supply: {}",
            sell_event.pool_id, sell_event.seller, sell_event.amount, new_circulating_supply
        );
        Ok(())
    }

    /// Process reservation created events
    async fn process_reservation_created_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!(
            "Processing SPT reservation created event: {}",
            event.event_id
        );

        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let reservation_event =
            serde_json::from_value::<SocialProofReservationCreatedEvent>(fields)
                .map_err(|e| anyhow!("Failed to parse ReservationCreatedEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        // 1. Create individual reservation record
        let mut reservation_record =
            reservation_event.into_reservation_model(timestamp, event.tx_digest.clone())?;
        reservation_record.time = datetime;

        diesel::insert_into(schema::spt_reservations::table)
            .values(&reservation_record)
            .execute(&mut conn)
            .await?;

        // 2. Create or update reservation pool record
        let pool_id = format!("reservation_pool_{}", reservation_event.associated_id);

        // Check if reservation pool already exists
        let existing_pool = schema::spt_reservation_pools::table
            .filter(schema::spt_reservation_pools::pool_id.eq(&pool_id))
            .order_by(schema::spt_reservation_pools::time.desc())
            .first::<SptReservationPool>(&mut conn)
            .await
            .optional()?;

        if let Some(existing) = existing_pool {
            // Update existing pool
            let updated_pool = NewSptReservationPool {
                pool_id: existing.pool_id.clone(),
                associated_id: existing.associated_id.clone(),
                owner: existing.owner.clone(),
                token_type: existing.token_type,
                total_reserved: reservation_event.total_reserved,
                required_threshold: existing.required_threshold,
                status: if reservation_event.threshold_met {
                    "threshold_met".to_string()
                } else {
                    existing.status
                },
                created_at: existing.created_at,
                time: datetime,
                transaction_id: event.tx_digest.clone(),
            };

            diesel::insert_into(schema::spt_reservation_pools::table)
                .values(&updated_pool)
                .execute(&mut conn)
                .await?;
        } else {
            // Create new pool
            let required_threshold = if reservation_event.associated_id.starts_with("post_") {
                1000
            } else {
                10000
            };
            let new_pool = NewSptReservationPool {
                pool_id: pool_id.clone(),
                associated_id: reservation_event.associated_id.clone(),
                owner: reservation_event.reserver.clone(),
                token_type: if reservation_event.associated_id.starts_with("post_") {
                    2
                } else {
                    1
                },
                total_reserved: reservation_event.total_reserved,
                required_threshold,
                status: if reservation_event.threshold_met {
                    "threshold_met".to_string()
                } else {
                    "active".to_string()
                },
                created_at: timestamp,
                time: datetime,
                transaction_id: event.tx_digest.clone(),
            };

            diesel::insert_into(schema::spt_reservation_pools::table)
                .values(&new_pool)
                .execute(&mut conn)
                .await?;
        }

        // Update progress tracking
        self.update_progress().await?;

        info!(
            "Successfully processed reservation created event for pool: {}",
            pool_id
        );
        Ok(())
    }

    /// Process reservation withdrawn events
    async fn process_reservation_withdrawn_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!(
            "Processing SPT reservation withdrawn event: {}",
            event.event_id
        );

        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let withdrawal_event =
            serde_json::from_value::<SocialProofReservationWithdrawnEvent>(fields)
                .map_err(|e| anyhow!("Failed to parse ReservationWithdrawnEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        // 1. Record the withdrawal as a new reservation entry with withdrawn amount
        let mut withdrawal_record =
            withdrawal_event.into_reservation_model(timestamp, event.tx_digest.clone())?;
        withdrawal_record.time = datetime;

        diesel::insert_into(schema::spt_reservations::table)
            .values(&withdrawal_record)
            .execute(&mut conn)
            .await?;

        // 2. Update the reservation pool to reflect the new total_reserved amount
        let pool_id = format!("reservation_pool_{}", withdrawal_event.associated_id);

        // Check if reservation pool exists
        let existing_pool = schema::spt_reservation_pools::table
            .filter(schema::spt_reservation_pools::pool_id.eq(&pool_id))
            .order_by(schema::spt_reservation_pools::time.desc())
            .first::<SptReservationPool>(&mut conn)
            .await
            .optional()?;

        if let Some(existing) = existing_pool {
            // Create updated pool record with new total_reserved amount
            let updated_pool = NewSptReservationPool {
                pool_id: existing.pool_id.clone(),
                associated_id: existing.associated_id.clone(),
                owner: existing.owner.clone(),
                token_type: existing.token_type,
                total_reserved: withdrawal_event.total_reserved,
                required_threshold: existing.required_threshold,
                status: if withdrawal_event.total_reserved >= existing.required_threshold {
                    existing.status // Keep existing status if still above threshold
                } else {
                    "active".to_string() // Reset to active if below threshold after withdrawal
                },
                created_at: existing.created_at,
                time: datetime,
                transaction_id: event.tx_digest.clone(),
            };

            diesel::insert_into(schema::spt_reservation_pools::table)
                .values(&updated_pool)
                .execute(&mut conn)
                .await?;

            info!(
                "Successfully processed reservation withdrawal for pool: {}, reserver: {}, amount: {}, remaining total: {}",
                pool_id, withdrawal_event.reserver, withdrawal_event.amount, withdrawal_event.total_reserved
            );
        } else {
            warn!(
                "Reservation pool not found for withdrawal event: {}, pool_id: {}",
                event.event_id, pool_id
            );

            // This shouldn't happen in normal operation, but we'll create a minimal pool record
            // to maintain data consistency
            let required_threshold = if withdrawal_event.associated_id.starts_with("post_") {
                1000
            } else {
                10000
            };

            let minimal_pool = NewSptReservationPool {
                pool_id: pool_id.clone(),
                associated_id: withdrawal_event.associated_id.clone(),
                owner: "unknown".to_string(), // We don't have this info from the withdrawal event
                token_type: if withdrawal_event.associated_id.starts_with("post_") {
                    2
                } else {
                    1
                },
                total_reserved: withdrawal_event.total_reserved,
                required_threshold,
                status: "active".to_string(),
                created_at: timestamp,
                time: datetime,
                transaction_id: event.tx_digest.clone(),
            };

            diesel::insert_into(schema::spt_reservation_pools::table)
                .values(&minimal_pool)
                .execute(&mut conn)
                .await?;

            warn!(
                "Created minimal reservation pool record for missing pool: {}",
                pool_id
            );
        }

        // 3. Update progress tracking
        self.update_progress().await?;

        info!(
            "Successfully processed reservation withdrawn event for pool: {}",
            pool_id
        );
        Ok(())
    }

    /// Process reservation pool created events
    async fn process_reservation_pool_created_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        let fields = Self::extract_event_fields(&event.data)?;
        let pool_event = serde_json::from_value::<ReservationPoolCreatedEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse ReservationPoolCreatedEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        let mut reservation_pool = pool_event.into_reservation_pool_model(
            (event.timestamp_ms / 1000) as u64,
            event.tx_digest.clone(),
        )?;
        reservation_pool.time = datetime;

        diesel::insert_into(schema::spt_reservation_pools::table)
            .values(&reservation_pool)
            .execute(&mut conn)
            .await?;

        // Update profile with reservation pool address if this is a profile token
        if pool_event.token_type == 1 {
            // 1 = Profile token type
            diesel::update(schema::profiles::table)
                .filter(schema::profiles::owner_address.eq(&pool_event.owner))
                .set((
                    schema::profiles::reservation_pool_address.eq(&pool_event.pool_object_id),
                    schema::profiles::updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(&mut conn)
                .await?;
        }

        self.update_progress().await?;
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

        // Update reservation pool status to threshold_met
        let mut reservation_pool =
            threshold_event.into_reservation_pool_model(timestamp, event.tx_digest.clone())?;
        reservation_pool.time = datetime;

        diesel::insert_into(schema::spt_reservation_pools::table)
            .values(&reservation_pool)
            .execute(&mut conn)
            .await?;

        // Update progress tracking
        self.update_progress().await?;

        info!("Successfully processed threshold met event");
        Ok(())
    }

    /// Process PoC redirection updated events
    async fn process_poc_redirection_updated_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        let fields = Self::extract_event_fields(&event.data)?;
        let redirection_event = serde_json::from_value::<PocRedirectionUpdatedEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse PocRedirectionUpdatedEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;

        // Update the post's revenue redirection fields
        diesel::update(schema::posts::table)
            .filter(schema::posts::post_id.eq(&redirection_event.post_id))
            .set((
                schema::posts::revenue_redirect_to.eq(redirection_event.redirect_to.as_ref()),
                schema::posts::revenue_redirect_percentage.eq(
                    redirection_event.redirect_percentage.map(|p| p as i64),
                ),
            ))
            .execute(&mut conn)
            .await?;

        self.update_progress().await?;
        Ok(())
    }

    /// Process config updated events
    async fn process_config_updated_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        let fields = Self::extract_event_fields(&event.data)?;
        let config_event = serde_json::from_value::<ConfigUpdatedEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse ConfigUpdatedEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        let mut config = config_event.into_exchange_config_model(
            (event.timestamp_ms / 1000) as u64,
            event.tx_digest.clone(),
        )?;
        config.time = datetime;

        diesel::insert_into(schema::spt_exchange_config::table)
            .values(&config)
            .execute(&mut conn)
            .await?;

        self.update_progress().await?;
        Ok(())
    }

    /// Process post pool auto-initialized events
    async fn process_post_pool_auto_initialized_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!(
            "Processing SPT post pool auto-initialized event: {}",
            event.event_id
        );

        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let auto_init_event = serde_json::from_value::<PostPoolAutoInitializedEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse PostPoolAutoInitializedEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        // Convert to database model
        let mut token_pool = auto_init_event.into_model(timestamp as u64, event.tx_digest.clone())?;
        token_pool.time = datetime;

        // Insert into database
        diesel::insert_into(schema::social_proof_token_pools::table)
            .values(&token_pool)
            .execute(&mut conn)
            .await?;

        // Update progress tracking
        self.update_progress().await?;

        info!(
            "Successfully processed post pool auto-initialized event for post: {}, owner: {}",
            auto_init_event.post_id, auto_init_event.owner
        );
        Ok(())
    }

    /// Process tokens added events (when tokens are added to existing holdings)
    async fn process_tokens_added_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT tokens added event: {}", event.event_id);

        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let tokens_added_event = serde_json::from_value::<TokensAddedEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse TokensAddedEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        // Update holding for the owner
        let mut holding = tokens_added_event.into_holding_model(timestamp as u64, event.tx_digest.clone())?;
        holding.time = datetime;

        diesel::insert_into(schema::spt_holdings::table)
            .values(&holding)
            .execute(&mut conn)
            .await?;

        // Get the latest token pool to update supply
        let latest_pool = schema::social_proof_token_pools::table
            .filter(schema::social_proof_token_pools::pool_id.eq(&tokens_added_event.pool_id))
            .order_by(schema::social_proof_token_pools::time.desc())
            .first::<SocialProofTokenPool>(&mut conn)
            .await
            .optional()?;

        if let Some(pool) = latest_pool {
            // Update circulating supply
            let new_circulating_supply = pool.circulating_supply + tokens_added_event.amount as i64;
            let updated_pool = NewSocialProofTokenPool {
                pool_id: pool.pool_id.clone(),
                owner: pool.owner.clone(),
                name: pool.name.clone(),
                symbol: pool.symbol.clone(),
                token_type: pool.token_type,
                associated_id: pool.associated_id.clone(),
                base_price: pool.base_price,
                quadratic_coefficient: pool.quadratic_coefficient,
                circulating_supply: new_circulating_supply,
                created_at: pool.created_at,
                time: datetime,
                transaction_id: event.tx_digest.clone(),
            };

            diesel::insert_into(schema::social_proof_token_pools::table)
                .values(&updated_pool)
                .execute(&mut conn)
                .await?;
        }

        // Update progress tracking
        self.update_progress().await?;

        info!(
            "Successfully processed tokens added event for pool: {}, owner: {}, amount: {}",
            tokens_added_event.pool_id, tokens_added_event.owner, tokens_added_event.amount
        );
        Ok(())
    }

    /// Process emergency kill switch events
    async fn process_emergency_kill_switch_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!("Processing SPT emergency kill switch event: {}", event.event_id);

        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let kill_switch_event = serde_json::from_value::<EmergencyKillSwitchEvent>(fields)
            .map_err(|e| anyhow!("Failed to parse EmergencyKillSwitchEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        // Log the kill switch event to exchange config table
        let mut config = kill_switch_event.into_exchange_config_model(timestamp as u64, event.tx_digest.clone())?;
        config.time = datetime;

        diesel::insert_into(schema::spt_exchange_config::table)
            .values(&config)
            .execute(&mut conn)
            .await?;

        // Update progress tracking
        self.update_progress().await?;

        warn!(
            "Emergency kill switch {} by admin: {}, reason: {}",
            if kill_switch_event.trading_halted { "ACTIVATED" } else { "DEACTIVATED" },
            kill_switch_event.admin,
            kill_switch_event.reason
        );
        Ok(())
    }

    /// Process token pool created events and update profile with social proof token address
    async fn process_token_pool_created_event(&mut self, event: &BlockchainEvent) -> Result<()> {
        info!(
            "Processing SPT token pool created event: {}",
            event.event_id
        );

        // Extract and parse the event
        let fields = Self::extract_event_fields(&event.data)?;
        let token_pool_event = serde_json::from_value::<
            crate::events::social_proof_token_events::TokenPoolCreatedEvent,
        >(fields)
        .map_err(|e| anyhow!("Failed to parse TokenPoolCreatedEvent: {}", e))?;

        let mut conn = self.base.get_connection().await?;
        let timestamp = (event.timestamp_ms / 1000) as i64;
        let datetime = Self::timestamp_to_datetime(event.timestamp_ms);

        // Convert to database model
        let mut token_pool =
            token_pool_event.into_model(timestamp as u64, event.tx_digest.clone())?;
        token_pool.time = datetime;

        // Insert into database
        diesel::insert_into(schema::social_proof_token_pools::table)
            .values(&token_pool)
            .execute(&mut conn)
            .await?;

        // Update profile with social proof token address if this is a profile token
        if token_pool_event.token_type == 1 {
            // 1 = Profile token type
            // Update the profile's social_proof_token_address and updated_at timestamp
            diesel::update(schema::profiles::table)
                .filter(schema::profiles::owner_address.eq(&token_pool_event.owner))
                .set((
                    schema::profiles::social_proof_token_address.eq(&token_pool_event.id),
                    schema::profiles::updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(&mut conn)
                .await?;

            info!(
                "Updated profile {} with social proof token address: {}",
                token_pool_event.owner, token_pool_event.id
            );
        }

        // Create initial price history for the pool
        let price_history =
            token_pool_event.create_price_history(timestamp as u64, event.tx_digest.clone())?;
        diesel::insert_into(schema::spt_price_history::table)
            .values(&price_history)
            .execute(&mut conn)
            .await?;

        // Update progress tracking
        self.update_progress().await?;

        info!(
            "Successfully processed token pool created event for pool: {}, owner: {}, token_type: {}, associated_id: {}",
            token_pool_event.id, token_pool_event.owner, token_pool_event.token_type, token_pool_event.associated_id
        );
        Ok(())
    }

    /// Update progress tracking with meaningful metrics
    async fn update_progress(&self) -> Result<()> {
        let mut conn = self.base.get_connection().await?;
        let now = chrono::Utc::now().naive_utc();

        // Use actual events processed count as the "checkpoint" value for better tracking
        let events_processed = self.base.stats.events_processed as i64;

        let progress = NewIndexerProgress {
            id: self.base.name.clone(),
            last_checkpoint_processed: events_processed, // Use events processed as checkpoint
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

        // Log meaningful progress every 50 events (more frequent than base handler's 100)
        if events_processed > 0 && events_processed % 50 == 0 {
            info!(
                "SPT Handler Progress: {} events processed, {} failures, last processed: {}",
                self.base.stats.events_processed,
                self.base.stats.events_failed,
                self.base
                    .stats
                    .last_processed_timestamp
                    .map(|ts| format!("{}ms", ts))
                    .unwrap_or_else(|| "never".to_string())
            );
        }

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
            // Legacy event names (for backward compatibility)
            t if t.contains("::social_proof_token::") && t.ends_with("::InitPoolEvent") => {
                self.process_init_pool_event(&event).await
            }
            t if t.contains("::social_proof_token::") && t.ends_with("::BuyEvent") => {
                self.process_buy_event(&event).await
            }
            t if t.contains("::social_proof_token::") && t.ends_with("::SellEvent") => {
                self.process_sell_event(&event).await
            }
            // Current event names from smart contract
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::TokenPoolCreatedEvent") || t.ends_with("TokenPoolCreatedEvent")) =>
            {
                self.process_token_pool_created_event(&event).await
            }
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::PostPoolAutoInitializedEvent") || t.ends_with("PostPoolAutoInitializedEvent")) =>
            {
                self.process_post_pool_auto_initialized_event(&event).await
            }
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::TokenBoughtEvent") || t.ends_with("TokenBoughtEvent")) =>
            {
                self.process_token_bought_event(&event).await
            }
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::TokenSoldEvent") || t.ends_with("TokenSoldEvent")) =>
            {
                self.process_token_sold_event(&event).await
            }
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::TokensAddedEvent") || t.ends_with("TokensAddedEvent")) =>
            {
                self.process_tokens_added_event(&event).await
            }
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::ReservationPoolCreatedEvent") || t.ends_with("ReservationPoolCreatedEvent")) =>
            {
                self.process_reservation_pool_created_event(&event).await
            }
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::ReservationCreatedEvent") || t.ends_with("ReservationCreatedEvent")) =>
            {
                self.process_reservation_created_event(&event).await
            }
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::ReservationWithdrawnEvent") || t.ends_with("ReservationWithdrawnEvent")) =>
            {
                self.process_reservation_withdrawn_event(&event).await
            }
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::ThresholdMetEvent") || t.ends_with("ThresholdMetEvent")) =>
            {
                self.process_threshold_met_event(&event).await
            }
            t if t.contains("::social_proof_tokens::") && t.contains("ConfigUpdated") => {
                self.process_config_updated_event(&event).await
            }
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::PocRedirectionUpdatedEvent") || t.ends_with("PocRedirectionUpdatedEvent")) =>
            {
                self.process_poc_redirection_updated_event(&event).await
            }
            t if t.contains("::social_proof_tokens::")
                && (t.ends_with("::EmergencyKillSwitchEvent") || t.ends_with("EmergencyKillSwitchEvent")) =>
            {
                self.process_emergency_kill_switch_event(&event).await
            }
            _ => {
                warn!("Received unhandled SPT event type: {} (event_id: {})", event_type, event.event_id);
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
