// invalid, object cannot have drop since UID does not have drop

module a::m {
    use mys::object;
    struct S has key, drop {
        id: object::UID,
        flag: bool
    }
}

module mys::object {
    struct UID has store {
        id: address,
    }
}
