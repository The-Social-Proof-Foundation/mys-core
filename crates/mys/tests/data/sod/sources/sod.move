// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module sod::sod {
    use mys::object::{Self, UID};
    use mys::tx_context::TxContext;
    use mys::transfer;

    public struct A has key, store {
        id: UID,
    }

    public fun start(ctx: &mut TxContext) {
        let a = A { id: object::new(ctx) };
        transfer::public_share_object(a);
    }

    public entry fun delete(a: A) {
        let A { id } = a;
        object::delete(id);
    }
}
