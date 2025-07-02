// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module mys::linear_vesting_tests {
    use mys::linear_vesting;
    use mys::coin::{Self, Coin};
    use mys::clock::{Self, Clock};
    use mys::mys::{Self, MYS};
    use mys::test_scenario::{Self as test, Scenario};
    use mys::transfer;

    const ALICE: address = @0xa;
    const BOB: address = @0xb;
    
    const TOTAL_AMOUNT: u64 = 1000000; // 1M MIST
    const VESTING_DURATION: u64 = 365 * 24 * 60 * 60 * 1000; // 1 year in ms
    const START_TIME: u64 = 1000000; // Some future time

    #[test]
    fun test_create_vesting_wallet() {
        let mut scenario = test::begin(ALICE);
        let ctx = test::ctx(&mut scenario);
        
        // Create a clock for testing
        let mut clock = clock::create_for_testing(ctx);
        clock.set_for_testing(START_TIME - 1000); // Set time before vesting starts
        
        // Create some test coins
        let coins = coin::mint_for_testing<MYS>(TOTAL_AMOUNT, ctx);
        
        // Create vesting wallet
        let wallet = linear_vesting::new_vesting_wallet(
            coins,
            &clock,
            START_TIME,
            VESTING_DURATION,
            ALICE,
            ctx
        );
        
        // Verify initial state
        assert!(linear_vesting::balance(&wallet) == TOTAL_AMOUNT);
        assert!(linear_vesting::start(&wallet) == START_TIME);
        assert!(linear_vesting::duration(&wallet) == VESTING_DURATION);
        assert!(linear_vesting::beneficiary(&wallet) == ALICE);
        assert!(linear_vesting::claimed(&wallet) == 0);
        assert!(linear_vesting::total_amount(&wallet) == TOTAL_AMOUNT);
        
        // Check that nothing is claimable before start time
        assert!(linear_vesting::claimable(&wallet, &clock) == 0);
        
        // Clean up
        clock.destroy_for_testing();
        linear_vesting::delete_wallet(wallet);
        test::end(scenario);
    }

    #[test]
    fun test_linear_vesting_calculation() {
        let mut scenario = test::begin(ALICE);
        let ctx = test::ctx(&mut scenario);
        
        let mut clock = clock::create_for_testing(ctx);
        let coins = coin::mint_for_testing<MYS>(TOTAL_AMOUNT, ctx);
        
        let wallet = linear_vesting::new_genesis_vesting_wallet(
            coins,
            START_TIME,
            VESTING_DURATION,
            ALICE,
            ctx
        );
        
        // Test at different time points
        
        // Before start: 0% claimable
        clock.set_for_testing(START_TIME - 1);
        assert!(linear_vesting::claimable(&wallet, &clock) == 0);
        
        // At start: still 0% claimable (just started)
        clock.set_for_testing(START_TIME);
        assert!(linear_vesting::claimable(&wallet, &clock) == 0);
        
        // After 25% of duration: 25% claimable
        clock.set_for_testing(START_TIME + VESTING_DURATION / 4);
        let expected_25_percent = TOTAL_AMOUNT / 4;
        assert!(linear_vesting::claimable(&wallet, &clock) == expected_25_percent);
        
        // After 50% of duration: 50% claimable
        clock.set_for_testing(START_TIME + VESTING_DURATION / 2);
        let expected_50_percent = TOTAL_AMOUNT / 2;
        assert!(linear_vesting::claimable(&wallet, &clock) == expected_50_percent);
        
        // After 100% of duration: 100% claimable
        clock.set_for_testing(START_TIME + VESTING_DURATION);
        assert!(linear_vesting::claimable(&wallet, &clock) == TOTAL_AMOUNT);
        
        // After end: still 100% claimable
        clock.set_for_testing(START_TIME + VESTING_DURATION + 1000);
        assert!(linear_vesting::claimable(&wallet, &clock) == TOTAL_AMOUNT);
        
        // Clean up
        clock.destroy_for_testing();
        linear_vesting::delete_wallet(wallet);
        test::end(scenario);
    }

    #[test]
    fun test_claiming_tokens() {
        let mut scenario = test::begin(ALICE);
        
        {
            let ctx = test::ctx(&mut scenario);
            let mut clock = clock::create_for_testing(ctx);
            let coins = coin::mint_for_testing<MYS>(TOTAL_AMOUNT, ctx);
            
            let wallet = linear_vesting::new_genesis_vesting_wallet(
                coins,
                START_TIME,
                VESTING_DURATION,
                ALICE,
                ctx
            );
            
            // Transfer wallet to Alice
            linear_vesting::transfer_wallet(wallet, ALICE);
            clock.set_for_testing(START_TIME + VESTING_DURATION / 2); // 50% vested
            transfer::public_transfer(clock, ALICE);
        };
        
        // Alice claims 50% of tokens
        test::next_tx(&mut scenario, ALICE);
        {
            let mut wallet = test::take_from_sender<linear_vesting::VestingWallet<MYS>>(&scenario);
            let clock = test::take_from_sender<Clock>(&scenario);
            let ctx = test::ctx(&mut scenario);
            
            // Check claimable amount
            let claimable_amount = linear_vesting::claimable(&wallet, &clock);
            assert!(claimable_amount == TOTAL_AMOUNT / 2);
            
            // Claim tokens
            let claimed_coins = linear_vesting::claim(&mut wallet, &clock, ctx);
            assert!(coin::value(&claimed_coins) == TOTAL_AMOUNT / 2);
            
            // Verify wallet state after claiming
            assert!(linear_vesting::claimed(&wallet) == TOTAL_AMOUNT / 2);
            assert!(linear_vesting::balance(&wallet) == TOTAL_AMOUNT / 2);
            assert!(linear_vesting::claimable(&wallet, &clock) == 0); // Nothing more claimable now
            
            // Clean up
            transfer::public_transfer(claimed_coins, ALICE);
            test::return_to_sender(&scenario, wallet);
            test::return_to_sender(&scenario, clock);
        };
        
        test::end(scenario);
    }

    #[test]
    fun test_multiple_claims() {
        let mut scenario = test::begin(ALICE);
        
        {
            let ctx = test::ctx(&mut scenario);
            let mut clock = clock::create_for_testing(ctx);
            let coins = coin::mint_for_testing<MYS>(TOTAL_AMOUNT, ctx);
            
            let wallet = linear_vesting::new_genesis_vesting_wallet(
                coins,
                START_TIME,
                VESTING_DURATION,
                ALICE,
                ctx
            );
            
            linear_vesting::transfer_wallet(wallet, ALICE);
            clock.set_for_testing(START_TIME + VESTING_DURATION / 4); // 25% vested
            transfer::public_transfer(clock, ALICE);
        };
        
        // First claim: 25%
        test::next_tx(&mut scenario, ALICE);
        {
            let mut wallet = test::take_from_sender<linear_vesting::VestingWallet<MYS>>(&scenario);
            let mut clock = test::take_from_sender<Clock>(&scenario);
            let ctx = test::ctx(&mut scenario);
            
            let first_claim = linear_vesting::claim(&mut wallet, &clock, ctx);
            assert!(coin::value(&first_claim) == TOTAL_AMOUNT / 4);
            
            // Advance time to 75% completion
            clock.set_for_testing(START_TIME + 3 * VESTING_DURATION / 4);
            
            // Second claim: should be able to claim another 50% (75% - 25% already claimed)
            let second_claim = linear_vesting::claim(&mut wallet, &clock, ctx);
            assert!(coin::value(&second_claim) == TOTAL_AMOUNT / 2);
            
            // Verify final state
            assert!(linear_vesting::claimed(&wallet) == 3 * TOTAL_AMOUNT / 4);
            assert!(linear_vesting::balance(&wallet) == TOTAL_AMOUNT / 4);
            
            transfer::public_transfer(first_claim, ALICE);
            transfer::public_transfer(second_claim, ALICE);
            test::return_to_sender(&scenario, wallet);
            test::return_to_sender(&scenario, clock);
        };
        
        test::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = linear_vesting::EUnauthorizedClaimer)]
    fun test_unauthorized_claim() {
        let mut scenario = test::begin(ALICE);
        
        {
            let ctx = test::ctx(&mut scenario);
            let mut clock = clock::create_for_testing(ctx);
            let coins = coin::mint_for_testing<MYS>(TOTAL_AMOUNT, ctx);
            
            let wallet = linear_vesting::new_genesis_vesting_wallet(
                coins,
                START_TIME,
                VESTING_DURATION,
                ALICE, // Alice is the beneficiary
                ctx
            );
            
            linear_vesting::share_wallet(wallet); // Make it shared so Bob can access
            clock.set_for_testing(START_TIME + VESTING_DURATION / 2);
            transfer::public_transfer(clock, BOB);
        };
        
        // Bob tries to claim (should fail)
        test::next_tx(&mut scenario, BOB);
        {
            let mut wallet = test::take_shared<linear_vesting::VestingWallet<MYS>>(&scenario);
            let clock = test::take_from_sender<Clock>(&scenario);
            let ctx = test::ctx(&mut scenario);
            
            // This should abort with EUnauthorizedClaimer
            let _coins = linear_vesting::claim(&mut wallet, &clock, ctx);
            
            test::return_shared(wallet);
            test::return_to_sender(&scenario, clock);
        };
        
        test::end(scenario);
    }

    #[test]
    fun test_transfer_ownership() {
        let mut scenario = test::begin(ALICE);
        
        {
            let ctx = test::ctx(&mut scenario);
            let clock = clock::create_for_testing(ctx);
            let coins = coin::mint_for_testing<MYS>(TOTAL_AMOUNT, ctx);
            
            let mut wallet = linear_vesting::new_genesis_vesting_wallet(
                coins,
                START_TIME,
                VESTING_DURATION,
                ALICE,
                ctx
            );
            
            // Transfer ownership to Bob
            linear_vesting::transfer_ownership(&mut wallet, BOB, ctx);
            assert!(linear_vesting::beneficiary(&wallet) == BOB);
            
            linear_vesting::delete_wallet(wallet);
            clock.destroy_for_testing();
        };
        
        test::end(scenario);
    }
}