// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::balance::Balance;
use crate::base_types::MysAddress;
use crate::coin::Coin;
use crate::effects::{
    AccumulatorOperation, AccumulatorValue, TransactionEffects, TransactionEffectsAPI,
};
use crate::full_checkpoint_content::ObjectSet;
use crate::object::Object;
use crate::object::Owner;
use crate::storage::ObjectKey;
use crate::TypeTag;

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BalanceChange {
    /// Owner of the balance change
    pub address: MysAddress,

    /// Type of the Coin
    pub coin_type: TypeTag,

    /// The amount indicate the balance value changes.
    ///
    /// A negative amount means spending coin value and positive means receiving coin value.
    pub amount: i128,
}

impl std::fmt::Debug for BalanceChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BalanceChange")
            .field("address", &self.address)
            .field("coin_type", &self.coin_type.to_canonical_string(true))
            .field("amount", &self.amount)
            .finish()
    }
}

fn coins(objects: &[Object]) -> impl Iterator<Item = (&MysAddress, TypeTag, u64)> + '_ {
    objects.iter().filter_map(|object| {
        let address = match object.owner() {
            Owner::AddressOwner(mys_address)
            | Owner::ObjectOwner(mys_address)
            | Owner::ConsensusAddressOwner {
                owner: mys_address, ..
            } => mys_address,
            Owner::Shared { .. } | Owner::Immutable => return None,
        };
        let (coin_type, balance) = Coin::extract_balance_if_coin(object).ok().flatten()?;
        Some((address, coin_type, balance))
    })
}

/// Extract balance changes from accumulator events that have `Balance<T>` types.
/// Returns an iterator of (address, coin_type, signed_amount) tuples.
pub fn address_balance_changes_from_accumulator_events(
    effects: &TransactionEffects,
) -> impl Iterator<Item = (MysAddress, TypeTag, i128)> + '_ {
    effects
        .accumulator_events()
        .into_iter()
        .filter_map(|event| {
            let ty = &event.write.address.ty;
            // Only process events with Balance<T> types
            let coin_type = Balance::maybe_get_balance_type_param(ty)?;

            let amount = match &event.write.value {
                AccumulatorValue::Integer(v) => *v as i128,
                // IntegerTuple and EventDigest are not balance-related
                AccumulatorValue::IntegerTuple(_, _) | AccumulatorValue::EventDigest(_) => {
                    return None;
                }
            };

            // Convert operation to signed amount: Split means balance decreased, Merge means increased
            let signed_amount = match event.write.operation {
                AccumulatorOperation::Split => -amount,
                AccumulatorOperation::Merge => amount,
            };

            Some((event.write.address.address, coin_type, signed_amount))
        })
}

pub fn derive_balance_changes(
    effects: &TransactionEffects,
    input_objects: &[Object],
    output_objects: &[Object],
) -> Vec<BalanceChange> {
    // 1. subtract all input coins
    let balances = coins(input_objects).fold(
        std::collections::BTreeMap::<_, i128>::new(),
        |mut acc, (address, coin_type, balance)| {
            *acc.entry((*address, coin_type)).or_default() -= balance as i128;
            acc
        },
    );

    // 2. add all mutated/output coins
    let balances =
        coins(output_objects).fold(balances, |mut acc, (address, coin_type, balance)| {
            *acc.entry((*address, coin_type)).or_default() += balance as i128;
            acc
        });

    // 3. add address balance changes from accumulator events
    let balances = address_balance_changes_from_accumulator_events(effects).fold(
        balances,
        |mut acc, (address, coin_type, signed_amount)| {
            *acc.entry((address, coin_type)).or_default() += signed_amount;
            acc
        },
    );

    balances
        .into_iter()
        .filter_map(|((address, coin_type), amount)| {
            if amount == 0 {
                return None;
            }

            Some(BalanceChange {
                address,
                coin_type,
                amount,
            })
        })
        .collect()
}

pub fn derive_balance_changes_2(
    effects: &TransactionEffects,
    objects: &ObjectSet,
) -> Vec<BalanceChange> {
    let input_objects = effects
        .modified_at_versions()
        .into_iter()
        .filter_map(|(object_id, version)| objects.get(&ObjectKey(object_id, version)).cloned())
        .collect::<Vec<_>>();
    let output_objects = effects
        .all_changed_objects()
        .into_iter()
        .filter_map(|(object_ref, _owner, _kind)| objects.get(&object_ref.into()).cloned())
        .collect::<Vec<_>>();

    // 1. subtract all input coins
    let balances = coins(&input_objects).fold(
        std::collections::BTreeMap::<_, i128>::new(),
        |mut acc, (address, coin_type, balance)| {
            *acc.entry((*address, coin_type)).or_default() -= balance as i128;
            acc
        },
    );

    // 2. add all mutated/output coins
    let balances =
        coins(&output_objects).fold(balances, |mut acc, (address, coin_type, balance)| {
            *acc.entry((*address, coin_type)).or_default() += balance as i128;
            acc
        });

    // 3. add address balance changes from accumulator events
    let balances = address_balance_changes_from_accumulator_events(effects).fold(
        balances,
        |mut acc, (address, coin_type, signed_amount)| {
            *acc.entry((address, coin_type)).or_default() += signed_amount;
            acc
        },
    );

    balances
        .into_iter()
        .filter_map(|((address, coin_type), amount)| {
            if amount == 0 {
                return None;
            }

            Some(BalanceChange {
                address,
                coin_type,
                amount,
            })
        })
        .collect()
}
