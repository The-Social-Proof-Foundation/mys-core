// SPDX-License-Identifier: Apache-2.0

use crate::social::schema::wallet_social_graph;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Model for wallet social graph counts (for wallet addresses without profiles)
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = wallet_social_graph)]
pub struct WalletSocialGraph {
    pub wallet_address: String,
    pub followers_count: i32,
    pub following_count: i32,
    pub blocked_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// DTO for creating a new wallet social graph entry
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = wallet_social_graph)]
pub struct NewWalletSocialGraph {
    pub wallet_address: String,
    pub followers_count: i32,
    pub following_count: i32,
    pub blocked_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
