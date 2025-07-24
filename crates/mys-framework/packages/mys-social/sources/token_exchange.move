// Copyright (c) The Social Proof Foundation LLC
// SPDX-License-Identifier: Apache-2.0

/// Token Exchange module for MySocial platform.
/// This module provides functionality for creation and trading of both profile tokens
/// and post tokens using an Automated Market Maker (AMM) with a quadratic pricing curve.
/// It includes fee distribution mechanisms for transactions, splitting between profile owner,
/// platform, and ecosystem treasury.

#[allow(unused_field, deprecated_usage, unused_const, duplicate_alias, unused_use)]
module social_contracts::token_exchange {
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
    
    use social_contracts::profile::{Self, Profile, UsernameRegistry};
    use social_contracts::post::{Self, Post};
    use social_contracts::block_list::{BlockListRegistry};
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

    // === Constants ===
    // Token types
    const TOKEN_TYPE_PROFILE: u8 = 1;
    const TOKEN_TYPE_POST: u8 = 2;

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

    // Staking threshold constants for social proof token creation
    const DEFAULT_POST_THRESHOLD: u64 = 1_000_000_000_000; // 1,000 MYS in smallest units (9 decimals)
    const DEFAULT_PROFILE_THRESHOLD: u64 = 10_000_000_000_000; // 10,000 MYS in smallest units (9 decimals)
    const DEFAULT_MAX_INDIVIDUAL_STAKE_BPS: u64 = 2000; // 20% (1/5 of threshold)



    // === Structs ===

    /// Admin capability for the token exchange
    public struct ExchangeAdminCap has key, store {
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
        /// Ecosystem treasury address
        ecosystem_treasury: address,
        /// Maximum percentage a single wallet can hold of any token
        max_hold_percent_bps: u64,
        /// Staking thresholds for social proof token creation
        post_threshold: u64,
        profile_threshold: u64,
        /// Maximum percentage any individual can stake towards a single post/profile
        max_individual_stake_bps: u64,
        /// Emergency kill switch - when true, all trading is halted
        trading_halted: bool,
    }

    /// Registry of all tokens in the exchange
    public struct TokenRegistry has key {
        id: UID,
        /// Table from token ID to token info
        tokens: Table<address, TokenInfo>,
        /// Table from profile/post ID to staking pool info
        stake_pools: Table<address, StakePool>,
        /// Version for upgrades
        version: u64,
    }

    /// Staking pool for a specific post or profile
    public struct StakePool has store, copy, drop {
        /// Associated profile or post ID
        associated_id: address,
        /// Token type (1=profile, 2=post)
        token_type: u8,
        /// Owner of the profile/post
        owner: address,
        /// Total MYS staked towards this post/profile
        total_staked: u64,
        /// Required threshold to enable auction creation
        required_threshold: u64,
        /// List of all stakers (for efficient iteration)
        stakers: vector<address>,
        /// Creation timestamp
        created_at: u64,
    }

    /// Individual stake information
    public struct StakeInfo has store, copy, drop {
        /// Staker's address
        staker: address,
        /// Amount staked in MYS
        amount: u64,
        /// Timestamp when stake was created
        staked_at: u64,
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



    /// Staking pool for collecting MYS stakes towards posts/profiles
    public struct StakePoolObject has key {
        id: UID,
        /// Stake pool info
        info: StakePool,
        /// MYS balance staked in this pool
        mys_balance: Balance<MYS>,
        /// Mapping of stakers' addresses to their stake amounts
        stakes: Table<address, u64>,
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



    /// Event emitted when MYS is staked towards a post/profile
    public struct StakeCreatedEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        staker: address,
        amount: u64,
        total_staked: u64,
        threshold_met: bool,
        staked_at: u64,
    }

    /// Event emitted when MYS stake is withdrawn
    public struct StakeWithdrawnEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        staker: address,
        amount: u64,
        total_staked: u64,
        withdrawn_at: u64,
    }

    /// Event emitted when staking threshold is met for the first time
    public struct ThresholdMetEvent has copy, drop {
        associated_id: address,
        token_type: u8,
        owner: address,
        total_staked: u64,
        required_threshold: u64,
        timestamp: u64,
    }

