// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module base::friend_module {
    public fun call_friend(): u64 { base::base_module::friend_fun() }
}
