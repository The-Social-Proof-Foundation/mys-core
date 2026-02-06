// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module mys_system::validator_wrapper {
    use mys::versioned::Versioned;

    public struct ValidatorWrapper has store {
        inner: Versioned
    }
}
