// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

use crate::models::social_proof_token::{
    NewSocialProofTokenPool, NewSocialProofTokenTransaction,
    NewSocialProofTokenHolding, NewSocialProofPriceHistory,
    NewSocialProofAuctionPool, NewSocialProofAuctionContribution,
    TRANSACTION_TYPE_BUY, TRANSACTION_TYPE_SELL,
    TOKEN_TYPE_PROFILE, TOKEN_TYPE_POST, AUCTION_STATUS_ACTIVE
};

/// Event emitted when a token pool is created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPoolCreatedEvent {
    pub id: String,
    pub token_type: u8,
    pub owner: String,
    pub associated_id: String,
    pub symbol: String,
    pub name: String,
    pub base_price: u64,
    pub quadratic_coefficient: u64,
}

impl TokenPoolCreatedEvent {
    /// Convert the event to a database model
    pub fn into_model(&self, timestamp: u64, transaction_id: String) -> Result<NewSocialProofTokenPool> {
        let token_type = match self.token_type {
            1 => TOKEN_TYPE_PROFILE,
            2 => TOKEN_TYPE_POST,
            _ => return Err(anyhow!("Invalid token type: {}", self.token_type)),
        };
        
        Ok(NewSocialProofTokenPool {
            pool_id: self.id.clone(),
            token_type,
            owner: self.owner.clone(),
            associated_id: self.associated_id.clone(),
            symbol: self.symbol.clone(),
            name: self.name.clone(),
            circulating_supply: 0, // Initial supply is 0
            base_price: self.base_price as i64,
            quadratic_coefficient: self.quadratic_coefficient as i64,
            created_at: timestamp as i64,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }
    
    /// Create initial price history for the pool
    pub fn create_price_history(&self, _timestamp: u64, transaction_id: String) -> Result<NewSocialProofPriceHistory> {
        Ok(NewSocialProofPriceHistory {
            pool_id: self.id.clone(),
            price: self.base_price as i64, // Initial price is base price
            circulating_supply: 0,        // Initial supply is 0
            time: chrono::Utc::now(),
            transaction_id,
        })
    }
}

/// Event emitted when tokens are bought
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBoughtEvent {
    pub id: String,
    pub buyer: String,
    pub amount: u64,
    pub mys_amount: u64,
    pub fee_amount: u64,
    pub creator_fee: u64,
    pub platform_fee: u64,
    pub treasury_fee: u64,
    pub new_price: u64,
}

impl TokenBoughtEvent {
    /// Convert the event to a transaction model
    pub fn into_transaction_model(&self, timestamp: u64, transaction_id: String) -> Result<NewSocialProofTokenTransaction> {
        Ok(NewSocialProofTokenTransaction {
            pool_id: self.id.clone(),
            transaction_type: TRANSACTION_TYPE_BUY.to_string(),
            sender: self.buyer.clone(),
            amount: self.amount as i64,
            mys_amount: self.mys_amount as i64,
            fee_amount: self.fee_amount as i64,
            creator_fee: self.creator_fee as i64,
            platform_fee: self.platform_fee as i64,
            treasury_fee: self.treasury_fee as i64,
            price: self.new_price as i64,
            created_at: timestamp as i64,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }
    
    /// Convert the event to a holding model
    pub fn into_holding_model(&self, timestamp: u64, transaction_id: String) -> Result<NewSocialProofTokenHolding> {
        Ok(NewSocialProofTokenHolding {
            pool_id: self.id.clone(),
            holder_address: self.buyer.clone(),
            amount: self.amount as i64,
            acquired_at: timestamp as i64,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }
    
    /// Create price history for the transaction
    pub fn create_price_history(&self, circulating_supply: i64, _timestamp: u64, transaction_id: String) -> Result<NewSocialProofPriceHistory> {
        Ok(NewSocialProofPriceHistory {
            pool_id: self.id.clone(),
            price: self.new_price as i64,
            circulating_supply,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }

    /// Create SPT revenue record for swap fees
    pub fn create_spt_revenue(&self, creator_address: String, platform_address: String, treasury_address: String, timestamp: u64, transaction_id: String) -> Result<crate::models::NewSptRevenue> {
        Ok(crate::models::NewSptRevenue::from_buy_event(
            self.id.clone(),
            self.buyer.clone(),
            creator_address,
            platform_address,
            treasury_address,
            self.creator_fee as i64,
            self.platform_fee as i64,
            self.treasury_fee as i64,
            self.amount as i64,
            self.mys_amount as i64,
            self.new_price as i64,
            timestamp as i64,
            transaction_id,
        ))
    }

    /// Create unified revenue records for creator, platform, and treasury fees
    pub fn create_unified_revenue_records(&self, creator_address: String, platform_address: String, treasury_address: String, timestamp: u64, transaction_id: String) -> Result<Vec<crate::models::NewUnifiedRevenue>> {
        let mut records = Vec::new();

        // Creator fee revenue
        if self.creator_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_CREATOR_FEE.to_string(),
                creator_address.clone(),
                Some(platform_address.clone()),
                self.creator_fee as i64,
                self.id.clone(),
                self.buyer.clone(),
                creator_address.clone(),
                timestamp as i64,
                transaction_id.clone(),
            ));
        }

        // Platform fee revenue
        if self.platform_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_PLATFORM_FEE.to_string(),
                creator_address.clone(),
                Some(platform_address.clone()),
                self.platform_fee as i64,
                self.id.clone(),
                self.buyer.clone(),
                platform_address,
                timestamp as i64,
                transaction_id.clone(),
            ));
        }

        // Treasury fee revenue
        if self.treasury_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_TREASURY_FEE.to_string(),
                creator_address.clone(),
                None,
                self.treasury_fee as i64,
                self.id.clone(),
                self.buyer.clone(),
                treasury_address,
                timestamp as i64,
                transaction_id,
            ));
        }

