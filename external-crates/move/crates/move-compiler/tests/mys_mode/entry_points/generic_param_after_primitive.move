module a::m {
    use mys::object;
    struct Obj has key {
        id: object::UID,
    }
    public entry fun foo<T>(_: Obj, _: u64, _: T) {
        abort 0
    }

}

module mys::object {
    struct UID has store {
        id: address,
    }
}
