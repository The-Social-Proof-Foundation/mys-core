// not allowed since Foo is not packed with a fresh UID
module a::m {
    use mys::object::UID;

    struct Foo has key {
        id: UID,
    }

    fun transfer(_: Foo) {
        abort 0
    }

    public fun foo(f: Foo) {
        let Foo { id } = f;
        transfer(Foo { id });
    }

}

// allowed since no packing occurs
module k::m {
    use mys::object::UID;

    struct Foo has key {
        id: UID,
    }

    fun transfer(_: UID) {
        abort 0
    }

    public fun foo(f: Foo) {
        let Foo { id } = f;
        transfer(id);
    }

}


module mys::object {
    struct UID has store {
        id: address,
    }
}
