// invalid, objects need UID not ID
module a::m {
    use mys::object;
    struct S has key {
        id: object::ID
    }
}

module mys::object {
    struct ID has store {
        id: address,
    }
}
