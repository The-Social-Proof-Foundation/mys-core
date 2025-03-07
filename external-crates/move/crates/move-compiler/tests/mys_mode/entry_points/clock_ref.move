// valid, Clock by immutable reference

module a::m {
    public entry fun yes_clock_ref(_: &mys::clock::Clock) {
        abort 0
    }
}

module mys::clock {
    struct Clock has key {
        id: mys::object::UID,
    }
}

module mys::object {
    struct UID has store {
        id: address,
    }
}
