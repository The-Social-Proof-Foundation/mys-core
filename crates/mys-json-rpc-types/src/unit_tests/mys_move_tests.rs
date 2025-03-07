// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use mys_enum_compat_util::*;

use crate::{MysMoveStruct, MysMoveValue};

#[test]
fn enforce_order_test() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "mys_move_struct.yaml"]);
    check_enum_compat_order::<MysMoveStruct>(path);

    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "mys_move_value.yaml"]);
    check_enum_compat_order::<MysMoveValue>(path);
}
