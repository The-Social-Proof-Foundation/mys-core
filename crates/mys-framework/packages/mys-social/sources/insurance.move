// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Insurance module for SPoT positions
/// Sells coverage against losing outcomes and pays out deterministically on SPoT resolution.

#[allow(duplicate_alias, unused_use, unused_const, unused_variable, lint(self_transfer, share_owned))]
module social_contracts::insurance {
    use std::option::{Self, Option};
    use std::vector;

    use mys::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        clock::{Self, Clock},
        coin::{Self, Coin},
        balance::{Self, Balance},
        table::{Self, Table},
        event,
    };
    use mys::mys::MYS;

    use social_contracts::social_proof_of_truth as spot;

    /// Errors
    const ENotAdmin: u64 = 1;
    const EPaused: u64 = 2;
    const EInvalidCoverage: u64 = 3;
    const EInvalidDuration: u64 = 4;
    const EInvalidAmount: u64 = 5;
    const EInvalidVault: u64 = 6;
    const EInsufficientCapital: u64 = 7;
    const EMarketClosed: u64 = 8;
    const EPolicyNotActive: u64 = 9;
    const EPolicyExpired: u64 = 10;
    const ENotPolicyOwner: u64 = 11;
    const EOverflow: u64 = 12;
    const EMarketMismatch: u64 = 13;
    const EExposureLimit: u64 = 14;
    const EInsufficientPremium: u64 = 15;

    /// Status
    const STATUS_ACTIVE: u8 = 1;
    const STATUS_CANCELLED: u8 = 2;
    const STATUS_CLAIMED: u8 = 3;
    const STATUS_EXPIRED: u8 = 4;

    /// Constants
    const BPS_DENOM: u64 = 10_000;
    const DAY_MS: u64 = 86_400_000;
    const MAX_U64: u64 = 18446744073709551615;
    const DEFAULT_VERSION: u64 = 1;
    const DEFAULT_MIN_COVERAGE_BPS: u64 = 1000;
    const DEFAULT_MAX_COVERAGE_BPS: u64 = 9000;
    const DEFAULT_MAX_DURATION_MS: u64 = 30 * DAY_MS;
    const DEFAULT_FEE_BPS: u64 = 50;

    public struct InsuranceAdminCap has key, store {
        id: UID,
    }

    public struct InsuranceConfig has key {
        id: UID,
        paused: bool,
        min_coverage_bps: u64,
        max_coverage_bps: u64,
        max_duration_ms: u64,
        fee_bps: u64,
        treasury: address,
        version: u64,
    }

    public struct UnderwriterVault has key {
        id: UID,
        underwriter: address,
        capital: Balance<MYS>,
        reserved: u64,
        base_rate_bps_per_day: u64,
        utilization_multiplier_bps: u64,
        max_exposure_per_market: u64,
        max_exposure_per_user: u64,
        market_exposures: Table<address, MarketExposure>,
        user_exposures: Table<address, u64>,
        version: u64,
    }

    public struct MarketExposure has store, drop {
        market_id: address,
        total_reserved: u64,
        reserved_by_option: Table<u8, u64>,
    }

    public struct CoveragePolicy has key {
        id: UID,
        market_id: address,
        insured: address,
        option_id: u8,
        covered_amount: u64,
        coverage_bps: u64,
        premium_paid: u64,
        start_time_ms: u64,
        expiry_time_ms: u64,
        vault_id: ID,
        status: u8,
    }

    /// Events
    public struct ConfigInitializedEvent has copy, drop {
        admin: address,
        min_coverage_bps: u64,
        max_coverage_bps: u64,
        max_duration_ms: u64,
        fee_bps: u64,
        treasury: address,
    }

    public struct VaultCreatedEvent has copy, drop {
        vault_id: ID,
        underwriter: address,
        base_rate_bps_per_day: u64,
        utilization_multiplier_bps: u64,
        max_exposure_per_market: u64,
        max_exposure_per_user: u64,
    }

    public struct VaultDepositedEvent has copy, drop {
        vault_id: ID,
        amount: u64,
        new_balance: u64,
    }

    public struct VaultWithdrawnEvent has copy, drop {
        vault_id: ID,
        amount: u64,
        new_balance: u64,
    }

    public struct CoveragePurchasedEvent has copy, drop {
        policy_id: ID,
        market_id: address,
        insured: address,
        option_id: u8,
        covered_amount: u64,
        coverage_bps: u64,
        premium_paid: u64,
        reserve_locked: u64,
        expiry_time_ms: u64,
    }

    public struct CoverageCancelledEvent has copy, drop {
        policy_id: ID,
        insured: address,
        refunded_amount: u64,
        fee_paid: u64,
    }

    public struct CoverageClaimedEvent has copy, drop {
        policy_id: ID,
        insured: address,
        payout: u64,
    }

    /// Initialize config
    public entry fun init_config(
        min_coverage_bps: u64,
        max_coverage_bps: u64,
        max_duration_ms: u64,
        fee_bps: u64,
        treasury: address,
        ctx: &mut TxContext
    ) {
        assert!(min_coverage_bps > 0, EInvalidCoverage);
        assert!(min_coverage_bps <= max_coverage_bps, EInvalidCoverage);
        assert!(max_coverage_bps <= BPS_DENOM, EInvalidCoverage);
        assert!(max_duration_ms > 0, EInvalidDuration);
        assert!(fee_bps <= BPS_DENOM, EInvalidCoverage);

        let admin = tx_context::sender(ctx);
        transfer::share_object(InsuranceConfig {
            id: object::new(ctx),
            paused: false,
            min_coverage_bps,
            max_coverage_bps,
            max_duration_ms,
            fee_bps,
            treasury,
            version: DEFAULT_VERSION,
        });
        transfer::public_transfer(InsuranceAdminCap { id: object::new(ctx) }, admin);

        event::emit(ConfigInitializedEvent {
            admin,
            min_coverage_bps,
            max_coverage_bps,
            max_duration_ms,
            fee_bps,
            treasury,
        });
    }

    /// Update config (admin only)
    public entry fun set_config(
        _: &InsuranceAdminCap,
        config: &mut InsuranceConfig,
        min_coverage_bps: u64,
        max_coverage_bps: u64,
        max_duration_ms: u64,
        fee_bps: u64,
        treasury: address,
        ctx: &mut TxContext
    ) {
        assert!(min_coverage_bps > 0, EInvalidCoverage);
        assert!(min_coverage_bps <= max_coverage_bps, EInvalidCoverage);
        assert!(max_coverage_bps <= BPS_DENOM, EInvalidCoverage);
        assert!(max_duration_ms > 0, EInvalidDuration);
        assert!(fee_bps <= BPS_DENOM, EInvalidCoverage);

        config.min_coverage_bps = min_coverage_bps;
        config.max_coverage_bps = max_coverage_bps;
        config.max_duration_ms = max_duration_ms;
        config.fee_bps = fee_bps;
        config.treasury = treasury;
        let _ = ctx;
    }

    /// Emergency pause toggle (admin only)
    public entry fun set_paused(
        _: &InsuranceAdminCap,
        config: &mut InsuranceConfig,
        paused: bool
    ) {
        config.paused = paused;
    }

    public(package) fun create_insurance_admin_cap(ctx: &mut TxContext): InsuranceAdminCap {
        InsuranceAdminCap { id: object::new(ctx) }
    }

    public(package) fun bootstrap_init(ctx: &mut TxContext) {
        let admin = tx_context::sender(ctx);
        transfer::share_object(InsuranceConfig {
            id: object::new(ctx),
            paused: false,
            min_coverage_bps: DEFAULT_MIN_COVERAGE_BPS,
            max_coverage_bps: DEFAULT_MAX_COVERAGE_BPS,
            max_duration_ms: DEFAULT_MAX_DURATION_MS,
            fee_bps: DEFAULT_FEE_BPS,
            treasury: admin,
            version: DEFAULT_VERSION,
        });
    }

    /// Create an underwriter vault
    public entry fun create_vault(
        base_rate_bps_per_day: u64,
        utilization_multiplier_bps: u64,
        max_exposure_per_market: u64,
        max_exposure_per_user: u64,
        ctx: &mut TxContext
    ) {
        let underwriter = tx_context::sender(ctx);
        let vault = UnderwriterVault {
            id: object::new(ctx),
            underwriter,
            capital: balance::zero(),
            reserved: 0,
            base_rate_bps_per_day,
            utilization_multiplier_bps,
            max_exposure_per_market,
            max_exposure_per_user,
            market_exposures: table::new(ctx),
            user_exposures: table::new(ctx),
            version: DEFAULT_VERSION,
        };
        let vault_id = object::id(&vault);
        transfer::share_object(vault);

        event::emit(VaultCreatedEvent {
            vault_id,
            underwriter,
            base_rate_bps_per_day,
            utilization_multiplier_bps,
            max_exposure_per_market,
            max_exposure_per_user,
        });
    }

    /// Deposit capital into vault
    public entry fun deposit_capital(
        config: &InsuranceConfig,
        vault: &mut UnderwriterVault,
        payment: Coin<MYS>,
        ctx: &mut TxContext
    ) {
        assert!(!config.paused, EPaused);
        let deposit_amount = coin::value(&payment);
        assert!(deposit_amount > 0, EInvalidAmount);
        balance::join(&mut vault.capital, coin::into_balance(payment));
        event::emit(VaultDepositedEvent {
            vault_id: object::id(vault),
            amount: deposit_amount,
            new_balance: balance::value(&vault.capital),
        });
    }

    /// Withdraw unreserved capital (underwriter only)
    public entry fun withdraw_capital(
        config: &InsuranceConfig,
        vault: &mut UnderwriterVault,
        amount: u64,
        ctx: &mut TxContext
    ) {
        assert!(!config.paused, EPaused);
        assert!(tx_context::sender(ctx) == vault.underwriter, ENotAdmin);
        assert!(amount > 0, EInvalidAmount);

        let capital_value = balance::value(&vault.capital);
        assert!(capital_value >= vault.reserved, EOverflow);
        let free_capital = capital_value - vault.reserved;
        assert!(free_capital >= amount, EInsufficientCapital);

        let payout_balance = balance::split(&mut vault.capital, amount);
        let payout_coin = coin::from_balance(payout_balance, ctx);
        transfer::public_transfer(payout_coin, vault.underwriter);

        event::emit(VaultWithdrawnEvent {
            vault_id: object::id(vault),
            amount,
            new_balance: balance::value(&vault.capital),
        });
    }

    /// Quote premium based on vault utilization
    public fun quote_premium(
        vault: &UnderwriterVault,
        covered_amount: u64,
        coverage_bps: u64,
        duration_ms: u64
    ): u64 {
        let capital_value = balance::value(&vault.capital);
        let utilization_bps = if (capital_value == 0) {
            BPS_DENOM
        } else {
            let utilization_u128 = (vault.reserved as u128) * (BPS_DENOM as u128) / (capital_value as u128);
            assert!(utilization_u128 <= (MAX_U64 as u128), EOverflow);
            utilization_u128 as u64
        };
        let utilization_factor = (utilization_bps * vault.utilization_multiplier_bps) / BPS_DENOM;
        let total_rate_bps_per_day = vault.base_rate_bps_per_day + utilization_factor;

        let numerator = (covered_amount as u128)
            * (coverage_bps as u128)
            * (total_rate_bps_per_day as u128)
            * (duration_ms as u128);
        let denominator = (BPS_DENOM as u128) * (BPS_DENOM as u128) * (DAY_MS as u128);
        let premium_u128 = numerator / denominator;
        assert!(premium_u128 <= (MAX_U64 as u128), EOverflow);
        premium_u128 as u64
    }

    /// Buy coverage for a SPoT position
    public entry fun buy_coverage(
        config: &InsuranceConfig,
        vault: &mut UnderwriterVault,
        record: &spot::SpotRecord,
        option_id: u8,
        requested_coverage_amount: u64,
        coverage_bps: u64,
        duration_ms: u64,
        mut payment: Coin<MYS>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(!config.paused, EPaused);
        assert!(spot::is_open(record), EMarketClosed);
        assert!(coverage_bps >= config.min_coverage_bps, EInvalidCoverage);
        assert!(coverage_bps <= config.max_coverage_bps, EInvalidCoverage);
        assert!(duration_ms > 0 && duration_ms <= config.max_duration_ms, EInvalidDuration);
        assert!(requested_coverage_amount > 0, EInvalidAmount);

        let insured = tx_context::sender(ctx);
        let market_id = spot::get_id_address(record);
        let position_amount = spot::get_user_option_amount(record, insured, option_id);
        let covered_amount = if (requested_coverage_amount <= position_amount) {
            requested_coverage_amount
        } else {
            position_amount
        };
        assert!(covered_amount > 0, EInvalidAmount);

        let reserve_amount = compute_reserve(covered_amount, coverage_bps);
        let capital_value = balance::value(&vault.capital);
        assert!(capital_value >= vault.reserved, EOverflow);
        let free_capital = capital_value - vault.reserved;
        assert!(free_capital >= reserve_amount, EInsufficientCapital);
        assert!(vault.reserved <= MAX_U64 - reserve_amount, EOverflow);

        enforce_exposure_limits(vault, market_id, insured, option_id, reserve_amount, ctx);

        let premium = quote_premium(vault, covered_amount, coverage_bps, duration_ms);
        assert!(premium > 0, EInsufficientPremium);
        assert!(coin::value(&payment) >= premium, EInsufficientPremium);

        let premium_coin = coin::split(&mut payment, premium, ctx);
        balance::join(&mut vault.capital, coin::into_balance(premium_coin));

        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, insured);
        } else {
            coin::destroy_zero(payment);
        };

        vault.reserved = vault.reserved + reserve_amount;
        add_exposure(vault, market_id, insured, option_id, reserve_amount, ctx);

        let now = clock::timestamp_ms(clock);
        assert!(now <= MAX_U64 - duration_ms, EOverflow);
        let expiry_time_ms = now + duration_ms;
        let policy = CoveragePolicy {
            id: object::new(ctx),
            market_id,
            insured,
            option_id,
            covered_amount,
            coverage_bps,
            premium_paid: premium,
            start_time_ms: now,
            expiry_time_ms,
            vault_id: object::id(vault),
            status: STATUS_ACTIVE,
        };
        let policy_id = object::id(&policy);
        transfer::public_transfer(policy, insured);

        event::emit(CoveragePurchasedEvent {
            policy_id,
            market_id,
            insured,
            option_id,
            covered_amount,
            coverage_bps,
            premium_paid: premium,
            reserve_locked: reserve_amount,
            expiry_time_ms,
        });
    }

    /// Cancel coverage while the market is open
    public entry fun cancel_coverage(
        config: &InsuranceConfig,
        vault: &mut UnderwriterVault,
        record: &spot::SpotRecord,
        policy: &mut CoveragePolicy,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(!config.paused, EPaused);
        assert!(spot::is_open(record), EMarketClosed);
        assert!(policy.status == STATUS_ACTIVE, EPolicyNotActive);
        assert!(tx_context::sender(ctx) == policy.insured, ENotPolicyOwner);
        assert!(policy.market_id == spot::get_id_address(record), EMarketMismatch);
        assert!(policy.vault_id == object::id(vault), EInvalidVault);

        let now = clock::timestamp_ms(clock);
        assert!(now < policy.expiry_time_ms, EPolicyExpired);

        let total_duration = policy.expiry_time_ms - policy.start_time_ms;
        let remaining = policy.expiry_time_ms - now;
        let refund_u128 = (policy.premium_paid as u128) * (remaining as u128) / (total_duration as u128);
        assert!(refund_u128 <= (MAX_U64 as u128), EOverflow);
        let mut refund = refund_u128 as u64;

        let fee = (refund * config.fee_bps) / BPS_DENOM;
        let total_refund = refund;
        let capital_value = balance::value(&vault.capital);
        assert!(capital_value >= total_refund, EInsufficientCapital);
        if (fee > 0) {
            refund = refund - fee;
            let fee_balance = balance::split(&mut vault.capital, fee);
            let fee_coin = coin::from_balance(fee_balance, ctx);
            transfer::public_transfer(fee_coin, config.treasury);
        };

        if (refund > 0) {
            let refund_balance = balance::split(&mut vault.capital, refund);
            let refund_coin = coin::from_balance(refund_balance, ctx);
            transfer::public_transfer(refund_coin, policy.insured);
        };

        let reserve_amount = compute_reserve(policy.covered_amount, policy.coverage_bps);
        release_exposure(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
        assert!(vault.reserved >= reserve_amount, EOverflow);
        vault.reserved = vault.reserved - reserve_amount;
        policy.status = STATUS_CANCELLED;

        event::emit(CoverageCancelledEvent {
            policy_id: object::id(policy),
            insured: policy.insured,
            refunded_amount: refund,
            fee_paid: fee,
        });
    }

    /// Claim payout after SPoT resolution
    public entry fun claim(
        config: &InsuranceConfig,
        vault: &mut UnderwriterVault,
        record: &spot::SpotRecord,
        policy: &mut CoveragePolicy,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(!config.paused, EPaused);
        assert!(policy.status == STATUS_ACTIVE, EPolicyNotActive);
        assert!(tx_context::sender(ctx) == policy.insured, ENotPolicyOwner);
        assert!(policy.market_id == spot::get_id_address(record), EMarketMismatch);
        assert!(policy.vault_id == object::id(vault), EInvalidVault);
        assert!(spot::is_resolved(record), EMarketClosed);

        let now = clock::timestamp_ms(clock);
        assert!(now <= policy.expiry_time_ms, EPolicyExpired);

        let outcome_opt = spot::get_outcome(record);
        assert!(option::is_some(outcome_opt), EMarketClosed);
        let outcome = *option::borrow(outcome_opt);

        let mut payout = 0;
        if (outcome != spot::outcome_draw() && outcome != spot::outcome_unapplicable()) {
            if (outcome != policy.option_id) {
                let current_position = spot::get_user_option_amount(record, policy.insured, policy.option_id);
                let eligible_amount = if (current_position < policy.covered_amount) {
                    current_position
                } else {
                    policy.covered_amount
                };
                let payout_u128 = (eligible_amount as u128) * (policy.coverage_bps as u128) / (BPS_DENOM as u128);
                assert!(payout_u128 <= (MAX_U64 as u128), EOverflow);
                payout = payout_u128 as u64;
            };
        };

        if (payout > 0) {
            let payout_balance = balance::split(&mut vault.capital, payout);
            let payout_coin = coin::from_balance(payout_balance, ctx);
            transfer::public_transfer(payout_coin, policy.insured);
        };

        let reserve_amount = compute_reserve(policy.covered_amount, policy.coverage_bps);
        release_exposure(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
        assert!(vault.reserved >= reserve_amount, EOverflow);
        vault.reserved = vault.reserved - reserve_amount;
        policy.status = STATUS_CLAIMED;

        event::emit(CoverageClaimedEvent {
            policy_id: object::id(policy),
            insured: policy.insured,
            payout,
        });
    }

    /// Expire policy and release reserves
    public entry fun expire_policy(
        vault: &mut UnderwriterVault,
        policy: &mut CoveragePolicy,
        clock: &Clock
    ) {
        if (policy.status != STATUS_ACTIVE) {
            return;
        };
        if (policy.vault_id != object::id(vault)) {
            return;
        };
        let now = clock::timestamp_ms(clock);
        if (now < policy.expiry_time_ms) {
            return;
        };

        let reserve_amount = compute_reserve(policy.covered_amount, policy.coverage_bps);
        release_exposure(vault, policy.market_id, policy.insured, policy.option_id, reserve_amount);
        assert!(vault.reserved >= reserve_amount, EOverflow);
        vault.reserved = vault.reserved - reserve_amount;
        policy.status = STATUS_EXPIRED;
    }

    fun compute_reserve(covered_amount: u64, coverage_bps: u64): u64 {
        let reserve_u128 = (covered_amount as u128) * (coverage_bps as u128);
        let reserve_u128 = reserve_u128 / (BPS_DENOM as u128);
        assert!(reserve_u128 <= (MAX_U64 as u128), EOverflow);
        reserve_u128 as u64
    }

    fun enforce_exposure_limits(
        vault: &mut UnderwriterVault,
        market_id: address,
        insured: address,
        option_id: u8,
        reserve_amount: u64,
        ctx: &mut TxContext
    ) {
        if (vault.max_exposure_per_market > 0) {
            let exposure = get_market_exposure_mut(vault, market_id, ctx);
            assert!(exposure.total_reserved <= MAX_U64 - reserve_amount, EOverflow);
            let new_total = exposure.total_reserved + reserve_amount;
            assert!(new_total <= vault.max_exposure_per_market, EExposureLimit);
        };
        if (vault.max_exposure_per_user > 0) {
            let current_user = get_user_exposure(vault, insured);
            assert!(current_user <= MAX_U64 - reserve_amount, EOverflow);
            let new_user = current_user + reserve_amount;
            assert!(new_user <= vault.max_exposure_per_user, EExposureLimit);
        };

        let exposure = get_market_exposure_mut(vault, market_id, ctx);
        let option_reserved = get_option_reserved(exposure, option_id);
        assert!(option_reserved <= MAX_U64 - reserve_amount, EOverflow);
    }

    fun add_exposure(
        vault: &mut UnderwriterVault,
        market_id: address,
        insured: address,
        option_id: u8,
        reserve_amount: u64,
        ctx: &mut TxContext
    ) {
        let exposure = get_market_exposure_mut(vault, market_id, ctx);
        assert!(exposure.total_reserved <= MAX_U64 - reserve_amount, EOverflow);
        exposure.total_reserved = exposure.total_reserved + reserve_amount;
        let option_reserved = get_option_reserved(exposure, option_id);
        assert!(option_reserved <= MAX_U64 - reserve_amount, EOverflow);
        let new_option_reserved = option_reserved + reserve_amount;
        set_option_reserved(exposure, option_id, new_option_reserved);

        let current_user = get_user_exposure(vault, insured);
        assert!(current_user <= MAX_U64 - reserve_amount, EOverflow);
        let new_user = current_user + reserve_amount;
        set_user_exposure(vault, insured, new_user);
    }

    fun release_exposure(
        vault: &mut UnderwriterVault,
        market_id: address,
        insured: address,
        option_id: u8,
        reserve_amount: u64
    ) {
        if (!table::contains(&vault.market_exposures, market_id)) {
            return;
        };
        let exposure = table::borrow_mut(&mut vault.market_exposures, market_id);
        if (exposure.total_reserved >= reserve_amount) {
            exposure.total_reserved = exposure.total_reserved - reserve_amount;
        };

        if (table::contains(&exposure.reserved_by_option, option_id)) {
            let current_option = *table::borrow(&exposure.reserved_by_option, option_id);
            if (current_option >= reserve_amount) {
                let option_ref = table::borrow_mut(&mut exposure.reserved_by_option, option_id);
                *option_ref = current_option - reserve_amount;
            };
        };

        if (table::contains(&vault.user_exposures, insured)) {
            let current_user = *table::borrow(&vault.user_exposures, insured);
            if (current_user >= reserve_amount) {
                let user_ref = table::borrow_mut(&mut vault.user_exposures, insured);
                *user_ref = current_user - reserve_amount;
            };
        };
    }

    fun get_market_exposure_mut(
        vault: &mut UnderwriterVault,
        market_id: address,
        ctx: &mut TxContext
    ): &mut MarketExposure {
        if (!table::contains(&vault.market_exposures, market_id)) {
            let exposure = MarketExposure {
                market_id,
                total_reserved: 0,
                reserved_by_option: table::new(ctx),
            };
            table::add(&mut vault.market_exposures, market_id, exposure);
        };
        table::borrow_mut(&mut vault.market_exposures, market_id)
    }

    fun get_user_exposure(vault: &UnderwriterVault, insured: address): u64 {
        if (table::contains(&vault.user_exposures, insured)) {
            *table::borrow(&vault.user_exposures, insured)
        } else {
            0
        }
    }

    fun set_user_exposure(vault: &mut UnderwriterVault, insured: address, amount: u64) {
        if (table::contains(&vault.user_exposures, insured)) {
            let user_ref = table::borrow_mut(&mut vault.user_exposures, insured);
            *user_ref = amount;
        } else {
            table::add(&mut vault.user_exposures, insured, amount);
        };
    }

    fun get_option_reserved(exposure: &MarketExposure, option_id: u8): u64 {
        if (table::contains(&exposure.reserved_by_option, option_id)) {
            *table::borrow(&exposure.reserved_by_option, option_id)
        } else {
            0
        }
    }

    fun set_option_reserved(exposure: &mut MarketExposure, option_id: u8, amount: u64) {
        if (table::contains(&exposure.reserved_by_option, option_id)) {
            let option_ref = table::borrow_mut(&mut exposure.reserved_by_option, option_id);
            *option_ref = amount;
        } else {
            table::add(&mut exposure.reserved_by_option, option_id, amount);
        };
    }
}
