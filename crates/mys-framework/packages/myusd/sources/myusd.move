// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module myusd::myusd {
    use mys::coin;

    public struct MYUSD has drop {}

    const DECIMAL: u8 = 9;

    fun init(otw: MYUSD, ctx: &mut mys::tx_context::TxContext) {
        let (treasury_cap, metadata) = coin::create_currency(
            otw,
            DECIMAL,
            b"myUSD",
            b"MyUSD",
            b"The official MySocial USD stablecoin.",
            std::option::none(),
            ctx
        );
        mys::transfer::public_freeze_object(metadata);
        mys::transfer::public_transfer(treasury_cap, mys::tx_context::sender(ctx));
    }

    /// Test-only function to create bridge-compatible token setup
    /// This mirrors the pattern used in bridge tests
    #[test_only]
    public fun create_bridge_token(ctx: &mut mys::tx_context::TxContext): (mys::package::UpgradeCap, mys::coin::TreasuryCap<MYUSD>, mys::coin::CoinMetadata<MYUSD>) {
        use std::ascii;
        use std::type_name;
        use mys::address;
        use mys::hex;
        use mys::package::test_publish;
        use mys::test_utils::create_one_time_witness;

        let otw = create_one_time_witness<MYUSD>();
        let (treasury_cap, metadata) = coin::create_currency(
            otw,
            DECIMAL,
            b"myUSD",
            b"MyUSD",
            b"The official MySocial USD stablecoin.",
            std::option::none(),
            ctx
        );
        
        let type_name = type_name::get<MYUSD>();
        let address_bytes = hex::decode(
            ascii::into_bytes(type_name::get_address(&type_name)),
        );
        let coin_id = address::from_bytes(address_bytes).to_id();
        let upgrade_cap = test_publish(coin_id, ctx);
        
        (upgrade_cap, treasury_cap, metadata)
    }
}
