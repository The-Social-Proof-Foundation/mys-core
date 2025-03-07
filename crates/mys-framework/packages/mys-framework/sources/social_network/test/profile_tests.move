// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module mys::profile_tests {
    use mys::test_scenario::{Self, Scenario};
    use mys::test_utils;
    use mys::social_network::profile::{Self, Profile};
    use mys::tx_context;
    use mys::object::{Self, UID};
    use mys::url::{Self, Url};
    use std::string::{Self, String};

    const TEST_SENDER: address = @0xCAFE;
    const OTHER_USER: address = @0xFACE;
    
    #[test]
    fun test_create_profile() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create a profile
        {
            let display_name = string::utf8(b"Test User");
            let bio = string::utf8(b"This is a test bio");
            let profile_picture = url::new_unsafe_from_bytes(b"https://example.com/profile.jpg");
            
            profile::create_profile(
                display_name,
                bio,
                profile_picture,
                test_scenario::ctx(&mut scenario)
            );
        };
        
        // Verify profile was created and has correct data
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            assert!(profile::display_name(&profile) == string::utf8(b"Test User"), 0);
            assert!(profile::bio(&profile) == string::utf8(b"This is a test bio"), 1);
            assert!(url::inner_url(profile::profile_picture(&profile)) == b"https://example.com/profile.jpg", 2);
            assert!(profile::owner(&profile) == TEST_SENDER, 3);
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_update_profile() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create a profile
        {
            let display_name = string::utf8(b"Test User");
            let bio = string::utf8(b"This is a test bio");
            let profile_picture = url::new_unsafe_from_bytes(b"https://example.com/profile.jpg");
            
            profile::create_profile(
                display_name,
                bio,
                profile_picture,
                test_scenario::ctx(&mut scenario)
            );
        };
        
        // Update the profile
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            let new_display_name = string::utf8(b"Updated Name");
            let new_bio = string::utf8(b"Updated bio information");
            let new_profile_picture = url::new_unsafe_from_bytes(b"https://example.com/new-profile.jpg");
            
            profile::update_profile(
                &mut profile,
                new_display_name,
                new_bio,
                new_profile_picture,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // Verify profile was updated correctly
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            assert!(profile::display_name(&profile) == string::utf8(b"Updated Name"), 0);
            assert!(profile::bio(&profile) == string::utf8(b"Updated bio information"), 1);
            assert!(url::inner_url(profile::profile_picture(&profile)) == b"https://example.com/new-profile.jpg", 2);
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    #[expected_failure(abort_code = profile::ENotProfileOwner)]
    fun test_update_profile_not_owner() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create a profile
        {
            let display_name = string::utf8(b"Test User");
            let bio = string::utf8(b"This is a test bio");
            let profile_picture = url::new_unsafe_from_bytes(b"https://example.com/profile.jpg");
            
            profile::create_profile(
                display_name,
                bio,
                profile_picture,
                test_scenario::ctx(&mut scenario)
            );
        };
        
        // Try to update the profile as a different user
        test_scenario::next_tx(&mut scenario, OTHER_USER);
        {
            let profile = test_scenario::take_from_address<Profile>(&scenario, TEST_SENDER);
            
            let new_display_name = string::utf8(b"Updated Name");
            let new_bio = string::utf8(b"Updated bio information");
            let new_profile_picture = url::new_unsafe_from_bytes(b"https://example.com/new-profile.jpg");
            
            profile::update_profile(
                &mut profile,
                new_display_name,
                new_bio,
                new_profile_picture,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_address(TEST_SENDER, profile);
        };
        
        test_scenario::end(scenario);
    }
}