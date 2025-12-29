// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, unused_assignment, duplicate_alias)]
module social_contracts::social_proof_of_truth_tests {
    use std::{string::{Self, String}, option, vector};

    use mys::test_scenario::{Self, Scenario};
    use mys::tx_context;
    use mys::object;
    use mys::coin::{Self, Coin};
    use mys::balance;
    use mys::mys::MYS;

    use social_contracts::social_proof_of_truth as spot;
    use social_contracts::social_proof_tokens as spt;
    use social_contracts::post::{Self, Post};
    use social_contracts::platform::{Self, Platform, PlatformRegistry};
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::profile;

    // Test addresses
    const ADMIN: address = @0xA0;
    const CREATOR: address = @0xC1;
    const USER1: address = @0x01;
    const USER2: address = @0x02;

    const SCALING: u64 = 1000000000; // 1e9

    // --- Helpers ---
    fun setup_env(): Scenario {
        let mut scen = test_scenario::begin(ADMIN);

        // Init core modules used by SPoT flow
        spt::init_for_testing(test_scenario::ctx(&mut scen));

        test_scenario::next_tx(&mut scen, ADMIN);
        { block_list::test_init(test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        { platform::test_init(test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        { post::test_init(test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        { spot::test_init(test_scenario::ctx(&mut scen)); };

        // Mint funds
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            transfer_to(USER1, 10_000 * SCALING, test_scenario::ctx(&mut scen));
            transfer_to(USER2, 10_000 * SCALING, test_scenario::ctx(&mut scen));
            transfer_to(CREATOR, 10_000 * SCALING, test_scenario::ctx(&mut scen));
        };

        // Create a platform owned by USER1 (simplified)
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut preg = test_scenario::take_shared<PlatformRegistry>(&scen);
            platform::create_platform(
                &mut preg,
                string::utf8(b"SPoT Test Platform"),
                string::utf8(b"Tag"),
                string::utf8(b"Desc"),
                string::utf8(b"https://logo"),
                string::utf8(b"https://tos"),
                string::utf8(b"https://pp"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example")],
                3,
                string::utf8(b"2024-01-01"),
                false,
                option::none(), option::none(), option::none(), option::none(), option::none(), option::none(), option::none(), option::none(),
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_shared(preg);
        };

        scen
    }

    fun transfer_to(to: address, amount: u64, ctx: &mut tx_context::TxContext) {
        let c = coin::mint_for_testing<MYS>(amount, ctx);
        mys::transfer::public_transfer(c, to);
    }

    /// Create a simple post without platform/profile constraints (test helper in post module)
    fun create_test_post(owner: address, ctx: &mut tx_context::TxContext): address {
        post::test_create_post(owner, owner, string::utf8(b"truth?"), ctx)
    }

    // --- Tests ---

    #[test]
    fun test_spot_bootstrap_and_update_config() {
        let mut scen = setup_env();

        // Update SPoT config to enable immediate resolution and set low fee for tests
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            spot::update_spot_config(
                &admin_cap,
                &mut cfg,
                true, // enable
                7000, // confidence_threshold
                0,    // resolution_window_epochs (immediate)
                0,    // max_resolution_window_epochs (immediate)
                0,    // payout_delay_epochs
                50,   // fee_bps 0.5%
                5000, // platform split
                ADMIN,
                ADMIN,
                ADMIN,
                0,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_spot_bet_and_resolve_yes() {
        let mut scen = setup_env();

        // Configure SPoT for instant resolve
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 0, 0, 0, 0, 0, 5000, ADMIN, ADMIN, ADMIN, 0, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
        };

        // Create post
        test_scenario::next_tx(&mut scen, CREATOR);
        let post_id_addr = {
            let ctx = test_scenario::ctx(&mut scen);
            create_test_post(CREATOR, ctx)
        };

        // Create SPoT record with betting options
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg, 
                &p, 
                betting_options,
                option::none(), // resolution_window_epochs - immediate resolution
                option::some(0), // max_resolution_window_epochs - immediate refunds (must be Some for refund_unresolved)
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
        };

        // User1 places bet on option 0 (Yes)
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut spot_rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let pay = coin::mint_for_testing<MYS>(1000 * SCALING, test_scenario::ctx(&mut scen));
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            
            spot::place_spot_bet(
                &spot_cfg,
                &mut spot_rec,
                &post_ref,
                pay,
                0, // option_id 0 = "Yes"
                1000 * SCALING,
                test_scenario::ctx(&mut scen)
            );

            // Assertions on record via getters
            assert!(spot::get_option_escrow(&spot_rec, 0) == 1000 * SCALING, 1);
            assert!(spot::get_bets_len(&spot_rec) == 1, 2);

            test_scenario::return_shared(spot_rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
        };

        // Oracle resolves option 0 (Yes) immediately (confidence high)
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let mut evidence_urls = vector::empty<String>();
            vector::push_back(&mut evidence_urls, string::utf8(b"https://example.com/evidence1"));
            spot::oracle_resolve(
                &oracle_admin_cap,
                &cfg, 
                &mut rec, 
                &post_ref, 
                0, // outcome_option_id 0 = "Yes"
                9000, 
                string::utf8(b"Test reasoning: High confidence resolution"),
                evidence_urls,
                test_scenario::ctx(&mut scen)
            );
            // Resolved
            assert!(spot::get_status(&rec) == 3, 3); // STATUS_RESOLVED
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_spot_dao_required_and_finalize_draw() {
        let mut scen = setup_env();

        // Lower confidence threshold to require DAO
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 9000, 0, 0, 0, 0, 5000, ADMIN, ADMIN, ADMIN, 0, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
        };

        // Create post and record
        test_scenario::next_tx(&mut scen, CREATOR);
        { create_test_post(CREATOR, test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg, 
                &p, 
                betting_options,
                option::none(), // resolution_window_epochs - immediate resolution
                option::some(0), // max_resolution_window_epochs - immediate refunds (must be Some for refund_unresolved)
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
        };

        // Place bet with USER1 on option 1 (No)
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let pay = coin::mint_for_testing<MYS>(500 * SCALING, test_scenario::ctx(&mut scen));
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            
            spot::place_spot_bet(&spot_cfg, &mut rec, &post_ref, pay, 1, 500 * SCALING, test_scenario::ctx(&mut scen)); // option_id 1 = "No"

            // Check state updated via getters
            assert!(spot::get_option_escrow(&rec, 1) == 500 * SCALING, 1);
            assert!(spot::get_bets_len(&rec) == 1, 2);

            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
        };

        // Oracle says confidence is too low → DAO_REQUIRED
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let mut evidence_urls = vector::empty<String>();
            vector::push_back(&mut evidence_urls, string::utf8(b"https://example.com/evidence2"));
            spot::oracle_resolve(
                &oracle_admin_cap,
                &cfg, 
                &mut rec, 
                &post_ref, 
                0, // outcome_option_id 0 = "Yes" (but confidence too low)
                1000, 
                string::utf8(b"Test reasoning: Low confidence, requires DAO"),
                evidence_urls,
                test_scenario::ctx(&mut scen)
            );
            assert!(spot::get_status(&rec) == 2, 3); // DAO_REQUIRED
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
        };

        // DAO finalizes DRAW → everyone refunded
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            spot::finalize_via_dao(
                &cfg, 
                &mut rec, 
                &post_ref, 
                255, // OUTCOME_DRAW (changed from 3 to 255)
                option::some(string::utf8(b"DAO consensus: Draw outcome")),
                option::none(),
                test_scenario::ctx(&mut scen)
            );
            assert!(spot::get_status(&rec) == 3, 4); // RESOLVED
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_spot_refund_unresolved() {
        let mut scen = setup_env();

        // Set max window = 0 for immediate refunds
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 7000, 0, 0, 0, 0, 5000, ADMIN, ADMIN, ADMIN, 0, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
        };

        // Create post + record
        test_scenario::next_tx(&mut scen, CREATOR);
        { create_test_post(CREATOR, test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg, 
                &p, 
                betting_options,
                option::none(), // resolution_window_epochs - immediate resolution
                option::some(0), // max_resolution_window_epochs - immediate refunds (must be Some for refund_unresolved)
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
        };

        // Place a bet on option 0 (Yes)
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let pay = coin::mint_for_testing<MYS>(250 * SCALING, test_scenario::ctx(&mut scen));
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            
            spot::place_spot_bet(&spot_cfg, &mut rec, &post_ref, pay, 0, 250 * SCALING, test_scenario::ctx(&mut scen)); // option_id 0 = "Yes"

            assert!(spot::get_option_escrow(&rec, 0) == 250 * SCALING, 1);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
        };

        // Immediately allow refund_unresolved (max window already 0) - now requires oracle admin cap
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            spot::refund_unresolved(&oracle_admin_cap, &cfg, &mut rec, &post_ref, test_scenario::ctx(&mut scen));
            assert!(spot::get_status(&rec) == 4, 2); // REFUNDABLE
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
        };

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::social_proof_of_truth::EDuplicateOption)]
    fun test_spot_duplicate_options_rejected() {
        let mut scen = setup_env();

        // Configure SPoT
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 7000, 0, 0, 0, 0, 5000, ADMIN, ADMIN, ADMIN, 0, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
        };

        // Create post
        test_scenario::next_tx(&mut scen, CREATOR);
        { create_test_post(CREATOR, test_scenario::ctx(&mut scen)); };

        // Try to create record with duplicate options - should fail
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"Yes")); // Duplicate!
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg, 
                &p, 
                betting_options,
                option::none(),
                option::none(),
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
        };

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::social_proof_of_truth::EWithdrawalNotAllowed)]
    fun test_spot_withdrawal_not_allowed_dao_required() {
        let mut scen = setup_env();

        // Configure SPoT with high confidence threshold to force DAO_REQUIRED
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 9000, 0, 0, 0, 0, 5000, ADMIN, ADMIN, ADMIN, 0, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
        };

        // Create post and record
        test_scenario::next_tx(&mut scen, CREATOR);
        { create_test_post(CREATOR, test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg, 
                &p, 
                betting_options,
                option::none(),
                option::none(),
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
        };

        // Place bet
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let pay = coin::mint_for_testing<MYS>(500 * SCALING, test_scenario::ctx(&mut scen));
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            
            spot::place_spot_bet(&spot_cfg, &mut rec, &post_ref, pay, 0, 500 * SCALING, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
        };

        // Oracle resolves with low confidence -> DAO_REQUIRED
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let mut evidence_urls = vector::empty<String>();
            vector::push_back(&mut evidence_urls, string::utf8(b"https://example.com/evidence"));
            spot::oracle_resolve(
                &oracle_admin_cap,
                &cfg, 
                &mut rec, 
                &post_ref, 
                0,
                1000, // Low confidence
                string::utf8(b"Low confidence resolution"),
                evidence_urls,
                test_scenario::ctx(&mut scen)
            );
            assert!(spot::get_status(&rec) == 2, 1); // DAO_REQUIRED
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
        };

        // Try to withdraw when status is DAO_REQUIRED - should fail
        test_scenario::next_tx(&mut scen, USER1);
        {
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            spot::withdraw_spot_bet(&spot_cfg, &mut rec, &post_ref, 0, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
        };

        test_scenario::end(scen);
    }
}
