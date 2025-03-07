module mys::object {
    public struct ID()
    public struct UID()
}
module mys::transfer {}
module mys::tx_context {
    public struct TxContext()
}

module a::m {
    use mys::object::{Self, ID, UID};
    use mys::transfer;
    use mys::tx_context::{Self, TxContext};
}