        Ok(records)
    }
}

/// Event emitted when tokens are sold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSoldEvent {
    pub id: String,
    pub seller: String,
    pub amount: u64,
    pub mys_amount: u64,
    pub fee_amount: u64,
    pub creator_fee: u64,
    pub platform_fee: u64,
    pub treasury_fee: u64,
    pub new_price: u64,
}

impl TokenSoldEvent {
    /// Convert the event to a transaction model
    pub fn into_transaction_model(&self, timestamp: u64, transaction_id: String) -> Result<NewSocialProofTokenTransaction> {
        Ok(NewSocialProofTokenTransaction {
            pool_id: self.id.clone(),
            transaction_type: TRANSACTION_TYPE_SELL.to_string(),
            sender: self.seller.clone(),
            amount: self.amount as i64,
            mys_amount: self.mys_amount as i64,
            fee_amount: self.fee_amount as i64,
            creator_fee: self.creator_fee as i64,
            platform_fee: self.platform_fee as i64,
            treasury_fee: self.treasury_fee as i64,
            price: self.new_price as i64,
            created_at: timestamp as i64,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }
    
    /// Create price history for the transaction
    pub fn create_price_history(&self, circulating_supply: i64, _timestamp: u64, transaction_id: String) -> Result<NewSocialProofPriceHistory> {
        Ok(NewSocialProofPriceHistory {
            pool_id: self.id.clone(),
            price: self.new_price as i64,
            circulating_supply,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }

    /// Create SPT revenue record for swap fees
    pub fn create_spt_revenue(&self, creator_address: String, platform_address: String, treasury_address: String, timestamp: u64, transaction_id: String) -> Result<crate::models::NewSptRevenue> {
        Ok(crate::models::NewSptRevenue::from_sell_event(
            self.id.clone(),
            self.seller.clone(),
            creator_address,
            platform_address,
            treasury_address,
            self.creator_fee as i64,
            self.platform_fee as i64,
            self.treasury_fee as i64,
            self.amount as i64,
            self.mys_amount as i64,
            self.new_price as i64,
            timestamp as i64,
            transaction_id,
        ))
    }

    /// Create unified revenue records for creator, platform, and treasury fees
    pub fn create_unified_revenue_records(&self, creator_address: String, platform_address: String, treasury_address: String, timestamp: u64, transaction_id: String) -> Result<Vec<crate::models::NewUnifiedRevenue>> {
        let mut records = Vec::new();

        // Creator fee revenue
        if self.creator_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_CREATOR_FEE.to_string(),
                creator_address.clone(),
                Some(platform_address.clone()),
                self.creator_fee as i64,
                self.id.clone(),
                self.seller.clone(),
                creator_address.clone(),
                timestamp as i64,
                transaction_id.clone(),
            ));
        }

        // Platform fee revenue
        if self.platform_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_PLATFORM_FEE.to_string(),
                creator_address.clone(),
                Some(platform_address.clone()),
                self.platform_fee as i64,
                self.id.clone(),
                self.seller.clone(),
                platform_address,
                timestamp as i64,
                transaction_id.clone(),
            ));
        }

        // Treasury fee revenue
        if self.treasury_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_TREASURY_FEE.to_string(),
                creator_address.clone(),
                None,
                self.treasury_fee as i64,
                self.id.clone(),
                self.seller.clone(),
                treasury_address,
                timestamp as i64,
                transaction_id,
            ));
        }

        Ok(records)
    }
}

