module a::m {
    use mys::tx_context;

    public entry fun foo<T>(_: T, _: &mut tx_context::TxContext) {
        abort 0
    }

}

module mys::tx_context {
    struct TxContext has drop {}
}
