// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module c::c {
    use a::a;
    use b::b;

    public fun c() {
        a::a();
        b::b();
    }
}