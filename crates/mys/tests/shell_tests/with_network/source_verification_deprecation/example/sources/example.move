// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Module: example
module example::example;

use dependency::dependency::f;

public fun g(): u64 { f() }
