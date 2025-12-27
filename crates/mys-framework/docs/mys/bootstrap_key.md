---
title: Module `mys::bootstrap_key`
---

Centralized one-time bootstrap key for framework and platform initialization.
Ensures all admin capabilities can only be created once during initial bootstrap.


-  [Struct `BootstrapKey`](#mys_bootstrap_key_BootstrapKey)
-  [Constants](#@Constants_0)
-  [Function `init`](#mys_bootstrap_key_init)
-  [Function `is_used`](#mys_bootstrap_key_is_used)
-  [Function `version`](#mys_bootstrap_key_version)
-  [Function `assert_not_used`](#mys_bootstrap_key_assert_not_used)
-  [Function `finalize_bootstrap`](#mys_bootstrap_key_finalize_bootstrap)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="mys_bootstrap_key_BootstrapKey"></a>

## Struct `BootstrapKey`

One-time bootstrap key - protects all admin capability creation


<pre><code><b>public</b> <b>struct</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">BootstrapKey</a> <b>has</b> key
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
<code>used: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mys/bootstrap_key.md#mys_bootstrap_key_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_bootstrap_key_EAlreadyUsed"></a>

Bootstrap key has already been used


<pre><code><b>const</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_EAlreadyUsed">EAlreadyUsed</a>: u64 = 0;
</code></pre>



<a name="mys_bootstrap_key_init"></a>

## Function `init`

Creates the shared BootstrapKey on module publication


<pre><code><b>fun</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_init">init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_init">init</a>(ctx: &<b>mut</b> TxContext) {
    <a href="../mys/transfer.md#mys_transfer_share_object">transfer::share_object</a>(<a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">BootstrapKey</a> {
        id: <a href="../mys/object.md#mys_object_new">object::new</a>(ctx),
        used: <b>false</b>,
        <a href="../mys/bootstrap_key.md#mys_bootstrap_key_version">version</a>: 1,
    });
}
</code></pre>



</details>

<a name="mys_bootstrap_key_is_used"></a>

## Function `is_used`



<pre><code><b>public</b> <b>fun</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_is_used">is_used</a>(key: &<a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">mys::bootstrap_key::BootstrapKey</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_is_used">is_used</a>(key: &<a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">BootstrapKey</a>): bool {
    key.used
}
</code></pre>



</details>

<a name="mys_bootstrap_key_version"></a>

## Function `version`



<pre><code><b>public</b> <b>fun</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_version">version</a>(key: &<a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">mys::bootstrap_key::BootstrapKey</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_version">version</a>(key: &<a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">BootstrapKey</a>): u64 {
    key.<a href="../mys/bootstrap_key.md#mys_bootstrap_key_version">version</a>
}
</code></pre>



</details>

<a name="mys_bootstrap_key_assert_not_used"></a>

## Function `assert_not_used`

Aborts if the key has already been used


<pre><code><b>public</b> <b>fun</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_assert_not_used">assert_not_used</a>(key: &<a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">mys::bootstrap_key::BootstrapKey</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_assert_not_used">assert_not_used</a>(key: &<a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">BootstrapKey</a>) {
    <b>assert</b>!(!key.used, <a href="../mys/bootstrap_key.md#mys_bootstrap_key_EAlreadyUsed">EAlreadyUsed</a>);
}
</code></pre>



</details>

<a name="mys_bootstrap_key_finalize_bootstrap"></a>

## Function `finalize_bootstrap`

Finalize bootstrap by marking the key as used (irreversible)
This should ONLY be called after all admin capabilities have been created and distributed.
Combines the check and mark in one operation to prevent DOS attacks.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_finalize_bootstrap">finalize_bootstrap</a>(key: &<b>mut</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">mys::bootstrap_key::BootstrapKey</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_finalize_bootstrap">finalize_bootstrap</a>(key: &<b>mut</b> <a href="../mys/bootstrap_key.md#mys_bootstrap_key_BootstrapKey">BootstrapKey</a>) {
    <a href="../mys/bootstrap_key.md#mys_bootstrap_key_assert_not_used">assert_not_used</a>(key);
    key.used = <b>true</b>;
}
</code></pre>



</details>
