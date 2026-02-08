// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use once_cell::sync::Lazy;

use crate::{
    base_types::SequenceNumber, digests::TransactionDigest, error::ExecutionErrorKind,
    execution_status::CongestedObjects, transaction::CheckedInputObjects,
};

pub type ExecutionOrEarlyError = Result<(), ExecutionErrorKind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FundsWithdrawStatus {
    /// Either we don't know yet whether the funds withdrawals are sufficient or not,
    /// or we know for sure that the funds withdrawals are sufficient.
    /// The reason we don't need to distinguish between unknown and sufficient funds is that
    /// in either case we would have to go ahead and execute the transaction anyway.
    MaybeSufficient,
    // TODO(address-funds): Add information on the address and type?
    /// We know for sure that the funds withdrawals in this transaction do not all have enough funds.
    /// This takes account of both address and object funds withdrawals.
    Insufficient,
}

/// Determine if a transaction is predetermined to fail execution.
/// If so, return the error kind, otherwise return `None`.
/// When we pass this to the execution engine, we will not execute the transaction
/// if it is predetermined to fail execution.
pub fn get_early_execution_error(
    transaction_digest: &TransactionDigest,
    input_objects: &CheckedInputObjects,
    config_certificate_deny_set: &HashSet<TransactionDigest>,
    funds_withdraw_status: &FundsWithdrawStatus,
) -> Option<ExecutionErrorKind> {
    if is_certificate_denied(transaction_digest, config_certificate_deny_set) {
        return Some(ExecutionErrorKind::CertificateDenied);
    }

    if input_objects
        .inner()
        .contains_consensus_stream_ended_objects()
    {
        return Some(ExecutionErrorKind::InputObjectDeleted);
    }

    let cancelled_objects = input_objects.inner().get_cancelled_objects();
    if let Some((cancelled_objects, reason)) = cancelled_objects {
        match reason {
            SequenceNumber::CONGESTED => {
                return Some(
                    ExecutionErrorKind::ExecutionCancelledDueToSharedObjectCongestion {
                        congested_objects: CongestedObjects(cancelled_objects),
                    },
                );
            }
            SequenceNumber::RANDOMNESS_UNAVAILABLE => {
                return Some(ExecutionErrorKind::ExecutionCancelledDueToRandomnessUnavailable);
            }
            _ => panic!("invalid cancellation reason SequenceNumber: {reason}"),
        }
    }

    if matches!(funds_withdraw_status, FundsWithdrawStatus::Insufficient) {
        return Some(ExecutionErrorKind::InsufficientFundsForWithdraw);
    }

    None
}

/// If a transaction digest shows up in this list, when executing such transaction,
/// we will always return `ExecutionError::CertificateDenied` without executing it (but still do
/// gas smashing). Because this list is not gated by protocol version, there are a few important
/// criteria for adding a digest to this list:
/// 1. The certificate must be causing all validators to either panic or hang forever deterministically.
/// 2. If we ever ship a fix to make it no longer panic or hang when executing such transaction, we
///    must make sure the transaction is already in this list. Otherwise nodes running the newer
///    version without these transactions in the list will generate forked result.
///
/// Below is a scenario of when we need to use this list:
/// 1. We detect that a specific transaction is causing all validators to either panic or hang forever deterministically.
/// 2. We push a CertificateDenyConfig to deny such transaction to all validators asap.
/// 3. To make sure that all fullnodes are able to sync to the latest version, we need to add the
///    transaction digest to this list as well asap, and ship this binary to all fullnodes, so that
///    they can sync past this transaction.
/// 4. We then can start fixing the issue, and ship the fix to all nodes.
/// 5. Unfortunately, we can't remove the transaction digest from this list, because if we do so,
///    any future node that sync from genesis will fork on this transaction. We may be able to
///    remove it once we have stable snapshots and the binary has a minimum supported protocol
///    version past the epoch.
fn get_denied_certificates() -> &'static HashSet<TransactionDigest> {
    static DENIED_CERTIFICATES: Lazy<HashSet<TransactionDigest>> = Lazy::new(|| HashSet::new());
    Lazy::force(&DENIED_CERTIFICATES)
}

// This is needed to initialize static variables in the simtest environment.
#[cfg(msim)]
pub fn get_denied_certificates_for_sim_test() -> &'static HashSet<TransactionDigest> {
    get_denied_certificates()
}

fn is_certificate_denied(
    transaction_digest: &TransactionDigest,
    certificate_deny_set: &HashSet<TransactionDigest>,
) -> bool {
    certificate_deny_set.contains(transaction_digest)
        || get_denied_certificates().contains(transaction_digest)
}
