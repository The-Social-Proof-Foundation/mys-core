module a::trigger_lint_cases {
    use mys::object::UID;

    // This should trigger the linter warning (true positive)
    struct MissingKeyAbility {
        id: UID,
    }

}

module mys::object {
    struct UID has store {
        id: address,
    }
}
