// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

mod gas_status_displays;
pub mod transaction_displays;

pub struct Pretty<'a, T>(pub &'a T);
