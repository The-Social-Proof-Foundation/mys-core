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
    SocialProofStartAuctionEvent, SocialProofContributeAuctionEvent,
    SocialProofFinalizeAuctionEvent,
};
use crate::models::social_proof_token::{
    NewSocialProofTokenPool, SocialProofTokenPool,
    NewSocialProofAuctionPool, SocialProofAuctionPool,
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
            "social_proof_token::auction_pool::StartAuctionEvent" => {
                self.handle_start_auction_event(event).await
            },
            "social_proof_token::auction_pool::ContributeAuctionEvent" => {
                self.handle_contribute_auction_event(event).await
            },
            "social_proof_token::auction_pool::FinalizeAuctionEvent" => {
                self.handle_finalize_auction_event(event).await
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
    
    // Handle start auction events
    async fn handle_start_auction_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling start auction event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse the event
        let auction_event = SocialProofStartAuctionEvent::try_from(event.contents.clone())?;
        
        // Process the event data
        let mut auction_pool = auction_event.into_model(timestamp, transaction_id.clone())?;
        auction_pool.time = datetime;
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // Insert the auction pool
        diesel::insert_into(crate::schema::spt_auction_pools::table)
            .values(&auction_pool)
            .execute(&mut conn)
            .await?;
        
        info!("Processed start auction event for auction ID: {}", auction_pool.auction_id);
        Ok(())
    }
    
    // Handle contribute to auction events
    async fn handle_contribute_auction_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling contribute to auction event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse the event
        let contribute_event = SocialProofContributeAuctionEvent::try_from(event.contents.clone())?;
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // 1. Create contribution record
        let mut contribution = contribute_event.into_model(timestamp, transaction_id.clone())?;
        contribution.time = datetime;
        
        diesel::insert_into(crate::schema::spt_auction_contributions::table)
            .values(&contribution)
            .execute(&mut conn)
            .await?;
        
        // 2. Update the auction pool with new values
        let latest_auction = crate::schema::spt_auction_pools::table
            .filter(crate::schema::spt_auction_pools::auction_id.eq(&contribute_event.auction_id))
            .order_by(crate::schema::spt_auction_pools::time.desc())
            .first::<SocialProofAuctionPool>(&mut conn)
            .await
            .map_err(|e| anyhow!("Failed to get latest auction pool: {}", e))?;
        
        let new_total_contribution = latest_auction.total_contribution + contribute_event.amount;
        
        let updated_auction = NewSocialProofAuctionPool {
            auction_id: latest_auction.auction_id.clone(),
            associated_id: latest_auction.associated_id.clone(),
            token_type: latest_auction.token_type,
            owner: latest_auction.owner.clone(),
            status: latest_auction.status,
            total_contribution: new_total_contribution,
            total_tokens: latest_auction.total_tokens,
            start_time: latest_auction.start_time,
            duration: latest_auction.duration,
            finalized_at: latest_auction.finalized_at,
            time: datetime,
            transaction_id: transaction_id.clone(),
        };
        
        diesel::insert_into(crate::schema::spt_auction_pools::table)
            .values(&updated_auction)
            .execute(&mut conn)
            .await?;
        
        info!("Processed contribute auction event for auction ID: {}", contribution.auction_id);
        Ok(())
    }
    
    // Handle finalize auction events
    async fn handle_finalize_auction_event(&self, event_with_meta: MysEventWithMetadata) -> Result<()> {
        info!("Handling finalize auction event");
        
        let event = &event_with_meta.event;
        let transaction_id = event_with_meta.transaction_digest.clone();
        let timestamp = event_with_meta.timestamp;
        let datetime = Self::timestamp_to_datetime(timestamp);
        
        // Parse the event
        let finalize_event = SocialProofFinalizeAuctionEvent::try_from(event.contents.clone())?;
        
        // Get database connection
        let mut conn = self.db.get_connection().await?;
        
        // 1. Get the latest auction record
        let latest_auction = crate::schema::spt_auction_pools::table
            .filter(crate::schema::spt_auction_pools::auction_id.eq(&finalize_event.auction_id))
            .order_by(crate::schema::spt_auction_pools::time.desc())
            .first::<SocialProofAuctionPool>(&mut conn)
            .await
            .map_err(|e| anyhow!("Failed to get latest auction pool: {}", e))?;
        
        // 2. Create updated auction record with finalized status
        let updated_auction = NewSocialProofAuctionPool {
            auction_id: latest_auction.auction_id.clone(),
            associated_id: latest_auction.associated_id.clone(),
            token_type: latest_auction.token_type,
            owner: latest_auction.owner.clone(),
            status: 2, // Finalized
            total_contribution: latest_auction.total_contribution,
            total_tokens: latest_auction.total_tokens,
            start_time: latest_auction.start_time,
            duration: latest_auction.duration,
            finalized_at: Some(finalize_event.finalized_at),
            time: datetime,
            transaction_id: transaction_id.clone(),
        };
        
        // 3. Insert updated auction record
        diesel::insert_into(crate::schema::spt_auction_pools::table)
            .values(&updated_auction)
            .execute(&mut conn)
            .await?;
        
        info!("Processed finalize auction event for auction ID: {}", finalize_event.auction_id);
        Ok(())
    }
} 