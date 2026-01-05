// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Block list module for the MySocial network
/// Manages user blocking between wallet addresses

#[allow(duplicate_alias, unused_use)]
module social_contracts::block_list {
    use mys::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        event,
        table::{Self, Table},
        vec_set::{Self, VecSet}
    };
    use std::{string, option, vector};
    
    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::social_graph;
    
    /// Error codes
    const EAlreadyBlocked: u64 = 1;
    const ENotBlocked: u64 = 2;
    const ECannotBlockSelf: u64 = 3;
    const EWrongVersion: u64 = 4;
    
    /// Registry to track all block lists
    /// Uses unified table architecture (like SocialGraph) for wallet-level blocking
    /// Works for both users and platforms (both are just addresses)
    public struct BlockListRegistry has key {
        id: UID,
        /// Unified table: blocker_address -> set of blocked addresses
        blocked: Table<address, VecSet<address>>,
        /// Version for upgrades
        version: u64,
    }

    /// Block event
    public struct UserBlockEvent has copy, drop {
        /// The blocker wallet address (who initiated the block)
        blocker: address,
        /// The blocked wallet address (who was blocked)
        blocked: address,
    }

    /// Unblock event
    public struct UserUnblockEvent has copy, drop {
        /// The blocker wallet address (who initiated the unblock)
        blocker: address,
        /// The unblocked wallet address (who was unblocked)
        unblocked: address,
    }

    /// Bootstrap initialization function - creates the block list registry
    public(package) fun bootstrap_init(ctx: &mut TxContext) {
        let registry = BlockListRegistry {
            id: object::new(ctx),
            blocked: table::new(ctx),
            version: upgrade::current_version(),
        };
        
        // Share the registry to make it globally accessible
        transfer::share_object(registry);
    }
    
    /// Test-only initializer for the block list registry
    #[test_only]
    public fun test_init(ctx: &mut TxContext) {
        bootstrap_init(ctx)
    }
    
    /// Internal helper function to block a wallet with a specific blocker address
    /// This allows platforms (shared objects) to block wallets on behalf of their address
    /// Uses unified table architecture with lazy initialization (like following)
    public(package) fun block_wallet_internal(
        registry: &mut BlockListRegistry,
        social_graph: &mut social_graph::SocialGraph,
        blocker_address: address,
        blocked_wallet_address: address
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        // Cannot block self
        assert!(blocker_address != blocked_wallet_address, ECannotBlockSelf);
        
        // Initialize blocker's blocked set if it doesn't exist (lazy initialization)
        if (!table::contains(&registry.blocked, blocker_address)) {
            table::add(&mut registry.blocked, blocker_address, vec_set::empty());
        };
        
        // Get the blocked set and check if already blocked
        let blocked_set = table::borrow_mut(&mut registry.blocked, blocker_address);
        
        // Check if already blocked
        if (vec_set::contains(blocked_set, &blocked_wallet_address)) {
            abort EAlreadyBlocked
        };
        
        // Add to blocked set
        vec_set::insert(blocked_set, blocked_wallet_address);
        
        // Emit block event
        event::emit(UserBlockEvent {
            blocker: blocker_address,
            blocked: blocked_wallet_address,
        });
        
        // Perform bidirectional unfollow after blocking succeeds (wallet-level)
        // Blocker unfollows blocked (if following)
        social_graph::unfollow_internal(social_graph, blocker_address, blocked_wallet_address);
        
        // Blocked unfollows blocker (if following)
        social_graph::unfollow_internal(social_graph, blocked_wallet_address, blocker_address);
        // Continue - blocking succeeds regardless of unfollow results
    }

    /// Block a wallet address
    /// Uses the caller's wallet address as the blocker
    /// Automatically unfollows in both directions if users are following each other
    public entry fun block_wallet(
        registry: &mut BlockListRegistry,
        social_graph: &mut social_graph::SocialGraph,
        blocked_wallet_address: address,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        block_wallet_internal(registry, social_graph, sender, blocked_wallet_address);
    }

    /// Internal helper function to unblock a wallet with a specific blocker address
    public(package) fun unblock_wallet_internal(
        registry: &mut BlockListRegistry,
        blocker_address: address,
        blocked_wallet_address: address
    ) {
        // Check version compatibility
        assert!(registry.version == upgrade::current_version(), EWrongVersion);
        
        // Check if blocker has any blocked addresses
        if (!table::contains(&registry.blocked, blocker_address)) {
            abort ENotBlocked
        };
        
        // Get the blocked set
        let blocked_set = table::borrow_mut(&mut registry.blocked, blocker_address);
        
        // Check if the wallet is actually blocked
        if (!vec_set::contains(blocked_set, &blocked_wallet_address)) {
            abort ENotBlocked
        };
        
        // Remove from blocked set
        vec_set::remove(blocked_set, &blocked_wallet_address);
        
        // Emit unblock event
        event::emit(UserUnblockEvent {
            blocker: blocker_address,
            unblocked: blocked_wallet_address,
        });
    }

    /// Unblock a wallet address
    /// Uses the caller's wallet address as the blocker
    public entry fun unblock_wallet(
        registry: &mut BlockListRegistry,
        blocked_wallet_address: address,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        unblock_wallet_internal(registry, sender, blocked_wallet_address);
    }

    // === PUBLIC API ===

    /// Check if a wallet address is blocked by a blocker
    public fun is_blocked(registry: &BlockListRegistry, blocker: address, blocked: address): bool {
        if (!table::contains(&registry.blocked, blocker)) {
            return false
        };
        
        let blocked_set = table::borrow(&registry.blocked, blocker);
        vec_set::contains(blocked_set, &blocked)
    }

    /// Get the number of blocked wallet addresses for a blocker
    public fun blocked_count(registry: &BlockListRegistry, blocker: address): u64 {
        if (!table::contains(&registry.blocked, blocker)) {
            return 0
        };
        
        let blocked_set = table::borrow(&registry.blocked, blocker);
        vec_set::size(blocked_set)
    }

    /// Get the list of blocked wallet addresses for a blocker
    public fun get_blocked_wallets(registry: &BlockListRegistry, blocker: address): vector<address> {
        if (!table::contains(&registry.blocked, blocker)) {
            return std::vector::empty()
        };
        
        let blocked_set = table::borrow(&registry.blocked, blocker);
        vec_set::into_keys(*blocked_set)
    }

    // === Versioning Functions ===

    /// Get the version of the block list registry
    public fun registry_version(registry: &BlockListRegistry): u64 {
        registry.version
    }

    /// Get a mutable reference to the registry version (for upgrade module)
    public(package) fun borrow_registry_version_mut(registry: &mut BlockListRegistry): &mut u64 {
        &mut registry.version
    }

    /// Migration function for BlockListRegistry
    public entry fun migrate_block_list_registry(
        registry: &mut BlockListRegistry,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(registry.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = registry.version;
        registry.version = current_version;
        
        // Emit event for object migration
        let registry_id = object::id(registry);
        upgrade::emit_migration_event(
            registry_id,
            string::utf8(b"BlockListRegistry"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }
}