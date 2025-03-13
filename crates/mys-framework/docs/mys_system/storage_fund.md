---
title: Module `mys_system::storage_fund`
---



-  [Struct `StorageFund`](#mys_system_storage_fund_StorageFund)
-  [Function `new`](#mys_system_storage_fund_new)
-  [Function `advance_epoch`](#mys_system_storage_fund_advance_epoch)
-  [Function `total_object_storage_rebates`](#mys_system_storage_fund_total_object_storage_rebates)
-  [Function `total_balance`](#mys_system_storage_fund_total_balance)


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
<b>use</b> <a href="../mys/mys.md#mys_mys">mys::mys</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
</code></pre>



<a name="mys_system_storage_fund_StorageFund"></a>

## Struct `StorageFund`

Struct representing the storage fund, containing two <code>Balance</code>s:
- <code><a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a></code> has the invariant that it's the sum of <code>storage_rebate</code> of
all objects currently stored on-chain. To maintain this invariant, the only inflow of this
balance is storage charges collected from transactions, and the only outflow is storage rebates
of transactions, including both the portion refunded to the transaction senders as well as
the non-refundable portion taken out and put into <code>non_refundable_balance</code>.
- <code>non_refundable_balance</code> contains any remaining inflow of the storage fund that should not
be taken out of the fund.


<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_StorageFund">StorageFund</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>non_refundable_balance: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_system_storage_fund_new"></a>

## Function `new`

Called by <code><a href="../mys_system/mys_system.md#mys_system_mys_system">mys_system</a></code> at genesis time.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_new">new</a>(initial_fund: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;): <a href="../mys_system/storage_fund.md#mys_system_storage_fund_StorageFund">mys_system::storage_fund::StorageFund</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_new">new</a>(initial_fund: Balance&lt;MYS&gt;) : <a href="../mys_system/storage_fund.md#mys_system_storage_fund_StorageFund">StorageFund</a> {
    <a href="../mys_system/storage_fund.md#mys_system_storage_fund_StorageFund">StorageFund</a> {
        // At the beginning there's no object in the storage yet
        <a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>: balance::zero(),
        non_refundable_balance: initial_fund,
    }
}
</code></pre>



</details>

<a name="mys_system_storage_fund_advance_epoch"></a>

## Function `advance_epoch`

Called by <code><a href="../mys_system/mys_system.md#mys_system_mys_system">mys_system</a></code> at epoch change times to process the inflows and outflows of storage fund.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_StorageFund">mys_system::storage_fund::StorageFund</a>, storage_charges: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, storage_fund_reinvestment: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, leftover_staking_rewards: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, storage_rebate_amount: u64, non_refundable_storage_fee_amount: u64): <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_StorageFund">StorageFund</a>,
    storage_charges: Balance&lt;MYS&gt;,
    storage_fund_reinvestment: Balance&lt;MYS&gt;,
    leftover_staking_rewards: Balance&lt;MYS&gt;,
    storage_rebate_amount: u64,
    non_refundable_storage_fee_amount: u64,
) : Balance&lt;MYS&gt; {
    // Both the reinvestment and leftover rewards are not to be refunded so they go to the non-refundable balance.
    self.non_refundable_balance.join(storage_fund_reinvestment);
    self.non_refundable_balance.join(leftover_staking_rewards);
    // The storage charges <b>for</b> the epoch come from the storage rebate of the <a href="../mys_system/storage_fund.md#mys_system_storage_fund_new">new</a> objects created
    // and the <a href="../mys_system/storage_fund.md#mys_system_storage_fund_new">new</a> storage rebates of the objects modified during the epoch so we put the charges
    // into `<a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>`.
    self.<a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.join(storage_charges);
    // Split out the non-refundable portion of the storage rebate and put it into the non-refundable balance.
    <b>let</b> non_refundable_storage_fee = self.<a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.split(non_refundable_storage_fee_amount);
    self.non_refundable_balance.join(non_refundable_storage_fee);
    // `storage_rebates` include the already refunded rebates of deleted objects and old rebates of modified objects and
    // should be taken out of the `<a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>`.
    <b>let</b> storage_rebate = self.<a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.split(storage_rebate_amount);
    // The storage rebate <b>has</b> already been returned to individual transaction senders' gas coins
    // so we <b>return</b> the balance to be burnt at the very end of epoch change.
    storage_rebate
}
</code></pre>



</details>

<a name="mys_system_storage_fund_total_object_storage_rebates"></a>

## Function `total_object_storage_rebates`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>(self: &<a href="../mys_system/storage_fund.md#mys_system_storage_fund_StorageFund">mys_system::storage_fund::StorageFund</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>(self: &<a href="../mys_system/storage_fund.md#mys_system_storage_fund_StorageFund">StorageFund</a>): u64 {
    self.<a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.value()
}
</code></pre>



</details>

<a name="mys_system_storage_fund_total_balance"></a>

## Function `total_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_balance">total_balance</a>(self: &<a href="../mys_system/storage_fund.md#mys_system_storage_fund_StorageFund">mys_system::storage_fund::StorageFund</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_balance">total_balance</a>(self: &<a href="../mys_system/storage_fund.md#mys_system_storage_fund_StorageFund">StorageFund</a>): u64 {
    self.<a href="../mys_system/storage_fund.md#mys_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.value() + self.non_refundable_balance.value()
}
</code></pre>



</details>
