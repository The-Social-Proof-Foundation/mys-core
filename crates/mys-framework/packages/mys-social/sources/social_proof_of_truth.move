// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Social Proof of Truth (SPoT)
/// Prediction market for post truthfulness. Users bet on custom options (2-10 options per record).
/// All bets go directly to escrow. Oracle/DAO resolves the outcome, and winners receive
/// pro-rata payouts from the total escrow pool. Users can withdraw bets before resolution
/// with the same fee structure as payouts. Time-based resolution windows are optional per record.

#[allow(duplicate_alias, unused_use, unused_const, unused_variable, lint(self_transfer, share_owned))]
module social_contracts::social_proof_of_truth {
    use std::option::{Self, Option};
    use std::string::{Self, String};
    use std::vector;

    use mys::{
        object::{Self, UID},
        tx_context::{Self, TxContext},
        transfer,
        event,
        coin::{Self, Coin},
        balance::{Self, Balance},
        table::{Self, Table},
    };
    use mys::mys::MYS;

    use social_contracts::post::{Self, Post};
    use social_contracts::platform::{Self, Platform};
    use social_contracts::block_list::BlockListRegistry;
    use social_contracts::upgrade::{Self, UpgradeAdminCap};

    /// Errors
    const EDisabled: u64 = 1;
    const EInvalidAmount: u64 = 2;
    const EAlreadyResolved: u64 = 3;
    const ETooEarly: u64 = 4;
    const ETooClose: u64 = 5;
    const EWrongStatus: u64 = 6;
    const ENotOracle: u64 = 7;
    const ENoBets: u64 = 8;
    const EOverflow: u64 = 9;
    const EInvalidReasoning: u64 = 10;
    const EInvalidOptionId: u64 = 11;
    const EWithdrawalNotAllowed: u64 = 12;
    const EBetNotFound: u64 = 13;
    const EAlreadyInitialized: u64 = 14;
    const EDuplicateOption: u64 = 15;
    const ETooManyBets: u64 = 16;
    const EWrongVersion: u64 = 17;

    /// Status
    const STATUS_OPEN: u8 = 1;
    const STATUS_DAO_REQUIRED: u8 = 2;
    const STATUS_RESOLVED: u8 = 3;
    const STATUS_REFUNDABLE: u8 = 4;

    /// Outcomes
    /// Note: For multi-option betting, outcome is the winning option_id (0-indexed)
    /// Special outcomes: DRAW = 255, UNAPPLICABLE = 254
    const OUTCOME_DRAW: u8 = 255;
    const OUTCOME_UNAPPLICABLE: u8 = 254;

    /// Config defaults
    const DEFAULT_CONFIDENCE_THRESHOLD_BPS: u64 = 7000; // 70%
    const DEFAULT_ENABLE: bool = false;
    const DEFAULT_RESOLUTION_WINDOW_EPOCHS: u64 = 72; // depends on epoch length
    const DEFAULT_MAX_RESOLUTION_WINDOW_EPOCHS: u64 = 144;
    const DEFAULT_PAYOUT_DELAY_EPOCHS: u64 = 0;
    const DEFAULT_FEE_BPS: u64 = 100; // 1%
    const DEFAULT_FEE_SPLIT_PLATFORM_BPS: u64 = 5000; // 50% of fee to platform
    const DEFAULT_MAX_BETS_PER_RECORD: u64 = 10000; // Default maximum bets allowed per SPoT record

    /// Maximum u64 value for overflow protection
    const MAX_U64: u64 = 18446744073709551615;
    
    /// Validation constants
    const MAX_REASONING_LENGTH: u64 = 5000; // Max characters for reasoning
    const MAX_EVIDENCE_URLS: u64 = 10; // Max number of evidence URLs
    const MIN_REASONING_LENGTH: u64 = 10; // Minimum characters for reasoning
    const MAX_BETTING_OPTIONS: u64 = 10; // Maximum number of betting options per record
    const MIN_BETTING_OPTIONS: u64 = 2; // Minimum number of betting options per record

    /// Admin capability for SPoT (controls SpotConfig updates)
    public struct SpotAdminCap has key, store { id: UID }

