// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module mys::social_graph_tests {
    use mys::test_scenario::{Self, Scenario};
    use mys::test_utils;
    use mys::social_network::profile::{Self, Profile};
    use mys::social_network::social_graph::{Self, SocialGraph};
    use mys::tx_context;
    use mys::object::{Self, UID};
    use mys::url::{Self, Url};
    use std::string::{Self, String};
    use std::vector;

    const TEST_SENDER: address = @0xCAFE;
    const OTHER_USER: address = @0xFACE;
    const THIRD_USER: address = @0xBEEF;
    
    // Helper function to create a profile
    fun create_test_profile(scenario: &mut Scenario) {
        let display_name = string::utf8(b"Test User");
        let bio = string::utf8(b"This is a test bio");
        let profile_picture = url::new_unsafe_from_bytes(b"https://example.com/profile.jpg");
        
        profile::create_profile(
            display_name,
            bio,
            profile_picture,
            test_scenario::ctx(scenario)
        );
    }
    
    #[test]
    fun test_create_social_graph() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create a profile first (requirement for social graph)
        {
            create_test_profile(&mut scenario);
        };
        
        // Create a social graph
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            social_graph::create_social_graph(
                &profile,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // Verify social graph was created
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let social_graph = test_scenario::take_from_sender<SocialGraph>(&scenario);
            
            // Verify the social graph is empty initially
            assert!(vector::length(social_graph::get_following(&social_graph)) == 0, 0);
            assert!(vector::length(social_graph::get_followers(&social_graph)) == 0, 1);
            
            test_scenario::return_to_sender(&scenario, social_graph);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_follow_user() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create profiles and social graphs for both users
        {
            // Create profile for TEST_SENDER
            create_test_profile(&mut scenario);
        };
        
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            social_graph::create_social_graph(
                &profile,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // Create profile and social graph for OTHER_USER
        test_scenario::next_tx(&mut scenario, OTHER_USER);
        {
            let display_name = string::utf8(b"Other User");
            let bio = string::utf8(b"Other user bio");
            let profile_picture = url::new_unsafe_from_bytes(b"https://example.com/other-profile.jpg");
            
            profile::create_profile(
                display_name,
                bio,
                profile_picture,
                test_scenario::ctx(&mut scenario)
            );
            
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            social_graph::create_social_graph(
                &profile,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // TEST_SENDER follows OTHER_USER
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let social_graph_obj = test_scenario::take_from_sender<SocialGraph>(&scenario);
            
            social_graph::follow(
                &profile,
                &mut social_graph_obj,
                OTHER_USER,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_to_sender(&scenario, social_graph_obj);
        };
        
        // Verify TEST_SENDER is following OTHER_USER
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let social_graph_obj = test_scenario::take_from_sender<SocialGraph>(&scenario);
            
            let following = social_graph::get_following(&social_graph_obj);
            assert!(vector::length(following) == 1, 0);
            assert!(*vector::borrow(following, 0) == OTHER_USER, 1);
            
            // Verify is_following function
            assert!(social_graph::is_following(&social_graph_obj, OTHER_USER), 2);
            assert!(!social_graph::is_following(&social_graph_obj, THIRD_USER), 3);
            
            test_scenario::return_to_sender(&scenario, social_graph_obj);
        };
        
        // Verify OTHER_USER has TEST_SENDER as follower
        test_scenario::next_tx(&mut scenario, OTHER_USER);
        {
            let social_graph_obj = test_scenario::take_from_sender<SocialGraph>(&scenario);
            
            let followers = social_graph::get_followers(&social_graph_obj);
            assert!(vector::length(followers) == 1, 0);
            assert!(*vector::borrow(followers, 0) == TEST_SENDER, 1);
            
            test_scenario::return_to_sender(&scenario, social_graph_obj);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_unfollow_user() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create profiles and social graphs for both users
        {
            // Create profile for TEST_SENDER
            create_test_profile(&mut scenario);
        };
        
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            social_graph::create_social_graph(
                &profile,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // Create profile and social graph for OTHER_USER
        test_scenario::next_tx(&mut scenario, OTHER_USER);
        {
            let display_name = string::utf8(b"Other User");
            let bio = string::utf8(b"Other user bio");
            let profile_picture = url::new_unsafe_from_bytes(b"https://example.com/other-profile.jpg");
            
            profile::create_profile(
                display_name,
                bio,
                profile_picture,
                test_scenario::ctx(&mut scenario)
            );
            
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            social_graph::create_social_graph(
                &profile,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // TEST_SENDER follows OTHER_USER
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let social_graph_obj = test_scenario::take_from_sender<SocialGraph>(&scenario);
            
            social_graph::follow(
                &profile,
                &mut social_graph_obj,
                OTHER_USER,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_to_sender(&scenario, social_graph_obj);
        };
        
        // TEST_SENDER unfollows OTHER_USER
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let social_graph_obj = test_scenario::take_from_sender<SocialGraph>(&scenario);
            
            social_graph::unfollow(
                &profile,
                &mut social_graph_obj,
                OTHER_USER,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_to_sender(&scenario, social_graph_obj);
        };
        
        // Verify TEST_SENDER is no longer following OTHER_USER
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let social_graph_obj = test_scenario::take_from_sender<SocialGraph>(&scenario);
            
            let following = social_graph::get_following(&social_graph_obj);
            assert!(vector::length(following) == 0, 0);
            
            // Verify is_following function
            assert!(!social_graph::is_following(&social_graph_obj, OTHER_USER), 1);
            
            test_scenario::return_to_sender(&scenario, social_graph_obj);
        };
        
        // Verify OTHER_USER no longer has TEST_SENDER as follower
        test_scenario::next_tx(&mut scenario, OTHER_USER);
        {
            let social_graph_obj = test_scenario::take_from_sender<SocialGraph>(&scenario);
            
            let followers = social_graph::get_followers(&social_graph_obj);
            assert!(vector::length(followers) == 0, 0);
            
            test_scenario::return_to_sender(&scenario, social_graph_obj);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    #[expected_failure(abort_code = social_graph::ESelfFollow)]
    fun test_cannot_follow_self() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create profile and social graph
        {
            create_test_profile(&mut scenario);
        };
        
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            social_graph::create_social_graph(
                &profile,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // Try to follow self (should fail)
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let social_graph_obj = test_scenario::take_from_sender<SocialGraph>(&scenario);
            
            social_graph::follow(
                &profile,
                &mut social_graph_obj,
                TEST_SENDER, // Trying to follow self
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_to_sender(&scenario, social_graph_obj);
        };
        
        test_scenario::end(scenario);
    }
}