// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module locked_stake::locked_stake;

use locked_stake::epoch_time_lock::{Self, EpochTimeLock};
use mys::{balance::{Self, Balance}, coin, mys::MYS, vec_map::{Self, VecMap}};
use mys_system::{staking_pool::StakedMys, mys_system::{Self, MysSystemState}};

const EInsufficientBalance: u64 = 0;
const EStakeObjectNonExistent: u64 = 1;

/// An object that locks MYS tokens and stake objects until a given epoch, and allows
/// staking and unstaking operations when locked.
public struct LockedStake has key {
    id: UID,
    staked_mys: VecMap<ID, StakedMys>,
    mys: Balance<MYS>,
    locked_until_epoch: EpochTimeLock,
}

// ============================= basic operations =============================

/// Create a new LockedStake object with empty staked_mys and mys balance given a lock time.
/// Aborts if the given epoch has already passed.
public fun new(locked_until_epoch: u64, ctx: &mut TxContext): LockedStake {
    LockedStake {
        id: object::new(ctx),
        staked_mys: vec_map::empty(),
        mys: balance::zero(),
        locked_until_epoch: epoch_time_lock::new(locked_until_epoch, ctx),
    }
}

/// Unlocks and returns all the assets stored inside this LockedStake object.
/// Aborts if the unlock epoch is in the future.
public fun unlock(ls: LockedStake, ctx: &TxContext): (VecMap<ID, StakedMys>, Balance<MYS>) {
    let LockedStake { id, staked_mys, mys, locked_until_epoch } = ls;
    epoch_time_lock::destroy(locked_until_epoch, ctx);
    object::delete(id);
    (staked_mys, mys)
}

/// Deposit a new stake object to the LockedStake object.
public fun deposit_staked_mys(ls: &mut LockedStake, staked_mys: StakedMys) {
    let id = object::id(&staked_mys);
    // This insertion can't abort since each object has a unique id.
    vec_map::insert(&mut ls.staked_mys, id, staked_mys);
}

/// Deposit mys balance to the LockedStake object.
public fun deposit_mys(ls: &mut LockedStake, mys: Balance<MYS>) {
    balance::join(&mut ls.mys, mys);
}

/// Take `amount` of MYS from the mys balance, stakes it, and puts the stake object
/// back into the staked mys vec map.
public fun stake(
    ls: &mut LockedStake,
    mys_system: &mut MysSystemState,
    amount: u64,
    validator_address: address,
    ctx: &mut TxContext,
) {
    assert!(balance::value(&ls.mys) >= amount, EInsufficientBalance);
    let stake = mys_system::request_add_stake_non_entry(
        mys_system,
        coin::from_balance(balance::split(&mut ls.mys, amount), ctx),
        validator_address,
        ctx,
    );
    deposit_staked_mys(ls, stake);
}

/// Unstake the stake object with `staked_mys_id` and puts the resulting principal
/// and rewards back into the locked mys balance.
/// Returns the amount of MYS unstaked, including both principal and rewards.
/// Aborts if no stake exists with the given id.
public fun unstake(
    ls: &mut LockedStake,
    mys_system: &mut MysSystemState,
    staked_mys_id: ID,
    ctx: &mut TxContext,
): u64 {
    assert!(vec_map::contains(&ls.staked_mys, &staked_mys_id), EStakeObjectNonExistent);
    let (_, stake) = vec_map::remove(&mut ls.staked_mys, &staked_mys_id);
    let mys_balance = mys_system::request_withdraw_stake_non_entry(mys_system, stake, ctx);
    let amount = balance::value(&mys_balance);
    deposit_mys(ls, mys_balance);
    amount
}

// ============================= getters =============================

public fun staked_mys(ls: &LockedStake): &VecMap<ID, StakedMys> {
    &ls.staked_mys
}

public fun mys_balance(ls: &LockedStake): u64 {
    balance::value(&ls.mys)
}

public fun locked_until_epoch(ls: &LockedStake): u64 {
    epoch_time_lock::epoch(&ls.locked_until_epoch)
}

// TODO: possibly add some scenarios like switching stake, creating a new LockedStake and transferring
// it to the sender, etc. But these can also be done as PTBs.
