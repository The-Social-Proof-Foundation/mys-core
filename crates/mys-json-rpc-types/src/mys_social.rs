// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::Page;
use mys_types::base_types::{ObjectDigest, ObjectID, SequenceNumber, TransactionDigest, MysAddress};
use mys_types::mys_serde::SequenceNumber as AsSequenceNumber;

/// Type for paginated social value results
pub type SocialValuePage = Page<SocialValueData, String>;

/// Social value data structure for RPC responses
#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocialValueData {
    /// The social value object ID
    pub value_id: ObjectID,
    /// The sequence number of the social value object
    #[schemars(with = "AsSequenceNumber")]
    #[serde_as(as = "AsSequenceNumber")]
    pub version: SequenceNumber,
    /// Object digest
    pub digest: ObjectDigest,
    /// The numeric value stored
    pub value: u64,
    /// Social value owner address
    pub owner: MysAddress,
    /// Transaction digest that created or last updated the social value
    pub previous_transaction: TransactionDigest,
}