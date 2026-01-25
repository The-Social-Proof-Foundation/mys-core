// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module bridged_myusd::myusd {
    use std::option;

    use mys::coin::{Self, CoinCreationAdminCap};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};

    struct MYUSD has drop {}

    const DECIMAL: u8 = 6;

    fun init(_otw: MYUSD, _ctx: &mut TxContext) {
        // Empty - coin creation moved to init_coin entry function
    }

    public entry fun init_coin(
        admin_cap: &CoinCreationAdminCap,
        ctx: &mut TxContext
    ) {
        let (treasury_cap, metadata) = coin::create_currency_with_admin<MYUSD>(
            DECIMAL,
            b"MyUSD",
            b"MyUSD",
            b"The official MySocial USD stablecoin.",
            option::none(),
            admin_cap,
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }
}
