// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, unused_assignment, duplicate_alias)]
module social_contracts::insurance_tests {
    use std::{string, option, vector};

    use mys::test_scenario::{Self, Scenario};
    use mys::tx_context;
    use mys::coin::{Self, Coin};
    use mys::clock::{Self, Clock};
    use mys::transfer;
    use mys::mys::MYS;
    use mys::object;

    use social_contracts::insurance;
    use social_contracts::social_proof_of_truth as spot;
    use social_contracts::social_proof_tokens as spt;
    use social_contracts::post::{Self, Post};
    use social_contracts::platform::{Self, Platform, PlatformRegistry};
    use social_contracts::block_list;
    use social_contracts::profile::{Self, EcosystemTreasury};

    const ADMIN: address = @0xA0;
    const CREATOR: address = @0xC1;
    const UNDERWRITER: address = @0xB1;
    const USER1: address = @0x01;
    const TEST_PLATFORM_ID: address = @0x01; // Use USER1's address as test platform ID

    const SCALING: u64 = 1_000_000_000; // 1e9
    const DAY_MS: u64 = 86_400_000;

    fun setup_env(): Scenario {
        let mut scen = test_scenario::begin(ADMIN);

        spt::init_for_testing(test_scenario::ctx(&mut scen));

        test_scenario::next_tx(&mut scen, ADMIN);
        { block_list::test_init(test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        { platform::test_init(test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        { post::test_init(test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        { profile::init_for_testing(test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        { spot::test_init(test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 7000, 0, 0, 0, 100, 5000, ADMIN, 0, 10000, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            clock::share_for_testing(clock);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            transfer_to(USER1, 10_000 * SCALING, test_scenario::ctx(&mut scen));
            transfer_to(CREATOR, 10_000 * SCALING, test_scenario::ctx(&mut scen));
            transfer_to(UNDERWRITER, 10_000 * SCALING, test_scenario::ctx(&mut scen));
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut preg = test_scenario::take_shared<PlatformRegistry>(&scen);
            platform::create_platform(
                &mut preg,
                string::utf8(b"Insurance Test Platform"),
                string::utf8(b"Tag"),
                string::utf8(b"Desc"),
                string::utf8(b"https://logo"),
                string::utf8(b"https://tos"),
                string::utf8(b"https://pp"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example")],
                string::utf8(b"Social Network"),
                option::none(),
                3,
                string::utf8(b"2024-01-01"),
                false,
                option::none(), option::none(), option::none(), option::none(),
                option::none(), option::none(), option::none(), option::none(),
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_shared(preg);
        };

        scen
    }

    fun transfer_to(to: address, amount: u64, ctx: &mut tx_context::TxContext) {
        let c = coin::mint_for_testing<MYS>(amount, ctx);
        transfer::public_transfer(c, to);
    }

    fun create_test_post(owner: address, ctx: &mut tx_context::TxContext): address {
        post::test_create_post_with_spot(owner, owner, TEST_PLATFORM_ID, string::utf8(b"truth?"), ctx)
    }

    // === Helper Functions ===

    fun setup_env_with_insurance(): Scenario {
        let mut scen = setup_env();
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            insurance::init_config(
                1000,
                9000,
                7 * DAY_MS,
                50,
                test_scenario::ctx(&mut scen)
            );
        };
        scen
    }

    fun setup_vault_with_capital(scenario: &mut Scenario, capital_amount: u64) {
        test_scenario::next_tx(scenario, UNDERWRITER);
        {
            insurance::create_vault(25, 5000, 0, 0, test_scenario::ctx(scenario));
        };
        // Enable the config before depositing capital
        test_scenario::next_tx(scenario, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<insurance::InsuranceAdminCap>(scenario);
            let mut config = test_scenario::take_shared<insurance::InsuranceConfig>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            insurance::set_enable_flag(&admin_cap, &mut config, true, &clock, test_scenario::ctx(scenario));
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(scenario, admin_cap);
        };
        test_scenario::next_tx(scenario, UNDERWRITER);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(scenario);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(scenario);
            let deposit = coin::mint_for_testing<MYS>(capital_amount, test_scenario::ctx(scenario));
            insurance::deposit_capital(&config, &mut vault, deposit, test_scenario::ctx(scenario));
            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
        };
    }

    fun create_open_market_with_bet(scenario: &mut Scenario, bettor: address, bet_amount: u64, option_id: u8) {
        test_scenario::next_tx(scenario, CREATOR);
        { create_test_post(CREATOR, test_scenario::ctx(scenario)); };

        test_scenario::next_tx(scenario, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(scenario);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(scenario);
            let mut p = test_scenario::take_shared<Post>(scenario);
            let mut betting_options = vector::empty<string::String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg,
                &mut p,
                betting_options,
                option::none(),
                option::some(0),
                test_scenario::ctx(scenario)
            );
            test_scenario::return_to_sender(scenario, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
        };

        test_scenario::next_tx(scenario, bettor);
        {
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(scenario);
            let post_ref = test_scenario::take_shared<Post>(scenario);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(scenario);
            let pay = coin::mint_for_testing<MYS>(bet_amount, test_scenario::ctx(scenario));
            spot::place_spot_bet(
                &spot_cfg,
                &mut rec,
                &post_ref,
                pay,
                option_id,
                bet_amount,
                test_scenario::ctx(scenario)
            );
            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
        };
    }

    #[test]
    fun test_buy_and_claim_insurance() {
        let mut scen = setup_env();

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            insurance::init_config(
                1000,
                9000,
                7 * DAY_MS,
                50,
                test_scenario::ctx(&mut scen)
            );
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        { create_test_post(CREATOR, test_scenario::ctx(&mut scen)); };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<string::String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg,
                &mut p,
                betting_options,
                option::none(),
                option::some(0),
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let pay = coin::mint_for_testing<MYS>(1000 * SCALING, test_scenario::ctx(&mut scen));

            spot::place_spot_bet(
                &spot_cfg,
                &mut rec,
                &post_ref,
                pay,
                0,
                1000 * SCALING,
                test_scenario::ctx(&mut scen)
            );

            assert!(spot::get_user_option_amount(&rec, USER1, 0) == 1000 * SCALING, 1);

            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
        };

        test_scenario::next_tx(&mut scen, UNDERWRITER);
        {
            insurance::create_vault(25, 5000, 0, 0, test_scenario::ctx(&mut scen));
        };

        // Enable the config before depositing capital
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<insurance::InsuranceAdminCap>(&scen);
            let mut config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            insurance::set_enable_flag(&admin_cap, &mut config, true, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scen, admin_cap);
        };

        test_scenario::next_tx(&mut scen, UNDERWRITER);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let deposit = coin::mint_for_testing<MYS>(5_000 * SCALING, test_scenario::ctx(&mut scen));
            insurance::deposit_capital(&config, &mut vault, deposit, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let payment = coin::mint_for_testing<MYS>(500 * SCALING, test_scenario::ctx(&mut scen));

            insurance::buy_coverage(
                &config,
                &spot_config,
                &mut vault,
                &record,
                0,
                1000 * SCALING,
                8000,
                3 * DAY_MS,
                payment,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let mut platform = test_scenario::take_shared<Platform>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let mut evidence_urls = vector::empty<string::String>();
            vector::push_back(&mut evidence_urls, string::utf8(b"https://example.com/evidence"));
            spot::oracle_resolve(
                &oracle_admin_cap,
                &cfg,
                &mut rec,
                &post_ref,
                &mut platform,
                &treasury,
                1,
                9000,
                string::utf8(b"Test resolution"),
                evidence_urls,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(treasury);
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut policy_id_opt = test_scenario::most_recent_id_for_sender<insurance::CoveragePolicy>(&scen);
            assert!(option::is_some(&policy_id_opt), 1);
            let policy_id = option::extract(&mut policy_id_opt);
            let mut policy = test_scenario::take_from_sender_by_id<insurance::CoveragePolicy>(&scen, policy_id);

            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            insurance::claim(&config, &spot_config, &mut vault, &record, &mut policy, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(spot_config);

            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scen, policy);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_cancel_coverage() {
        let mut scen = setup_env_with_insurance();
        create_open_market_with_bet(&mut scen, USER1, 1000 * SCALING, 0);
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        // Buy coverage
        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let payment = coin::mint_for_testing<MYS>(500 * SCALING, test_scenario::ctx(&mut scen));

            insurance::buy_coverage(
                &config,
                &spot_config,
                &mut vault,
                &record,
                0,
                1000 * SCALING,
                8000,
                3 * DAY_MS,
                payment,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
        };

        // Advance clock by 1 day (1/3 of policy duration)
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let mut clock = test_scenario::take_shared<Clock>(&scen);
            clock::increment_for_testing(&mut clock, DAY_MS);
            test_scenario::return_shared(clock);
        };

        // Cancel coverage - should get 2/3 refund minus fee
        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut policy_id_opt = test_scenario::most_recent_id_for_sender<insurance::CoveragePolicy>(&scen);
            assert!(option::is_some(&policy_id_opt), 1);
            let policy_id = option::extract(&mut policy_id_opt);
            let mut policy = test_scenario::take_from_sender_by_id<insurance::CoveragePolicy>(&scen, policy_id);

            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            insurance::cancel_coverage(
                &config,
                &spot_config,
                &treasury,
                &mut vault,
                &record,
                &mut policy,
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(treasury);

            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scen, policy);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_withdraw_capital() {
        let mut scen = setup_env_with_insurance();
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        // Withdraw some unreserved capital
        test_scenario::next_tx(&mut scen, UNDERWRITER);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            insurance::withdraw_capital(
                &config,
                &mut vault,
                2_000 * SCALING,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_expire_policy() {
        let mut scen = setup_env_with_insurance();
        create_open_market_with_bet(&mut scen, USER1, 1000 * SCALING, 0);
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        // Buy coverage with 1 day duration
        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotRecord>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let payment = coin::mint_for_testing<MYS>(500 * SCALING, test_scenario::ctx(&mut scen));

            insurance::buy_coverage(
                &config,
                &spot_config,
                &mut vault,
                &record,
                0,
                1000 * SCALING,
                8000,
                DAY_MS,
                payment,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
        };

        // Advance clock past expiry time
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let mut clock = test_scenario::take_shared<Clock>(&scen);
            clock::increment_for_testing(&mut clock, DAY_MS + 1000); // 1 day + 1 second
            test_scenario::return_shared(clock);
        };

        // Expire the policy
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut policy_id_opt = test_scenario::most_recent_id_for_sender<insurance::CoveragePolicy>(&scen);
            assert!(option::is_some(&policy_id_opt), 1);
            let policy_id = option::extract(&mut policy_id_opt);
            let mut policy = test_scenario::take_from_sender_by_id<insurance::CoveragePolicy>(&scen, policy_id);

            insurance::expire_policy(&mut vault, &mut policy, &clock);

            test_scenario::return_shared(vault);
            test_scenario::return_to_sender(&scen, policy);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_quote_premium() {
        let mut scen = setup_env_with_insurance();
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            
            // Test basic premium calculation
            let premium1 = insurance::quote_premium(&vault, 1000 * SCALING, 8000, DAY_MS);
            assert!(premium1 > 0, 1);

            // Test longer duration = higher premium
            let premium2 = insurance::quote_premium(&vault, 1000 * SCALING, 8000, 2 * DAY_MS);
            assert!(premium2 > premium1, 2);

            // Test higher coverage = higher premium
            let premium3 = insurance::quote_premium(&vault, 2000 * SCALING, 8000, DAY_MS);
            assert!(premium3 > premium1, 3);

            test_scenario::return_shared(vault);
        };

        test_scenario::end(scen);
    }
}
