// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module mys_system::staking_pool_tests {
    use mys::test_scenario::{Self, Scenario};
    use mys_system::staking_pool::{StakingPool, Self};
    use mys::balance::{Self};

    #[test]
    fun test_join_fungible_staked_mys_happy() {
        let mut scenario = test_scenario::begin(@0x0);
        let staking_pool = staking_pool::new(scenario.ctx());

        let mut fungible_staked_mys_1 = staking_pool.create_fungible_staked_mys_for_testing(100_000_000_000, scenario.ctx());
        let fungible_staked_mys_2 = staking_pool.create_fungible_staked_mys_for_testing(200_000_000_000, scenario.ctx());

        fungible_staked_mys_1.join(fungible_staked_mys_2);

        assert!(fungible_staked_mys_1.value() == 300_000_000_000, 0);

        mys::test_utils::destroy(staking_pool);
        mys::test_utils::destroy(fungible_staked_mys_1);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = 1, location = mys_system::staking_pool)]
    fun test_join_fungible_staked_mys_fail() {
        let mut scenario = test_scenario::begin(@0x0);
        let staking_pool_1 = staking_pool::new(scenario.ctx());
        let staking_pool_2 = staking_pool::new(scenario.ctx());

        let mut fungible_staked_mys_1 = staking_pool_1.create_fungible_staked_mys_for_testing(100_000_000_000, scenario.ctx());
        let fungible_staked_mys_2 = staking_pool_2.create_fungible_staked_mys_for_testing(200_000_000_000, scenario.ctx());

        fungible_staked_mys_1.join(fungible_staked_mys_2);

        mys::test_utils::destroy(staking_pool_1);
        mys::test_utils::destroy(staking_pool_2);
        mys::test_utils::destroy(fungible_staked_mys_1);

        scenario.end();
    }

    #[test]
    fun test_split_fungible_staked_mys_happy() {
        let mut scenario = test_scenario::begin(@0x0);
        let staking_pool = staking_pool::new(scenario.ctx());

        let mut fungible_staked_mys_1 = staking_pool.create_fungible_staked_mys_for_testing(100_000_000_000, scenario.ctx());

        let fungible_staked_mys_2 = fungible_staked_mys_1.split(75_000_000_000, scenario.ctx());

        assert!(fungible_staked_mys_1.value() == 25_000_000_000, 0);
        assert!(fungible_staked_mys_2.value() == 75_000_000_000, 0);

        mys::test_utils::destroy(staking_pool);
        mys::test_utils::destroy(fungible_staked_mys_1);
        mys::test_utils::destroy(fungible_staked_mys_2);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = 0, location = mys_system::staking_pool)]
    fun test_split_fungible_staked_mys_fail_too_much() {
        let mut scenario = test_scenario::begin(@0x0);
        let staking_pool = staking_pool::new(scenario.ctx());

        let mut fungible_staked_mys_1 = staking_pool.create_fungible_staked_mys_for_testing(100_000_000_000, scenario.ctx());

        let fungible_staked_mys_2 = fungible_staked_mys_1.split(100_000_000_000 + 1, scenario.ctx());

        mys::test_utils::destroy(staking_pool);
        mys::test_utils::destroy(fungible_staked_mys_1);
        mys::test_utils::destroy(fungible_staked_mys_2);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = 19, location = mys_system::staking_pool)]
    fun test_convert_to_fungible_staked_mys_fail_too_early() {
        let mut scenario = test_scenario::begin(@0x0);
        let mut staking_pool = staking_pool::new(scenario.ctx());

        let mys = balance::create_for_testing(1_000_000_000);
        let staked_mys = staking_pool.request_add_stake(mys, scenario.ctx().epoch() + 1, scenario.ctx());
        let fungible_staked_mys = staking_pool.convert_to_fungible_staked_mys(staked_mys, scenario.ctx());

        mys::test_utils::destroy(staking_pool);
        mys::test_utils::destroy(fungible_staked_mys);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = 1, location = mys_system::staking_pool)]
    fun test_convert_to_fungible_staked_mys_fail_wrong_pool() {
        let mut scenario = test_scenario::begin(@0x0);
        let mut staking_pool_1 = staking_pool::new(scenario.ctx());
        let mut staking_pool_2 = staking_pool::new(scenario.ctx());

        let mys = balance::create_for_testing(1_000_000_000);
        let staked_mys = staking_pool_1.request_add_stake(mys, scenario.ctx().epoch() + 1, scenario.ctx());

        let fungible_staked_mys = staking_pool_2.convert_to_fungible_staked_mys(staked_mys, scenario.ctx());

        mys::test_utils::destroy(staking_pool_1);
        mys::test_utils::destroy(staking_pool_2);
        mys::test_utils::destroy(fungible_staked_mys);

        scenario.end();
    }

    #[test]
    fun test_convert_to_fungible_staked_mys_happy() {
        let mut scenario = test_scenario::begin(@0x0);
        let mut staking_pool = staking_pool::new(scenario.ctx());
        staking_pool.activate_staking_pool(0);

        // setup

        let mys = balance::create_for_testing(1_000_000_000);
        let staked_mys_1 = staking_pool.request_add_stake(mys, scenario.ctx().epoch() + 1, scenario.ctx());

        assert!(distribute_rewards_and_advance_epoch(&mut staking_pool, &mut scenario, 0) == 1, 0);

        let latest_exchange_rate = staking_pool.pool_token_exchange_rate_at_epoch(1);
        assert!(latest_exchange_rate.mys_amount() == 1_000_000_000, 0);
        assert!(latest_exchange_rate.pool_token_amount() == 1_000_000_000, 0);

        let mys = balance::create_for_testing(1_000_000_000);
        let staked_mys_2 = staking_pool.request_add_stake(mys, scenario.ctx().epoch() + 1, scenario.ctx());

        assert!(distribute_rewards_and_advance_epoch(&mut staking_pool, &mut scenario, 1_000_000_000) == 2, 0);

        let latest_exchange_rate = staking_pool.pool_token_exchange_rate_at_epoch(2);
        assert!(latest_exchange_rate.mys_amount() == 3_000_000_000, 0);
        assert!(latest_exchange_rate.pool_token_amount() == 1_500_000_000, 0);

        // test basically starts from here.

        let fungible_staked_mys_1 = staking_pool.convert_to_fungible_staked_mys(staked_mys_1, scenario.ctx());
        assert!(fungible_staked_mys_1.value() == 1_000_000_000, 0);
        assert!(fungible_staked_mys_1.pool_id() == object::id(&staking_pool), 0);

        let fungible_staked_mys_data = staking_pool.fungible_staked_mys_data();
        assert!(fungible_staked_mys_data.total_supply() == 1_000_000_000, 0);
        assert!(fungible_staked_mys_data.principal_value() == 1_000_000_000, 0);

        let fungible_staked_mys_2 = staking_pool.convert_to_fungible_staked_mys(staked_mys_2, scenario.ctx());
        assert!(fungible_staked_mys_2.value() == 500_000_000, 0);
        assert!(fungible_staked_mys_2.pool_id() == object::id(&staking_pool), 0);

        let fungible_staked_mys_data = staking_pool.fungible_staked_mys_data();
        assert!(fungible_staked_mys_data.total_supply() == 1_500_000_000, 0);
        assert!(fungible_staked_mys_data.principal_value() == 2_000_000_000, 0);

        mys::test_utils::destroy(staking_pool);
        // mys::test_utils::destroy(fungible_staked_mys);
        mys::test_utils::destroy(fungible_staked_mys_1);
        mys::test_utils::destroy(fungible_staked_mys_2);

        scenario.end();
    }

    #[test]
    fun test_redeem_fungible_staked_mys_happy() {
        let mut scenario = test_scenario::begin(@0x0);
        let mut staking_pool = staking_pool::new(scenario.ctx());
        staking_pool.activate_staking_pool(0);

        // setup

        let mys = balance::create_for_testing(1_000_000_000);
        let staked_mys_1 = staking_pool.request_add_stake(mys, scenario.ctx().epoch() + 1, scenario.ctx());

        assert!(distribute_rewards_and_advance_epoch(&mut staking_pool, &mut scenario, 0) == 1, 0);

        let latest_exchange_rate = staking_pool.pool_token_exchange_rate_at_epoch(1);
        assert!(latest_exchange_rate.mys_amount() == 1_000_000_000, 0);
        assert!(latest_exchange_rate.pool_token_amount() == 1_000_000_000, 0);

        let mys = balance::create_for_testing(1_000_000_000);
        let staked_mys_2 = staking_pool.request_add_stake(mys, scenario.ctx().epoch() + 1, scenario.ctx());

        assert!(distribute_rewards_and_advance_epoch(&mut staking_pool, &mut scenario, 1_000_000_000) == 2, 0);

        let latest_exchange_rate = staking_pool.pool_token_exchange_rate_at_epoch(2);
        assert!(latest_exchange_rate.mys_amount() == 3_000_000_000, 0);
        assert!(latest_exchange_rate.pool_token_amount() == 1_500_000_000, 0);

        let fungible_staked_mys_1 = staking_pool.convert_to_fungible_staked_mys(staked_mys_1, scenario.ctx());
        assert!(fungible_staked_mys_1.value() == 1_000_000_000, 0);
        assert!(fungible_staked_mys_1.pool_id() == object::id(&staking_pool), 0);

        let fungible_staked_mys_data = staking_pool.fungible_staked_mys_data();
        assert!(fungible_staked_mys_data.total_supply() == 1_000_000_000, 0);
        assert!(fungible_staked_mys_data.principal_value() == 1_000_000_000, 0);

        let fungible_staked_mys_2 = staking_pool.convert_to_fungible_staked_mys(staked_mys_2, scenario.ctx());
        assert!(fungible_staked_mys_2.value() == 500_000_000, 0);
        assert!(fungible_staked_mys_2.pool_id() == object::id(&staking_pool), 0);

        let fungible_staked_mys_data = staking_pool.fungible_staked_mys_data();
        assert!(fungible_staked_mys_data.total_supply() == 1_500_000_000, 0);
        assert!(fungible_staked_mys_data.principal_value() == 2_000_000_000, 0);

        // test starts here
        assert!(distribute_rewards_and_advance_epoch(&mut staking_pool, &mut scenario, 3_000_000_000) == 3, 0);

        let latest_exchange_rate = staking_pool.pool_token_exchange_rate_at_epoch(3);
        assert!(latest_exchange_rate.mys_amount() == 6_000_000_000, 0);
        assert!(latest_exchange_rate.pool_token_amount() == 1_500_000_000, 0);

        assert!(staking_pool.pending_stake_withdraw_amount() == 0, 0);
        assert!(staking_pool.pending_pool_token_withdraw_amount() == 0, 0);

        let mys_1 = staking_pool.redeem_fungible_staked_mys(fungible_staked_mys_1, scenario.ctx());
        assert!(mys_1.value() <= 4_000_000_000, 0);
        assert!(mys_1.value() == 4_000_000_000 - 1, 0);

        let fungible_staked_mys_data = staking_pool.fungible_staked_mys_data();
        assert!(fungible_staked_mys_data.total_supply() == 500_000_000, 0);
        assert!(fungible_staked_mys_data.principal_value() == 2_000_000_000 / 3 + 1, 0); // round against user

        assert!(staking_pool.pending_stake_withdraw_amount() == 4_000_000_000 - 1, 0);
        assert!(staking_pool.pending_pool_token_withdraw_amount() == 1_000_000_000, 0);

        let mys_2 = staking_pool.redeem_fungible_staked_mys(fungible_staked_mys_2, scenario.ctx());
        assert!(mys_2.value() == 2_000_000_000, 0);

        let fungible_staked_mys_data = staking_pool.fungible_staked_mys_data();
        assert!(fungible_staked_mys_data.total_supply() == 0, 0);
        assert!(fungible_staked_mys_data.principal_value() == 0, 0);

        assert!(staking_pool.pending_stake_withdraw_amount() == 6_000_000_000 - 1, 0);
        assert!(staking_pool.pending_pool_token_withdraw_amount() == 1_500_000_000, 0);

        mys::test_utils::destroy(staking_pool);
        mys::test_utils::destroy(mys_1);
        mys::test_utils::destroy(mys_2);

        scenario.end();
    }

    #[test]
    fun test_redeem_fungible_staked_mys_regression_rounding() {
        let mut scenario = test_scenario::begin(@0x0);
        let mut staking_pool = staking_pool::new(scenario.ctx());
        staking_pool.activate_staking_pool(0);

        // setup

        let mys = balance::create_for_testing(1_000_000_000);
        let staked_mys_1 = staking_pool.request_add_stake(mys, scenario.ctx().epoch() + 1, scenario.ctx());

        assert!(distribute_rewards_and_advance_epoch(&mut staking_pool, &mut scenario, 0) == 1, 0);

        let latest_exchange_rate = staking_pool.pool_token_exchange_rate_at_epoch(1);
        assert!(latest_exchange_rate.mys_amount() == 1_000_000_000, 0);
        assert!(latest_exchange_rate.pool_token_amount() == 1_000_000_000, 0);

        let mys = balance::create_for_testing(1_000_000_001);
        let staked_mys_2 = staking_pool.request_add_stake(mys, scenario.ctx().epoch() + 1, scenario.ctx());

        assert!(distribute_rewards_and_advance_epoch(&mut staking_pool, &mut scenario, 1_000_000_000) == 2, 0);

        let latest_exchange_rate = staking_pool.pool_token_exchange_rate_at_epoch(2);
        assert!(latest_exchange_rate.mys_amount() == 3_000_000_001, 0);
        assert!(latest_exchange_rate.pool_token_amount() == 1_500_000_000, 0);

        let fungible_staked_mys = staking_pool.convert_to_fungible_staked_mys(staked_mys_2, scenario.ctx());
        assert!(fungible_staked_mys.value() == 500_000_000, 0); // rounding!
        assert!(fungible_staked_mys.pool_id() == object::id(&staking_pool), 0);

        let fungible_staked_mys_data = staking_pool.fungible_staked_mys_data();
        assert!(fungible_staked_mys_data.total_supply() == 500_000_000, 0);
        assert!(fungible_staked_mys_data.principal_value() == 1_000_000_001, 0);

        // this line used to error
        let mys = staking_pool.redeem_fungible_staked_mys(fungible_staked_mys, scenario.ctx());
        assert!(mys.value() == 1_000_000_000, 0);

        let fungible_staked_mys_data = staking_pool.fungible_staked_mys_data();
        assert!(fungible_staked_mys_data.total_supply() == 0, 0);
        assert!(fungible_staked_mys_data.principal_value() == 1, 0);

        mys::test_utils::destroy(staking_pool);
        mys::test_utils::destroy(staked_mys_1);
        mys::test_utils::destroy(mys);

        scenario.end();
    }

    #[test_only]
    fun distribute_rewards_and_advance_epoch(
        staking_pool: &mut StakingPool, 
        scenario: &mut Scenario,
        reward_amount: u64
    ): u64 {
        use mys::tx_context::{epoch};
        use mys::coin::{Self};
        use mys::mys::MYS;

        let rewards = coin::mint_for_testing<MYS>(reward_amount, scenario.ctx());
        staking_pool.deposit_rewards(coin::into_balance(rewards));

        staking_pool.process_pending_stakes_and_withdraws(scenario.ctx());
        test_scenario::next_epoch(scenario, @0x0);

        scenario.ctx().epoch()
    }
}
