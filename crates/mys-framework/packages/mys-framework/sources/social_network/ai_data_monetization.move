// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// AI Data Monetization module for MySocial network.
/// This module enables users to opt-in to monetize their data through AI agents,
/// with revenue sharing between users, platforms, and MySocial.
module mys::ai_data_monetization {
    use std::vector;
    use std::string::{Self, String};
    use mys::object::{Self, UID, ID};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
    use mys::event;
    use mys::table::{Self, Table};
    use mys::coin::{Self, Coin};
    use mys::balance::{Self, Balance};
    use mys::mys::MYS;
    
    use mys::ai_agent_mpc::{Self, AgentRegistry, AgentCap};
    use mys::ai_agent_integration::{Self};
    use mys::platform::{Self, Platform};
    use mys::profile::{Self, Profile};
    
    // === Errors ===
    /// Unauthorized operation
    const EUnauthorized: u64 = 0;
    /// Agent not registered
    const EAgentNotRegistered: u64 = 1;
    /// User not opted in for monetization
    const EUserNotOptedIn: u64 = 2;
    /// Invalid fee configuration
    const EInvalidFeeConfig: u64 = 3;
    /// Insufficient balance
    const EInsufficientBalance: u64 = 4;
    /// Invalid payment amount
    const EInvalidPayment: u64 = 5;
    /// Withdrawal exceeds available balance
    const EWithdrawalExceedsBalance: u64 = 6;
    
    // === Data Usage Types ===
    /// Analytics usage
    const USAGE_ANALYTICS: u8 = 0;
    /// Profile data usage
    const USAGE_PROFILE: u8 = 1;
    /// Content data usage
    const USAGE_CONTENT: u8 = 2;
    /// Social graph data usage
    const USAGE_SOCIAL_GRAPH: u8 = 3;
    
    // === Monetization Levels ===
    /// Basic data monetization (public analytics only)
    const MONETIZATION_BASIC: u8 = 0;
    /// Standard data monetization (profile data, content engagement)
    const MONETIZATION_STANDARD: u8 = 1;
    /// Premium data monetization (comprehensive data access)
    const MONETIZATION_PREMIUM: u8 = 2;
    
    // === Default Revenue Shares ===
    /// Default user share percentage (50%)
    const DEFAULT_USER_SHARE: u64 = 50;
    /// Default platform share percentage (30%)
    const DEFAULT_PLATFORM_SHARE: u64 = 30;
    /// Default MySocial share percentage (20%)
    const DEFAULT_MYSOCIAL_SHARE: u64 = 20;
    
    // === Structs ===
    
    /// Fee configuration for data monetization
    struct FeeConfig has key {
        id: UID,
        /// Default fee for basic data usage (in MYS tokens)
        basic_fee: u64,
        /// Default fee for standard data usage (in MYS tokens)
        standard_fee: u64,
        /// Default fee for premium data usage (in MYS tokens)
        premium_fee: u64,
        /// User share percentage (out of 100)
        user_share: u64,
        /// Platform share percentage (out of 100)
        platform_share: u64,
        /// MySocial share percentage (out of 100)
        mysocial_share: u64,
        /// MySocial treasury address
        treasury_address: address,
    }
    
    /// AI Data Monetization Treasury
    struct Treasury has key {
        id: UID,
        /// Balance holding MySocial's share
        balance: Balance<MYS>,
    }
    
    /// AI Data Monetization Manager
    struct DataMonetizationManager has key {
        id: UID,
        /// Map from profile ID to monetization settings
        profile_settings: Table<ID, ProfileMonetizationSettings>,
        /// Map from platform ID to platform treasury
        platform_treasuries: Table<ID, PlatformTreasury>,
        /// Map from agent ID to payment record
        agent_payments: Table<ID, AgentPaymentRecord>,
        /// Map from agent ID to fee override
        agent_fee_overrides: Table<ID, AgentFeeOverride>,
        /// Total earnings across all users
        total_earnings: u64,
    }
    
