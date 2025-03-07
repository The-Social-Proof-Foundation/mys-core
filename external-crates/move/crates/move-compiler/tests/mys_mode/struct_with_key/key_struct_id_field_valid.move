// valid
module a::m {
    use mys::object;
    struct S has key {
        id: object::UID
    }
}

module mys::object {
    struct UID has store {
        id: address,
    }
}
