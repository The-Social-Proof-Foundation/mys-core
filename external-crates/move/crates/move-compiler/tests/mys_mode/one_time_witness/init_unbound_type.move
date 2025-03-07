module a::m {
    fun init(_ctx: who::TxContext) {}
}

module a::beep {
    struct BEEP has drop {}
    fun init(_: Who, _ctx: &mut mys::tx_context::TxContext) {}
}

module mys::tx_context {
    struct TxContext {}
}
