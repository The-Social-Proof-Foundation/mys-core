// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use mys_types::base_types::{ObjectID, MysAddress, TransactionDigest};

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialValueResponse {
    pub value_id: ObjectID,
    pub value: u64,
    pub owner: MysAddress,
    pub tx_digest: Option<TransactionDigest>,
}