// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Centralized one-time bootstrap key for framework and platform initialization.
/// Ensures all admin capabilities can only be created once during initial bootstrap.
module mys::bootstrap_key;

// === ERROR CODES ===

/// Bootstrap key has already been used
const EAlreadyUsed: u64 = 0;
/// Sender is not @0x0 the system address.
const ENotSystemAddress: u64 = 1;

/// One-time bootstrap key - protects all admin capability creation
public struct BootstrapKey has key {
    id: UID,
    used: bool,
    version: u64,
}

/// Creates the shared BootstrapKey on module publication
public fun bootstrap_init(ctx: &mut TxContext) {
    assert!(ctx.sender() == @0x0, ENotSystemAddress);

    transfer::share_object(BootstrapKey {
        id: object::new(ctx),
        used: false,
        version: 1,
    });
}

public fun is_used(key: &BootstrapKey): bool {
    key.used
}

public fun version(key: &BootstrapKey): u64 {
    key.version
}

/// Aborts if the key has already been used
public fun assert_not_used(key: &BootstrapKey) {
    assert!(!key.used, EAlreadyUsed);
}

/// Finalize bootstrap by marking the key as used (irreversible)
/// This should ONLY be called after all admin capabilities have been created and distributed.
/// Combines the check and mark in one operation to prevent DOS attacks.
public fun finalize_bootstrap(key: &mut BootstrapKey) {
    assert_not_used(key);
    key.used = true;
}

#[test_only]
public fun init_for_testing(ctx: &mut TxContext) {
    bootstrap_init(ctx)
}

#[test_only]
public fun create_test_bootstrap_key(ctx: &mut TxContext): BootstrapKey {
    BootstrapKey {
        id: object::new(ctx),
        used: false,
        version: 1,
    }
}

#[test_only]
public fun mark_used_for_testing(key: &mut BootstrapKey) {
    key.used = true;
}

#[test_only]
public fun destroy_for_testing(key: BootstrapKey) {
    let BootstrapKey { id, used: _, version: _ } = key;
    object::delete(id);
}
