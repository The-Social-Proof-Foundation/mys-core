// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Platform module for MySocial network. This module implements the platform system
/// that allows creating and managing social network platforms with their own tokens,
/// content timelines, and reputation systems.
module mys::platform {
    use std::ascii;
    use std::string::{Self, String};
    use std::vector;
    use mys::object::{Self, UID, ID};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
    use mys::event;
    use mys::url::{Self, Url};
    use mys::coin::{Self, Coin, TreasuryCap, CoinMetadata};
    use mys::balance::{Self, Balance};
    use mys::table::{Self, Table};
    use mys::reputation;
    use mys::events;
    use mys::block_list::{Self, BlockList, BlockListRegistry};
    
    // === Errors ===
    /// Operation can only be performed by the system admin
    const ENotAuthorized: u64 = 0;
    /// User is not the platform owner
    const ENotPlatformOwner: u64 = 1;
    /// Platform with this name already exists
    const EPlatformExists: u64 = 2;
    /// Invalid argument provided
    const EInvalidArgument: u64 = 3;
    /// Platform not found
    const EPlatformNotFound: u64 = 4;
    /// Token already exists for this platform
    const ETokenExists: u64 = 5;
    /// Token operation failed
    const ETokenOperationFailed: u64 = 6;
    /// Post operation failed
    const EPostOperationFailed: u64 = 7;
    /// Reputation operation failed
    const EReputationOperationFailed: u64 = 8;
    /// Invalid supply cap
    const EInvalidSupplyCap: u64 = 9;
    /// Feature not enabled
    const EFeatureNotEnabled: u64 = 10;
    /// Entity is blocked
    const EEntityBlocked: u64 = 11;
    
    // === Constants ===
    // Default initial platform reputation score
    const DEFAULT_INITIAL_REPUTATION: u64 = 500;
    // Minimum reputation score for a platform
    const MIN_REPUTATION_SCORE: u64 = 1;
    // Maximum reputation score for a platform
    const MAX_REPUTATION_SCORE: u64 = 1000;
    
    // === Structs ===
    
    /// Admin capability for the platform system
    struct AdminCap has key, store {
        id: UID,
    }
    
    /// A platform within the MySocial ecosystem
    struct Platform has key {
        id: UID,
        name: String,
        description: String,
        url: String,
        logo_url: Option<Url>,
        owner: address,
        creation_timestamp: u64,
        category: String,
        reputation_score: u64,
        features_enabled: vector<u8>, // Bitmap of enabled features
        token_id: Option<address>,    // Address of the platform token, if created
        total_posts: u64,             // Total number of posts created in this platform
        total_users: u64,             // Total number of users active in this platform
        verified: bool,               // Whether the platform is verified by the system
    }
    
    /// Information about a platform token
    struct PlatformTokenInfo has store, drop, copy {
        platform_id: ID,              // The platform this token belongs to
        token_id: address,            // The token's type ID
        total_supply: u64,            // Maximum token supply
        circulating_supply: u64,      // Current supply in circulation
        owner: address,               // Platform owner address
    }
    
    /// Registry of all platforms in the system
    struct PlatformRegistry has key {
        id: UID,
        // Map from platform name to platform ID
        platforms_by_name: Table<String, ID>,
        // Map from platform ID to token info
        platform_tokens: Table<ID, PlatformTokenInfo>,
        // List of all platforms
        platforms: vector<ID>,
        // Total number of platforms
        total_platforms: u64,
    }
    
    /// Request to create a token for a platform
    struct TokenCreationRequest has key, store {
        id: UID,
        platform_id: ID,             // The platform requesting a token
        requester: address,          // The address of the requester (should be platform owner)
        symbol: String,
        name: String,
        description: String,
        icon_url: Option<Url>,
        supply_cap: u64,             // Maximum supply cap for the token
    }
    
