---
title: Module `social_contracts::block_list`
---

Block list module for the MySocial network
Manages user blocking between wallet addresses


-  [Struct `BlockList`](#social_contracts_block_list_BlockList)
-  [Struct `BlockListRegistry`](#social_contracts_block_list_BlockListRegistry)
-  [Struct `UserBlockEvent`](#social_contracts_block_list_UserBlockEvent)
-  [Struct `UserUnblockEvent`](#social_contracts_block_list_UserUnblockEvent)
-  [Struct `BlockListCreatedEvent`](#social_contracts_block_list_BlockListCreatedEvent)
-  [Constants](#@Constants_0)
-  [Function `create_block_list`](#social_contracts_block_list_create_block_list)
-  [Function `create_block_list_for_sender`](#social_contracts_block_list_create_block_list_for_sender)
-  [Function `init`](#social_contracts_block_list_init)
-  [Function `get_blocked_wallets_key`](#social_contracts_block_list_get_blocked_wallets_key)
-  [Function `block_wallet`](#social_contracts_block_list_block_wallet)
-  [Function `unblock_wallet`](#social_contracts_block_list_unblock_wallet)
-  [Function `has_block_list`](#social_contracts_block_list_has_block_list)
-  [Function `find_block_list_id`](#social_contracts_block_list_find_block_list_id)
-  [Function `is_blocked`](#social_contracts_block_list_is_blocked)
-  [Function `blocked_count`](#social_contracts_block_list_blocked_count)
-  [Function `get_blocked_wallets`](#social_contracts_block_list_get_blocked_wallets)
-  [Function `version`](#social_contracts_block_list_version)
-  [Function `borrow_version_mut`](#social_contracts_block_list_borrow_version_mut)
-  [Function `registry_version`](#social_contracts_block_list_registry_version)
-  [Function `borrow_registry_version_mut`](#social_contracts_block_list_borrow_registry_version_mut)
-  [Function `migrate_block_list`](#social_contracts_block_list_migrate_block_list)
-  [Function `migrate_block_list_registry`](#social_contracts_block_list_migrate_block_list_registry)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bcs.md#mys_bcs">mys::bcs</a>;
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
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_block_list_BlockList"></a>

## Struct `BlockList`

Block list for a user's wallet


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">BlockList</a> <b>has</b> key
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
<code>owner: <b>address</b></code>
</dt>
<dd>
 The wallet address this block list belongs to
</dd>
<dt>
<code><a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_block_list_BlockListRegistry"></a>

## Struct `BlockListRegistry`

Registry to track all block lists


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
<code>wallet_block_lists: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <b>address</b>&gt;</code>
</dt>
<dd>
 Table mapping wallet addresses to block list IDs
</dd>
<dt>
<code><a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>: u64</code>
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

<a name="social_contracts_block_list_BlockListCreatedEvent"></a>

## Struct `BlockListCreatedEvent`

Event emitted when a block list is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListCreatedEvent">BlockListCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>block_list_id: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_block_list_BLOCKED_WALLETS_KEY"></a>

Key for storing blocked wallets in the registry


<pre><code><b>const</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BLOCKED_WALLETS_KEY">BLOCKED_WALLETS_KEY</a>: vector&lt;u8&gt; = vector[98, 108, 111, 99, 107, 101, 100, 95, 119, 97, 108, 108, 101, 116, 115];
</code></pre>



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



<a name="social_contracts_block_list_create_block_list"></a>

## Function `create_block_list`

Create a new block list


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_create_block_list">create_block_list</a>(owner: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">social_contracts::block_list::BlockList</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_create_block_list">create_block_list</a>(owner: <b>address</b>, ctx: &<b>mut</b> tx_context::TxContext): <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">BlockList</a> {
    <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">BlockList</a> {
        id: object::new(ctx),
        owner,
        <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    }
}
</code></pre>



</details>

<a name="social_contracts_block_list_create_block_list_for_sender"></a>

## Function `create_block_list_for_sender`

Create a new block list for the sender
This is an explicit operation to create a block list, even if not blocking anyone yet


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_create_block_list_for_sender">create_block_list_for_sender</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_create_block_list_for_sender">create_block_list_for_sender</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>, ctx: &<b>mut</b> tx_context::TxContext) {
    <b>let</b> sender = tx_context::sender(ctx);
    // Check <b>if</b> a block list already exists <b>for</b> the sender
    <b>if</b> (table::contains(&registry.wallet_block_lists, sender)) {
        <b>return</b>
    };
    // Create a new block list
    <b>let</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a> = <a href="../social_contracts/block_list.md#social_contracts_block_list_create_block_list">create_block_list</a>(sender, ctx);
    <b>let</b> block_list_id = object::uid_to_address(&<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>.id);
    // Register the block list
    table::add(&<b>mut</b> registry.wallet_block_lists, sender, block_list_id);
    // Initialize an empty blocked wallets set in the registry
    dynamic_field::add(&<b>mut</b> registry.id, <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets_key">get_blocked_wallets_key</a>(sender), vec_set::empty&lt;<b>address</b>&gt;());
    // Emit block list created event
    event::emit(<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListCreatedEvent">BlockListCreatedEvent</a> {
        owner: sender,
        block_list_id,
    });
    // Return the block list to the caller
    transfer::transfer(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, sender);
}
</code></pre>



</details>

<a name="social_contracts_block_list_init"></a>

## Function `init`

Module initializer to create the block list registry


<pre><code><b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_init">init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_init">init</a>(ctx: &<b>mut</b> tx_context::TxContext) {
    <b>let</b> registry = <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a> {
        id: object::new(ctx),
        wallet_block_lists: table::new(ctx),
        <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Share the registry to make it globally accessible
    transfer::share_object(registry);
}
</code></pre>



</details>

<a name="social_contracts_block_list_get_blocked_wallets_key"></a>

## Function `get_blocked_wallets_key`

Generate a unique key for storing a user's blocked wallets


<pre><code><b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets_key">get_blocked_wallets_key</a>(user_address: <b>address</b>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets_key">get_blocked_wallets_key</a>(user_address: <b>address</b>): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> key = <a href="../social_contracts/block_list.md#social_contracts_block_list_BLOCKED_WALLETS_KEY">BLOCKED_WALLETS_KEY</a>;
    <b>let</b> address_bytes = <a href="../mys/bcs.md#mys_bcs_to_bytes">mys::bcs::to_bytes</a>(&user_address);
    <a href="../std/vector.md#std_vector_append">std::vector::append</a>(&<b>mut</b> key, address_bytes);
    key
}
</code></pre>



</details>

<a name="social_contracts_block_list_block_wallet"></a>

## Function `block_wallet`

Block a wallet address
Uses the caller's wallet address as the blocker


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_block_wallet">block_wallet</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, blocked_wallet_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_block_wallet">block_wallet</a>(
    registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>,
    blocked_wallet_address: <b>address</b>,
    ctx: &<b>mut</b> tx_context::TxContext
) {
    // Get the sender <b>address</b> (wallet <b>address</b> of the blocker)
    <b>let</b> sender = tx_context::sender(ctx);
    // Cannot block self
    <b>assert</b>!(sender != blocked_wallet_address, <a href="../social_contracts/block_list.md#social_contracts_block_list_ECannotBlockSelf">ECannotBlockSelf</a>);
    // Check <b>if</b> sender already <b>has</b> a block list
    <b>let</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_has_block_list">has_block_list</a> = table::contains(&registry.wallet_block_lists, sender);
    <b>if</b> (<a href="../social_contracts/block_list.md#social_contracts_block_list_has_block_list">has_block_list</a>) {
        // Get key <b>for</b> finding blocked wallets
        <b>let</b> key = <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets_key">get_blocked_wallets_key</a>(sender);
        // Get the blocked wallets set from registry
        <b>let</b> blocked_wallets = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<b>mut</b> registry.id, key);
        // Check <b>if</b> already blocked
        <b>if</b> (vec_set::contains(blocked_wallets, &blocked_wallet_address)) {
            <b>abort</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_EAlreadyBlocked">EAlreadyBlocked</a>
        };
        // Add to blocked wallets
        vec_set::insert(blocked_wallets, blocked_wallet_address);
        // Emit block event
        event::emit(<a href="../social_contracts/block_list.md#social_contracts_block_list_UserBlockEvent">UserBlockEvent</a> {
            blocker: sender,
            blocked: blocked_wallet_address,
        });
    } <b>else</b> {
        // Create a new block list <b>for</b> first-time blockers
        <b>let</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a> = <a href="../social_contracts/block_list.md#social_contracts_block_list_create_block_list">create_block_list</a>(sender, ctx);
        <b>let</b> block_list_id = object::uid_to_address(&<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>.id);
        // Register the block list
        table::add(&<b>mut</b> registry.wallet_block_lists, sender, block_list_id);
        // Create a new blocked wallets set with the blocked <b>address</b>
        <b>let</b> <b>mut</b> blocked_wallets = vec_set::empty&lt;<b>address</b>&gt;();
        vec_set::insert(&<b>mut</b> blocked_wallets, blocked_wallet_address);
        // Add the blocked wallets set to the registry
        dynamic_field::add(&<b>mut</b> registry.id, <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets_key">get_blocked_wallets_key</a>(sender), blocked_wallets);
        // Emit block list created event
        event::emit(<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListCreatedEvent">BlockListCreatedEvent</a> {
            owner: sender,
            block_list_id,
        });
        // Emit block event
        event::emit(<a href="../social_contracts/block_list.md#social_contracts_block_list_UserBlockEvent">UserBlockEvent</a> {
            blocker: sender,
            blocked: blocked_wallet_address,
        });
        // Return the block list to the caller
        transfer::transfer(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>, sender);
    }
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
    ctx: &<b>mut</b> tx_context::TxContext
) {
    // Get the sender <b>address</b> (wallet <b>address</b> of the blocker)
    <b>let</b> sender = tx_context::sender(ctx);
    // Check <b>if</b> there's a block list <b>for</b> this wallet
    <b>if</b> (!table::contains(&registry.wallet_block_lists, sender)) {
        <b>abort</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_ENotBlocked">ENotBlocked</a>
    };
    // Get key <b>for</b> finding blocked wallets
    <b>let</b> key = <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets_key">get_blocked_wallets_key</a>(sender);
    // Check <b>if</b> blocked wallets set exists
    <b>if</b> (!dynamic_field::exists_(&registry.id, key)) {
        <b>abort</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_ENotBlocked">ENotBlocked</a>
    };
    // Get the blocked wallets set
    <b>let</b> blocked_wallets = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<b>mut</b> registry.id, key);
    // Check <b>if</b> the wallet is actually blocked
    <b>if</b> (!vec_set::contains(blocked_wallets, &blocked_wallet_address)) {
        <b>abort</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_ENotBlocked">ENotBlocked</a>
    };
    // Remove from blocked wallets
    vec_set::remove(blocked_wallets, &blocked_wallet_address);
    // Emit unblock event
    event::emit(<a href="../social_contracts/block_list.md#social_contracts_block_list_UserUnblockEvent">UserUnblockEvent</a> {
        blocker: sender,
        unblocked: blocked_wallet_address,
    });
}
</code></pre>



</details>

<a name="social_contracts_block_list_has_block_list"></a>

## Function `has_block_list`

Check if a wallet has a block list


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_has_block_list">has_block_list</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, wallet_address: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_has_block_list">has_block_list</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>, wallet_address: <b>address</b>): bool {
    table::contains(&registry.wallet_block_lists, wallet_address)
}
</code></pre>



</details>

<a name="social_contracts_block_list_find_block_list_id"></a>

## Function `find_block_list_id`

Find a block list ID for a wallet address


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_find_block_list_id">find_block_list_id</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, wallet_address: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_find_block_list_id">find_block_list_id</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>, wallet_address: <b>address</b>): option::Option&lt;<b>address</b>&gt; {
    <b>if</b> (table::contains(&registry.wallet_block_lists, wallet_address)) {
        option::some(*table::borrow(&registry.wallet_block_lists, wallet_address))
    } <b>else</b> {
        option::none()
    }
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
    // First check <b>if</b> blocker <b>has</b> a block list
    <b>if</b> (!table::contains(&registry.wallet_block_lists, blocker)) {
        <b>return</b> <b>false</b>
    };
    // Get key <b>for</b> finding blocked wallets
    <b>let</b> key = <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets_key">get_blocked_wallets_key</a>(blocker);
    // Check <b>if</b> blocked wallets set exists
    <b>if</b> (!dynamic_field::exists_(&registry.id, key)) {
        <b>return</b> <b>false</b>
    };
    // Get the blocked wallets set and check <b>if</b> blocked <b>address</b> is in it
    <b>let</b> blocked_wallets = dynamic_field::borrow&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&registry.id, key);
    vec_set::contains(blocked_wallets, &blocked)
}
</code></pre>



</details>

<a name="social_contracts_block_list_blocked_count"></a>

## Function `blocked_count`

Get the number of blocked wallet addresses


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_blocked_count">blocked_count</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, blocker: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_blocked_count">blocked_count</a>(registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>, blocker: <b>address</b>): u64 {
    // First check <b>if</b> blocker <b>has</b> a block list
    <b>if</b> (!table::contains(&registry.wallet_block_lists, blocker)) {
        <b>return</b> 0
    };
    // Get key <b>for</b> finding blocked wallets
    <b>let</b> key = <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets_key">get_blocked_wallets_key</a>(blocker);
    // Check <b>if</b> blocked wallets set exists
    <b>if</b> (!dynamic_field::exists_(&registry.id, key)) {
        <b>return</b> 0
    };
    // Get the blocked wallets set and <b>return</b> its size
    <b>let</b> blocked_wallets = dynamic_field::borrow&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&registry.id, key);
    vec_set::size(blocked_wallets)
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
    // First check <b>if</b> blocker <b>has</b> a block list
    <b>if</b> (!table::contains(&registry.wallet_block_lists, blocker)) {
        <b>return</b> <a href="../std/vector.md#std_vector_empty">std::vector::empty</a>()
    };
    // Get key <b>for</b> finding blocked wallets
    <b>let</b> key = <a href="../social_contracts/block_list.md#social_contracts_block_list_get_blocked_wallets_key">get_blocked_wallets_key</a>(blocker);
    // Check <b>if</b> blocked wallets set exists
    <b>if</b> (!dynamic_field::exists_(&registry.id, key)) {
        <b>return</b> <a href="../std/vector.md#std_vector_empty">std::vector::empty</a>()
    };
    // Get the blocked wallets set and <b>return</b> its contents
    <b>let</b> blocked_wallets = dynamic_field::borrow&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&registry.id, key);
    vec_set::into_keys(*blocked_wallets)
}
</code></pre>



</details>

<a name="social_contracts_block_list_version"></a>

## Function `version`

Get the version of a block list


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">social_contracts::block_list::BlockList</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">BlockList</a>): u64 {
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>.<a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_block_list_borrow_version_mut"></a>

## Function `borrow_version_mut`

Get a mutable reference to the block list version (for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">social_contracts::block_list::BlockList</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">BlockList</a>): &<b>mut</b> u64 {
    &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>.<a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>
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
    registry.<a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_block_list_borrow_registry_version_mut"></a>

## Function `borrow_registry_version_mut`

Get a mutable reference to the registry version (for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">BlockListRegistry</a>): &<b>mut</b> u64 {
    &<b>mut</b> registry.<a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_block_list_migrate_block_list"></a>

## Function `migrate_block_list`

Migration function for BlockList


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_migrate_block_list">migrate_block_list</a>(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">social_contracts::block_list::BlockList</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_migrate_block_list">migrate_block_list</a>(
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">BlockList</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> tx_context::TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a> &gt; current <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>)
    <b>assert</b>!(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>.<a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a> &lt; current_version, <a href="../social_contracts/block_list.md#social_contracts_block_list_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a> and update to new <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>
    <b>let</b> old_version = <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>.<a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>;
    <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>.<a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> block_list_id = object::id(<a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        block_list_id,
        string::utf8(b"<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockList">BlockList</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
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
    ctx: &<b>mut</b> tx_context::TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a> &gt; current <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>)
    <b>assert</b>!(registry.<a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a> &lt; current_version, <a href="../social_contracts/block_list.md#social_contracts_block_list_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a> and update to new <a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>
    <b>let</b> old_version = registry.<a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a>;
    registry.<a href="../social_contracts/block_list.md#social_contracts_block_list_version">version</a> = current_version;
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
