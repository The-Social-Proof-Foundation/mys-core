// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

/// Token Exchange module for MySocial platform.
/// This module provides functionality for creation and trading of both profile tokens
/// and post tokens using an Automated Market Maker (AMM) with a quadratic pricing curve.
/// It includes fee distribution mechanisms for transactions, splitting between profile owner,
/// platform, and ecosystem treasury.
#[allow(unused_use, duplicate_alias, unused_const, unused_field, deprecated_usage)]
module social_contracts::token_exchange {
    use std::string::{Self, String};
    use std::ascii;
    use std::vector;
    use std::option::{Self, Option};
    
    use mys::object::{Self, UID, ID};
    use mys::tx_context::{Self, TxContext};
    use mys::transfer;
    use mys::event;
    use mys::table::{Self, Table};
    use mys::coin::{Self, Coin};
    use mys::mys::MYS;
    use mys::balance::{Self, Balance};
    use mys::clock::{Self, Clock};
    use mys::math;
    use mys::linked_table::{Self, LinkedTable};
    
    use social_contracts::profile::{Self, Profile, UsernameRegistry};
    use social_contracts::post::{Self, Post};
    use social_contracts::block_list::{Self, BlockListRegistry};

    // === Error codes ===
    /// Operation can only be performed by the admin
    const ENotAuthorized: u64 = 0;
    /// Invalid fee percentages configuration
    const EInvalidFeeConfig: u64 = 1;
    /// The token already exists
    const ETokenAlreadyExists: u64 = 2;
    /// The token does not exist
    const ETokenNotFound: u64 = 3;
    /// Exceeded maximum token hold percentage
    const EExceededMaxHold: u64 = 4;
    /// Insufficient funds for operation
    const EInsufficientFunds: u64 = 5;
    /// Sender doesn't own any tokens
    const ENoTokensOwned: u64 = 6;
    /// Invalid post or profile ID
    const EInvalidID: u64 = 7;
    /// Insufficient token liquidity
    const EInsufficientLiquidity: u64 = 8;
    /// Self trading not allowed
    const ESelfTrading: u64 = 9;
    /// Token already initialized in pool
    const ETokenAlreadyInitialized: u64 = 10;
    /// Curve parameters must be positive
    const EInvalidCurveParams: u64 = 11;
    /// Invalid token type
    const EInvalidTokenType: u64 = 12;
    /// Viral threshold not met
    const EViralThresholdNotMet: u64 = 13;
    /// Auction already in progress
    const EAuctionInProgress: u64 = 14;
    /// Invalid auction duration
    const EInvalidAuctionDuration: u64 = 15;
    /// Auction not active
    const EAuctionNotActive: u64 = 16;
    /// Auction not ended
    const EAuctionNotEnded: u64 = 17;
    /// Auction already finalized
    const EAuctionAlreadyFinalized: u64 = 18;
    /// No contribution to auction
    const ENoContribution: u64 = 19;
    /// Cannot buy token from a blocked user
    const EBlockedUser: u64 = 20;
    /// Invalid order ID
    const EInvalidOrderId: u64 = 21;
    /// Unauthorized order cancellation
    const EUnauthorizedCancel: u64 = 22;
    /// Tree not empty
    const ETreeNotEmpty: u64 = 23;
    /// Key already exists
    const EKeyAlreadyExist: u64 = 24;
    /// Leaf does not exist
    const ELeafNotExist: u64 = 25;
    /// Null parent
    const ENullParent: u64 = 26;
    /// Index out of range
    const EIndexOutOfRange: u64 = 27;
    /// Exceed capacity
    const EExceedCapacity: u64 = 28;

    // === Constants ===
    // Token types
    const TOKEN_TYPE_PROFILE: u8 = 1;
    const TOKEN_TYPE_POST: u8 = 2;
    
    // Critbit tree constants
    const PARTITION_INDEX: u64 = 0x8000000000000000; // 9223372036854775808
    const MAX_U64: u64 = 0xFFFFFFFFFFFFFFFF; // 18446744073709551615
    const MAX_CAPACITY: u64 = 0x7FFFFFFFFFFFFFFF;
    
    // Order types
    const ORDER_TYPE_BID: bool = true;
    const ORDER_TYPE_ASK: bool = false;

    // Default fee percentages (in basis points, 10000 = 100%)
    const DEFAULT_TOTAL_FEE_BPS: u64 = 150; // 1.5% total fee
    const DEFAULT_CREATOR_FEE_BPS: u64 = 100; // 1.0% to creator (profile/post owner)
    const DEFAULT_PLATFORM_FEE_BPS: u64 = 25; // 0.25% to platform
    const DEFAULT_TREASURY_FEE_BPS: u64 = 25; // 0.25% to ecosystem treasury

    // Maximum hold percentage per wallet (5% of supply)
    const MAX_HOLD_PERCENT_BPS: u64 = 500;

    // Default AMM curve parameters
    const DEFAULT_BASE_PRICE: u64 = 100_000_000; // 0.1 MYS in smallest units
    const DEFAULT_QUADRATIC_COEFFICIENT: u64 = 100_000; // Coefficient for quadratic curve

    // Viral threshold constants for posts
    const POST_LIKES_WEIGHT: u64 = 1;
    const POST_COMMENTS_WEIGHT: u64 = 3;
    const POST_TIPS_WEIGHT: u64 = 10;
    const POST_VIRAL_THRESHOLD: u64 = 100;

    // Viral threshold constants for profiles
    const PROFILE_FOLLOWS_WEIGHT: u64 = 1;
    const PROFILE_POSTS_WEIGHT: u64 = 1;
    const PROFILE_TIPS_WEIGHT: u64 = 5;
    const PROFILE_VIRAL_THRESHOLD: u64 = 100;

    // Auction duration limits (in seconds)
    const MIN_POST_AUCTION_DURATION: u64 = 1 * 60 * 60; // 1 hour
    const MAX_POST_AUCTION_DURATION: u64 = 3 * 60 * 60; // 3 hours
    const MIN_PROFILE_AUCTION_DURATION: u64 = 24 * 60 * 60; // 1 day
    const MAX_PROFILE_AUCTION_DURATION: u64 = 72 * 60 * 60; // 3 days

    // Auction status
    const AUCTION_STATUS_PENDING: u8 = 0;
    const AUCTION_STATUS_ACTIVE: u8 = 1;
    const AUCTION_STATUS_ENDED: u8 = 2;
    const AUCTION_STATUS_FINALIZED: u8 = 3;
    
    // === Critbit Tree Data Structures ===
    
    /// Leaf node in the Critbit Tree representing a price level
    public struct Leaf<V> has store, drop {
        key: u64,
        value: V,
        parent: u64,
    }

    /// Internal node in the Critbit Tree
    public struct InternalNode has store, drop {
        mask: u64,
        left_child: u64,
        right_child: u64,
        parent: u64,
    }

    /// Critbit Tree - a binary search tree optimized for fast lookups
    public struct CritbitTree<V: store> has store {
        root: u64,
        internal_nodes: Table<u64, InternalNode>,
        leaves: Table<u64, Leaf<V>>,
        min_leaf: u64,
        max_leaf: u64,
        next_internal_node_index: u64,
        next_leaf_index: u64
    }
    
    /// Price level in the order book
    public struct PriceLevel has store {
        price: u64,
        open_orders: LinkedTable<u64, Order>,
        // Vector to track order IDs at this price level since linked_table doesn't provide keys()
        order_ids: vector<u64>,
    }
    
    /// Order in the order book
    public struct Order has store, copy, drop {
        order_id: u64,
        client_order_id: u64,
        price: u64,
        original_quantity: u64,
        quantity: u64,
        is_bid: bool,
        owner: address,
        expire_timestamp: u64,
    }
    
    /// Order book for a token pool
    public struct OrderBook has store {
        bids: CritbitTree<PriceLevel>,
        asks: CritbitTree<PriceLevel>,
        next_order_id: u64,
        user_orders: Table<address, LinkedTable<u64, u64>>,
    }

    // === Structs ===

    /// Admin capability for the token exchange
    public struct AdminCap has key, store {
        id: UID,
    }

    /// Global exchange configuration
    public struct ExchangeConfig has key {
        id: UID,
        /// Total fee percentage in basis points
        total_fee_bps: u64,
        /// Creator fee percentage in basis points
        creator_fee_bps: u64,
        /// Platform fee percentage in basis points
        platform_fee_bps: u64,
        /// Treasury fee percentage in basis points
        treasury_fee_bps: u64,
        /// Base price for new tokens
        base_price: u64,
        /// Quadratic coefficient for pricing curve
        quadratic_coefficient: u64,
        /// Platform treasury address
        platform_treasury: address,
        /// Ecosystem treasury address
        ecosystem_treasury: address,
        /// Maximum percentage a single wallet can hold of any token
        max_hold_percent_bps: u64,
    }

    /// Registry of all tokens in the exchange
    public struct TokenRegistry has key {
        id: UID,
        /// Table from token ID to token info
        tokens: Table<address, TokenInfo>,
        /// Table from profile/post ID to auction info
        auctions: Table<address, AuctionInfo>,
    }

    /// Information about a token
    public struct TokenInfo has store, copy, drop {
        /// The token ID (object ID of the pool)
        id: address,
        /// Type of token (1=profile, 2=post)
        token_type: u8,
        /// Owner/creator of the token
        owner: address,
        /// Associated profile or post ID
        associated_id: address,
        /// Token symbol
        symbol: String,
        /// Token name
        name: String,
        /// Current supply in circulation
        circulating_supply: u64,
        /// Base price for this token
        base_price: u64,
        /// Quadratic coefficient for this token's pricing curve
        quadratic_coefficient: u64,
        /// Creation timestamp
        created_at: u64,
    }

