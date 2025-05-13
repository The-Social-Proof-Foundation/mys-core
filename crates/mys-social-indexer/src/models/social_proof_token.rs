// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Token types
pub const TOKEN_TYPE_PROFILE: i16 = 1;
pub const TOKEN_TYPE_POST: i16 = 2;

/// Auction status constants
pub const AUCTION_STATUS_PENDING: i16 = 0;
pub const AUCTION_STATUS_ACTIVE: i16 = 1;
pub const AUCTION_STATUS_FINALIZED: i16 = 2;

/// Transaction types
pub const TRANSACTION_TYPE_BUY: &str = "BUY";
pub const TRANSACTION_TYPE_SELL: &str = "SELL";

/// SocialProofTokenPool represents a token pool in the database
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::social_proof_token_pools)]
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
#[diesel(table_name = crate::schema::social_proof_token_pools)]
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

/// SocialProofAuctionPool represents an auction pool in the database
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::spt_auction_pools)]
#[diesel(primary_key(auction_id, time))]
pub struct SocialProofAuctionPool {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub auction_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub associated_id: String,
    #[diesel(sql_type = diesel::sql_types::SmallInt)]
    pub token_type: i16,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub owner: String,
    #[diesel(sql_type = diesel::sql_types::SmallInt)]
    pub status: i16,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub start_time: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub duration: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub total_contribution: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub total_tokens: i64,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
    pub finalized_at: Option<i64>,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

/// NewSocialProofAuctionPool is used for inserting a new auction pool
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::spt_auction_pools)]
pub struct NewSocialProofAuctionPool {
    pub auction_id: String,
    pub associated_id: String,
    pub token_type: i16,
    pub owner: String,
    pub status: i16,
    pub start_time: i64,
    pub duration: i64,
    pub total_contribution: i64,
    pub total_tokens: i64,
    pub finalized_at: Option<i64>,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// SocialProofAuctionContribution represents an auction contribution in the database
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[diesel(table_name = crate::schema::spt_auction_contributions)]
#[diesel(primary_key(auction_id, contributor_address, time))]
pub struct SocialProofAuctionContribution {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub id: i32,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub auction_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub contributor_address: String,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub amount: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub contributed_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub transaction_id: String,
}

/// NewSocialProofAuctionContribution is used for inserting a new auction contribution
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::spt_auction_contributions)]
pub struct NewSocialProofAuctionContribution {
    pub auction_id: String,
    pub contributor_address: String,
    pub amount: i64,
    pub contributed_at: i64,
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