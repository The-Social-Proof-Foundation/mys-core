// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_const, duplicate_alias, unused_use)]
module social_contracts::social_graph_tests {
    use std::string;
    use std::option;
    
    use mys::test_scenario;
    use mys::transfer;
    use mys::coin::{Self, Coin};
    use mys::mys::MYS;
    use mys::clock::{Self, Clock};
    
    use social_contracts::profile::{Self, Profile, UsernameRegistry};
    use social_contracts::social_graph::{Self, SocialGraph};
    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    
    // Test addresses
    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const USER2: address = @0x2;
    const USER3: address = @0x3;
    
    // === Test initiation helper ===
    
    fun initialize_modules(scenario: &mut test_scenario::Scenario) {
        // Initialize the profile registry
        profile::init_for_testing(test_scenario::ctx(scenario));
        
        // Initialize the social graph
        social_graph::init_for_testing(test_scenario::ctx(scenario));
        
        // Initialize upgrade module for testing
        upgrade::init_for_testing(test_scenario::ctx(scenario));
        
        // Create test clock and share it
        let clock = clock::create_for_testing(test_scenario::ctx(scenario));
        clock::share_for_testing(clock);
        
        // Mint coins for users
        let coins1 = coin::mint_for_testing<MYS>(20_000_000_000, test_scenario::ctx(scenario));
        transfer::public_transfer(coins1, USER1);
        
        let coins2 = coin::mint_for_testing<MYS>(20_000_000_000, test_scenario::ctx(scenario));
        transfer::public_transfer(coins2, USER2);
        
        let coins3 = coin::mint_for_testing<MYS>(20_000_000_000, test_scenario::ctx(scenario));
        transfer::public_transfer(coins3, USER3);
    }
    
    fun create_test_profiles(scenario: &mut test_scenario::Scenario) {
        // User1 creates profile
        test_scenario::next_tx(scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            profile::create_profile(
                &mut registry,
                string::utf8(b"User One"),
                string::utf8(b"user1"),
                string::utf8(b"Profile for User One"),
                b"https://example.com/user1.png",
                b"",
                test_scenario::ctx(scenario)
            );
            test_scenario::return_shared(registry);
        };
        
        // User2 creates profile
        test_scenario::next_tx(scenario, USER2);
        {
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            profile::create_profile(
                &mut registry,
                string::utf8(b"User Two"),
                string::utf8(b"user2"),
                string::utf8(b"Profile for User Two"),
                b"https://example.com/user2.png",
                b"",
                test_scenario::ctx(scenario)
            );
            test_scenario::return_shared(registry);
        };
        
        // User3 creates profile
        test_scenario::next_tx(scenario, USER3);
        {
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            profile::create_profile(
                &mut registry,
                string::utf8(b"User Three"),
                string::utf8(b"user3"),
                string::utf8(b"Profile for User Three"),
                b"https://example.com/user3.png",
                b"",
                test_scenario::ctx(scenario)
            );
            test_scenario::return_shared(registry);
        };
    }
    
    
    // === Basic follow test ===
    
    #[test]
    fun test_follow() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize modules and create profiles
        initialize_modules(&mut scenario);
        create_test_profiles(&mut scenario);
        
