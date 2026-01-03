// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, Result};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::{Database, DbConnection};
use crate::schema::indexer_watermarks;

/// Watermark record for tracking checkpoint progress
#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = indexer_watermarks)]
pub struct IndexerWatermark {
    pub id: i32,
    pub checkpoint_seq: i64,
    pub tx_digest: String,
    pub reader_watermark: Option<i64>,
    pub committer_watermark: Option<i64>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// New watermark record for insertion
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = indexer_watermarks)]
pub struct NewIndexerWatermark {
    pub checkpoint_seq: i64,
    pub tx_digest: String,
    pub reader_watermark: Option<i64>,
    pub committer_watermark: Option<i64>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Watermark manager for tracking ReaderWatermark and CommitterWatermark
pub struct WatermarkManager {
    db: Arc<Database>,
}

impl WatermarkManager {
    /// Create a new watermark manager
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Get a database connection
    async fn get_connection(&self) -> Result<DbConnection> {
        self.db
            .get_connection()
            .await
            .map_err(|e| anyhow!("Failed to get database connection: {}", e))
    }

    /// Get the current reader watermark (last checkpoint read from blockchain)
    pub async fn get_reader_watermark(&self) -> Result<Option<i64>> {
        let mut conn = self.get_connection().await?;

        let watermark = indexer_watermarks::table
            .select(indexer_watermarks::reader_watermark)
            .filter(indexer_watermarks::reader_watermark.is_not_null())
            .order(indexer_watermarks::reader_watermark.desc())
            .limit(1)
            .first::<Option<i64>>(&mut conn)
            .await
            .optional()?;

        Ok(watermark.flatten())
    }

    /// Update the reader watermark (mark checkpoint as read)
    pub async fn update_reader_watermark(
        &self,
        checkpoint_seq: i64,
        tx_digest: &str,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now();

        // Try to update existing record, or insert new one
        let updated = diesel::update(
            indexer_watermarks::table
                .filter(indexer_watermarks::tx_digest.eq(tx_digest)),
        )
        .set((
            indexer_watermarks::checkpoint_seq.eq(checkpoint_seq),
            indexer_watermarks::reader_watermark.eq(checkpoint_seq),
            indexer_watermarks::updated_at.eq(now),
        ))
        .execute(&mut conn)
        .await?;

        if updated == 0 {
            // Insert new record if update didn't affect any rows
            let new_watermark = NewIndexerWatermark {
                checkpoint_seq,
                tx_digest: tx_digest.to_string(),
                reader_watermark: Some(checkpoint_seq),
                committer_watermark: None,
                updated_at: now,
            };

            diesel::insert_into(indexer_watermarks::table)
                .values(&new_watermark)
                .on_conflict(indexer_watermarks::tx_digest)
                .do_update()
                .set((
                    indexer_watermarks::checkpoint_seq.eq(checkpoint_seq),
                    indexer_watermarks::reader_watermark.eq(checkpoint_seq),
                    indexer_watermarks::updated_at.eq(now),
                ))
                .execute(&mut conn)
                .await?;
        }

        Ok(())
    }

    /// Get the current committer watermark (last checkpoint committed to database)
    pub async fn get_committer_watermark(&self) -> Result<Option<i64>> {
        let mut conn = self.get_connection().await?;

        let watermark = indexer_watermarks::table
            .select(indexer_watermarks::committer_watermark)
            .filter(indexer_watermarks::committer_watermark.is_not_null())
            .order(indexer_watermarks::committer_watermark.desc())
            .limit(1)
            .first::<Option<i64>>(&mut conn)
            .await
            .optional()?;

        Ok(watermark.flatten())
    }

    /// Update the committer watermark (mark checkpoint as committed)
    pub async fn update_committer_watermark(
        &self,
        checkpoint_seq: i64,
        tx_digest: &str,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now();

        // Try to update existing record, or insert new one
        let updated = diesel::update(
            indexer_watermarks::table
                .filter(indexer_watermarks::tx_digest.eq(tx_digest)),
        )
        .set((
            indexer_watermarks::checkpoint_seq.eq(checkpoint_seq),
            indexer_watermarks::committer_watermark.eq(checkpoint_seq),
            indexer_watermarks::updated_at.eq(now),
        ))
        .execute(&mut conn)
        .await?;

        if updated == 0 {
            // Insert new record if update didn't affect any rows
            let new_watermark = NewIndexerWatermark {
                checkpoint_seq,
                tx_digest: tx_digest.to_string(),
                reader_watermark: None,
                committer_watermark: Some(checkpoint_seq),
                updated_at: now,
            };

            diesel::insert_into(indexer_watermarks::table)
                .values(&new_watermark)
                .on_conflict(indexer_watermarks::tx_digest)
                .do_update()
                .set((
                    indexer_watermarks::checkpoint_seq.eq(checkpoint_seq),
                    indexer_watermarks::committer_watermark.eq(checkpoint_seq),
                    indexer_watermarks::updated_at.eq(now),
                ))
                .execute(&mut conn)
                .await?;
        }

        Ok(())
    }

    /// Update both watermarks atomically (for a single transaction)
    pub async fn update_both_watermarks(
        &self,
        checkpoint_seq: i64,
        tx_digest: &str,
        reader_watermark: Option<i64>,
        committer_watermark: Option<i64>,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let now = Utc::now();

        let new_watermark = NewIndexerWatermark {
            checkpoint_seq,
            tx_digest: tx_digest.to_string(),
            reader_watermark,
            committer_watermark,
            updated_at: now,
        };

        diesel::insert_into(indexer_watermarks::table)
            .values(&new_watermark)
            .on_conflict(indexer_watermarks::tx_digest)
            .do_update()
            .set((
                indexer_watermarks::checkpoint_seq.eq(checkpoint_seq),
                indexer_watermarks::reader_watermark.eq(reader_watermark),
                indexer_watermarks::committer_watermark.eq(committer_watermark),
                indexer_watermarks::updated_at.eq(now),
            ))
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

