// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, unused_assignment, duplicate_alias)]
module social_contracts::proof_of_creativity_tests {
    use std::string::{Self, String};

    use mys::test_scenario::{Self, Scenario};
    use mys::tx_context;

    use social_contracts::proof_of_creativity as poc;

    // Test addresses
    const ADMIN: address = @0xA0;

    #[test]
    fun test_poc_bootstrap_and_update_config() {
        let mut scen = test_scenario::begin(ADMIN);

        // Initialize PoC
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            poc::test_init(test_scenario::ctx(&mut scen));
        };

        // Update PoC config including max_votes_per_dispute
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<poc::PoCAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<poc::PoCConfig>(&scen);
            poc::update_poc_config(
                &admin_cap,
                &mut cfg,
                ADMIN, // oracle_address
                95,    // image_threshold
                95,    // video_threshold
                95,    // audio_threshold
                100,   // revenue_redirect_percentage
                5_000_000_000, // dispute_cost
                1_000_000_000, // dispute_protocol_fee
                1_000_000_000, // min_vote_stake
                100_000_000_000, // max_vote_stake
                7,     // voting_duration_epochs
                5000,  // max_reasoning_length
                10,    // max_evidence_urls
                5000,  // max_votes_per_dispute
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
        };

        test_scenario::end(scen);
    }
}

