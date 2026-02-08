// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::SimpleObject;

use crate::api::scalars::base64::Base64;
use crate::api::scalars::mys_address::MysAddress;
use crate::api::types::transaction_kind::programmable::commands::TransactionArgument;

/// Upgrades a Move Package.
#[derive(SimpleObject, Clone)]
pub struct UpgradeCommand {
    /// Bytecode for the modules to be published, BCS serialized and Base64 encoded.
    pub modules: Option<Vec<Base64>>,

    /// IDs of the transitive dependencies of the package to be published.
    pub dependencies: Option<Vec<MysAddress>>,

    /// ID of the package being upgraded.
    pub current_package: Option<MysAddress>,

    /// The `UpgradeTicket` authorizing the upgrade.
    pub upgrade_ticket: Option<TransactionArgument>,
}
