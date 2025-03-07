// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module locked_stake::locked_stake_tests;

use locked_stake::{epoch_time_lock, locked_stake as ls};
use mys::{balance, coin, test_scenario, test_utils::{assert_eq, destroy}, vec_map};
use mys_system::{
    governance_test_utils::{advance_epoch, set_up_mys_system_state},
    mys_system::{Self, MysSystemState}
};

const MIST_PER_MYS: u64 = 1_000_000_000;

#[test]
#[expected_failure(abort_code = epoch_time_lock::EEpochAlreadyPassed)]
fun test_incorrect_creation() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_mys_system_state(vector[@0x1, @0x2, @0x3]);

    // Advance epoch twice so we are now at epoch 2.
    advance_epoch(scenario);
    advance_epoch(scenario);
    let ctx = test_scenario::ctx(scenario);
    assert_eq(tx_context::epoch(ctx), 2);

    // Create a locked stake with epoch 1. Should fail here.
    let ls = ls::new(1, ctx);

    destroy(ls);
    test_scenario::end(scenario_val);
}

#[test]
fun test_deposit_stake_unstake() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_mys_system_state(vector[@0x1, @0x2, @0x3]);

    let mut ls = ls::new(10, test_scenario::ctx(scenario));

    // Deposit 100 MYS.
    ls::deposit_mys(&mut ls, balance::create_for_testing(100 * MIST_PER_MYS));

    assert_eq(ls::mys_balance(&ls), 100 * MIST_PER_MYS);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<MysSystemState>(scenario);

    // Stake 10 of the 100 MYS.
    ls::stake(&mut ls, &mut system_state, 10 * MIST_PER_MYS, @0x1, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);

    assert_eq(ls::mys_balance(&ls), 90 * MIST_PER_MYS);
    assert_eq(vec_map::size(ls::staked_mys(&ls)), 1);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<MysSystemState>(scenario);
    let ctx = test_scenario::ctx(scenario);

    // Create a StakedMys object and add it to the LockedStake object.
    let staked_mys = mys_system::request_add_stake_non_entry(
        &mut system_state,
        coin::mint_for_testing(20 * MIST_PER_MYS, ctx),
        @0x2,
        ctx,
    );
    test_scenario::return_shared(system_state);

    ls::deposit_staked_mys(&mut ls, staked_mys);
    assert_eq(ls::mys_balance(&ls), 90 * MIST_PER_MYS);
    assert_eq(vec_map::size(ls::staked_mys(&ls)), 2);
    advance_epoch(scenario);

    test_scenario::next_tx(scenario, @0x1);
    let (staked_mys_id, _) = vec_map::get_entry_by_idx(ls::staked_mys(&ls), 0);
    let mut system_state = test_scenario::take_shared<MysSystemState>(scenario);

    // Unstake both stake objects
    ls::unstake(&mut ls, &mut system_state, *staked_mys_id, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);
    assert_eq(ls::mys_balance(&ls), 100 * MIST_PER_MYS);
    assert_eq(vec_map::size(ls::staked_mys(&ls)), 1);

    test_scenario::next_tx(scenario, @0x1);
    let (staked_mys_id, _) = vec_map::get_entry_by_idx(ls::staked_mys(&ls), 0);
    let mut system_state = test_scenario::take_shared<MysSystemState>(scenario);
    ls::unstake(&mut ls, &mut system_state, *staked_mys_id, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);
    assert_eq(ls::mys_balance(&ls), 120 * MIST_PER_MYS);
    assert_eq(vec_map::size(ls::staked_mys(&ls)), 0);

    destroy(ls);
    test_scenario::end(scenario_val);
}

#[test]
fun test_unlock_correct_epoch() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_mys_system_state(vector[@0x1, @0x2, @0x3]);

    let mut ls = ls::new(2, test_scenario::ctx(scenario));

    ls::deposit_mys(&mut ls, balance::create_for_testing(100 * MIST_PER_MYS));

    assert_eq(ls::mys_balance(&ls), 100 * MIST_PER_MYS);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<MysSystemState>(scenario);
    ls::stake(&mut ls, &mut system_state, 10 * MIST_PER_MYS, @0x1, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);

    advance_epoch(scenario);
    advance_epoch(scenario);
    advance_epoch(scenario);
    advance_epoch(scenario);

    let (staked_mys, mys_balance) = ls::unlock(ls, test_scenario::ctx(scenario));
    assert_eq(balance::value(&mys_balance), 90 * MIST_PER_MYS);
    assert_eq(vec_map::size(&staked_mys), 1);

    destroy(staked_mys);
    destroy(mys_balance);
    test_scenario::end(scenario_val);
}

#[test]
#[expected_failure(abort_code = epoch_time_lock::EEpochNotYetEnded)]
fun test_unlock_incorrect_epoch() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_mys_system_state(vector[@0x1, @0x2, @0x3]);

    let ls = ls::new(2, test_scenario::ctx(scenario));
    let (staked_mys, mys_balance) = ls::unlock(ls, test_scenario::ctx(scenario));
    destroy(staked_mys);
    destroy(mys_balance);
    test_scenario::end(scenario_val);
}
