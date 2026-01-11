// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::big_int::BigInt;
use async_graphql::*;

/// Parameters that control the distribution of the stake subsidy.
#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
pub(crate) struct StakeSubsidy {
    /// MYS set aside for stake subsidies -- reduces over time as stake subsidies are paid out over
    /// time.
    pub balance: Option<BigInt>,

    /// Number of times stake subsidies have been distributed subsidies are distributed with other
    /// staking rewards, at the end of the epoch.
    pub distribution_counter: Option<u64>,

    /// Current stake subsidy APY in basis points -- decays over time.
    pub current_apy_bps: Option<BigInt>,

    /// Maximum number of stake subsidy distributions that occur with the same APY
    /// (before the APY is reduced).
    pub period_length: Option<u64>,

    /// Percentage of the current APY to deduct at the end of the current subsidy
    /// period, expressed in basis points.
    pub decrease_rate: Option<u64>,

    /// The annual percentage yield from the stake subsidy in basis points.
    /// To get the APY in percentage, divide by 100.
    pub apy: Option<u64>,
}
