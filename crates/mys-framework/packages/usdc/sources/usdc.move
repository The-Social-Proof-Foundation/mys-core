// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module usdc::usdc {
    use mys::coin;

    public struct USDC has drop {}

    const DECIMAL: u8 = 6;

    fun init(otw: USDC, ctx: &mut mys::tx_context::TxContext) {
        let (treasury_cap, metadata) = coin::create_currency(
            otw,
            DECIMAL,
            b"USDC",
            b"USD Coin",
            b"Bridged USD Coin token",
            std::option::none(),
            ctx
        );
        mys::transfer::public_freeze_object(metadata);
        mys::transfer::public_transfer(treasury_cap, mys::tx_context::sender(ctx));
    }
}
