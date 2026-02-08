// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;

use mys_sdk_types::{Address, Object, Version};
use mys_sdk_types::{CheckpointSequenceNumber, EpochId, SignedTransaction, ValidatorCommittee};
use mys_types::balance_change::BalanceChange;
use mys_types::base_types::{ObjectID, ObjectType};
use mys_types::storage::ObjectKey;
use mys_types::storage::RpcStateReader;
use mys_types::storage::error::{Error as StorageError, Result};
use mys_types::storage::{ObjectStore, TransactionInfo};
use tap::Pipe;

use crate::Direction;

#[derive(Clone)]
pub struct StateReader {
    inner: Arc<dyn RpcStateReader>,
}

impl StateReader {
    pub fn new(inner: Arc<dyn RpcStateReader>) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &Arc<dyn RpcStateReader> {
        &self.inner
    }

    #[allow(unused)]
    #[tracing::instrument(skip(self))]
    pub fn get_object(&self, object_id: Address) -> crate::Result<Option<Object>> {
        self.inner
            .get_object(&object_id.into())
            .map(TryInto::try_into)
            .transpose()
            .map_err(Into::into)
    }

    #[allow(unused)]
    #[tracing::instrument(skip(self))]
    pub fn get_object_with_version(
        &self,
        object_id: Address,
        version: Version,
    ) -> crate::Result<Option<Object>> {
        self.inner
            .get_object_by_key(&object_id.into(), version.into())
            .map(TryInto::try_into)
            .transpose()
            .map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    pub fn get_committee(&self, epoch: EpochId) -> Option<ValidatorCommittee> {
        self.inner
            .get_committee(epoch)
            .map(|committee| (*committee).clone().into())
    }

    #[tracing::instrument(skip(self))]
    pub fn get_system_state(&self) -> Result<mys_types::mys_system_state::MysSystemState> {
        mys_types::mys_system_state::get_mys_system_state(self.inner())
            .map_err(StorageError::custom)
            .map_err(StorageError::custom)
    }

    #[tracing::instrument(skip(self))]
    pub fn get_system_state_summary(
        &self,
    ) -> Result<mys_types::mys_system_state::mys_system_state_summary::MysSystemStateSummary> {
        use mys_types::mys_system_state::MysSystemStateTrait;

        let system_state = self.get_system_state()?;
        let summary = system_state.into_mys_system_state_summary();

        Ok(summary)
    }

    pub fn get_authenticator_state(
        &self,
    ) -> Result<Option<mys_types::authenticator_state::AuthenticatorStateInner>> {
        mys_types::authenticator_state::get_authenticator_state(self.inner())
            .map_err(StorageError::custom)
    }

    #[tracing::instrument(skip(self))]
    pub fn get_transaction(
        &self,
        digest: mys_sdk_types::Digest,
    ) -> crate::Result<(
        mys_sdk_types::SignedTransaction,
        mys_types::effects::TransactionEffects,
        Option<mys_types::effects::TransactionEvents>,
    )> {
        use mys_types::effects::TransactionEffectsAPI;

        let transaction_digest = digest.into();

        let transaction = (*self
            .inner()
            .get_transaction(&transaction_digest)
            .ok_or(TransactionNotFoundError(digest))?)
        .clone()
        .into_inner();
        let effects = self
            .inner()
            .get_transaction_effects(&transaction_digest)
            .ok_or(TransactionNotFoundError(digest))?;
        let events = if effects.events_digest().is_some() {
            self.inner()
                .get_events(effects.transaction_digest())
                .ok_or(TransactionNotFoundError(digest))?
                .pipe(Some)
        } else {
            None
        };

        Ok((transaction.try_into()?, effects, events))
    }

    #[tracing::instrument(skip(self))]
    pub fn get_transaction_info(
        &self,
        digest: &mys_types::digests::TransactionDigest,
    ) -> Option<TransactionInfo> {
        self.inner()
            .indexes()?
            .get_transaction_info(digest)
            .ok()
            .flatten()
    }

    #[tracing::instrument(skip(self))]
    pub fn get_transaction_read(
        &self,
        digest: mys_sdk_types::Digest,
    ) -> crate::Result<TransactionRead> {
        let (
            SignedTransaction {
                transaction,
                signatures,
            },
            effects,
            events,
        ) = self.get_transaction(digest)?;

        let (checkpoint, balance_changes, object_types) =
            if let Some(info) = self.get_transaction_info(&(digest.into())) {
                (
                    Some(info.checkpoint),
                    Some(info.balance_changes),
                    Some(info.object_types),
                )
            } else {
                let checkpoint = self.inner().get_transaction_checkpoint(&(digest.into()));
                (checkpoint, None, None)
            };
        let timestamp_ms = if let Some(checkpoint) = checkpoint {
            self.inner()
                .get_checkpoint_by_sequence_number(checkpoint)
                .map(|checkpoint| checkpoint.timestamp_ms)
        } else {
            None
        };

        let unchanged_loaded_runtime_objects = self
            .inner()
            .get_unchanged_loaded_runtime_objects(&(digest.into()));

        Ok(TransactionRead {
            digest: transaction.digest().into(),
            transaction,
            signatures,
            effects,
            events,
            checkpoint,
            timestamp_ms,
            balance_changes,
            object_types,
            unchanged_loaded_runtime_objects,
        })
    }

    #[allow(unused)]
    pub fn checkpoint_iter(
        &self,
        direction: Direction,
        start: CheckpointSequenceNumber,
    ) -> CheckpointIter {
        CheckpointIter::new(self.clone(), direction, start)
    }

    #[allow(unused)]
    pub fn transaction_iter(
        &self,
        direction: Direction,
        cursor: (CheckpointSequenceNumber, Option<usize>),
    ) -> CheckpointTransactionsIter {
        CheckpointTransactionsIter::new(self.clone(), direction, cursor)
    }

    pub fn lookup_address_balance(
        &self,
        owner: mys_types::base_types::MysAddress,
        coin_type: move_core_types::language_storage::StructTag,
    ) -> Option<u64> {
        use mys_types::MoveTypeTagTraitGeneric;
        use mys_types::MYS_ACCUMULATOR_ROOT_OBJECT_ID;
        use mys_types::accumulator_root::{AccumulatorKey, U128};
        use mys_types::dynamic_field::get_dynamic_field_from_store;

        let balance_type = mys_types::balance::Balance::type_tag(coin_type.into());

        let key = AccumulatorKey { owner };
        
        get_dynamic_field_from_store::<AccumulatorKey, U128>(self.inner(), MYS_ACCUMULATOR_ROOT_OBJECT_ID, &key)
            .ok()
            .map(|u128| u128.value as u64)
    }
}

#[derive(Debug)]
pub struct TransactionRead {
    pub digest: mys_sdk_types::Digest,
    pub transaction: mys_sdk_types::Transaction,
    pub signatures: Vec<mys_sdk_types::UserSignature>,
    pub effects: mys_types::effects::TransactionEffects,
    pub events: Option<mys_types::effects::TransactionEvents>,
    pub checkpoint: Option<u64>,
    pub timestamp_ms: Option<u64>,
    pub balance_changes: Option<Vec<BalanceChange>>,
    pub object_types: Option<HashMap<ObjectID, ObjectType>>,
    pub unchanged_loaded_runtime_objects: Option<Vec<ObjectKey>>,
}

pub struct CheckpointTransactionsIter {
    reader: StateReader,
    direction: Direction,

