// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::events::event_utils::deserialize_u64_from_string;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::social_proof_token::{
    NewSocialProofPriceHistory, NewSocialProofTokenHolding, NewSocialProofTokenPool,
    NewSocialProofTokenTransaction, NewSptExchangeConfig, NewSptReservation, NewSptReservationPool,
    RESERVATION_POOL_STATUS_ACTIVE, RESERVATION_POOL_STATUS_THRESHOLD_MET, TOKEN_TYPE_POST,
    TOKEN_TYPE_PROFILE, TRANSACTION_TYPE_BUY, TRANSACTION_TYPE_SELL,
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
    pub fn into_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenPool> {
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
    pub fn create_price_history(
        &self,
        _timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSocialProofPriceHistory> {
        Ok(NewSocialProofPriceHistory {
            pool_id: self.id.clone(),
            price: self.base_price as i64, // Initial price is base price
            circulating_supply: 0,         // Initial supply is 0
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
    pub fn into_transaction_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenTransaction> {
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
    pub fn into_holding_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenHolding> {
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
    pub fn create_price_history(
        &self,
        circulating_supply: i64,
        _timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSocialProofPriceHistory> {
        Ok(NewSocialProofPriceHistory {
            pool_id: self.id.clone(),
            price: self.new_price as i64,
            circulating_supply,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }

    /// Create SPT revenue record for swap fees
    pub fn create_spt_revenue(
        &self,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<crate::models::NewSptRevenue> {
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
    pub fn create_unified_revenue_records(
        &self,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<Vec<crate::models::NewUnifiedRevenue>> {
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
    pub fn into_transaction_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenTransaction> {
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

    /// Convert the event to a holding model (negative amount for sell)
    pub fn into_holding_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenHolding> {
        Ok(NewSocialProofTokenHolding {
            pool_id: self.id.clone(),
            holder_address: self.seller.clone(),
            amount: -(self.amount as i64), // Negative amount as the seller is selling
            acquired_at: timestamp as i64,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }

    /// Create price history for the transaction
    pub fn create_price_history(
        &self,
        circulating_supply: i64,
        _timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSocialProofPriceHistory> {
        Ok(NewSocialProofPriceHistory {
            pool_id: self.id.clone(),
            price: self.new_price as i64,
            circulating_supply,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }

    /// Create SPT revenue record for swap fees
    pub fn create_spt_revenue(
        &self,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<crate::models::NewSptRevenue> {
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
    pub fn create_unified_revenue_records(
        &self,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<Vec<crate::models::NewUnifiedRevenue>> {
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
    pub fn into_holding_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenHolding> {
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

/// Event emitted when MYS is reserved towards a post/profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationCreatedEvent {
    pub associated_id: String,
    pub token_type: u8,
    pub reserver: String,
    pub amount: u64,
    pub total_reserved: u64,
    pub threshold_met: bool,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub reserved_at: u64,
}

impl ReservationCreatedEvent {
    /// Convert the event to a reservation model
    pub fn into_reservation_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSptReservation> {
        let pool_id = format!("reservation_pool_{}", self.associated_id);

        Ok(NewSptReservation {
            pool_id,
            reserver_address: self.reserver.clone(),
            amount: self.amount as i64,
            reserved_at: self.reserved_at as i64,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            transaction_id,
        })
    }

    /// Convert the event to a reservation pool model (for updating total)
    pub fn into_reservation_pool_model(
        &self,
        timestamp: u64,
        transaction_id: String,
        required_threshold: i64,
    ) -> Result<NewSptReservationPool> {
        let token_type = match self.token_type {
            1 => TOKEN_TYPE_PROFILE,
            2 => TOKEN_TYPE_POST,
            _ => return Err(anyhow!("Invalid token type: {}", self.token_type)),
        };

        let pool_id = format!("reservation_pool_{}", self.associated_id);
        let status = if self.threshold_met {
            RESERVATION_POOL_STATUS_THRESHOLD_MET.to_string()
        } else {
            RESERVATION_POOL_STATUS_ACTIVE.to_string()
        };

        Ok(NewSptReservationPool {
            pool_id,
            associated_id: self.associated_id.clone(),
            token_type,
            owner: "".to_string(), // Will be filled from the actual event data
            total_reserved: self.total_reserved as i64,
            required_threshold,
            status,
            created_at: self.reserved_at as i64,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            transaction_id,
        })
    }
}

/// Event emitted when MYS reservation is withdrawn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationWithdrawnEvent {
    pub associated_id: String,
    pub token_type: u8,
    pub reserver: String,
    pub amount: u64,
    pub total_reserved: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub withdrawn_at: u64,
}

impl ReservationWithdrawnEvent {
    /// Convert the event to a reservation model (for withdrawals, amount is negative)
    pub fn into_reservation_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSptReservation> {
        let pool_id = format!("reservation_pool_{}", self.associated_id);

        // For withdrawals, we store the remaining amount, not the withdrawn amount
        Ok(NewSptReservation {
            pool_id,
            reserver_address: self.reserver.clone(),
            amount: 0, // This represents the final amount after withdrawal
            reserved_at: self.withdrawn_at as i64,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            transaction_id,
        })
    }
}

/// Event emitted when reservation threshold is met for the first time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdMetEvent {
    pub associated_id: String,
    pub token_type: u8,
    pub owner: String,
    pub total_reserved: u64,
    pub required_threshold: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

impl ThresholdMetEvent {
    /// Convert the event to update reservation pool status
    pub fn into_reservation_pool_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSptReservationPool> {
        let token_type = match self.token_type {
            1 => TOKEN_TYPE_PROFILE,
            2 => TOKEN_TYPE_POST,
            _ => return Err(anyhow!("Invalid token type: {}", self.token_type)),
        };

        let pool_id = format!("reservation_pool_{}", self.associated_id);

        Ok(NewSptReservationPool {
            pool_id,
            associated_id: self.associated_id.clone(),
            token_type,
            owner: self.owner.clone(),
            total_reserved: self.total_reserved as i64,
            required_threshold: self.required_threshold as i64,
            status: RESERVATION_POOL_STATUS_THRESHOLD_MET.to_string(),
            created_at: self.timestamp as i64,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            transaction_id,
        })
    }
}

/// Event emitted when exchange config is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdatedEvent {
    pub updated_by: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
    pub total_fee_bps: u64,
    pub creator_fee_bps: u64,
    pub platform_fee_bps: u64,
    pub treasury_fee_bps: u64,
    pub base_price: u64,
    pub quadratic_coefficient: u64,
    pub ecosystem_treasury: String,
    pub max_hold_percent_bps: u64,
    pub post_threshold: u64,
    pub profile_threshold: u64,
    pub max_individual_reservation_bps: u64,
}

impl TryFrom<Value> for ConfigUpdatedEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("Expected object"))?;

        Ok(Self {
            updated_by: obj
                .get("updated_by")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid updated_by"))?
                .to_string(),
            timestamp: obj
                .get("timestamp")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid timestamp"))?,
            total_fee_bps: obj
                .get("total_fee_bps")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid total_fee_bps"))?,
            creator_fee_bps: obj
                .get("creator_fee_bps")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid creator_fee_bps"))?,
            platform_fee_bps: obj
                .get("platform_fee_bps")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid platform_fee_bps"))?,
            treasury_fee_bps: obj
                .get("treasury_fee_bps")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid treasury_fee_bps"))?,
            base_price: obj
                .get("base_price")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid base_price"))?,
            quadratic_coefficient: obj
                .get("quadratic_coefficient")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid quadratic_coefficient"))?,
            ecosystem_treasury: obj
                .get("ecosystem_treasury")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid ecosystem_treasury"))?
                .to_string(),
            max_hold_percent_bps: obj
                .get("max_hold_percent_bps")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid max_hold_percent_bps"))?,
            post_threshold: obj
                .get("post_threshold")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid post_threshold"))?,
            profile_threshold: obj
                .get("profile_threshold")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid profile_threshold"))?,
            max_individual_reservation_bps: obj
                .get("max_individual_reservation_bps")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("Missing or invalid max_individual_reservation_bps"))?,
        })
    }
}

impl ConfigUpdatedEvent {
    /// Convert the event to an exchange config model
    pub fn into_exchange_config_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSptExchangeConfig> {
        Ok(NewSptExchangeConfig {
            updated_by: self.updated_by.clone(),
            post_threshold: self.post_threshold as i64,
            profile_threshold: self.profile_threshold as i64,
            max_individual_reservation_bps: self.max_individual_reservation_bps as i64,
            total_fee_bps: self.total_fee_bps as i64,
            creator_fee_bps: self.creator_fee_bps as i64,
            platform_fee_bps: self.platform_fee_bps as i64,
            treasury_fee_bps: self.treasury_fee_bps as i64,
            base_price: self.base_price as i64,
            quadratic_coefficient: self.quadratic_coefficient as i64,
            ecosystem_treasury: self.ecosystem_treasury.clone(),
            max_hold_percent_bps: self.max_hold_percent_bps as i64,
            trading_halted: false, // Will be set from actual event data
            updated_at: self.timestamp as i64,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            transaction_id,
        })
    }
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
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("Expected object"))?;

        Ok(Self {
            pool_id: obj
                .get("pool_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid pool_id"))?
                .to_string(),
            owner: obj
                .get("owner")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid owner"))?
                .to_string(),
            name: obj
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid name"))?
                .to_string(),
            symbol: obj
                .get("symbol")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid symbol"))?
                .to_string(),
            token_type: obj
                .get("token_type")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid token_type"))?
                as i16,
            associated_id: obj
                .get("associated_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid associated_id"))?
                .to_string(),
            base_price: obj
                .get("base_price")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid base_price"))?,
            quadratic_coefficient: obj
                .get("quadratic_coefficient")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid quadratic_coefficient"))?,
        })
    }
}

impl SocialProofInitPoolEvent {
    pub fn into_model(&self, time: i64, transaction_id: String) -> Result<NewSocialProofTokenPool> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0).unwrap_or_else(|| Utc::now());

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
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("Expected object"))?;

        Ok(Self {
            pool_id: obj
                .get("pool_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid pool_id"))?
                .to_string(),
            buyer: obj
                .get("buyer")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid buyer"))?
                .to_string(),
            amount: obj
                .get("amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid amount"))?,
            mys_amount: obj
                .get("mys_amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid mys_amount"))?,
            fee_amount: obj
                .get("fee_amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid fee_amount"))?,
            creator_fee: obj
                .get("creator_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid creator_fee"))?,
            platform_fee: obj
                .get("platform_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid platform_fee"))?,
            treasury_fee: obj
                .get("treasury_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid treasury_fee"))?,
            price: obj
                .get("price")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid price"))?,
        })
    }
}

impl SocialProofBuyEvent {
    pub fn into_transaction_model(
        &self,
        time: i64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenTransaction> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0).unwrap_or_else(|| Utc::now());

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

    pub fn into_holding_model(
        &self,
        time: i64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenHolding> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0).unwrap_or_else(|| Utc::now());

        Ok(NewSocialProofTokenHolding {
            pool_id: self.pool_id.clone(),
            holder_address: self.buyer.clone(),
            amount: self.amount,
            acquired_at: time,
            time: datetime,
            transaction_id,
        })
    }

    pub fn into_price_history_model(
        &self,
        circulating_supply: i64,
        time: i64,
        transaction_id: String,
    ) -> Result<NewSocialProofPriceHistory> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0).unwrap_or_else(|| Utc::now());

        Ok(NewSocialProofPriceHistory {
            pool_id: self.pool_id.clone(),
            price: self.price,
            circulating_supply,
            time: datetime,
            transaction_id,
        })
    }

    /// Create SPT revenue record for swap fees
    pub fn create_spt_revenue(
        &self,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        time: i64,
        transaction_id: String,
    ) -> Result<crate::models::NewSptRevenue> {
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
    pub fn create_unified_revenue_records(
        &self,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        time: i64,
        transaction_id: String,
    ) -> Result<Vec<crate::models::NewUnifiedRevenue>> {
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
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("Expected object"))?;

        Ok(Self {
            pool_id: obj
                .get("pool_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid pool_id"))?
                .to_string(),
            seller: obj
                .get("seller")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid seller"))?
                .to_string(),
            amount: obj
                .get("amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid amount"))?,
            mys_amount: obj
                .get("mys_amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid mys_amount"))?,
            fee_amount: obj
                .get("fee_amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid fee_amount"))?,
            creator_fee: obj
                .get("creator_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid creator_fee"))?,
            platform_fee: obj
                .get("platform_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid platform_fee"))?,
            treasury_fee: obj
                .get("treasury_fee")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid treasury_fee"))?,
            price: obj
                .get("price")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid price"))?,
        })
    }
}

