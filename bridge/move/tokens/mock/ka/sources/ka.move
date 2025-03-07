// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridged_ka::ka {
    use std::option;

    use mys::coin;
    use mys::transfer;
    use mys::tx_context;
    use mys::tx_context::TxContext;

    struct KA has drop {}

    const DECIMAL: u8 = 9;

    fun init(otw: KA, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = coin::create_currency(
            otw,
            DECIMAL,
            b"Ka",
            b"Ka Coin",
            b"Ka, the opposite of Mys",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }
}