    /// Event emitted when exchange config is updated
    public struct ConfigUpdatedEvent has copy, drop {
        /// Who performed the update
        updated_by: address,
        /// When the update occurred
        timestamp: u64,
        /// Fee percentages
        total_fee_bps: u64,
        creator_fee_bps: u64,
        platform_fee_bps: u64,
        treasury_fee_bps: u64,
        /// Curve parameters
        base_price: u64,
        quadratic_coefficient: u64,
        /// Treasury addresses
        ecosystem_treasury: address,
        /// Maximum hold percentage
        max_hold_percent_bps: u64,
        /// Staking thresholds
        post_threshold: u64,
        profile_threshold: u64,
        max_individual_stake_bps: u64,
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
        /// New state of trading (true = halted, false = active)
        trading_halted: bool,
        /// Timestamp of the action
        timestamp: u64,
        /// Reason for the action (optional)
        reason: String,
    }

    // === Initialization ===
    
    /// Initialize the token exchange system
    fun init(ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        
        // Create and transfer admin capability to the transaction sender
        transfer::public_transfer(
            ExchangeAdminCap {
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
                ecosystem_treasury: sender, // Initially set to sender, should be updated
                max_hold_percent_bps: MAX_HOLD_PERCENT_BPS,
                post_threshold: DEFAULT_POST_THRESHOLD,
                profile_threshold: DEFAULT_PROFILE_THRESHOLD,
                max_individual_stake_bps: DEFAULT_MAX_INDIVIDUAL_STAKE_BPS,
                trading_halted: false, // Trading is enabled by default
            }
        );
        
        // Create and share token registry
        transfer::share_object(
            TokenRegistry {
                id: object::new(ctx),
                tokens: table::new(ctx),
                stake_pools: table::new(ctx),
                version: upgrade::current_version(),
            }
        );
    }

    // === Admin Functions ===

