// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::events::event_utils::{
    deserialize_u64_from_string, deserialize_optional_u64_from_string,
    deserialize_i64_from_string,
    parse_i64_from_string_or_number, parse_u64_from_string_or_number,
    parse_optional_u64_from_string_or_number,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::social_proof_token::{
    NewSocialProofPriceHistory, NewSocialProofTokenHolding, NewSocialProofTokenPool,
    NewSocialProofTokenTransaction, NewSptExchangeConfig, NewSptReservation, NewSptReservationPool,
    SptExchangeConfig, RESERVATION_POOL_STATUS_ACTIVE, RESERVATION_POOL_STATUS_THRESHOLD_MET,
    TOKEN_TYPE_POST, TOKEN_TYPE_PROFILE, TRANSACTION_TYPE_BUY, TRANSACTION_TYPE_SELL,
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

/// Event emitted when a reservation pool is created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationPoolCreatedEvent {
    pub associated_id: String,
    pub token_type: u8,
    pub owner: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub required_threshold: u64,
    pub pool_object_id: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub created_at: u64,
}

impl TryFrom<Value> for ReservationPoolCreatedEvent {
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
                .ok_or_else(|| anyhow!("Missing token_type"))
                .and_then(|v| parse_u64_from_string_or_number(v))
                .map(|n| n as u8)?,
            owner: obj
                .get("owner")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid owner"))?
                .to_string(),
            required_threshold: obj
                .get("required_threshold")
                .ok_or_else(|| anyhow!("Missing required_threshold"))
                .and_then(|v| parse_u64_from_string_or_number(v))?,
            pool_object_id: obj
                .get("pool_object_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid pool_object_id"))?
                .to_string(),
            created_at: obj
                .get("created_at")
                .ok_or_else(|| anyhow!("Missing created_at"))
                .and_then(|v| parse_u64_from_string_or_number(v))?,
        })
    }
}

impl ReservationPoolCreatedEvent {
    /// Convert the event to a reservation pool model
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

        // Use pool_object_id directly as pool_id (the actual pool address from blockchain)
        let pool_id = self.pool_object_id.clone();

        Ok(NewSptReservationPool {
            pool_id,
            associated_id: self.associated_id.clone(),
            token_type,
            owner: self.owner.clone(),
            total_reserved: 0,
            required_threshold: self.required_threshold as i64,
            status: RESERVATION_POOL_STATUS_ACTIVE.to_string(),
            created_at: self.created_at as i64,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
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

/// Event emitted when PoC redirection is updated for a token pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocRedirectionUpdatedEvent {
    pub pool_id: String,
    pub post_id: String,
    pub redirect_to: Option<String>,
    pub redirect_percentage: Option<u64>,
    pub updated_by: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
}

impl TryFrom<Value> for PocRedirectionUpdatedEvent {
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
            post_id: obj
                .get("post_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid post_id"))?
                .to_string(),
            redirect_to: obj
                .get("redirect_to")
                .map(|v| {
                    if v.is_null() {
                        None
                    } else {
                        v.as_str().map(|s| s.to_string())
                    }
                })
                .unwrap_or(None),
            redirect_percentage: parse_optional_u64_from_string_or_number(obj.get("redirect_percentage"))?,
            updated_by: obj
                .get("updated_by")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid updated_by"))?
                .to_string(),
            timestamp: obj
                .get("timestamp")
                .ok_or_else(|| anyhow!("Missing timestamp"))
                .and_then(|v| parse_u64_from_string_or_number(v))?,
        })
    }
}

/// Event emitted when exchange config is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdatedEvent {
    pub updated_by: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
    // New separate trading fees (required in new contract)
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub trading_creator_fee_bps: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub trading_platform_fee_bps: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub trading_treasury_fee_bps: u64,
    // New separate reservation fees (required in new contract)
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub reservation_creator_fee_bps: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub reservation_platform_fee_bps: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub reservation_treasury_fee_bps: u64,
    // New field: max_reservers_per_pool
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub max_reservers_per_pool: u64,
    // Old unified fee fields (optional for backward compatibility)
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub total_fee_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub creator_fee_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub platform_fee_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub treasury_fee_bps: Option<u64>,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub base_price: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub quadratic_coefficient: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub max_hold_percent_bps: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub post_threshold: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub profile_threshold: u64,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub max_individual_reservation_bps: u64,
}

