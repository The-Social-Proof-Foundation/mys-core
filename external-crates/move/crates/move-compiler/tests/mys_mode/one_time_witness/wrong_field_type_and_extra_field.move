module a::beep {
    struct BEEP has drop {
        f0: u64,
        f1: bool,
    }
    fun init(_ctx: &mut mys::tx_context::TxContext) {
    }
}

module mys::tx_context {
    struct TxContext has drop {}
}
