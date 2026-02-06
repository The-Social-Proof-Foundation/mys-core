// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module a::a {
    use b::b::b;
    use b::b::c;
    
    public fun a() : u64 {
        b() + c()
    }
}