    /// Profile monetization settings
    struct ProfileMonetizationSettings has store, drop {
        /// Profile ID
        profile_id: ID,
        /// Monetization enabled
        monetization_enabled: bool,
        /// Monetization level
        monetization_level: u8,
        /// Allowed data usage types
        allowed_usage_types: vector<u8>,
        /// Revenue earned (not yet withdrawn)
        earned_revenue: u64,
        /// Total all-time earnings
        total_earnings: u64,
        /// Last updated timestamp
        last_updated: u64,
    }
    
    /// Platform treasury for data monetization
    struct PlatformTreasury has store {
        /// Platform ID
        platform_id: ID,
        /// Balance for platform's share
        balance: Balance<MYS>,
        /// Total earnings
        total_earnings: u64,
    }
    
    /// Agent payment record
    struct AgentPaymentRecord has store, drop {
        /// Agent ID
        agent_id: ID,
        /// Total payments made
        total_payments: u64,
        /// Count of data usages paid for
        usage_count: u64,
        /// Last payment timestamp
        last_payment: u64,
    }
    
    /// Fee override for a specific agent
    struct AgentFeeOverride has store, drop {
        /// Agent ID
        agent_id: ID,
        /// Basic usage fee override
        basic_fee: u64,
        /// Standard usage fee override
        standard_fee: u64,
        /// Premium usage fee override
        premium_fee: u64,
        /// Custom user share percentage
        user_share: u64,
        /// Custom platform share percentage
        platform_share: u64,
        /// Custom MySocial share percentage
        mysocial_share: u64,
    }
    
    /// User's data usage authorization token
    struct DataUsageAuthorization has key, store {
        id: UID,
        /// Profile that authorized the data usage
        profile_id: ID,
        /// Agent authorized to use the data
        agent_id: ID,
        /// Platform where the data will be used
        platform_id: ID,
        /// Monetization level of the authorization
        monetization_level: u8,
        /// Allowed data usage types
        allowed_usage_types: vector<u8>,
        /// Payment amount for this authorization
        payment_amount: u64,
        /// When the authorization was created
        creation_timestamp: u64,
        /// When the authorization expires
        expiration_timestamp: u64,
    }
    
    // === Events ===
    
    /// Event emitted when a user opts in for data monetization
    struct DataMonetizationOptInEvent has copy, drop {
        profile_id: ID,
        monetization_level: u8,
        allowed_usage_types: vector<u8>,
        timestamp: u64,
    }
    
    /// Event emitted when an agent pays for data usage
    struct DataUsagePaymentEvent has copy, drop {
        agent_id: ID,
        platform_id: ID,
        profile_id: ID,
        usage_type: u8,
        monetization_level: u8,
        payment_amount: u64,
        user_share: u64,
        platform_share: u64,
        mysocial_share: u64,
        timestamp: u64,
    }
    
    /// Event emitted when a user withdraws their earnings
    struct EarningsWithdrawalEvent has copy, drop {
        profile_id: ID,
        amount: u64,
        recipient: address,
        timestamp: u64,
    }
    
    // === Initialization ===
    
    /// Initialize the AI Data Monetization system
    fun init(ctx: &mut TxContext) {
        // Create and share fee configuration
        transfer::share_object(
            FeeConfig {
                id: object::new(ctx),
                basic_fee: 10, // 10 MYS tokens for basic usage
                standard_fee: 50, // 50 MYS tokens for standard usage
                premium_fee: 100, // 100 MYS tokens for premium usage
                user_share: DEFAULT_USER_SHARE,
                platform_share: DEFAULT_PLATFORM_SHARE,
                mysocial_share: DEFAULT_MYSOCIAL_SHARE,
                treasury_address: tx_context::sender(ctx),
            }
        );
        
        // Create and share treasury
        transfer::share_object(
            Treasury {
                id: object::new(ctx),
                balance: balance::zero(),
            }
        );
        
        // Create and share data monetization manager
        transfer::share_object(
            DataMonetizationManager {
                id: object::new(ctx),
                profile_settings: table::new(ctx),
                platform_treasuries: table::new(ctx),
                agent_payments: table::new(ctx),
                agent_fee_overrides: table::new(ctx),
                total_earnings: 0,
            }
        );
    }
    