    /// Request to create a new platform
    struct PlatformCreationRequest has key, store {
        id: UID,
        name: String,
        description: String,
        url: String,
        logo_url: Option<Url>,
        owner: address,
        category: String,
    }
    
    /// Timeline for a platform
    struct Timeline has key {
        id: UID,
        platform_id: ID,
        post_ids: vector<ID>,        // Posts in chronological order
        featured_post_ids: vector<ID>, // Featured/pinned posts
        last_updated: u64,
    }
    
    /// Statistics for a platform
    /// Simplified version with minimal on-chain state, most analytics delegated to indexers
    struct PlatformStats has key {
        id: UID,
        platform_id: ID,
        // Keep reputation_history for on-chain reputation verification
        reputation_history: vector<u64>,     // Historical reputation scores
        // Add last_updated field for sync points
        last_updated: u64,                  // Timestamp of last update
    }
    
    // === Feature flags ===
    // Features that can be enabled for platforms
    const FEATURE_TOKEN: u8 = 0;            // Platform has its own token
    const FEATURE_MODERATION: u8 = 1;       // Platform has moderation capabilities
    const FEATURE_ADVERTISING: u8 = 2;      // Platform can host advertisements
    const FEATURE_VERIFIED_USERS: u8 = 3;   // Platform can verify users
    const FEATURE_PREMIUM_CONTENT: u8 = 4;  // Platform can host premium content
    const FEATURE_NFT_CONTENT: u8 = 5;      // Platform can mint NFTs
    const FEATURE_ENCRYPTED_CONTENT: u8 = 6; // Platform has encrypted content
    
    // === Events ===
    
    /// Event emitted when a new platform is created
    struct PlatformCreatedEvent has copy, drop {
        platform_id: ID,
        name: String,
        owner: address,
        category: String,
        timestamp: u64,
    }
    
    /// Event emitted when a platform token is created
    struct PlatformTokenCreatedEvent has copy, drop {
        platform_id: ID,
        token_id: address,
        name: String,
        symbol: String,
        supply_cap: u64,
        timestamp: u64,
    }
    
    /// Event emitted when platform token supply is adjusted
    struct TokenSupplyChangedEvent has copy, drop {
        platform_id: ID,
        token_id: address,
        old_supply: u64,
        new_supply: u64,
        reason: String,
        timestamp: u64,
    }
    
    /// Event emitted when a post is added to a platform
    struct PostAddedEvent has copy, drop {
        platform_id: ID,
        post_id: ID,
        author: address,
        timestamp: u64,
    }
    
    // Reputation events now centralized in reputation.move
    
    // === Initialization ===
    
    /// Initialize the platform system
    fun init(ctx: &mut TxContext) {
        // Create and transfer admin capability to the transaction sender
        transfer::transfer(
            AdminCap {
                id: object::new(ctx)
            },
            tx_context::sender(ctx)
        );
        
        // Create and share platform registry
        transfer::share_object(
            PlatformRegistry {
                id: object::new(ctx),
                platforms_by_name: table::new(ctx),
                platform_tokens: table::new(ctx),
                platforms: vector::empty(),
                total_platforms: 0,
            }
        );
    }
    
    // === Admin Functions ===
    
    /// Create a request for a new platform
    public entry fun create_platform_request(
        name: vector<u8>,
        description: vector<u8>,
        url: vector<u8>,
        logo_url: Option<Url>,
        category: vector<u8>,
        ctx: &mut TxContext
    ) {
        let request = PlatformCreationRequest {
            id: object::new(ctx),
            name: string::utf8(name),
            description: string::utf8(description),
            url: string::utf8(url),
            logo_url,
            owner: tx_context::sender(ctx),
            category: string::utf8(category),
        };
        
        // Transfer the request to transaction sender (who can then send it to admin)
        transfer::transfer(request, tx_context::sender(ctx));
    }
    
