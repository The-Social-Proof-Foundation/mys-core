// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde_json::Value;
use tracing;

use crate::db::DbConnection;
use crate::schema;

/// Write an event to the relay outbox table for CDC
pub async fn write_to_outbox(
    conn: &mut DbConnection,
    event_type: &str,
    event_data: &Value,
    event_id: Option<&str>,
    transaction_id: Option<&str>,
) -> Result<()> {
    diesel::insert_into(schema::relay_outbox::table)
        .values((
            schema::relay_outbox::event_type.eq(event_type),
            schema::relay_outbox::event_data.eq(event_data),
            schema::relay_outbox::event_id.eq(event_id),
            schema::relay_outbox::transaction_id.eq(transaction_id),
            schema::relay_outbox::created_at.eq(Utc::now()),
        ))
        .execute(conn)
        .await?;

    tracing::debug!("Wrote event to relay_outbox: {}", event_type);
    Ok(())
}

/// Helper to write notification-triggering events to outbox
pub async fn write_notification_event(
    conn: &mut DbConnection,
    event_type: &str,
    event_data: &Value,
    event_id: Option<&str>,
    transaction_id: Option<&str>,
) -> Result<()> {
    write_to_outbox(conn, event_type, event_data, event_id, transaction_id).await
}

