// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Token types
pub const TOKEN_TYPE_PROFILE: i16 = 1;
pub const TOKEN_TYPE_POST: i16 = 2;

// Reservation pool status constants
pub const RESERVATION_POOL_STATUS_ACTIVE: &str = "active";
pub const RESERVATION_POOL_STATUS_THRESHOLD_MET: &str = "threshold_met";
pub const RESERVATION_POOL_STATUS_CONVERTED: &str = "converted";

/// Transaction types
pub const TRANSACTION_TYPE_BUY: &str = "BUY";
pub const TRANSACTION_TYPE_SELL: &str = "SELL";

/// SocialProofTokenPool represents a token pool in the database
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::spt_pools)]
#[diesel(primary_key(pool_id, time))]
pub struct SocialProofTokenPool {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub pool_id: String,
    #[diesel(sql_type = diesel::sql_types::SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub owner: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub associated_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub symbol: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub name: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub circulating_supply: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub base_price: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub quadratic_coefficient: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

/// NewSocialProofTokenPool is used for inserting a new token pool
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::spt_pools)]
pub struct NewSocialProofTokenPool {
    pub pool_id: String,
    pub token_type: i16,
    pub owner: String,
    pub associated_id: String,
    pub symbol: String,
    pub name: String,
    pub circulating_supply: i64,
    pub base_price: i64,
    pub quadratic_coefficient: i64,
    pub created_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// SocialProofTokenHolding represents a token holding in the database
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::spt_holdings)]
#[diesel(primary_key(pool_id, holder_address, time))]
pub struct SocialProofTokenHolding {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub pool_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub holder_address: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub amount: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub acquired_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

/// NewSocialProofTokenHolding is used for inserting a new token holding
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::spt_holdings)]
pub struct NewSocialProofTokenHolding {
    pub pool_id: String,
    pub holder_address: String,
    pub amount: i64,
    pub acquired_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// SocialProofTokenTransaction represents a token transaction in the database
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::spt_transactions)]
#[diesel(primary_key(pool_id, transaction_id, time))]
pub struct SocialProofTokenTransaction {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub pool_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_type: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub sender: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub amount: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub mys_amount: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub fee_amount: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub creator_fee: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub platform_fee: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub treasury_fee: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub price: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

/// NewSocialProofTokenTransaction is used for inserting a new token transaction
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::spt_transactions)]
pub struct NewSocialProofTokenTransaction {
    pub pool_id: String,
    pub transaction_type: String,
    pub sender: String,
    pub amount: i64,
    pub mys_amount: i64,
    pub fee_amount: i64,
    pub creator_fee: i64,
    pub platform_fee: i64,
    pub treasury_fee: i64,
    pub price: i64,
    pub created_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// SptReservationPool represents a reservation pool in the database
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::spt_reservation_pools)]
#[diesel(primary_key(pool_id, time))]
pub struct SptReservationPool {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub pool_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub associated_id: String,
    #[diesel(sql_type = diesel::sql_types::SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub owner: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub total_reserved: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub required_threshold: i64,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub status: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

/// NewSptReservationPool is used for inserting a new reservation pool
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::spt_reservation_pools)]
pub struct NewSptReservationPool {
    pub pool_id: String,
    pub associated_id: String,
    pub token_type: i16,
    pub owner: String,
    pub total_reserved: i64,
    pub required_threshold: i64,
    pub status: String,
    pub created_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// SptReservation represents an individual reservation in the database
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::spt_reservations)]
#[diesel(primary_key(pool_id, reserver_address, time))]
pub struct SptReservation {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub pool_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub reserver_address: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub amount: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub reserved_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

/// NewSptReservation is used for inserting a new reservation
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::spt_reservations)]
pub struct NewSptReservation {
    pub pool_id: String,
    pub reserver_address: String,
    pub amount: i64,
    pub reserved_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// SocialProofPriceHistory represents a price history entry in the database
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::spt_price_history)]
#[diesel(primary_key(pool_id, time))]
pub struct SocialProofPriceHistory {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub pool_id: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub price: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub circulating_supply: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

/// NewSocialProofPriceHistory is used for inserting a new price history entry
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::spt_price_history)]
pub struct NewSocialProofPriceHistory {
    pub pool_id: String,
    pub price: i64,
    pub circulating_supply: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// SocialProofTokenPoolWithPrice extends SocialProofTokenPool with current price
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
pub struct SocialProofTokenPoolWithPrice {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub pool_id: String,
    #[diesel(sql_type = diesel::sql_types::SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub owner: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub associated_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub symbol: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub name: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub circulating_supply: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub base_price: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub quadratic_coefficient: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub current_price: i64,
}

/// SocialProofPriceAggregation represents price data aggregated over a time period
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
pub struct SocialProofPriceAggregation {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub pool_id: String,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub bucket: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub open: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub high: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub low: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub close: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub circulating_supply: i64,
}

/// PopularTokenPool represents a token pool with popularity metrics
#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
pub struct PopularTokenPool {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub pool_id: String,
    #[diesel(sql_type = diesel::sql_types::SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub owner: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub associated_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub symbol: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub name: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub circulating_supply: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub transaction_count: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub buy_volume: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub sell_volume: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub total_volume: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub current_price: i64,
}

/// SptExchangeConfig represents exchange configuration in the database
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::spt_exchange_config)]
#[diesel(primary_key(id, time))]
pub struct SptExchangeConfig {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub updated_by: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub post_threshold: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub profile_threshold: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub max_individual_reservation_bps: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub total_fee_bps: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub creator_fee_bps: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub platform_fee_bps: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub treasury_fee_bps: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub base_price: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub quadratic_coefficient: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub max_hold_percent_bps: i64,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub trading_enabled: bool,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

/// NewSptExchangeConfig is used for inserting a new exchange config
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::spt_exchange_config)]
pub struct NewSptExchangeConfig {
    pub updated_by: String,
    pub post_threshold: i64,
    pub profile_threshold: i64,
    pub max_individual_reservation_bps: i64,
    pub total_fee_bps: i64,
    pub creator_fee_bps: i64,
    pub platform_fee_bps: i64,
    pub treasury_fee_bps: i64,
    pub base_price: i64,
    pub quadratic_coefficient: i64,
    pub max_hold_percent_bps: i64,
    pub trading_enabled: bool,
    pub updated_at: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// UserTokenHoldings represents a user's holdings across different tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTokenHoldings {
    pub holder_address: String,
    pub holdings: Vec<UserTokenHolding>,
    pub total_value: i64,
}

/// UserTokenHolding represents a user's holding of a specific token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTokenHolding {
    pub pool_id: String,
    pub symbol: String,
    pub name: String,
    pub amount: i64,
    pub current_price: i64,
    pub value: i64,
}
