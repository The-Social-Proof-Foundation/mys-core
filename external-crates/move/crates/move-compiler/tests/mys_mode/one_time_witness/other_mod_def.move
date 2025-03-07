// invalid, one-time witness type candidate used in a different module

module a::n {
    use mys::mys;
    use mys::tx_context;

    fun init(_otw: mys::MYS, _ctx: &mut tx_context::TxContext) {
    }

}


module mys::tx_context {
    struct TxContext has drop {}
}

module mys::mys {
    struct MYS has drop {}
}
