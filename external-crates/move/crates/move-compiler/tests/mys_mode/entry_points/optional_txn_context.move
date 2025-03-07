// TxContext does not have to be present

module a::m {
    public entry fun t() {
        abort 0
    }

    struct Obj has key { id: mys::object::UID }
    public entry fun t2(_: bool, _: &mut Obj) {
        abort 0
    }
}

module mys::object {
    struct UID has store {
        id: address,
    }
}
