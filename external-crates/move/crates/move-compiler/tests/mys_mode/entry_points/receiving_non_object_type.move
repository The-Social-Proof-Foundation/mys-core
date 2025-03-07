// invalid, Receiving type with non-object type param

module a::m {
    use mys::transfer::Receiving;

    public entry fun no(_: Receiving<u64>) { abort 0 }
}

module mys::transfer {
    struct Receiving<phantom T: key> has drop {
        id: address
    }
}
