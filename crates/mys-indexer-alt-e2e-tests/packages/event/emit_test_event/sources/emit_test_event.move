// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

module emit_test_event::emit_test_event {
    use mys::event;

    public struct TestEvent has copy, drop {
        value: u64,
    }

    public fun emit_test_event() {
        event::emit(TestEvent {
            value: 1,
        });
    }
}