/// Event emitted when tokens are added to an existing holding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokensAddedEvent {
    pub owner: String,
    pub pool_id: String,
    pub amount: u64,
}

impl TokensAddedEvent {
    /// Convert the event to a holding model
    pub fn into_holding_model(&self, timestamp: u64, transaction_id: String) -> Result<NewSocialProofTokenHolding> {
        Ok(NewSocialProofTokenHolding {
            pool_id: self.pool_id.clone(),
            holder_address: self.owner.clone(),
            amount: self.amount as i64,
            acquired_at: timestamp as i64,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }
}

/// Event emitted when an auction is created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionCreatedEvent {
    pub auction_id: String,
    pub associated_id: String,
    pub token_type: u8,
    pub owner: String,
    pub start_time: u64,
    pub duration: u64,
}

impl AuctionCreatedEvent {
    /// Convert the event to an auction pool model
    pub fn into_model(&self, _timestamp: u64, transaction_id: String) -> Result<NewSocialProofAuctionPool> {
        let token_type = match self.token_type {
            1 => TOKEN_TYPE_PROFILE,
            2 => TOKEN_TYPE_POST,
            _ => return Err(anyhow!("Invalid token type: {}", self.token_type)),
        };
        
        Ok(NewSocialProofAuctionPool {
            auction_id: self.auction_id.clone(),
            associated_id: self.associated_id.clone(),
            token_type,
            owner: self.owner.clone(),
            status: AUCTION_STATUS_ACTIVE,
            start_time: self.start_time as i64,
            duration: self.duration as i64,
            total_contribution: 0,
            total_tokens: 0,
            finalized_at: None,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }
}

/// Event emitted when a contribution is made to an auction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionContributionEvent {
    pub auction_id: String,
    pub contributor: String,
    pub amount: u64,
    pub total_contribution: u64,
}

impl AuctionContributionEvent {
    /// Convert the event to an auction contribution model
    pub fn into_model(&self, timestamp: u64, transaction_id: String) -> Result<NewSocialProofAuctionContribution> {
        Ok(NewSocialProofAuctionContribution {
            auction_id: self.auction_id.clone(),
            contributor_address: self.contributor.clone(),
            amount: self.amount as i64,
            contributed_at: timestamp as i64,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }
}

/// Event emitted when an auction is finalized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionFinalizedEvent {
    pub auction_id: String,
    pub associated_id: String,
    pub total_contribution: u64,
    pub total_tokens: u64,
    pub token_price: u64,
    pub pool_id: String,
}

/// Event emitted when exchange config is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdatedEvent {
    pub updated_by: String,
    pub timestamp: u64,
    pub total_fee_bps: u64,
    pub creator_fee_bps: u64,
    pub platform_fee_bps: u64,
    pub treasury_fee_bps: u64,
    pub base_price: u64,
    pub quadratic_coefficient: u64,
    pub ecosystem_treasury: String,
    pub max_hold_percent_bps: u64,
    pub post_viral_threshold: u64,
    pub profile_viral_threshold: u64,
    pub min_post_auction_duration: u64,
    pub max_post_auction_duration: u64,
    pub min_profile_auction_duration: u64,
    pub max_profile_auction_duration: u64,
}

// Init pool event structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialProofInitPoolEvent {
    pub pool_id: String,
    pub owner: String,
    pub name: String,
    pub symbol: String,
    pub token_type: i16,
    pub associated_id: String,
    pub base_price: i64,
    pub quadratic_coefficient: i64,
}

impl TryFrom<Value> for SocialProofInitPoolEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value.as_object().ok_or_else(|| anyhow!("Expected object"))?;
        
        Ok(Self {
            pool_id: obj.get("pool_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid pool_id"))?
                .to_string(),
            owner: obj.get("owner")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid owner"))?
                .to_string(),
            name: obj.get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid name"))?
                .to_string(),
            symbol: obj.get("symbol")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid symbol"))?
                .to_string(),
            token_type: obj.get("token_type")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid token_type"))?
                as i16,
            associated_id: obj.get("associated_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid associated_id"))?
                .to_string(),
            base_price: obj.get("base_price")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid base_price"))?,
            quadratic_coefficient: obj.get("quadratic_coefficient")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid quadratic_coefficient"))?,
        })
    }
}

