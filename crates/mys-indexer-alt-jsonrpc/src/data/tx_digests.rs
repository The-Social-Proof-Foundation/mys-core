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
use mys_indexer_alt_schema::{schema::tx_digests, transactions::StoredTxDigest};

use super::reader::{ReadError, Reader};

/// Key for fetching a transaction's digest by its sequence number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TxDigestKey(pub u64);

impl Loader<TxDigestKey> for Reader {
    type Value = StoredTxDigest;
    type Error = Arc<ReadError>;

    fn load(
        &self,
        keys: &[TxDigestKey],
    ) -> impl Future<Output = Result<HashMap<TxDigestKey, Self::Value>, Self::Error>> + Send {
        let self_clone = self.clone();
        let keys_vec = keys.to_vec();
        async move {
            use tx_digests::dsl as d;

            if keys_vec.is_empty() {
                return Ok(HashMap::new());
            }

            let mut conn = self_clone.connect().await.map_err(Arc::new)?;

            let seqs: BTreeSet<_> = keys_vec.iter().map(|d| d.0 as i64).collect();
            let stored: Vec<StoredTxDigest> = conn
                .results(d::tx_digests.filter(d::tx_sequence_number.eq_any(seqs)))
                .await
                .map_err(Arc::new)?;

            Ok(stored
                .into_iter()
                .map(|stored| {
                    let key = TxDigestKey(stored.tx_sequence_number as u64);
                    (key, stored)
                })
                .collect())
        }
    }
}
