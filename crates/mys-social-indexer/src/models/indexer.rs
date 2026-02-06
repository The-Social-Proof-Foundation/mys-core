// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::schema::indexer_progress;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = indexer_progress)]
pub struct IndexerProgress {
    pub id: String,
    pub last_checkpoint_processed: i64,
    pub last_processed_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = indexer_progress)]
pub struct NewIndexerProgress {
    pub id: String,
    pub last_checkpoint_processed: i64,
    pub last_processed_at: NaiveDateTime,
}