    /// Approve platform creation (admin only)
    public entry fun approve_platform(
        _admin_cap: &AdminCap,
        registry: &mut PlatformRegistry,
        request: PlatformCreationRequest,
        ctx: &mut TxContext
    ) {
        let PlatformCreationRequest {
            id,
            name,
            description,
            url,
            logo_url,
            owner,
            category,
        } = request;
        
        // Clean up request ID
        object::delete(id);
        
        // Ensure platform with this name doesn't already exist
        assert!(!table::contains(&registry.platforms_by_name, name), EPlatformExists);
        
        // Create the platform
        let platform = Platform {
            id: object::new(ctx),
            name: name,
            description,
            url,
            logo_url,
            owner,
            creation_timestamp: tx_context::epoch_timestamp_ms(ctx),
            category,
            reputation_score: DEFAULT_INITIAL_REPUTATION,
            features_enabled: vector::empty(),
            token_id: option::none(),
            total_posts: 0,
            total_users: 0,
            verified: false,
        };
        
        let platform_id = object::id(&platform);
        
        // Update registry
        table::add(&mut registry.platforms_by_name, name, platform_id);
        vector::push_back(&mut registry.platforms, platform_id);
        registry.total_platforms = registry.total_platforms + 1;
        
        // Create platform timeline
        let timeline = Timeline {
            id: object::new(ctx),
            platform_id,
            post_ids: vector::empty(),
            featured_post_ids: vector::empty(),
            last_updated: tx_context::epoch_timestamp_ms(ctx),
        };
        
        // Create platform stats (simplified)
        let stats = PlatformStats {
            id: object::new(ctx),
            platform_id,
            reputation_history: vector::empty(),
            last_updated: tx_context::epoch_timestamp_ms(ctx),
        };
        
        // Emit event using the standardized events module
        events::emit_platform_created(
            tx_context::epoch_timestamp_ms(ctx),
            platform_id,
            string::bytes(&name),
            owner,
            string::bytes(&category)
        );
        
        // Transfer platform to owner
        transfer::transfer(platform, owner);
        
        // Share timeline and stats
        transfer::share_object(timeline);
        transfer::share_object(stats);
    }
    
    /// Request to create a token for a platform
    public entry fun create_token_request(
        platform: &Platform,
        symbol: vector<u8>,
        name: vector<u8>,
        description: vector<u8>,
        icon_url: Option<Url>,
        supply_cap: u64,
        ctx: &mut TxContext
    ) {
        // Only platform owner can request token creation
        assert!(platform.owner == tx_context::sender(ctx), ENotPlatformOwner);
        
        // Platform should not already have a token
        assert!(option::is_none(&platform.token_id), ETokenExists);
        
        // Create the token request
        let request = TokenCreationRequest {
            id: object::new(ctx),
            platform_id: object::id(platform),
            requester: tx_context::sender(ctx),
            symbol: string::utf8(symbol),
            name: string::utf8(name),
            description: string::utf8(description),
            icon_url,
            supply_cap,
        };
        
        // Transfer request to sender (who can then transfer to admin)
        transfer::transfer(request, tx_context::sender(ctx));
    }
    