impl SocialProofInitPoolEvent {
    pub fn into_model(&self, time: i64, transaction_id: String) -> Result<NewSocialProofTokenPool> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0)
            .unwrap_or_else(|| Utc::now());
        
        Ok(NewSocialProofTokenPool {
            pool_id: self.pool_id.clone(),
            owner: self.owner.clone(),
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            token_type: self.token_type,
            associated_id: self.associated_id.clone(),
            base_price: self.base_price,
            quadratic_coefficient: self.quadratic_coefficient,
            circulating_supply: 0, // New pool starts with 0 supply
            created_at: time,
            time: datetime,
            transaction_id,
        })
    }
}

// Buy event structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialProofBuyEvent {
    pub pool_id: String,
    pub buyer: String,
    pub amount: i64,
    pub mys_amount: i64,
    pub fee_amount: i64,
    pub creator_fee: i64,
    pub platform_fee: i64,
    pub treasury_fee: i64,
    pub price: i64,
}

impl TryFrom<Value> for SocialProofBuyEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value.as_object().ok_or_else(|| anyhow!("Expected object"))?;
        
        Ok(Self {
            pool_id: obj.get("pool_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid pool_id"))?
                .to_string(),
            buyer: obj.get("buyer")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid buyer"))?
                .to_string(),
            amount: obj.get("amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid amount"))?,
            mys_amount: obj.get("mys_amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid mys_amount"))?,
            fee_amount: obj.get("fee_amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid fee_amount"))?,
            creator_fee: obj.get("creator_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid creator_fee"))?,
            platform_fee: obj.get("platform_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid platform_fee"))?,
            treasury_fee: obj.get("treasury_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid treasury_fee"))?,
            price: obj.get("price")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid price"))?,
        })
    }
}

impl SocialProofBuyEvent {
    pub fn into_transaction_model(&self, time: i64, transaction_id: String) -> Result<NewSocialProofTokenTransaction> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0)
            .unwrap_or_else(|| Utc::now());
            
        Ok(NewSocialProofTokenTransaction {
            pool_id: self.pool_id.clone(),
            transaction_type: TRANSACTION_TYPE_BUY.to_string(),
            sender: self.buyer.clone(),
            amount: self.amount,
            mys_amount: self.mys_amount,
            fee_amount: self.fee_amount,
            creator_fee: self.creator_fee,
            platform_fee: self.platform_fee,
            treasury_fee: self.treasury_fee,
            price: self.price,
            created_at: time,
            time: datetime,
            transaction_id,
        })
    }

    pub fn into_holding_model(&self, time: i64, transaction_id: String) -> Result<NewSocialProofTokenHolding> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0)
            .unwrap_or_else(|| Utc::now());
            
        Ok(NewSocialProofTokenHolding {
            pool_id: self.pool_id.clone(),
            holder_address: self.buyer.clone(),
            amount: self.amount,
            acquired_at: time,
            time: datetime,
            transaction_id,
        })
    }

    pub fn into_price_history_model(&self, circulating_supply: i64, time: i64, transaction_id: String) -> Result<NewSocialProofPriceHistory> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0)
            .unwrap_or_else(|| Utc::now());
            
        Ok(NewSocialProofPriceHistory {
            pool_id: self.pool_id.clone(),
            price: self.price,
            circulating_supply,
            time: datetime,
            transaction_id,
        })
    }

    /// Create SPT revenue record for swap fees
    pub fn create_spt_revenue(&self, creator_address: String, platform_address: String, treasury_address: String, time: i64, transaction_id: String) -> Result<crate::models::NewSptRevenue> {
        Ok(crate::models::NewSptRevenue::from_buy_event(
            self.pool_id.clone(),
            self.buyer.clone(),
            creator_address,
            platform_address,
            treasury_address,
            self.creator_fee,
            self.platform_fee,
            self.treasury_fee,
            self.amount,
            self.mys_amount,
            self.price,
            time,
            transaction_id,
        ))
    }

    /// Create unified revenue records for creator, platform, and treasury fees
    pub fn create_unified_revenue_records(&self, creator_address: String, platform_address: String, treasury_address: String, time: i64, transaction_id: String) -> Result<Vec<crate::models::NewUnifiedRevenue>> {
        let mut records = Vec::new();

        // Creator fee revenue
        if self.creator_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_CREATOR_FEE.to_string(),
                creator_address.clone(),
                Some(platform_address.clone()),
                self.creator_fee,
                self.pool_id.clone(),
                self.buyer.clone(),
                creator_address.clone(),
                time,
                transaction_id.clone(),
            ));
        }

        // Platform fee revenue
        if self.platform_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_PLATFORM_FEE.to_string(),
                creator_address.clone(),
                Some(platform_address.clone()),
                self.platform_fee,
                self.pool_id.clone(),
                self.buyer.clone(),
                platform_address,
                time,
                transaction_id.clone(),
            ));
        }

        // Treasury fee revenue
        if self.treasury_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_TREASURY_FEE.to_string(),
                creator_address.clone(),
                None,
                self.treasury_fee,
                self.pool_id.clone(),
                self.buyer.clone(),
                treasury_address,
                time,
                transaction_id,
            ));
        }

        Ok(records)
    }
}

