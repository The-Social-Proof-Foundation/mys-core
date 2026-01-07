// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Social Proof Tokens module for MySocial platform.
/// This module provides functionality for creation and trading of both profile tokens
/// and post tokens using an Automated Market Maker (AMM) with a quadratic pricing curve.
/// It includes fee distribution mechanisms for transactions, splitting between profile owner,
/// platform, and ecosystem treasury.

#[allow(unused_field, deprecated_usage, unused_const, duplicate_alias, unused_use)]
module social_contracts::social_proof_tokens {
    use std::string::{Self, String};
    use std::option::{Self, Option};
    use std::vector;

    use mys::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        event,
        table::{Self, Table},
        coin::{Self, Coin},
        mys::MYS,
        balance::{Self, Balance},
        clock::{Self, Clock},
        math,
        package::{Self, Publisher}
    };
    
    use social_contracts::profile::{Self, Profile, UsernameRegistry, EcosystemTreasury};
    use social_contracts::post::{Self, Post};
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::platform::{Self, PlatformRegistry};
    use social_contracts::upgrade::{Self, UpgradeAdminCap};

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
    /// Trading is halted by emergency kill switch
    const ETradingHalted: u64 = 21;
    /// Arithmetic overflow detected
    const EOverflow: u64 = 22;
    /// Wrong version - object version mismatch
    const EWrongVersion: u64 = 23;
    /// User has not joined the platform
    const EUserNotJoinedPlatform: u64 = 24;
    /// User is blocked by the platform
    const EUserBlockedByPlatform: u64 = 25;
    /// Reservation pool already converted to token
    const EReservationPoolConverted: u64 = 27;
    /// User already owns tokens for this pool
    const EAlreadyOwnsTokens: u64 = 28;
    /// Too many reservers for conversion (DoS prevention)
    const ETooManyReservers: u64 = 29;

    // === Constants ===
    // Token types
    const TOKEN_TYPE_PROFILE: u8 = 1;
    const TOKEN_TYPE_POST: u8 = 2;

    // Default trading fee percentages (in basis points, 10000 = 100%)
    const DEFAULT_TRADING_CREATOR_FEE_BPS: u64 = 100; // 1.0% to creator (profile/post owner)
    const DEFAULT_TRADING_PLATFORM_FEE_BPS: u64 = 25; // 0.25% to platform
    const DEFAULT_TRADING_TREASURY_FEE_BPS: u64 = 25; // 0.25% to ecosystem treasury

    // Default reservation fee percentages (in basis points, 10000 = 100%)
    const DEFAULT_RESERVATION_CREATOR_FEE_BPS: u64 = 100; // 1.0% to creator (profile/post owner)
    const DEFAULT_RESERVATION_PLATFORM_FEE_BPS: u64 = 25; // 0.25% to platform
    const DEFAULT_RESERVATION_TREASURY_FEE_BPS: u64 = 25; // 0.25% to ecosystem treasury

    // Maximum hold percentage per wallet (5% of supply)
    const MAX_HOLD_PERCENT_BPS: u64 = 500;

    // Default AMM curve parameters
    const DEFAULT_BASE_PRICE: u64 = 100_000_000; // 0.1 MYS in smallest units
    const DEFAULT_QUADRATIC_COEFFICIENT: u64 = 100_000; // Coefficient for quadratic curve

    // Reservation threshold constants for social proof token creation
    const DEFAULT_POST_THRESHOLD: u64 = 1_000_000_000_000; // 1,000 MYS in smallest units (9 decimals)
    const DEFAULT_PROFILE_THRESHOLD: u64 = 10_000_000_000_000; // 10,000 MYS in smallest units (9 decimals)
    const DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS: u64 = 2000; // 20% (1/5 of threshold)
    const DEFAULT_MAX_RESERVERS_PER_POOL: u64 = 1000;

    // Maximum u64 value for overflow protection
    const MAX_U64: u64 = 18446744073709551615;

    // === Structs ===

    /// Admin capability for the social proof tokens system
    public struct SocialProofTokensAdminCap has key, store {
        id: UID,
    }

    /// Global social proof tokens configuration
    public struct SocialProofTokensConfig has key {
        id: UID,
        /// Creator fee percentage in basis points (for trading)
        trading_creator_fee_bps: u64,
        /// Platform fee percentage in basis points (for trading)
        trading_platform_fee_bps: u64,
        /// Treasury fee percentage in basis points (for trading)
        trading_treasury_fee_bps: u64,
        /// Creator reservation fee percentage in basis points
        reservation_creator_fee_bps: u64,
        /// Platform reservation fee percentage in basis points
        reservation_platform_fee_bps: u64,
        /// Treasury reservation fee percentage in basis points
        reservation_treasury_fee_bps: u64,
        /// Base price for new tokens
        base_price: u64,
        /// Quadratic coefficient for pricing curve
        quadratic_coefficient: u64,
        /// Maximum percentage a single wallet can hold of any token
        max_hold_percent_bps: u64,
        /// Reservation thresholds for social proof token creation
        post_threshold: u64,
        profile_threshold: u64,
        /// Maximum percentage any individual can reserve towards a single post/profile
        max_individual_reservation_bps: u64,
        /// Maximum number of unique reservers allowed per pool (DoS protection)
        max_reservers_per_pool: u64,
        /// Emergency kill switch - when false, all trading is halted
        trading_enabled: bool,
    }

    /// Registry of all tokens in the exchange
    public struct TokenRegistry has key {
        id: UID,
        /// Table keyed by associated_id (post/profile ID), not pool ID, to token info
        tokens: Table<address, TokenInfo>,
        /// Table from profile/post ID to reservation pool info
        reservation_pools: Table<address, ReservationPool>,
        /// Version for upgrades
        version: u64,
    }

    /// Reservation pool for a specific post or profile
    /// Note: reservers vector is only stored in ReservationPoolObject, not in registry
    public struct ReservationPool has store, drop {
        /// Associated profile or post ID
        associated_id: address,
        /// Token type (1=profile, 2=post)
        token_type: u8,
        /// Owner of the profile/post
        owner: address,
        /// Total MYS reserved towards this post/profile
        total_reserved: u64,
        /// Required threshold to enable auction creation
        required_threshold: u64,
        /// Creation timestamp
        created_at: u64,
    }

    /// Information about a token
    public struct TokenInfo has store, drop {
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
    public struct TokenPool has key, store {
        id: UID,
        /// The token's info
        info: TokenInfo,
        /// MYS balance in the pool
        mys_balance: Balance<MYS>,
        /// Mapping of holders' addresses to their token balances
        holders: Table<address, u64>,
        /// PoC revenue redirection address (for post tokens only)
        poc_redirect_to: Option<address>,
        /// PoC revenue redirection percentage (for post tokens only)
        poc_redirect_percentage: Option<u64>,
        /// Version for upgrades
        version: u64,
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

    /// Reservation pool for collecting MYS reservations towards posts/profiles
    public struct ReservationPoolObject has key {
        id: UID,
        /// Reservation pool info (without reservers - kept separately below)
        info: ReservationPool,
        /// MYS balance reserved in this pool
        mys_balance: Balance<MYS>,
        /// Mapping of reservers' addresses to their reservation amounts
        reservations: Table<address, u64>,
        /// List of all reservers (for efficient iteration) - only in object, not in registry
        reservers: vector<address>,
        /// Flag indicating if this pool has been converted to a token
        converted: bool,
        /// Version for upgrades
        version: u64,
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

    /// Event emitted when a post pool is auto-initialized by SPoT flow
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

    /// Event emitted when MYS is reserved towards a post/profile
    public struct ReservationCreatedEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        reserver: address,
        amount: u64,
        total_reserved: u64,
        threshold_met: bool,
        reserved_at: u64,
        fee_amount: u64,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
    }

    /// Event emitted when MYS reservation is withdrawn
    public struct ReservationWithdrawnEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        reserver: address,
        amount: u64,
        total_reserved: u64,
        withdrawn_at: u64,
        fee_amount: u64,
        creator_fee: u64,
        platform_fee: u64,
        treasury_fee: u64,
    }

    /// Event emitted when reservation threshold is met for the first time
    public struct ThresholdMetEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        owner: address,
        total_reserved: u64,
        required_threshold: u64,
        timestamp: u64,
    }

    /// Event emitted when a reservation pool is created
    public struct ReservationPoolCreatedEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        owner: address,
        required_threshold: u64,
        pool_object_id: address,
        created_at: u64,
    }

    /// Event emitted when social proof tokens config is updated
    public struct ConfigUpdatedEvent has copy, drop {
        /// Who performed the update
        updated_by: address,
        /// When the update occurred
        timestamp: u64,
        /// Trading fee percentages
        total_fee_bps: u64,
        trading_creator_fee_bps: u64,
        trading_platform_fee_bps: u64,
        trading_treasury_fee_bps: u64,
        /// Reservation fee percentages
        reservation_total_fee_bps: u64,
        reservation_creator_fee_bps: u64,
        reservation_platform_fee_bps: u64,
        reservation_treasury_fee_bps: u64,
        /// Curve parameters
        base_price: u64,
        quadratic_coefficient: u64,
        /// Maximum hold percentage
        max_hold_percent_bps: u64,
        /// Reservation thresholds
        post_threshold: u64,
        profile_threshold: u64,
        max_individual_reservation_bps: u64,
        max_reservers_per_pool: u64,
    }

    /// Event emitted when tokens are purchased by someone who already has a social token
    public struct TokensAddedEvent has copy, drop {
        owner: address, 
        pool_id: address,
        amount: u64,
    }

    /// Event emitted when emergency kill switch is toggled
    public struct EmergencyKillSwitchEvent has copy, drop {
        /// Admin who activated/deactivated the kill switch
        admin: address,
        /// New state of trading (true = enabled, false = halted)
        trading_enabled: bool,
        /// Timestamp of the action
        timestamp: u64,
        /// Reason for the action (optional)
        reason: String,
    }

    /// Event emitted when PoC redirection data is updated for a token pool
    public struct PocRedirectionUpdatedEvent has copy, drop {
        /// Token pool ID
        pool_id: address,
        /// Associated post ID
        post_id: address,
        /// Address to redirect revenue to (None if cleared)
        redirect_to: Option<address>,
        /// Percentage of revenue to redirect (None if cleared)
        redirect_percentage: Option<u64>,
        /// Who performed the update
        updated_by: address,
        /// Timestamp of the update
        timestamp: u64,
    }

    /// Bootstrap initialization function - creates the social proof tokens configuration and registry
    public(package) fun bootstrap_init(ctx: &mut TxContext) {
        // Create and share social proof tokens config with proper treasury
        transfer::share_object(
            SocialProofTokensConfig {
                id: object::new(ctx),
                trading_creator_fee_bps: DEFAULT_TRADING_CREATOR_FEE_BPS,
                trading_platform_fee_bps: DEFAULT_TRADING_PLATFORM_FEE_BPS,
                trading_treasury_fee_bps: DEFAULT_TRADING_TREASURY_FEE_BPS,
                reservation_creator_fee_bps: DEFAULT_RESERVATION_CREATOR_FEE_BPS,
                reservation_platform_fee_bps: DEFAULT_RESERVATION_PLATFORM_FEE_BPS,
                reservation_treasury_fee_bps: DEFAULT_RESERVATION_TREASURY_FEE_BPS,
                base_price: DEFAULT_BASE_PRICE,
                quadratic_coefficient: DEFAULT_QUADRATIC_COEFFICIENT,
                max_hold_percent_bps: MAX_HOLD_PERCENT_BPS,
                post_threshold: DEFAULT_POST_THRESHOLD,
                profile_threshold: DEFAULT_PROFILE_THRESHOLD,
                max_individual_reservation_bps: DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS,
                max_reservers_per_pool: DEFAULT_MAX_RESERVERS_PER_POOL,
                trading_enabled: false, // Trading disabled by default during bootstrap
            }
        );
        
        // Create and share token registry
        transfer::share_object(
            TokenRegistry {
                id: object::new(ctx),
                tokens: table::new(ctx),
                reservation_pools: table::new(ctx),
                version: upgrade::current_version(),
            }
        );
    }

    // === Admin Functions ===

    /// Update social proof tokens configuration
    public entry fun update_social_proof_tokens_config(
        _admin_cap: &SocialProofTokensAdminCap,
        config: &mut SocialProofTokensConfig,
        trading_creator_fee_bps: u64,
        trading_platform_fee_bps: u64,
        trading_treasury_fee_bps: u64,
        reservation_creator_fee_bps: u64,
        reservation_platform_fee_bps: u64,
        reservation_treasury_fee_bps: u64,
        base_price: u64,
        quadratic_coefficient: u64,
        max_hold_percent_bps: u64,
        post_threshold: u64,
        profile_threshold: u64,
        max_individual_reservation_bps: u64,
        max_reservers_per_pool: u64,
        ctx: &mut TxContext
    ) {
        // Verify curve parameters are valid
        assert!(base_price > 0 && quadratic_coefficient > 0, EInvalidCurveParams);
        
        // Validate fee configurations to prevent division by zero and overflow
        // Calculate totals before updating to validate
        let total_fee_bps = trading_creator_fee_bps + trading_platform_fee_bps + trading_treasury_fee_bps;
        let reservation_total_fee_bps = reservation_creator_fee_bps + reservation_platform_fee_bps + reservation_treasury_fee_bps;
        
        // Ensure fee totals are valid (prevent division by zero and overflow)
        assert!(total_fee_bps > 0 && total_fee_bps <= 10000, EInvalidFeeConfig);
        assert!(reservation_total_fee_bps > 0 && reservation_total_fee_bps <= 10000, EInvalidFeeConfig);
        
        // Validate individual fee components don't exceed 100%
        
        // Validate max_hold_percent_bps (should be <= 10000, i.e., <= 100%)
        assert!(max_hold_percent_bps > 0 && max_hold_percent_bps <= 10000, EInvalidFeeConfig);
        
        // Validate thresholds (must be positive)
        assert!(post_threshold > 0, EInvalidFeeConfig);
        assert!(profile_threshold > 0, EInvalidFeeConfig);
        
        // Validate max_individual_reservation_bps (should be <= 10000, i.e., <= 100%)
        assert!(max_individual_reservation_bps > 0 && max_individual_reservation_bps <= 10000, EInvalidFeeConfig);
        
        // Validate max_reservers_per_pool (DoS protection limit)
        assert!(max_reservers_per_pool > 0 && max_reservers_per_pool <= 50000, EInvalidFeeConfig);
        
        assert!(trading_creator_fee_bps <= 10000, EInvalidFeeConfig);
        assert!(trading_platform_fee_bps <= 10000, EInvalidFeeConfig);
        assert!(trading_treasury_fee_bps <= 10000, EInvalidFeeConfig);
        assert!(reservation_creator_fee_bps <= 10000, EInvalidFeeConfig);
        assert!(reservation_platform_fee_bps <= 10000, EInvalidFeeConfig);
        assert!(reservation_treasury_fee_bps <= 10000, EInvalidFeeConfig);
        
        // Update trading fee config
        config.trading_creator_fee_bps = trading_creator_fee_bps;
        config.trading_platform_fee_bps = trading_platform_fee_bps;
        config.trading_treasury_fee_bps = trading_treasury_fee_bps;
        
        // Update reservation fee config
        config.reservation_creator_fee_bps = reservation_creator_fee_bps;
        config.reservation_platform_fee_bps = reservation_platform_fee_bps;
        config.reservation_treasury_fee_bps = reservation_treasury_fee_bps;
        
        // Update curve parameters
        config.base_price = base_price;
        config.quadratic_coefficient = quadratic_coefficient;
        
        // Update max hold percentage
        config.max_hold_percent_bps = max_hold_percent_bps;
        
        // Update reservation thresholds
        config.post_threshold = post_threshold;
        config.profile_threshold = profile_threshold;
        config.max_individual_reservation_bps = max_individual_reservation_bps;
        config.max_reservers_per_pool = max_reservers_per_pool;
        
        // Calculate totals for event emission
        let total_fee_bps = calculate_total_fee_bps(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        
        // Emit config updated event
        event::emit(ConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            timestamp: tx_context::epoch(ctx),
            total_fee_bps,
            trading_creator_fee_bps,
            trading_platform_fee_bps,
            trading_treasury_fee_bps,
            reservation_total_fee_bps,
            reservation_creator_fee_bps,
            reservation_platform_fee_bps,
            reservation_treasury_fee_bps,
            base_price,
            quadratic_coefficient,
            max_hold_percent_bps,
            post_threshold,
            profile_threshold,
            max_individual_reservation_bps,
            max_reservers_per_pool,
        });
    }

    /// Emergency kill switch function - only callable by admin
    /// This function can immediately enable or halt all trading on the platform
    public entry fun toggle_emergency_kill_switch(
        _admin_cap: &SocialProofTokensAdminCap,
        config: &mut SocialProofTokensConfig,
        enable_trading: bool,
        reason: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Update the trading enabled status
        config.trading_enabled = enable_trading;
        
        // Emit event for audit trail
        event::emit(EmergencyKillSwitchEvent {
            admin: tx_context::sender(ctx),
            trading_enabled: enable_trading,
            timestamp: tx_context::epoch(ctx),
            reason: string::utf8(reason),
        });
    }

    /// Check if trading is currently enabled
    public fun is_trading_enabled(config: &SocialProofTokensConfig): bool {
        config.trading_enabled
    }

    /// Calculate total trading fee from component fees
    public(package) fun calculate_total_fee_bps(config: &SocialProofTokensConfig): u64 {
        config.trading_creator_fee_bps + config.trading_platform_fee_bps + config.trading_treasury_fee_bps
    }

    /// Calculate total reservation fee from component fees
    public(package) fun calculate_reservation_total_fee_bps(config: &SocialProofTokensConfig): u64 {
        config.reservation_creator_fee_bps + config.reservation_platform_fee_bps + config.reservation_treasury_fee_bps
    }

    /// Validate trading fee config before use
    public(package) fun validate_trading_fees(config: &SocialProofTokensConfig) {
        let total_fee_bps = calculate_total_fee_bps(config);
        assert!(total_fee_bps > 0 && total_fee_bps <= 10000, EInvalidFeeConfig);
    }

    /// Validate reservation fee config before use
    public(package) fun validate_reservation_fees(config: &SocialProofTokensConfig) {
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        assert!(reservation_total_fee_bps > 0 && reservation_total_fee_bps <= 10000, EInvalidFeeConfig);
    }

    /// Calculate fee amount with overflow protection
    /// amount * fee_bps can overflow before division, so check first
    public(package) fun calculate_fee_amount_safe(amount: u64, fee_bps: u64): u64 {
        // Overflow protection: amount * fee_bps can overflow before division
        if (amount == 0 || fee_bps == 0) {
            return 0
        };
        assert!(amount <= MAX_U64 / fee_bps, EOverflow);
        (amount * fee_bps) / 10000
    }

    /// Calculate component fee from total fee amount
    /// component_fee = (fee_amount * component_bps) / total_bps
    /// With overflow protection
    public(package) fun calculate_component_fee_safe(fee_amount: u64, component_bps: u64, total_bps: u64): u64 {
        if (fee_amount == 0 || component_bps == 0 || total_bps == 0) {
            return 0
        };
        // Overflow protection: fee_amount * component_bps can overflow
        assert!(fee_amount <= MAX_U64 / component_bps, EOverflow);
        (fee_amount * component_bps) / total_bps
    }

    // === Reservation Functions ===

    /// Reserve MYS tokens towards a post to support social proof token creation
    /// Anyone can call this function - the post owner is stored in the reservation pool
    /// Reserve MYS tokens towards a post to support social proof token creation
    /// Non-platform version: platform fees go to ecosystem treasury
    #[allow(lint(self_transfer))]
    public fun reserve_towards_post(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        post: &Post,
        payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        // Prevent reservations after conversion to token
        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        
        let reserver = tx_context::sender(ctx);
        // Get post ID and owner from reservation pool
        let post_id = reservation_pool_object.info.associated_id;
        let post_owner = reservation_pool_object.info.owner;
        let now = tx_context::epoch(ctx);
        
        // Verify reservation pool is for a post
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_POST, EInvalidTokenType);
        
        // Verify post matches reservation pool
        assert!(post::get_id_address(post) == post_id, EInvalidID);
        
        // Ensure reserver has enough funds
        assert!(coin::value(&payment) >= amount && amount > 0, EInsufficientFunds);
        
        // Calculate and distribute fees (non-platform version)
        let (mut remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = distribute_reservation_fees_with_post(
            config,
            reservation_pool_object,
            post,
            amount,
            payment,
            treasury,
            ctx
        );
        
        // Net amount after fees
        let net_amount = amount - fee_amount;
        
        // Check individual reservation limit (based on net amount)
        let max_individual_reservation = (config.post_threshold * config.max_individual_reservation_bps) / 10000;
        let current_reservation = if (table::contains(&reservation_pool_object.reservations, reserver)) {
            *table::borrow(&reservation_pool_object.reservations, reserver)
        } else {
            0
        };
        assert!(current_reservation + net_amount <= max_individual_reservation, EExceededMaxHold);
        
        // Extract net reservation payment
        let reservation_payment = coin::split(&mut remaining_payment, net_amount, ctx);
        balance::join(&mut reservation_pool_object.mys_balance, coin::into_balance(reservation_payment));
        
        // Update reserver's balance in the pool (store net amount)
        if (table::contains(&reservation_pool_object.reservations, reserver)) {
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            assert!(*reservation_balance <= MAX_U64 - net_amount, EOverflow);
            *reservation_balance = *reservation_balance + net_amount;
        } else {
            // DoS protection: limit number of unique reservers per pool
            let current_reservers_count = vector::length(&reservation_pool_object.reservers);
            assert!(current_reservers_count < config.max_reservers_per_pool, ETooManyReservers);
            
            table::add(&mut reservation_pool_object.reservations, reserver, net_amount);
            // Add to reservers list for tracking
            vector::push_back(&mut reservation_pool_object.reservers, reserver);
        };

        // Update total reserved (with net amount)

        assert!(reservation_pool_object.info.total_reserved <= MAX_U64 - net_amount, EOverflow);
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;

        // Update registry
        if (table::contains(&registry.reservation_pools, post_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, post_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        } else {
            // Create registry entry if it doesn't exist
            let reservation_pool = ReservationPool {
                associated_id: post_id,
                token_type: TOKEN_TYPE_POST,
                owner: post_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.post_threshold,
                created_at: now,
            };
            table::add(&mut registry.reservation_pools, post_id, reservation_pool);
        };
        
        // Check if threshold was just met
        let threshold_met = reservation_pool_object.info.total_reserved >= config.post_threshold;
        let was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) >= config.post_threshold;
        
        // Emit threshold met event if this reservation pushed us over the threshold
        if (threshold_met && !was_threshold_met) {
            event::emit(ThresholdMetEvent {
                associated_id: post_id,
                token_type: TOKEN_TYPE_POST,
                owner: post_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.post_threshold,
                timestamp: now,
            });
        };
        
        // Return excess payment
        if (coin::value(&remaining_payment) > 0) {
            transfer::public_transfer(remaining_payment, reserver);
        } else {
            coin::destroy_zero(remaining_payment);
        };
        
        // Emit reservation created event
        event::emit(ReservationCreatedEvent {
            associated_id: post_id,
            token_type: TOKEN_TYPE_POST,
            reserver,
            amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            threshold_met,
            reserved_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Reserve MYS tokens towards a post to support social proof token creation
    /// Platform version: platform fees go to platform treasury, includes platform validation
    #[allow(lint(self_transfer))]
    public fun reserve_towards_post_with_platform(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        platform: &mut social_contracts::platform::Platform,
        block_list_registry: &BlockListRegistry,
        post: &Post,
        payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        // Prevent reservations after conversion to token
        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        
        let reserver = tx_context::sender(ctx);
        // Get post ID and owner from reservation pool
        let post_id = reservation_pool_object.info.associated_id;
        let post_owner = reservation_pool_object.info.owner;
        let now = tx_context::epoch(ctx);
        
        // Verify reservation pool is for a post
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_POST, EInvalidTokenType);
        
        // Verify post matches reservation pool
        assert!(post::get_id_address(post) == post_id, EInvalidID);
        
        // Ensure reserver has enough funds
        assert!(coin::value(&payment) >= amount && amount > 0, EInsufficientFunds);
        
        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, reserver), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, reserver), EUserBlockedByPlatform);
        
        // Calculate and distribute fees (platform version)
        let (mut remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = distribute_reservation_fees_with_post_and_platform(
            config,
            reservation_pool_object,
            post,
            amount,
            payment,
            treasury,
            platform,
            ctx
        );
        
        // Net amount after fees
        let net_amount = amount - fee_amount;
        
        // Check individual reservation limit (based on net amount)
        let max_individual_reservation = (config.post_threshold * config.max_individual_reservation_bps) / 10000;
        let current_reservation = if (table::contains(&reservation_pool_object.reservations, reserver)) {
            *table::borrow(&reservation_pool_object.reservations, reserver)
        } else {
            0
        };
        assert!(current_reservation + net_amount <= max_individual_reservation, EExceededMaxHold);
        
        // Extract net reservation payment
        let reservation_payment = coin::split(&mut remaining_payment, net_amount, ctx);
        balance::join(&mut reservation_pool_object.mys_balance, coin::into_balance(reservation_payment));
        
        // Update reserver's balance in the pool (store net amount)
        if (table::contains(&reservation_pool_object.reservations, reserver)) {
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            assert!(*reservation_balance <= MAX_U64 - net_amount, EOverflow);
            *reservation_balance = *reservation_balance + net_amount;
        } else {
            // DoS protection: limit number of unique reservers per pool
            let current_reservers_count = vector::length(&reservation_pool_object.reservers);
            assert!(current_reservers_count < config.max_reservers_per_pool, ETooManyReservers);
            
            table::add(&mut reservation_pool_object.reservations, reserver, net_amount);
            // Add to reservers list for tracking
            vector::push_back(&mut reservation_pool_object.reservers, reserver);
        };

        // Update total reserved (with net amount)
        assert!(reservation_pool_object.info.total_reserved <= MAX_U64 - net_amount, EOverflow);
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;

        // Update registry
        if (table::contains(&registry.reservation_pools, post_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, post_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        } else {
            // Create registry entry if it doesn't exist
            let reservation_pool = ReservationPool {
                associated_id: post_id,
                token_type: TOKEN_TYPE_POST,
                owner: post_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.post_threshold,
                created_at: now,
            };
            table::add(&mut registry.reservation_pools, post_id, reservation_pool);
        };
        
        // Check if threshold was just met
        let threshold_met = reservation_pool_object.info.total_reserved >= config.post_threshold;
        let was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) >= config.post_threshold;
        
        // Emit threshold met event if this reservation pushed us over the threshold
        if (threshold_met && !was_threshold_met) {
            event::emit(ThresholdMetEvent {
                associated_id: post_id,
                token_type: TOKEN_TYPE_POST,
                owner: post_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.post_threshold,
                timestamp: now,
            });
        };
        
        // Return excess payment
        if (coin::value(&remaining_payment) > 0) {
            transfer::public_transfer(remaining_payment, reserver);
        } else {
            coin::destroy_zero(remaining_payment);
        };
        
        // Emit reservation created event
        event::emit(ReservationCreatedEvent {
            associated_id: post_id,
            token_type: TOKEN_TYPE_POST,
            reserver,
            amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            threshold_met,
            reserved_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Reserve MYS tokens towards a profile to support social proof token creation
    /// Non-platform version: platform fees go to ecosystem treasury
    /// Anyone can call this function - the profile owner is stored in the reservation pool
    #[allow(lint(self_transfer))]
    public fun reserve_towards_profile(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        // Prevent reservations after conversion to token
        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        
        let reserver = tx_context::sender(ctx);
        // Get profile ID and owner from reservation pool
        let profile_id = reservation_pool_object.info.associated_id;
        let profile_owner = reservation_pool_object.info.owner;
        let now = tx_context::epoch(ctx);
        
        // Verify reservation pool is for a profile
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_PROFILE, EInvalidTokenType);
        
        // Ensure reserver has enough funds
        assert!(coin::value(&payment) >= amount && amount > 0, EInsufficientFunds);
        
        // Calculate and distribute fees (non-platform version, no PoC for profiles)
        let (mut remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = distribute_reservation_fees_no_poc(
            config,
            reservation_pool_object,
            amount,
            payment,
            treasury,
            ctx
        );
        
        // Net amount after fees
        let net_amount = amount - fee_amount;
        
        // Check individual reservation limit (based on net amount)
        let max_individual_reservation = (config.profile_threshold * config.max_individual_reservation_bps) / 10000;
        let current_reservation = if (table::contains(&reservation_pool_object.reservations, reserver)) {
            *table::borrow(&reservation_pool_object.reservations, reserver)
        } else {
            0
        };
        assert!(current_reservation + net_amount <= max_individual_reservation, EExceededMaxHold);
        
        // Extract net reservation payment
        let reservation_payment = coin::split(&mut remaining_payment, net_amount, ctx);
        balance::join(&mut reservation_pool_object.mys_balance, coin::into_balance(reservation_payment));

        // Update reserver's balance in the pool (store net amount)
        if (table::contains(&reservation_pool_object.reservations, reserver)) {
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            assert!(*reservation_balance <= MAX_U64 - net_amount, EOverflow);
            *reservation_balance = *reservation_balance + net_amount;
        } else {
            // DoS protection: limit number of unique reservers per pool
            let current_reservers_count = vector::length(&reservation_pool_object.reservers);
            assert!(current_reservers_count < config.max_reservers_per_pool, ETooManyReservers);
            
            table::add(&mut reservation_pool_object.reservations, reserver, net_amount);
            // Add to reservers list for tracking
            vector::push_back(&mut reservation_pool_object.reservers, reserver);
        };

        // Update total reserved (with net amount)

        assert!(reservation_pool_object.info.total_reserved <= MAX_U64 - net_amount, EOverflow);
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;

        // Update registry
        if (table::contains(&registry.reservation_pools, profile_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, profile_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        } else {
            // Create registry entry if it doesn't exist
            let reservation_pool = ReservationPool {
                associated_id: profile_id,
                token_type: TOKEN_TYPE_PROFILE,
                owner: profile_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.profile_threshold,
                created_at: now,
            };
            table::add(&mut registry.reservation_pools, profile_id, reservation_pool);
        };
        
        // Check if threshold was just met
        let threshold_met = reservation_pool_object.info.total_reserved >= config.profile_threshold;
        let was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) >= config.profile_threshold;
        
        // Emit threshold met event if this reservation pushed us over the threshold
        if (threshold_met && !was_threshold_met) {
            event::emit(ThresholdMetEvent {
                associated_id: profile_id,
                token_type: TOKEN_TYPE_PROFILE,
                owner: profile_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.profile_threshold,
                timestamp: now,
            });
        };
        
        // Return excess payment
        if (coin::value(&remaining_payment) > 0) {
            transfer::public_transfer(remaining_payment, reserver);
        } else {
            coin::destroy_zero(remaining_payment);
        };
        
        // Emit reservation created event
        event::emit(ReservationCreatedEvent {
            associated_id: profile_id,
            token_type: TOKEN_TYPE_PROFILE,
            reserver,
            amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            threshold_met,
            reserved_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Reserve MYS tokens towards a profile to support social proof token creation
    /// Platform version: platform fees go to platform treasury, includes platform validation
    /// Anyone can call this function - the profile owner is stored in the reservation pool
    #[allow(lint(self_transfer))]
    public fun reserve_towards_profile_with_platform(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        platform: &mut social_contracts::platform::Platform,
        block_list_registry: &BlockListRegistry,
        payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        // Prevent reservations after conversion to token
        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        
        let reserver = tx_context::sender(ctx);
        // Get profile ID and owner from reservation pool
        let profile_id = reservation_pool_object.info.associated_id;
        let profile_owner = reservation_pool_object.info.owner;
        let now = tx_context::epoch(ctx);
        
        // Verify reservation pool is for a profile
        assert!(reservation_pool_object.info.token_type == TOKEN_TYPE_PROFILE, EInvalidTokenType);
        
        // Ensure reserver has enough funds
        assert!(coin::value(&payment) >= amount && amount > 0, EInsufficientFunds);
        
        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, reserver), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, reserver), EUserBlockedByPlatform);
        
        // Calculate and distribute fees (platform version, no PoC for profiles)
        let (mut remaining_payment, fee_amount, creator_fee, platform_fee, treasury_fee) = distribute_reservation_fees_no_poc_with_platform(
            config,
            reservation_pool_object,
            amount,
            payment,
            treasury,
            platform,
            ctx
        );
        
        // Net amount after fees
        let net_amount = amount - fee_amount;
        
        // Check individual reservation limit (based on net amount)
        let max_individual_reservation = (config.profile_threshold * config.max_individual_reservation_bps) / 10000;
        let current_reservation = if (table::contains(&reservation_pool_object.reservations, reserver)) {
            *table::borrow(&reservation_pool_object.reservations, reserver)
        } else {
            0
        };
        assert!(current_reservation + net_amount <= max_individual_reservation, EExceededMaxHold);
        
        // Extract net reservation payment
        let reservation_payment = coin::split(&mut remaining_payment, net_amount, ctx);
        balance::join(&mut reservation_pool_object.mys_balance, coin::into_balance(reservation_payment));

        // Update reserver's balance in the pool (store net amount)
        if (table::contains(&reservation_pool_object.reservations, reserver)) {
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            assert!(*reservation_balance <= MAX_U64 - net_amount, EOverflow);
            *reservation_balance = *reservation_balance + net_amount;
        } else {
            // DoS protection: limit number of unique reservers per pool
            let current_reservers_count = vector::length(&reservation_pool_object.reservers);
            assert!(current_reservers_count < config.max_reservers_per_pool, ETooManyReservers);
            
            table::add(&mut reservation_pool_object.reservations, reserver, net_amount);
            // Add to reservers list for tracking
            vector::push_back(&mut reservation_pool_object.reservers, reserver);
        };

        // Update total reserved (with net amount)
        assert!(reservation_pool_object.info.total_reserved <= MAX_U64 - net_amount, EOverflow);
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved + net_amount;

        // Update registry
        if (table::contains(&registry.reservation_pools, profile_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, profile_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        } else {
            // Create registry entry if it doesn't exist
            let reservation_pool = ReservationPool {
                associated_id: profile_id,
                token_type: TOKEN_TYPE_PROFILE,
                owner: profile_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.profile_threshold,
                created_at: now,
            };
            table::add(&mut registry.reservation_pools, profile_id, reservation_pool);
        };
        
        // Check if threshold was just met
        let threshold_met = reservation_pool_object.info.total_reserved >= config.profile_threshold;
        let was_threshold_met = (reservation_pool_object.info.total_reserved - net_amount) >= config.profile_threshold;
        
        // Emit threshold met event if this reservation pushed us over the threshold
        if (threshold_met && !was_threshold_met) {
            event::emit(ThresholdMetEvent {
                associated_id: profile_id,
                token_type: TOKEN_TYPE_PROFILE,
                owner: profile_owner,
                total_reserved: reservation_pool_object.info.total_reserved,
                required_threshold: config.profile_threshold,
                timestamp: now,
            });
        };
        
        // Return excess payment
        if (coin::value(&remaining_payment) > 0) {
            transfer::public_transfer(remaining_payment, reserver);
        } else {
            coin::destroy_zero(remaining_payment);
        };
        
        // Emit reservation created event
        event::emit(ReservationCreatedEvent {
            associated_id: profile_id,
            token_type: TOKEN_TYPE_PROFILE,
            reserver,
            amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            threshold_met,
            reserved_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Withdraw MYS reservation from a post or profile
    /// Non-platform version: platform fees go to ecosystem treasury
    #[allow(lint(self_transfer))]
    public fun withdraw_reservation(
        registry: &mut TokenRegistry,
        _config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        _treasury: &EcosystemTreasury,
        amount: u64,
        ctx: &mut TxContext
    ) {
        let reserver = tx_context::sender(ctx);
        let associated_id = reservation_pool_object.info.associated_id;
        let now = tx_context::epoch(ctx);
        
        // Prevent withdrawals after conversion to token
        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        
        // Validate amount is positive
        assert!(amount > 0, EInsufficientFunds);
        
        // Verify reserver has a reservation
        assert!(table::contains(&reservation_pool_object.reservations, reserver), ENoTokensOwned);
        
        let current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
        
        // Model A: Fee only on deposit, so amount is net withdrawal amount (no fee on withdraw)
        // Ensure user has enough net reservation balance (we store net amounts)
        assert!(current_reservation >= amount, EInsufficientLiquidity);
        
        // Ensure pool has enough liquidity for net refund (pool contains net deposits)
        assert!(balance::value(&reservation_pool_object.mys_balance) >= amount, EInsufficientLiquidity);
        
        // Update reserver's balance (subtract net amount, since reservations store net)
        if (current_reservation == amount) {
            // Remove reserver completely
            table::remove(&mut reservation_pool_object.reservations, reserver);
            
            // Remove from reservers list
            let mut i = 0;
            let len = vector::length(&reservation_pool_object.reservers);
            while (i < len) {
                if (*vector::borrow(&reservation_pool_object.reservers, i) == reserver) {
                    vector::remove(&mut reservation_pool_object.reservers, i);
                    break
                };
                i = i + 1;
            };
        } else {
            // Reduce reservation amount by net amount (since we store net)
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            *reservation_balance = *reservation_balance - amount;
        };
        
        // Update total reserved (subtract net amount, since we track net)
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved - amount;
        
        // Update registry
        if (table::contains(&registry.reservation_pools, associated_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, associated_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        };
        
        // Transfer net amount to reserver (no fees on withdrawal in Model A)
        let refund_balance = balance::split(&mut reservation_pool_object.mys_balance, amount);
        let refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, reserver);
        
        // Emit reservation withdrawn event
        // Model A: No fees on withdrawal, so all fee fields are 0
        event::emit(ReservationWithdrawnEvent {
            associated_id,
            token_type: reservation_pool_object.info.token_type,
            reserver,
            amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            withdrawn_at: now,
            fee_amount: 0,
            creator_fee: 0,
            platform_fee: 0,
            treasury_fee: 0,
        });
    }

    /// Withdraw MYS reservation from a post or profile
    /// Platform version: platform fees go to platform treasury
    #[allow(lint(self_transfer))]
    public fun withdraw_reservation_with_platform(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        platform: &mut social_contracts::platform::Platform,
        block_list_registry: &BlockListRegistry,
        amount: u64,
        ctx: &mut TxContext
    ) {
        let reserver = tx_context::sender(ctx);
        let associated_id = reservation_pool_object.info.associated_id;
        let now = tx_context::epoch(ctx);
        
        // Prevent withdrawals after conversion to token
        assert!(!reservation_pool_object.converted, EReservationPoolConverted);
        
        // Validate amount is positive
        assert!(amount > 0, EInsufficientFunds);
        
        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, reserver), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, reserver), EUserBlockedByPlatform);
        
        // Verify reserver has a reservation
        assert!(table::contains(&reservation_pool_object.reservations, reserver), ENoTokensOwned);
        
        let current_reservation = *table::borrow(&reservation_pool_object.reservations, reserver);
        
        // amount is gross withdrawal amount; calculate fees to get net amount
        // Validate fees and calculate with overflow protection
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Net amount after fees (this is what we subtract from reservation tracking, since we store net)
        let net_amount = amount - fee_amount;
        
        // Ensure user has enough net reservation balance (we store net amounts)
        assert!(current_reservation >= net_amount, EInsufficientLiquidity);
        
        // Ensure pool has enough liquidity for gross refund + all fees
        assert!(balance::value(&reservation_pool_object.mys_balance) >= amount, EInsufficientLiquidity);
        
        // Update reserver's balance (subtract net amount, since reservations store net)
        if (current_reservation == net_amount) {
            // Remove reserver completely
            table::remove(&mut reservation_pool_object.reservations, reserver);
            
            // Remove from reservers list
            let mut i = 0;
            let len = vector::length(&reservation_pool_object.reservers);
            while (i < len) {
                if (*vector::borrow(&reservation_pool_object.reservers, i) == reserver) {
                    vector::remove(&mut reservation_pool_object.reservers, i);
                    break
                };
                i = i + 1;
            };
        } else {
            // Reduce reservation amount by net amount (since we store net)
            let reservation_balance = table::borrow_mut(&mut reservation_pool_object.reservations, reserver);
            *reservation_balance = *reservation_balance - net_amount;
        };
        
        // Update total reserved (subtract net amount, since we track net)
        reservation_pool_object.info.total_reserved = reservation_pool_object.info.total_reserved - net_amount;
        
        // Update registry
        if (table::contains(&registry.reservation_pools, associated_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, associated_id);
            registry_pool.total_reserved = reservation_pool_object.info.total_reserved;
        };
        
        // Distribute fees from pool balance (no PoC redirection on withdrawals)
        if (fee_amount > 0) {
            // Send creator fee directly to owner (no PoC redirection on withdrawals)
            if (creator_fee > 0) {
                let creator_fee_coin = coin::from_balance(balance::split(&mut reservation_pool_object.mys_balance, creator_fee), ctx);
                transfer::public_transfer(creator_fee_coin, reservation_pool_object.info.owner);
            };
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::from_balance(balance::split(&mut reservation_pool_object.mys_balance, platform_fee), ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, ctx);
                coin::destroy_zero(platform_fee_coin);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::from_balance(balance::split(&mut reservation_pool_object.mys_balance, treasury_fee), ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Transfer net refund to reserver
        let refund_balance = balance::split(&mut reservation_pool_object.mys_balance, net_amount);
        let refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, reserver);
        
        // Emit reservation withdrawn event with actual fee amounts
        event::emit(ReservationWithdrawnEvent {
            associated_id,
            token_type: reservation_pool_object.info.token_type,
            reserver,
            amount,
            total_reserved: reservation_pool_object.info.total_reserved,
            withdrawn_at: now,
            fee_amount,
            creator_fee,
            platform_fee,
            treasury_fee,
        });
    }

    /// Create a new reservation pool for a post
    public entry fun create_reservation_pool_for_post(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        post: &Post,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let caller = tx_context::sender(ctx);
        let associated_id = post::get_id_address(post);
        let owner = post::get_post_owner(post);
        
        // Verify caller is the actual post owner
        assert!(caller == owner, ENotAuthorized);
        
        // Verify post ID matches
        assert!(associated_id == post::get_id_address(post), EInvalidID);
        
        // Check if reservation pool already exists
        assert!(!table::contains(&registry.reservation_pools, associated_id), ETokenAlreadyExists);
        
        let now = tx_context::epoch(ctx);
        let required_threshold = config.post_threshold;
        
        // Create reservation pool info (without reservers vector - only in ReservationPoolObject)
        let reservation_pool = ReservationPool {
            associated_id,
            token_type: TOKEN_TYPE_POST,
            owner,
            total_reserved: 0,
            required_threshold,
            created_at: now,
        };
        
        // Create reservation pool object first (before moving reservation_pool)
        let reservation_pool_object = ReservationPoolObject {
            id: object::new(ctx),
            info: reservation_pool,
            mys_balance: balance::zero(),
            reservations: table::new(ctx),
            reservers: vector::empty(),
            converted: false,
            version: upgrade::current_version(),
        };
        
        // Add to registry (reconstruct ReservationPool from object's info since original was moved)
        let pool_info = ReservationPool {
            associated_id: reservation_pool_object.info.associated_id,
            token_type: reservation_pool_object.info.token_type,
            owner: reservation_pool_object.info.owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: reservation_pool_object.info.required_threshold,
            created_at: reservation_pool_object.info.created_at,
        };
        table::add(&mut registry.reservation_pools, associated_id, pool_info);
        
        let pool_object_id = object::uid_to_address(&reservation_pool_object.id);
        
        // Emit reservation pool created event
        event::emit(ReservationPoolCreatedEvent {
            associated_id,
            token_type: TOKEN_TYPE_POST,
            owner,
            required_threshold,
            pool_object_id,
            created_at: now,
        });
        
        transfer::share_object(reservation_pool_object);
    }

    /// Create a new reservation pool for a profile
    public entry fun create_reservation_pool_for_profile(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        profile: &Profile,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let caller = tx_context::sender(ctx);
        let associated_id = profile::get_id_address(profile);
        let owner = profile::owner(profile);
        
        // Verify caller is the actual profile owner
        assert!(caller == owner, ENotAuthorized);
        
        // Verify profile ID matches
        assert!(associated_id == profile::get_id_address(profile), EInvalidID);
        
        // Check if reservation pool already exists
        assert!(!table::contains(&registry.reservation_pools, associated_id), ETokenAlreadyExists);
        
        let now = tx_context::epoch(ctx);
        let required_threshold = config.profile_threshold;
        
        // Create reservation pool info (without reservers vector - only in ReservationPoolObject)
        let reservation_pool = ReservationPool {
            associated_id,
            token_type: TOKEN_TYPE_PROFILE,
            owner,
            total_reserved: 0,
            required_threshold,
            created_at: now,
        };
        
        // Create reservation pool object first (before moving reservation_pool)
        let reservation_pool_object = ReservationPoolObject {
            id: object::new(ctx),
            info: reservation_pool,
            mys_balance: balance::zero(),
            reservations: table::new(ctx),
            reservers: vector::empty(),
            converted: false,
            version: upgrade::current_version(),
        };
        
        // Add to registry (reconstruct ReservationPool from object's info since original was moved)
        let pool_info = ReservationPool {
            associated_id: reservation_pool_object.info.associated_id,
            token_type: reservation_pool_object.info.token_type,
            owner: reservation_pool_object.info.owner,
            total_reserved: reservation_pool_object.info.total_reserved,
            required_threshold: reservation_pool_object.info.required_threshold,
            created_at: reservation_pool_object.info.created_at,
        };
        table::add(&mut registry.reservation_pools, associated_id, pool_info);
        
        let pool_object_id = object::uid_to_address(&reservation_pool_object.id);
        
        // Emit reservation pool created event
        event::emit(ReservationPoolCreatedEvent {
            associated_id,
            token_type: TOKEN_TYPE_PROFILE,
            owner,
            required_threshold,
            pool_object_id,
            created_at: now,
        });
        
        transfer::share_object(reservation_pool_object);
    }

    /// Check if reservation threshold is met for auction creation
    public fun can_create_auction(
        registry: &TokenRegistry,
        associated_id: address
    ): bool {
        if (!table::contains(&registry.reservation_pools, associated_id)) {
            return false
        };
        
        let reservation_pool = table::borrow(&registry.reservation_pools, associated_id);
        reservation_pool.total_reserved >= reservation_pool.required_threshold
    }
    
    /// Create a social proof token directly from a reservation pool once threshold is met
    /// This replaces the auction system - only the post/profile owner can call this
    public entry fun create_social_proof_token(
        registry: &mut TokenRegistry,
        config: &SocialProofTokensConfig,
        reservation_pool_object: &mut ReservationPoolObject,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let caller = tx_context::sender(ctx);
        let associated_id = reservation_pool_object.info.associated_id;
        
        // Verify caller is the owner of the post/profile
        assert!(caller == reservation_pool_object.info.owner, ENotAuthorized);
        
        // Check if reservation threshold has been met
        assert!(can_create_auction(registry, associated_id), EViralThresholdNotMet);
        
        // Verify token has not already been created
        assert!(!table::contains(&registry.tokens, associated_id), ETokenAlreadyExists);
        
        // Calculate initial token supply based on total reserved amount
        // Use the same scaling formula as the old auction system
        let total_reserved = reservation_pool_object.info.total_reserved;
        let sqrt_reserved = math::sqrt(total_reserved);
        let fourth_root_reserved = math::sqrt(sqrt_reserved); // fourth root: sqrt(sqrt(x)) = x^(1/4)
        // Overflow protection: check before multiplication
        assert!(sqrt_reserved == 0 || sqrt_reserved <= MAX_U64 / fourth_root_reserved, EOverflow);
        let mut scale_factor = sqrt_reserved * fourth_root_reserved; // reserved^0.75
        
        // Divide the scale factor to make each token worth more than 1 MYS
        scale_factor = scale_factor / 1000;
        
        // Apply different base multipliers for profile vs post tokens
        let mut initial_token_supply = if (reservation_pool_object.info.token_type == TOKEN_TYPE_PROFILE) {
            // Profile tokens - lower supply (more valuable per token)
            scale_factor
        } else {
            // Post tokens - higher supply (more collectible)
            scale_factor * 10
        };
        
        // Ensure we have at least 1 token
        if (initial_token_supply == 0) {
            initial_token_supply = 1;
        };
        
        // Create token info
        let token_info = TokenInfo {
            id: @0x0, // Temporary, will be updated
            token_type: reservation_pool_object.info.token_type,
            owner: reservation_pool_object.info.owner,
            associated_id,
            symbol: if (reservation_pool_object.info.token_type == TOKEN_TYPE_PROFILE) {
                string::utf8(b"PUSER")
            } else {
                string::utf8(b"PPOST")
            },
            name: if (reservation_pool_object.info.token_type == TOKEN_TYPE_PROFILE) {
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
        
        // Update token info with actual pool address
        let mut updated_token_info = token_info;
        updated_token_info.id = pool_address;
        
        // Create a copy of token info for the pool (since TokenInfo doesn't have copy, we'll reconstruct)
        let pool_token_info = TokenInfo {
            id: updated_token_info.id,
            token_type: updated_token_info.token_type,
            owner: updated_token_info.owner,
            associated_id: updated_token_info.associated_id,
            symbol: updated_token_info.symbol,
            name: updated_token_info.name,
            circulating_supply: updated_token_info.circulating_supply,
            base_price: updated_token_info.base_price,
            quadratic_coefficient: updated_token_info.quadratic_coefficient,
            created_at: updated_token_info.created_at,
        };
        
        let mut token_pool = TokenPool {
            id: pool_id,
            info: pool_token_info,
            mys_balance: balance::zero(),
            holders: table::new(ctx),
            poc_redirect_to: option::none(),
            poc_redirect_percentage: option::none(),
            version: upgrade::current_version(),
        };
        
        // Distribute tokens to reservers proportionally
        // Limit number of reservers to prevent DoS via gas exhaustion
        let reservers = &reservation_pool_object.reservers;
        let num_reservers = vector::length(reservers);
        assert!(num_reservers <= config.max_reservers_per_pool, ETooManyReservers);
        
        let mut distributed_total = 0;
        let mut i = 0;
        while (i < num_reservers) {
            let reserver = *vector::borrow(reservers, i);
            let reservation_amount = *table::borrow(&reservation_pool_object.reservations, reserver);
            
            // Calculate token amount based on reserver's proportion of total reservation
            // Overflow protection: check before multiplication
            assert!(reservation_amount <= MAX_U64 / initial_token_supply, EOverflow);
            let token_amount = (reservation_amount * initial_token_supply) / total_reserved;
            
            if (token_amount > 0) {
                // Update holder's balance in the pool
                // Handle duplicate reservers: if already exists, add to existing balance
                if (table::contains(&token_pool.holders, reserver)) {
                    let existing_balance = table::borrow_mut(&mut token_pool.holders, reserver);
                    assert!(*existing_balance <= MAX_U64 - token_amount, EOverflow);
                    *existing_balance = *existing_balance + token_amount;
                } else {
                    table::add(&mut token_pool.holders, reserver, token_amount);
                };
                
                // Track distributed tokens to ensure accurate circulating supply
                assert!(distributed_total <= MAX_U64 - token_amount, EOverflow);
                distributed_total = distributed_total + token_amount;
                
                // Create social token for the reserver
                let social_token = SocialToken {
                    id: object::new(ctx),
                    pool_id: pool_address,
                    token_type: reservation_pool_object.info.token_type,
                    amount: token_amount,
                };
                
                // Transfer social token to reserver
                transfer::public_transfer(social_token, reserver);
            };
            
            i = i + 1;
        };
        
        // Handle rounding remainder: allocate any undistributed tokens to the owner
        let remainder = initial_token_supply - distributed_total;
        if (remainder > 0) {
            // Allocate remainder to the owner
            if (table::contains(&token_pool.holders, reservation_pool_object.info.owner)) {
                let owner_balance = table::borrow_mut(&mut token_pool.holders, reservation_pool_object.info.owner);
                assert!(*owner_balance <= MAX_U64 - remainder, EOverflow);
                *owner_balance = *owner_balance + remainder;
            } else {
                table::add(&mut token_pool.holders, reservation_pool_object.info.owner, remainder);
            };
            distributed_total = distributed_total + remainder;
        };
        
        // Update circulating supply to match actually distributed tokens
        token_pool.info.circulating_supply = distributed_total;
        updated_token_info.circulating_supply = distributed_total;
        
        // Transfer all reserved MYS to the token pool as initial liquidity
        balance::join(&mut token_pool.mys_balance, balance::withdraw_all(&mut reservation_pool_object.mys_balance));
        
        // Mark reservation pool as converted and clear total reserved
        reservation_pool_object.converted = true;
        reservation_pool_object.info.total_reserved = 0;
        
        // Update registry reservation pool entry to reflect conversion
        if (table::contains(&registry.reservation_pools, associated_id)) {
            let registry_pool = table::borrow_mut(&mut registry.reservation_pools, associated_id);
            registry_pool.total_reserved = 0;
        };
        
        // Add to registry (use updated_token_info which has the correct circulating_supply)
        table::add(&mut registry.tokens, associated_id, updated_token_info);
        
        // Emit token created event (use token_pool.info which has the final state)
        event::emit(TokenPoolCreatedEvent {
            id: pool_address,
            token_type: token_pool.info.token_type,
            owner: token_pool.info.owner,
            associated_id: token_pool.info.associated_id,
            symbol: token_pool.info.symbol,
            name: token_pool.info.name,
            base_price: token_pool.info.base_price,
            quadratic_coefficient: token_pool.info.quadratic_coefficient,
        });
        
        // Share the token pool
        transfer::share_object(token_pool);
    }

    // === PoC Revenue Redirection Functions ===

    /// Update PoC redirection data for a token pool (called by PoC system)
    /// This function copies PoC data from a post into the corresponding token pool
    public entry fun update_token_poc_data(
        pool: &mut TokenPool,
        post: &Post,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Verify this is a post token pool
        assert!(pool.info.token_type == TOKEN_TYPE_POST, EInvalidTokenType);
        
        // Verify the post matches the token pool
        let post_id = post::get_id_address(post);
        assert!(post_id == pool.info.associated_id, EInvalidID);
        
        // Verify caller is authorized (post owner)
        let caller = tx_context::sender(ctx);
        assert!(caller == post::get_post_owner(post), ENotAuthorized);
        
        // Copy PoC data from post to pool
        let redirect_to = if (option::is_some(post::get_revenue_redirect_to(post))) {
            option::some(*option::borrow(post::get_revenue_redirect_to(post)))
        } else {
            option::none()
        };
        
        let redirect_percentage = if (option::is_some(post::get_revenue_redirect_percentage(post))) {
            let percentage = *option::borrow(post::get_revenue_redirect_percentage(post));
            // Validate PoC redirect percentage is in valid range (0-100 percent)
            assert!(percentage <= 100, EInvalidFeeConfig);
            option::some(percentage)
        } else {
            option::none()
        };
        
        pool.poc_redirect_to = redirect_to;
        pool.poc_redirect_percentage = redirect_percentage;
        
        // Emit PoC redirection updated event
        event::emit(PocRedirectionUpdatedEvent {
            pool_id: object::uid_to_address(&pool.id),
            post_id,
            redirect_to,
            redirect_percentage,
            updated_by: caller,
            timestamp: tx_context::epoch(ctx),
        });
    }

    /// Calculate PoC revenue split - shared utility for consistent logic
    fun calculate_poc_split(amount: u64, redirect_percentage: u64): (u64, u64) {
        // Validate redirect percentage to prevent underflow
        assert!(redirect_percentage <= 100, EInvalidFeeConfig);
        let redirected_amount = (amount * redirect_percentage) / 100;
        let remaining_amount = amount - redirected_amount;
        (redirected_amount, remaining_amount)
    }

    /// Apply PoC redirection to creator fees with consolidated logic
    fun apply_token_poc_redirection(
        pool: &TokenPool,
        amount: u64,
        _ctx: &mut TxContext  
    ): (u64, u64) {
        if (has_poc_redirection(pool)) {
            let redirect_percentage = *option::borrow(&pool.poc_redirect_percentage);
            // Use shared utility function for consistent calculation
            calculate_poc_split(amount, redirect_percentage)
        } else {
            (0, amount)
        }
    }

    /// Distribute creator fees with automatic PoC redirection  
    fun distribute_creator_fee(
        pool: &TokenPool,
        creator_fee_amount: u64,
        creator_fee_coin: &mut Coin<MYS>,
        ctx: &mut TxContext
    ) {
        if (creator_fee_amount == 0) {
            return
        };

        let (redirected_amount, _remaining_amount) = apply_token_poc_redirection(pool, creator_fee_amount, ctx);
        let mut fee_coin = coin::split(creator_fee_coin, creator_fee_amount, ctx);
        
        if (redirected_amount > 0) {
            // Split the fee: redirected portion goes to original creator, remainder to post owner
            let redirected_fee = coin::split(&mut fee_coin, redirected_amount, ctx);
            let redirect_to = *option::borrow(&pool.poc_redirect_to);
            transfer::public_transfer(redirected_fee, redirect_to);
            
            // Send remainder to current post owner
            if (coin::value(&fee_coin) > 0) {
                transfer::public_transfer(fee_coin, pool.info.owner);
            } else {
                coin::destroy_zero(fee_coin);
            };
        } else {
            // No redirection - send full amount to current post owner
            transfer::public_transfer(fee_coin, pool.info.owner);
        };
    }

    /// Distribute creator fees from pool balance with PoC redirection support
    fun distribute_creator_fee_from_pool(
        pool: &mut TokenPool,
        creator_fee: u64,
        ctx: &mut TxContext
    ) {
        if (creator_fee == 0) {
            return
        };

        let (redirected_amount, _remaining_amount) = apply_token_poc_redirection(pool, creator_fee, ctx);
        let mut fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, creator_fee), ctx);
        
        if (redirected_amount > 0) {
            // Split the fee: redirected portion goes to original creator, remainder to post owner
            let redirected_fee = coin::split(&mut fee_coin, redirected_amount, ctx);
            let redirect_to = *option::borrow(&pool.poc_redirect_to);
            transfer::public_transfer(redirected_fee, redirect_to);
            
            // Send remainder to current post owner
            if (coin::value(&fee_coin) > 0) {
                transfer::public_transfer(fee_coin, pool.info.owner);
            } else {
                coin::destroy_zero(fee_coin);
            };
        } else {
            // No redirection - send full amount to current post owner
            transfer::public_transfer(fee_coin, pool.info.owner);
        };
    }

    // === Reservation Fee Distribution Functions ===

    /// Apply PoC redirection from post (reuses calculate_poc_split utility)
    public(package) fun apply_post_poc_redirection(
        post: &Post,
        amount: u64
    ): (u64, u64) {
        if (option::is_some(post::get_revenue_redirect_to(post)) && option::is_some(post::get_revenue_redirect_percentage(post))) {
            let redirect_percentage = *option::borrow(post::get_revenue_redirect_percentage(post));
            // Validate at use-site to prevent underflow (Post may contain invalid data)
            assert!(redirect_percentage <= 100, EInvalidFeeConfig);
            calculate_poc_split(amount, redirect_percentage)
        } else {
            (0, amount)
        }
    }

    /// Distribute creator fees with PoC redirection from post (reuses existing pattern)
    /// This follows the same logic as distribute_creator_fee but works with Post instead of TokenPool
    public(package) fun distribute_reservation_creator_fee(
        reservation_pool: &ReservationPoolObject,
        post: &Post,
        creator_fee_amount: u64,
        creator_fee_coin: &mut Coin<MYS>,
        ctx: &mut TxContext
    ) {
        if (creator_fee_amount == 0) {
            return
        };

        let (redirected_amount, _remaining_amount) = apply_post_poc_redirection(post, creator_fee_amount);
        let mut fee_coin = coin::split(creator_fee_coin, creator_fee_amount, ctx);
        
        if (redirected_amount > 0) {
            // Split the fee: redirected portion goes to original creator, remainder to post owner
            let redirected_fee = coin::split(&mut fee_coin, redirected_amount, ctx);
            let redirect_to = *option::borrow(post::get_revenue_redirect_to(post));
            transfer::public_transfer(redirected_fee, redirect_to);
            
            // Send remainder to current post owner
            if (coin::value(&fee_coin) > 0) {
                transfer::public_transfer(fee_coin, reservation_pool.info.owner);
            } else {
                coin::destroy_zero(fee_coin);
            };
        } else {
            // No redirection - send full amount to current post owner
            transfer::public_transfer(fee_coin, reservation_pool.info.owner);
        };
    }

    /// Distribute creator fees without PoC (for profile reservations)
    public(package) fun distribute_reservation_creator_fee_no_poc(
        reservation_pool: &ReservationPoolObject,
        creator_fee_amount: u64,
        creator_fee_coin: &mut Coin<MYS>,
        ctx: &mut TxContext
    ) {
        if (creator_fee_amount == 0) {
            return
        };

        let fee_coin = coin::split(creator_fee_coin, creator_fee_amount, ctx);
        transfer::public_transfer(fee_coin, reservation_pool.info.owner);
    }

    /// Calculate and distribute all reservation fees (for post reservations with PoC)
    /// Non-platform version: routes platform fees to ecosystem treasury
    public(package) fun distribute_reservation_fees_with_post(
        config: &SocialProofTokensConfig,
        reservation_pool: &ReservationPoolObject,
        post: &Post,
        amount: u64,
        mut payment: Coin<MYS>,
        treasury: &EcosystemTreasury,
        ctx: &mut TxContext
    ): (Coin<MYS>, u64, u64, u64, u64) {
        // Validate fees and calculate with overflow protection
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Distribute fees (same pattern as trading fees)
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_reservation_creator_fee(reservation_pool, post, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to ecosystem treasury (no platform involved)
            if (platform_fee > 0) {
                let platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                transfer::public_transfer(platform_fee_coin, profile::get_treasury_address(treasury));
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Return remaining payment and fee amounts
        (payment, fee_amount, creator_fee, platform_fee, treasury_fee)
    }

    /// Calculate and distribute all reservation fees (for post reservations with PoC)
    /// Platform version: routes platform fees to platform treasury
    public(package) fun distribute_reservation_fees_with_post_and_platform(
        config: &SocialProofTokensConfig,
        reservation_pool: &ReservationPoolObject,
        post: &Post,
        amount: u64,
        mut payment: Coin<MYS>,
        treasury: &EcosystemTreasury,
        platform: &mut social_contracts::platform::Platform,
        ctx: &mut TxContext
    ): (Coin<MYS>, u64, u64, u64, u64) {
        // Validate fees and calculate with overflow protection
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Distribute fees (same pattern as trading fees)
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_reservation_creator_fee(reservation_pool, post, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, ctx);
                coin::destroy_zero(platform_fee_coin);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Return remaining payment and fee amounts
        (payment, fee_amount, creator_fee, platform_fee, treasury_fee)
    }

    /// Calculate and distribute all reservation fees (for profile reservations without PoC)
    /// Non-platform version: routes platform fees to ecosystem treasury
    public(package) fun distribute_reservation_fees_no_poc(
        config: &SocialProofTokensConfig,
        reservation_pool: &ReservationPoolObject,
        amount: u64,
        mut payment: Coin<MYS>,
        treasury: &EcosystemTreasury,
        ctx: &mut TxContext
    ): (Coin<MYS>, u64, u64, u64, u64) {
        // Validate fees and calculate with overflow protection
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Distribute fees (same pattern as trading fees)
        if (fee_amount > 0) {
            // Send creator fee without PoC redirection
            if (creator_fee > 0) {
                distribute_reservation_creator_fee_no_poc(reservation_pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to ecosystem treasury (no platform involved)
            if (platform_fee > 0) {
                let platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                transfer::public_transfer(platform_fee_coin, profile::get_treasury_address(treasury));
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Return remaining payment and fee amounts
        (payment, fee_amount, creator_fee, platform_fee, treasury_fee)
    }

    /// Calculate and distribute all reservation fees (for profile reservations without PoC)
    /// Platform version: routes platform fees to platform treasury
    public(package) fun distribute_reservation_fees_no_poc_with_platform(
        config: &SocialProofTokensConfig,
        reservation_pool: &ReservationPoolObject,
        amount: u64,
        mut payment: Coin<MYS>,
        treasury: &EcosystemTreasury,
        platform: &mut social_contracts::platform::Platform,
        ctx: &mut TxContext
    ): (Coin<MYS>, u64, u64, u64, u64) {
        // Validate fees and calculate with overflow protection
        validate_reservation_fees(config);
        let reservation_total_fee_bps = calculate_reservation_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(amount, reservation_total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.reservation_creator_fee_bps, reservation_total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.reservation_platform_fee_bps, reservation_total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Distribute fees (same pattern as trading fees)
        if (fee_amount > 0) {
            // Send creator fee without PoC redirection
            if (creator_fee > 0) {
                distribute_reservation_creator_fee_no_poc(reservation_pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, ctx);
                coin::destroy_zero(platform_fee_coin);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
            };
        };
        
        // Return remaining payment and fee amounts
        (payment, fee_amount, creator_fee, platform_fee, treasury_fee)
    }

    // === Trading Functions ===

    /// Buy tokens from the pool - first purchase
    /// Non-platform version: platform fees go to ecosystem treasury
    /// This function handles buying tokens for first-time buyers of a specific token
    #[allow(lint(self_transfer))]
    public fun buy_tokens(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        profile_registry: &UsernameRegistry,
        block_list_registry: &BlockListRegistry,
        mut payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let buyer = tx_context::sender(ctx);
        
        // Look up the buyer's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, buyer);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);
        
        // Prevent self-trading for token owners
        assert!(buyer != pool.info.owner, ESelfTrading);
        
        // Check if token owner is blocked by the buyer
        assert!(!block_list::is_blocked(block_list_registry, buyer, pool.info.owner), EBlockedUser);
        
        // Calculate the price for the tokens based on quadratic curve
        let (price, _) = calculate_buy_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );
        
        // Ensure buyer has enough funds
        assert!(coin::value(&payment) >= price, EInsufficientFunds);
        
        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(price, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate the net amount to the liquidity pool
        let net_amount = price - fee_amount;
        
        // Extract payment and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee(pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to ecosystem treasury (no platform involved)
            if (platform_fee > 0) {
                let platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                transfer::public_transfer(platform_fee_coin, profile::get_treasury_address(treasury));
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
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
        
        // Update holder's balance with overflow protection
        // First check addition overflow
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        let new_supply = pool.info.circulating_supply + amount;
        
        // Then check multiplication overflow for max_hold calculation
        assert!(new_supply == 0 || new_supply <= MAX_U64 / config.max_hold_percent_bps, EOverflow);
        let max_hold = (new_supply * config.max_hold_percent_bps) / 10000;
        let current_hold = if (table::contains(&pool.holders, buyer)) {
            *table::borrow(&pool.holders, buyer)
        } else {
            0
        };
        
        // Check max holding limit with overflow protection
        assert!(current_hold <= MAX_U64 - amount, EOverflow);
        assert!(current_hold + amount <= max_hold, EExceededMaxHold);
        
        // Check that this is the first purchase (user must not already own tokens)
        assert!(current_hold == 0, EAlreadyOwnsTokens);
        
        // Update holder's balance
        table::add(&mut pool.holders, buyer, amount);

        // Update circulating supply

        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
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

    /// Buy tokens from the pool - first purchase
    /// Platform version: platform fees go to platform treasury, includes platform validation
    /// This function handles buying tokens for first-time buyers of a specific token
    #[allow(lint(self_transfer))]
    public fun buy_tokens_with_platform(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        profile_registry: &UsernameRegistry,
        block_list_registry: &BlockListRegistry,
        platform: &mut social_contracts::platform::Platform,
        mut payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let buyer = tx_context::sender(ctx);
        
        // Look up the buyer's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, buyer);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);
        
        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, buyer), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, buyer), EUserBlockedByPlatform);
        
        // Prevent self-trading for token owners
        assert!(buyer != pool.info.owner, ESelfTrading);
        
        // Check if token owner is blocked by the buyer
        assert!(!block_list::is_blocked(block_list_registry, buyer, pool.info.owner), EBlockedUser);
        
        // Calculate the price for the tokens based on quadratic curve
        let (price, _) = calculate_buy_price(
            pool.info.base_price,
            pool.info.quadratic_coefficient,
            pool.info.circulating_supply,
            amount
        );
        
        // Ensure buyer has enough funds
        assert!(coin::value(&payment) >= price, EInsufficientFunds);
        
        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(price, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate the net amount to the liquidity pool
        let net_amount = price - fee_amount;
        
        // Extract payment and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee(pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, ctx);
                coin::destroy_zero(platform_fee_coin);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
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
        
        // Update holder's balance with overflow protection
        // First check addition overflow
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        let new_supply = pool.info.circulating_supply + amount;
        
        // Then check multiplication overflow for max_hold calculation
        assert!(new_supply == 0 || new_supply <= MAX_U64 / config.max_hold_percent_bps, EOverflow);
        let max_hold = (new_supply * config.max_hold_percent_bps) / 10000;
        let current_hold = if (table::contains(&pool.holders, buyer)) {
            *table::borrow(&pool.holders, buyer)
        } else {
            0
        };
        
        // Check max holding limit with overflow protection
        assert!(current_hold <= MAX_U64 - amount, EOverflow);
        assert!(current_hold + amount <= max_hold, EExceededMaxHold);
        
        // Check that this is the first purchase (user must not already own tokens)
        assert!(current_hold == 0, EAlreadyOwnsTokens);
        
        // Update holder's balance
        table::add(&mut pool.holders, buyer, amount);

        // Update circulating supply
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
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
    /// Non-platform version: platform fees go to ecosystem treasury
    /// This function allows users to add to their existing token holdings using MYS Coin
    #[allow(lint(self_transfer))]
    public fun buy_more_tokens(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        profile_registry: &UsernameRegistry,
        block_list_registry: &BlockListRegistry,
        mut payment: Coin<MYS>,
        amount: u64,
        social_token: &mut SocialToken,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let buyer = tx_context::sender(ctx);
        
        // Look up the buyer's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, buyer);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);
        
        // Prevent self-trading for token owners
        assert!(buyer != pool.info.owner, ESelfTrading);
        
        // Check if token owner is blocked by the buyer
        assert!(!block_list::is_blocked(block_list_registry, buyer, pool.info.owner), EBlockedUser);
        
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
        
        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(price, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate the net amount to the liquidity pool
        let net_amount = price - fee_amount;
        
        // Extract payment and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee(pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to ecosystem treasury (no platform involved)
            if (platform_fee > 0) {
                let platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                transfer::public_transfer(platform_fee_coin, profile::get_treasury_address(treasury));
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
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
        
        // Update holder's balance with overflow protection
        // First check addition overflow
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        let new_supply = pool.info.circulating_supply + amount;
        
        // Then check multiplication overflow for max_hold calculation
        assert!(new_supply == 0 || new_supply <= MAX_U64 / config.max_hold_percent_bps, EOverflow);
        let max_hold = (new_supply * config.max_hold_percent_bps) / 10000;
        let current_hold = if (table::contains(&pool.holders, buyer)) {
            *table::borrow(&pool.holders, buyer)
        } else {
            0
        };
        
        // Check max holding limit with overflow protection
        assert!(current_hold <= MAX_U64 - amount, EOverflow);
        assert!(current_hold + amount <= max_hold, EExceededMaxHold);
        
        // Update holder's balance
        if (table::contains(&pool.holders, buyer)) {
            let holder_balance = table::borrow_mut(&mut pool.holders, buyer);
            assert!(*holder_balance <= MAX_U64 - amount, EOverflow);
            *holder_balance = *holder_balance + amount;
        } else {
            table::add(&mut pool.holders, buyer, amount);
        };

        // Update circulating supply
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        pool.info.circulating_supply = pool.info.circulating_supply + amount;

        // Update the user's social token

        assert!(social_token.amount <= MAX_U64 - amount, EOverflow);
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

    /// Buy more tokens when you already have a social token
    /// Platform version: platform fees go to platform treasury, includes platform validation
    /// This function allows users to add to their existing token holdings using MYS Coin
    #[allow(lint(self_transfer))]
    public fun buy_more_tokens_with_platform(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        profile_registry: &UsernameRegistry,
        block_list_registry: &BlockListRegistry,
        platform: &mut social_contracts::platform::Platform,
        mut payment: Coin<MYS>,
        amount: u64,
        social_token: &mut SocialToken,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let buyer = tx_context::sender(ctx);
        
        // Look up the buyer's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, buyer);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);
        
        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, buyer), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, buyer), EUserBlockedByPlatform);
        
        // Prevent self-trading for token owners
        assert!(buyer != pool.info.owner, ESelfTrading);
        
        // Check if token owner is blocked by the buyer
        assert!(!block_list::is_blocked(block_list_registry, buyer, pool.info.owner), EBlockedUser);
        
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
        
        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(price, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate the net amount to the liquidity pool
        let net_amount = price - fee_amount;
        
        // Extract payment and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee(pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, ctx);
                coin::destroy_zero(platform_fee_coin);
            };
            
            // Send treasury fee
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::split(&mut payment, treasury_fee, ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
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
        
        // Update holder's balance with overflow protection
        // First check addition overflow
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        let new_supply = pool.info.circulating_supply + amount;
        
        // Then check multiplication overflow for max_hold calculation
        assert!(new_supply == 0 || new_supply <= MAX_U64 / config.max_hold_percent_bps, EOverflow);
        let max_hold = (new_supply * config.max_hold_percent_bps) / 10000;
        let current_hold = if (table::contains(&pool.holders, buyer)) {
            *table::borrow(&pool.holders, buyer)
        } else {
            0
        };
        
        // Check max holding limit with overflow protection
        assert!(current_hold <= MAX_U64 - amount, EOverflow);
        assert!(current_hold + amount <= max_hold, EExceededMaxHold);
        
        // Update holder's balance
        if (table::contains(&pool.holders, buyer)) {
            let holder_balance = table::borrow_mut(&mut pool.holders, buyer);
            assert!(*holder_balance <= MAX_U64 - amount, EOverflow);
            *holder_balance = *holder_balance + amount;
        } else {
            table::add(&mut pool.holders, buyer, amount);
        };

        // Update circulating supply
        assert!(pool.info.circulating_supply <= MAX_U64 - amount, EOverflow);
        pool.info.circulating_supply = pool.info.circulating_supply + amount;

        // Update the user's social token
        assert!(social_token.amount <= MAX_U64 - amount, EOverflow);
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
    /// Non-platform version: platform fees go to ecosystem treasury
    #[allow(lint(self_transfer))]
    public fun sell_tokens(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        profile_registry: &UsernameRegistry,
        _block_list_registry: &BlockListRegistry,
        social_token: &mut SocialToken,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let seller = tx_context::sender(ctx);
        let pool_id = object::uid_to_address(&pool.id);
        
        // Look up the seller's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, seller);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);
        
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
        
        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(refund_amount, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate net refund
        let net_refund = refund_amount - fee_amount;
        
        // Ensure pool has enough liquidity for refund + all fees
        assert!(balance::value(&pool.mys_balance) >= refund_amount, EInsufficientLiquidity);
        
        // Verify seller has tokens in the pool
        assert!(table::contains(&pool.holders, seller), ENoTokensOwned);
        
        // Update holder balance
        let holder_balance = table::borrow_mut(&mut pool.holders, seller);
        if (*holder_balance == amount) {
            // Remove holder completely if selling all tokens
            table::remove(&mut pool.holders, seller);
        } else {
            // Reduce balance
            *holder_balance = *holder_balance - amount;
        };
        
        // Update user's social token
        social_token.amount = social_token.amount - amount;
        
        // Update circulating supply
        pool.info.circulating_supply = pool.info.circulating_supply - amount;
        
        // Extract net refund from pool
        let refund_balance = balance::split(&mut pool.mys_balance, net_refund);
        
        // Process and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send fee to creator with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee_from_pool(pool, creator_fee, ctx);
            };
            
            // Send platform fee to ecosystem treasury (no platform involved)
            if (platform_fee > 0) {
                let platform_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, platform_fee), ctx);
                transfer::public_transfer(platform_fee_coin, profile::get_treasury_address(treasury));
            };
            
            // Send fee to treasury
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, treasury_fee), ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
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

    /// Sell tokens back to the pool
    /// Platform version: platform fees go to platform treasury, includes platform validation
    #[allow(lint(self_transfer))]
    public fun sell_tokens_with_platform(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &SocialProofTokensConfig,
        treasury: &EcosystemTreasury,
        platform_registry: &PlatformRegistry,
        profile_registry: &UsernameRegistry,
        block_list_registry: &BlockListRegistry,
        platform: &mut social_contracts::platform::Platform,
        social_token: &mut SocialToken,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(pool.version == upgrade::current_version(), EWrongVersion);
        
        // Check if trading is halted
        assert!(config.trading_enabled, ETradingHalted);
        
        let seller = tx_context::sender(ctx);
        let pool_id = object::uid_to_address(&pool.id);
        
        // Look up the seller's profile ID
        let profile_id_option = profile::lookup_profile_by_owner(profile_registry, seller);
        assert!(option::is_some(&profile_id_option), ENotAuthorized);
        
        // Platform validation
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), ENotAuthorized);
        assert!(platform::has_joined_platform(platform, seller), EUserNotJoinedPlatform);
        assert!(!block_list::is_blocked(block_list_registry, platform_id, seller), EUserBlockedByPlatform);
        
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
        
        // Validate fees and calculate with overflow protection
        validate_trading_fees(config);
        let total_fee_bps = calculate_total_fee_bps(config);
        let fee_amount = calculate_fee_amount_safe(refund_amount, total_fee_bps);
        let creator_fee = calculate_component_fee_safe(fee_amount, config.trading_creator_fee_bps, total_fee_bps);
        let platform_fee = calculate_component_fee_safe(fee_amount, config.trading_platform_fee_bps, total_fee_bps);
        let treasury_fee = fee_amount - creator_fee - platform_fee;
        
        // Calculate net refund
        let net_refund = refund_amount - fee_amount;
        
        // Ensure pool has enough liquidity for refund + all fees
        assert!(balance::value(&pool.mys_balance) >= refund_amount, EInsufficientLiquidity);
        
        // Verify seller has tokens in the pool
        assert!(table::contains(&pool.holders, seller), ENoTokensOwned);
        
        // Update holder balance
        let holder_balance = table::borrow_mut(&mut pool.holders, seller);
        if (*holder_balance == amount) {
            // Remove holder completely if selling all tokens
            table::remove(&mut pool.holders, seller);
        } else {
            // Reduce balance
            *holder_balance = *holder_balance - amount;
        };
        
        // Update user's social token
        social_token.amount = social_token.amount - amount;
        
        // Update circulating supply
        pool.info.circulating_supply = pool.info.circulating_supply - amount;
        
        // Extract net refund from pool
        let refund_balance = balance::split(&mut pool.mys_balance, net_refund);
        
        // Process and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send fee to creator with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee_from_pool(pool, creator_fee, ctx);
            };
            
            // Send platform fee to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, platform_fee), ctx);
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, ctx);
                coin::destroy_zero(platform_fee_coin);
            };
            
            // Send fee to treasury
            if (treasury_fee > 0) {
                let treasury_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, treasury_fee), ctx);
                transfer::public_transfer(treasury_fee_coin, profile::get_treasury_address(treasury));
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
        // Overflow protection: check before squaring
        assert!(supply == 0 || supply <= MAX_U64 / supply, EOverflow);
        let squared_supply = supply * supply;
        
        // Overflow protection: check before multiplying by coefficient
        assert!(squared_supply == 0 || squared_supply <= MAX_U64 / quadratic_coefficient, EOverflow);
        let product = quadratic_coefficient * squared_supply / 10000;
        
        // Overflow protection: check before adding base_price
        assert!(product <= MAX_U64 - base_price, EOverflow);
        base_price + product
    }

    /// Calculate price to buy a specific amount of tokens using closed-form sum
    /// Price = base_price + (quadratic_coefficient * supply^2)
    /// Sum from i=current_supply to current_supply+amount-1 of price(i)
    /// = amount * base_price + (quadratic_coefficient / 10000) * sum(i^2)
    /// where sum(i^2) from n to n+k-1 = sum_squares(n+k-1) - sum_squares(n-1)
    /// Returns (total price, average price per token)
    public fun calculate_buy_price(
        base_price: u64,
        quadratic_coefficient: u64,
        current_supply: u64,
        amount: u64
    ): (u64, u64) {
        if (amount == 0) {
            return (0, 0)
        };
        
        // Base price component
        assert!(amount <= MAX_U64 / base_price, EOverflow);
        let base_component = amount * base_price;
        
        // Sum of squares: sum(i^2) from current_supply to current_supply+amount-1
        let end_supply = current_supply + amount - 1;
        let start_supply_minus_one = if (current_supply == 0) { 0 } else { current_supply - 1 };
        
        // Calculate sum_squares(end_supply) - sum_squares(start_supply_minus_one)
        let sum_squares_end = calculate_sum_squares(end_supply);
        let sum_squares_start = calculate_sum_squares(start_supply_minus_one);
        
        // Overflow protection
        assert!(sum_squares_end >= sum_squares_start, EOverflow);
        let sum_squares_range = sum_squares_end - sum_squares_start;
        
        // Multiply by coefficient and divide by 10000
        assert!(sum_squares_range <= MAX_U64 / quadratic_coefficient, EOverflow);
        let quadratic_component = (sum_squares_range * quadratic_coefficient) / 10000;
        
        // Total price
        assert!(base_component <= MAX_U64 - quadratic_component, EOverflow);
        let total_price = base_component + quadratic_component;
        
        let avg_price = total_price / amount;
        (total_price, avg_price)
    }

    /// Helper: Calculate sum of squares from 1 to n: n(n+1)(2n+1)/6
    fun calculate_sum_squares(n: u64): u64 {
        if (n == 0) {
            return 0
        };
        
        // Early guard: prevent n == MAX_U64 case where n + 1 overflows
        assert!(n < MAX_U64, EOverflow);
        
        // Overflow protection for intermediate calculations
        // n * (n+1) can overflow, so check first
        assert!(n <= MAX_U64 / (n + 1), EOverflow);
        let n_times_n_plus_one = n * (n + 1);
        
        // (2n+1) can overflow
        assert!(n <= (MAX_U64 - 1) / 2, EOverflow);
        let two_n_plus_one = 2 * n + 1;
        
        // n(n+1) * (2n+1) can overflow
        assert!(n_times_n_plus_one <= MAX_U64 / two_n_plus_one, EOverflow);
        let numerator = n_times_n_plus_one * two_n_plus_one;
        
        numerator / 6
    }

    /// Calculate refund amount when selling tokens using closed-form sum
    /// Selling reduces supply, so we sum from current_supply-amount to current_supply-1
    /// Returns (total refund, average price per token)
    public fun calculate_sell_price(
        base_price: u64,
        quadratic_coefficient: u64,
        current_supply: u64,
        amount: u64
    ): (u64, u64) {
        if (amount == 0) {
            return (0, 0)
        };
        
        // Prevent underflow: ensure we have enough supply to sell
        assert!(current_supply >= amount, EInsufficientLiquidity);
        
        // Selling reduces supply, so we sum from current_supply-amount to current_supply-1
        let start_supply = current_supply - amount;
        let end_supply = current_supply - 1;
        
        // Base price component
        assert!(amount <= MAX_U64 / base_price, EOverflow);
        let base_component = amount * base_price;
        
        // Sum of squares from start_supply to end_supply
        let sum_squares_end = calculate_sum_squares(end_supply);
        let sum_squares_start = if (start_supply == 0) { 0 } else { calculate_sum_squares(start_supply - 1) };
        
        assert!(sum_squares_end >= sum_squares_start, EOverflow);
        let sum_squares_range = sum_squares_end - sum_squares_start;
        
        assert!(sum_squares_range <= MAX_U64 / quadratic_coefficient, EOverflow);
        let quadratic_component = (sum_squares_range * quadratic_coefficient) / 10000;
        
        assert!(base_component <= MAX_U64 - quadratic_component, EOverflow);
        let total_refund = base_component + quadratic_component;
        
        let avg_price = total_refund / amount;
        (total_refund, avg_price)
    }

    /// Get token info from registry by associated_id (post/profile ID), not pool ID
    /// Returns a reference since TokenInfo no longer has copy ability
    public fun get_token_info(registry: &TokenRegistry, id: address): &TokenInfo {
        assert!(table::contains(&registry.tokens, id), ETokenNotFound);
        table::borrow(&registry.tokens, id)
    }

    /// Check if a token exists in the registry
    public fun token_exists(registry: &TokenRegistry, id: address): bool {
        table::contains(&registry.tokens, id)
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

    /// Get PoC redirection data from token pool
    public fun get_poc_redirect_to(pool: &TokenPool): &Option<address> {
        &pool.poc_redirect_to
    }

    /// Get PoC redirection percentage from token pool
    public fun get_poc_redirect_percentage(pool: &TokenPool): &Option<u64> {
        &pool.poc_redirect_percentage
    }

    /// Check if token pool has PoC redirection configured
    public fun has_poc_redirection(pool: &TokenPool): bool {
        option::is_some(&pool.poc_redirect_to) && option::is_some(&pool.poc_redirect_percentage)
    }

    /// Get the associated ID (post/profile ID) from a token pool
    public fun get_pool_associated_id(pool: &TokenPool): address {
        pool.info.associated_id
    }

    /// Set PoC redirection data for a token pool (called by PoC system)
    /// Set PoC redirection for a token pool (package-only, requires auth via entry function)
    public(package) fun set_poc_redirection(
        pool: &mut TokenPool,
        redirect_to: Option<address>,
        redirect_percentage: Option<u64>
    ) {
        // Validate PoC redirect percentage is in valid range (0-100 percent) if provided
        if (option::is_some(&redirect_percentage)) {
            let percentage = *option::borrow(&redirect_percentage);
            assert!(percentage <= 100, EInvalidFeeConfig);
        };
        pool.poc_redirect_to = redirect_to;
        pool.poc_redirect_percentage = redirect_percentage;
    }
    
    /// Entry function to set PoC redirection (requires pool owner)
    public entry fun set_poc_redirection_entry(
        registry: &TokenRegistry,
        pool: &mut TokenPool,
        redirect_to: Option<address>,
        redirect_percentage: Option<u64>,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        let token_info = get_token_info(registry, pool.info.associated_id);
        
        // Require caller to be pool owner
        assert!(caller == token_info.owner, ENotAuthorized);
        
        set_poc_redirection(pool, redirect_to, redirect_percentage);
    }
    
    /// Admin entry function to set PoC redirection (requires admin cap)
    public entry fun set_poc_redirection_admin(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        _admin_cap: &SocialProofTokensAdminCap,
        redirect_to: Option<address>,
        redirect_percentage: Option<u64>,
        _ctx: &mut TxContext
    ) {
        // Admin can set redirection for any pool
        set_poc_redirection(pool, redirect_to, redirect_percentage);
    }

    /// Clear PoC redirection data from a token pool (called by PoC system)
    public(package) fun clear_poc_redirection(pool: &mut TokenPool) {
        pool.poc_redirect_to = option::none();
        pool.poc_redirect_percentage = option::none();
    }


    // Test-only functions
    #[test_only]
    /// Initialize the social proof tokens system for testing
    /// In testing, we create admin caps directly for convenience
    public fun init_for_testing(ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        
        // Create and transfer admin capability to the transaction sender
        transfer::public_transfer(
            SocialProofTokensAdminCap {
                id: object::new(ctx),
            },
            sender
        );
        
        // Create and share social proof tokens config (same as production init)
        transfer::share_object(
            SocialProofTokensConfig {
                id: object::new(ctx),
                trading_creator_fee_bps: DEFAULT_TRADING_CREATOR_FEE_BPS,
                trading_platform_fee_bps: DEFAULT_TRADING_PLATFORM_FEE_BPS,
                trading_treasury_fee_bps: DEFAULT_TRADING_TREASURY_FEE_BPS,
                reservation_creator_fee_bps: DEFAULT_RESERVATION_CREATOR_FEE_BPS,
                reservation_platform_fee_bps: DEFAULT_RESERVATION_PLATFORM_FEE_BPS,
                reservation_treasury_fee_bps: DEFAULT_RESERVATION_TREASURY_FEE_BPS,
                base_price: DEFAULT_BASE_PRICE,
                quadratic_coefficient: DEFAULT_QUADRATIC_COEFFICIENT,
                max_hold_percent_bps: MAX_HOLD_PERCENT_BPS,
                post_threshold: DEFAULT_POST_THRESHOLD,
                profile_threshold: DEFAULT_PROFILE_THRESHOLD,
                max_individual_reservation_bps: DEFAULT_MAX_INDIVIDUAL_RESERVATION_BPS,
                max_reservers_per_pool: DEFAULT_MAX_RESERVERS_PER_POOL,
                trading_enabled: true,
            }
        );
        
        // Create and share token registry
        transfer::share_object(
            TokenRegistry {
                id: object::new(ctx),
                tokens: table::new(ctx),
                reservation_pools: table::new(ctx),
                version: upgrade::current_version(),
            }
        );
    }
    /// Create a new SocialProofTokensAdminCap for testing
    #[test_only]
    public fun create_admin_cap_for_testing(ctx: &mut TxContext): SocialProofTokensAdminCap {
        SocialProofTokensAdminCap {
            id: object::new(ctx)
        }
    }

    /// Create a mock TokenInfo for testing
    #[test_only]
    public fun create_mock_token_info(
        id: address,
        token_type: u8,
        owner: address,
        associated_id: address,
        symbol: String,
        name: String,
        circulating_supply: u64,
        base_price: u64,
        quadratic_coefficient: u64,
        created_at: u64
    ): TokenInfo {
        TokenInfo {
            id,
            token_type,
            owner,
            associated_id,
            symbol,
            name,
            circulating_supply,
            base_price,
            quadratic_coefficient,
            created_at,
        }
    }

    /// Create a mock TokenPool for testing
    #[test_only]
    public fun create_mock_token_pool(
        token_info: TokenInfo,
        ctx: &mut TxContext
    ): TokenPool {
        TokenPool {
            id: object::new(ctx),
            info: token_info,
            mys_balance: balance::zero(),
            holders: table::new(ctx),
            poc_redirect_to: option::none(),
            poc_redirect_percentage: option::none(),
            version: upgrade::current_version(),
        }
    }

    // === Versioning Functions ===

    /// Get the version of the token registry
    public fun registry_version(registry: &TokenRegistry): u64 {
        registry.version
    }

    /// Get a mutable reference to the registry version (for upgrade module)
    public(package) fun borrow_registry_version_mut(registry: &mut TokenRegistry): &mut u64 {
        &mut registry.version
    }

    /// Get the version of a token pool
    public fun pool_version(pool: &TokenPool): u64 {
        pool.version
    }

    /// Get a mutable reference to the pool version (for upgrade module)
    public(package) fun borrow_pool_version_mut(pool: &mut TokenPool): &mut u64 {
        &mut pool.version
    }

    /// Get the version of a reservation pool
    public fun reservation_pool_version(pool: &ReservationPoolObject): u64 {
        pool.version
    }

    /// Get a mutable reference to the reservation pool version (for upgrade module)
    public(package) fun borrow_reservation_pool_version_mut(pool: &mut ReservationPoolObject): &mut u64 {
        &mut pool.version
    }

    /// Migration function for TokenRegistry
    public entry fun migrate_token_registry(
        registry: &mut TokenRegistry,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(registry.version < current_version, EInvalidFeeConfig);
        
        // Remember old version and update to new version
        let old_version = registry.version;
        registry.version = current_version;
        
        // Emit event for object migration
        let registry_id = object::id(registry);
        upgrade::emit_migration_event(
            registry_id,
            string::utf8(b"TokenRegistry"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }

    /// Migration function for TokenPool
    public entry fun migrate_token_pool(
        pool: &mut TokenPool,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(pool.version < current_version, EInvalidFeeConfig);
        
        // Remember old version and update to new version
        let old_version = pool.version;
        pool.version = current_version;
        
        // Emit event for object migration
        let pool_id = object::id(pool);
        upgrade::emit_migration_event(
            pool_id,
            string::utf8(b"TokenPool"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }

    /// Migration function for ReservationPoolObject
    public entry fun migrate_reservation_pool(
        pool: &mut ReservationPoolObject,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(pool.version < current_version, EInvalidFeeConfig);
        
        // Remember old version and update to new version
        let old_version = pool.version;
        pool.version = current_version;
        
        // Emit event for object migration
        let pool_id = object::id(pool);
        upgrade::emit_migration_event(
            pool_id,
            string::utf8(b"ReservationPoolObject"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }
    
    /// Create a SocialProofTokensAdminCap for bootstrap (package visibility only)
    /// This function is only callable by other modules in the same package
    public(package) fun create_social_proof_tokens_admin_cap(ctx: &mut TxContext): SocialProofTokensAdminCap {
        SocialProofTokensAdminCap {
            id: object::new(ctx)
        }
    }
} 
