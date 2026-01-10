---
title: Module `social_contracts::block_list`
---

Block list module for the MySocial network
Manages user blocking between wallet addresses


-  [Struct `BlockListRegistry`](#social_contracts_block_list_BlockListRegistry)
-  [Struct `UserBlockEvent`](#social_contracts_block_list_UserBlockEvent)
-  [Struct `UserUnblockEvent`](#social_contracts_block_list_UserUnblockEvent)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_block_list_bootstrap_init)
-  [Function `block_wallet_internal`](#social_contracts_block_list_block_wallet_internal)
-  [Function `block_wallet`](#social_contracts_block_list_block_wallet)
-  [Function `unblock_wallet_internal`](#social_contracts_block_list_unblock_wallet_internal)
-  [Function `unblock_wallet`](#social_contracts_block_list_unblock_wallet)
-  [Function `is_blocked`](#social_contracts_block_list_is_blocked)
-  [Function `blocked_count`](#social_contracts_block_list_blocked_count)
-  [Function `get_blocked_wallets`](#social_contracts_block_list_get_blocked_wallets)
-  [Function `registry_version`](#social_contracts_block_list_registry_version)
-  [Function `borrow_registry_version_mut`](#social_contracts_block_list_borrow_registry_version_mut)
-  [Function `migrate_block_list_registry`](#social_contracts_block_list_migrate_block_list_registry)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/dynamic_field.md#mys_dynamic_field">mys::dynamic_field</a>;
<b>use</b> <a href="../mys/event.md#mys_event">mys::event</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/package.md#mys_package">mys::package</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_block_list_BlockListRegistry"></a>

## Struct `BlockListRegistry`

Registry to track all block lists
Uses unified table architecture (like SocialGraph) for wallet-level blocking
Works for both users and platforms (both are just addresses)


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>blocked: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../mys/vec_set.md#mys_vec_set_VecSet">mys::vec_set::VecSet</a>&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
 Unified table: blocker_address -> set of blocked addresses
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_block_list_UserBlockEvent"></a>

## Struct `UserBlockEvent`

Block event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_UserBlockEvent">UserBlockEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>blocker: <b>address</b></code>
</dt>
<dd>
 The blocker wallet address (who initiated the block)
</dd>
<dt>
<code>blocked: <b>address</b></code>
</dt>
<dd>
 The blocked wallet address (who was blocked)
</dd>
</dl>


</details>

<a name="social_contracts_block_list_UserUnblockEvent"></a>

## Struct `UserUnblockEvent`

Unblock event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_UserUnblockEvent">UserUnblockEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>blocker: <b>address</b></code>
</dt>
<dd>
 The blocker wallet address (who initiated the unblock)
</dd>
<dt>
<code>unblocked: <b>address</b></code>
</dt>
<dd>
 The unblocked wallet address (who was unblocked)
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_block_list_EAlreadyBlocked"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_EAlreadyBlocked">EAlreadyBlocked</a>: u64 = 1;
</code></pre>



<a name="social_contracts_block_list_ECannotBlockSelf"></a>



<pre><code><b>const</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_ECannotBlockSelf">ECannotBlockSelf</a>: u64 = 3;
</code></pre>



<a name="social_contracts_block_list_ENotBlocked"></a>



<pre><code><b>const</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_ENotBlocked">ENotBlocked</a>: u64 = 2;
</code></pre>



<a name="social_contracts_block_list_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_EWrongVersion">EWrongVersion</a>: u64 = 4;
</code></pre>



<a name="social_contracts_block_list_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the block list registry


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> registry = <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a> {
        id: object::new(ctx),
        blocked: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Share the registry to make it globally accessible
    transfer::share_object(registry);
}
</code></pre>



</details>

<a name="social_contracts_block_list_block_wallet_internal"></a>

## Function `block_wallet_internal`

Internal helper function to block a wallet with a specific blocker address
This allows platforms (shared objects) to block wallets on behalf of their address
Uses unified table architecture with lazy initialization (like following)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_block_wallet_internal">block_wallet_internal</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, blocker_address: <b>address</b>, blocked_wallet_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_block_wallet_internal">block_wallet_internal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>,
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_graph::SocialGraph</a>,
    blocker_address: <b>address</b>,
    blocked_wallet_address: <b>address</b>
) {
    // Check version compatibility
    <b>assert</b>!(registry.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/block_list.md#social_contracts_block_list_EWrongVersion">EWrongVersion</a>);
    // Cannot block self
    <b>assert</b>!(blocker_address != blocked_wallet_address, <a href="../social_contracts/block_list.md#social_contracts_block_list_ECannotBlockSelf">ECannotBlockSelf</a>);
    // Initialize blocker's blocked set <b>if</b> it doesn't exist (lazy initialization)
    <b>if</b> (!table::contains(&registry.blocked, blocker_address)) {
        table::add(&<b>mut</b> registry.blocked, blocker_address, vec_set::empty());
    };
    // Get the blocked set and check <b>if</b> already blocked
    <b>let</b> blocked_set = table::borrow_mut(&<b>mut</b> registry.blocked, blocker_address);
    // Check <b>if</b> already blocked
    <b>if</b> (vec_set::contains(blocked_set, &blocked_wallet_address)) {
        <b>abort</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_EAlreadyBlocked">EAlreadyBlocked</a>
    };
    // Add to blocked set
    vec_set::insert(blocked_set, blocked_wallet_address);
    // Emit block event
    event::emit(<a href="../social_contracts/block_list.md#social_contracts_block_list_UserBlockEvent">UserBlockEvent</a> {
        blocker: blocker_address,
        blocked: blocked_wallet_address,
    });
    // Perform bidirectional unfollow after blocking succeeds (wallet-level)
    // Blocker unfollows blocked (<b>if</b> following)
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_internal">social_graph::unfollow_internal</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>, blocker_address, blocked_wallet_address);
    // Blocked unfollows blocker (<b>if</b> following)
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph_unfollow_internal">social_graph::unfollow_internal</a>(<a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>, blocked_wallet_address, blocker_address);
    // Continue - blocking succeeds regardless of unfollow results
}
</code></pre>



</details>

<a name="social_contracts_block_list_block_wallet"></a>

## Function `block_wallet`

Block a wallet address
Uses the caller's wallet address as the blocker
Automatically unfollows in both directions if users are following each other


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_block_wallet">block_wallet</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, blocked_wallet_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_block_wallet">block_wallet</a>(
    registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>,
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_graph::SocialGraph</a>,
    blocked_wallet_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/block_list.md#social_contracts_block_list_block_wallet_internal">block_wallet_internal</a>(registry, <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>, sender, blocked_wallet_address);
}
</code></pre>



</details>

<a name="social_contracts_block_list_unblock_wallet_internal"></a>

## Function `unblock_wallet_internal`

Internal helper function to unblock a wallet with a specific blocker address


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_unblock_wallet_internal">unblock_wallet_internal</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, blocker_address: <b>address</b>, blocked_wallet_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_unblock_wallet_internal">unblock_wallet_internal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>,
    blocker_address: <b>address</b>,
    blocked_wallet_address: <b>address</b>
) {
    // Check version compatibility
    <b>assert</b>!(registry.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/block_list.md#social_contracts_block_list_EWrongVersion">EWrongVersion</a>);
    // Check <b>if</b> blocker <b>has</b> any blocked addresses
    <b>if</b> (!table::contains(&registry.blocked, blocker_address)) {
        <b>abort</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_ENotBlocked">ENotBlocked</a>
    };
    // Get the blocked set
    <b>let</b> blocked_set = table::borrow_mut(&<b>mut</b> registry.blocked, blocker_address);
    // Check <b>if</b> the wallet is actually blocked
    <b>if</b> (!vec_set::contains(blocked_set, &blocked_wallet_address)) {
        <b>abort</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_ENotBlocked">ENotBlocked</a>
    };
    // Remove from blocked set
    vec_set::remove(blocked_set, &blocked_wallet_address);
    // Emit unblock event
    event::emit(<a href="../social_contracts/block_list.md#social_contracts_block_list_UserUnblockEvent">UserUnblockEvent</a> {
        blocker: blocker_address,
        unblocked: blocked_wallet_address,
    });
}
</code></pre>



</details>

<a name="social_contracts_block_list_unblock_wallet"></a>

## Function `unblock_wallet`

Unblock a wallet address
Uses the caller's wallet address as the blocker


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_unblock_wallet">unblock_wallet</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, blocked_wallet_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_unblock_wallet">unblock_wallet</a>(
    registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>,
    blocked_wallet_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/block_list.md#social_contracts_block_list_unblock_wallet_internal">unblock_wallet_internal</a>(registry, sender, blocked_wallet_address);
}
</code></pre>



</details>

<a name="social_contracts_block_list_is_blocked"></a>

## Function `is_blocked`

Check if a wallet address is blocked by a blocker


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">is_blocked</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, blocker: <b>address</b>, blocked: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">is_blocked</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>, blocker: <b>address</b>, blocked: <b>address</b>): bool {
    <b>if</b> (!table::contains(&registry.blocked, blocker)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> blocked_set = table::borrow(&registry.blocked, blocker);
    vec_set::contains(blocked_set, &blocked)
}
</code></pre>



</details>

<a name="social_contracts_block_list_blocked_count"></a>

## Function `blocked_count`

Get the number of blocked wallet addresses for a blocker


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_blocked_count">blocked_count</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, blocker: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_blocked_count">blocked_count</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>, blocker: <b>address</b>): u64 {
    <b>if</b> (!table::contains(&registry.blocked, blocker)) {
        <b>return</b> 0
    };
    <b>let</b> blocked_set = table::borrow(&registry.blocked, blocker);
    vec_set::size(blocked_set)
}
</code></pre>



</details>

<a name="social_contracts_block_list_get_blocked_wallets"></a>

## Function `get_blocked_wallets`

Get the list of blocked wallet addresses for a blocker


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets">get_blocked_wallets</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, blocker: <b>address</b>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets">get_blocked_wallets</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>, blocker: <b>address</b>): vector&lt;<b>address</b>&gt; {
    <b>if</b> (!table::contains(&registry.blocked, blocker)) {
        <b>return</b> <a href="../std/vector.md#std_vector_empty">std::vector::empty</a>()
    };
    <b>let</b> blocked_set = table::borrow(&registry.blocked, blocker);
    vec_set::into_keys(*blocked_set)
}
</code></pre>



</details>

<a name="social_contracts_block_list_registry_version"></a>

## Function `registry_version`

Get the version of the block list registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_registry_version">registry_version</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_registry_version">registry_version</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>): u64 {
    registry.version
}
</code></pre>



</details>

<a name="social_contracts_block_list_borrow_registry_version_mut"></a>

## Function `borrow_registry_version_mut`

Get a mutable reference to the registry version (for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>): &<b>mut</b> u64 {
    &<b>mut</b> registry.version
}
</code></pre>



</details>

<a name="social_contracts_block_list_migrate_block_list_registry"></a>

## Function `migrate_block_list_registry`

Migration function for BlockListRegistry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_migrate_block_list_registry">migrate_block_list_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_migrate_block_list_registry">migrate_block_list_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(registry.version &lt; current_version, <a href="../social_contracts/block_list.md#social_contracts_block_list_EWrongVersion">EWrongVersion</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = registry.version;
    registry.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> registry_id = object::id(registry);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        registry_id,
        string::utf8(b"<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>
