// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::gas::GasCostSummary;
use async_graphql::*;

/// Information about whether epoch changes are using safe mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SafeMode {
    /// Whether safe mode was used for the last epoch change.  The system will retry a full epoch
    /// change on every epoch boundary and automatically reset this flag if so.
    pub enabled: Option<bool>,

    /// Accumulated fees for computation and cost that have not been added to the various reward
    /// pools, because the full epoch change did not happen.
    pub gas_summary: Option<GasCostSummary>,
}

#[Object]
impl SafeMode {
    /// Whether safe mode was used for the last epoch change.  The system will retry a full epoch
    /// change on every epoch boundary and automatically reset this flag if so.
    async fn enabled(&self) -> Option<bool> {
        self.enabled
    }

    /// Accumulated fees for computation and cost that have not been added to the various reward
    /// pools, because the full epoch change did not happen.
    async fn gas_summary(&self) -> Option<&GasCostSummary> {
        self.gas_summary.as_ref()
    }
}