impl SocialProofSellEvent {
    pub fn into_transaction_model(
        &self,
        time: i64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenTransaction> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0).unwrap_or_else(|| Utc::now());

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

    pub fn into_holding_model(
        &self,
        time: i64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenHolding> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0).unwrap_or_else(|| Utc::now());

        Ok(NewSocialProofTokenHolding {
            pool_id: self.pool_id.clone(),
            holder_address: self.seller.clone(),
            amount: -self.amount, // Negative amount as the seller is selling
            acquired_at: time,
            time: datetime,
            transaction_id,
        })
    }

    pub fn into_price_history_model(
        &self,
        circulating_supply: i64,
        time: i64,
        transaction_id: String,
    ) -> Result<NewSocialProofPriceHistory> {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0).unwrap_or_else(|| Utc::now());

        Ok(NewSocialProofPriceHistory {
            pool_id: self.pool_id.clone(),
            price: self.price,
            circulating_supply,
            time: datetime,
            transaction_id,
        })
    }

    /// Create SPT revenue record for swap fees
    pub fn create_spt_revenue(
        &self,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        time: i64,
        transaction_id: String,
    ) -> Result<crate::models::NewSptRevenue> {
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
    pub fn create_unified_revenue_records(
        &self,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        time: i64,
        transaction_id: String,
    ) -> Result<Vec<crate::models::NewUnifiedRevenue>> {
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

// Reservation created event parsing from Move contract
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialProofReservationCreatedEvent {
    pub associated_id: String,
    pub token_type: i16,
    pub reserver: String,
    pub amount: i64,
    pub total_reserved: i64,
    pub threshold_met: bool,
    pub reserved_at: i64,
}

impl TryFrom<Value> for SocialProofReservationCreatedEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("Expected object"))?;

        Ok(Self {
            associated_id: obj
                .get("associated_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid associated_id"))?
                .to_string(),
            token_type: obj
                .get("token_type")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid token_type"))?
                as i16,
            reserver: obj
                .get("reserver")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid reserver"))?
                .to_string(),
            amount: obj
                .get("amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid amount"))?,
            total_reserved: obj
                .get("total_reserved")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid total_reserved"))?,
            threshold_met: obj
                .get("threshold_met")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            reserved_at: obj
                .get("reserved_at")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid reserved_at"))?,
        })
    }
}