// Sell event structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialProofSellEvent {
    pub pool_id: String,
    pub seller: String,
    pub amount: i64,
    pub mys_amount: i64,
    pub fee_amount: i64,
    pub creator_fee: i64,
    pub platform_fee: i64,
    pub treasury_fee: i64,
    pub price: i64,
}

impl TryFrom<Value> for SocialProofSellEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value.as_object().ok_or_else(|| anyhow!("Expected object"))?;
        
        Ok(Self {
            pool_id: obj.get("pool_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid pool_id"))?
                .to_string(),
            seller: obj.get("seller")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid seller"))?
                .to_string(),
            amount: obj.get("amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid amount"))?,
            mys_amount: obj.get("mys_amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid mys_amount"))?,
            fee_amount: obj.get("fee_amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid fee_amount"))?,
            creator_fee: obj.get("creator_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid creator_fee"))?,
            platform_fee: obj.get("platform_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid platform_fee"))?,
            treasury_fee: obj.get("treasury_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid treasury_fee"))?,
            price: obj.get("price")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid price"))?,
        })
    }
}

impl SocialProofSellEvent {
    pub fn into_transaction_model(&self, time: i64, transaction_id: String) -> Result<NewSocialProofTokenTransaction> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0)
            .unwrap_or_else(|| Utc::now());
            
        Ok(NewSocialProofTokenTransaction {
            pool_id: self.pool_id.clone(),
            transaction_type: TRANSACTION_TYPE_SELL.to_string(),
            sender: self.seller.clone(),
            amount: self.amount,
            mys_amount: self.mys_amount,
            fee_amount: self.fee_amount,
            creator_fee: self.creator_fee,
            platform_fee: self.platform_fee,
            treasury_fee: self.treasury_fee,
            price: self.price,
            created_at: time,
            time: datetime,
            transaction_id,
        })
    }

    pub fn into_holding_model(&self, time: i64, transaction_id: String) -> Result<NewSocialProofTokenHolding> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0)
            .unwrap_or_else(|| Utc::now());
            
        Ok(NewSocialProofTokenHolding {
            pool_id: self.pool_id.clone(),
            holder_address: self.seller.clone(),
            amount: -self.amount, // Negative amount as the seller is selling
            acquired_at: time,
            time: datetime,
            transaction_id,
        })
    }

    pub fn into_price_history_model(&self, circulating_supply: i64, time: i64, transaction_id: String) -> Result<NewSocialProofPriceHistory> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0)
            .unwrap_or_else(|| Utc::now());
            
        Ok(NewSocialProofPriceHistory {
            pool_id: self.pool_id.clone(),
            price: self.price,
            circulating_supply,
            time: datetime,
            transaction_id,
        })
    }

    /// Create SPT revenue record for swap fees
    pub fn create_spt_revenue(&self, creator_address: String, platform_address: String, treasury_address: String, time: i64, transaction_id: String) -> Result<crate::models::NewSptRevenue> {
        Ok(crate::models::NewSptRevenue::from_sell_event(
            self.pool_id.clone(),
            self.seller.clone(),
            creator_address,
            platform_address,
            treasury_address,
            self.creator_fee,
            self.platform_fee,
            self.treasury_fee,
            self.amount,
            self.mys_amount,
            self.price,
            time,
            transaction_id,
        ))
    }

    /// Create unified revenue records for creator, platform, and treasury fees
    pub fn create_unified_revenue_records(&self, creator_address: String, platform_address: String, treasury_address: String, time: i64, transaction_id: String) -> Result<Vec<crate::models::NewUnifiedRevenue>> {
        let mut records = Vec::new();

        // Creator fee revenue
        if self.creator_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_CREATOR_FEE.to_string(),
                creator_address.clone(),
                Some(platform_address.clone()),
                self.creator_fee,
                self.pool_id.clone(),
                self.seller.clone(),
                creator_address.clone(),
                time,
                transaction_id.clone(),
            ));
        }

        // Platform fee revenue
        if self.platform_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_PLATFORM_FEE.to_string(),
                creator_address.clone(),
                Some(platform_address.clone()),
                self.platform_fee,
                self.pool_id.clone(),
                self.seller.clone(),
                platform_address,
                time,
                transaction_id.clone(),
            ));
        }

        // Treasury fee revenue
        if self.treasury_fee > 0 {
            records.push(crate::models::NewUnifiedRevenue::from_spt(
                crate::models::revenue::REVENUE_TYPE_SPT_TREASURY_FEE.to_string(),
                creator_address.clone(),
                None,
                self.treasury_fee,
                self.pool_id.clone(),
                self.seller.clone(),
                treasury_address,
                time,
                transaction_id,
            ));
        }

        Ok(records)
    }
}