    /// Update exchange configuration
    public entry fun update_exchange_config(
        _admin_cap: &ExchangeAdminCap,
        config: &mut ExchangeConfig,
        total_fee_bps: u64, 
        creator_fee_bps: u64,
        platform_fee_bps: u64,
        treasury_fee_bps: u64,
        base_price: u64,
        quadratic_coefficient: u64,
        ecosystem_treasury: address,
        max_hold_percent_bps: u64,
        post_threshold: u64,
        profile_threshold: u64,
        max_individual_stake_bps: u64,
        ctx: &mut TxContext
    ) {
        // Verify sum of fee percentages equals total
        assert!(creator_fee_bps + platform_fee_bps + treasury_fee_bps == total_fee_bps, EInvalidFeeConfig);
        
        // Verify curve parameters are valid
        assert!(base_price > 0 && quadratic_coefficient > 0, EInvalidCurveParams);
        
        // Update fee config
        config.total_fee_bps = total_fee_bps;
        config.creator_fee_bps = creator_fee_bps;
        config.platform_fee_bps = platform_fee_bps;
        config.treasury_fee_bps = treasury_fee_bps;
        
        // Update curve parameters
        config.base_price = base_price;
        config.quadratic_coefficient = quadratic_coefficient;
        
        // Update treasury addresses
        config.ecosystem_treasury = ecosystem_treasury;
        config.max_hold_percent_bps = max_hold_percent_bps;
        
        // Update staking thresholds
        config.post_threshold = post_threshold;
        config.profile_threshold = profile_threshold;
        config.max_individual_stake_bps = max_individual_stake_bps;
        
        // Emit config updated event
        event::emit(ConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            timestamp: tx_context::epoch(ctx),
            total_fee_bps,
            creator_fee_bps,
            platform_fee_bps,
            treasury_fee_bps,
            base_price,
            quadratic_coefficient,
            ecosystem_treasury,
            max_hold_percent_bps,
            post_threshold,
            profile_threshold,
            max_individual_stake_bps,
        });
    }

    /// Emergency kill switch function - only callable by admin
    /// This function can immediately halt all trading on the platform
    public entry fun toggle_emergency_kill_switch(
        _admin_cap: &ExchangeAdminCap,
        config: &mut ExchangeConfig,
        halt_trading: bool,
        reason: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Update the trading halted status
        config.trading_halted = halt_trading;
        
        // Emit event for audit trail
        event::emit(EmergencyKillSwitchEvent {
            admin: tx_context::sender(ctx),
            trading_halted: halt_trading,
            timestamp: tx_context::epoch(ctx),
            reason: string::utf8(reason),
        });
    }

    /// Check if trading is currently halted
    public fun is_trading_halted(config: &ExchangeConfig): bool {
        config.trading_halted
    }

    // === Staking Functions ===

    /// Stake MYS tokens towards a post to support social proof token creation
    public entry fun stake_towards_post(
        registry: &mut TokenRegistry,
        config: &ExchangeConfig,
        stake_pool_object: &mut StakePoolObject,
        post: &Post,
        mut payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(!config.trading_halted, ETradingHalted);
        
        let staker = tx_context::sender(ctx);
        let post_id = post::get_id_address(post);
        let post_owner = post::get_post_owner(post);
        let now = tx_context::epoch(ctx);
        
        // Verify stake pool matches the post
        assert!(stake_pool_object.info.associated_id == post_id, EInvalidID);
        assert!(stake_pool_object.info.token_type == TOKEN_TYPE_POST, EInvalidTokenType);
        
        // Ensure staker has enough funds
        assert!(coin::value(&payment) >= amount && amount > 0, EInsufficientFunds);
        
        // Check individual stake limit
        let max_individual_stake = (config.post_threshold * config.max_individual_stake_bps) / 10000;
        let current_stake = if (table::contains(&stake_pool_object.stakes, staker)) {
            *table::borrow(&stake_pool_object.stakes, staker)
        } else {
            0
        };
        assert!(current_stake + amount <= max_individual_stake, EExceededMaxHold);
        
        // Extract stake payment
        let stake_payment = coin::split(&mut payment, amount, ctx);
        balance::join(&mut stake_pool_object.mys_balance, coin::into_balance(stake_payment));
        
        // Update staker's balance in the pool
        if (table::contains(&stake_pool_object.stakes, staker)) {
            let stake_balance = table::borrow_mut(&mut stake_pool_object.stakes, staker);
            *stake_balance = *stake_balance + amount;
        } else {
            table::add(&mut stake_pool_object.stakes, staker, amount);
            // Add to stakers list for tracking
            vector::push_back(&mut stake_pool_object.info.stakers, staker);
        };
        
        // Update total staked
        stake_pool_object.info.total_staked = stake_pool_object.info.total_staked + amount;
        
        // Update registry
        if (table::contains(&registry.stake_pools, post_id)) {
            let registry_pool = table::borrow_mut(&mut registry.stake_pools, post_id);
            registry_pool.total_staked = stake_pool_object.info.total_staked;
        } else {
            // Create registry entry if it doesn't exist
            let stake_pool = StakePool {
                associated_id: post_id,
                token_type: TOKEN_TYPE_POST,
                owner: post_owner,
                total_staked: stake_pool_object.info.total_staked,
                required_threshold: config.post_threshold,
                stakers: stake_pool_object.info.stakers,
                created_at: now,
            };
            table::add(&mut registry.stake_pools, post_id, stake_pool);
        };
        
        // Check if threshold was just met
        let threshold_met = stake_pool_object.info.total_staked >= config.post_threshold;
        let was_threshold_met = (stake_pool_object.info.total_staked - amount) >= config.post_threshold;
        
        // Emit threshold met event if this stake pushed us over the threshold
        if (threshold_met && !was_threshold_met) {
            event::emit(ThresholdMetEvent {
                associated_id: post_id,
                token_type: TOKEN_TYPE_POST,
                owner: post_owner,
                total_staked: stake_pool_object.info.total_staked,
                required_threshold: config.post_threshold,
                timestamp: now,
            });
        };
        
        // Return excess payment
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, staker);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Emit stake created event
        event::emit(StakeCreatedEvent {
            associated_id: post_id,
            token_type: TOKEN_TYPE_POST,
            staker,
            amount,
            total_staked: stake_pool_object.info.total_staked,
            threshold_met,
            staked_at: now,
        });
    }

    /// Stake MYS tokens towards a profile to support social proof token creation
    public entry fun stake_towards_profile(
        registry: &mut TokenRegistry,
        config: &ExchangeConfig,
        stake_pool_object: &mut StakePoolObject,
        profile: &Profile,
        mut payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(!config.trading_halted, ETradingHalted);
        
        let staker = tx_context::sender(ctx);
        let profile_id = profile::get_id_address(profile);
        let profile_owner = profile::get_owner(profile);
        let now = tx_context::epoch(ctx);
        
        // Verify stake pool matches the profile
        assert!(stake_pool_object.info.associated_id == profile_id, EInvalidID);
        assert!(stake_pool_object.info.token_type == TOKEN_TYPE_PROFILE, EInvalidTokenType);
        
        // Ensure staker has enough funds
        assert!(coin::value(&payment) >= amount && amount > 0, EInsufficientFunds);
        
        // Check individual stake limit
        let max_individual_stake = (config.profile_threshold * config.max_individual_stake_bps) / 10000;
        let current_stake = if (table::contains(&stake_pool_object.stakes, staker)) {
            *table::borrow(&stake_pool_object.stakes, staker)
        } else {
            0
        };
        assert!(current_stake + amount <= max_individual_stake, EExceededMaxHold);
        
        // Extract stake payment
        let stake_payment = coin::split(&mut payment, amount, ctx);
        balance::join(&mut stake_pool_object.mys_balance, coin::into_balance(stake_payment));
        
        // Update staker's balance in the pool
        if (table::contains(&stake_pool_object.stakes, staker)) {
            let stake_balance = table::borrow_mut(&mut stake_pool_object.stakes, staker);
            *stake_balance = *stake_balance + amount;
        } else {
            table::add(&mut stake_pool_object.stakes, staker, amount);
            // Add to stakers list for tracking
            vector::push_back(&mut stake_pool_object.info.stakers, staker);
        };
        
        // Update total staked
        stake_pool_object.info.total_staked = stake_pool_object.info.total_staked + amount;
        
        // Update registry
        if (table::contains(&registry.stake_pools, profile_id)) {
            let registry_pool = table::borrow_mut(&mut registry.stake_pools, profile_id);
            registry_pool.total_staked = stake_pool_object.info.total_staked;
        } else {
            // Create registry entry if it doesn't exist
            let stake_pool = StakePool {
                associated_id: profile_id,
                token_type: TOKEN_TYPE_PROFILE,
                owner: profile_owner,
                total_staked: stake_pool_object.info.total_staked,
                required_threshold: config.profile_threshold,
                stakers: stake_pool_object.info.stakers,
                created_at: now,
            };
            table::add(&mut registry.stake_pools, profile_id, stake_pool);
        };
        
        // Check if threshold was just met
        let threshold_met = stake_pool_object.info.total_staked >= config.profile_threshold;
        let was_threshold_met = (stake_pool_object.info.total_staked - amount) >= config.profile_threshold;
        
        // Emit threshold met event if this stake pushed us over the threshold
        if (threshold_met && !was_threshold_met) {
            event::emit(ThresholdMetEvent {
                associated_id: profile_id,
                token_type: TOKEN_TYPE_PROFILE,
                owner: profile_owner,
                total_staked: stake_pool_object.info.total_staked,
                required_threshold: config.profile_threshold,
                timestamp: now,
            });
        };
        
        // Return excess payment
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, staker);
        } else {
            coin::destroy_zero(payment);
        };
        
        // Emit stake created event
        event::emit(StakeCreatedEvent {
            associated_id: profile_id,
            token_type: TOKEN_TYPE_PROFILE,
            staker,
            amount,
            total_staked: stake_pool_object.info.total_staked,
            threshold_met,
            staked_at: now,
        });
    }

    /// Withdraw MYS stake from a post or profile
    public entry fun withdraw_stake(
        registry: &mut TokenRegistry,
        stake_pool_object: &mut StakePoolObject,
        amount: u64,
        ctx: &mut TxContext
    ) {
        let staker = tx_context::sender(ctx);
        let associated_id = stake_pool_object.info.associated_id;
        let now = tx_context::epoch(ctx);
        
        // Verify staker has a stake
        assert!(table::contains(&stake_pool_object.stakes, staker), ENoTokensOwned);
        
        let current_stake = *table::borrow(&stake_pool_object.stakes, staker);
        assert!(current_stake >= amount, EInsufficientLiquidity);
        
        // Update staker's balance
        if (current_stake == amount) {
            // Remove staker completely
            table::remove(&mut stake_pool_object.stakes, staker);
            
            // Remove from stakers list
            let mut i = 0;
            let len = vector::length(&stake_pool_object.info.stakers);
            while (i < len) {
                if (*vector::borrow(&stake_pool_object.info.stakers, i) == staker) {
                    vector::remove(&mut stake_pool_object.info.stakers, i);
                    break
                };
                i = i + 1;
            };
        } else {
            // Reduce stake amount
            let stake_balance = table::borrow_mut(&mut stake_pool_object.stakes, staker);
            *stake_balance = *stake_balance - amount;
        };
        
        // Update total staked
        stake_pool_object.info.total_staked = stake_pool_object.info.total_staked - amount;
        
        // Update registry
        if (table::contains(&registry.stake_pools, associated_id)) {
            let registry_pool = table::borrow_mut(&mut registry.stake_pools, associated_id);
            registry_pool.total_staked = stake_pool_object.info.total_staked;
        };
        
        // Transfer staked MYS back to staker
        let refund_balance = balance::split(&mut stake_pool_object.mys_balance, amount);
        let refund_coin = coin::from_balance(refund_balance, ctx);
        transfer::public_transfer(refund_coin, staker);
        
        // Emit stake withdrawn event
        event::emit(StakeWithdrawnEvent {
            associated_id,
            token_type: stake_pool_object.info.token_type,
            staker,
            amount,
            total_staked: stake_pool_object.info.total_staked,
            withdrawn_at: now,
        });
    }

    /// Create a new stake pool for a post or profile
    public entry fun create_stake_pool(
        registry: &mut TokenRegistry,
        config: &ExchangeConfig,
        associated_id: address,
        token_type: u8,
        owner: address,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(!config.trading_halted, ETradingHalted);
        
        // Verify caller is the owner
        assert!(tx_context::sender(ctx) == owner, ENotAuthorized);
        
        // Check if stake pool already exists
        assert!(!table::contains(&registry.stake_pools, associated_id), ETokenAlreadyExists);
        
        let now = tx_context::epoch(ctx);
        let required_threshold = if (token_type == TOKEN_TYPE_POST) {
            config.post_threshold
        } else if (token_type == TOKEN_TYPE_PROFILE) {
            config.profile_threshold
        } else {
            abort EInvalidTokenType
        };
        
        // Create stake pool info
        let stake_pool = StakePool {
            associated_id,
            token_type,
            owner,
            total_staked: 0,
            required_threshold,
            stakers: vector::empty(),
            created_at: now,
        };
        
        // Add to registry
        table::add(&mut registry.stake_pools, associated_id, stake_pool);
        
        // Create stake pool object
        let stake_pool_object = StakePoolObject {
            id: object::new(ctx),
            info: stake_pool,
            mys_balance: balance::zero(),
            stakes: table::new(ctx),
            version: upgrade::current_version(),
        };
        
        transfer::share_object(stake_pool_object);
    }

    /// Check if staking threshold is met for auction creation
    public fun can_create_auction(
        registry: &TokenRegistry,
        associated_id: address
    ): bool {
        if (!table::contains(&registry.stake_pools, associated_id)) {
            return false
        };
        
        let stake_pool = table::borrow(&registry.stake_pools, associated_id);
        stake_pool.total_staked >= stake_pool.required_threshold
    }
    
    /// Create a social proof token directly from a stake pool once threshold is met
    /// This replaces the auction system - only the post/profile owner can call this
    public entry fun create_social_proof_token(
        registry: &mut TokenRegistry,
        config: &ExchangeConfig,
        stake_pool_object: &mut StakePoolObject,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(!config.trading_halted, ETradingHalted);
        
        let caller = tx_context::sender(ctx);
        let associated_id = stake_pool_object.info.associated_id;
        
        // Verify caller is the owner of the post/profile
        assert!(caller == stake_pool_object.info.owner, ENotAuthorized);
        
        // Check if staking threshold has been met
        assert!(can_create_auction(registry, associated_id), EViralThresholdNotMet);
        
        // Verify token has not already been created
        assert!(!table::contains(&registry.tokens, associated_id), ETokenAlreadyExists);
        
        // Calculate initial token supply based on total staked amount
        // Use the same scaling formula as the old auction system
        let total_staked = stake_pool_object.info.total_staked;
        let sqrt_staked = math::sqrt(total_staked);
        let cbrt_staked = math::sqrt(sqrt_staked); // approximation of cube root
        let mut scale_factor = sqrt_staked * cbrt_staked; // staked^0.75
        
        // Divide the scale factor to make each token worth more than 1 MYS
        scale_factor = scale_factor / 1000;
        
        // Apply different base multipliers for profile vs post tokens
        let mut initial_token_supply = if (stake_pool_object.info.token_type == TOKEN_TYPE_PROFILE) {
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
            token_type: stake_pool_object.info.token_type,
            owner: stake_pool_object.info.owner,
            associated_id,
            symbol: if (stake_pool_object.info.token_type == TOKEN_TYPE_PROFILE) {
                string::utf8(b"PUSER")
            } else {
                string::utf8(b"PPOST")
            },
            name: if (stake_pool_object.info.token_type == TOKEN_TYPE_PROFILE) {
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
        
        let mut token_pool = TokenPool {
            id: pool_id,
            info: updated_token_info,
            mys_balance: balance::zero(),
            holders: table::new(ctx),
            poc_redirect_to: option::none(),
            poc_redirect_percentage: option::none(),
            version: upgrade::current_version(),
        };
        
        // Distribute tokens to stakers proportionally
        let stakers = &stake_pool_object.info.stakers;
        let num_stakers = vector::length(stakers);
        
        let mut i = 0;
        while (i < num_stakers) {
            let staker = *vector::borrow(stakers, i);
            let stake_amount = *table::borrow(&stake_pool_object.stakes, staker);
            
            // Calculate token amount based on staker's proportion of total stake
            let token_amount = (stake_amount * initial_token_supply) / total_staked;
            
            if (token_amount > 0) {
                // Update holder's balance in the pool
                table::add(&mut token_pool.holders, staker, token_amount);
                
                // Create social token for the staker
                let social_token = SocialToken {
                    id: object::new(ctx),
                    pool_id: pool_address,
                    token_type: stake_pool_object.info.token_type,
                    amount: token_amount,
                };
                
                // Transfer social token to staker
                transfer::public_transfer(social_token, staker);
            };
            
            i = i + 1;
        };
        
        // Transfer all staked MYS to the token pool as initial liquidity
        balance::join(&mut token_pool.mys_balance, balance::withdraw_all(&mut stake_pool_object.mys_balance));
        
        // Clear the stake pool since it's now converted to a token
        stake_pool_object.info.total_staked = 0;
        // Note: We keep the stakes table for reference but it's no longer active
        
        // Add to registry
        table::add(&mut registry.tokens, associated_id, updated_token_info);
        
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

    // === PoC Revenue Redirection Functions ===

    /// Update PoC redirection data for a token pool (called by PoC system)
    /// This function copies PoC data from a post into the corresponding token pool
    public entry fun update_token_poc_data(
        pool: &mut TokenPool,
        post: &Post,
        ctx: &mut TxContext
    ) {
        // Verify this is a post token pool
        assert!(pool.info.token_type == TOKEN_TYPE_POST, EInvalidTokenType);
        
        // Verify the post matches the token pool
        let post_id = post::get_id_address(post);
        assert!(post_id == pool.info.associated_id, EInvalidID);
        
        // Verify caller is authorized (post owner)
        let caller = tx_context::sender(ctx);
        assert!(caller == post::get_post_owner(post), ENotAuthorized);
        
        // Copy PoC data from post to pool
        pool.poc_redirect_to = if (option::is_some(post::get_revenue_redirect_to(post))) {
            option::some(*option::borrow(post::get_revenue_redirect_to(post)))
        } else {
            option::none()
        };
        
        pool.poc_redirect_percentage = if (option::is_some(post::get_revenue_redirect_percentage(post))) {
            option::some(*option::borrow(post::get_revenue_redirect_percentage(post)))
        } else {
            option::none()
        };
    }

    /// Calculate PoC revenue split - shared utility for consistent logic
    fun calculate_poc_split(amount: u64, redirect_percentage: u64): (u64, u64) {
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

    // === Trading Functions ===

    /// Buy tokens from the pool - first purchase
    /// This function handles buying tokens for first-time buyers of a specific token
    public entry fun buy_tokens(
        _registry: &TokenRegistry,
        pool: &mut TokenPool,
        config: &ExchangeConfig,
        block_list_registry: &BlockListRegistry,
        platform: &mut social_contracts::platform::Platform,
        mut payment: Coin<MYS>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(!config.trading_halted, ETradingHalted);
        
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
        
        // Extract payment and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee(pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee - add to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                // Add to platform treasury
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, ctx);
                // Destroy the emptied coin
                coin::destroy_zero(platform_fee_coin);
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
        platform: &mut social_contracts::platform::Platform,
        mut payment: Coin<MYS>,
        amount: u64,
        social_token: &mut SocialToken,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(!config.trading_halted, ETradingHalted);
        
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
        
        // Extract payment and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send creator fee with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee(pool, creator_fee, &mut payment, ctx);
            };
            
            // Send platform fee - add to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::split(&mut payment, platform_fee, ctx);
                // Add to platform treasury
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, ctx);
                // Destroy the emptied coin
                coin::destroy_zero(platform_fee_coin);
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
        platform: &mut social_contracts::platform::Platform,
        social_token: &mut SocialToken,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Check if trading is halted
        assert!(!config.trading_halted, ETradingHalted);
        
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
        
        // Process and distribute fees with PoC redirection support
        if (fee_amount > 0) {
            // Send fee to creator with PoC redirection support
            if (creator_fee > 0) {
                distribute_creator_fee_from_pool(pool, creator_fee, ctx);
            };
            
            // Send fee to platform - add to platform treasury
            if (platform_fee > 0) {
                let mut platform_fee_coin = coin::from_balance(balance::split(&mut pool.mys_balance, platform_fee), ctx);
                // Add to platform treasury
                social_contracts::platform::add_to_treasury(platform, &mut platform_fee_coin, platform_fee, ctx);
                // Destroy the emptied coin
                coin::destroy_zero(platform_fee_coin);
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
    public fun set_poc_redirection(
        pool: &mut TokenPool,
        redirect_to: Option<address>,
        redirect_percentage: Option<u64>
    ) {
        pool.poc_redirect_to = redirect_to;
        pool.poc_redirect_percentage = redirect_percentage;
    }

    /// Clear PoC redirection data from a token pool (called by PoC system)
    public fun clear_poc_redirection(pool: &mut TokenPool) {
        pool.poc_redirect_to = option::none();
        pool.poc_redirect_percentage = option::none();
    }

    // Test-only functions
    #[test_only]
    /// Initialize the token exchange for testing
    public fun init_for_testing(ctx: &mut TxContext) {
        init(ctx)
    }

    /// Create a new ExchangeAdminCap for testing
    #[test_only]
    public fun create_admin_cap_for_testing(ctx: &mut TxContext): ExchangeAdminCap {
        ExchangeAdminCap {
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
    public fun borrow_registry_version_mut(registry: &mut TokenRegistry): &mut u64 {
        &mut registry.version
    }

    /// Get the version of a token pool
    public fun pool_version(pool: &TokenPool): u64 {
        pool.version
    }

    /// Get a mutable reference to the pool version (for upgrade module)
    public fun borrow_pool_version_mut(pool: &mut TokenPool): &mut u64 {
        &mut pool.version
    }

    /// Get the version of a stake pool
    public fun stake_pool_version(pool: &StakePoolObject): u64 {
        pool.version
    }

    /// Get a mutable reference to the stake pool version (for upgrade module)
    public fun borrow_stake_pool_version_mut(pool: &mut StakePoolObject): &mut u64 {
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

    /// Migration function for StakePoolObject
    public entry fun migrate_stake_pool(
        pool: &mut StakePoolObject,
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
            string::utf8(b"StakePoolObject"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }
} 