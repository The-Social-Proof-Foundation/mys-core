---
title: Module `mys::mys`
---

Coin<MYS> is the token used to pay for gas in MySocial.
It has 9 decimals, and the smallest unit (10^-9) is called "mist".


-  [Struct `MYS`](#mys_mys_MYS)
-  [Constants](#@Constants_0)
-  [Function `new`](#mys_mys_new)
-  [Function `transfer`](#mys_mys_transfer)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
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
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
</code></pre>



<a name="mys_mys_MYS"></a>

## Struct `MYS`

Name of the coin


<pre><code><b>public</b> <b>struct</b> <a href="../mys/mys.md#mys_mys_MYS">MYS</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_mys_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="../mys/mys.md#mys_mys_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="mys_mys_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../mys/mys.md#mys_mys_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="mys_mys_MIST_PER_MYS"></a>

The amount of Mist per MySocial token based on the fact that mist is
10^-9 of a MySocial token


<pre><code><b>const</b> <a href="../mys/mys.md#mys_mys_MIST_PER_MYS">MIST_PER_MYS</a>: u64 = 1000000000;
</code></pre>



<a name="mys_mys_TOTAL_SUPPLY_MIST"></a>

The total supply of MySocial denominated in Mist (10 Billion * 10^9)


<pre><code><b>const</b> <a href="../mys/mys.md#mys_mys_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>: u64 = 10000000000000000000;
</code></pre>



<a name="mys_mys_TOTAL_SUPPLY_MYS"></a>

The total supply of MySocial denominated in whole MySocial tokens (10 Billion)


<pre><code><b>const</b> <a href="../mys/mys.md#mys_mys_TOTAL_SUPPLY_MYS">TOTAL_SUPPLY_MYS</a>: u64 = 10000000000;
</code></pre>



<a name="mys_mys_new"></a>

## Function `new`

Register the <code><a href="../mys/mys.md#mys_mys_MYS">MYS</a></code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="../mys/mys.md#mys_mys_new">new</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys/mys.md#mys_mys_new">new</a>(ctx: &<b>mut</b> TxContext): Balance&lt;<a href="../mys/mys.md#mys_mys_MYS">MYS</a>&gt; {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../mys/mys.md#mys_mys_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(ctx.epoch() == 0, <a href="../mys/mys.md#mys_mys_EAlreadyMinted">EAlreadyMinted</a>);
    <b>let</b> (treasury, metadata) = <a href="../mys/coin.md#mys_coin_create_currency">coin::create_currency</a>(
        <a href="../mys/mys.md#mys_mys_MYS">MYS</a> {},
        9,
        b"<a href="../mys/mys.md#mys_mys_MYS">MYS</a>",
        b"MySocial",
        // TODO: add appropriate description and logo <a href="../mys/url.md#mys_url">url</a>
        b"",
        option::none(),
        ctx,
    );
    <a href="../mys/transfer.md#mys_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> <b>mut</b> supply = treasury.treasury_into_supply();
    <b>let</b> total_mys = supply.increase_supply(<a href="../mys/mys.md#mys_mys_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>);
    supply.destroy_supply();
    total_mys
}
</code></pre>



</details>

<a name="mys_mys_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys/transfer.md#mys_transfer">transfer</a>(c: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys/transfer.md#mys_transfer">transfer</a>(c: <a href="../mys/coin.md#mys_coin_Coin">coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">MYS</a>&gt;, recipient: <b>address</b>) {
    <a href="../mys/transfer.md#mys_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>
