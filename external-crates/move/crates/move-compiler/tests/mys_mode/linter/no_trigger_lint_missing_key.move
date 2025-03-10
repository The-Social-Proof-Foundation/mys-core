module a::no_trigger_lint_cases {
    use mys::object::UID;

    // This should not trigger the linter warning (true negative)
    struct HasKeyAbility has key {
        id: UID,
    }
}

module mys::object {
    struct UID has store {
        id: address,
    }
}