    /// Approve token creation for a platform (admin only)
    public entry fun approve_token_creation<T: drop>(
        _admin_cap: &AdminCap,
        registry: &mut PlatformRegistry,
        platform: &mut Platform,
        request: TokenCreationRequest,
        decimals: u8,
        ctx: &mut TxContext
    ) {
        // Verify the request matches the platform
        assert!(request.platform_id == object::id(platform), EPlatformNotFound);
        
        // Verify the requester is the platform owner
        assert!(request.requester == platform.owner, ENotPlatformOwner);
        
        // Platform should not already have a token
        assert!(option::is_none(&platform.token_id), ETokenExists);
        
        // Extract request info
        let TokenCreationRequest {
            id,
            platform_id,
            requester: _,
            symbol,
            name,
            description,
            icon_url,
            supply_cap
        } = request;
        
        // Clean up request ID
        object::delete(id);
        
        // Create the coin currency with treasury cap
        let (treasury_cap, metadata) = coin::create_currency(
            T {},
            decimals,
            string::bytes(&symbol),
            string::bytes(&name),
            string::bytes(&description),
            icon_url,
            ctx,
        );
        
        // Get the token's type ID
        let token_id = tx_context::type_into_address<T>();
        
        // Create and store token info
        let token_info = PlatformTokenInfo {
            platform_id,
            token_id,
            total_supply: supply_cap,
            circulating_supply: 0,
            owner: platform.owner,
        };
        
        table::add(&mut registry.platform_tokens, platform_id, token_info);
        
        // Update platform with token ID
        platform.token_id = option::some(token_id);
        
        // Enable token feature for platform
        if (!has_feature(platform, FEATURE_TOKEN)) {
            enable_feature(platform, FEATURE_TOKEN);
        };
        
        // Emit event using the standardized events module
        events::emit_token_created(
            tx_context::epoch_timestamp_ms(ctx),
            token_id,
            string::bytes(&name),
            string::bytes(&symbol),
            true, // is platform token
            platform.owner,
            supply_cap
        );
        
        // Transfer treasury cap to platform owner
        transfer::transfer(treasury_cap, platform.owner);
        
        // Share metadata
        transfer::share_object(metadata);
    }
    
    /// Mint tokens for a platform (platform owner only)
    public entry fun mint_platform_tokens<T>(
        registry: &mut PlatformRegistry,
        platform: &Platform,
        treasury_cap: &mut TreasuryCap<T>,
        amount: u64,
        recipient: address,
        ctx: &mut TxContext
    ) {
        // Verify sender is platform owner
        assert!(platform.owner == tx_context::sender(ctx), ENotPlatformOwner);
        
        // Verify platform has the token feature enabled
        assert!(has_feature(platform, FEATURE_TOKEN), EFeatureNotEnabled);
        
        // Verify token matches platform
        let token_id = tx_context::type_into_address<T>();
        assert!(option::contains(&platform.token_id, &token_id), ETokenOperationFailed);
        
        // Get platform ID
        let platform_id = object::id(platform);
        
        // Get current platform token info
        let token_info = table::borrow_mut(&mut registry.platform_tokens, platform_id);
        
        // Verify minting doesn't exceed supply cap
        assert!(token_info.circulating_supply + amount <= token_info.total_supply, EInvalidSupplyCap);
        
        // Update circulating supply
        let old_supply = token_info.circulating_supply;
        token_info.circulating_supply = token_info.circulating_supply + amount;
        
        // Mint tokens
        let minted_coin = coin::mint(treasury_cap, amount, ctx);
        
        // Emit supply change event using the standardized events module
        events::emit_token_supply_changed(
            tx_context::epoch_timestamp_ms(ctx),
            token_id,
            platform.owner,
            old_supply,
            token_info.circulating_supply,
            b"mint"
        );
        
        // Transfer to recipient
        transfer::public_transfer(minted_coin, recipient);
    }
    
    /// Burn platform tokens (platform owner only)
    public entry fun burn_platform_tokens<T>(
        registry: &mut PlatformRegistry,
        platform: &Platform,
        treasury_cap: &mut TreasuryCap<T>,
        coin: Coin<T>,
        ctx: &mut TxContext
    ) {
        // Verify sender is platform owner
        assert!(platform.owner == tx_context::sender(ctx), ENotPlatformOwner);
        
        // Verify platform has the token feature enabled
        assert!(has_feature(platform, FEATURE_TOKEN), EFeatureNotEnabled);
        
        // Verify token matches platform
        let token_id = tx_context::type_into_address<T>();
        assert!(option::contains(&platform.token_id, &token_id), ETokenOperationFailed);
        
        // Get platform ID
        let platform_id = object::id(platform);
        
        // Get current platform token info
        let token_info = table::borrow_mut(&mut registry.platform_tokens, platform_id);
        
        // Get amount being burned
        let amount = coin::value(&coin);
        
        // Update circulating supply
        let old_supply = token_info.circulating_supply;
        token_info.circulating_supply = token_info.circulating_supply - amount;
        
        // Burn tokens
        coin::burn(treasury_cap, coin);
        
        // Emit supply change event using the standardized events module
        events::emit_token_supply_changed(
            tx_context::epoch_timestamp_ms(ctx),
            token_id,
            platform.owner,
            old_supply,
            token_info.circulating_supply,
            b"burn"
        );
    }
    