impl SocialProofReservationCreatedEvent {
    pub fn into_reservation_model(
        &self,
        time: i64,
        transaction_id: String,
    ) -> Result<NewSptReservation> {
        let pool_id = format!("reservation_pool_{}", self.associated_id);

        Ok(NewSptReservation {
            pool_id,
            reserver_address: self.reserver.clone(),
            amount: self.amount,
            reserved_at: self.reserved_at,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(time, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            transaction_id,
        })
    }
}

// Reservation withdrawn event parsing from Move contract
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialProofReservationWithdrawnEvent {
    pub associated_id: String,
    pub token_type: i16,
    pub reserver: String,
    pub amount: i64,
    pub total_reserved: i64,
    pub withdrawn_at: i64,
}

impl TryFrom<Value> for SocialProofReservationWithdrawnEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("Expected object"))?;

        Ok(Self {
            associated_id: obj
                .get("associated_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid associated_id"))?
                .to_string(),
            token_type: obj
                .get("token_type")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid token_type"))?
                as i16,
            reserver: obj
                .get("reserver")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid reserver"))?
                .to_string(),
            amount: obj
                .get("amount")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid amount"))?,
            total_reserved: obj
                .get("total_reserved")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid total_reserved"))?,
            withdrawn_at: obj
                .get("withdrawn_at")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid withdrawn_at"))?,
        })
    }
}

