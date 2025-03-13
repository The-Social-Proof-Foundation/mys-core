// Copyright (c) The Social Proof Foundation
// SPDX-License-Identifier: Apache-2.0

use diesel_async::AsyncPgConnection;
use mys_types::event::Event;
use mys_types::transaction::TransactionData;

use crate::errors::IndexerError;

/// Trait for implementing an indexing processor
pub trait IndexingProcessor {
    /// Name of the processor for identification
    fn name(&self) -> &'static str;

    /// Process a transaction and its events for indexing
    fn index_tx(
        &self,
        conn: &mut AsyncPgConnection,
        tx: &TransactionData,
        events: &[Event],
        timestamp_ms: u64,
        tx_sequence_number: i64,
        checkpoint_sequence_number: i64,
    ) -> Result<(), IndexerError>;
}