    /// Update platform supply cap (admin only)
    public entry fun update_supply_cap(
        _admin_cap: &AdminCap,
        registry: &mut PlatformRegistry,
        platform: &Platform,
        new_supply_cap: u64,
        ctx: &mut TxContext
    ) {
        // Verify platform has a token
        assert!(option::is_some(&platform.token_id), ETokenOperationFailed);
        
        // Get platform ID
        let platform_id = object::id(platform);
        
        // Get current platform token info
        let token_info = table::borrow_mut(&mut registry.platform_tokens, platform_id);
        
        // Verify new cap is not less than circulating supply
        assert!(new_supply_cap >= token_info.circulating_supply, EInvalidSupplyCap);
        
        // Update supply cap
        let old_supply = token_info.total_supply;
        token_info.total_supply = new_supply_cap;
        
        // Emit supply change event using the standardized events module
        events::emit_token_supply_changed(
            tx_context::epoch_timestamp_ms(ctx),
            token_info.token_id,
            token_info.owner,
            old_supply,
            new_supply_cap,
            b"cap_update"
        );
    }
    
    // === Platform Management Functions ===
    
    /// Add a post to a platform's timeline
    /// Simplified version with just essential on-chain state changes
    /// Also checks for blocks before allowing post creation
    public entry fun add_post(
        platform: &mut Platform,
        timeline: &mut Timeline,
        stats: &mut PlatformStats,
        post_id: ID,
        has_media: bool,
        content_hash: vector<u8>,
        profile_id: ID,
        block_list_registry: &BlockListRegistry,
        ctx: &mut TxContext
    ) {
        // Verify the timeline belongs to the platform
        assert!(timeline.platform_id == object::id(platform), EPlatformNotFound);
        
        // Verify the stats belong to the platform
        assert!(stats.platform_id == object::id(platform), EPlatformNotFound);
        
        // Check if profile is blocked by platform
        let platform_id = object::id(platform);
        
        // Check if the platform has blocked this profile
        let (has_platform_block_list, _) = block_list::find_block_list(block_list_registry, platform_id);
        if (has_platform_block_list) {
            assert!(!is_profile_blocked(platform, profile_id), EEntityBlocked);
        };
        
        // Add post to timeline
        vector::push_back(&mut timeline.post_ids, post_id);
        
        // Update timeline
        timeline.last_updated = tx_context::epoch_timestamp_ms(ctx);
        
        // Update platform total posts counter (minimal on-chain state)
        platform.total_posts = platform.total_posts + 1;
        
        // Emit event using the standardized events module - this will be used by the indexer
        events::emit_post_created(
            tx_context::epoch_timestamp_ms(ctx),
            post_id,
            platform_id,
            tx_context::sender(ctx),
            has_media,
            content_hash
        );
    }
    
    /// Feature a post on a platform's timeline
    public entry fun feature_post(
        platform: &Platform,
        timeline: &mut Timeline,
        post_id: ID,
        ctx: &mut TxContext
    ) {
        // Verify sender is platform owner
        assert!(platform.owner == tx_context::sender(ctx), ENotPlatformOwner);
        
        // Verify the timeline belongs to the platform
        assert!(timeline.platform_id == object::id(platform), EPlatformNotFound);
        
        // Add post to featured posts if not already featured
        if (!vector::contains(&timeline.featured_post_ids, &post_id)) {
            vector::push_back(&mut timeline.featured_post_ids, post_id);
        };
        
        // Update timeline
        timeline.last_updated = tx_context::epoch_timestamp_ms(ctx);
    }
    