// Start auction event structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialProofStartAuctionEvent {
    pub auction_id: String,
    pub associated_id: String,
    pub token_type: i16,
    pub owner: String,
    pub start_time: i64,
    pub duration: i64,
    pub total_tokens: i64,
}

impl TryFrom<Value> for SocialProofStartAuctionEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value.as_object().ok_or_else(|| anyhow!("Expected object"))?;
        
        Ok(Self {
            auction_id: obj.get("auction_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid auction_id"))?
                .to_string(),
            associated_id: obj.get("associated_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid associated_id"))?
                .to_string(),
            token_type: obj.get("token_type")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid token_type"))?
                as i16,
            owner: obj.get("owner")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid owner"))?
                .to_string(),
            start_time: obj.get("start_time")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid start_time"))?,
            duration: obj.get("duration")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid duration"))?,
            total_tokens: obj.get("total_tokens")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid total_tokens"))?,
        })
    }
}

impl SocialProofStartAuctionEvent {
    pub fn into_model(&self, time: i64, transaction_id: String) -> Result<NewSocialProofAuctionPool> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0)
            .unwrap_or_else(|| Utc::now());
            
        Ok(NewSocialProofAuctionPool {
            auction_id: self.auction_id.clone(),
            associated_id: self.associated_id.clone(),
            token_type: self.token_type,
            owner: self.owner.clone(),
            status: AUCTION_STATUS_ACTIVE,
            start_time: self.start_time,
            duration: self.duration,
            total_contribution: 0, // Starts with 0 contributions
            total_tokens: self.total_tokens,
            finalized_at: None,
            time: datetime,
            transaction_id,
        })
    }
}

// Contribute to auction event structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialProofContributeAuctionEvent {
    pub auction_id: String,
    pub contributor_address: String,
    pub amount: i64,
    pub contributed_at: i64,
}

impl TryFrom<Value> for SocialProofContributeAuctionEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value.as_object().ok_or_else(|| anyhow!("Expected object"))?;
        
        Ok(Self {
            auction_id: obj.get("auction_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid auction_id"))?
                .to_string(),
            contributor_address: obj.get("contributor_address")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid contributor_address"))?
                .to_string(),
            amount: obj.get("amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid amount"))?,
            contributed_at: obj.get("contributed_at")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid contributed_at"))?,
        })
    }
}

impl SocialProofContributeAuctionEvent {
    pub fn into_model(&self, time: i64, transaction_id: String) -> Result<NewSocialProofAuctionContribution> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0)
            .unwrap_or_else(|| Utc::now());
            
        Ok(NewSocialProofAuctionContribution {
            auction_id: self.auction_id.clone(),
            contributor_address: self.contributor_address.clone(),
            amount: self.amount,
            contributed_at: self.contributed_at,
            time: datetime,
            transaction_id,
        })
    }
}

// Finalize auction event structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialProofFinalizeAuctionEvent {
    pub auction_id: String,
    pub tokens_minted: i64,
    pub finalized_at: i64,
}

impl TryFrom<Value> for SocialProofFinalizeAuctionEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value.as_object().ok_or_else(|| anyhow!("Expected object"))?;
        
        Ok(Self {
            auction_id: obj.get("auction_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid auction_id"))?
                .to_string(),
            tokens_minted: obj.get("tokens_minted")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid tokens_minted"))?,
            finalized_at: obj.get("finalized_at")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid finalized_at"))?,
        })
    }
} 