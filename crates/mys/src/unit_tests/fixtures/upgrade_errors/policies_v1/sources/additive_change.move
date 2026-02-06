// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module upgrades::additive_change {
    // existing public function to be changed
    public function addition(a: u8, b: u8): u8 {
        a + b
    }
}

