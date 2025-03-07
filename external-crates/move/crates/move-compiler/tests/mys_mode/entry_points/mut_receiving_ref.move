// valid, Receiving type by mut ref with object type param

module a::m {
    use mys::object;
    use mys::transfer::Receiving;

    struct S has key { id: object::UID }

    public entry fun yes(_: &mut Receiving<S>) { }
}

module mys::object {
    struct UID has store {
        id: address,
    }
}

module mys::transfer {
    struct Receiving<phantom T: key> has drop {
        id: address
    }
}
