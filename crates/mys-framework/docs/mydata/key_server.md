---
title: Module `mydata::key_server`
---



-  [Struct `KeyServer`](#mydata_key_server_KeyServer)
-  [Struct `KeyServerV1`](#mydata_key_server_KeyServerV1)
-  [Constants](#@Constants_0)
-  [Function `create_and_transfer_v1`](#mydata_key_server_create_and_transfer_v1)
-  [Function `v1`](#mydata_key_server_v1)
-  [Function `name`](#mydata_key_server_name)
-  [Function `url`](#mydata_key_server_url)
-  [Function `key_type`](#mydata_key_server_key_type)
-  [Function `pk`](#mydata_key_server_pk)
-  [Function `id`](#mydata_key_server_id)
-  [Function `pk_as_bf_bls12381`](#mydata_key_server_pk_as_bf_bls12381)
-  [Function `update`](#mydata_key_server_update)
-  [Function `create_v1`](#mydata_key_server_create_v1)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bcs.md#mys_bcs">mys::bcs</a>;
<b>use</b> <a href="../mys/bls12381.md#mys_bls12381">mys::bls12381</a>;
<b>use</b> <a href="../mys/dynamic_field.md#mys_dynamic_field">mys::dynamic_field</a>;
<b>use</b> <a href="../mys/group_ops.md#mys_group_ops">mys::group_ops</a>;
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



<a name="mydata_key_server_KeyServer"></a>

## Struct `KeyServer`

KeyServer should always be guarded as it's a capability
on its own. It should either be an owned object, wrapped object,
or TTO'd object (where access to it is controlled externally).


<pre><code><b>public</b> <b>struct</b> <a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../mydata/key_server.md#mydata_key_server_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>first_version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>last_version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_key_server_KeyServerV1"></a>

## Struct `KeyServerV1`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/key_server.md#mydata_key_server_KeyServerV1">KeyServerV1</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../mydata/key_server.md#mydata_key_server_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/key_server.md#mydata_key_server_url">url</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mydata_key_server_EInvalidKeyType"></a>



<pre><code><b>const</b> <a href="../mydata/key_server.md#mydata_key_server_EInvalidKeyType">EInvalidKeyType</a>: u64 = 1;
</code></pre>



<a name="mydata_key_server_EInvalidVersion"></a>



<pre><code><b>const</b> <a href="../mydata/key_server.md#mydata_key_server_EInvalidVersion">EInvalidVersion</a>: u64 = 2;
</code></pre>



<a name="mydata_key_server_KEY_TYPE_BONEH_FRANKLIN_BLS12381"></a>



<pre><code><b>const</b> <a href="../mydata/key_server.md#mydata_key_server_KEY_TYPE_BONEH_FRANKLIN_BLS12381">KEY_TYPE_BONEH_FRANKLIN_BLS12381</a>: u8 = 0;
</code></pre>



<a name="mydata_key_server_create_and_transfer_v1"></a>

## Function `create_and_transfer_v1`



<pre><code><b>entry</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_create_and_transfer_v1">create_and_transfer_v1</a>(<a href="../mydata/key_server.md#mydata_key_server_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../mydata/key_server.md#mydata_key_server_url">url</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a>: u8, <a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_create_and_transfer_v1">create_and_transfer_v1</a>(
    <a href="../mydata/key_server.md#mydata_key_server_name">name</a>: String,
    <a href="../mydata/key_server.md#mydata_key_server_url">url</a>: String,
    <a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a>: u8,
    <a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <a href="../mydata/key_server.md#mydata_key_server">key_server</a> = <a href="../mydata/key_server.md#mydata_key_server_create_v1">create_v1</a>(<a href="../mydata/key_server.md#mydata_key_server_name">name</a>, <a href="../mydata/key_server.md#mydata_key_server_url">url</a>, <a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a>, <a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>, ctx);
    transfer::transfer(<a href="../mydata/key_server.md#mydata_key_server">key_server</a>, ctx.sender());
}
</code></pre>



</details>

<a name="mydata_key_server_v1"></a>

## Function `v1`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">mydata::key_server::KeyServer</a>): &<a href="../mydata/key_server.md#mydata_key_server_KeyServerV1">mydata::key_server::KeyServerV1</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a>): &<a href="../mydata/key_server.md#mydata_key_server_KeyServerV1">KeyServerV1</a> {
    <b>assert</b>!(df::exists_(&s.<a href="../mydata/key_server.md#mydata_key_server_id">id</a>, 1), <a href="../mydata/key_server.md#mydata_key_server_EInvalidVersion">EInvalidVersion</a>);
    df::borrow(&s.<a href="../mydata/key_server.md#mydata_key_server_id">id</a>, 1)
}
</code></pre>



</details>

<a name="mydata_key_server_name"></a>

## Function `name`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_name">name</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">mydata::key_server::KeyServer</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_name">name</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a>): String {
    s.<a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>().<a href="../mydata/key_server.md#mydata_key_server_name">name</a>
}
</code></pre>



</details>

<a name="mydata_key_server_url"></a>

## Function `url`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_url">url</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">mydata::key_server::KeyServer</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_url">url</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a>): String {
    s.<a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>().<a href="../mydata/key_server.md#mydata_key_server_url">url</a>
}
</code></pre>



</details>

<a name="mydata_key_server_key_type"></a>

## Function `key_type`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">mydata::key_server::KeyServer</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a>): u8 {
    s.<a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>().<a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a>
}
</code></pre>



</details>

<a name="mydata_key_server_pk"></a>

## Function `pk`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">mydata::key_server::KeyServer</a>): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a>): &vector&lt;u8&gt; {
    &s.<a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>().<a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>
}
</code></pre>



</details>

<a name="mydata_key_server_id"></a>

## Function `id`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_id">id</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">mydata::key_server::KeyServer</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_id">id</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a>): <b>address</b> {
    s.<a href="../mydata/key_server.md#mydata_key_server_id">id</a>.to_address()
}
</code></pre>



</details>

<a name="mydata_key_server_pk_as_bf_bls12381"></a>

## Function `pk_as_bf_bls12381`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_pk_as_bf_bls12381">pk_as_bf_bls12381</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">mydata::key_server::KeyServer</a>): <a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_pk_as_bf_bls12381">pk_as_bf_bls12381</a>(s: &<a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a>): Element&lt;G2&gt; {
    <b>let</b> <a href="../mydata/key_server.md#mydata_key_server_v1">v1</a> = s.<a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>();
    <b>assert</b>!(<a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>.<a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a> == <a href="../mydata/key_server.md#mydata_key_server_KEY_TYPE_BONEH_FRANKLIN_BLS12381">KEY_TYPE_BONEH_FRANKLIN_BLS12381</a>, <a href="../mydata/key_server.md#mydata_key_server_EInvalidKeyType">EInvalidKeyType</a>);
    g2_from_bytes(&<a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>.<a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>)
}
</code></pre>



</details>

<a name="mydata_key_server_update"></a>

## Function `update`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_update">update</a>(s: &<b>mut</b> <a href="../mydata/key_server.md#mydata_key_server_KeyServer">mydata::key_server::KeyServer</a>, <a href="../mydata/key_server.md#mydata_key_server_url">url</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_update">update</a>(s: &<b>mut</b> <a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a>, <a href="../mydata/key_server.md#mydata_key_server_url">url</a>: String) {
    <b>assert</b>!(df::exists_(&s.<a href="../mydata/key_server.md#mydata_key_server_id">id</a>, 1), <a href="../mydata/key_server.md#mydata_key_server_EInvalidVersion">EInvalidVersion</a>);
    <b>let</b> <a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>: &<b>mut</b> <a href="../mydata/key_server.md#mydata_key_server_KeyServerV1">KeyServerV1</a> = df::borrow_mut(&<b>mut</b> s.<a href="../mydata/key_server.md#mydata_key_server_id">id</a>, 1);
    <a href="../mydata/key_server.md#mydata_key_server_v1">v1</a>.<a href="../mydata/key_server.md#mydata_key_server_url">url</a> = <a href="../mydata/key_server.md#mydata_key_server_url">url</a>;
}
</code></pre>



</details>

<a name="mydata_key_server_create_v1"></a>

## Function `create_v1`



<pre><code><b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_create_v1">create_v1</a>(<a href="../mydata/key_server.md#mydata_key_server_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../mydata/key_server.md#mydata_key_server_url">url</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a>: u8, <a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mydata/key_server.md#mydata_key_server_KeyServer">mydata::key_server::KeyServer</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/key_server.md#mydata_key_server_create_v1">create_v1</a>(
    <a href="../mydata/key_server.md#mydata_key_server_name">name</a>: String,
    <a href="../mydata/key_server.md#mydata_key_server_url">url</a>: String,
    <a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a>: u8,
    <a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
): <a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a> {
    // Currently only BLS12-381 is supported.
    <b>assert</b>!(<a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a> == <a href="../mydata/key_server.md#mydata_key_server_KEY_TYPE_BONEH_FRANKLIN_BLS12381">KEY_TYPE_BONEH_FRANKLIN_BLS12381</a>, <a href="../mydata/key_server.md#mydata_key_server_EInvalidKeyType">EInvalidKeyType</a>);
    <b>let</b> _ = g2_from_bytes(&<a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>);
    <b>let</b> <b>mut</b> <a href="../mydata/key_server.md#mydata_key_server">key_server</a> = <a href="../mydata/key_server.md#mydata_key_server_KeyServer">KeyServer</a> {
        <a href="../mydata/key_server.md#mydata_key_server_id">id</a>: object::new(ctx),
        first_version: 1,
        last_version: 1,
    };
    <b>let</b> key_server_v1 = <a href="../mydata/key_server.md#mydata_key_server_KeyServerV1">KeyServerV1</a> {
        <a href="../mydata/key_server.md#mydata_key_server_name">name</a>,
        <a href="../mydata/key_server.md#mydata_key_server_url">url</a>,
        <a href="../mydata/key_server.md#mydata_key_server_key_type">key_type</a>,
        <a href="../mydata/key_server.md#mydata_key_server_pk">pk</a>,
    };
    df::add(&<b>mut</b> <a href="../mydata/key_server.md#mydata_key_server">key_server</a>.<a href="../mydata/key_server.md#mydata_key_server_id">id</a>, 1, key_server_v1);
    <a href="../mydata/key_server.md#mydata_key_server">key_server</a>
}
</code></pre>



</details>
