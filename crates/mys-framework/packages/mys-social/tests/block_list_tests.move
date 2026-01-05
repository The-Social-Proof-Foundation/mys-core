// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias, unused_mut_ref)]
module social_contracts::block_list_tests {
    use std::vector;
    use std::option;
    
    use mys::test_scenario;
    
    use social_contracts::block_list;
    use social_contracts::social_graph;
    use social_contracts::profile;
    
    // Test constants
    const USER1: address = @0x1;
    const USER2: address = @0x2;
    const USER3: address = @0x3;
    const ADMIN: address = @0xAD;
    
    // Initialize the block list registry, social graph, and profile registry for testing
    fun init_test_environment(scenario: &mut test_scenario::Scenario) {
        // Use the test-specific initialization functions
        block_list::test_init(test_scenario::ctx(scenario));
        social_graph::init_for_testing(test_scenario::ctx(scenario));
        profile::init_for_testing(test_scenario::ctx(scenario));
    }
    
    // Initialize the block list registry for testing (kept for backward compatibility)
    fun init_block_list_registry(scenario: &mut test_scenario::Scenario) {
        init_test_environment(scenario);
    }
    
    /// Test lazy initialization - blocked set is created automatically on first block
    #[test]
    fun test_create_block_list() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize block list registry
        init_block_list_registry(&mut scenario);
        
        // Verify no blocked addresses yet
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            assert!(block_list::blocked_count(&registry, USER1) == 0, 0);
            assert!(!block_list::is_blocked(&registry, USER1, USER2), 0);
            test_scenario::return_shared(registry);
        };
        
        // Block USER2 - this should automatically create the blocked set (lazy initialization)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(&mut scenario);
            block_list::block_wallet(&mut registry, &mut social_graph, USER2, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
            test_scenario::return_shared(social_graph);
        };
        
        // Verify blocked set was created automatically
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            assert!(block_list::is_blocked(&registry, USER1, USER2), 1);
            assert!(block_list::blocked_count(&registry, USER1) == 1, 2);
            test_scenario::return_shared(registry);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test blocking a wallet
    #[test]
    fun test_block_wallet() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize block list registry
        init_block_list_registry(&mut scenario);
        
        // Block USER2 (block list will be created automatically via lazy initialization)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(&mut scenario);
            block_list::block_wallet(&mut registry, &mut social_graph, USER2, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
            test_scenario::return_shared(social_graph);
        };
        
        // Verify USER2 is blocked
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            assert!(block_list::is_blocked(&registry, USER1, USER2), 0);
            assert!(block_list::blocked_count(&registry, USER1) == 1, 1);
            
            let blocked_wallets = block_list::get_blocked_wallets(&registry, USER1);
            assert!(vector::length(&blocked_wallets) == 1, 2);
            assert!(*vector::borrow(&blocked_wallets, 0) == USER2, 3);
            
            test_scenario::return_shared(registry);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test unblocking a wallet
    #[test]
    fun test_unblock_wallet() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize block list registry
        init_block_list_registry(&mut scenario);
        
        // Block USER2 (block list will be created automatically via lazy initialization)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(&mut scenario);
            block_list::block_wallet(&mut registry, &mut social_graph, USER2, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
            test_scenario::return_shared(social_graph);
        };
        
        // Unblock USER2
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            block_list::unblock_wallet(&mut registry, USER2, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
        };
        
        // Verify USER2 is no longer blocked
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            assert!(!block_list::is_blocked(&registry, USER1, USER2), 0);
            assert!(block_list::blocked_count(&registry, USER1) == 0, 0);
            
            let blocked_wallets = block_list::get_blocked_wallets(&registry, USER1);
            assert!(vector::length(&blocked_wallets) == 0, 0);
            
            test_scenario::return_shared(registry);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test blocking multiple users
    #[test]
    fun test_block_multiple_users() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize block list registry
        init_block_list_registry(&mut scenario);
        
        // Block USER2 (block list will be created automatically via lazy initialization)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(&mut scenario);
            block_list::block_wallet(&mut registry, &mut social_graph, USER2, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
            test_scenario::return_shared(social_graph);
        };
        
        // Block USER3
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(&mut scenario);
            block_list::block_wallet(&mut registry, &mut social_graph, USER3, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
            test_scenario::return_shared(social_graph);
        };
        
        // Verify both users are blocked
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            assert!(block_list::is_blocked(&registry, USER1, USER2), 0);
            assert!(block_list::is_blocked(&registry, USER1, USER3), 1);
            assert!(block_list::blocked_count(&registry, USER1) == 2, 2);
            
            let blocked_wallets = block_list::get_blocked_wallets(&registry, USER1);
            assert!(vector::length(&blocked_wallets) == 2, 3);
            
            test_scenario::return_shared(registry);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test that users cannot block themselves
    #[test]
    #[expected_failure(abort_code = block_list::ECannotBlockSelf, location = social_contracts::block_list)]
    fun test_cannot_block_self() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize block list registry
        init_block_list_registry(&mut scenario);
        
        // Try to block self (should fail - lazy initialization won't happen because validation fails first)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(&mut scenario);
            block_list::block_wallet(&mut registry, &mut social_graph, USER1, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
            test_scenario::return_shared(social_graph);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test that users cannot block the same user twice
    #[test]
    #[expected_failure(abort_code = block_list::EAlreadyBlocked, location = social_contracts::block_list)]
    fun test_already_blocked() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize block list registry
        init_block_list_registry(&mut scenario);
        
        // Block USER2 (block list will be created automatically via lazy initialization)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(&mut scenario);
            block_list::block_wallet(&mut registry, &mut social_graph, USER2, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
            test_scenario::return_shared(social_graph);
        };
        
        // Try to block USER2 again (should fail)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(&mut scenario);
            block_list::block_wallet(&mut registry, &mut social_graph, USER2, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
            test_scenario::return_shared(social_graph);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test that users cannot unblock users who aren't blocked
    #[test]
    #[expected_failure(abort_code = block_list::ENotBlocked, location = social_contracts::block_list)]
    fun test_not_blocked() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize block list registry
        init_block_list_registry(&mut scenario);
        
        // Try to unblock USER2 who isn't blocked (should fail - no block list exists)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            block_list::unblock_wallet(&mut registry, USER2, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test unified table behavior - verify multiple users can block independently
    #[test]
    fun test_unified_table_behavior() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize block list registry
        init_block_list_registry(&mut scenario);
        
        // USER1 blocks USER2
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(&mut scenario);
            block_list::block_wallet(&mut registry, &mut social_graph, USER2, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
            test_scenario::return_shared(social_graph);
        };
        
        // USER2 blocks USER3 (different blocker, should work independently)
        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(&mut scenario);
            block_list::block_wallet(&mut registry, &mut social_graph, USER3, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(registry);
            test_scenario::return_shared(social_graph);
        };
        
        // Verify unified table works correctly
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let registry = test_scenario::take_shared<block_list::BlockListRegistry>(&mut scenario);
            
            // USER1 blocked USER2
            assert!(block_list::is_blocked(&registry, USER1, USER2), 0);
            assert!(block_list::blocked_count(&registry, USER1) == 1, 1);
            
            // USER2 blocked USER3
            assert!(block_list::is_blocked(&registry, USER2, USER3), 2);
            assert!(block_list::blocked_count(&registry, USER2) == 1, 3);
            
            // USER1 did not block USER3
            assert!(!block_list::is_blocked(&registry, USER1, USER3), 4);
            
            test_scenario::return_shared(registry);
        };
        
        test_scenario::end(scenario);
    }
} 