// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module mys_system::stake_subsidy {
    use mys::balance::Balance;
    use mys::mys::MYS;
    use mys::bag::Bag;
    use mys::bag;

    public struct StakeSubsidy has store {
        /// Balance of MYS set aside for stake subsidies that will be drawn down over time.
        balance: Balance<MYS>,

        /// Count of the number of times stake subsidies have been distributed.
        distribution_counter: u64,

        /// The current APY (in basis points) used to compute stake subsidies.
        /// This amount decays and decreases over time.
        current_apy_bps: u64,

        /// Number of distributions to occur before the APY decays.
        stake_subsidy_period_length: u64,

        /// The rate at which the APY decays at the end of each
        /// period. Expressed in basis points.
        stake_subsidy_decrease_rate: u16,

        /// Maximum APY cap (in basis points). Effective APY will never exceed this.
        max_apy_bps: u64,

        /// Minimum APY floor (in basis points). Effective APY will never go below this.
        min_apy_bps: u64,

        /// Target duration for subsidy pool in years (e.g., 10).
        /// Used to calculate stake-aware APY reduction to ensure pool sustainability.
        intended_duration_years: u64,

        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    const BASIS_POINT_DENOMINATOR: u128 = 10000;

    const ESubsidyDecreaseRateTooLarge: u64 = 0;
    const ESubsidyInitialApyTooLarge: u64 = 1;
    const ESubsidyMinApyGreaterThanMax: u64 = 2;
    const ESubsidyMaxApyTooLarge: u64 = 3;
    const ESubsidyIntendedDurationZero: u64 = 4;

    const YEAR_IN_MS: u64 = 365 * 24 * 60 * 60 * 1000;

    public(package) fun create(
        balance: Balance<MYS>,
        initial_apy_bps: u64,
        stake_subsidy_period_length: u64,
        stake_subsidy_decrease_rate: u16,
        max_apy_bps: u64,
        min_apy_bps: u64,
        intended_duration_years: u64,
        ctx: &mut mys::tx_context::TxContext,
    ): StakeSubsidy {
        // Rate can't be higher than 100%.
        assert!(
            stake_subsidy_decrease_rate <= BASIS_POINT_DENOMINATOR as u16,
            ESubsidyDecreaseRateTooLarge,
        );
        assert!(
            initial_apy_bps <= BASIS_POINT_DENOMINATOR as u64,
            ESubsidyInitialApyTooLarge,
        );
        // Max APY can't be higher than 100%.
        assert!(
            max_apy_bps <= BASIS_POINT_DENOMINATOR as u64,
            ESubsidyMaxApyTooLarge,
        );
        // Min APY must be less than or equal to max APY.
        assert!(
            min_apy_bps <= max_apy_bps,
            ESubsidyMinApyGreaterThanMax,
        );
        // Intended duration must be greater than zero.
        assert!(
            intended_duration_years > 0,
            ESubsidyIntendedDurationZero,
        );

        StakeSubsidy {
            balance,
            distribution_counter: 0,
            current_apy_bps: initial_apy_bps,
            stake_subsidy_period_length,
            stake_subsidy_decrease_rate,
            max_apy_bps,
            min_apy_bps,
            intended_duration_years,
            extra_fields: bag::new(ctx),
        }
    }

    /// Advance the epoch counter and draw down the subsidy for the epoch.
    public(package) fun advance_epoch(
        self: &mut StakeSubsidy,
        total_staked_mist: u64,
        epoch_duration_ms: u64,
    ): Balance<MYS> {
        let epoch_subsidy_amount = calculate_epoch_subsidy_amount(
            self,
            total_staked_mist,
            epoch_duration_ms,
        );

        // Take the minimum of the reward amount and the remaining balance in
        // order to ensure we don't overdraft the remaining stake subsidy
        // balance
        let to_withdraw = epoch_subsidy_amount.min(self.balance.value());

        // Drawn down the subsidy for this epoch.
        let stake_subsidy = self.balance.split(to_withdraw);
        self.distribution_counter = self.distribution_counter + 1;

        // Decrease the subsidy amount only when the current period ends.
        if (self.distribution_counter % self.stake_subsidy_period_length == 0) {
            let decrease_amount = self.current_apy_bps as u128
                * (self.stake_subsidy_decrease_rate as u128) / BASIS_POINT_DENOMINATOR;
            self.current_apy_bps = self.current_apy_bps - (decrease_amount as u64)
        };

        stake_subsidy
    }

    /// Returns the amount of stake subsidy to be added at the end of the current epoch.
    public fun current_epoch_subsidy_amount(
        self: &StakeSubsidy,
        total_staked_mist: u64,
        epoch_duration_ms: u64,
    ): u64 {
        calculate_epoch_subsidy_amount(
            self,
            total_staked_mist,
            epoch_duration_ms,
        ).min(self.balance.value())
    }

    /// Calculate the effective APY considering stake-aware constraints and caps.
    /// This ensures the subsidy pool lasts the intended duration while respecting min/max bounds.
    fun calculate_effective_apy(
        self: &StakeSubsidy,
        total_staked_mist: u64,
        epoch_duration_ms: u64,
    ): u64 {
        // Start with the decayed target APY
        let target_apy_bps = self.current_apy_bps;

        // If no stake or zero epoch duration, return 0
        if (total_staked_mist == 0 || epoch_duration_ms == 0) {
            return 0
        };

        // Calculate projected yearly consumption at current stake and target APY
        let projected_yearly_consumption = (total_staked_mist as u128)
            * (target_apy_bps as u128)
            / BASIS_POINT_DENOMINATOR;

        // If projected consumption is zero, return 0
        if (projected_yearly_consumption == 0) {
            return 0
        };

        // Calculate remaining years at current rate
        let remaining_balance = self.balance.value() as u128;
        // remaining_years = remaining_balance / projected_yearly_consumption
        // We multiply by BASIS_POINT_DENOMINATOR to maintain precision in integer arithmetic
        // Use multiplication first to avoid precision loss from integer division
        let remaining_years_scaled = if (projected_yearly_consumption > 0 && remaining_balance > 0) {
            // Calculate: (remaining_balance * BASIS_POINT_DENOMINATOR) / projected_yearly_consumption
            // This gives us remaining_years scaled by BASIS_POINT_DENOMINATOR for precision
            let numerator = remaining_balance * BASIS_POINT_DENOMINATOR;
            numerator / projected_yearly_consumption
        } else {
            0
        };
        let intended_duration_years_scaled = (self.intended_duration_years as u128) * BASIS_POINT_DENOMINATOR;

        // Calculate effective APY with stake-aware reduction
        // If remaining_years < intended_duration_years, reduce APY proportionally
        let effective_apy_bps = if (remaining_years_scaled > 0 && remaining_years_scaled < intended_duration_years_scaled) {
            // Reduce APY to ensure pool lasts intended duration
            // effective_apy = target_apy * (remaining_years / intended_duration_years)
            // Calculate with proper precision: (target_apy * remaining_years_scaled) / intended_duration_years_scaled
            let scaled_apy = (target_apy_bps as u128) * remaining_years_scaled;
            scaled_apy / intended_duration_years_scaled
        } else if (remaining_years_scaled >= intended_duration_years_scaled) {
            // Pool will last longer than intended, use target APY
            target_apy_bps as u128
        } else {
            // remaining_years_scaled is 0, which means balance is very small relative to consumption
            // Use min_apy_bps to ensure some payout happens
            self.min_apy_bps as u128
        };

        // Apply min/max caps (ensure min is always respected)
        let capped_apy = if (effective_apy_bps < self.min_apy_bps as u128) {
            self.min_apy_bps as u128
        } else if (effective_apy_bps > self.max_apy_bps as u128) {
            self.max_apy_bps as u128
        } else {
            effective_apy_bps
        };

        capped_apy as u64
    }

    /// Calculate the epoch subsidy amount using stake-aware APY calculation.
    fun calculate_epoch_subsidy_amount(
        self: &StakeSubsidy,
        total_staked_mist: u64,
        epoch_duration_ms: u64,
    ): u64 {
        if (total_staked_mist == 0 || epoch_duration_ms == 0) {
            return 0
        };

        // Calculate effective APY with stake-aware constraints
        let effective_apy_bps = calculate_effective_apy(self, total_staked_mist, epoch_duration_ms);

        // Calculate epoch rewards from effective APY
        let epochs_per_year = (YEAR_IN_MS / epoch_duration_ms).max(1);
        let yearly_rewards = (total_staked_mist as u128)
            * (effective_apy_bps as u128)
            / BASIS_POINT_DENOMINATOR;
        let per_epoch_rewards = yearly_rewards / (epochs_per_year as u128);
        per_epoch_rewards as u64
    }

    /// Returns the number of distributions that have occurred.
    public(package) fun get_distribution_counter(self: &StakeSubsidy): u64 {
        self.distribution_counter
    }

    #[test_only]
    public(package) fun set_distribution_counter(self: &mut StakeSubsidy, distribution_counter: u64) {
        self.distribution_counter = distribution_counter;
    }
}