impl TryFrom<Value> for ConfigUpdatedEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("Expected object"))?;

        // Helper to parse u64 from string or number
        let parse_u64 = |key: &str| -> Result<u64> {
            obj.get(key)
                .and_then(|v| {
                    if let Some(s) = v.as_str() {
                        s.parse::<u64>().ok()
                    } else {
                        v.as_u64()
                    }
                })
                .ok_or_else(|| anyhow!("Missing or invalid {}", key))
        };

        // Helper to parse optional u64
        let parse_optional_u64 = |key: &str| -> Result<Option<u64>> {
            match obj.get(key) {
                Some(v) => {
                    if v.is_null() {
                        Ok(None)
                    } else if let Some(s) = v.as_str() {
                        s.parse::<u64>().map(Some).map_err(|_| anyhow!("Invalid {} value", key))
                    } else if let Some(n) = v.as_u64() {
                        Ok(Some(n))
                    } else {
                        Ok(None)
                    }
                }
                None => Ok(None),
            }
        };

        Ok(Self {
            updated_by: obj
                .get("updated_by")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid updated_by"))?
                .to_string(),
            timestamp: parse_u64("timestamp")?,
            // New required fields
            trading_creator_fee_bps: parse_u64("trading_creator_fee_bps")?,
            trading_platform_fee_bps: parse_u64("trading_platform_fee_bps")?,
            trading_treasury_fee_bps: parse_u64("trading_treasury_fee_bps")?,
            reservation_creator_fee_bps: parse_u64("reservation_creator_fee_bps")?,
            reservation_platform_fee_bps: parse_u64("reservation_platform_fee_bps")?,
            reservation_treasury_fee_bps: parse_u64("reservation_treasury_fee_bps")?,
            max_reservers_per_pool: parse_u64("max_reservers_per_pool")?,
            // Old optional fields for backward compatibility
            total_fee_bps: parse_optional_u64("total_fee_bps")?,
            creator_fee_bps: parse_optional_u64("creator_fee_bps")?,
            platform_fee_bps: parse_optional_u64("platform_fee_bps")?,
            treasury_fee_bps: parse_optional_u64("treasury_fee_bps")?,
            base_price: parse_u64("base_price")?,
            quadratic_coefficient: parse_u64("quadratic_coefficient")?,
            max_hold_percent_bps: parse_u64("max_hold_percent_bps")?,
            post_threshold: parse_u64("post_threshold")?,
            profile_threshold: parse_u64("profile_threshold")?,
            max_individual_reservation_bps: parse_u64("max_individual_reservation_bps")?,
        })
    }
}