    next_cursor: Option<(CheckpointSequenceNumber, Option<usize>)>,
    checkpoint: Option<(
        mys_types::messages_checkpoint::CheckpointSummary,
        mys_types::messages_checkpoint::CheckpointContents,
    )>,
}

impl CheckpointTransactionsIter {
    #[allow(unused)]
    pub fn new(
        reader: StateReader,
        direction: Direction,
        start: (CheckpointSequenceNumber, Option<usize>),
    ) -> Self {
        Self {
            reader,
            direction,
            next_cursor: Some(start),
            checkpoint: None,
        }
    }
}

impl Iterator for CheckpointTransactionsIter {
    type Item = Result<(CursorInfo, mys_types::digests::TransactionDigest)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (current_checkpoint, transaction_index) = self.next_cursor?;

            let (checkpoint, contents) = if let Some(checkpoint) = &self.checkpoint {
                if checkpoint.0.sequence_number != current_checkpoint {
                    self.checkpoint = None;
                    continue;
                } else {
                    checkpoint
                }
            } else {
                let checkpoint = self
                    .reader
                    .inner()
                    .get_checkpoint_by_sequence_number(current_checkpoint)?;
                let contents = self
                    .reader
                    .inner()
                    .get_checkpoint_contents_by_sequence_number(checkpoint.sequence_number)?;

                self.checkpoint = Some((checkpoint.into_inner().into_data(), contents));
                self.checkpoint.as_ref().unwrap()
            };