    /// Liquidity pool for a token
    public struct TokenPool has key {
        id: UID,
        /// The token's info
        info: TokenInfo,
        /// MYS balance in the pool
        mys_balance: Balance<MYS>,
        /// Mapping of holders' addresses to their token balances
        holders: Table<address, u64>,
        /// Order book for limit orders
        order_book: OrderBook,
    }

    /// Social token that represents a user's owned tokens
    public struct SocialToken has key, store {
        id: UID,
        /// Token pool ID
        pool_id: address,
        /// Token type (1=profile, 2=post)
        token_type: u8,
        /// Amount of tokens held
        amount: u64,
    }

    /// Information about an auction
    public struct AuctionInfo has store, copy, drop {
        /// Associated profile or post ID
        associated_id: address,
        /// Token type (1=profile, 2=post)
        token_type: u8,
        /// Owner of the profile/post
        owner: address,
        /// Status of the auction
        status: u8, // 0=pending, 1=active, 2=ended, 3=finalized
        /// Time when the auction was started
        start_time: u64,
        /// Duration of the auction in seconds
        duration: u64,
        /// Total MYS contributed to the auction
        total_contribution: u64,
        /// Total tokens to be distributed
        total_tokens: u64,
        /// List of contributors' addresses
        contributors: vector<address>,
    }

    /// Pre-launch auction pool
    public struct AuctionPool has key {
        id: UID,
        /// Auction info
        info: AuctionInfo,
        /// MYS balance contributed to the auction
        mys_balance: Balance<MYS>,
        /// Mapping of contributors' addresses to their MYS contributions
        contributions: Table<address, u64>,
    }

    // === Events ===

    /// Event emitted when a token pool is created
    public struct TokenPoolCreatedEvent has copy, drop {
        id: address,
        token_type: u8,
        owner: address,
        associated_id: address,
        symbol: String,
        name: String,
        base_price: u64,
        quadratic_coefficient: u64,
    }

    /// Event emitted when tokens are bought
    public struct TokenBoughtEvent has copy, drop {
        id: address,
        buyer: address,
        amount: u64,
        mys_amount: u64,
        fee_amount: u64,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
        new_price: u64,
    }

    /// Event emitted when tokens are sold
    public struct TokenSoldEvent has copy, drop {
        id: address,
        seller: address,
        amount: u64,
        mys_amount: u64,
        fee_amount: u64,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
        new_price: u64,
    }

    /// Event emitted when an auction is created
    public struct AuctionCreatedEvent has copy, drop {
        auction_id: address,
        associated_id: address,
        token_type: u8,
        owner: address,
        start_time: u64,
        duration: u64,
    }

    /// Event emitted when a user contributes to an auction
    public struct AuctionContributionEvent has copy, drop {
        auction_id: address,
        contributor: address,
        amount: u64,
        total_contribution: u64,
    }

    /// Event emitted when an auction is finalized
    public struct AuctionFinalizedEvent has copy, drop {
        auction_id: address,
        associated_id: address,
        total_contribution: u64,
        total_tokens: u64,
        token_price: u64,
        pool_id: address,
    }

    /// Event emitted when exchange config is updated
    public struct ConfigUpdatedEvent has copy, drop {
        total_fee_bps: u64,
        creator_fee_bps: u64,
        platform_fee_bps: u64,
        treasury_fee_bps: u64,
        base_price: u64,
        quadratic_coefficient: u64,
    }

    /// Event emitted when tokens are purchased by someone who already has a social token
    public struct TokensAddedEvent has copy, drop {
        owner: address, 
        pool_id: address,
        amount: u64,
    }

    /// Event emitted when a limit order is placed
    public struct LimitOrderPlacedEvent has copy, drop {
        pool_id: address,
        order_id: u64,
        client_order_id: u64,
        is_bid: bool,
        owner: address,
        quantity: u64,
        price: u64,
        expire_timestamp: u64,
    }
    
    /// Event emitted when a limit order is canceled
    public struct LimitOrderCanceledEvent has copy, drop {
        pool_id: address,
        order_id: u64,
        client_order_id: u64,
        is_bid: bool,
        owner: address,
        quantity: u64,
        price: u64,
    }
    
    /// Event emitted when a limit order is filled
    public struct LimitOrderFilledEvent has copy, drop {
        pool_id: address,
        order_id: u64,
        taker_address: address,
        maker_address: address,
        is_bid: bool,
        quantity_filled: u64,
        quantity_remaining: u64,
        price: u64,
        fee_amount: u64,
    }

    // === Initialization ===
    
    /// Initialize the token exchange system
    fun init(ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        
        // Create and transfer admin capability to the transaction sender
        transfer::public_transfer(
            AdminCap {
                id: object::new(ctx),
            },
            sender
        );
        
        // Create and share exchange config
        transfer::share_object(
            ExchangeConfig {
                id: object::new(ctx),
                total_fee_bps: DEFAULT_TOTAL_FEE_BPS,
                creator_fee_bps: DEFAULT_CREATOR_FEE_BPS,
                platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
                treasury_fee_bps: DEFAULT_TREASURY_FEE_BPS,
                base_price: DEFAULT_BASE_PRICE,
                quadratic_coefficient: DEFAULT_QUADRATIC_COEFFICIENT,
                platform_treasury: sender, // Initially set to sender, should be updated
                ecosystem_treasury: sender, // Initially set to sender, should be updated
                max_hold_percent_bps: MAX_HOLD_PERCENT_BPS,
            }
        );
        
        // Create and share token registry
        transfer::share_object(
            TokenRegistry {
                id: object::new(ctx),
                tokens: table::new(ctx),
                auctions: table::new(ctx),
            }
        );
    }

    // === Admin Functions ===

    /// Update exchange configuration
    public entry fun update_config(
        _admin_cap: &AdminCap,
        config: &mut ExchangeConfig,
        total_fee_bps: u64, 
        creator_fee_bps: u64,
        platform_fee_bps: u64,
        treasury_fee_bps: u64,
        base_price: u64,
        quadratic_coefficient: u64,
        platform_treasury: address,
        ecosystem_treasury: address,
        max_hold_percent_bps: u64,
        _ctx: &mut TxContext
    ) {
        // Verify sum of fee percentages equals total
        assert!(creator_fee_bps + platform_fee_bps + treasury_fee_bps == total_fee_bps, EInvalidFeeConfig);
        
        // Verify curve parameters are valid
        assert!(base_price > 0 && quadratic_coefficient > 0, EInvalidCurveParams);
        
        // Update config
        config.total_fee_bps = total_fee_bps;
        config.creator_fee_bps = creator_fee_bps;
        config.platform_fee_bps = platform_fee_bps;
        config.treasury_fee_bps = treasury_fee_bps;
        config.base_price = base_price;
        config.quadratic_coefficient = quadratic_coefficient;
        config.platform_treasury = platform_treasury;
        config.ecosystem_treasury = ecosystem_treasury;
        config.max_hold_percent_bps = max_hold_percent_bps;
        
        // Emit config updated event
        event::emit(ConfigUpdatedEvent {
            total_fee_bps,
            creator_fee_bps,
            platform_fee_bps,
            treasury_fee_bps,
            base_price,
            quadratic_coefficient,
        });
    }

    // === Viral Threshold Checks ===

    /// Check if a post has reached the viral threshold
    public fun check_post_viral_threshold(
        post: &Post
    ): (bool, u64) {
        // Calculate viral score based on post metrics
        let likes = post::get_reaction_count(post) * POST_LIKES_WEIGHT;
        let comments = post::get_comment_count(post) * POST_COMMENTS_WEIGHT;
        let tips = post::get_tips_received(post) * POST_TIPS_WEIGHT;
        
        let viral_score = likes + comments + tips;
        
        // Check if the score exceeds the threshold
        (viral_score >= POST_VIRAL_THRESHOLD, viral_score)
    }
    
    /// Check if a profile has reached the viral threshold
    public fun check_profile_viral_threshold(
        profile: &Profile,
        _registry: &UsernameRegistry
    ): (bool, u64) {
        // Use accessor functions instead of direct field access
        let follows = profile::get_followers_count(profile) * PROFILE_FOLLOWS_WEIGHT;
        let posts = profile::get_post_count(profile) * PROFILE_POSTS_WEIGHT;
        let tips = profile::get_tips_received(profile) * PROFILE_TIPS_WEIGHT;
        
        let viral_score = follows + posts + tips;
        
        // Check if the score exceeds the threshold
        (viral_score >= PROFILE_VIRAL_THRESHOLD, viral_score)
    }
    
    // === Auction Functions ===
    
