// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Social graph module for the MySocial network
/// Manages social relationships between users (following/followers)

#[allow(duplicate_alias, unused_use)]
module social_contracts::social_graph {
    use std::{string, option, vector};
    
    use mys::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        event,
        table::{Self, Table},
        vec_set::{Self, VecSet}
    };
    
    use social_contracts::upgrade;

    /// Error codes
    const EAlreadyFollowing: u64 = 0;
    const ENotFollowing: u64 = 1;
    const ECannotFollowSelf: u64 = 2;
    const EWrongVersion: u64 = 4;

    /// Global social graph object that tracks relationships between wallet addresses
    /// Uses wallet-level architecture - no profile required
    public struct SocialGraph has key {
        id: UID,
        /// Table mapping wallet addresses to sets of addresses they are following
        following: Table<address, VecSet<address>>,
        /// Table mapping wallet addresses to sets of addresses following them
        followers: Table<address, VecSet<address>>,
        /// Current version of the object
        version: u64,
    }

    /// Follow event - emitted when a wallet address follows another wallet address
    public struct FollowEvent has copy, drop {
        /// Wallet address of the follower
        follower: address,
        /// Wallet address being followed
        following: address,
    }

    /// Unfollow event - emitted when a wallet address unfollows another wallet address
    public struct UnfollowEvent has copy, drop {
        /// Wallet address of the unfollower
        follower: address,
        /// Wallet address being unfollowed
        unfollowed: address,
    }

    /// Bootstrap initialization function - creates the social graph shared object
    public(package) fun bootstrap_init(ctx: &mut TxContext) {
        let social_graph = SocialGraph {
            id: object::new(ctx),
            following: table::new(ctx),
            followers: table::new(ctx),
            version: upgrade::current_version(),
        };

        // Share the social graph to make it globally accessible
        transfer::share_object(social_graph);
    }

    #[test_only]
    /// Initialize the social graph for testing
    public fun init_for_testing(ctx: &mut TxContext) {
        bootstrap_init(ctx)
    }

    /// Follow a wallet address
    /// Uses wallet-level architecture - no profile required
    public entry fun follow(
        social_graph: &mut SocialGraph,
        following_address: address,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(social_graph.version == upgrade::current_version(), EWrongVersion);
        
        let sender = tx_context::sender(ctx);
        
        // Cannot follow self
        assert!(sender != following_address, ECannotFollowSelf);
        
        // Initialize follower's following set if it doesn't exist (lazy initialization)
        if (!table::contains(&social_graph.following, sender)) {
            table::add(&mut social_graph.following, sender, vec_set::empty());
        };
        
        // Initialize followed's followers set if it doesn't exist (lazy initialization)
        if (!table::contains(&social_graph.followers, following_address)) {
            table::add(&mut social_graph.followers, following_address, vec_set::empty());
        };
        
        // Get mutable references to the sets
        let follower_following = table::borrow_mut(&mut social_graph.following, sender);
        let following_followers = table::borrow_mut(&mut social_graph.followers, following_address);
        
        // Check if already following
        assert!(!vec_set::contains(follower_following, &following_address), EAlreadyFollowing);
        
        // Add to sets
        vec_set::insert(follower_following, following_address);
        vec_set::insert(following_followers, sender);
        
        // Emit follow event
        event::emit(FollowEvent {
            follower: sender,
            following: following_address,
        });
    }

    /// Unfollow a wallet address
    /// Uses wallet-level architecture - no profile required
    public entry fun unfollow(
        social_graph: &mut SocialGraph,
        following_address: address,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(social_graph.version == upgrade::current_version(), EWrongVersion);
        
        let sender = tx_context::sender(ctx);
        
        // Check if following sets exist
        if (!table::contains(&social_graph.following, sender)) {
            abort ENotFollowing
        };
        if (!table::contains(&social_graph.followers, following_address)) {
            abort ENotFollowing
        };
        
        // Get mutable references to the sets
        let follower_following = table::borrow_mut(&mut social_graph.following, sender);
        let following_followers = table::borrow_mut(&mut social_graph.followers, following_address);
        
        // Check if following
        if (!vec_set::contains(follower_following, &following_address)) {
            abort ENotFollowing
        };
        
        // Remove from sets
        vec_set::remove(follower_following, &following_address);
        vec_set::remove(following_followers, &sender);
        
        // Emit unfollow event
        event::emit(UnfollowEvent {
            follower: sender,
            unfollowed: following_address,
        });
    }

    /// Internal unfollow function that accepts explicit wallet addresses
    /// Used for bidirectional unfollow during blocking operations
    /// Returns true if unfollow occurred, false if not following
    public(package) fun unfollow_internal(
        social_graph: &mut SocialGraph,
        follower_address: address,
        following_address: address
    ): bool {
        // Check if following relationship exists
        if (!is_following(social_graph, follower_address, following_address)) {
            return false  // Not following, nothing to do
        };
        
        // Check if following sets exist (defensive)
        if (!table::contains(&social_graph.following, follower_address)) {
            return false
        };
        if (!table::contains(&social_graph.followers, following_address)) {
            return false
        };
        
        // Get mutable references to the sets
        let follower_following = table::borrow_mut(&mut social_graph.following, follower_address);
        let following_followers = table::borrow_mut(&mut social_graph.followers, following_address);
        
        // Remove if present (defensive check)
        if (vec_set::contains(follower_following, &following_address)) {
            vec_set::remove(follower_following, &following_address);
            vec_set::remove(following_followers, &follower_address);
            
            // Emit unfollow event
            event::emit(UnfollowEvent {
                follower: follower_address,
                unfollowed: following_address,
            });
            
            return true
        };
        
        false
    }

    /// Migrate the social graph to a new version
    /// Only callable by the admin with the AdminCap
    public entry fun migrate_social_graph(
        social_graph: &mut SocialGraph,
        _: &upgrade::UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(social_graph.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = social_graph.version;
        social_graph.version = current_version;
        
        // Emit event for object migration
        let graph_id = object::id(social_graph);
        upgrade::emit_migration_event(
            graph_id,
            string::utf8(b"SocialGraph"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }

    /// Get a mutable reference to the version field (for upgrade module)
    public(package) fun borrow_version_mut(social_graph: &mut SocialGraph): &mut u64 {
        &mut social_graph.version
    }

    // === Getters ===

    /// Get the version of the social graph
    public fun version(social_graph: &SocialGraph): u64 {
        social_graph.version
    }

    /// Check if a wallet address is following another wallet address
    public fun is_following(social_graph: &SocialGraph, follower_address: address, following_address: address): bool {
        if (!table::contains(&social_graph.following, follower_address)) {
            return false
        };
        
        let follower_following = table::borrow(&social_graph.following, follower_address);
        vec_set::contains(follower_following, &following_address)
    }

    /// Get the number of wallet addresses a user is following
    public fun following_count(social_graph: &SocialGraph, wallet_address: address): u64 {
        if (!table::contains(&social_graph.following, wallet_address)) {
            return 0
        };
        
        let following = table::borrow(&social_graph.following, wallet_address);
        vec_set::size(following)
    }

    /// Get the number of followers a wallet address has
    public fun follower_count(social_graph: &SocialGraph, wallet_address: address): u64 {
        if (!table::contains(&social_graph.followers, wallet_address)) {
            return 0
        };
        
        let followers = table::borrow(&social_graph.followers, wallet_address);
        vec_set::size(followers)
    }

    /// Get the list of wallet addresses a user is following
    public fun get_following(social_graph: &SocialGraph, wallet_address: address): vector<address> {
        if (!table::contains(&social_graph.following, wallet_address)) {
            return vector::empty()
        };
        
        let following = table::borrow(&social_graph.following, wallet_address);
        vec_set::into_keys(*following)
    }

    /// Get the list of followers for a wallet address
    public fun get_followers(social_graph: &SocialGraph, wallet_address: address): vector<address> {
        if (!table::contains(&social_graph.followers, wallet_address)) {
            return vector::empty()
        };
        
        let followers = table::borrow(&social_graph.followers, wallet_address);
        vec_set::into_keys(*followers)
    }
}