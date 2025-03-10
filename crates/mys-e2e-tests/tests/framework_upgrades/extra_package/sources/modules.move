// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module mys_extra::msim_extra_1 {
    use mys::object::{Self, UID};
    use mys::transfer;
    use mys::tx_context::TxContext;

    public struct S has key { id: UID }

    fun init(ctx: &mut TxContext) {
        transfer::share_object(S {
            id: object::new(ctx)
        })
    }

    public fun canary(): u64 {
        43
    }
}