    /// Start a pre-launch auction for a post
    public entry fun start_post_auction(
        registry: &mut TokenRegistry,
        post: &Post,
        _symbol: vector<u8>,
        _name: vector<u8>,
        duration_hours: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let post_id = post::get_id_address(post);
        let owner = post::get_owner(post);
        
        // Verify caller is the post owner
        assert!(tx_context::sender(ctx) == owner, ENotAuthorized);
        
        // Check if an auction already exists for this post
        assert!(!table::contains(&registry.auctions, post_id), EAuctionInProgress);
        
        // Check if the post has reached the viral threshold
        let (is_viral, _viral_score) = check_post_viral_threshold(post);
        assert!(is_viral, EViralThresholdNotMet);
        
        // Validate auction duration
        let duration_seconds = duration_hours * 60 * 60;
        assert!(
            duration_seconds >= MIN_POST_AUCTION_DURATION && 
            duration_seconds <= MAX_POST_AUCTION_DURATION,
            EInvalidAuctionDuration
        );
        
        // Create auction info
        let start_time = clock::timestamp_ms(clock) / 1000; // Convert to seconds
        let auction_info = AuctionInfo {
            associated_id: post_id,
            token_type: TOKEN_TYPE_POST,
            owner,
            status: AUCTION_STATUS_ACTIVE,
            start_time,
            duration: duration_seconds,
            total_contribution: 0,
            total_tokens: 0,
            contributors: vector::empty(),
        };
        
        // Create auction pool
        let auction_pool = AuctionPool {
            id: object::new(ctx),
            info: auction_info,
            mys_balance: balance::zero(),
            contributions: table::new(ctx),
        };
        
        // Add to registry
        table::add(&mut registry.auctions, post_id, auction_info);
        
        // Emit event
        event::emit(AuctionCreatedEvent {
            auction_id: object::uid_to_address(&auction_pool.id),
            associated_id: post_id,
            token_type: TOKEN_TYPE_POST,
            owner,
            start_time,
            duration: duration_seconds,
        });
        
        // Share the auction pool
        transfer::share_object(auction_pool);
    }
    
    /// Start a pre-launch auction for a profile
    public entry fun start_profile_auction(
        registry: &mut TokenRegistry,
        profile: &Profile,
        username_registry: &UsernameRegistry,
        _symbol: vector<u8>,
        _name: vector<u8>,
        duration_days: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let profile_id = profile::get_id_address(profile);
        let owner = profile::get_owner(profile);
        
        // Verify caller is the profile owner
        assert!(tx_context::sender(ctx) == owner, ENotAuthorized);
        
        // Check if an auction already exists for this profile
        assert!(!table::contains(&registry.auctions, profile_id), EAuctionInProgress);
        
        // Check if the profile has reached the viral threshold
        let (is_viral, _viral_score) = check_profile_viral_threshold(profile, username_registry);
        assert!(is_viral, EViralThresholdNotMet);
        
        // Validate auction duration
        let duration_seconds = duration_days * 24 * 60 * 60;
        assert!(
            duration_seconds >= MIN_PROFILE_AUCTION_DURATION && 
            duration_seconds <= MAX_PROFILE_AUCTION_DURATION,
            EInvalidAuctionDuration
        );
        
        // Create auction info
        let start_time = clock::timestamp_ms(clock) / 1000; // Convert to seconds
        let auction_info = AuctionInfo {
            associated_id: profile_id,
            token_type: TOKEN_TYPE_PROFILE,
            owner,
            status: AUCTION_STATUS_ACTIVE,
            start_time,
            duration: duration_seconds,
            total_contribution: 0,
            total_tokens: 0,
            contributors: vector::empty(),
        };
        
        // Create auction pool
        let auction_pool = AuctionPool {
            id: object::new(ctx),
            info: auction_info,
            mys_balance: balance::zero(),
            contributions: table::new(ctx),
        };
        
        // Add to registry
        table::add(&mut registry.auctions, profile_id, auction_info);
        
        // Emit event
        event::emit(AuctionCreatedEvent {
            auction_id: object::uid_to_address(&auction_pool.id),
            associated_id: profile_id,
            token_type: TOKEN_TYPE_PROFILE,
            owner,
            start_time,
            duration: duration_seconds,
        });
        
        // Share the auction pool
        transfer::share_object(auction_pool);
    }
    
