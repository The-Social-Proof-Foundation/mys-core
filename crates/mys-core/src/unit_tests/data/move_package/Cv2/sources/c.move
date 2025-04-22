// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module c::c {
    public struct C {
        x: u64
    }

    public struct D {
        x: u64,
        y: u64,
    }

    public fun c(): u64 {
        43
    }
}
