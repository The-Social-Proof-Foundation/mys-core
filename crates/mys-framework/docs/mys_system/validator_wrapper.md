---
title: Module `mys_system::validator_wrapper`
---



-  [Struct `ValidatorWrapper`](#mys_system_validator_wrapper_ValidatorWrapper)
-  [Constants](#@Constants_0)
-  [Function `create_v1`](#mys_system_validator_wrapper_create_v1)
-  [Function `load_validator_maybe_upgrade`](#mys_system_validator_wrapper_load_validator_maybe_upgrade)
-  [Function `destroy`](#mys_system_validator_wrapper_destroy)
-  [Function `upgrade_to_latest`](#mys_system_validator_wrapper_upgrade_to_latest)
-  [Function `version`](#mys_system_validator_wrapper_version)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bag.md#mys_bag">mys::bag</a>;
<b>use</b> <a href="../mys/balance.md#mys_balance">mys::balance</a>;
<b>use</b> <a href="../mys/coin.md#mys_coin">mys::coin</a>;
<b>use</b> <a href="../mys/config.md#mys_config">mys::config</a>;
<b>use</b> <a href="../mys/deny_list.md#mys_deny_list">mys::deny_list</a>;
<b>use</b> <a href="../mys/dynamic_field.md#mys_dynamic_field">mys::dynamic_field</a>;
<b>use</b> <a href="../mys/dynamic_object_field.md#mys_dynamic_object_field">mys::dynamic_object_field</a>;
<b>use</b> <a href="../mys/event.md#mys_event">mys::event</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/mys.md#mys_mys">mys::mys</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../mys/versioned.md#mys_versioned">mys::versioned</a>;
<b>use</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool">mys_system::staking_pool</a>;
<b>use</b> <a href="../mys_system/validator.md#mys_system_validator">mys_system::validator</a>;
<b>use</b> <a href="../mys_system/validator_cap.md#mys_system_validator_cap">mys_system::validator_cap</a>;
</code></pre>



<a name="mys_system_validator_wrapper_ValidatorWrapper"></a>

## Struct `ValidatorWrapper`



<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: <a href="../mys/versioned.md#mys_versioned_Versioned">mys::versioned::Versioned</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_system_validator_wrapper_EInvalidVersion"></a>



<pre><code><b>const</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_EInvalidVersion">EInvalidVersion</a>: u64 = 0;
</code></pre>



<a name="mys_system_validator_wrapper_create_v1"></a>

## Function `create_v1`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_create_v1">create_v1</a>(<a href="../mys_system/validator.md#mys_system_validator">validator</a>: <a href="../mys_system/validator.md#mys_system_validator_Validator">mys_system::validator::Validator</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">mys_system::validator_wrapper::ValidatorWrapper</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_create_v1">create_v1</a>(<a href="../mys_system/validator.md#mys_system_validator">validator</a>: Validator, ctx: &<b>mut</b> TxContext): <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> {
    <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> {
        inner: versioned::create(1, <a href="../mys_system/validator.md#mys_system_validator">validator</a>, ctx)
    }
}
</code></pre>



</details>

<a name="mys_system_validator_wrapper_load_validator_maybe_upgrade"></a>

## Function `load_validator_maybe_upgrade`

This function should always return the latest supported version.
If the inner version is old, we upgrade it lazily in-place.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">mys_system::validator_wrapper::ValidatorWrapper</a>): &<b>mut</b> <a href="../mys_system/validator.md#mys_system_validator_Validator">mys_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): &<b>mut</b> Validator {
    <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self);
    versioned::load_value_mut(&<b>mut</b> self.inner)
}
</code></pre>



</details>

<a name="mys_system_validator_wrapper_destroy"></a>

## Function `destroy`

Destroy the wrapper and retrieve the inner validator object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_destroy">destroy</a>(self: <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">mys_system::validator_wrapper::ValidatorWrapper</a>): <a href="../mys_system/validator.md#mys_system_validator_Validator">mys_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_destroy">destroy</a>(self: <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): Validator {
    <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(&self);
    <b>let</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> { inner } = self;
    versioned::destroy(inner)
}
</code></pre>



</details>

<a name="mys_system_validator_wrapper_upgrade_to_latest"></a>

## Function `upgrade_to_latest`



<pre><code><b>fun</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">mys_system::validator_wrapper::ValidatorWrapper</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>) {
    <b>let</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_version">version</a> = <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_version">version</a>(self);
    // TODO: When new versions are added, we need to explicitly upgrade here.
    <b>assert</b>!(<a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_version">version</a> == 1, <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_EInvalidVersion">EInvalidVersion</a>);
}
</code></pre>



</details>

<a name="mys_system_validator_wrapper_version"></a>

## Function `version`



<pre><code><b>fun</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_version">version</a>(self: &<a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">mys_system::validator_wrapper::ValidatorWrapper</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_version">version</a>(self: &<a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): u64 {
    versioned::version(&self.inner)
}
</code></pre>



</details>