    /// Contribute MYS to an auction
    public entry fun contribute_to_auction(
        registry: &mut TokenRegistry,
        auction_pool: &mut AuctionPool,
        mut payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        let contributor = tx_context::sender(ctx);
        
        // Verify auction is active
        assert!(auction_pool.info.status == AUCTION_STATUS_ACTIVE, EAuctionNotActive);
        
        // Verify auction info matches registry
        let stored_info = table::borrow(&registry.auctions, auction_pool.info.associated_id);
        assert!(
            stored_info.owner == auction_pool.info.owner && 
            stored_info.start_time == auction_pool.info.start_time,
            EInvalidID
        );
        
        // Ensure contributor has enough funds
        assert!(coin::value(&payment) >= amount, EInsufficientFunds);
        
        // Extract payment
        let contribution = coin::split(&mut payment, amount, ctx);
        
        // Update contribution record
        if (table::contains(&auction_pool.contributions, contributor)) {
            let current_contribution = table::borrow_mut(&mut auction_pool.contributions, contributor);
            *current_contribution = *current_contribution + amount;
        } else {
            table::add(&mut auction_pool.contributions, contributor, amount);
            // Add to contributors list for tracking
            vector::push_back(&mut auction_pool.info.contributors, contributor);
        };
        
        // Add to pool balance
        balance::join(&mut auction_pool.mys_balance, coin::into_balance(contribution));
        
        // Update total contribution
        auction_pool.info.total_contribution = auction_pool.info.total_contribution + amount;
        
        // Update registry
        let mut updated_info = *stored_info;
        updated_info.total_contribution = auction_pool.info.total_contribution;
        
        // If this is a new contributor, add them to the registry's contributor list too
        if (!table::contains(&auction_pool.contributions, contributor)) {
            vector::push_back(&mut updated_info.contributors, contributor);
        };
        
        *table::borrow_mut(&mut registry.auctions, auction_pool.info.associated_id) = updated_info;
        
        // Return any excess payment
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, contributor);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Emit contribution event
        event::emit(AuctionContributionEvent {
            auction_id: object::uid_to_address(&auction_pool.id),
            contributor,
            amount,
            total_contribution: auction_pool.info.total_contribution,
        });
    }
    
    /// Check if an auction has ended
    public fun is_auction_ended(
        auction_info: &AuctionInfo, 
        clock: &Clock
    ): bool {
        let current_time = clock::timestamp_ms(clock) / 1000; // Convert to seconds
        let end_time = auction_info.start_time + auction_info.duration;
        current_time >= end_time
    }
    
    /// Finalize an auction and create the token pool
    /// This function checks if the auction has ended and finalizes it by creating a token pool
    public entry fun finalize_auction(
        registry: &mut TokenRegistry,
        config: &ExchangeConfig,
        auction_pool: &mut AuctionPool,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check if auction has ended but status not updated
        if (auction_pool.info.status == AUCTION_STATUS_ACTIVE && is_auction_ended(&auction_pool.info, clock)) {
            // Update status to ended
            auction_pool.info.status = AUCTION_STATUS_ENDED;
            
            // Update registry
            let mut updated_info = *table::borrow(&registry.auctions, auction_pool.info.associated_id);
            updated_info.status = AUCTION_STATUS_ENDED;
            *table::borrow_mut(&mut registry.auctions, auction_pool.info.associated_id) = updated_info;
        };
        
        // Verify auction has ended
        assert!(auction_pool.info.status == AUCTION_STATUS_ENDED, EAuctionNotEnded);
        assert!(is_auction_ended(&auction_pool.info, clock), EAuctionNotEnded);
        
        // Verify auction has not been finalized
        assert!(
            !table::contains(&registry.tokens, auction_pool.info.associated_id),
            EAuctionAlreadyFinalized
        );
        
        // Verify there are contributions
        assert!(auction_pool.info.total_contribution > 0, ENoContribution);
        
        // Calculate initial token supply with dynamic scaling based on contribution size
        // This creates a non-linear relationship where larger pools get proportionally 
        // more tokens, helping to prevent front-running and maintain AMM efficiency
        
        // Use square root scaling to balance between very large and small pools
        // We use total_contribution^0.75 as our scaling factor
        // (Using integer math for the calculation)
        let contribution = auction_pool.info.total_contribution;
        let sqrt_contribution = math::sqrt(contribution);
        let cbrt_contribution = math::sqrt(sqrt_contribution); // approximation of cube root
        let mut scale_factor = sqrt_contribution * cbrt_contribution; // contribution^0.75
        
        // Divide the scale factor to make each token worth more than 1 MYSO
        // This ensures tokens are premium assets compared to the base currency
        scale_factor = scale_factor / 1000;
        
        // Apply different base multipliers for profile vs post tokens
        // Profile tokens have lower supply (more valuable per token)
        // Post tokens have higher supply (more collectible, less valuable per token)
        let mut initial_token_supply = if (auction_pool.info.token_type == TOKEN_TYPE_PROFILE) {
            // Profile tokens - lower supply (1x base multiplier)
            // These represent long-term investment in a person/brand
            scale_factor
        } else {
            // Post tokens - higher supply (10x base multiplier)
            // These are more collectible with many tokens per viral post
            scale_factor * 10
        };
        
        // Ensure we have at least 1 token
        if (initial_token_supply == 0) {
            initial_token_supply = 1;
        };
        
        let token_price = auction_pool.info.total_contribution / initial_token_supply;
        
        // Create token info
        let token_info = TokenInfo {
            id: @0x0, // Temporary, will be updated
            token_type: auction_pool.info.token_type,
            owner: auction_pool.info.owner,
            associated_id: auction_pool.info.associated_id,
            symbol: if (auction_pool.info.token_type == TOKEN_TYPE_PROFILE) {
                string::utf8(b"PUSER")
            } else {
                string::utf8(b"PPOST")
            },
            name: if (auction_pool.info.token_type == TOKEN_TYPE_PROFILE) {
                string::utf8(b"Profile Token")
            } else {
                string::utf8(b"Post Token")
            },
            circulating_supply: initial_token_supply,
            base_price: config.base_price,
            quadratic_coefficient: config.quadratic_coefficient,
            created_at: tx_context::epoch(ctx),
        };
        
        // Create token pool
        let pool_id = object::new(ctx);
        let pool_address = object::uid_to_address(&pool_id);
        
        // Create pool with updated token info
        let mut updated_token_info = token_info;
        updated_token_info.id = pool_address;
        
        let mut token_pool = TokenPool {
            id: pool_id,
            info: updated_token_info,
            mys_balance: balance::zero(),
            holders: table::new(ctx),
            order_book: OrderBook {
                bids: critbit_new(ctx),
                asks: critbit_new(ctx),
                next_order_id: 1,
                user_orders: table::new(ctx),
            },
        };
        
        // Distribute tokens to contributors
        // Production implementation that efficiently distributes tokens to all contributors
        let contributors = &auction_pool.info.contributors;
        let num_contributors = vector::length(contributors);
        
        // Iterate through all contributors who participated in the auction
        let mut i = 0;
        while (i < num_contributors) {
            let contributor = *vector::borrow(contributors, i);
            let contribution_amount = *table::borrow(&auction_pool.contributions, contributor);
            
            // Calculate token amount based on contributor's proportion of total contribution
            let token_amount = (contribution_amount * initial_token_supply) / auction_pool.info.total_contribution;
            
            // Only process non-zero token amounts
            if (token_amount > 0) {
                // Update holder's balance in the pool
                table::add(&mut token_pool.holders, contributor, token_amount);
                
                // Create social token
                let social_token = SocialToken {
                    id: object::new(ctx),
                    pool_id: pool_address,
                    token_type: auction_pool.info.token_type,
                    amount: token_amount,
                };
                
                // Transfer social token to contributor
                transfer::public_transfer(social_token, contributor);
            };
            
            i = i + 1;
        };
        
        // Add contribution to pool balance
        balance::join(&mut token_pool.mys_balance, balance::withdraw_all(&mut auction_pool.mys_balance));
        
        // Update the registry
        table::add(&mut registry.tokens, auction_pool.info.associated_id, updated_token_info);
        
        // Update auction status
        auction_pool.info.status = AUCTION_STATUS_FINALIZED;
        auction_pool.info.total_tokens = initial_token_supply;
        
        // Update registry auction info
        let mut updated_auction_info = *table::borrow(&registry.auctions, auction_pool.info.associated_id);
        updated_auction_info.status = AUCTION_STATUS_FINALIZED;
        updated_auction_info.total_tokens = initial_token_supply;
        *table::borrow_mut(&mut registry.auctions, auction_pool.info.associated_id) = updated_auction_info;
        
        // Emit finalized event
        event::emit(AuctionFinalizedEvent {
            auction_id: object::uid_to_address(&auction_pool.id),
            associated_id: auction_pool.info.associated_id,
            total_contribution: auction_pool.info.total_contribution,
            total_tokens: initial_token_supply,
            token_price,
            pool_id: pool_address,
        });
        
        // Emit token created event
        event::emit(TokenPoolCreatedEvent {
            id: pool_address,
            token_type: updated_token_info.token_type,
            owner: updated_token_info.owner,
            associated_id: updated_token_info.associated_id,
            symbol: updated_token_info.symbol,
            name: updated_token_info.name,
            base_price: updated_token_info.base_price,
            quadratic_coefficient: updated_token_info.quadratic_coefficient,
        });
        
        // Share the token pool
        transfer::share_object(token_pool);
    }

    // === Trading Functions ===

    /// Buy tokens from the pool - first purchase
    /// This function handles buying tokens for first-time buyers of a specific token
    public entry fun buy_tokens(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &ExchangeConfig,
        block_list_registry: &BlockListRegistry,
        mut payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        let buyer = tx_context::sender(ctx);
        
        // Prevent self-trading for token owners
        assert!(buyer != pool.info.owner, ESelfTrading);
        
        // Check if token owner is blocked by the buyer
        assert!(!social_contracts::block_list::is_blocked(block_list_registry, buyer, pool.info.owner), EBlockedUser);
        
        // Calculate the price for the tokens based on quadratic curve
        let (price, _) = calculate_buy_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );
        
        // Ensure buyer has enough funds
        assert!(coin::value(&payment) >= price, EInsufficientFunds);
        
        // Calculate fees
        let fee_amount = (price * config.total_fee_bps) / 10000;
        let creator_fee = (fee_amount * config.creator_fee_bps) / config.total_fee_bps;
        let platform_fee = (fee_amount * config.platform_fee_bps) / config.total_fee_bps;
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate the net amount to the liquidity pool
        let net_amount = price - fee_amount;
        
        // Extract payment and distribute fees directly
        if (fee_amount > 0) {
            // Send creator fee
            if (creator_fee > 0) {
                let creator_fee_coin = coin::split(&mut payment, creator_fee, ctx);
                transfer::public_transfer(creator_fee_coin, pool.info.owner);
            };
            
            // Send platform fee
            if (platform_fee > 0) {
                let platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                transfer::public_transfer(platform_fee_coin, config.platform_treasury);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, config.ecosystem_treasury);
            };
        };
        
        // Add remaining payment to pool
        let pool_payment = coin::split(&mut payment, net_amount, ctx);
        balance::join(&mut pool.mys_balance, coin::into_balance(pool_payment));
        
        // Refund any excess payment
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, buyer);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Update holder's balance
        let max_hold = (pool.info.circulating_supply + amount) * config.max_hold_percent_bps / 10000;
        let current_hold = if (table::contains(&pool.holders, buyer)) {
            *table::borrow(&pool.holders, buyer)
        } else {
            0
        };
        
        // Check max holding limit
        assert!(current_hold + amount <= max_hold, EExceededMaxHold);
        
        // Check that this is the first purchase
        assert!(current_hold == 0, ETokenAlreadyExists);
        
        // Update holder's balance
        table::add(&mut pool.holders, buyer, amount);
        
        // Update circulating supply
        pool.info.circulating_supply = pool.info.circulating_supply + amount;
        
        // Mint new social token for the user
        let social_token = SocialToken {
            id: object::new(ctx),
            pool_id: object::uid_to_address(&pool.id),
            token_type: pool.info.token_type,
            amount,
        };
        transfer::public_transfer(social_token, buyer);
        
        // Calculate the new price after purchase
        let new_price = calculate_token_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        );
        
        // Emit buy event
        event::emit(TokenBoughtEvent {
            id: object::uid_to_address(&pool.id),
            buyer,
            amount,
            mys_amount: price,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
            new_price,
        });
    }

    /// Buy more tokens when you already have a social token
    /// This function allows users to add to their existing token holdings using MYS Coin
    public entry fun buy_more_tokens(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &ExchangeConfig,
        block_list_registry: &BlockListRegistry,
        mut payment: Coin<MYS>,
        amount: u64,
        social_token: &mut SocialToken,
        ctx: &mut TxContext
    ) {
        let buyer = tx_context::sender(ctx);
        
        // Prevent self-trading for token owners
        assert!(buyer != pool.info.owner, ESelfTrading);
        
        // Check if token owner is blocked by the buyer
        assert!(!social_contracts::block_list::is_blocked(block_list_registry, buyer, pool.info.owner), EBlockedUser);
        
        // Verify social token matches the pool
        assert!(social_token.pool_id == object::uid_to_address(&pool.id), EInvalidID);
        
        // Calculate the price for the tokens based on quadratic curve
        let (price, _) = calculate_buy_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );
        
        // Ensure buyer has enough funds
        assert!(coin::value(&payment) >= price, EInsufficientFunds);
        
        // Calculate fees
        let fee_amount = (price * config.total_fee_bps) / 10000;
        let creator_fee = (fee_amount * config.creator_fee_bps) / config.total_fee_bps;
        let platform_fee = (fee_amount * config.platform_fee_bps) / config.total_fee_bps;
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate the net amount to the liquidity pool
        let net_amount = price - fee_amount;
        
        // Extract payment and distribute fees directly
        if (fee_amount > 0) {
            // Send creator fee
            if (creator_fee > 0) {
                let creator_fee_coin = coin::split(&mut payment, creator_fee, ctx);
                transfer::public_transfer(creator_fee_coin, pool.info.owner);
            };
            
            // Send platform fee
            if (platform_fee > 0) {
                let platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                transfer::public_transfer(platform_fee_coin, config.platform_treasury);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, config.ecosystem_treasury);
            };
        };
        
        // Add remaining payment to pool
        let pool_payment = coin::split(&mut payment, net_amount, ctx);
        balance::join(&mut pool.mys_balance, coin::into_balance(pool_payment));
        
        // Refund any excess payment
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, buyer);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Update holder's balance
        let max_hold = (pool.info.circulating_supply + amount) * config.max_hold_percent_bps / 10000;
        let current_hold = if (table::contains(&pool.holders, buyer)) {
            *table::borrow(&pool.holders, buyer)
        } else {
            0
        };
        
        // Check max holding limit
        assert!(current_hold + amount <= max_hold, EExceededMaxHold);
        
        // Update holder's balance
        if (table::contains(&pool.holders, buyer)) {
            let holder_balance = table::borrow_mut(&mut pool.holders, buyer);
            *holder_balance = *holder_balance + amount;
        } else {
            table::add(&mut pool.holders, buyer, amount);
        };
        
        // Update circulating supply
        pool.info.circulating_supply = pool.info.circulating_supply + amount;
        
        // Update the user's social token
        social_token.amount = social_token.amount + amount;
        
        // Calculate the new price after purchase
        let new_price = calculate_token_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        );
        
        // Emit buy event
        event::emit(TokenBoughtEvent {
            id: object::uid_to_address(&pool.id),
            buyer,
            amount,
            mys_amount: price,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
            new_price,
        });
    }

    /// Sell tokens back to the pool
    public entry fun sell_tokens(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &ExchangeConfig,
        social_token: &mut SocialToken,
        amount: u64,
        ctx: &mut TxContext
    ) {
        let seller = tx_context::sender(ctx);
        let pool_id = object::uid_to_address(&pool.id);
        
        // Verify social token matches the pool
        assert!(social_token.pool_id == pool_id, EInvalidID);
        assert!(social_token.amount >= amount, EInsufficientLiquidity);
        
        // Calculate the sell price based on quadratic curve
        let (refund_amount, _) = calculate_sell_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );
        
        // Calculate fees
        let fee_amount = (refund_amount * config.total_fee_bps) / 10000;
        let creator_fee = (fee_amount * config.creator_fee_bps) / config.total_fee_bps;
        let platform_fee = (fee_amount * config.platform_fee_bps) / config.total_fee_bps;
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate net refund
        let net_refund = refund_amount - fee_amount;
        
        // Ensure pool has enough liquidity
        assert!(balance::value(&pool.mys_balance) >= net_refund, EInsufficientLiquidity);
        
        // Update holder balance
        let holder_balance = table::borrow_mut(&mut pool.holders, seller);
        *holder_balance = *holder_balance - amount;
        
        // Update user's social token
        social_token.amount = social_token.amount - amount;
        
        // Update circulating supply
        pool.info.circulating_supply = pool.info.circulating_supply - amount;
        
        // Extract net refund from pool
        let refund_balance = balance::split(&mut pool.mys_balance, net_refund);
        
        // Process and distribute fees
        if (fee_amount > 0) {
            // Send fee to creator
            if (creator_fee > 0) {
                let creator_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, creator_fee), ctx);
                transfer::public_transfer(creator_fee_coin, pool.info.owner);
            };
            
            // Send fee to platform
            if (platform_fee > 0) {
                let platform_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, platform_fee), ctx);
                transfer::public_transfer(platform_fee_coin, config.platform_treasury);
            };
            
            // Send fee to treasury
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, treasury_fee), ctx);
                transfer::public_transfer(treasury_fee_coin, config.ecosystem_treasury);
            };
        };
        
        // Transfer refund to seller
        let refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, seller);
        
        // Calculate the new price after sale
        let new_price = calculate_token_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        );
        
        // Emit sell event
        event::emit(TokenSoldEvent {
            id: pool_id,
            seller,
            amount,
            mys_amount: refund_amount,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
            new_price,
        });
    }

    // === Utility Functions ===

    /// Calculate token price at current supply based on quadratic curve
    /// Price = base_price + (quadratic_coefficient * supply^2)
    public fun calculate_token_price(
        base_price: u64,
        quadratic_coefficient: u64,
        supply: u64
    ): u64 {
        let squared_supply = supply * supply;
        base_price + (quadratic_coefficient * squared_supply / 10000)
    }

    /// Calculate price to buy a specific amount of tokens
    /// Returns (total price, average price per token)
    public fun calculate_buy_price(
        base_price: u64,
        quadratic_coefficient: u64,
        current_supply: u64,
        amount: u64
    ): (u64, u64) {
        let mut total_price = 0;
        let mut current = current_supply;
        let mut i = 0;
        
        // Integrate the price curve over the purchase amount
        while (i < amount) {
            let token_price = calculate_token_price(base_price, quadratic_coefficient, current);
            total_price = total_price + token_price;
            current = current + 1;
            i = i + 1;
        };
        
        let avg_price = if (amount > 0) {
            total_price / amount
        } else {
            0
        };
        
        (total_price, avg_price)
    }

    /// Calculate refund amount when selling tokens
    /// Returns (total refund, average price per token)
    public fun calculate_sell_price(
        base_price: u64,
        quadratic_coefficient: u64,
        current_supply: u64,
        amount: u64
    ): (u64, u64) {
        let mut total_refund = 0;
        let mut current = current_supply;
        let mut i = 0;
        
        // Integrate the price curve over the sell amount
        while (i < amount) {
            current = current - 1;
            let token_price = calculate_token_price(base_price, quadratic_coefficient, current);
            total_refund = total_refund + token_price;
            i = i + 1;
        };
        
        let avg_price = if (amount > 0) {
            total_refund / amount
        } else {
            0
        };
        
        (total_refund, avg_price)
    }

    /// Get token info from registry
    public fun get_token_info(registry: &TokenRegistry, id: address): TokenInfo {
        assert!(table::contains(&registry.tokens, id), ETokenNotFound);
        *table::borrow(&registry.tokens, id)
    }

    /// Get token owner's address
    public fun get_token_owner(registry: &TokenRegistry, id: address): address {
        let info = get_token_info(registry, id);
        info.owner
    }

    /// Get current token price for a specific pool
    public fun get_pool_price(pool: &TokenPool): u64 {
        calculate_token_price(
            pool.info.base_price, 
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply
        )
    }

    /// Get user's token balance
    public fun get_user_balance(pool: &TokenPool, user: address): u64 {
        if (table::contains(&pool.holders, user)) {
            *table::borrow(&pool.holders, user)
        } else {
            0
        }
    }

    // Test-only functions
    #[test_only]
    /// Initialize the token exchange for testing
    public fun init_for_testing(ctx: &mut TxContext) {
        init(ctx)
    }
    
    // === Critbit Tree Implementation ===
    
    /// Create a new Critbit tree
    fun critbit_new<V: store>(ctx: &mut TxContext): CritbitTree<V> {
        CritbitTree<V> {
            root: PARTITION_INDEX,
            internal_nodes: table::new(ctx),
            leaves: table::new(ctx),
            min_leaf: PARTITION_INDEX,
            max_leaf: PARTITION_INDEX,
            next_internal_node_index: 0,
            next_leaf_index: 0
        }
    }
    
    /// Check if the Critbit tree is empty
    fun critbit_is_empty<V: store>(tree: &CritbitTree<V>): bool {
        table::is_empty(&tree.leaves)
    }
    
    /// Get the size of the Critbit tree
    fun critbit_size<V: store>(tree: &CritbitTree<V>): u64 {
        table::length(&tree.leaves)
    }
    
    /// Return (key, index) of the leaf with minimum value
    fun critbit_min_leaf<V: store>(tree: &CritbitTree<V>): (u64, u64) {
        assert!(!critbit_is_empty(tree), ELeafNotExist);
        let min_leaf = table::borrow(&tree.leaves, tree.min_leaf);
        (min_leaf.key, tree.min_leaf)
    }
    
    /// Return (key, index) of the leaf with maximum value
    fun critbit_max_leaf<V: store>(tree: &CritbitTree<V>): (u64, u64) {
        assert!(!critbit_is_empty(tree), ELeafNotExist);
        let max_leaf = table::borrow(&tree.leaves, tree.max_leaf);
        (max_leaf.key, tree.max_leaf)
    }
    
    /// Find a leaf in the tree based on key, returns (exists, index)
    fun critbit_find_leaf<V: store>(tree: &CritbitTree<V>, key: u64): (bool, u64) {
        if (critbit_is_empty(tree)) {
            return (false, PARTITION_INDEX)
        };
        let closest_leaf_index = get_closest_leaf_index_by_key(tree, key);
        let closeset_leaf = table::borrow(&tree.leaves, closest_leaf_index);
        if (closeset_leaf.key != key){
            return (false, PARTITION_INDEX)
        } else{
            return (true, closest_leaf_index)
        }
    }
    
    /// Return the previous leaf (key, index) of the input leaf
    fun critbit_previous_leaf<V: store>(tree: &CritbitTree<V>, key: u64): (u64, u64) {
        let (_, mut index) = critbit_find_leaf(tree, key);
        assert!(index != PARTITION_INDEX, ELeafNotExist);
        let mut ptr = MAX_U64 - index;
        let mut parent = table::borrow(&tree.leaves, index).parent;
        while (parent != PARTITION_INDEX && is_left_child(tree, parent, ptr)){
            ptr = parent;
            parent = table::borrow(&tree.internal_nodes, ptr).parent;
        };
        if(parent == PARTITION_INDEX) {
            return (0, PARTITION_INDEX)
        };
        index = MAX_U64 - right_most_leaf(tree, table::borrow(&tree.internal_nodes, parent).left_child);
        let key = table::borrow(&tree.leaves, index).key;
        return (key, index)
    }
    
    /// Return the next leaf (key, index) of the input leaf
    fun critbit_next_leaf<V: store>(tree: &CritbitTree<V>, key: u64): (u64, u64) {
        let (_, mut index) = critbit_find_leaf(tree, key);
        assert!(index != PARTITION_INDEX, ELeafNotExist);
        let mut ptr = MAX_U64 - index;
        let mut parent = table::borrow(&tree.leaves, index).parent;
        while (parent != PARTITION_INDEX && !is_left_child(tree, parent, ptr)){
            ptr = parent;
            parent = table::borrow(&tree.internal_nodes, ptr).parent;
        };
        if(parent == PARTITION_INDEX) {
            return (0, PARTITION_INDEX)
        };
        index = MAX_U64 - left_most_leaf(tree, table::borrow(&tree.internal_nodes, parent).right_child);
        let key = table::borrow(&tree.leaves, index).key;
        return (key, index)
    }
    
    /// Helper for finding the left-most leaf in a subtree
    fun left_most_leaf<V: store>(tree: &CritbitTree<V>, root: u64): u64 {
        let mut ptr = root;
        while (ptr < PARTITION_INDEX){
            ptr = table::borrow(& tree.internal_nodes, ptr).left_child;
        };
        ptr
    }
    
    /// Helper for finding the right-most leaf in a subtree
    fun right_most_leaf<V: store>(tree: &CritbitTree<V>, root: u64): u64 {
        let mut ptr = root;
        while (ptr < PARTITION_INDEX){
            ptr = table::borrow(& tree.internal_nodes, ptr).right_child;
        };
        ptr
    }
    
    /// Count leading zeros in a u128 value
    fun count_leading_zeros(mut x: u128): u8 {
        if (x == 0) {
            128
        } else {
            let mut n: u8 = 0;
            if (x & 0xFFFFFFFFFFFFFFFF0000000000000000 == 0) {
                // x's higher 64 is all zero, shift the lower part over
                x = x << 64;
                n = n + 64;
            };
            if (x & 0xFFFFFFFF000000000000000000000000 == 0) {
                // x's higher 32 is all zero, shift the lower part over
                x = x << 32;
                n = n + 32;
            };
            if (x & 0xFFFF0000000000000000000000000000 == 0) {
                // x's higher 16 is all zero, shift the lower part over
                x = x << 16;
                n = n + 16;
            };
            if (x & 0xFF000000000000000000000000000000 == 0) {
                // x's higher 8 is all zero, shift the lower part over
                x = x << 8;
                n = n + 8;
            };
            if (x & 0xF0000000000000000000000000000000 == 0) {
                // x's higher 4 is all zero, shift the lower part over
                x = x << 4;
                n = n + 4;
            };
            if (x & 0xC0000000000000000000000000000000 == 0) {
                // x's higher 2 is all zero, shift the lower part over
                x = x << 2;
                n = n + 2;
            };
            if (x & 0x80000000000000000000000000000000 == 0) {
                n = n + 1;
            };

            n
        }
    }
    
    /// Insert a new leaf into the critbit tree
    fun critbit_insert_leaf<V: store>(tree: &mut CritbitTree<V>, key: u64, value: V): u64 {
        let new_leaf = Leaf<V>{
            key,
            value,
            parent: PARTITION_INDEX,
        };
        let new_leaf_index = tree.next_leaf_index;
        tree.next_leaf_index = tree.next_leaf_index + 1;
        assert!(new_leaf_index < MAX_CAPACITY - 1, EExceedCapacity);
        table::add(&mut tree.leaves, new_leaf_index, new_leaf);

        let closest_leaf_index = get_closest_leaf_index_by_key(tree, key);

        // Handle the first insertion
        if (closest_leaf_index == PARTITION_INDEX) {
            assert!(new_leaf_index == 0, ETreeNotEmpty);
            tree.root = MAX_U64 - new_leaf_index;
            tree.min_leaf = new_leaf_index;
            tree.max_leaf = new_leaf_index;
            return 0
        };

        let closest_key = table::borrow(&tree.leaves, closest_leaf_index).key;
        assert!(closest_key != key, EKeyAlreadyExist);

        // Note that we reserve count_leading_zeros of form u128 for future use
        let critbit = 64 - (count_leading_zeros((closest_key ^ key) as u128) - 64);
        let new_mask = 1u64 << (critbit - 1);

        let new_internal_node= InternalNode {
            mask: new_mask,
            left_child: PARTITION_INDEX,
            right_child: PARTITION_INDEX,
            parent: PARTITION_INDEX,
        };
        let new_internal_node_index = tree.next_internal_node_index;
        tree.next_internal_node_index = tree.next_internal_node_index + 1;
        table::add(&mut tree.internal_nodes, new_internal_node_index, new_internal_node);

        let mut ptr = tree.root;
        let mut new_internal_node_parent_index = PARTITION_INDEX;
        // Search position of the new internal node
        while (ptr < PARTITION_INDEX) {
            let internal_node = table::borrow(&tree.internal_nodes, ptr);
            if (new_mask > internal_node.mask) {
                break
            };
            new_internal_node_parent_index = ptr;
            if (key & internal_node.mask == 0) {
                ptr = internal_node.left_child;
            } else {
                ptr = internal_node.right_child;
            };
        };

        // Update the child info of new internal node's parent
        if (new_internal_node_parent_index == PARTITION_INDEX){
            // if the new internal node is root
            tree.root = new_internal_node_index;
        } else{
            // In another case, we update the child field of the new internal node's parent
            // And the parent field of the new internal node
            let is_left_child = is_left_child(tree, new_internal_node_parent_index, ptr);
            update_child(tree, new_internal_node_parent_index, new_internal_node_index, is_left_child);
        };

        // Finally, update the child field of the new internal node
        let is_left_child = new_mask & key == 0;
        update_child(tree, new_internal_node_index, MAX_U64 - new_leaf_index, is_left_child);
        update_child(tree, new_internal_node_index, ptr, !is_left_child);

        if (table::borrow(&tree.leaves, tree.min_leaf).key > key) {
            tree.min_leaf = new_leaf_index;
        };
        if (table::borrow(&tree.leaves, tree.max_leaf).key < key) {
            tree.max_leaf = new_leaf_index;
        };
        new_leaf_index
    }
    
    /// Remove a leaf from the critbit tree by index
    fun critbit_remove_leaf_by_index<V: store>(tree: &mut CritbitTree<V>, index: u64): V {
        let key = table::borrow(& tree.leaves, index).key;
        if (tree.min_leaf == index) {
            let (_, index) = critbit_next_leaf(tree, key);
            tree.min_leaf = index;
        };
        if (tree.max_leaf == index) {
            let (_, index) = critbit_previous_leaf(tree, key);
            tree.max_leaf = index;
        };

        let mut is_left_child_;
        let Leaf<V> {key: _, value, parent: removed_leaf_parent_index} = table::remove(&mut tree.leaves, index);

        if (critbit_size(tree) == 0) {
            tree.root = PARTITION_INDEX;
            tree.min_leaf = PARTITION_INDEX;
            tree.max_leaf = PARTITION_INDEX;
            tree.next_internal_node_index = 0;
            tree.next_leaf_index = 0;
        } else {
            assert!(removed_leaf_parent_index != PARTITION_INDEX, EIndexOutOfRange);
            let removed_leaf_parent = table::borrow(&tree.internal_nodes, removed_leaf_parent_index);
            let removed_leaf_grand_parent_index = removed_leaf_parent.parent;

            // Note that sibling of the removed leaf can be a leaf or an internal node
            is_left_child_ = is_left_child(tree, removed_leaf_parent_index, MAX_U64 - index);
            let sibling_index = if (is_left_child_) { removed_leaf_parent.right_child }
            else { removed_leaf_parent.left_child };

            if (removed_leaf_grand_parent_index == PARTITION_INDEX) {
                // Parent of the removed leaf is the tree root
                // Update the parent of the sibling node and set sibling as the tree root
                if (sibling_index < PARTITION_INDEX) {
                    // sibling is an internal node
                    table::borrow_mut(&mut tree.internal_nodes, sibling_index).parent = PARTITION_INDEX;
                } else{
                    // sibling is a leaf
                    table::borrow_mut(&mut tree.leaves, MAX_U64 - sibling_index).parent = PARTITION_INDEX;
                };
                tree.root = sibling_index;
            } else {
                // grand parent of the removed leaf is a internal node
                // set sibling as the child of the grand parent of the removed leaf
                is_left_child_ = is_left_child(tree, removed_leaf_grand_parent_index, removed_leaf_parent_index);
                update_child(tree, removed_leaf_grand_parent_index, sibling_index, is_left_child_);
            };
            table::remove(&mut tree.internal_nodes, removed_leaf_parent_index);
        };
        value
    }
    
    /// Access a leaf in the critbit tree by index
    fun critbit_borrow_leaf_by_index<V: store>(tree: &CritbitTree<V>, index: u64): &V {
        let entry = table::borrow(&tree.leaves, index);
        &entry.value
    }
    
    /// Access a mutable leaf in the critbit tree by index
    fun critbit_borrow_mut_leaf_by_index<V: store>(tree: &mut CritbitTree<V>, index: u64): &mut V {
        let entry = table::borrow_mut(&mut tree.leaves, index);
        &mut entry.value
    }
    
    /// Access a leaf in the critbit tree by key
    fun critbit_borrow_leaf_by_key<V: store>(tree: &CritbitTree<V>, key: u64): &V {
        let (is_exist, index) = critbit_find_leaf(tree, key);
        assert!(is_exist, ELeafNotExist);
        critbit_borrow_leaf_by_index(tree, index)
    }
    
    /// Get the closest leaf index by key
    fun get_closest_leaf_index_by_key<V: store>(tree: &CritbitTree<V>, key: u64): u64 {
        let mut ptr = tree.root;
        // if tree is empty, return the partition index
        if(ptr == PARTITION_INDEX) return PARTITION_INDEX;
        while (ptr < PARTITION_INDEX){
            let node = table::borrow(&tree.internal_nodes, ptr);
            if (key & node.mask == 0){
                ptr = node.left_child;
            } else {
                ptr = node.right_child;
            }
        };
        return (MAX_U64 - ptr)
    }
    
    /// Check if a node is a left child
    fun is_left_child<V: store>(tree: &CritbitTree<V>, parent_index: u64, index: u64): bool {
        table::borrow(&tree.internal_nodes, parent_index).left_child == index
    }
    
    /// Update the child of a node
    fun update_child<V: store>(tree: &mut CritbitTree<V>, parent_index: u64, new_child: u64, is_left_child: bool) {
        assert!(parent_index != PARTITION_INDEX, ENullParent);
        if (is_left_child) {
            table::borrow_mut(&mut tree.internal_nodes, parent_index).left_child = new_child;
        } else{
            table::borrow_mut(&mut tree.internal_nodes, parent_index).right_child = new_child;
        };
        if (new_child > PARTITION_INDEX) {
            table::borrow_mut(&mut tree.leaves, MAX_U64 - new_child).parent = parent_index;
        } else {
            table::borrow_mut(&mut tree.internal_nodes, new_child).parent = parent_index;
        }
    }

    // === Limit Order Functions ===
    
    /// Place a limit order to buy or sell tokens
    public entry fun place_limit_order(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &ExchangeConfig,
        block_list_registry: &BlockListRegistry,
        client_order_id: u64,
        price: u64,
        quantity: u64,
        is_bid: bool,
        expire_timestamp: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let owner = tx_context::sender(ctx);
        
        // Prevent self-trading for token owners
        assert!(owner != pool.info.owner, ESelfTrading);
        
        // Check if token owner is blocked by the buyer/seller
        assert!(!social_contracts::block_list::is_blocked(block_list_registry, owner, pool.info.owner), EBlockedUser);
        
        // Validate inputs
        assert!(price > 0, EInvalidCurveParams);
        assert!(quantity > 0, EInsufficientLiquidity);
        assert!(expire_timestamp > clock::timestamp_ms(clock), EAuctionNotEnded);
        
        // Check if this is a bid or ask order
        if (is_bid) {
            // For buy orders, check if user has enough MYS funds
            let total_cost = price * quantity;
            
            // Check if user has deposited enough MYS to cover the order
            let (_, _, quote_avail, _) = account_balance(_registry, pool, owner);
            assert!(quote_avail >= total_cost, EInsufficientFunds);
            
            // Lock funds for the order
            lock_user_funds(pool, owner, total_cost);
        } else {
            // For sell orders, check if user has enough tokens
            let (base_avail, _, _, _) = account_balance(_registry, pool, owner);
            assert!(base_avail >= quantity, EInsufficientLiquidity);
            
            // Lock tokens for the order
            let holder_balance = table::borrow_mut(&mut pool.holders, owner);
            *holder_balance = *holder_balance - quantity;
        };
        
        // Create the order
        let order = Order {
            order_id: pool.order_book.next_order_id,
            client_order_id,
            price,
            original_quantity: quantity,
            quantity,
            is_bid,
            owner,
            expire_timestamp,
        };
        
        // Insert the order into the order book
        let order_id = pool.order_book.next_order_id;
        pool.order_book.next_order_id = pool.order_book.next_order_id + 1;
        
        // Find or create price level
        let (tick_exists, mut tick_index) = critbit_find_leaf(
            if (is_bid) { &pool.order_book.bids } else { &pool.order_book.asks },
            price
        );
        
        if (!tick_exists) {
            let price_level = PriceLevel {
                price,
                open_orders: linked_table::new(ctx),
                order_ids: vector::empty(),
            };
            
            tick_index = critbit_insert_leaf(
                if (is_bid) { &mut pool.order_book.bids } else { &mut pool.order_book.asks },
                price,
                price_level
            );
        };
        
        // Add order to price level
        let tick_level = critbit_borrow_mut_leaf_by_index(
            if (is_bid) { &mut pool.order_book.bids } else { &mut pool.order_book.asks },
            tick_index
        );
        linked_table::push_back(&mut tick_level.open_orders, order_id, order);
        // Add order_id to our tracking vector
        vector::push_back(&mut tick_level.order_ids, order_id);
        
        // Add to user's open orders
        if (!table::contains(&pool.order_book.user_orders, owner)) {
            table::add(&mut pool.order_book.user_orders, owner, linked_table::new(ctx));
        };
        linked_table::push_back(table::borrow_mut(&mut pool.order_book.user_orders, owner), order_id, price);
        
        // Emit event
        event::emit(LimitOrderPlacedEvent {
            pool_id: object::uid_to_address(&pool.id),
            order_id,
            client_order_id,
            is_bid,
            owner,
            quantity,
            price,
            expire_timestamp,
        });
        
        // Try to match the order immediately if possible
        match_limit_order(pool, config, order_id, ctx);
    }
    
    /// Helper function to lock user funds for a buy order
    fun lock_user_funds(pool: &mut TokenPool, user: address, amount: u64) {
        // No explicit locking mechanism in the current design, 
        // we just reduce the available balance
        let holder_balance = if (table::contains(&pool.holders, user)) {
            table::borrow_mut(&mut pool.holders, user)
        } else {
            table::add(&mut pool.holders, user, 0);
            table::borrow_mut(&mut pool.holders, user)
        };
        *holder_balance = *holder_balance - amount;
    }
    
    /// Try to match a limit order against existing orders
    fun match_limit_order(
        pool: &mut TokenPool, 
        config: &ExchangeConfig,
        order_id: u64,
        ctx: &mut TxContext
    ) {
        // Determine if this is a bid or ask order and get the reference to the order
        let (is_bid, price, quantity, owner) = get_order_info(pool, order_id);
        
        // If this is a bid order, we try to match it with ask orders
        // If this is an ask order, we try to match it with bid orders
        let target_book = if (is_bid) { &pool.order_book.asks } else { &pool.order_book.bids };
        
        // Check if the order book is empty
        if (critbit_is_empty(target_book)) {
            return
        };
        
        // Get the best price in the opposite side of the order book
        let (best_price, _) = if (is_bid) { 
            // For buy orders, we want the lowest sell price
            critbit_min_leaf(target_book)
        } else {
            // For sell orders, we want the highest buy price
            critbit_max_leaf(target_book)
        };
        
        // Check if the order can be matched based on price
        let matchable = if (is_bid) {
            // For buy orders, the buy price must be >= sell price
            price >= best_price
        } else {
            // For sell orders, the sell price must be <= buy price
            price <= best_price
        };
        
        if (!matchable) {
            return
        };
        
        // Process the matching
        let mut remaining_quantity = quantity;
        let mut current_price = best_price;
        
        while (remaining_quantity > 0 && matchable) {
            let (price_exists, tick_index) = critbit_find_leaf(target_book, current_price);
            
            if (!price_exists) {
                break
            };
            
            let tick_level = critbit_borrow_leaf_by_index(target_book, tick_index);
            
            if (linked_table::is_empty(&tick_level.open_orders)) {
                // Move to next price level
                let (next_price, _) = if (is_bid) {
                    critbit_next_leaf(target_book, current_price)
                } else {
                    critbit_previous_leaf(target_book, current_price)
                };
                
                if (next_price == 0) {
                    break
                };
                
                current_price = next_price;
                matchable = if (is_bid) {
                    price >= current_price
                } else {
                    price <= current_price
                };
                
                continue
            };
            
            // Process orders at this price level
            let (matched_quantity, continue_matching) = process_matching_at_price_level(
                pool,
                config,
                order_id,
                is_bid,
                owner,
                remaining_quantity,
                current_price,
                tick_index,
                ctx
            );
            
            remaining_quantity = remaining_quantity - matched_quantity;
            
            if (!continue_matching) {
                break
            };
            
            // Move to next price level if needed
            if (remaining_quantity > 0) {
                let (next_price, _) = if (is_bid) {
                    critbit_next_leaf(target_book, current_price)
                } else {
                    critbit_previous_leaf(target_book, current_price)
                };
                
                if (next_price == 0) {
                    break
                };
                
                current_price = next_price;
                matchable = if (is_bid) {
                    price >= current_price
                } else {
                    price <= current_price
                };
            };
        };
    }
    
    /// Match orders at a specific price level
    fun process_matching_at_price_level(
        pool: &mut TokenPool,
        config: &ExchangeConfig,
        order_id: u64,
        is_bid: bool,
        owner: address,
        remaining_quantity: u64,
        price: u64,
        tick_index: u64,
        ctx: &mut TxContext
    ): (u64, bool) {
        let target_book = if (is_bid) { &mut pool.order_book.asks } else { &mut pool.order_book.bids };
        let tick_level = critbit_borrow_mut_leaf_by_index(target_book, tick_index);
        
        let mut matched_quantity = 0;
        let mut continue_matching = true;
        
        // Iterate through orders at this price level using our vector of order IDs
        let order_ids = &tick_level.order_ids;
        let num_orders = vector::length(order_ids);
        
        let mut i = 0;
        while (i < num_orders && remaining_quantity > 0) {
            let maker_order_id = *vector::borrow(order_ids, i);
            let maker_order = linked_table::borrow(&tick_level.open_orders, maker_order_id);
            
            // Skip if maker is the same as taker (no self-matching)
            if (maker_order.owner == owner) {
                i = i + 1;
                continue
            };
            
            // Calculate fill amount
            let fill_amount = if (maker_order.quantity <= remaining_quantity) {
                maker_order.quantity
            } else {
                remaining_quantity
            };
            
            // Process the match
            let mut maker_order_mut = *maker_order;
            maker_order_mut.quantity = maker_order_mut.quantity - fill_amount;
            
            // Execute the trade - transfer tokens and funds
            execute_matched_trade(
                pool,
                config,
                is_bid,
                owner,
                maker_order.owner,
                fill_amount,
                price,
                ctx
            );
            
            // Update order quantities
            matched_quantity = matched_quantity + fill_amount;
            remaining_quantity = remaining_quantity - fill_amount;
            
            // Remove or update maker order
            if (maker_order_mut.quantity == 0) {
                // Remove the order completely
                linked_table::remove(&mut tick_level.open_orders, maker_order_id);
                // Also remove from our tracking vector - this is inefficient but safe for now
                let mut j = 0;
                let num_ids = vector::length(&tick_level.order_ids);
                while (j < num_ids) {
                    if (*vector::borrow(&tick_level.order_ids, j) == maker_order_id) {
                        vector::remove(&mut tick_level.order_ids, j);
                        break
                    };
                    j = j + 1;
                };
                
                // Remove from user open orders
                let maker_user_orders = table::borrow_mut(&mut pool.order_book.user_orders, maker_order.owner);
                linked_table::remove(maker_user_orders, maker_order_id);
            } else {
                // Update the order quantity
                let maker_order_update = linked_table::borrow_mut(&mut tick_level.open_orders, maker_order_id);
                *maker_order_update = maker_order_mut;
            };
            
            // Emit order filled event
            event::emit(LimitOrderFilledEvent {
                pool_id: object::uid_to_address(&pool.id),
                order_id: maker_order_id,
                taker_address: owner,
                maker_address: maker_order.owner,
                is_bid: !is_bid, // maker order is opposite side
                quantity_filled: fill_amount,
                quantity_remaining: maker_order_mut.quantity,
                price,
                fee_amount: 0, // Fees are handled separately in execute_matched_trade
            });
            
            i = i + 1;
        };
        
        // If the price level is now empty, remove it
        if (linked_table::is_empty(&tick_level.open_orders)) {
            // Need to implement a function to remove the tick level
            // For now we leave it, though it's empty
            continue_matching = false;
        };
        
        (matched_quantity, continue_matching)
    }
    
    /// Execute a matched trade by transferring tokens and funds
    fun execute_matched_trade(
        pool: &mut TokenPool,
        config: &ExchangeConfig,
        is_taker_bid: bool,
        taker: address,
        maker: address,
        quantity: u64,
        price: u64,
        ctx: &mut TxContext
    ) {
        let total_amount = price * quantity;
        
        // Calculate fees
        let fee_amount = (total_amount * config.total_fee_bps) / 10000;
        let creator_fee = (fee_amount * config.creator_fee_bps) / config.total_fee_bps;
        let platform_fee = (fee_amount * config.platform_fee_bps) / config.total_fee_bps;
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        if (is_taker_bid) {
            // Taker is buying, maker is selling
            
            // Update taker's token balance
            let taker_balance = if (table::contains(&pool.holders, taker)) {
                table::borrow_mut(&mut pool.holders, taker)
            } else {
                table::add(&mut pool.holders, taker, 0);
                table::borrow_mut(&mut pool.holders, taker)
            };
            *taker_balance = *taker_balance + quantity;
            
            // Pay maker the total amount minus fees
            let maker_balance = if (table::contains(&pool.holders, maker)) {
                table::borrow_mut(&mut pool.holders, maker)
            } else {
                table::add(&mut pool.holders, maker, 0);
                table::borrow_mut(&mut pool.holders, maker)
            };
            *maker_balance = *maker_balance + (total_amount - fee_amount);
            
            // Process fees
            if (fee_amount > 0) {
                // Distribute fees from the pool's balance
                if (creator_fee > 0) {
                    let creator_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, creator_fee), ctx);
                    transfer::public_transfer(creator_fee_coin, pool.info.owner);
                };
                
                if (platform_fee > 0) {
                    let platform_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, platform_fee), ctx);
                    transfer::public_transfer(platform_fee_coin, config.platform_treasury);
                };
                
                if (treasury_fee > 0) {
                    let treasury_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, treasury_fee), ctx);
                    transfer::public_transfer(treasury_fee_coin, config.ecosystem_treasury);
                };
            };
        } else {
            // Taker is selling, maker is buying
            
            // Update maker's token balance
            let maker_balance = if (table::contains(&pool.holders, maker)) {
                table::borrow_mut(&mut pool.holders, maker)
            } else {
                table::add(&mut pool.holders, maker, 0);
                table::borrow_mut(&mut pool.holders, maker)
            };
            *maker_balance = *maker_balance + quantity;
            
            // Pay taker the total amount minus fees
            let taker_balance = if (table::contains(&pool.holders, taker)) {
                table::borrow_mut(&mut pool.holders, taker)
            } else {
                table::add(&mut pool.holders, taker, 0);
                table::borrow_mut(&mut pool.holders, taker)
            };
            *taker_balance = *taker_balance + (total_amount - fee_amount);
            
            // Process fees
            if (fee_amount > 0) {
                // Distribute fees from the pool's balance
                if (creator_fee > 0) {
                    let creator_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, creator_fee), ctx);
                    transfer::public_transfer(creator_fee_coin, pool.info.owner);
                };
                
                if (platform_fee > 0) {
                    let platform_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, platform_fee), ctx);
                    transfer::public_transfer(platform_fee_coin, config.platform_treasury);
                };
                
                if (treasury_fee > 0) {
                    let treasury_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, treasury_fee), ctx);
                    transfer::public_transfer(treasury_fee_coin, config.ecosystem_treasury);
                };
            };
        };
    }
    
    /// Helper to get order information
    fun get_order_info(pool: &TokenPool, order_id: u64): (bool, u64, u64, address) {
        let (is_bid, order) = if (order_id % 2 == 0) {
            // Even order IDs are bids
            let (price_exists, tick_index) = critbit_find_leaf(&pool.order_book.bids, order_id);
            assert!(price_exists, EInvalidOrderId);
            let tick_level = critbit_borrow_leaf_by_index(&pool.order_book.bids, tick_index);
            (true, linked_table::borrow(&tick_level.open_orders, order_id))
        } else {
            // Odd order IDs are asks
            let (price_exists, tick_index) = critbit_find_leaf(&pool.order_book.asks, order_id);
            assert!(price_exists, EInvalidOrderId);
            let tick_level = critbit_borrow_leaf_by_index(&pool.order_book.asks, tick_index);
            (false, linked_table::borrow(&tick_level.open_orders, order_id))
        };
        
        (is_bid, order.price, order.quantity, order.owner)
    }
    
    /// Cancel a limit order
    public entry fun cancel_limit_order(
        pool: &mut TokenPool,
        order_id: u64,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        
        // Find the order and verify ownership
        let (is_bid, price, quantity, owner) = get_order_info(pool, order_id);
        assert!(sender == owner, EUnauthorizedCancel);
        
        // Remove the order from the order book
        let target_book = if (is_bid) { &mut pool.order_book.bids } else { &mut pool.order_book.asks };
        let (_, tick_index) = critbit_find_leaf(target_book, price);
        let tick_level = critbit_borrow_mut_leaf_by_index(target_book, tick_index);
        
        // Get the order details before removing it
        let order = linked_table::remove(&mut tick_level.open_orders, order_id);
        
        // Remove the order from our tracking vector
        let mut i = 0;
        let num_ids = vector::length(&tick_level.order_ids);
        while (i < num_ids) {
            if (*vector::borrow(&tick_level.order_ids, i) == order_id) {
                vector::remove(&mut tick_level.order_ids, i);
                break
            };
            i = i + 1;
        };
        
        // Remove from user open orders
        let user_orders = table::borrow_mut(&mut pool.order_book.user_orders, owner);
        linked_table::remove(user_orders, order_id);
        
        // Refund locked funds or tokens
        if (is_bid) {
            // Refund MYS for buy orders
            let total_amount = price * quantity;
            let user_balance = table::borrow_mut(&mut pool.holders, owner);
            *user_balance = *user_balance + total_amount;
        } else {
            // Refund tokens for sell orders
            let user_balance = table::borrow_mut(&mut pool.holders, owner);
            *user_balance = *user_balance + quantity;
        };
        
        // Emit cancel event
        event::emit(LimitOrderCanceledEvent {
            pool_id: object::uid_to_address(&pool.id),
            order_id,
            client_order_id: order.client_order_id,
            is_bid,
            owner,
            quantity,
            price,
        });
    }
    
    /// Query open orders for a user
    public fun get_user_open_orders(pool: &TokenPool, user: address): vector<u64> {
        if (!table::contains(&pool.order_book.user_orders, user)) {
            return vector::empty<u64>()
        };
        
        let user_orders = table::borrow(&pool.order_book.user_orders, user);
        let result = vector::empty<u64>();
        
        // Start with the front item
        let maybe_front_id = linked_table::front(user_orders);
        if (option::is_none(maybe_front_id)) {
            return result
        };
        
        // Get the first ID and add it to the result
        let current_id = *option::borrow(maybe_front_id);
        vector::push_back(&mut result, current_id);
        
        // Now iterate through next links until we reach the end
        let mut next_id_opt = linked_table::next(user_orders, current_id);
        
        while (option::is_some(next_id_opt)) {
            let next_id = *option::borrow(next_id_opt);
            vector::push_back(&mut result, next_id);
            next_id_opt = linked_table::next(user_orders, next_id);
        };
        
        result
    }
    
    /// Get the current best bid and ask prices
    public fun get_best_prices(pool: &TokenPool): (Option<u64>, Option<u64>) {
        let best_bid = if (!critbit_is_empty(&pool.order_book.bids)) {
            let (price, _) = critbit_max_leaf(&pool.order_book.bids);
            option::some(price)
        } else {
            option::none()
        };
        
        let best_ask = if (!critbit_is_empty(&pool.order_book.asks)) {
            let (price, _) = critbit_min_leaf(&pool.order_book.asks);
            option::some(price)
        } else {
            option::none()
        };
        
        (best_bid, best_ask)
    }
    
    /// Helper to query token balance and locked funds
    fun account_balance(
        _registry: &TokenRegistry,
        pool: &TokenPool,
        owner: address
    ): (u64, u64, u64, u64) {
        let base_avail = if (table::contains(&pool.holders, owner)) {
            *table::borrow(&pool.holders, owner)
        } else {
            0
        };
        
        // The current implementation doesn't track locked tokens separately,
        // but in a production system, you would want to track this
        let base_locked = 0;
        let quote_avail = 0;
        let quote_locked = 0;
        
        (base_avail, base_locked, quote_avail, quote_locked)
    }
} 