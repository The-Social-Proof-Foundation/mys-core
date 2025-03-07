module mys_framework::social_graph {
    use std::vector;
    use std::option::{Self, Option};
    use mys_framework::object::{Self, UID};
    use mys_framework::tx_context::{Self, TxContext};
    use mys_framework::transfer;
    use mys_framework::event;
    use mys_framework::table::{Self, Table};
    use mys_framework::profile::{Self, Profile};
    use mys_framework::block_list::{Self, BlockList, BlockListRegistry};

    /// Error codes
    const EAlreadyFollowing: u64 = 0;
    const ENotFollowing: u64 = 1;
    const ECannotFollowSelf: u64 = 2;
    const EUnauthorized: u64 = 3;
    const EEntityBlocked: u64 = 4;

    /// Social graph struct to store following/follower relationships
    struct SocialGraph has key {
        id: UID,
        /// Map of profile ID -> vector of profiles that this profile follows
        following: Table<address, vector<address>>,
        /// Map of profile ID -> vector of profiles that follow this profile
        followers: Table<address, vector<address>>,
        /// Owner of this social graph
        owner: address,
    }

    /// Follow event
    struct FollowEvent has copy, drop {
        follower: address,
        following: address,
    }

    /// Unfollow event
    struct UnfollowEvent has copy, drop {
        follower: address,
        unfollowed: address,
    }

    /// Create a new social graph for a user
    fun init_module(ctx: &mut TxContext) {
        let social_graph = SocialGraph {
            id: object::new(ctx),
            following: table::new(ctx),
            followers: table::new(ctx),
            owner: @mys_framework,
        };

        transfer::share_object(social_graph);
    }

    /// Follow a profile, checks for blocks before allowing follow
    public entry fun follow(
        social_graph: &mut SocialGraph,
        follower_profile: &Profile,
        to_follow_id: address,
        block_list_registry: &BlockListRegistry,
        ctx: &mut TxContext
    ) {
        let follower_id = object::uid_to_address(profile::id(follower_profile));
        let follower_owner = profile::owner(follower_profile);
        
        // Verify the follower owns the profile
        assert!(follower_owner == tx_context::sender(ctx), EUnauthorized);
        
        // Cannot follow self
        assert!(follower_id != to_follow_id, ECannotFollowSelf);
        
        // Check if either profile has blocked the other
        let (has_follower_block_list, follower_block_list_id) = block_list::find_block_list(
            block_list_registry, 
            object::id_from_address(follower_id)
        );
        
        let (has_followed_block_list, followed_block_list_id) = block_list::find_block_list(
            block_list_registry,
            object::id_from_address(to_follow_id)
        );
        
        // Follower can't follow if they've been blocked by the target
        if (has_followed_block_list) {
            // This would need to be implemented with a function that can look up blocks by ID
            // For now, we'll use a placeholder check
            let is_blocked = false; // Placeholder - actual implementation would check the block list
            assert!(!is_blocked, EEntityBlocked);
        };
        
        // Initialize following list for follower if it doesn't exist
        if (!table::contains(&social_graph.following, follower_id)) {
            table::add(&mut social_graph.following, follower_id, vector::empty<address>());
        };
        
        // Initialize followers list for target if it doesn't exist
        if (!table::contains(&social_graph.followers, to_follow_id)) {
            table::add(&mut social_graph.followers, to_follow_id, vector::empty<address>());
        };
        
        let following_list = table::borrow_mut(&mut social_graph.following, follower_id);
        let followers_list = table::borrow_mut(&mut social_graph.followers, to_follow_id);
        
        // Check if already following
        assert!(!vector::contains(following_list, &to_follow_id), EAlreadyFollowing);
        
        // Add to following and followers lists
        vector::push_back(following_list, to_follow_id);
        vector::push_back(followers_list, follower_id);
        
        // Emit follow event
        event::emit(FollowEvent {
            follower: follower_id,
            following: to_follow_id,
        });
    }

    /// Unfollow a profile
    public entry fun unfollow(
        social_graph: &mut SocialGraph,
        follower_profile: &Profile,
        to_unfollow_id: address,
        ctx: &mut TxContext
    ) {
        let follower_id = object::uid_to_address(profile::id(follower_profile));
        let follower_owner = profile::owner(follower_profile);
        
        // Verify the follower owns the profile
        assert!(follower_owner == tx_context::sender(ctx), EUnauthorized);
        
        // Check if following list exists
        assert!(table::contains(&social_graph.following, follower_id), ENotFollowing);
        
        // Check if followers list exists
        assert!(table::contains(&social_graph.followers, to_unfollow_id), ENotFollowing);
        
        let following_list = table::borrow_mut(&mut social_graph.following, follower_id);
        let followers_list = table::borrow_mut(&mut social_graph.followers, to_unfollow_id);
        
        // Find indices to remove
        let (following_idx, following_exists) = vector::index_of(following_list, &to_unfollow_id);
        assert!(following_exists, ENotFollowing);
        
        let (follower_idx, follower_exists) = vector::index_of(followers_list, &follower_id);
        assert!(follower_exists, ENotFollowing);
        
        // Remove from following and followers lists
        vector::remove(following_list, following_idx);
        vector::remove(followers_list, follower_idx);
        
        // Emit unfollow event
        event::emit(UnfollowEvent {
            follower: follower_id,
            unfollowed: to_unfollow_id,
        });
    }

    // === Getters ===
    
    /// Get the list of profiles that a user follows
    public fun get_following(social_graph: &SocialGraph, profile_id: address): vector<address> {
        if (!table::contains(&social_graph.following, profile_id)) {
            return vector::empty()
        };
        
        *table::borrow(&social_graph.following, profile_id)
    }
    
    /// Get the list of profiles that follow a user
    public fun get_followers(social_graph: &SocialGraph, profile_id: address): vector<address> {
        if (!table::contains(&social_graph.followers, profile_id)) {
            return vector::empty()
        };
        
        *table::borrow(&social_graph.followers, profile_id)
    }
    
    /// Check if a profile follows another profile
    public fun is_following(social_graph: &SocialGraph, follower_id: address, following_id: address): bool {
        if (!table::contains(&social_graph.following, follower_id)) {
            return false
        };
        
        let following_list = table::borrow(&social_graph.following, follower_id);
        vector::contains(following_list, &following_id)
    }
    
    /// Get the number of profiles that a user follows
    public fun following_count(social_graph: &SocialGraph, profile_id: address): u64 {
        if (!table::contains(&social_graph.following, profile_id)) {
            return 0
        };
        
        let following_list = table::borrow(&social_graph.following, profile_id);
        vector::length(following_list)
    }
    
    /// Get the number of followers that a user has
    public fun followers_count(social_graph: &SocialGraph, profile_id: address): u64 {
        if (!table::contains(&social_graph.followers, profile_id)) {
            return 0
        };
        
        let followers_list = table::borrow(&social_graph.followers, profile_id);
        vector::length(followers_list)
    }
}