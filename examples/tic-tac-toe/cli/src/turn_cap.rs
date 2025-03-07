// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use mys_types::base_types::ObjectID;

/// Rust representation of a Move `owned::TurnCap`, ttttable for deserializing from their BCS
/// representation.
#[allow(dead_code)]
#[derive(Deserialize)]
pub(crate) struct TurnCap {
    pub id: ObjectID,
    pub game: ObjectID,
}
