---
title: Module `myusd::myusd`
---



-  [Struct `MYUSD`](#myusd_myusd_MYUSD)
-  [Constants](#@Constants_0)
-  [Function `init`](#myusd_myusd_init)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
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
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="myusd_myusd_MYUSD"></a>

## Struct `MYUSD`



<pre><code><b>public</b> <b>struct</b> <a href="../myusd/myusd.md#myusd_myusd_MYUSD">MYUSD</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myusd_myusd_DECIMAL"></a>



<pre><code><b>const</b> <a href="../myusd/myusd.md#myusd_myusd_DECIMAL">DECIMAL</a>: u8 = 9;
</code></pre>



<a name="myusd_myusd_init"></a>

## Function `init`



<pre><code><b>fun</b> <a href="../myusd/myusd.md#myusd_myusd_init">init</a>(otw: <a href="../myusd/myusd.md#myusd_myusd_MYUSD">myusd::myusd::MYUSD</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myusd/myusd.md#myusd_myusd_init">init</a>(otw: <a href="../myusd/myusd.md#myusd_myusd_MYUSD">MYUSD</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>) {
    <b>let</b> (treasury_cap, metadata) = coin::create_currency(
        otw,
        <a href="../myusd/myusd.md#myusd_myusd_DECIMAL">DECIMAL</a>,
        b"myUSD",
        b"MyUSD",
        b"The official MySocial USD stablecoin.",
        <a href="../std/option.md#std_option_none">std::option::none</a>(),
        ctx
    );
    <a href="../mys/transfer.md#mys_transfer_public_freeze_object">mys::transfer::public_freeze_object</a>(metadata);
    <a href="../mys/transfer.md#mys_transfer_public_transfer">mys::transfer::public_transfer</a>(treasury_cap, <a href="../mys/tx_context.md#mys_tx_context_sender">mys::tx_context::sender</a>(ctx));
}
</code></pre>



</details>
