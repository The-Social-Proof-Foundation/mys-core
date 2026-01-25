// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module bridged_myusd::myusd {
    use std::option;

    use mys::coin;
    use mys::transfer;
    use mys::tx_context;
    use mys::tx_context::TxContext;

    struct MYUSD has drop {}

    const DECIMAL: u8 = 6;

    fun init(otw: MYUSD, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = coin::create_currency(
            otw,
            DECIMAL,
            b"MyUSD",
            b"MyUSD",
            b"The official MySocial USD stablecoin.",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }
}