    // === Monetization Settings ===
    
    /// Opt in for data monetization
    public entry fun opt_in_for_monetization(
        manager: &mut DataMonetizationManager,
        profile: &Profile,
        monetization_level: u8,
        allowed_usage_types: vector<u8>,
        ctx: &mut TxContext
    ) {
        // Verify caller owns the profile
        assert!(profile::owner(profile) == tx_context::sender(ctx), EUnauthorized);
        
        // Verify monetization level is valid
        assert!(
            monetization_level == MONETIZATION_BASIC ||
            monetization_level == MONETIZATION_STANDARD ||
            monetization_level == MONETIZATION_PREMIUM,
            EInvalidFeeConfig
        );
        
        let profile_id = object::id(profile);
        
        // Create settings
        let settings = ProfileMonetizationSettings {
            profile_id,
            monetization_enabled: true,
            monetization_level,
            allowed_usage_types,
            earned_revenue: 0,
            total_earnings: 0,
            last_updated: tx_context::epoch_timestamp_ms(ctx),
        };
        
        // Add or update settings
        if (table::contains(&manager.profile_settings, profile_id)) {
            let existing_settings = table::borrow_mut(&mut manager.profile_settings, profile_id);
            
            // Preserve earnings when updating settings
            let earned_revenue = existing_settings.earned_revenue;
            let total_earnings = existing_settings.total_earnings;
            
            *existing_settings = settings;
            existing_settings.earned_revenue = earned_revenue;
            existing_settings.total_earnings = total_earnings;
        } else {
            table::add(&mut manager.profile_settings, profile_id, settings);
        };
        
        // Emit event
        event::emit(DataMonetizationOptInEvent {
            profile_id,
            monetization_level,
            allowed_usage_types,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }
    
    /// Opt out from data monetization
    public entry fun opt_out_from_monetization(
        manager: &mut DataMonetizationManager,
        profile: &Profile,
        ctx: &mut TxContext
    ) {
        // Verify caller owns the profile
        assert!(profile::owner(profile) == tx_context::sender(ctx), EUnauthorized);
        
        let profile_id = object::id(profile);
        
        // Verify profile has monetization settings
        assert!(table::contains(&manager.profile_settings, profile_id), EUserNotOptedIn);
        
        // Get settings and disable monetization
        let settings = table::borrow_mut(&mut manager.profile_settings, profile_id);
        settings.monetization_enabled = false;
        settings.last_updated = tx_context::epoch_timestamp_ms(ctx);
    }
    
    // === Agent Payment ===
    
    /// Pay for data usage
    public entry fun pay_for_data_usage(
        manager: &mut DataMonetizationManager,
        treasury: &mut Treasury,
        fee_config: &FeeConfig,
        agent_cap: &AgentCap,
        platform_id: ID,
        profile_id: ID,
        usage_type: u8,
        payment: &mut Coin<MYS>,
        duration_hours: u64,
        ctx: &mut TxContext
    ) {
        // Verify profile has opted in for monetization
        assert!(table::contains(&manager.profile_settings, profile_id), EUserNotOptedIn);
        let profile_settings = table::borrow(&manager.profile_settings, profile_id);
        assert!(profile_settings.monetization_enabled, EUserNotOptedIn);
        
        // Verify usage type is allowed for this profile
        assert!(vector::contains(&profile_settings.allowed_usage_types, &usage_type), EUserNotOptedIn);
        
        // Determine payment amount based on monetization level
        let payment_amount = if (table::contains(&manager.agent_fee_overrides, agent_cap.agent_id)) {
            // Use agent-specific fee override if available
            let fee_override = table::borrow(&manager.agent_fee_overrides, agent_cap.agent_id);
            if (profile_settings.monetization_level == MONETIZATION_BASIC) {
                fee_override.basic_fee
            } else if (profile_settings.monetization_level == MONETIZATION_STANDARD) {
                fee_override.standard_fee
            } else {
                fee_override.premium_fee
            }
        } else {
            // Use default fees from config
            if (profile_settings.monetization_level == MONETIZATION_BASIC) {
                fee_config.basic_fee
            } else if (profile_settings.monetization_level == MONETIZATION_STANDARD) {
                fee_config.standard_fee
            } else {
                fee_config.premium_fee
            }
        };
        
        // Verify payment is sufficient
        assert!(coin::value(payment) >= payment_amount, EInsufficientBalance);
        
        // Determine revenue shares
        let (user_share, platform_share, mysocial_share) = if (
            table::contains(&manager.agent_fee_overrides, agent_cap.agent_id)
        ) {
            let fee_override = table::borrow(&manager.agent_fee_overrides, agent_cap.agent_id);
            (fee_override.user_share, fee_override.platform_share, fee_override.mysocial_share)
        } else {
            (fee_config.user_share, fee_config.platform_share, fee_config.mysocial_share)
        };
        
        // Calculate share amounts
        let user_amount = (payment_amount * user_share) / 100;
        let platform_amount = (payment_amount * platform_share) / 100;
        let mysocial_amount = payment_amount - user_amount - platform_amount;
        
        // Extract payment from coin
        let payment_balance = coin::extract(payment, payment_amount);
        
        // Distribute shares
        
        // User share
        let profile_settings = table::borrow_mut(&mut manager.profile_settings, profile_id);
        profile_settings.earned_revenue = profile_settings.earned_revenue + user_amount;
        profile_settings.total_earnings = profile_settings.total_earnings + user_amount;
        
        // Platform share
        if (!table::contains(&manager.platform_treasuries, platform_id)) {
            table::add(
                &mut manager.platform_treasuries,
                platform_id,
                PlatformTreasury {
                    platform_id,
                    balance: balance::zero(),
                    total_earnings: 0,
                }
            );
        };
        let platform_treasury = table::borrow_mut(&mut manager.platform_treasuries, platform_id);
        balance::join(&mut platform_treasury.balance, balance::split(&mut payment_balance, platform_amount));
        platform_treasury.total_earnings = platform_treasury.total_earnings + platform_amount;
        
        // MySocial share
        balance::join(&mut treasury.balance, balance::split(&mut payment_balance, mysocial_amount));
        
        // Add any remaining dust to MySocial balance (should be 0)
        if (balance::value(&payment_balance) > 0) {
            balance::join(&mut treasury.balance, payment_balance);
        } else {
            balance::destroy_zero(payment_balance);
        };
        
        // Update agent payment record
        if (!table::contains(&manager.agent_payments, agent_cap.agent_id)) {
            table::add(
                &mut manager.agent_payments,
                agent_cap.agent_id,
                AgentPaymentRecord {
                    agent_id: agent_cap.agent_id,
                    total_payments: 0,
                    usage_count: 0,
                    last_payment: 0,
                }
            );
        };
        let payment_record = table::borrow_mut(&mut manager.agent_payments, agent_cap.agent_id);
        payment_record.total_payments = payment_record.total_payments + payment_amount;
        payment_record.usage_count = payment_record.usage_count + 1;
        payment_record.last_payment = tx_context::epoch_timestamp_ms(ctx);
        
        // Update total earnings
        manager.total_earnings = manager.total_earnings + payment_amount;
        
        // Create data usage authorization token
        let current_time = tx_context::epoch_timestamp_ms(ctx);
        let expiration_time = current_time + (duration_hours * 3600000); // Convert hours to milliseconds
        
        let authorization = DataUsageAuthorization {
            id: object::new(ctx),
            profile_id,
            agent_id: agent_cap.agent_id,
            platform_id,
            monetization_level: profile_settings.monetization_level,
            allowed_usage_types: profile_settings.allowed_usage_types,
            payment_amount,
            creation_timestamp: current_time,
            expiration_timestamp: expiration_time,
        };
        
        // Emit event
        event::emit(DataUsagePaymentEvent {
            agent_id: agent_cap.agent_id,
            platform_id,
            profile_id,
            usage_type,
            monetization_level: profile_settings.monetization_level,
            payment_amount,
            user_share: user_amount,
            platform_share: platform_amount,
            mysocial_share: mysocial_amount,
            timestamp: current_time,
        });
        
        // Transfer authorization to agent owner
        transfer::transfer(authorization, tx_context::sender(ctx));
    }
    
    // === Earnings Withdrawal ===
    
    /// Withdraw user earnings
    public entry fun withdraw_user_earnings(
        manager: &mut DataMonetizationManager,
        profile: &Profile,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Verify caller owns the profile
        assert!(profile::owner(profile) == tx_context::sender(ctx), EUnauthorized);
        
        let profile_id = object::id(profile);
        
        // Verify profile has monetization settings
        assert!(table::contains(&manager.profile_settings, profile_id), EUserNotOptedIn);
        
        let settings = table::borrow_mut(&mut manager.profile_settings, profile_id);
        
        // Verify sufficient balance
        assert!(settings.earned_revenue >= amount, EWithdrawalExceedsBalance);
        
        // Deduct from earned revenue
        settings.earned_revenue = settings.earned_revenue - amount;
        
        // Create and transfer coin
        let coin = coin::from_value(amount, ctx);
        transfer::public_transfer(coin, profile::owner(profile));
        
        // Emit event
        event::emit(EarningsWithdrawalEvent {
            profile_id,
            amount,
            recipient: profile::owner(profile),
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }
    
    /// Withdraw platform earnings
    public entry fun withdraw_platform_earnings(
        manager: &mut DataMonetizationManager,
        platform: &Platform,
        amount: u64,
        ctx: &mut TxContext
    ) {
        // Verify caller is platform owner
        assert!(platform::get_owner(platform) == tx_context::sender(ctx), EUnauthorized);
        
        let platform_id = object::id(platform);
        
        // Verify platform has a treasury
        assert!(table::contains(&manager.platform_treasuries, platform_id), EInvalidFeeConfig);
        
        let platform_treasury = table::borrow_mut(&mut manager.platform_treasuries, platform_id);
        
        // Verify sufficient balance
        assert!(balance::value(&platform_treasury.balance) >= amount, EWithdrawalExceedsBalance);
        
        // Create coin from balance
        let withdraw_balance = balance::split(&mut platform_treasury.balance, amount);
        let coin = coin::from_balance(withdraw_balance, ctx);
        
        // Transfer to platform owner
        transfer::public_transfer(coin, platform::get_owner(platform));
    }
    
    // === Admin Functions ===
    
    /// Update fee configuration
    public entry fun update_fee_config(
        fee_config: &mut FeeConfig,
        basic_fee: u64,
        standard_fee: u64,
        premium_fee: u64,
        user_share: u64,
        platform_share: u64,
        mysocial_share: u64,
        ctx: &mut TxContext
    ) {
        // In a real implementation, we would verify the caller has admin privileges
        // Here, we're assuming the caller is authorized
        
        // Verify shares add up to 100%
        assert!(user_share + platform_share + mysocial_share == 100, EInvalidFeeConfig);
        
        // Update fee configuration
        fee_config.basic_fee = basic_fee;
        fee_config.standard_fee = standard_fee;
        fee_config.premium_fee = premium_fee;
        fee_config.user_share = user_share;
        fee_config.platform_share = platform_share;
        fee_config.mysocial_share = mysocial_share;
    }
    
    /// Set agent fee override
    public entry fun set_agent_fee_override(
        manager: &mut DataMonetizationManager,
        agent_cap: &AgentCap,
        basic_fee: u64,
        standard_fee: u64,
        premium_fee: u64,
        user_share: u64,
        platform_share: u64,
        mysocial_share: u64,
        ctx: &mut TxContext
    ) {
        // Verify caller owns the agent
        assert!(agent_cap.owner == tx_context::sender(ctx), EUnauthorized);
        
        // Verify shares add up to 100%
        assert!(user_share + platform_share + mysocial_share == 100, EInvalidFeeConfig);
        
        // Create or update fee override
        let fee_override = AgentFeeOverride {
            agent_id: agent_cap.agent_id,
            basic_fee,
            standard_fee,
            premium_fee,
            user_share,
            platform_share,
            mysocial_share,
        };
        
        if (table::contains(&manager.agent_fee_overrides, agent_cap.agent_id)) {
            let existing_override = table::borrow_mut(&mut manager.agent_fee_overrides, agent_cap.agent_id);
            *existing_override = fee_override;
        } else {
            table::add(&mut manager.agent_fee_overrides, agent_cap.agent_id, fee_override);
        };
    }
    
    // === Public Accessor Functions ===
    
    /// Check if a profile has opted in for monetization
    public fun is_monetization_enabled(
        manager: &DataMonetizationManager,
        profile_id: ID
    ): bool {
        if (!table::contains(&manager.profile_settings, profile_id)) {
            return false
        };
        
        table::borrow(&manager.profile_settings, profile_id).monetization_enabled
    }
    
    /// Get profile monetization settings
    public fun get_profile_monetization_settings(
        manager: &DataMonetizationManager,
        profile_id: ID
    ): (bool, bool, u8, vector<u8>, u64, u64) {
        if (!table::contains(&manager.profile_settings, profile_id)) {
            return (false, false, 0, vector::empty(), 0, 0)
        };
        
        let settings = table::borrow(&manager.profile_settings, profile_id);
        (
            true,
            settings.monetization_enabled,
            settings.monetization_level,
            settings.allowed_usage_types,
            settings.earned_revenue,
            settings.total_earnings
        )
    }
    
    /// Get agent payment record
    public fun get_agent_payment_record(
        manager: &DataMonetizationManager,
        agent_id: ID
    ): (bool, u64, u64, u64) {
        if (!table::contains(&manager.agent_payments, agent_id)) {
            return (false, 0, 0, 0)
        };
        
        let record = table::borrow(&manager.agent_payments, agent_id);
        (
            true,
            record.total_payments,
            record.usage_count,
            record.last_payment
        )
    }
    
    /// Get platform treasury info
    public fun get_platform_treasury_info(
        manager: &DataMonetizationManager,
        platform_id: ID
    ): (bool, u64, u64) {
        if (!table::contains(&manager.platform_treasuries, platform_id)) {
            return (false, 0, 0)
        };
        
        let treasury = table::borrow(&manager.platform_treasuries, platform_id);
        (
            true,
            balance::value(&treasury.balance),
            treasury.total_earnings
        )
    }
    
    /// Get fee for data usage
    public fun get_data_usage_fee(
        manager: &DataMonetizationManager,
        fee_config: &FeeConfig,
        agent_id: ID,
        monetization_level: u8
    ): u64 {
        if (table::contains(&manager.agent_fee_overrides, agent_id)) {
            // Use agent-specific fee override
            let fee_override = table::borrow(&manager.agent_fee_overrides, agent_id);
            if (monetization_level == MONETIZATION_BASIC) {
                fee_override.basic_fee
            } else if (monetization_level == MONETIZATION_STANDARD) {
                fee_override.standard_fee
            } else {
                fee_override.premium_fee
            }
        } else {
            // Use default fees from config
            if (monetization_level == MONETIZATION_BASIC) {
                fee_config.basic_fee
            } else if (monetization_level == MONETIZATION_STANDARD) {
                fee_config.standard_fee
            } else {
                fee_config.premium_fee
            }
        }
    }
    
    // === Monetization Constants ===
    
    /// Get basic monetization level constant
    public fun basic_monetization_level(): u8 {
        MONETIZATION_BASIC
    }
    
    /// Get standard monetization level constant
    public fun standard_monetization_level(): u8 {
        MONETIZATION_STANDARD
    }
    
    /// Get premium monetization level constant
    public fun premium_monetization_level(): u8 {
        MONETIZATION_PREMIUM
    }
    
    /// Get analytics usage type constant
    public fun analytics_usage_type(): u8 {
        USAGE_ANALYTICS
    }
    
    /// Get profile data usage type constant
    public fun profile_usage_type(): u8 {
        USAGE_PROFILE
    }
    
    /// Get content data usage type constant
    public fun content_usage_type(): u8 {
        USAGE_CONTENT
    }
    
    /// Get social graph data usage type constant
    public fun social_graph_usage_type(): u8 {
        USAGE_SOCIAL_GRAPH
    }
}