        // User1 follows User2 (using wallet addresses directly)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            social_graph::follow(
                &mut social_graph,
                USER2,
                test_scenario::ctx(&mut scenario)
            );
            
            // Check that User1 is following User2
            assert!(social_graph::is_following(&social_graph, USER1, USER2), 1);
            
            // Check counts
            assert!(social_graph::following_count(&social_graph, USER1) == 1, 2);
            assert!(social_graph::follower_count(&social_graph, USER2) == 1, 3);
            
            // Check lists
            let following = social_graph::get_following(&social_graph, USER1);
            let followers = social_graph::get_followers(&social_graph, USER2);
            
            assert!(vector::length(&following) == 1, 4);
            assert!(vector::length(&followers) == 1, 5);
            assert!(*vector::borrow(&following, 0) == USER2, 6);
            assert!(*vector::borrow(&followers, 0) == USER1, 7);
            
            test_scenario::return_shared(social_graph);
        };
        
        test_scenario::end(scenario);
    }
    
    // === Test follow then unfollow ===
    
    #[test]
    fun test_unfollow() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize modules and create profiles
        initialize_modules(&mut scenario);
        create_test_profiles(&mut scenario);
        
        // User1 follows User2
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            social_graph::follow(
                &mut social_graph,
                USER2,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(social_graph);
        };
        
        // User1 unfollows User2
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            social_graph::unfollow(
                &mut social_graph,
                USER2,
                test_scenario::ctx(&mut scenario)
            );
            
            // Check that User1 is no longer following User2
            assert!(!social_graph::is_following(&social_graph, USER1, USER2), 1);
            
            // Check counts are back to 0
            assert!(social_graph::following_count(&social_graph, USER1) == 0, 2);
            assert!(social_graph::follower_count(&social_graph, USER2) == 0, 3);
            
            test_scenario::return_shared(social_graph);
        };
        
        test_scenario::end(scenario);
    }
    
    // === Test multiple follows ===
    
    #[test]
    fun test_multiple_follows() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize modules and create profiles
        initialize_modules(&mut scenario);
        create_test_profiles(&mut scenario);
        
        // User1 follows User2 and User3
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            // User1 follows User2
            social_graph::follow(
                &mut social_graph,
                USER2,
                test_scenario::ctx(&mut scenario)
            );
            
            // User1 follows User3
            social_graph::follow(
                &mut social_graph,
                USER3,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(social_graph);
        };
        
        // Verify multiple follows
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            // Check that User1 is following both User2 and User3
            assert!(social_graph::is_following(&social_graph, USER1, USER2), 1);
            assert!(social_graph::is_following(&social_graph, USER1, USER3), 2);
            
            // Check following count
            assert!(social_graph::following_count(&social_graph, USER1) == 2, 3);
            
            // Check follower counts
            assert!(social_graph::follower_count(&social_graph, USER2) == 1, 4);
            assert!(social_graph::follower_count(&social_graph, USER3) == 1, 5);
            
            // Check following list contains both User2 and User3
            let following = social_graph::get_following(&social_graph, USER1);
            assert!(vector::length(&following) == 2, 6);
            
            // The order might not be guaranteed, so check both are in the list
            let mut has_user2 = false;
            let mut has_user3 = false;
            
            let mut i = 0;
            while (i < vector::length(&following)) {
                let address = *vector::borrow(&following, i);
                if (address == USER2) {
                    has_user2 = true;
                };
                if (address == USER3) {
                    has_user3 = true;
                };
                i = i + 1;
            };
            
            assert!(has_user2, 7);
            assert!(has_user3, 8);
            
            test_scenario::return_shared(social_graph);
        };
        
        test_scenario::end(scenario);
    }
    
    // === Test mutual follows ===
    
    #[test]
    fun test_mutual_follows() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize modules and create profiles
        initialize_modules(&mut scenario);
        create_test_profiles(&mut scenario);
        
        // User1 follows User2
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            social_graph::follow(
                &mut social_graph,
                USER2,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(social_graph);
        };
        
        // User2 follows User1 (mutual follow)
        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            social_graph::follow(
                &mut social_graph,
                USER1,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(social_graph);
        };
        
        // Verify mutual follows
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            // Check that User1 and User2 are following each other
            assert!(social_graph::is_following(&social_graph, USER1, USER2), 1);
            assert!(social_graph::is_following(&social_graph, USER2, USER1), 2);
            
            // Check counts
            assert!(social_graph::following_count(&social_graph, USER1) == 1, 3);
            assert!(social_graph::following_count(&social_graph, USER2) == 1, 4);
            assert!(social_graph::follower_count(&social_graph, USER1) == 1, 5);
            assert!(social_graph::follower_count(&social_graph, USER2) == 1, 6);
            
            test_scenario::return_shared(social_graph);
        };
        
        test_scenario::end(scenario);
    }
    
    // === Test error cases ===
    
    #[test]
    #[expected_failure(abort_code = social_graph::ECannotFollowSelf, location = social_contracts::social_graph)]
    fun test_cannot_follow_self() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize modules and create profiles
        initialize_modules(&mut scenario);
        create_test_profiles(&mut scenario);
        
        // User1 tries to follow themselves
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            // This should fail with ECannotFollowSelf
            social_graph::follow(
                &mut social_graph,
                USER1,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(social_graph);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    #[expected_failure(abort_code = social_graph::EAlreadyFollowing, location = social_contracts::social_graph)]
    fun test_already_following() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize modules and create profiles
        initialize_modules(&mut scenario);
        create_test_profiles(&mut scenario);
        
        // User1 follows User2
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            social_graph::follow(
                &mut social_graph,
                USER2,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(social_graph);
        };
        
        // User1 tries to follow User2 again
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            // This should fail with EAlreadyFollowing
            social_graph::follow(
                &mut social_graph,
                USER2,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(social_graph);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    #[expected_failure(abort_code = social_graph::ENotFollowing, location = social_contracts::social_graph)]
    fun test_not_following() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize modules and create profiles
        initialize_modules(&mut scenario);
        create_test_profiles(&mut scenario);
        
        // User1 tries to unfollow User2 without following them first
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            // This should fail with ENotFollowing
            social_graph::unfollow(
                &mut social_graph,
                USER2,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(social_graph);
        };
        
        test_scenario::end(scenario);
    }
    
    // === Test version migration ===
    
    #[test]
    fun test_migrate_social_graph() {
        // This is just a mock test to verify we can call the function and handle the AdminCap
        // We're not actually testing the version migration functionality
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize modules
        initialize_modules(&mut scenario);
        
        // Skip actual migration attempt since we can't modify the package version in tests
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            let admin_cap = test_scenario::take_from_sender<UpgradeAdminCap>(&scenario);
            
            // Just check the initial version
            assert!(social_graph::version(&social_graph) == 1, 1);
            
            // Return objects
            test_scenario::return_shared(social_graph);
            test_scenario::return_to_sender(&scenario, admin_cap);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_follow_without_profile() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize modules but DON'T create profiles
        initialize_modules(&mut scenario);
        
        // User1 follows User2 without having a profile (wallet-level following)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            
            // This should work - no profile required
            social_graph::follow(
                &mut social_graph,
                USER2,
                test_scenario::ctx(&mut scenario)
            );
            
            // Verify following relationship exists
            assert!(social_graph::is_following(&social_graph, USER1, USER2), 1);
            assert!(social_graph::following_count(&social_graph, USER1) == 1, 2);
            assert!(social_graph::follower_count(&social_graph, USER2) == 1, 3);
            
            test_scenario::return_shared(social_graph);
        };
        
        test_scenario::end(scenario);
    }
} 