// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module orderbook_client::orderbook_client {
    use orderbook::clob::Order;

    public fun f(): Order {
        abort(0)
    }
}
