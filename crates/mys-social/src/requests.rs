// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use mys_types::base_types::MysAddress;

#[derive(Debug, Serialize, Deserialize)]
pub struct SetValueRequest {
    pub owner: MysAddress,
    pub value: u64,
}