// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::social::db::DbConnection;
use crate::social::schema;

/// EcosystemTreasury represents a treasury address update record
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::ecosystem_treasury)]
pub struct EcosystemTreasury {
    pub id: i32,
    pub treasury_address: String,
    pub updated_by: String,
    pub timestamp_ms: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

/// NewEcosystemTreasury is used for inserting a new treasury update
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::ecosystem_treasury)]
pub struct NewEcosystemTreasury {
    pub treasury_address: String,
    pub updated_by: String,
    pub timestamp_ms: i64,
    pub time: DateTime<Utc>,
    pub transaction_id: String,
}

impl NewEcosystemTreasury {
    /// Create a new treasury record from an event
    pub fn from_event(
        treasury_address: String,
        updated_by: String,
        timestamp_ms: u64,
        transaction_id: String,
    ) -> Self {
        let timestamp_secs = (timestamp_ms / 1000) as i64;
        let time = DateTime::<Utc>::from_timestamp(timestamp_secs, 0)
            .unwrap_or_else(|| Utc::now());

        Self {
            treasury_address,
            updated_by,
            timestamp_ms: timestamp_ms as i64,
            time,
            transaction_id,
        }
    }
}

/// Get the current treasury address from the ecosystem_treasury table
pub async fn get_current_treasury_address(
    conn: &mut DbConnection,
) -> Result<String> {
    let treasury = schema::ecosystem_treasury::table
        .order_by(schema::ecosystem_treasury::time.desc())
        .first::<EcosystemTreasury>(conn)
        .await
        .map_err(|e| anyhow!("Failed to query current treasury address: {}", e))?;

    Ok(treasury.treasury_address)
}

/// Get the current treasury details (full record) from the ecosystem_treasury table
pub async fn get_current_treasury_details(
    conn: &mut DbConnection,
) -> Result<EcosystemTreasury> {
    let treasury = schema::ecosystem_treasury::table
        .order_by(schema::ecosystem_treasury::time.desc())
        .first::<EcosystemTreasury>(conn)
        .await
        .map_err(|e| anyhow!("Failed to query current treasury details: {}", e))?;

    Ok(treasury)
}

/// Get treasury update history
pub async fn get_treasury_history(
    conn: &mut DbConnection,
    limit: i64,
) -> Result<Vec<EcosystemTreasury>> {
    let history = schema::ecosystem_treasury::table
        .order_by(schema::ecosystem_treasury::time.desc())
        .limit(limit)
        .load::<EcosystemTreasury>(conn)
        .await
        .map_err(|e| anyhow!("Failed to query treasury history: {}", e))?;

    Ok(history)
}

