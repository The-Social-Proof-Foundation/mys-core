// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module mys::post_tests {
    use mys::test_scenario::{Self, Scenario};
    use mys::test_utils;
    use mys::social_network::profile::{Self, Profile};
    use mys::social_network::post::{Self, Post, Comment};
    use mys::tx_context;
    use mys::object::{Self, UID};
    use mys::url::{Self, Url};
    use std::string::{Self, String};
    use std::vector;
    use std::option;

    const TEST_SENDER: address = @0xCAFE;
    const OTHER_USER: address = @0xFACE;
    
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
    fun test_create_post() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create a profile first
        {
            create_test_profile(&mut scenario);
        };
        
        // Create a post
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let content = string::utf8(b"This is a test post");
            let media_urls = vector::empty<Url>();
            vector::push_back(&mut media_urls, url::new_unsafe_from_bytes(b"https://example.com/image.jpg"));
            let mentions = vector::empty<address>();
            vector::push_back(&mut mentions, OTHER_USER);
            
            post::create_post(
                &profile,
                content,
                media_urls,
                mentions,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // Verify post was created with correct data
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let post = test_scenario::take_from_sender<Post>(&scenario);
            
            assert!(post::content(&post) == string::utf8(b"This is a test post"), 0);
            let media = post::media(&post);
            assert!(vector::length(media) == 1, 1);
            assert!(url::inner_url(vector::borrow(media, 0)) == b"https://example.com/image.jpg", 2);
            
            let mentions = post::mentions(&post);
            assert!(vector::length(mentions) == 1, 3);
            assert!(*vector::borrow(mentions, 0) == OTHER_USER, 4);
            
            test_scenario::return_to_sender(&scenario, post);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_create_comment() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create a profile first
        {
            create_test_profile(&mut scenario);
        };
        
        // Create a post
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let content = string::utf8(b"This is a test post");
            let media_urls = vector::empty<Url>();
            let mentions = vector::empty<address>();
            
            post::create_post(
                &profile,
                content,
                media_urls,
                mentions,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // Create a comment on the post
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let post_obj = test_scenario::take_from_sender<Post>(&scenario);
            
            let comment_content = string::utf8(b"This is a test comment");
            
            post::create_comment(
                &profile,
                &post_obj,
                comment_content,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_to_sender(&scenario, post_obj);
        };
        
        // Verify comment was created with correct data
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let comment = test_scenario::take_from_sender<Comment>(&scenario);
            
            assert!(post::comment_content(&comment) == string::utf8(b"This is a test comment"), 0);
            assert!(post::comment_author(&comment) == TEST_SENDER, 1);
            
            test_scenario::return_to_sender(&scenario, comment);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_like_post() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create a profile first
        {
            create_test_profile(&mut scenario);
        };
        
        // Create a post
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let content = string::utf8(b"This is a test post");
            let media_urls = vector::empty<Url>();
            let mentions = vector::empty<address>();
            
            post::create_post(
                &profile,
                content,
                media_urls,
                mentions,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // Like the post
        test_scenario::next_tx(&mut scenario, OTHER_USER);
        {
            // Create a profile for OTHER_USER first
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
            let post_obj = test_scenario::take_from_address<Post>(&scenario, TEST_SENDER);
            
            post::like_post(
                &profile,
                &mut post_obj,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_to_address(TEST_SENDER, post_obj);
        };
        
        // Verify post was liked
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let post_obj = test_scenario::take_from_sender<Post>(&scenario);
            
            assert!(post::is_post_liked_by(&post_obj, OTHER_USER), 0);
            assert!(!post::is_post_liked_by(&post_obj, TEST_SENDER), 1);
            
            test_scenario::return_to_sender(&scenario, post_obj);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_unlike_post() {
        let scenario = test_scenario::begin(TEST_SENDER);
        
        // Create a profile first
        {
            create_test_profile(&mut scenario);
        };
        
        // Create a post
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let content = string::utf8(b"This is a test post");
            let media_urls = vector::empty<Url>();
            let mentions = vector::empty<address>();
            
            post::create_post(
                &profile,
                content,
                media_urls,
                mentions,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        // Like the post
        test_scenario::next_tx(&mut scenario, OTHER_USER);
        {
            // Create a profile for OTHER_USER first
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
            let post_obj = test_scenario::take_from_address<Post>(&scenario, TEST_SENDER);
            
            post::like_post(
                &profile,
                &mut post_obj,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_to_address(TEST_SENDER, post_obj);
        };
        
        // Unlike the post
        test_scenario::next_tx(&mut scenario, OTHER_USER);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let post_obj = test_scenario::take_from_address<Post>(&scenario, TEST_SENDER);
            
            post::unlike_post(
                &profile,
                &mut post_obj,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_to_address(TEST_SENDER, post_obj);
        };
        
        // Verify post was unliked
        test_scenario::next_tx(&mut scenario, TEST_SENDER);
        {
            let post_obj = test_scenario::take_from_sender<Post>(&scenario);
            
            assert!(!post::is_post_liked_by(&post_obj, OTHER_USER), 0);
            
            test_scenario::return_to_sender(&scenario, post_obj);
        };
        
        test_scenario::end(scenario);
    }
}