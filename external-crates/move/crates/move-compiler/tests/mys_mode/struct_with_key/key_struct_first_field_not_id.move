// invalid, first field of an ojbect must be mys::object::UID
module a::m {
    struct S has key {
        flag: bool
    }
}
