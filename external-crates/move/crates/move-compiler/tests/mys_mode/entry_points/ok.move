// TxContext by immutable or mutable ref

module a::m {
    use mys::tx_context;
    public entry fun f(_: &tx_context::TxContext) {
        abort 0
    }

    public entry fun t2(_: &mut tx_context::TxContext) {
        abort 0
    }
}

module mys::tx_context {
    struct TxContext has drop {}
}