    /// Enable a feature for a platform (admin only)
    public entry fun enable_platform_feature(
        _admin_cap: &AdminCap,
        platform: &mut Platform,
        feature: u8,
        ctx: &mut TxContext
    ) {
        enable_feature(platform, feature);
    }
    
    /// Verify a platform (admin only)
    public entry fun verify_platform(
        _admin_cap: &AdminCap,
        platform: &mut Platform,
        _ctx: &mut TxContext
    ) {
        platform.verified = true;
    }
    
    /// Update platform reputation (admin only)
    /// This is now a wrapper around the centralized reputation module
    public entry fun update_reputation(
        admin_cap: &AdminCap,
        platform: &mut Platform,
        stats: &mut PlatformStats,
        new_score: u64,
        reason: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Update reputation through the centralized reputation module
        // The reputation module will handle all validation, history tracking, and events
        reputation::update_platform_reputation_internal(
            platform,
            stats,
            new_score,
            reason,
            ctx
        );
    }
    
    /// Record user activity on the platform
    /// Simplified version that only updates total user count and emits an event,
    /// delegating analytics to the indexer
    public entry fun record_user_activity(
        platform: &mut Platform,
        stats: &mut PlatformStats,
        ctx: &mut TxContext
    ) {
        // Verify stats belong to platform
        assert!(stats.platform_id == object::id(platform), EPlatformNotFound);
        
        // Update platform stats (just increment total users)
        platform.total_users = platform.total_users + 1;
        
        // Emit a standardized event for this user activity that can be indexed
        events::emit_entity_updated(
            tx_context::epoch_timestamp_ms(ctx),
            b"platform_user",
            object::id(platform),
            tx_context::sender(ctx),
            b"new_user"
        );
    }
    
    /// Record engagement on the platform (likes, comments, shares)
    /// Simplified version that just emits an event for the indexer
    public entry fun record_engagement(
        platform: &Platform,
        stats: &mut PlatformStats,
        engagement_type: u8, // 0 = like, 1 = comment, 2 = share, etc.
        ctx: &mut TxContext
    ) {
        // Verify stats belong to platform
        assert!(stats.platform_id == object::id(platform), EPlatformNotFound);
        
        // Convert engagement type to a string for the event
        let engagement_type_str = if (engagement_type == 0) {
            b"like"
        } else if (engagement_type == 1) {
            b"comment"
        } else if (engagement_type == 2) {
            b"share"
        } else {
            b"other"
        };
        
        // Emit a standardized event for this engagement that can be indexed
        events::emit_entity_updated(
            tx_context::epoch_timestamp_ms(ctx),
            b"platform_engagement",
            object::id(platform),
            tx_context::sender(ctx),
            engagement_type_str
        );
    }
    
    // === Helper Functions ===
    
    /// Check if a platform has a specific feature enabled
    fun has_feature(platform: &Platform, feature: u8): bool {
        if (feature >= 8) return false;
        
        let i = 0;
        let len = vector::length(&platform.features_enabled);
        
        while (i < len) {
            if (*vector::borrow(&platform.features_enabled, i) == feature) {
                return true
            };
            i = i + 1;
        };
        
        false
    }
    
    /// Enable a feature for a platform
    fun enable_feature(platform: &mut Platform, feature: u8) {
        if (feature >= 8) return;
        
        if (!has_feature(platform, feature)) {
            vector::push_back(&mut platform.features_enabled, feature);
        };
    }
    
    // === Public Accessor Functions ===
    
