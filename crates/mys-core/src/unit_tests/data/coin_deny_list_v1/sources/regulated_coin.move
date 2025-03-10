// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module coin_deny_list_v1::regulated_coin {
    use std::option;
    use mys::coin;
    use mys::object::UID;
    use mys::transfer;
    use mys::tx_context;
    use mys::tx_context::TxContext;

    public struct REGULATED_COIN has drop {}

    public struct Wallet has key {
        id: UID,
    }

    fun init(otw: REGULATED_COIN, ctx: &mut TxContext) {
        let (treasury_cap, deny_cap, metadata) = coin::create_regulated_currency(
            otw,
            9,
            b"RC",
            b"REGULATED_COIN",
            b"A new regulated coin",
            option::none(),
            ctx
        );
        transfer::public_transfer(deny_cap, tx_context::sender(ctx));
        transfer::public_freeze_object(treasury_cap);
        transfer::public_freeze_object(metadata);
    }
}
