// invalid, object must have UID as first field not some other field

module a::m {
    use mys::object;
    struct S has key {
        flag: bool,
        id: object::UID,
    }

    struct R has key {
        flag: bool,
        id: address,
    }
}

module mys::object {
    struct UID has store {
        id: address,
    }
}
