// init is unused but does not error because we are in Mys mode
module a::m {
    fun init(_: &mut mys::tx_context::TxContext) {}
}

module mys::tx_context {
    struct TxContext has drop {}
}
