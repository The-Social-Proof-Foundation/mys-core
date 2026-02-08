// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use anyhow::bail;
use mys_types::base_types::ObjectRef;
use mys_types::base_types::MysAddress;
use mys_types::effects::TransactionEffects;
use mys_types::effects::TransactionEffectsAPI;
use mys_types::execution_status::ExecutionStatus;
use mys_types::object::Owner;

/// Returns the reference for the first address-owned object created in the effects, or an error if
/// there is none.
pub fn address_owned(fx: &TransactionEffects) -> anyhow::Result<ObjectRef> {
    if let ExecutionStatus::Failure { error, command } = fx.status() {
        bail!("Transaction failed: {error} (command {command:?})");
    }

    fx.created()
        .into_iter()
        .find_map(|(oref, owner)| matches!(owner, Owner::AddressOwner(_)).then_some(oref))
        .context("Could not find created object")
}

/// Returns the reference for the first address-owned object created in the effects owned by
/// `owner`, or an error if there is none.
pub fn address_owned_by(fx: &TransactionEffects, owner: MysAddress) -> anyhow::Result<ObjectRef> {
    if let ExecutionStatus::Failure { error, command } = fx.status() {
        bail!("Transaction failed: {error} (command {command:?})");
    }

    fx.created()
        .into_iter()
        .find_map(|(oref, o)| {
            matches!(o, Owner::AddressOwner(addr) if addr == owner).then_some(oref)
        })
        .context("Could not find created object")
}

/// Returns the reference for the first immutable object created in the effects, or an error if
/// there is none.
pub fn immutable(fx: &TransactionEffects) -> anyhow::Result<ObjectRef> {
    if let ExecutionStatus::Failure { error, command } = fx.status() {
        bail!("Transaction failed: {error} (command {command:?})");
    }

    fx.created()
        .into_iter()
        .find_map(|(oref, owner)| matches!(owner, Owner::Immutable).then_some(oref))
        .context("Could not find created object")
}

/// Returns the reference for the first shared object created in the effects, or an error if there
/// is none.
pub fn shared(fx: &TransactionEffects) -> anyhow::Result<ObjectRef> {
    if let ExecutionStatus::Failure { error, command } = fx.status() {
        bail!("Transaction failed: {error} (command {command:?})");
    }

    fx.created()
        .into_iter()
        .find_map(|(oref, owner)| matches!(owner, Owner::Shared { .. }).then_some(oref))
        .context("Could not find created object")
}

/// Returns the reference for the first address-owned object mutated in the effects that is not a
/// gas payment, or an error if there is none.
pub fn address_mutated(fx: &TransactionEffects) -> anyhow::Result<ObjectRef> {
    if let ExecutionStatus::Failure { error, command } = fx.status() {
        bail!("Transaction failed: {error} (command {command:?})");
    }

    fx.mutated_excluding_gas()
        .into_iter()
        .find_map(|(oref, owner)| matches!(owner, Owner::AddressOwner(_)).then_some(oref))
        .context("Could not find mutated object")
}
