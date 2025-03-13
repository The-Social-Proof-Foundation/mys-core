// Copyright (c) The Social Proof Foundation
// SPDX-License-Identifier: Apache-2.0

/// A simple module for testing publishing
module social::simple {
    // A simple value struct with no dependencies
    struct SimpleValue has drop, copy {
        value: u64
    }

    // Create a new simple value
    public fun create_value(value: u64): SimpleValue {
        SimpleValue { value }
    }

    // Get the value from a SimpleValue
    public fun get_value(simple: &SimpleValue): u64 {
        simple.value
    }

    // Add to a value
    public fun add_value(simple: &mut SimpleValue, amount: u64) {
        simple.value = simple.value + amount;
    }

    // Reset the value to zero
    public fun reset_value(simple: &mut SimpleValue) {
        simple.value = 0;
    }

    // Test function that returns the meaning of life
    public fun meaning_of_life(): u64 {
        42
    }
}