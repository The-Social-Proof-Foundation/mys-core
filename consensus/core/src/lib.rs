// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Consensus modules.
mod ancestor;
mod authority_node;
mod authority_service;
mod base_committer;
mod block;
mod block_manager;
mod block_verifier;
mod commit;
mod commit_consumer;
mod commit_finalizer;
mod commit_observer;
mod commit_syncer;
mod commit_vote_monitor;
mod context;
mod core;
mod core_thread;
mod dag_state;
mod error;
mod leader_schedule;
mod leader_scoring;
mod leader_timeout;
mod linearizer;
mod metrics;
mod network;
mod proposed_block_handler;
mod round_prober;
mod round_tracker;
mod stake_aggregator;
pub mod storage;
mod subscriber;
mod synchronizer;
mod threshold_clock;
mod transaction;
mod transaction_certifier;
mod universal_committer;

/// Consensus test utilities.
mod commit_test_fixture;
#[cfg(test)]
mod test_dag;
mod test_dag_builder;
#[cfg(test)]
mod test_dag_parser;

/// Randomized integration tests.
#[cfg(test)]
#[path = "tests/randomized_tests.rs"]
mod randomized_tests;

/// Exported Consensus API.
pub use authority_node::{ConsensusAuthority, NetworkType};
pub use block::{BlockAPI, CertifiedBlock, CertifiedBlocksOutput};
pub use consensus_types::block::{BlockRef, TransactionIndex};

/// Exported API for testing and tools.
pub use block::{TestBlock, Transaction, VerifiedBlock};
pub use commit::{CommitAPI, CommitDigest, CommitIndex, CommitRange, CommitRef, CommittedSubDag};
pub use commit_consumer::{CommitConsumerArgs, CommitConsumerMonitor};
pub use context::Clock;
pub use metrics::Metrics;
pub use network::metrics::NetworkRouteMetrics;
pub use transaction::{
    BlockStatus, ClientError, TransactionClient, TransactionVerifier, ValidationError,
};

/// Connection status for monitoring peer connectivity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

// Exported API for benchmarking
pub use block_verifier::{BlockVerifier, NoopBlockVerifier};
pub use commit_finalizer::CommitFinalizer;
pub use context::Context;
pub use dag_state::DagState;
pub use linearizer::Linearizer;
pub use storage::mem_store::MemStore;
pub use test_dag_builder::DagBuilder;
pub use transaction_certifier::TransactionCertifier;

// Exported API for simtests.
#[cfg(msim)]
pub use block::Slot;
pub use commit_test_fixture::{
    CommitTestFixture, RandomDag, RandomDagConfig, RandomDagIterator, assert_commit_sequences_match,
};
#[cfg(msim)]
pub use network::tonic_network::to_socket_addr;
#[cfg(msim)]
pub use transaction::NoopTransactionVerifier;