impl ConfigUpdatedEvent {
    /// Convert the event to an exchange config model
    /// Note: Treasury address is no longer stored in config, it's tracked separately in ecosystem_treasury table
    /// Uses values from event when present, falls back to latest DB config if missing
    pub fn into_exchange_config_model(
        &self,
        timestamp: u64,
        transaction_id: String,
        latest_config: Option<&SptExchangeConfig>,
    ) -> Result<NewSptExchangeConfig> {
        // Use new trading/reservation fees from event (required in new contract)
        // For old fields, compute total_fee_bps from new fields if not provided
        let total_fee_bps = self.total_fee_bps.unwrap_or_else(|| {
            self.trading_creator_fee_bps
                + self.trading_platform_fee_bps
                + self.trading_treasury_fee_bps
        }) as i64;
        
        // For backward compatibility, use old fields if new ones are missing (shouldn't happen with new contract)
        // Otherwise use new fields
        let creator_fee_bps = self.creator_fee_bps.unwrap_or(self.trading_creator_fee_bps) as i64;
        let platform_fee_bps = self.platform_fee_bps.unwrap_or(self.trading_platform_fee_bps) as i64;
        let treasury_fee_bps = self.treasury_fee_bps.unwrap_or(self.trading_treasury_fee_bps) as i64;

        Ok(NewSptExchangeConfig {
            updated_by: self.updated_by.clone(),
            post_threshold: self.post_threshold as i64,
            profile_threshold: self.profile_threshold as i64,
            max_individual_reservation_bps: self.max_individual_reservation_bps as i64,
            total_fee_bps,
            creator_fee_bps,
            platform_fee_bps,
            treasury_fee_bps,
            trading_creator_fee_bps: self.trading_creator_fee_bps as i64,
            trading_platform_fee_bps: self.trading_platform_fee_bps as i64,
            trading_treasury_fee_bps: self.trading_treasury_fee_bps as i64,
            reservation_creator_fee_bps: self.reservation_creator_fee_bps as i64,
            reservation_platform_fee_bps: self.reservation_platform_fee_bps as i64,
            reservation_treasury_fee_bps: self.reservation_treasury_fee_bps as i64,
            max_reservers_per_pool: self.max_reservers_per_pool as i64,
            base_price: self.base_price as i64,
            quadratic_coefficient: self.quadratic_coefficient as i64,
            max_hold_percent_bps: self.max_hold_percent_bps as i64,
            trading_enabled: latest_config
                .map(|c| c.trading_enabled)
                .unwrap_or(true), // Preserve existing trading_enabled value, default to true if no config exists
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
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub base_price: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
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
                .ok_or_else(|| anyhow!("Missing token_type"))
                .and_then(|v| parse_i64_from_string_or_number(v))
                .map(|n| n as i16)?,
            associated_id: obj
                .get("associated_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid associated_id"))?
                .to_string(),
            base_price: obj
                .get("base_price")
                .ok_or_else(|| anyhow!("Missing base_price"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            quadratic_coefficient: obj
                .get("quadratic_coefficient")
                .ok_or_else(|| anyhow!("Missing quadratic_coefficient"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
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
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub amount: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub mys_amount: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub fee_amount: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub creator_fee: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub platform_fee: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub treasury_fee: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
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
                .ok_or_else(|| anyhow!("Missing amount"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            mys_amount: obj
                .get("mys_amount")
                .ok_or_else(|| anyhow!("Missing mys_amount"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            fee_amount: obj
                .get("fee_amount")
                .ok_or_else(|| anyhow!("Missing fee_amount"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            creator_fee: obj
                .get("creator_fee")
                .ok_or_else(|| anyhow!("Missing creator_fee"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            platform_fee: obj
                .get("platform_fee")
                .ok_or_else(|| anyhow!("Missing platform_fee"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            treasury_fee: obj
                .get("treasury_fee")
                .ok_or_else(|| anyhow!("Missing treasury_fee"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            price: obj
                .get("price")
                .ok_or_else(|| anyhow!("Missing price"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
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
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub amount: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub mys_amount: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub fee_amount: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub creator_fee: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub platform_fee: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub treasury_fee: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
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
                .ok_or_else(|| anyhow!("Missing amount"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            mys_amount: obj
                .get("mys_amount")
                .ok_or_else(|| anyhow!("Missing mys_amount"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            fee_amount: obj
                .get("fee_amount")
                .ok_or_else(|| anyhow!("Missing fee_amount"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            creator_fee: obj
                .get("creator_fee")
                .ok_or_else(|| anyhow!("Missing creator_fee"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            platform_fee: obj
                .get("platform_fee")
                .ok_or_else(|| anyhow!("Missing platform_fee"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            treasury_fee: obj
                .get("treasury_fee")
                .ok_or_else(|| anyhow!("Missing treasury_fee"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            price: obj
                .get("price")
                .ok_or_else(|| anyhow!("Missing price"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
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
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub amount: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub total_reserved: i64,
    pub threshold_met: bool,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
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
                .ok_or_else(|| anyhow!("Missing token_type"))
                .and_then(|v| parse_i64_from_string_or_number(v))
                .map(|n| n as i16)?,
            reserver: obj
                .get("reserver")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid reserver"))?
                .to_string(),
            amount: obj
                .get("amount")
                .ok_or_else(|| anyhow!("Missing amount"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            total_reserved: obj
                .get("total_reserved")
                .ok_or_else(|| anyhow!("Missing total_reserved"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            threshold_met: obj
                .get("threshold_met")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            reserved_at: obj
                .get("reserved_at")
                .ok_or_else(|| anyhow!("Missing reserved_at"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
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
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub amount: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub total_reserved: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
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
                .ok_or_else(|| anyhow!("Missing token_type"))
                .and_then(|v| parse_i64_from_string_or_number(v))
                .map(|n| n as i16)?,
            reserver: obj
                .get("reserver")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid reserver"))?
                .to_string(),
            amount: obj
                .get("amount")
                .ok_or_else(|| anyhow!("Missing amount"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            total_reserved: obj
                .get("total_reserved")
                .ok_or_else(|| anyhow!("Missing total_reserved"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            withdrawn_at: obj
                .get("withdrawn_at")
                .ok_or_else(|| anyhow!("Missing withdrawn_at"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
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
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub total_reserved: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub required_threshold: i64,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
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
                .ok_or_else(|| anyhow!("Missing token_type"))
                .and_then(|v| parse_i64_from_string_or_number(v))
                .map(|n| n as i16)?,
            owner: obj
                .get("owner")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid owner"))?
                .to_string(),
            total_reserved: obj
                .get("total_reserved")
                .ok_or_else(|| anyhow!("Missing total_reserved"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            required_threshold: obj
                .get("required_threshold")
                .ok_or_else(|| anyhow!("Missing required_threshold"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
            timestamp: obj
                .get("timestamp")
                .ok_or_else(|| anyhow!("Missing timestamp"))
                .and_then(|v| parse_i64_from_string_or_number(v))?,
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
    pub trading_enabled: bool,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub timestamp: u64,
    pub reason: String,
    // Optional config fields - will fallback to latest DB config if missing
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub total_fee_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub creator_fee_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub platform_fee_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub treasury_fee_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub base_price: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub quadratic_coefficient: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub max_hold_percent_bps: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub post_threshold: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub profile_threshold: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
    pub max_individual_reservation_bps: Option<u64>,
}

impl TryFrom<Value> for EmergencyKillSwitchEvent {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value
            .as_object()
            .ok_or_else(|| anyhow!("Expected object"))?;

        // Helper to parse optional u64 from string or number
        let parse_optional_u64 = |key: &str| -> Result<Option<u64>> {
            match obj.get(key) {
                Some(v) => {
                    if let Some(s) = v.as_str() {
                        if s.is_empty() {
                            Ok(None)
                        } else {
                            s.parse::<u64>().map(Some).map_err(|e| anyhow!("Invalid {}: {}", key, e))
                        }
                    } else if let Some(n) = v.as_u64() {
                        Ok(Some(n))
                    } else if v.is_null() {
                        Ok(None)
                    } else {
                        Err(anyhow!("Invalid type for {}", key))
                    }
                }
                None => Ok(None),
            }
        };

        Ok(Self {
            admin: obj
                .get("admin")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing or invalid admin"))?
                .to_string(),
            trading_enabled: obj
                .get("trading_enabled")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| anyhow!("Missing or invalid trading_enabled"))?,
            timestamp: parse_optional_u64("timestamp")?
                .ok_or_else(|| anyhow!("Missing timestamp"))?,
            reason: obj
                .get("reason")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            total_fee_bps: parse_optional_u64("total_fee_bps")?,
            creator_fee_bps: parse_optional_u64("creator_fee_bps")?,
            platform_fee_bps: parse_optional_u64("platform_fee_bps")?,
            treasury_fee_bps: parse_optional_u64("treasury_fee_bps")?,
            base_price: parse_optional_u64("base_price")?,
            quadratic_coefficient: parse_optional_u64("quadratic_coefficient")?,
            max_hold_percent_bps: parse_optional_u64("max_hold_percent_bps")?,
            post_threshold: parse_optional_u64("post_threshold")?,
            profile_threshold: parse_optional_u64("profile_threshold")?,
            max_individual_reservation_bps: parse_optional_u64("max_individual_reservation_bps")?,
        })
    }
}

impl EmergencyKillSwitchEvent {
    /// Convert the event to an exchange config model (for logging kill switch state)
    /// Uses values from event when present, falls back to latest DB config if missing
    pub fn into_exchange_config_model(
        &self,
        timestamp: u64,
        transaction_id: String,
        latest_config: Option<&SptExchangeConfig>,
    ) -> Result<NewSptExchangeConfig> {
        // Helper to get value from event or fallback to latest config
        let get_value = |event_val: Option<u64>, config_val: i64| -> i64 {
            event_val.map(|v| v as i64).unwrap_or(config_val)
        };

        Ok(NewSptExchangeConfig {
            updated_by: self.admin.clone(),
            post_threshold: get_value(
                self.post_threshold,
                latest_config.map(|c| c.post_threshold).unwrap_or(0),
            ),
            profile_threshold: get_value(
                self.profile_threshold,
                latest_config.map(|c| c.profile_threshold).unwrap_or(0),
            ),
            max_individual_reservation_bps: get_value(
                self.max_individual_reservation_bps,
                latest_config
                    .map(|c| c.max_individual_reservation_bps)
                    .unwrap_or(0),
            ),
            total_fee_bps: get_value(
                self.total_fee_bps,
                latest_config.map(|c| c.total_fee_bps).unwrap_or(0),
            ),
            creator_fee_bps: get_value(
                self.creator_fee_bps,
                latest_config.map(|c| c.creator_fee_bps).unwrap_or(0),
            ),
            platform_fee_bps: get_value(
                self.platform_fee_bps,
                latest_config.map(|c| c.platform_fee_bps).unwrap_or(0),
            ),
            treasury_fee_bps: get_value(
                self.treasury_fee_bps,
                latest_config.map(|c| c.treasury_fee_bps).unwrap_or(0),
            ),
            base_price: get_value(
                self.base_price,
                latest_config.map(|c| c.base_price).unwrap_or(0),
            ),
            quadratic_coefficient: get_value(
                self.quadratic_coefficient,
                latest_config.map(|c| c.quadratic_coefficient).unwrap_or(0),
            ),
            max_hold_percent_bps: get_value(
                self.max_hold_percent_bps,
                latest_config.map(|c| c.max_hold_percent_bps).unwrap_or(0),
            ),
            trading_creator_fee_bps: latest_config
                .map(|c| c.trading_creator_fee_bps)
                .unwrap_or(100), // Default from Move contract
            trading_platform_fee_bps: latest_config
                .map(|c| c.trading_platform_fee_bps)
                .unwrap_or(25), // Default from Move contract
            trading_treasury_fee_bps: latest_config
                .map(|c| c.trading_treasury_fee_bps)
                .unwrap_or(25), // Default from Move contract
            reservation_creator_fee_bps: latest_config
                .map(|c| c.reservation_creator_fee_bps)
                .unwrap_or(100), // Default from Move contract
            reservation_platform_fee_bps: latest_config
                .map(|c| c.reservation_platform_fee_bps)
                .unwrap_or(25), // Default from Move contract
            reservation_treasury_fee_bps: latest_config
                .map(|c| c.reservation_treasury_fee_bps)
                .unwrap_or(25), // Default from Move contract
            max_reservers_per_pool: latest_config
                .map(|c| c.max_reservers_per_pool)
                .unwrap_or(1000), // Default from Move contract
            trading_enabled: self.trading_enabled, // Always use event value for trading_enabled
            updated_at: self.timestamp as i64,
            time: chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            transaction_id,
        })
    }
}
