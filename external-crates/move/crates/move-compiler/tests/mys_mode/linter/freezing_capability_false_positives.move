// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module a::test_false_positives {
    use mys::object::UID;
    use mys::transfer;

    struct NoCap has key {
       id: UID
    }

    struct CapAndHat has key {
       id: UID
    }

    struct Recap has key {
       id: UID
    }

    struct MyCapybara has key {
       id: UID
    }

    public fun freeze_capture(w: NoCap) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_handicap(w: CapAndHat) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_recap(w: Recap) {
        transfer::public_freeze_object(w);
    }

    public fun freeze_capybara(w: MyCapybara) {
        transfer::public_freeze_object(w);
    }
}

module mys::object {
    struct UID has store {
        id: address,
    }
}

module mys::transfer {
    const ZERO: u64 = 0;
    public fun public_freeze_object<T: key>(_: T) {
        abort ZERO
    }
}
