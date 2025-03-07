// invalid Random by mutable reference

module a::m {
    public entry fun no_random_mut(_: &mut mys::random::Random) {
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
