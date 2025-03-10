---
title: Module `mys::versioned`
---



-  [Struct `Versioned`](#mys_versioned_Versioned)
-  [Struct `VersionChangeCap`](#mys_versioned_VersionChangeCap)
-  [Constants](#@Constants_0)
-  [Function `create`](#mys_versioned_create)
-  [Function `version`](#mys_versioned_version)
-  [Function `load_value`](#mys_versioned_load_value)
-  [Function `load_value_mut`](#mys_versioned_load_value_mut)
-  [Function `remove_value_for_upgrade`](#mys_versioned_remove_value_for_upgrade)
-  [Function `upgrade`](#mys_versioned_upgrade)
-  [Function `destroy`](#mys_versioned_destroy)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/dynamic_field.md#mys_dynamic_field">mys::dynamic_field</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
</code></pre>



<a name="mys_versioned_Versioned"></a>

## Struct `Versioned`

A wrapper type that supports versioning of the inner type.
The inner type is a dynamic field of the Versioned object, and is keyed using version.
User of this type could load the inner object using corresponding type based on the version.
You can also upgrade the inner object to a new type version.
If you want to support lazy upgrade of the inner type, one caveat is that all APIs would have
to use mutable reference even if it's a read-only API.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/versioned.md#mys_versioned_Versioned">Versioned</a> <b>has</b> key, store
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
<code><a href="../mys/versioned.md#mys_versioned_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_versioned_VersionChangeCap"></a>

## Struct `VersionChangeCap`

Represents a hot potato object generated when we take out the dynamic field.
This is to make sure that we always put a new value back.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/versioned.md#mys_versioned_VersionChangeCap">VersionChangeCap</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>versioned_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>old_version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_versioned_EInvalidUpgrade"></a>

Failed to upgrade the inner object due to invalid capability or new version.


<pre><code><b>const</b> <a href="../mys/versioned.md#mys_versioned_EInvalidUpgrade">EInvalidUpgrade</a>: u64 = 0;
</code></pre>



<a name="mys_versioned_create"></a>

## Function `create`

Create a new Versioned object that contains a initial value of type <code>T</code> with an initial version.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_create">create</a>&lt;T: store&gt;(init_version: u64, init_value: T, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/versioned.md#mys_versioned_Versioned">mys::versioned::Versioned</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_create">create</a>&lt;T: store&gt;(init_version: u64, init_value: T, ctx: &<b>mut</b> TxContext): <a href="../mys/versioned.md#mys_versioned_Versioned">Versioned</a> {
    <b>let</b> <b>mut</b> self = <a href="../mys/versioned.md#mys_versioned_Versioned">Versioned</a> {
        id: <a href="../mys/object.md#mys_object_new">object::new</a>(ctx),
        <a href="../mys/versioned.md#mys_versioned_version">version</a>: init_version,
    };
    <a href="../mys/dynamic_field.md#mys_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id, init_version, init_value);
    self
}
</code></pre>



</details>

<a name="mys_versioned_version"></a>

## Function `version`

Get the current version of the inner type.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_version">version</a>(self: &<a href="../mys/versioned.md#mys_versioned_Versioned">mys::versioned::Versioned</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_version">version</a>(self: &<a href="../mys/versioned.md#mys_versioned_Versioned">Versioned</a>): u64 {
    self.<a href="../mys/versioned.md#mys_versioned_version">version</a>
}
</code></pre>



</details>

<a name="mys_versioned_load_value"></a>

## Function `load_value`

Load the inner value based on the current version. Caller specifies an expected type T.
If the type mismatch, the load will fail.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_load_value">load_value</a>&lt;T: store&gt;(self: &<a href="../mys/versioned.md#mys_versioned_Versioned">mys::versioned::Versioned</a>): &T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_load_value">load_value</a>&lt;T: store&gt;(self: &<a href="../mys/versioned.md#mys_versioned_Versioned">Versioned</a>): &T {
    <a href="../mys/dynamic_field.md#mys_dynamic_field_borrow">dynamic_field::borrow</a>(&self.id, self.<a href="../mys/versioned.md#mys_versioned_version">version</a>)
}
</code></pre>



</details>

<a name="mys_versioned_load_value_mut"></a>

## Function `load_value_mut`

Similar to load_value, but return a mutable reference.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_load_value_mut">load_value_mut</a>&lt;T: store&gt;(self: &<b>mut</b> <a href="../mys/versioned.md#mys_versioned_Versioned">mys::versioned::Versioned</a>): &<b>mut</b> T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_load_value_mut">load_value_mut</a>&lt;T: store&gt;(self: &<b>mut</b> <a href="../mys/versioned.md#mys_versioned_Versioned">Versioned</a>): &<b>mut</b> T {
    <a href="../mys/dynamic_field.md#mys_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>(&<b>mut</b> self.id, self.<a href="../mys/versioned.md#mys_versioned_version">version</a>)
}
</code></pre>



</details>

<a name="mys_versioned_remove_value_for_upgrade"></a>

## Function `remove_value_for_upgrade`

Take the inner object out for upgrade. To ensure we always upgrade properly, a capability object is returned
and must be used when we upgrade.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_remove_value_for_upgrade">remove_value_for_upgrade</a>&lt;T: store&gt;(self: &<b>mut</b> <a href="../mys/versioned.md#mys_versioned_Versioned">mys::versioned::Versioned</a>): (T, <a href="../mys/versioned.md#mys_versioned_VersionChangeCap">mys::versioned::VersionChangeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_remove_value_for_upgrade">remove_value_for_upgrade</a>&lt;T: store&gt;(self: &<b>mut</b> <a href="../mys/versioned.md#mys_versioned_Versioned">Versioned</a>): (T, <a href="../mys/versioned.md#mys_versioned_VersionChangeCap">VersionChangeCap</a>) {
    (
        <a href="../mys/dynamic_field.md#mys_dynamic_field_remove">dynamic_field::remove</a>(&<b>mut</b> self.id, self.<a href="../mys/versioned.md#mys_versioned_version">version</a>),
        <a href="../mys/versioned.md#mys_versioned_VersionChangeCap">VersionChangeCap</a> {
            versioned_id: <a href="../mys/object.md#mys_object_id">object::id</a>(self),
            old_version: self.<a href="../mys/versioned.md#mys_versioned_version">version</a>,
        },
    )
}
</code></pre>



</details>

<a name="mys_versioned_upgrade"></a>

## Function `upgrade`

Upgrade the inner object with a new version and new value. Must use the capability returned
by calling remove_value_for_upgrade.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_upgrade">upgrade</a>&lt;T: store&gt;(self: &<b>mut</b> <a href="../mys/versioned.md#mys_versioned_Versioned">mys::versioned::Versioned</a>, new_version: u64, new_value: T, cap: <a href="../mys/versioned.md#mys_versioned_VersionChangeCap">mys::versioned::VersionChangeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_upgrade">upgrade</a>&lt;T: store&gt;(
    self: &<b>mut</b> <a href="../mys/versioned.md#mys_versioned_Versioned">Versioned</a>,
    new_version: u64,
    new_value: T,
    cap: <a href="../mys/versioned.md#mys_versioned_VersionChangeCap">VersionChangeCap</a>,
) {
    <b>let</b> <a href="../mys/versioned.md#mys_versioned_VersionChangeCap">VersionChangeCap</a> { versioned_id, old_version } = cap;
    <b>assert</b>!(versioned_id == <a href="../mys/object.md#mys_object_id">object::id</a>(self), <a href="../mys/versioned.md#mys_versioned_EInvalidUpgrade">EInvalidUpgrade</a>);
    <b>assert</b>!(old_version &lt; new_version, <a href="../mys/versioned.md#mys_versioned_EInvalidUpgrade">EInvalidUpgrade</a>);
    <a href="../mys/dynamic_field.md#mys_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id, new_version, new_value);
    self.<a href="../mys/versioned.md#mys_versioned_version">version</a> = new_version;
}
</code></pre>



</details>

<a name="mys_versioned_destroy"></a>

## Function `destroy`

Destroy this Versioned container, and return the inner object.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_destroy">destroy</a>&lt;T: store&gt;(self: <a href="../mys/versioned.md#mys_versioned_Versioned">mys::versioned::Versioned</a>): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/versioned.md#mys_versioned_destroy">destroy</a>&lt;T: store&gt;(self: <a href="../mys/versioned.md#mys_versioned_Versioned">Versioned</a>): T {
    <b>let</b> <a href="../mys/versioned.md#mys_versioned_Versioned">Versioned</a> { <b>mut</b> id, <a href="../mys/versioned.md#mys_versioned_version">version</a> } = self;
    <b>let</b> ret = <a href="../mys/dynamic_field.md#mys_dynamic_field_remove">dynamic_field::remove</a>(&<b>mut</b> id, <a href="../mys/versioned.md#mys_versioned_version">version</a>);
    id.delete();
    ret
}
</code></pre>



</details>
