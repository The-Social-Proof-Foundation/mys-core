// valid init function
module a::m {
    use mys::tx_context;
    fun init(_: &mut tx_context::TxContext) {
    }
}

module mys::tx_context {
    struct TxContext has drop {}
}
