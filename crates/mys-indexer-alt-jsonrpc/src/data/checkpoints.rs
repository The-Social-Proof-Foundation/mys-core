// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::{BTreeSet, HashMap},
    future::Future,
    sync::Arc,
};

use async_graphql::dataloader::Loader;
use diesel::{ExpressionMethods, QueryDsl};
use mys_indexer_alt_schema::{checkpoints::StoredCheckpoint, schema::kv_checkpoints};

use super::reader::{ReadError, Reader};

/// Key for fetching a checkpoint's content by its sequence number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CheckpointKey(pub u64);

impl Loader<CheckpointKey> for Reader {
    type Value = StoredCheckpoint;
    type Error = Arc<ReadError>;

    fn load(
        &self,
        keys: &[CheckpointKey],
    ) -> impl Future<Output = Result<HashMap<CheckpointKey, Self::Value>, Self::Error>> + Send {
        let self_clone = self.clone();
        let keys_vec = keys.to_vec();
        async move {
            use kv_checkpoints::dsl as c;

            if keys_vec.is_empty() {
                return Ok(HashMap::new());
            }

            let mut conn = self_clone.connect().await.map_err(Arc::new)?;

            let seqs: BTreeSet<_> = keys_vec.iter().map(|d| d.0 as i64).collect();
            let checkpoints: Vec<StoredCheckpoint> = conn
                .results(c::kv_checkpoints.filter(c::sequence_number.eq_any(seqs)))
                .await
                .map_err(Arc::new)?;

            Ok(checkpoints
                .into_iter()
                .map(|c| (CheckpointKey(c.sequence_number as u64), c))
                .collect())
        }
    }
}
