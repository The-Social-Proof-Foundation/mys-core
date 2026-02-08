// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeSet;
use std::collections::HashMap;

use anyhow::Context;
use anyhow::anyhow;
use async_graphql::dataloader::Loader;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::Queryable;
use diesel::Selectable;
use diesel::SelectableHelper;
use prost_types::FieldMask;
use mys_indexer_alt_schema::schema::kv_transactions;
use mys_kvstore::TransactionEventsData;
use mys_rpc::field::FieldMaskUtil;
use mys_rpc::proto::proto_to_timestamp_ms;
use mys_rpc::proto::mys::rpc::v2 as proto;
use mys_rpc_api::proto::types::{Digest as ProtoDigest};
use mys_types::digests::TransactionDigest;
use mys_types::effects::TransactionEvents;
use mys_sdk_types::TransactionDigest as SdkTransactionDigest;

use crate::bigtable_reader::BigtableReader;
use crate::error::Error;
use crate::ledger_grpc_reader::LedgerGrpcReader;
use crate::pg_reader::PgReader;

/// Key for fetching transaction events contents (Events, TimestampMs) by digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionEventsKey(pub TransactionDigest);

/// Partial transaction and events for when you only need transaction content for events
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = kv_transactions)]
pub struct StoredTransactionEvents {
    pub events: Vec<u8>,
    pub timestamp_ms: i64,
}

#[async_trait::async_trait]
impl Loader<TransactionEventsKey> for PgReader {
    type Value = StoredTransactionEvents;
    type Error = Error;

    async fn load(
        &self,
        keys: &[TransactionEventsKey],
    ) -> Result<HashMap<TransactionEventsKey, Self::Value>, Self::Error> {
        use kv_transactions::dsl as t;

        if keys.is_empty() {
            return Ok(HashMap::new());
        }

        let mut conn = self.connect().await?;

        let digests: BTreeSet<_> = keys.iter().map(|d| d.0.into_inner()).collect();
        let transactions: Vec<(Vec<u8>, StoredTransactionEvents)> = conn
            .results(
                t::kv_transactions
                    .select((t::tx_digest, StoredTransactionEvents::as_select()))
                    .filter(t::tx_digest.eq_any(digests)),
            )
            .await?;
        let digest_to_stored: HashMap<_, _> = transactions
            .into_iter()
            .map(|(tx_digest, stored)| (tx_digest.clone(), stored))
            .collect();

        Ok(keys
            .iter()
            .filter_map(|key| {
                let slice: &[u8] = key.0.as_ref();
                Some((*key, digest_to_stored.get(slice).cloned()?))
            })
            .collect())
    }
}

#[async_trait::async_trait]
impl Loader<TransactionEventsKey> for BigtableReader {
    type Value = TransactionEventsData;
    type Error = Error;

    async fn load(
        &self,
        keys: &[TransactionEventsKey],
    ) -> Result<HashMap<TransactionEventsKey, Self::Value>, Self::Error> {
        if keys.is_empty() {
            return Ok(HashMap::new());
        }

        let digests: Vec<_> = keys.iter().map(|k| k.0).collect();
        Ok(self
            .transactions_events(&digests)
            .await?
            .into_iter()
            .map(|(digest, events)| (TransactionEventsKey(digest), events))
            .collect())
    }
}

#[async_trait::async_trait]
impl Loader<TransactionEventsKey> for LedgerGrpcReader {
    type Value = TransactionEventsData;
    type Error = Error;

    async fn load(
        &self,
        keys: &[TransactionEventsKey],
    ) -> Result<HashMap<TransactionEventsKey, Self::Value>, Self::Error> {
        if keys.is_empty() {
            return Ok(HashMap::new());
        }

        let mut results = HashMap::new();
        for key in keys {
            let sdk_digest: SdkTransactionDigest = key.0.into();
            // Convert TransactionDigest to Digest for grpc proto
            let proto_digest: ProtoDigest = sdk_digest.into();
            // Construct mys-rpc-api's GetTransactionRequest
            let mut api_request = mys_rpc_api::proto::node::v2::GetTransactionRequest::new(proto_digest);
            api_request.read_mask = Some(FieldMask::from_paths(["events.bcs", "timestamp"]));
            // Encode api_request to bytes and decode as grpc::GetTransactionRequest
            // This works if both use the same proto definition (they should, both use mys.node.v2)
            let api_bytes = prost::Message::encode_to_vec(&api_request);
            let mut request: proto::GetTransactionRequest = prost::Message::decode(api_bytes.as_slice())
                .map_err(|e| Error::from(anyhow::anyhow!("Failed to convert GetTransactionRequest: {}", e)))?;
            // Ensure read_mask is set (it might not decode correctly)
            request.read_mask = Some(FieldMask::from_paths(["events.bcs", "timestamp"]));

            match self.get_transaction(request).await {
                Ok(response) => {
                    let executed = response.transaction.context("No transaction returned")?;

                    let events = executed
                        .events
                        .as_ref()
                        .and_then(|e| e.bcs.as_ref())
                        .map(|bcs| -> anyhow::Result<_> {
                            let tx_events: TransactionEvents = bcs
                                .deserialize()
                                .context("Failed to deserialize transaction events")?;
                            Ok(tx_events.data)
                        })
                        .transpose()?
                        .unwrap_or_default();

                    let timestamp_ms = executed
                        .timestamp
                        .map(proto_to_timestamp_ms)
                        .transpose()
                        .map_err(|e| anyhow!("Failed to parse timestamp: {}", e))?
                        .unwrap_or(0);

                    results.insert(
                        *key,
                        TransactionEventsData {
                            events,
                            timestamp_ms,
                        },
                    );
                }
                Err(status) if status.code() == tonic::Code::NotFound => continue,
                Err(e) => return Err(e.into()),
            }
        }
        Ok(results)
    }
}
