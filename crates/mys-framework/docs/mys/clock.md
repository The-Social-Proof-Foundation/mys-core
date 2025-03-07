---
title: Module `mys::clock`
---

APIs for accessing time from move calls, via the <code><a href="../mys/clock.md#mys_clock_Clock">Clock</a></code>: a unique
shared object that is created at 0x6 during genesis.


-  [Struct `Clock`](#mys_clock_Clock)
-  [Constants](#@Constants_0)
-  [Function `timestamp_ms`](#mys_clock_timestamp_ms)
-  [Function `create`](#mys_clock_create)
-  [Function `consensus_commit_prologue`](#mys_clock_consensus_commit_prologue)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
</code></pre>



<a name="mys_clock_Clock"></a>

## Struct `Clock`

Singleton shared object that exposes time to Move calls.  This
object is found at address 0x6, and can only be read (accessed
via an immutable reference) by entry functions.

Entry Functions that attempt to accept <code><a href="../mys/clock.md#mys_clock_Clock">Clock</a></code> by mutable
reference or value will fail to verify, and honest validators
will not sign or execute transactions that use <code><a href="../mys/clock.md#mys_clock_Clock">Clock</a></code> as an
input parameter, unless it is passed by immutable reference.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/clock.md#mys_clock_Clock">Clock</a> <b>has</b> key
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
<code><a href="../mys/clock.md#mys_clock_timestamp_ms">timestamp_ms</a>: u64</code>
</dt>
<dd>
 The clock's timestamp, which is set automatically by a
 system transaction every time consensus commits a
 schedule, or by <code>mys::clock::increment_for_testing</code> during
 testing.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_clock_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../mys/clock.md#mys_clock_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="mys_clock_timestamp_ms"></a>

## Function `timestamp_ms`

The <code><a href="../mys/clock.md#mys_clock">clock</a></code>'s current timestamp as a running total of
milliseconds since an arbitrary point in the past.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/clock.md#mys_clock_timestamp_ms">timestamp_ms</a>(<a href="../mys/clock.md#mys_clock">clock</a>: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/clock.md#mys_clock_timestamp_ms">timestamp_ms</a>(<a href="../mys/clock.md#mys_clock">clock</a>: &<a href="../mys/clock.md#mys_clock_Clock">Clock</a>): u64 {
    <a href="../mys/clock.md#mys_clock">clock</a>.<a href="../mys/clock.md#mys_clock_timestamp_ms">timestamp_ms</a>
}
</code></pre>



</details>

<a name="mys_clock_create"></a>

## Function `create`

Create and share the singleton Clock -- this function is
called exactly once, during genesis.


<pre><code><b>fun</b> <a href="../mys/clock.md#mys_clock_create">create</a>(ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys/clock.md#mys_clock_create">create</a>(ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../mys/clock.md#mys_clock_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../mys/transfer.md#mys_transfer_share_object">transfer::share_object</a>(<a href="../mys/clock.md#mys_clock_Clock">Clock</a> {
        id: <a href="../mys/object.md#mys_object_clock">object::clock</a>(),
        // Initialised to zero, but set to a real timestamp by a
        // system transaction before it can be witnessed by a <b>move</b>
        // call.
        <a href="../mys/clock.md#mys_clock_timestamp_ms">timestamp_ms</a>: 0,
    })
}
</code></pre>



</details>

<a name="mys_clock_consensus_commit_prologue"></a>

## Function `consensus_commit_prologue`



<pre><code><b>fun</b> <a href="../mys/clock.md#mys_clock_consensus_commit_prologue">consensus_commit_prologue</a>(<a href="../mys/clock.md#mys_clock">clock</a>: &<b>mut</b> <a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, <a href="../mys/clock.md#mys_clock_timestamp_ms">timestamp_ms</a>: u64, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys/clock.md#mys_clock_consensus_commit_prologue">consensus_commit_prologue</a>(<a href="../mys/clock.md#mys_clock">clock</a>: &<b>mut</b> <a href="../mys/clock.md#mys_clock_Clock">Clock</a>, <a href="../mys/clock.md#mys_clock_timestamp_ms">timestamp_ms</a>: u64, ctx: &TxContext) {
    // Validator will make a special system call with sender set <b>as</b> 0x0.
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../mys/clock.md#mys_clock_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../mys/clock.md#mys_clock">clock</a>.<a href="../mys/clock.md#mys_clock_timestamp_ms">timestamp_ms</a> = <a href="../mys/clock.md#mys_clock_timestamp_ms">timestamp_ms</a>
}
</code></pre>



</details>
