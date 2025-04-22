// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module deepbook_client::deepbook_client {
    use deepbook::clob::Order;

    public fun f(): Order {
        abort(0)
    }
}
