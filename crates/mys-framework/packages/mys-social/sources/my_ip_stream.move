// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module social_contracts::my_ip_stream {
    use std::ascii::String;
    use mys::object::{Self, UID, new};
    use mys::url::Url;
    use mys::option::{Self, Option};
    use mys::transfer;
    use mys::tx_context::TxContext;

    /// Basic representation of an encrypted data stream owned by a user.
    public struct MyIPDataStream has key, store {
        id: UID,
        owner: address,
        data_type: String,
        encrypted_uri: Url,
        price: Option<u64>,
        version: u64,
    }

    /// Mint a new data stream and transfer it to the creator.
    public entry fun mint(
        data_type: String,
        encrypted_uri: Url,
        price: Option<u64>,
        ctx: &mut TxContext,
    ) {
        let stream = MyIPDataStream {
            id: new(ctx),
            owner: ctx.sender(),
            data_type,
            encrypted_uri,
            price,
            version: 0,
        };
        transfer::transfer(stream, ctx.sender());
    }
}