            let index = transaction_index
                .map(|idx| idx.clamp(0, contents.size().saturating_sub(1)))
                .unwrap_or_else(|| match self.direction {
                    Direction::Ascending => 0,
                    Direction::Descending => contents.size().saturating_sub(1),
                });

            self.next_cursor = {
                let next_index = match self.direction {
                    Direction::Ascending => {
                        let next_index = index + 1;
                        if next_index >= contents.size() {
                            None
                        } else {
                            Some(next_index)
                        }
                    }
                    Direction::Descending => index.checked_sub(1),
                };

                let next_checkpoint = if next_index.is_some() {
                    Some(current_checkpoint)
                } else {
                    match self.direction {
                        Direction::Ascending => current_checkpoint.checked_add(1),
                        Direction::Descending => current_checkpoint.checked_sub(1),
                    }
                };

                next_checkpoint.map(|checkpoint| (checkpoint, next_index))
            };

            if contents.size() == 0 {
                continue;
            }

            let digest = contents.inner()[index].transaction;

            let cursor_info = CursorInfo {
                checkpoint: checkpoint.sequence_number,
                timestamp_ms: checkpoint.timestamp_ms,
                index: index as u64,
                next_cursor: self.next_cursor,
            };

            return Some(Ok((cursor_info, digest)));
        }
    }
}

#[allow(unused)]
pub struct CursorInfo {
    pub checkpoint: CheckpointSequenceNumber,
    pub timestamp_ms: u64,
    #[allow(unused)]
    pub index: u64,

    // None if there are no more transactions in the store
    pub next_cursor: Option<(CheckpointSequenceNumber, Option<usize>)>,
}

pub struct CheckpointIter {
    reader: StateReader,
    direction: Direction,

    next_cursor: Option<CheckpointSequenceNumber>,
}

impl CheckpointIter {
    #[allow(unused)]
    pub fn new(reader: StateReader, direction: Direction, start: CheckpointSequenceNumber) -> Self {
        Self {
            reader,
            direction,
            next_cursor: Some(start),
        }
    }
}

impl Iterator for CheckpointIter {
    type Item = Result<(
        mys_types::messages_checkpoint::CertifiedCheckpointSummary,
        mys_types::messages_checkpoint::CheckpointContents,
    )>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_checkpoint = self.next_cursor?;

        let checkpoint = self
            .reader
            .inner()
            .get_checkpoint_by_sequence_number(current_checkpoint)?
            .into_inner();
        let contents = self
            .reader
            .inner()
            .get_checkpoint_contents_by_sequence_number(checkpoint.sequence_number)?;

        self.next_cursor = match self.direction {
            Direction::Ascending => current_checkpoint.checked_add(1),
            Direction::Descending => current_checkpoint.checked_sub(1),
        };

        Some(Ok((checkpoint, contents)))
    }
}

#[derive(Debug)]
pub struct TransactionNotFoundError(pub mys_sdk_types::Digest);

impl std::fmt::Display for TransactionNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction {} not found", self.0)
    }
}

impl std::error::Error for TransactionNotFoundError {}

impl From<TransactionNotFoundError> for crate::RpcError {
    fn from(value: TransactionNotFoundError) -> Self {
        Self::new(tonic::Code::NotFound, value.to_string())
    }
}