impl SocialProofReservationWithdrawnEvent {
    pub fn into_reservation_model(
        &self,
        time: i64,
        transaction_id: String,
    ) -> Result<NewSptReservation> {
        let pool_id = format!("reservation_pool_{}", self.associated_id);

        // For withdrawals, we record the remaining amount (0 means full withdrawal)
        Ok(NewSptReservation {
            pool_id,
            reserver_address: self.reserver.clone(),
            amount: 0, // Represents final amount after withdrawal
            reserved_at: self.withdrawn_at,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(time, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            transaction_id,
        })
    }
}

// Threshold met event parsing from Move contract
#[derive(Debug, Serialize, Deserialize)]
pub struct SocialProofThresholdMetEvent {
    pub associated_id: String,
    pub token_type: i16,
    pub owner: String,
    pub total_reserved: i64,
    pub required_threshold: i64,
    pub timestamp: i64,
}

impl TryFrom<Value> for SocialProofThresholdMetEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("Expected object"))?;

        Ok(Self {
            associated_id: obj
                .get("associated_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid associated_id"))?
                .to_string(),
            token_type: obj
                .get("token_type")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid token_type"))?
                as i16,
            owner: obj
                .get("owner")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid owner"))?
                .to_string(),
            total_reserved: obj
                .get("total_reserved")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid total_reserved"))?,
            required_threshold: obj
                .get("required_threshold")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid required_threshold"))?,
            timestamp: obj
                .get("timestamp")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| anyhow!("Missing or invalid timestamp"))?,
        })
    }
}

