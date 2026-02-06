// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//# init --addresses Test=0x0

//# publish --upgradeable
module Test::M1 {
    fun init(_ctx: &mut TxContext) { }
}

//# view-object 1,1
