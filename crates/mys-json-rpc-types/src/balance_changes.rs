// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use move_core_types::language_storage::TypeTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use std::fmt::{Display, Formatter, Result};
use mys_types::object::Owner;
use mys_types::mys_serde::MysTypeTag;

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BalanceChange {
    /// Owner of the balance change
    pub owner: Owner,
    #[schemars(with = "String")]
    #[serde_as(as = "MysTypeTag")]
    pub coin_type: TypeTag,
    /// The amount indicate the balance value changes,
    /// negative amount means spending coin value and positive means receiving coin value.
    #[schemars(with = "String")]
    #[serde_as(as = "DisplayFromStr")]
    pub amount: i128,
}

impl Display for BalanceChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            " ┌──\n │ Owner: {} \n │ CoinType: {} \n │ Amount: {}\n └──",
            self.owner, self.coin_type, self.amount
        )
    }
}