    /// Get platform token info
    public fun get_platform_token_info(
        registry: &PlatformRegistry,
        platform_id: ID
    ): (bool, PlatformTokenInfo) {
        if (table::contains(&registry.platform_tokens, platform_id)) {
            (true, *table::borrow(&registry.platform_tokens, platform_id))
        } else {
            (false, PlatformTokenInfo {
                platform_id,
                token_id: @0x0,
                total_supply: 0,
                circulating_supply: 0,
                owner: @0x0,
            })
        }
    }
    
    /// Get platform ID by name
    public fun get_platform_id_by_name(
        registry: &PlatformRegistry,
        name: String
    ): (bool, ID) {
        if (table::contains(&registry.platforms_by_name, name)) {
            (true, *table::borrow(&registry.platforms_by_name, name))
        } else {
            (false, object::id_from_address(@0x0))
        }
    }
    
    /// Get all platform IDs
    public fun get_all_platform_ids(registry: &PlatformRegistry): vector<ID> {
        registry.platforms
    }
    
    /// Get platform reputation score
    public fun get_platform_reputation(platform: &Platform): u64 {
        platform.reputation_score
    }
    
    /// Get platform token ID
    public fun get_platform_token_id(platform: &Platform): (bool, address) {
        if (option::is_some(&platform.token_id)) {
            (true, *option::borrow(&platform.token_id))
        } else {
            (false, @0x0)
        }
    }
    
    /// Check if platform is verified
    public fun is_platform_verified(platform: &Platform): bool {
        platform.verified
    }
    
    /// Get platform stats (simplified)
    /// Just returns the core metrics tracked on-chain
    public fun get_platform_stats(platform: &Platform, stats: &PlatformStats): (u64, u64, u64) {
        assert!(stats.platform_id == object::id(platform), EPlatformNotFound);
        
        (
            platform.total_posts,
            platform.total_users,
            stats.last_updated
        )
    }
    
    /// Get platform timeline info
    public fun get_platform_timeline(platform: &Platform, timeline: &Timeline): (vector<ID>, vector<ID>, u64) {
        assert!(timeline.platform_id == object::id(platform), EPlatformNotFound);
        
        (
            timeline.post_ids,
            timeline.featured_post_ids,
            timeline.last_updated
        )
    }
    
    // === Block List Functions ===
    
    /// Check if a profile is blocked by a platform
    public fun is_profile_blocked(platform: &Platform, profile_id: ID): bool {
        // This function accesses the block list object and checks if the profile is blocked
        // In practice, this would need to access the platform's block list
        // For this implementation, we'll rely on the block_list module's functionality directly
        
        // Create a temporary dummy implementation
        false
    }
    
    /// Check if a profile is blocked by a platform using platform ID
    public fun is_profile_blocked_by_id(platform_id: ID, profile_id: ID): bool {
        // This function is a static version that uses IDs instead of references
        // It would look up the block list for the platform and check if the profile is blocked
        
        // Create a temporary dummy implementation
        false
    }
    
    /// Block a profile from interacting with a platform
    public entry fun block_profile(
        platform: &Platform,
        block_list: &mut BlockList,
        profile_id: ID,
        reason: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify sender is platform owner
        assert!(platform.owner == tx_context::sender(ctx), ENotPlatformOwner);
        
        // Verify block list belongs to platform
        assert!(block_list.owner_id == object::id(platform), ENotAuthorized);
        
        // Use block_list module to add the block
        block_list::block_entity(block_list, profile_id, reason, ctx);
    }
    
    /// Unblock a profile
    public entry fun unblock_profile(
        platform: &Platform,
        block_list: &mut BlockList,
        profile_id: ID,
        ctx: &mut TxContext
    ) {
        // Verify sender is platform owner
        assert!(platform.owner == tx_context::sender(ctx), ENotPlatformOwner);
        
        // Verify block list belongs to platform
        assert!(block_list.owner_id == object::id(platform), ENotAuthorized);
        
        // Use block_list module to remove the block
        block_list::unblock_entity(block_list, profile_id, ctx);
    }
}