    /// Oracle admin capability for SPoT (controls oracle decisions: record creation and resolution)
    public struct SpotOracleAdminCap has key, store { id: UID }

    /// Global configuration for SPoT
    public struct SpotConfig has key {
        id: UID,
        enable_flag: bool,
        confidence_threshold_bps: u64,
        resolution_window_epochs: u64,
        max_resolution_window_epochs: u64,
        payout_delay_epochs: u64,
        fee_bps: u64,
        fee_split_bps_platform: u64,
        platform_treasury: address,
        chain_treasury: address,
        oracle_address: address,
        max_single_bet: u64,
        max_bets_per_record: u64,
        version: u64,
    }

    /// A single bet
    public struct SpotBet has store, copy, drop {
        user: address,
        option_id: u8,
        amount: u64,
        timestamp: u64,
    }

    /// SPoT record per post
    public struct SpotRecord has key, store {
        id: UID,
        post_id: address,
        created_epoch: u64,
        status: u8,
        outcome: Option<u8>,
        escrow: Balance<MYS>,
        betting_options: vector<String>,
        option_escrow: Table<u8, u64>,
        user_option_amounts: Table<address, vector<u64>>,
        bets: vector<SpotBet>,
        resolution_window_epochs: Option<u64>,
        max_resolution_window_epochs: Option<u64>,
        last_resolution_epoch: u64,
        version: u64,
    }

    /// Events
    public struct SpotBetPlacedEvent has copy, drop {
        post_id: address,
        user: address,
        option_id: u8,
        amount: u64,
    }

    public struct SpotResolvedEvent has copy, drop {
        post_id: address,
        outcome: u8, // Winning option_id, or OUTCOME_DRAW/OUTCOME_UNAPPLICABLE
        total_escrow: u64,
        fee_taken: u64,
        reasoning: String, // Required reasoning from oracle
        evidence_urls: vector<String>, // Required array of evidence URLs (at least 1)
    }

    public struct SpotDaoRequiredEvent has copy, drop {
        post_id: address,
        confidence_bps: u64,
        reasoning: String, // Required reasoning why DAO is needed
    }

    public struct SpotPayoutEvent has copy, drop {
        post_id: address,
        user: address,
        amount: u64,
    }

    public struct SpotRefundEvent has copy, drop {
        post_id: address,
        user: address,
        amount: u64,
    }

    public struct SpotConfigUpdatedEvent has copy, drop {
        updated_by: address,
        enable_flag: bool,
        confidence_threshold_bps: u64,
        resolution_window_epochs: u64,
        max_resolution_window_epochs: u64,
        payout_delay_epochs: u64,
        fee_bps: u64,
        fee_split_bps_platform: u64,
        platform_treasury: address,
        chain_treasury: address,
        oracle_address: address,
        max_single_bet: u64,
        max_bets_per_record: u64,
        timestamp: u64,
    }

    public struct SpotBetWithdrawnEvent has copy, drop {
        post_id: address,
        user: address,
        option_id: u8,
        amount: u64,
        fee_taken: u64,
    }

    public struct SpotRecordCreatedEvent has copy, drop {
        record_id: address,
        post_id: address,
        created_epoch: u64,
        betting_options: vector<String>,
        resolution_window_epochs: Option<u64>,
        max_resolution_window_epochs: Option<u64>,
    }

    // Public getters for testing/inspection
    public fun get_status(rec: &SpotRecord): u8 { rec.status }
    public fun get_bets_len(rec: &SpotRecord): u64 { vector::length(&rec.bets) }
    public fun get_betting_options(rec: &SpotRecord): vector<String> { rec.betting_options }
    public fun get_option_escrow(rec: &SpotRecord, option_id: u8): u64 {
        if (table::contains(&rec.option_escrow, option_id)) {
            *table::borrow(&rec.option_escrow, option_id)
        } else {
            0
        }
    }
    public fun get_id_address(rec: &SpotRecord): address {
        object::uid_to_address(&rec.id)
    }
    public fun get_outcome(rec: &SpotRecord): &Option<u8> { &rec.outcome }
    public fun is_open(rec: &SpotRecord): bool { rec.status == STATUS_OPEN }
    public fun is_resolved(rec: &SpotRecord): bool { rec.status == STATUS_RESOLVED }
    public fun outcome_draw(): u8 { OUTCOME_DRAW }
    public fun outcome_unapplicable(): u8 { OUTCOME_UNAPPLICABLE }
    public fun get_user_option_amount(rec: &SpotRecord, user: address, option_id: u8): u64 {
        if (!table::contains(&rec.user_option_amounts, user)) {
            0
        } else {
            let amounts = table::borrow(&rec.user_option_amounts, user);
            let idx = option_id as u64;
            if (idx >= vector::length(amounts)) {
                0
            } else {
                *vector::borrow(amounts, idx)
            }
        }
    }

