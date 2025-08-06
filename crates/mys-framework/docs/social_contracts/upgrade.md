---
title: Module `social_contracts::upgrade`
---

Module to manage package upgrades for MySocialContracts.
Provides versioning support for all shared objects.


-  [Struct `UpgradeAdminCap`](#social_contracts_upgrade_UpgradeAdminCap)
-  [Struct `UpgradeEvent`](#social_contracts_upgrade_UpgradeEvent)
-  [Struct `ObjectMigratedEvent`](#social_contracts_upgrade_ObjectMigratedEvent)
-  [Constants](#@Constants_0)
-  [Function `init`](#social_contracts_upgrade_init)
-  [Function `authorize_upgrade`](#social_contracts_upgrade_authorize_upgrade)
-  [Function `commit_upgrade`](#social_contracts_upgrade_commit_upgrade)
-  [Function `version`](#social_contracts_upgrade_version)
-  [Function `package_id`](#social_contracts_upgrade_package_id)
-  [Function `current_version`](#social_contracts_upgrade_current_version)
-  [Function `assert_version`](#social_contracts_upgrade_assert_version)
-  [Function `emit_migration_event`](#social_contracts_upgrade_emit_migration_event)
-  [Function `create_upgrade_admin_cap`](#social_contracts_upgrade_create_upgrade_admin_cap)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/event.md#mys_event">mys::event</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/package.md#mys_package">mys::package</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_upgrade_UpgradeAdminCap"></a>

## Struct `UpgradeAdminCap`

Admin capability for package upgrades


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">UpgradeAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_upgrade_UpgradeEvent"></a>

## Struct `UpgradeEvent`

Event emitted when a package is upgraded


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeEvent">UpgradeEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/upgrade.md#social_contracts_upgrade_package_id">package_id</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_upgrade_ObjectMigratedEvent"></a>

## Struct `ObjectMigratedEvent`

Event emitted when a shared object is migrated to a new version


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_ObjectMigratedEvent">ObjectMigratedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>object_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>object_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>old_version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>migrated_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_upgrade_CURRENT_VERSION"></a>



<pre><code><b>const</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_CURRENT_VERSION">CURRENT_VERSION</a>: u64 = 1;
</code></pre>



<a name="social_contracts_upgrade_EInvalidDigest"></a>



<pre><code><b>const</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_EInvalidDigest">EInvalidDigest</a>: u64 = 0;
</code></pre>



<a name="social_contracts_upgrade_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_EWrongVersion">EWrongVersion</a>: u64 = 1;
</code></pre>



<a name="social_contracts_upgrade_init"></a>

## Function `init`

Module initializer - runs once when the package is published


<pre><code><b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_init">init</a>(_ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_init">init</a>(_ctx: &<b>mut</b> tx_context::TxContext) {
    // Admin capability creation is now handled by the <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">bootstrap</a> <b>module</b>
    // The UpgradeCap will be automatically transferred to the publisher
    // by the MySocial system when the package is published
}
</code></pre>



</details>

<a name="social_contracts_upgrade_authorize_upgrade"></a>

## Function `authorize_upgrade`

Authorize an upgrade with the upgrade cap


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_authorize_upgrade">authorize_upgrade</a>(cap: &<b>mut</b> <a href="../mys/package.md#mys_package_UpgradeCap">mys::package::UpgradeCap</a>, digest: vector&lt;u8&gt;): <a href="../mys/package.md#mys_package_UpgradeTicket">mys::package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_authorize_upgrade">authorize_upgrade</a>(
    cap: &<b>mut</b> package::UpgradeCap,
    digest: vector&lt;u8&gt;
): package::UpgradeTicket {
    // Verify digest length is 32 bytes
    <b>assert</b>!(vector::length(&digest) == 32, <a href="../social_contracts/upgrade.md#social_contracts_upgrade_EInvalidDigest">EInvalidDigest</a>);
    // Use default <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> policy
    <b>let</b> policy = cap.policy();
    // Return the <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> ticket
    cap.<a href="../social_contracts/upgrade.md#social_contracts_upgrade_authorize_upgrade">authorize_upgrade</a>(policy, digest)
}
</code></pre>



</details>

<a name="social_contracts_upgrade_commit_upgrade"></a>

## Function `commit_upgrade`

Commit an upgrade with the receipt


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_commit_upgrade">commit_upgrade</a>(cap: &<b>mut</b> <a href="../mys/package.md#mys_package_UpgradeCap">mys::package::UpgradeCap</a>, receipt: <a href="../mys/package.md#mys_package_UpgradeReceipt">mys::package::UpgradeReceipt</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_commit_upgrade">commit_upgrade</a>(
    cap: &<b>mut</b> package::UpgradeCap,
    receipt: package::UpgradeReceipt
) {
    // Emit <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> event
    event::emit(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeEvent">UpgradeEvent</a> {
        <a href="../social_contracts/upgrade.md#social_contracts_upgrade_package_id">package_id</a>: receipt.package(),
        <a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>: cap.<a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>() + 1
    });
    // Commit the <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a>
    cap.<a href="../social_contracts/upgrade.md#social_contracts_upgrade_commit_upgrade">commit_upgrade</a>(receipt);
}
</code></pre>



</details>

<a name="social_contracts_upgrade_version"></a>

## Function `version`

Get the current package version


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>(cap: &<a href="../mys/package.md#mys_package_UpgradeCap">mys::package::UpgradeCap</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>(cap: &package::UpgradeCap): u64 {
    cap.<a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>()
}
</code></pre>



</details>

<a name="social_contracts_upgrade_package_id"></a>

## Function `package_id`

Get the package ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_package_id">package_id</a>(cap: &<a href="../mys/package.md#mys_package_UpgradeCap">mys::package::UpgradeCap</a>): <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_package_id">package_id</a>(cap: &package::UpgradeCap): object::ID {
    cap.package()
}
</code></pre>



</details>

<a name="social_contracts_upgrade_current_version"></a>

## Function `current_version`

Get the current package version constant


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">current_version</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">current_version</a>(): u64 {
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_CURRENT_VERSION">CURRENT_VERSION</a>
}
</code></pre>



</details>

<a name="social_contracts_upgrade_assert_version"></a>

## Function `assert_version`

Check if the version matches the current package version


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_assert_version">assert_version</a>(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_assert_version">assert_version</a>(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a>: u64) {
    <b>assert</b>!(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../social_contracts/upgrade.md#social_contracts_upgrade_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_upgrade_emit_migration_event"></a>

## Function `emit_migration_event`

Helper function to emit migration event
This can be called directly by other modules implementing their own migration


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">emit_migration_event</a>(object_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, object_type: <a href="../std/string.md#std_string_String">std::string::String</a>, old_version: u64, migrated_by: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">emit_migration_event</a>(
    object_id: ID,
    object_type: String,
    old_version: u64,
    migrated_by: <b>address</b>
) {
    event::emit(<a href="../social_contracts/upgrade.md#social_contracts_upgrade_ObjectMigratedEvent">ObjectMigratedEvent</a> {
        object_id,
        object_type,
        old_version,
        new_version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_CURRENT_VERSION">CURRENT_VERSION</a>,
        migrated_by
    });
}
</code></pre>



</details>

<a name="social_contracts_upgrade_create_upgrade_admin_cap"></a>

## Function `create_upgrade_admin_cap`

Create an UpgradeAdminCap for bootstrap (package visibility only)
This function is only callable by other modules in the same package


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_create_upgrade_admin_cap">create_upgrade_admin_cap</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade_create_upgrade_admin_cap">create_upgrade_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">UpgradeAdminCap</a> {
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">UpgradeAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>
