---
title: Module `mys::object`
---

MySocial object identifiers


-  [Struct `ID`](#mys_object_ID)
-  [Struct `UID`](#mys_object_UID)
-  [Constants](#@Constants_0)
-  [Function `id_to_bytes`](#mys_object_id_to_bytes)
-  [Function `id_to_address`](#mys_object_id_to_address)
-  [Function `id_from_bytes`](#mys_object_id_from_bytes)
-  [Function `id_from_address`](#mys_object_id_from_address)
-  [Function `mys_system_state`](#mys_object_mys_system_state)
-  [Function `clock`](#mys_object_clock)
-  [Function `authenticator_state`](#mys_object_authenticator_state)
-  [Function `randomness_state`](#mys_object_randomness_state)
-  [Function `mys_deny_list_object_id`](#mys_object_mys_deny_list_object_id)
-  [Function `bridge`](#mys_object_bridge)
-  [Function `uid_as_inner`](#mys_object_uid_as_inner)
-  [Function `uid_to_inner`](#mys_object_uid_to_inner)
-  [Function `uid_to_bytes`](#mys_object_uid_to_bytes)
-  [Function `uid_to_address`](#mys_object_uid_to_address)
-  [Function `new`](#mys_object_new)
-  [Function `delete`](#mys_object_delete)
-  [Function `id`](#mys_object_id)
-  [Function `borrow_id`](#mys_object_borrow_id)
-  [Function `id_bytes`](#mys_object_id_bytes)
-  [Function `id_address`](#mys_object_id_address)
-  [Function `borrow_uid`](#mys_object_borrow_uid)
-  [Function `new_uid_from_hash`](#mys_object_new_uid_from_hash)
-  [Function `delete_impl`](#mys_object_delete_impl)
-  [Function `record_new_uid`](#mys_object_record_new_uid)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
</code></pre>



<a name="mys_object_ID"></a>

## Struct `ID`

An object ID. This is used to reference MySocial Objects.
This is *not* guaranteed to be globally unique--anyone can create an <code><a href="../mys/object.md#mys_object_ID">ID</a></code> from a <code><a href="../mys/object.md#mys_object_UID">UID</a></code> or
from an object, and ID's can be freely copied and dropped.
Here, the values are not globally unique because there can be multiple values of type <code><a href="../mys/object.md#mys_object_ID">ID</a></code>
with the same underlying bytes. For example, <code><a href="../mys/object.md#mys_object_id">object::id</a>(&obj)</code> can be called as many times
as you want for a given <code>obj</code>, and each <code><a href="../mys/object.md#mys_object_ID">ID</a></code> value will be identical.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/object.md#mys_object_ID">ID</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bytes: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_object_UID"></a>

## Struct `UID`

Globally unique IDs that define an object's ID in storage. Any MySocial Object, that is a struct
with the <code>key</code> ability, must have <code><a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_UID">UID</a></code> as its first field.
These are globally unique in the sense that no two values of type <code><a href="../mys/object.md#mys_object_UID">UID</a></code> are ever equal, in
other words for any two values <code>id1: <a href="../mys/object.md#mys_object_UID">UID</a></code> and <code>id2: <a href="../mys/object.md#mys_object_UID">UID</a></code>, <code>id1</code> != <code>id2</code>.
This is a privileged type that can only be derived from a <code>TxContext</code>.
<code><a href="../mys/object.md#mys_object_UID">UID</a></code> doesn't have the <code>drop</code> ability, so deleting a <code><a href="../mys/object.md#mys_object_UID">UID</a></code> requires a call to <code><a href="../mys/object.md#mys_object_delete">delete</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/object.md#mys_object_UID">UID</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_object_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../mys/object.md#mys_object_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="mys_object_MYS_AUTHENTICATOR_STATE_ID"></a>

The hardcoded ID for the singleton AuthenticatorState Object.


<pre><code><b>const</b> <a href="../mys/object.md#mys_object_MYS_AUTHENTICATOR_STATE_ID">MYS_AUTHENTICATOR_STATE_ID</a>: <b>address</b> = 0x7;
</code></pre>



<a name="mys_object_MYS_BRIDGE_ID"></a>

The hardcoded ID for the Bridge Object.


<pre><code><b>const</b> <a href="../mys/object.md#mys_object_MYS_BRIDGE_ID">MYS_BRIDGE_ID</a>: <b>address</b> = 0x9;
</code></pre>



<a name="mys_object_MYS_CLOCK_OBJECT_ID"></a>

The hardcoded ID for the singleton Clock Object.


<pre><code><b>const</b> <a href="../mys/object.md#mys_object_MYS_CLOCK_OBJECT_ID">MYS_CLOCK_OBJECT_ID</a>: <b>address</b> = 0x6;
</code></pre>



<a name="mys_object_MYS_DENY_LIST_OBJECT_ID"></a>

The hardcoded ID for the singleton DenyList.


<pre><code><b>const</b> <a href="../mys/object.md#mys_object_MYS_DENY_LIST_OBJECT_ID">MYS_DENY_LIST_OBJECT_ID</a>: <b>address</b> = 0x403;
</code></pre>



<a name="mys_object_MYS_RANDOM_ID"></a>

The hardcoded ID for the singleton Random Object.


<pre><code><b>const</b> <a href="../mys/object.md#mys_object_MYS_RANDOM_ID">MYS_RANDOM_ID</a>: <b>address</b> = 0x8;
</code></pre>



<a name="mys_object_MYS_SYSTEM_STATE_OBJECT_ID"></a>

The hardcoded ID for the singleton MySocial System State Object.


<pre><code><b>const</b> <a href="../mys/object.md#mys_object_MYS_SYSTEM_STATE_OBJECT_ID">MYS_SYSTEM_STATE_OBJECT_ID</a>: <b>address</b> = 0x5;
</code></pre>



<a name="mys_object_id_to_bytes"></a>

## Function `id_to_bytes`

Get the raw bytes of a <code><a href="../mys/object.md#mys_object_ID">ID</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_to_bytes">id_to_bytes</a>(<a href="../mys/object.md#mys_object_id">id</a>: &<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_to_bytes">id_to_bytes</a>(<a href="../mys/object.md#mys_object_id">id</a>: &<a href="../mys/object.md#mys_object_ID">ID</a>): vector&lt;u8&gt; {
    <a href="../mys/bcs.md#mys_bcs_to_bytes">bcs::to_bytes</a>(&<a href="../mys/object.md#mys_object_id">id</a>.bytes)
}
</code></pre>



</details>

<a name="mys_object_id_to_address"></a>

## Function `id_to_address`

Get the inner bytes of <code><a href="../mys/object.md#mys_object_id">id</a></code> as an address.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_to_address">id_to_address</a>(<a href="../mys/object.md#mys_object_id">id</a>: &<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_to_address">id_to_address</a>(<a href="../mys/object.md#mys_object_id">id</a>: &<a href="../mys/object.md#mys_object_ID">ID</a>): <b>address</b> {
    <a href="../mys/object.md#mys_object_id">id</a>.bytes
}
</code></pre>



</details>

<a name="mys_object_id_from_bytes"></a>

## Function `id_from_bytes`

Make an <code><a href="../mys/object.md#mys_object_ID">ID</a></code> from raw bytes.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_from_bytes">id_from_bytes</a>(bytes: vector&lt;u8&gt;): <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_from_bytes">id_from_bytes</a>(bytes: vector&lt;u8&gt;): <a href="../mys/object.md#mys_object_ID">ID</a> {
    <a href="../mys/address.md#mys_address_from_bytes">address::from_bytes</a>(bytes).to_id()
}
</code></pre>



</details>

<a name="mys_object_id_from_address"></a>

## Function `id_from_address`

Make an <code><a href="../mys/object.md#mys_object_ID">ID</a></code> from an address.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_from_address">id_from_address</a>(bytes: <b>address</b>): <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_from_address">id_from_address</a>(bytes: <b>address</b>): <a href="../mys/object.md#mys_object_ID">ID</a> {
    <a href="../mys/object.md#mys_object_ID">ID</a> { bytes }
}
</code></pre>



</details>

<a name="mys_object_mys_system_state"></a>

## Function `mys_system_state`

Create the <code><a href="../mys/object.md#mys_object_UID">UID</a></code> for the singleton <code>MysSystemState</code> object.
This should only be called once from <code>mys_system</code>.


<pre><code><b>fun</b> <a href="../mys/object.md#mys_object_mys_system_state">mys_system_state</a>(ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys/object.md#mys_object_mys_system_state">mys_system_state</a>(ctx: &TxContext): <a href="../mys/object.md#mys_object_UID">UID</a> {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../mys/object.md#mys_object_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../mys/object.md#mys_object_UID">UID</a> {
        <a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_ID">ID</a> { bytes: <a href="../mys/object.md#mys_object_MYS_SYSTEM_STATE_OBJECT_ID">MYS_SYSTEM_STATE_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="mys_object_clock"></a>

## Function `clock`

Create the <code><a href="../mys/object.md#mys_object_UID">UID</a></code> for the singleton <code>Clock</code> object.
This should only be called once from <code><a href="../mys/clock.md#mys_clock">clock</a></code>.


<pre><code><b>public</b>(<a href="../mys/package.md#mys_package">package</a>) <b>fun</b> <a href="../mys/clock.md#mys_clock">clock</a>(): <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../mys/package.md#mys_package">package</a>) <b>fun</b> <a href="../mys/clock.md#mys_clock">clock</a>(): <a href="../mys/object.md#mys_object_UID">UID</a> {
    <a href="../mys/object.md#mys_object_UID">UID</a> {
        <a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_ID">ID</a> { bytes: <a href="../mys/object.md#mys_object_MYS_CLOCK_OBJECT_ID">MYS_CLOCK_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="mys_object_authenticator_state"></a>

## Function `authenticator_state`

Create the <code><a href="../mys/object.md#mys_object_UID">UID</a></code> for the singleton <code>AuthenticatorState</code> object.
This should only be called once from <code><a href="../mys/authenticator_state.md#mys_authenticator_state">authenticator_state</a></code>.


<pre><code><b>public</b>(<a href="../mys/package.md#mys_package">package</a>) <b>fun</b> <a href="../mys/authenticator_state.md#mys_authenticator_state">authenticator_state</a>(): <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../mys/package.md#mys_package">package</a>) <b>fun</b> <a href="../mys/authenticator_state.md#mys_authenticator_state">authenticator_state</a>(): <a href="../mys/object.md#mys_object_UID">UID</a> {
    <a href="../mys/object.md#mys_object_UID">UID</a> {
        <a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_ID">ID</a> { bytes: <a href="../mys/object.md#mys_object_MYS_AUTHENTICATOR_STATE_ID">MYS_AUTHENTICATOR_STATE_ID</a> },
    }
}
</code></pre>



</details>

<a name="mys_object_randomness_state"></a>

## Function `randomness_state`

Create the <code><a href="../mys/object.md#mys_object_UID">UID</a></code> for the singleton <code>Random</code> object.
This should only be called once from <code><a href="../mys/random.md#mys_random">random</a></code>.


<pre><code><b>public</b>(<a href="../mys/package.md#mys_package">package</a>) <b>fun</b> <a href="../mys/object.md#mys_object_randomness_state">randomness_state</a>(): <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../mys/package.md#mys_package">package</a>) <b>fun</b> <a href="../mys/object.md#mys_object_randomness_state">randomness_state</a>(): <a href="../mys/object.md#mys_object_UID">UID</a> {
    <a href="../mys/object.md#mys_object_UID">UID</a> {
        <a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_ID">ID</a> { bytes: <a href="../mys/object.md#mys_object_MYS_RANDOM_ID">MYS_RANDOM_ID</a> },
    }
}
</code></pre>



</details>

<a name="mys_object_mys_deny_list_object_id"></a>

## Function `mys_deny_list_object_id`

Create the <code><a href="../mys/object.md#mys_object_UID">UID</a></code> for the singleton <code>DenyList</code> object.
This should only be called once from <code><a href="../mys/deny_list.md#mys_deny_list">deny_list</a></code>.


<pre><code><b>public</b>(<a href="../mys/package.md#mys_package">package</a>) <b>fun</b> <a href="../mys/object.md#mys_object_mys_deny_list_object_id">mys_deny_list_object_id</a>(): <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../mys/package.md#mys_package">package</a>) <b>fun</b> <a href="../mys/object.md#mys_object_mys_deny_list_object_id">mys_deny_list_object_id</a>(): <a href="../mys/object.md#mys_object_UID">UID</a> {
    <a href="../mys/object.md#mys_object_UID">UID</a> {
        <a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_ID">ID</a> { bytes: <a href="../mys/object.md#mys_object_MYS_DENY_LIST_OBJECT_ID">MYS_DENY_LIST_OBJECT_ID</a> },
    }
}
</code></pre>



</details>

<a name="mys_object_bridge"></a>

## Function `bridge`

Create the <code><a href="../mys/object.md#mys_object_UID">UID</a></code> for the singleton <code>Bridge</code> object.
This should only be called once from <code><a href="../mys/object.md#mys_object_bridge">bridge</a></code>.


<pre><code><b>fun</b> <a href="../mys/object.md#mys_object_bridge">bridge</a>(): <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys/object.md#mys_object_bridge">bridge</a>(): <a href="../mys/object.md#mys_object_UID">UID</a> {
    <a href="../mys/object.md#mys_object_UID">UID</a> {
        <a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_ID">ID</a> { bytes: <a href="../mys/object.md#mys_object_MYS_BRIDGE_ID">MYS_BRIDGE_ID</a> },
    }
}
</code></pre>



</details>

<a name="mys_object_uid_as_inner"></a>

## Function `uid_as_inner`

Get the inner <code><a href="../mys/object.md#mys_object_ID">ID</a></code> of <code>uid</code>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_uid_as_inner">uid_as_inner</a>(uid: &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>): &<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_uid_as_inner">uid_as_inner</a>(uid: &<a href="../mys/object.md#mys_object_UID">UID</a>): &<a href="../mys/object.md#mys_object_ID">ID</a> {
    &uid.<a href="../mys/object.md#mys_object_id">id</a>
}
</code></pre>



</details>

<a name="mys_object_uid_to_inner"></a>

## Function `uid_to_inner`

Get the raw bytes of a <code>uid</code>'s inner <code><a href="../mys/object.md#mys_object_ID">ID</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_uid_to_inner">uid_to_inner</a>(uid: &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>): <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_uid_to_inner">uid_to_inner</a>(uid: &<a href="../mys/object.md#mys_object_UID">UID</a>): <a href="../mys/object.md#mys_object_ID">ID</a> {
    uid.<a href="../mys/object.md#mys_object_id">id</a>
}
</code></pre>



</details>

<a name="mys_object_uid_to_bytes"></a>

## Function `uid_to_bytes`

Get the raw bytes of a <code><a href="../mys/object.md#mys_object_UID">UID</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_uid_to_bytes">uid_to_bytes</a>(uid: &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_uid_to_bytes">uid_to_bytes</a>(uid: &<a href="../mys/object.md#mys_object_UID">UID</a>): vector&lt;u8&gt; {
    <a href="../mys/bcs.md#mys_bcs_to_bytes">bcs::to_bytes</a>(&uid.<a href="../mys/object.md#mys_object_id">id</a>.bytes)
}
</code></pre>



</details>

<a name="mys_object_uid_to_address"></a>

## Function `uid_to_address`

Get the inner bytes of <code><a href="../mys/object.md#mys_object_id">id</a></code> as an address.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_uid_to_address">uid_to_address</a>(uid: &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_uid_to_address">uid_to_address</a>(uid: &<a href="../mys/object.md#mys_object_UID">UID</a>): <b>address</b> {
    uid.<a href="../mys/object.md#mys_object_id">id</a>.bytes
}
</code></pre>



</details>

<a name="mys_object_new"></a>

## Function `new`

Create a new object. Returns the <code><a href="../mys/object.md#mys_object_UID">UID</a></code> that must be stored in a MySocial object.
This is the only way to create <code><a href="../mys/object.md#mys_object_UID">UID</a></code>s.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_new">new</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../mys/object.md#mys_object_UID">UID</a> {
    <a href="../mys/object.md#mys_object_UID">UID</a> {
        <a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_ID">ID</a> { bytes: ctx.fresh_object_address() },
    }
}
</code></pre>



</details>

<a name="mys_object_delete"></a>

## Function `delete`

Delete the object and its <code><a href="../mys/object.md#mys_object_UID">UID</a></code>. This is the only way to eliminate a <code><a href="../mys/object.md#mys_object_UID">UID</a></code>.
This exists to inform MySocial of object deletions. When an object
gets unpacked, the programmer will have to do something with its
<code><a href="../mys/object.md#mys_object_UID">UID</a></code>. The implementation of this function emits a deleted
system event so MySocial knows to process the object deletion


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_delete">delete</a>(<a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_delete">delete</a>(<a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_UID">UID</a>) {
    <b>let</b> <a href="../mys/object.md#mys_object_UID">UID</a> { <a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_ID">ID</a> { bytes } } = <a href="../mys/object.md#mys_object_id">id</a>;
    <a href="../mys/object.md#mys_object_delete_impl">delete_impl</a>(bytes)
}
</code></pre>



</details>

<a name="mys_object_id"></a>

## Function `id`

Get the underlying <code><a href="../mys/object.md#mys_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id">id</a>&lt;T: key&gt;(obj: &T): <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id">id</a>&lt;T: key&gt;(obj: &T): <a href="../mys/object.md#mys_object_ID">ID</a> {
    <a href="../mys/object.md#mys_object_borrow_uid">borrow_uid</a>(obj).<a href="../mys/object.md#mys_object_id">id</a>
}
</code></pre>



</details>

<a name="mys_object_borrow_id"></a>

## Function `borrow_id`

Borrow the underlying <code><a href="../mys/object.md#mys_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_borrow_id">borrow_id</a>&lt;T: key&gt;(obj: &T): &<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_borrow_id">borrow_id</a>&lt;T: key&gt;(obj: &T): &<a href="../mys/object.md#mys_object_ID">ID</a> {
    &<a href="../mys/object.md#mys_object_borrow_uid">borrow_uid</a>(obj).<a href="../mys/object.md#mys_object_id">id</a>
}
</code></pre>



</details>

<a name="mys_object_id_bytes"></a>

## Function `id_bytes`

Get the raw bytes for the underlying <code><a href="../mys/object.md#mys_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_bytes">id_bytes</a>&lt;T: key&gt;(obj: &T): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_bytes">id_bytes</a>&lt;T: key&gt;(obj: &T): vector&lt;u8&gt; {
    <a href="../mys/bcs.md#mys_bcs_to_bytes">bcs::to_bytes</a>(&<a href="../mys/object.md#mys_object_borrow_uid">borrow_uid</a>(obj).<a href="../mys/object.md#mys_object_id">id</a>)
}
</code></pre>



</details>

<a name="mys_object_id_address"></a>

## Function `id_address`

Get the inner bytes for the underlying <code><a href="../mys/object.md#mys_object_ID">ID</a></code> of <code>obj</code>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_address">id_address</a>&lt;T: key&gt;(obj: &T): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/object.md#mys_object_id_address">id_address</a>&lt;T: key&gt;(obj: &T): <b>address</b> {
    <a href="../mys/object.md#mys_object_borrow_uid">borrow_uid</a>(obj).<a href="../mys/object.md#mys_object_id">id</a>.bytes
}
</code></pre>



</details>

<a name="mys_object_borrow_uid"></a>

## Function `borrow_uid`

Get the <code><a href="../mys/object.md#mys_object_UID">UID</a></code> for <code>obj</code>.
Safe because MySocial has an extra bytecode verifier pass that forces every struct with
the <code>key</code> ability to have a distinguished <code><a href="../mys/object.md#mys_object_UID">UID</a></code> field.
Cannot be made public as the access to <code><a href="../mys/object.md#mys_object_UID">UID</a></code> for a given object must be privileged, and
restrictable in the object's module.


<pre><code><b>fun</b> <a href="../mys/object.md#mys_object_borrow_uid">borrow_uid</a>&lt;T: key&gt;(obj: &T): &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../mys/object.md#mys_object_borrow_uid">borrow_uid</a>&lt;T: key&gt;(obj: &T): &<a href="../mys/object.md#mys_object_UID">UID</a>;
</code></pre>



</details>

<a name="mys_object_new_uid_from_hash"></a>

## Function `new_uid_from_hash`

Generate a new UID specifically used for creating a UID from a hash


<pre><code><b>public</b>(<a href="../mys/package.md#mys_package">package</a>) <b>fun</b> <a href="../mys/object.md#mys_object_new_uid_from_hash">new_uid_from_hash</a>(bytes: <b>address</b>): <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../mys/package.md#mys_package">package</a>) <b>fun</b> <a href="../mys/object.md#mys_object_new_uid_from_hash">new_uid_from_hash</a>(bytes: <b>address</b>): <a href="../mys/object.md#mys_object_UID">UID</a> {
    <a href="../mys/object.md#mys_object_record_new_uid">record_new_uid</a>(bytes);
    <a href="../mys/object.md#mys_object_UID">UID</a> { <a href="../mys/object.md#mys_object_id">id</a>: <a href="../mys/object.md#mys_object_ID">ID</a> { bytes } }
}
</code></pre>



</details>

<a name="mys_object_delete_impl"></a>

## Function `delete_impl`



<pre><code><b>fun</b> <a href="../mys/object.md#mys_object_delete_impl">delete_impl</a>(<a href="../mys/object.md#mys_object_id">id</a>: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../mys/object.md#mys_object_delete_impl">delete_impl</a>(<a href="../mys/object.md#mys_object_id">id</a>: <b>address</b>);
</code></pre>



</details>

<a name="mys_object_record_new_uid"></a>

## Function `record_new_uid`



<pre><code><b>fun</b> <a href="../mys/object.md#mys_object_record_new_uid">record_new_uid</a>(<a href="../mys/object.md#mys_object_id">id</a>: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../mys/object.md#mys_object_record_new_uid">record_new_uid</a>(<a href="../mys/object.md#mys_object_id">id</a>: <b>address</b>);
</code></pre>



</details>
