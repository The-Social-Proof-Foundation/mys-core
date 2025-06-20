// Copyright (c) MySocial Team
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::token_exchange_config;

/// Model for token_exchange_config table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = token_exchange_config)]
pub struct TokenExchangeConfig {
    pub id: i32,
    pub trading_halted: bool,
    pub admin_address: String,
    pub reason: String,
    pub timestamp_ms: i64,
    pub updated_at: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for creating new token exchange config entries
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = token_exchange_config)]
pub struct NewTokenExchangeConfig {
    pub trading_halted: bool,
    pub admin_address: String,
    pub reason: String,
    pub timestamp_ms: i64,
    pub updated_at: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for updating token exchange config
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = token_exchange_config)]
pub struct UpdateTokenExchangeConfig {
    pub trading_halted: Option<bool>,
    pub admin_address: Option<String>,
    pub reason: Option<String>,
    pub timestamp_ms: Option<i64>,
    pub updated_at: Option<DateTime<Utc>>,
    pub transaction_id: Option<String>,
} 