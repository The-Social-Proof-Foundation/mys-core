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

        /// Any extra fields that's not defined statically.
        extra_fields: Bag,
    }

    const BASIS_POINT_DENOMINATOR: u128 = 10000;

    const ESubsidyDecreaseRateTooLarge: u64 = 0;
    const ESubsidyInitialApyTooLarge: u64 = 1;

    const YEAR_IN_MS: u64 = 365 * 24 * 60 * 60 * 1000;

    public(package) fun create(
        balance: Balance<MYS>,
        initial_apy_bps: u64,
        stake_subsidy_period_length: u64,
        stake_subsidy_decrease_rate: u16,
        ctx: &mut TxContext,
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

        StakeSubsidy {
            balance,
            distribution_counter: 0,
            current_apy_bps: initial_apy_bps,
            stake_subsidy_period_length,
            stake_subsidy_decrease_rate,
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
            self.current_apy_bps,
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
            self.current_apy_bps,
            total_staked_mist,
            epoch_duration_ms,
        ).min(self.balance.value())
    }

    fun calculate_epoch_subsidy_amount(
        current_apy_bps: u64,
        total_staked_mist: u64,
        epoch_duration_ms: u64,
    ): u64 {
        if (total_staked_mist == 0 || epoch_duration_ms == 0) {
            return 0
        };

        let epochs_per_year = (YEAR_IN_MS / epoch_duration_ms).max(1);
        let yearly_rewards = total_staked_mist as u128
            * (current_apy_bps as u128)
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