    // Public getter for SpotConfig
    public fun is_enabled(config: &SpotConfig): bool { config.enable_flag }

    // Bootstrap
    public(package) fun bootstrap_init(ctx: &mut TxContext) {
        let admin = tx_context::sender(ctx);
        transfer::share_object(SpotConfig {
            id: object::new(ctx),
            enable_flag: DEFAULT_ENABLE,
            confidence_threshold_bps: DEFAULT_CONFIDENCE_THRESHOLD_BPS,
            resolution_window_epochs: DEFAULT_RESOLUTION_WINDOW_EPOCHS,
            max_resolution_window_epochs: DEFAULT_MAX_RESOLUTION_WINDOW_EPOCHS,
            payout_delay_epochs: DEFAULT_PAYOUT_DELAY_EPOCHS,
            fee_bps: DEFAULT_FEE_BPS,
            fee_split_bps_platform: DEFAULT_FEE_SPLIT_PLATFORM_BPS,
            platform_treasury: admin,
            chain_treasury: admin,
            oracle_address: admin,
            max_single_bet: 0,
            max_bets_per_record: DEFAULT_MAX_BETS_PER_RECORD,
            version: upgrade::current_version(),
        });
    }

    /// Create a SpotAdminCap for bootstrap (package visibility only)
    public(package) fun create_spot_admin_cap(ctx: &mut TxContext): SpotAdminCap {
        SpotAdminCap {
            id: object::new(ctx)
        }
    }

    /// Create a SpotOracleAdminCap for bootstrap (package visibility only)
    public(package) fun create_spot_oracle_admin_cap(ctx: &mut TxContext): SpotOracleAdminCap {
        SpotOracleAdminCap {
            id: object::new(ctx)
        }
    }

    #[test_only]
    /// Initialize SPoT for testing - creates admin caps and config
    public fun test_init(ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        
        // Create and share config
        transfer::share_object(SpotConfig {
            id: object::new(ctx),
            enable_flag: DEFAULT_ENABLE,
            confidence_threshold_bps: DEFAULT_CONFIDENCE_THRESHOLD_BPS,
            resolution_window_epochs: DEFAULT_RESOLUTION_WINDOW_EPOCHS,
            max_resolution_window_epochs: DEFAULT_MAX_RESOLUTION_WINDOW_EPOCHS,
            payout_delay_epochs: DEFAULT_PAYOUT_DELAY_EPOCHS,
            fee_bps: DEFAULT_FEE_BPS,
            fee_split_bps_platform: DEFAULT_FEE_SPLIT_PLATFORM_BPS,
            platform_treasury: sender,
            chain_treasury: sender,
            oracle_address: sender,
            max_single_bet: 0,
            max_bets_per_record: DEFAULT_MAX_BETS_PER_RECORD,
            version: upgrade::current_version(),
        });
        
        // Create and transfer admin capabilities to the transaction sender
        transfer::public_transfer(SpotAdminCap { id: object::new(ctx) }, sender);
        transfer::public_transfer(SpotOracleAdminCap { id: object::new(ctx) }, sender);
    }

