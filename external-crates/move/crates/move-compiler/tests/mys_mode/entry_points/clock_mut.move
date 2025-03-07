// invalid, Clock by mutable reference

module a::m {
    public entry fun no_clock_mut(_: &mut mys::clock::Clock) {
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