impl SocialProofThresholdMetEvent {
    pub fn into_reservation_pool_model(
        &self,
        time: i64,
        transaction_id: String,
    ) -> Result<NewSptReservationPool> {
        let pool_id = format!("reservation_pool_{}", self.associated_id);

        Ok(NewSptReservationPool {
            pool_id,
            associated_id: self.associated_id.clone(),
            token_type: self.token_type,
            owner: self.owner.clone(),
            total_reserved: self.total_reserved,
            required_threshold: self.required_threshold,
            status: RESERVATION_POOL_STATUS_THRESHOLD_MET.to_string(),
            created_at: self.timestamp,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(time, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            transaction_id,
        })
    }
}

/// Event emitted when a post pool is auto-initialized by SPoT flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPoolAutoInitializedEvent {
    pub post_id: String,
    pub owner: String,
    pub base_price: u64,
    pub quadratic_coefficient: u64,
    pub by: String,
}

impl PostPoolAutoInitializedEvent {
    /// Convert the event to a token pool model
    pub fn into_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSocialProofTokenPool> {
        Ok(NewSocialProofTokenPool {
            pool_id: self.post_id.clone(), // Use post_id as pool_id for auto-initialized pools
            token_type: TOKEN_TYPE_POST,
            owner: self.owner.clone(),
            associated_id: self.post_id.clone(),
            symbol: "PPOST".to_string(),
            name: "Post Token".to_string(),
            circulating_supply: 1, // Auto-initialized pools start with 1 token
            base_price: self.base_price as i64,
            quadratic_coefficient: self.quadratic_coefficient as i64,
            created_at: timestamp as i64,
            time: chrono::Utc::now(),
            transaction_id,
        })
    }
}

/// Event emitted when emergency kill switch is toggled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyKillSwitchEvent {
    pub admin: String,
    pub trading_halted: bool,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
    pub reason: String,
}

impl EmergencyKillSwitchEvent {
    /// Convert the event to an exchange config model (for logging kill switch state)
    pub fn into_exchange_config_model(
        &self,
        timestamp: u64,
        transaction_id: String,
    ) -> Result<NewSptExchangeConfig> {
        // Create a minimal config record to log the kill switch event
        // Most fields will be defaults since we're only tracking the kill switch state
        Ok(NewSptExchangeConfig {
            updated_by: self.admin.clone(),
            post_threshold: 0, // Not updated in this event
            profile_threshold: 0, // Not updated in this event
            max_individual_reservation_bps: 0, // Not updated in this event
            total_fee_bps: 0, // Not updated in this event
            creator_fee_bps: 0, // Not updated in this event
            platform_fee_bps: 0, // Not updated in this event
            treasury_fee_bps: 0, // Not updated in this event
            base_price: 0, // Not updated in this event
            quadratic_coefficient: 0, // Not updated in this event
            ecosystem_treasury: "".to_string(), // Not updated in this event
            max_hold_percent_bps: 0, // Not updated in this event
            trading_halted: self.trading_halted,
            updated_at: self.timestamp as i64,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            transaction_id,
        })
    }
}
