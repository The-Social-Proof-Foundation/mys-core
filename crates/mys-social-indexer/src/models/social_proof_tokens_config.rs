// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::social_proof_tokens_config;

/// Model for social_proof_tokens_config table
#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = social_proof_tokens_config)]
pub struct SocialProofTokensConfig {
    pub id: i32,
    pub trading_halted: bool,
    pub admin_address: String,
    pub reason: String,
    pub timestamp_ms: i64,
    pub updated_at: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for creating new social proof tokens config entries
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_proof_tokens_config)]
pub struct NewSocialProofTokensConfig {
    pub trading_halted: bool,
    pub admin_address: String,
    pub reason: String,
    pub timestamp_ms: i64,
    pub updated_at: DateTime<Utc>,
    pub transaction_id: String,
}

/// Model for updating social proof tokens config
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = social_proof_tokens_config)]
pub struct UpdateSocialProofTokensConfig {
    pub trading_halted: Option<bool>,
    pub admin_address: Option<String>,
    pub reason: Option<String>,
    pub timestamp_ms: Option<i64>,
    pub updated_at: Option<DateTime<Utc>>,
    pub transaction_id: Option<String>,
}