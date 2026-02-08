// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use mys_rpc::proto::mys::rpc::v2::BalanceChange as ProtoBalanceChange;
use mys_types::balance_change::BalanceChange as CoreBalanceChange;

pub fn from_core_balance_change(value: CoreBalanceChange) -> ProtoBalanceChange {
    let sdk_balance_change: mys_sdk_types::BalanceChange = value.into();
    from_sdk_balance_change(sdk_balance_change)
}

pub fn from_sdk_balance_change(value: mys_sdk_types::BalanceChange) -> ProtoBalanceChange {
    // We can't import super types directly if they are generated in this crate but we are using foreign proto type
    // But ProtoBalanceChange expects options.
    // Let's assume we construct it manually.
    ProtoBalanceChange {
        address: Some(value.address.into()),
        coin_type: Some(value.coin_type.into()),
        amount: Some(value.amount.into()),
    }
}

