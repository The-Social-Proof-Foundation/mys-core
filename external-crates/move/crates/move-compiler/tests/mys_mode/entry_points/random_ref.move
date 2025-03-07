// valid Random by immutable reference

module a::m {
    public entry fun yes_random_ref(_: &mys::random::Random) {
        abort 0
    }
}

module mys::random {
    struct Random has key {
        id: mys::object::UID,
    }
}

module mys::object {
    struct UID has store {
        id: address,
    }
}
