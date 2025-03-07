module a::edge_cases {
    struct UID {}
    // Test case with a different UID type
    struct DifferentUID {
        id: mys::another::UID,
    }

    struct NotAnObject {
        id: UID,
    }

}

module mys::object {
    struct UID has store {
        id: address,
    }
}

module mys::another {
    struct UID has store {
        id: address,
    }
}
