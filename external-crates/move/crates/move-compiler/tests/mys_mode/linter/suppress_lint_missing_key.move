module a::trigger_lint_cases {
    use mys::object::UID;

    // 4. Suppress warning
    #[allow(lint(missing_key))]
    struct SuppressWarning {
       id: UID,
    }
}

module mys::object {
    struct UID has store {
        id: address,
    }
}