    /// Update SPoT configuration (admin only)
    public entry fun update_spot_config(
        _: &SpotAdminCap,
        config: &mut SpotConfig,
        enable_flag: bool,
        confidence_threshold_bps: u64,
        resolution_window_epochs: u64,
        max_resolution_window_epochs: u64,
        payout_delay_epochs: u64,
        fee_bps: u64,
        fee_split_bps_platform: u64,
        platform_treasury: address,
        chain_treasury: address,
        oracle_address: address,
        max_single_bet: u64,
        max_bets_per_record: u64,
        ctx: &mut TxContext
    ) {
        // Basic bounds
        assert!(confidence_threshold_bps <= 10000, EInvalidAmount);
        // windows may be zero in tests to resolve immediately

        config.enable_flag = enable_flag;
        config.confidence_threshold_bps = confidence_threshold_bps;
        config.resolution_window_epochs = resolution_window_epochs;
        config.max_resolution_window_epochs = max_resolution_window_epochs;
        config.payout_delay_epochs = payout_delay_epochs;
        config.fee_bps = fee_bps;
        config.fee_split_bps_platform = fee_split_bps_platform;
        config.platform_treasury = platform_treasury;
        config.chain_treasury = chain_treasury;
        config.oracle_address = oracle_address;
        config.max_single_bet = max_single_bet;
        config.max_bets_per_record = max_bets_per_record;
        
        // Emit config updated event
        event::emit(SpotConfigUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            enable_flag,
            confidence_threshold_bps,
            resolution_window_epochs,
            max_resolution_window_epochs,
            payout_delay_epochs,
            fee_bps,
            fee_split_bps_platform,
            platform_treasury,
            chain_treasury,
            oracle_address,
            max_single_bet,
            max_bets_per_record,
            timestamp: tx_context::epoch_timestamp_ms(ctx),
        });
    }

    // Create a SPoT record for a post
    public entry fun create_spot_record_for_post(
        _: &SpotOracleAdminCap,
        config: &SpotConfig,
        post: &mut Post,
        betting_options: vector<String>,
        resolution_window_epochs: Option<u64>,
        max_resolution_window_epochs: Option<u64>,
        ctx: &mut TxContext
    ) {
        assert!(config.enable_flag, EDisabled);
        
        // Verify SPoT is enabled for this post
        assert!(social_contracts::post::is_spot_enabled(post), EDisabled);
        
        // Validate betting options
        let options_len = vector::length(&betting_options);
        assert!(options_len >= MIN_BETTING_OPTIONS, EInvalidAmount);
        assert!(options_len <= MAX_BETTING_OPTIONS, EInvalidAmount);
        
        // Check for duplicate options (case-sensitive comparison)
        let mut i = 0;
        while (i < options_len) {
            let option_i = vector::borrow(&betting_options, i);
            let mut j = i + 1;
            while (j < options_len) {
                let option_j = vector::borrow(&betting_options, j);
                assert!(*option_i != *option_j, EDuplicateOption);
                j = j + 1;
            };
            i = i + 1;
        };
        
        let record = SpotRecord {
            id: object::new(ctx),
            post_id: post::get_id_address(post),
            created_epoch: tx_context::epoch(ctx),
            status: STATUS_OPEN,
            outcome: option::none(),
            escrow: balance::zero(),
            betting_options,
            option_escrow: table::new(ctx),
            user_option_amounts: table::new(ctx),
            bets: vector::empty<SpotBet>(),
            resolution_window_epochs,
            max_resolution_window_epochs,
            last_resolution_epoch: 0,
            version: upgrade::current_version(),
        };
        let record_id = object::uid_to_address(&record.id);
        let created_epoch = record.created_epoch;
        let post_id = record.post_id;
        let betting_options_copy = record.betting_options;
        let resolution_window = record.resolution_window_epochs;
        let max_resolution_window = record.max_resolution_window_epochs;
        
        // Store SPoT record ID in post
        social_contracts::post::set_spot_id(post, record_id);
        
        transfer::share_object(record);
        
        // Emit record created event
        event::emit(SpotRecordCreatedEvent {
            record_id,
            post_id,
            created_epoch,
            betting_options: betting_options_copy,
            resolution_window_epochs: resolution_window,
            max_resolution_window_epochs: max_resolution_window,
        });
    }

    /// Withdraw a bet before resolution
    /// Applies same fee structure as payouts
    /// Only allowed when status is OPEN (not DAO_REQUIRED, not RESOLVED, not REFUNDABLE)
    public entry fun withdraw_spot_bet(
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        bet_index: u64,
        ctx: &mut TxContext
    ) {
        assert!(spot_config.enable_flag, EDisabled);
        // Only allow withdrawal when status is OPEN (not DAO_REQUIRED or RESOLVED)
        assert!(record.status == STATUS_OPEN, EWithdrawalNotAllowed);
        
        let bets_len = vector::length(&record.bets);
        assert!(bet_index < bets_len, EBetNotFound);
        
        // Copy bet data before mutating vector
        let bet = *vector::borrow(&record.bets, bet_index);
        assert!(bet.user == tx_context::sender(ctx), EInvalidAmount); // User must own the bet
        assert!(bet.amount > 0, EInvalidAmount);
        
        // Calculate fee (same as payout fee structure)
        let mut fee = 0;
        if (spot_config.fee_bps > 0) {
            fee = (bet.amount * spot_config.fee_bps) / 10000;
        };
        let refund_amount = bet.amount - fee;
        
        // Split fee between platform and chain treasuries
        if (fee > 0) {
            let platform_part = (fee * spot_config.fee_split_bps_platform) / 10000;
            let chain_part = fee - platform_part;
            let mut fee_coin = coin::from_balance(balance::split(&mut record.escrow, fee), ctx);
            let platform_coin = coin::split(&mut fee_coin, platform_part, ctx);
            transfer::public_transfer(platform_coin, spot_config.platform_treasury);
            transfer::public_transfer(fee_coin, spot_config.chain_treasury);
        };
        
        // Refund remaining amount to user
        if (refund_amount > 0) {
            let refund_coin = coin::from_balance(balance::split(&mut record.escrow, refund_amount), ctx);
            transfer::public_transfer(refund_coin, bet.user);
        };
        
        // Update option escrow table
        let option_id = bet.option_id;
        if (table::contains(&record.option_escrow, option_id)) {
            let current_escrow = *table::borrow(&record.option_escrow, option_id);
            if (current_escrow >= bet.amount) {
                let escrow_ref = table::borrow_mut(&mut record.option_escrow, option_id);
                *escrow_ref = current_escrow - bet.amount;
            };
        };

        if (table::contains(&record.user_option_amounts, bet.user)) {
            let user_amounts = table::borrow_mut(&mut record.user_option_amounts, bet.user);
            let idx = bet.option_id as u64;
            if (idx < vector::length(user_amounts)) {
                let current_user_amount = *vector::borrow(user_amounts, idx);
                if (current_user_amount >= bet.amount) {
                    let user_amount_ref = vector::borrow_mut(user_amounts, idx);
                    *user_amount_ref = current_user_amount - bet.amount;
                };
            };
        };
        
        // Remove bet from vector (swap with last and pop)
        let last_index = bets_len - 1;
        if (bet_index != last_index) {
            let last_bet = *vector::borrow(&record.bets, last_index);
            let bet_ref = vector::borrow_mut(&mut record.bets, bet_index);
            *bet_ref = last_bet;
        };
        vector::pop_back(&mut record.bets);
        
        // Emit withdrawal event
        event::emit(SpotBetWithdrawnEvent {
            post_id: post::get_id_address(post),
            user: bet.user,
            option_id: bet.option_id,
            amount: bet.amount,
            fee_taken: fee,
        });
    }

    /// Place bet - all funds go to escrow
    public entry fun place_spot_bet(
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        mut payment: Coin<MYS>,
        option_id: u8,
        amount: u64,
        ctx: &mut TxContext
    ) {
        assert!(spot_config.enable_flag, EDisabled);
        assert!(amount > 0, EInvalidAmount);
        if (spot_config.max_single_bet > 0) { assert!(amount <= spot_config.max_single_bet, EInvalidAmount); };
        assert!(coin::value(&payment) >= amount, EInvalidAmount);
        
        // Check bet limit
        let current_bets = vector::length(&record.bets);
        assert!(current_bets < spot_config.max_bets_per_record, ETooManyBets);
        
        // Validate option_id exists
        let options_len = vector::length(&record.betting_options);
        assert!((option_id as u64) < options_len, EInvalidOptionId);

        // All funds go to escrow
        let bet_coin = coin::split(&mut payment, amount, ctx);
        balance::join(&mut record.escrow, coin::into_balance(bet_coin));

        // Update escrow totals with overflow protection
        let current_escrow = if (table::contains(&record.option_escrow, option_id)) {
            *table::borrow(&record.option_escrow, option_id)
        } else {
            0
        };
        assert!(current_escrow <= MAX_U64 - amount, EOverflow);
        if (table::contains(&record.option_escrow, option_id)) {
            let escrow_ref = table::borrow_mut(&mut record.option_escrow, option_id);
            *escrow_ref = current_escrow + amount;
        } else {
            table::add(&mut record.option_escrow, option_id, amount);
        };

        // Refund any excess
        if (coin::value(&payment) > 0) { 
            transfer::public_transfer(payment, tx_context::sender(ctx)); 
        } else { 
            coin::destroy_zero(payment); 
        };

        // Record bet
        vector::push_back(&mut record.bets, SpotBet {
            user: tx_context::sender(ctx),
            option_id,
            amount,
            timestamp: tx_context::epoch(ctx),
        });

        let user = tx_context::sender(ctx);
        let options_len = vector::length(&record.betting_options);
        if (!table::contains(&record.user_option_amounts, user)) {
            let mut amounts = vector::empty<u64>();
            let mut i = 0;
            while (i < options_len) {
                vector::push_back(&mut amounts, 0);
                i = i + 1;
            };
            table::add(&mut record.user_option_amounts, user, amounts);
        };
        let user_amounts = table::borrow_mut(&mut record.user_option_amounts, user);
        let idx = option_id as u64;
        let current_user_amount = *vector::borrow(user_amounts, idx);
        assert!(current_user_amount <= MAX_U64 - amount, EOverflow);
        let user_amount_ref = vector::borrow_mut(user_amounts, idx);
        *user_amount_ref = current_user_amount + amount;

        event::emit(SpotBetPlacedEvent {
            post_id: post::get_id_address(post),
            user: tx_context::sender(ctx),
            option_id,
            amount,
        });
    }

    /// Oracle resolution (option_id, or too close → DAO_REQUIRED)
    /// Requires reasoning and at least one evidence URL for transparency and accountability
    public entry fun oracle_resolve(
        _: &SpotOracleAdminCap,
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        outcome_option_id: u8,
        confidence_bps: u64,
        reasoning: String,
        evidence_urls: vector<String>,
        ctx: &mut TxContext
    ) {
        // Prevent resolving already resolved or refundable markets
        assert!(record.status == STATUS_OPEN, EWrongStatus);
        assert!(option::is_none(&record.outcome), EAlreadyResolved);
        
        // Enforce resolution window if specified
        let now = tx_context::epoch(ctx);
        if (option::is_some(&record.resolution_window_epochs)) {
            let window = *option::borrow(&record.resolution_window_epochs);
            assert!(now >= record.created_epoch + window, ETooEarly);
        };
        
        // Validate outcome_option_id exists
        let options_len = vector::length(&record.betting_options);
        assert!((outcome_option_id as u64) < options_len, EInvalidOptionId);

        // Validate reasoning is required and within limits
        let reasoning_len = string::length(&reasoning);
        assert!(reasoning_len >= MIN_REASONING_LENGTH, EInvalidReasoning);
        assert!(reasoning_len <= MAX_REASONING_LENGTH, EInvalidReasoning);
        
        // Validate evidence URLs - at least one required
        let evidence_urls_len = vector::length(&evidence_urls);
        assert!(evidence_urls_len > 0, EInvalidAmount); // At least one evidence URL required
        assert!(evidence_urls_len <= MAX_EVIDENCE_URLS, EInvalidAmount);

        if (confidence_bps < spot_config.confidence_threshold_bps) {
            record.status = STATUS_DAO_REQUIRED;
            event::emit(SpotDaoRequiredEvent { 
                post_id: post::get_id_address(post), 
                confidence_bps,
                reasoning,
            });
            return
        };

        // Resolve outcome - outcome_option_id is the winning option
        // Convert required vector to Option for internal function
        finalize_resolution_and_payout(spot_config, record, post, outcome_option_id, reasoning, option::some(evidence_urls), ctx);
    }

    /// DAO finalization (YES/NO/DRAW/UNAPPLICABLE)
    /// Reasoning is optional as it represents culmination of community discussion
    public entry fun finalize_via_dao(
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        outcome: u8,
        mut reasoning: Option<String>,
        evidence_urls: Option<vector<String>>,
        ctx: &mut TxContext
    ) {
        // Allow when DAO_REQUIRED or still OPEN (off-chain DAO direct)
        assert!(record.status == STATUS_DAO_REQUIRED || record.status == STATUS_OPEN, EWrongStatus);
        // Prevent resolving already resolved markets
        assert!(option::is_none(&record.outcome), EAlreadyResolved);
        
        // Validate reasoning if provided
        if (option::is_some(&reasoning)) {
            let reasoning_val = option::borrow(&reasoning);
            let reasoning_len = string::length(reasoning_val);
            assert!(reasoning_len <= MAX_REASONING_LENGTH, EInvalidReasoning);
        };
        
        // Validate evidence URLs if provided
        if (option::is_some(&evidence_urls)) {
            let urls = option::borrow(&evidence_urls);
            assert!(vector::length(urls) <= MAX_EVIDENCE_URLS, EInvalidAmount);
        };
        
        // Use provided reasoning or default message if not provided
        let final_reasoning = if (option::is_some(&reasoning)) {
            option::extract(&mut reasoning)
        } else {
            string::utf8(b"DAO resolution based on community discussion")
        };
        
        finalize_resolution_and_payout(spot_config, record, post, outcome, final_reasoning, evidence_urls, ctx);
    }

    /// Refund all escrow if unresolved beyond max window
    /// Requires SpotOracleAdminCap authorization
    /// If max_resolution_window_epochs is None, this function cannot be called (must be explicitly set)
    public entry fun refund_unresolved(
        _: &SpotOracleAdminCap,
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        ctx: &mut TxContext
    ) {
        // Require max_resolution_window_epochs to be Some - prevents permissionless abuse
        assert!(option::is_some(&record.max_resolution_window_epochs), EInvalidAmount);
        
        let now = tx_context::epoch(ctx);
        let max_window = *option::borrow(&record.max_resolution_window_epochs);
        assert!(now >= record.created_epoch + max_window, ETooEarly);
        
        assert!(record.status == STATUS_OPEN || record.status == STATUS_DAO_REQUIRED, EWrongStatus);
        assert!(vector::length(&record.bets) > 0, ENoBets);

        // Iterate all bets and refund escrow
        let mut i = 0;
        let len = vector::length(&record.bets);
        while (i < len) {
            let bet = vector::borrow(&record.bets, i);
            if (bet.amount > 0) {
                let c = coin::from_balance(balance::split(&mut record.escrow, bet.amount), ctx);
                transfer::public_transfer(c, bet.user);
                event::emit(SpotRefundEvent { post_id: record.post_id, user: bet.user, amount: bet.amount });
            };
            i = i + 1;
        };
        record.status = STATUS_REFUNDABLE;
        record.outcome = option::none();
        record.last_resolution_epoch = now;
        // Any dust stays in escrow balance if math rounding occurred
    }

    // Internal: finalize with payouts and fees
    fun finalize_resolution_and_payout(
        spot_config: &SpotConfig,
        record: &mut SpotRecord,
        post: &Post,
        outcome: u8, // Winning option_id, or OUTCOME_DRAW/OUTCOME_UNAPPLICABLE
        reasoning: String,
        evidence_urls: Option<vector<String>>,
        ctx: &mut TxContext
    ) {
        assert!(record.status == STATUS_OPEN || record.status == STATUS_DAO_REQUIRED, EWrongStatus);
        assert!(vector::length(&record.bets) > 0, ENoBets);

        // Calculate total escrow across all options
        let mut total_escrow = 0;
        let mut i = 0;
        let options_len = vector::length(&record.betting_options);
        while (i < options_len) {
            let option_id = (i as u8);
            if (table::contains(&record.option_escrow, option_id)) {
                total_escrow = total_escrow + *table::borrow(&record.option_escrow, option_id);
            };
            i = i + 1;
        };

        // Handle DRAW/UNAPPLICABLE: refund all escrow
        if (outcome == OUTCOME_DRAW || outcome == OUTCOME_UNAPPLICABLE) {
            let mut i = 0; let len = vector::length(&record.bets);
            while (i < len) {
                let bet = vector::borrow(&record.bets, i);
                if (bet.amount > 0) {
                    let c = coin::from_balance(balance::split(&mut record.escrow, bet.amount), ctx);
                    transfer::public_transfer(c, bet.user);
                    event::emit(SpotRefundEvent { post_id: record.post_id, user: bet.user, amount: bet.amount });
                };
                i = i + 1;
            };
            record.status = STATUS_RESOLVED;
            record.outcome = option::some(outcome);
            record.last_resolution_epoch = tx_context::epoch(ctx);
            // Convert Option to vector for event (use empty vector if None)
            let evidence_urls_vec = if (option::is_some(&evidence_urls)) {
                *option::borrow(&evidence_urls)
            } else {
                vector::empty<String>()
            };
            event::emit(SpotResolvedEvent { 
                post_id: post::get_id_address(post), 
                outcome, 
                total_escrow, 
                fee_taken: 0,
                reasoning,
                evidence_urls: evidence_urls_vec,
            });
            return
        };

        // Get winning option escrow total
        let winning_total = if (table::contains(&record.option_escrow, outcome)) {
            *table::borrow(&record.option_escrow, outcome)
        } else {
            0
        };

        // Fees on payouts (apply to total escrow)
        let mut fee = 0;
        if (spot_config.fee_bps > 0) { fee = (total_escrow * spot_config.fee_bps) / 10000; };
        let distributable = total_escrow - fee;

        // Split fee 50/50 (configurable)
        if (fee > 0) {
            let platform_part = (fee * spot_config.fee_split_bps_platform) / 10000;
            let chain_part = fee - platform_part;
            let mut fee_coin = coin::from_balance(balance::split(&mut record.escrow, fee), ctx);
            let platform_coin = coin::split(&mut fee_coin, platform_part, ctx);
            transfer::public_transfer(platform_coin, spot_config.platform_treasury);
            transfer::public_transfer(fee_coin, spot_config.chain_treasury);
        };

        // Distribute to winners pro-rata of winning option escrow
        let mut i = 0; let len = vector::length(&record.bets);
        while (i < len) {
            let bet = vector::borrow(&record.bets, i);
            let winner = bet.option_id == outcome;
            if (winner && winning_total > 0 && bet.amount > 0) {
                let payout = (((bet.amount as u128) * (distributable as u128)) / (winning_total as u128)) as u64;
                if (payout > 0) {
                    let c = coin::from_balance(balance::split(&mut record.escrow, payout), ctx);
                    transfer::public_transfer(c, bet.user);
                    event::emit(SpotPayoutEvent { post_id: record.post_id, user: bet.user, amount: payout });
                };
            };
            i = i + 1;
        };

        record.status = STATUS_RESOLVED;
        record.outcome = option::some(outcome);
        record.last_resolution_epoch = tx_context::epoch(ctx);
        // Convert Option to vector for event (use empty vector if None)
        let evidence_urls_vec = if (option::is_some(&evidence_urls)) {
            *option::borrow(&evidence_urls)
        } else {
            vector::empty<String>()
        };
        event::emit(SpotResolvedEvent { 
            post_id: post::get_id_address(post), 
            outcome, 
            total_escrow, 
            fee_taken: fee,
            reasoning,
            evidence_urls: evidence_urls_vec,
        });
    }

    /// Migration function for SpotConfig
    public entry fun migrate_config(
        config: &mut SpotConfig,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(config.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = config.version;
        config.version = current_version;
        
        // Emit event for object migration
        let config_id = object::id(config);
        upgrade::emit_migration_event(
            config_id,
            string::utf8(b"SpotConfig"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    /// Migration function for SpotRecord
    public entry fun migrate_record(
        record: &mut SpotRecord,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(record.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = record.version;
        record.version = current_version;
        
        // Emit event for object migration
        let record_id = object::id(record);
        upgrade::emit_migration_event(
            record_id,
            string::utf8(b"SpotRecord"),
            old_version,
            tx_context::sender(ctx)
        );
    }
}
