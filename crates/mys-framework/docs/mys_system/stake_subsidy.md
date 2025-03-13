---
title: Module `mys_system::stake_subsidy`
---



-  [Struct `StakeSubsidy`](#mys_system_stake_subsidy_StakeSubsidy)
-  [Constants](#@Constants_0)
-  [Function `create`](#mys_system_stake_subsidy_create)
-  [Function `advance_epoch`](#mys_system_stake_subsidy_advance_epoch)
-  [Function `current_epoch_subsidy_amount`](#mys_system_stake_subsidy_current_epoch_subsidy_amount)
-  [Function `get_distribution_counter`](#mys_system_stake_subsidy_get_distribution_counter)


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
</code></pre>



<a name="mys_system_stake_subsidy_StakeSubsidy"></a>

## Struct `StakeSubsidy`



<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 Balance of MYS set aside for stake subsidies that will be drawn down over time.
</dd>
<dt>
<code>distribution_counter: u64</code>
</dt>
<dd>
 Count of the number of times stake subsidies have been distributed.
</dd>
<dt>
<code>current_distribution_amount: u64</code>
</dt>
<dd>
 The amount of stake subsidy to be drawn down per distribution.
 This amount decays and decreases over time.
</dd>
<dt>
<code>stake_subsidy_period_length: u64</code>
</dt>
<dd>
 Number of distributions to occur before the distribution amount decays.
</dd>
<dt>
<code>stake_subsidy_decrease_rate: u16</code>
</dt>
<dd>
 The rate at which the distribution amount decays at the end of each
 period. Expressed in basis points.
</dd>
<dt>
<code>extra_fields: <a href="../mys/bag.md#mys_bag_Bag">mys::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_system_stake_subsidy_BASIS_POINT_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>: u128 = 10000;
</code></pre>



<a name="mys_system_stake_subsidy_ESubsidyDecreaseRateTooLarge"></a>



<pre><code><b>const</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>: u64 = 0;
</code></pre>



<a name="mys_system_stake_subsidy_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_create">create</a>(balance: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, initial_distribution_amount: u64, stake_subsidy_period_length: u64, stake_subsidy_decrease_rate: u16, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">mys_system::stake_subsidy::StakeSubsidy</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_create">create</a>(
    balance: Balance&lt;MYS&gt;,
    initial_distribution_amount: u64,
    stake_subsidy_period_length: u64,
    stake_subsidy_decrease_rate: u16,
    ctx: &<b>mut</b> TxContext,
): <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a> {
    // Rate can't be higher than 100%.
    <b>assert</b>!(
        stake_subsidy_decrease_rate &lt;= <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a> <b>as</b> u16,
        <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>,
    );
    <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a> {
        balance,
        distribution_counter: 0,
        current_distribution_amount: initial_distribution_amount,
        stake_subsidy_period_length,
        stake_subsidy_decrease_rate,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="mys_system_stake_subsidy_advance_epoch"></a>

## Function `advance_epoch`

Advance the epoch counter and draw down the subsidy for the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">mys_system::stake_subsidy::StakeSubsidy</a>): <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>): Balance&lt;MYS&gt; {
    // Take the minimum of the reward amount and the remaining balance in
    // order to ensure we don't overdraft the remaining stake subsidy
    // balance
    <b>let</b> to_withdraw = self.current_distribution_amount.min(self.balance.value());
    // Drawn down the subsidy <b>for</b> this epoch.
    <b>let</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy">stake_subsidy</a> = self.balance.split(to_withdraw);
    self.distribution_counter = self.distribution_counter + 1;
    // Decrease the subsidy amount only when the current period ends.
    <b>if</b> (self.distribution_counter % self.stake_subsidy_period_length == 0) {
        <b>let</b> decrease_amount = self.current_distribution_amount <b>as</b> u128
            * (self.stake_subsidy_decrease_rate <b>as</b> u128) / <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
        self.current_distribution_amount = self.current_distribution_amount - (decrease_amount <b>as</b> u64)
    };
    <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy">stake_subsidy</a>
}
</code></pre>



</details>

<a name="mys_system_stake_subsidy_current_epoch_subsidy_amount"></a>

## Function `current_epoch_subsidy_amount`

Returns the amount of stake subsidy to be added at the end of the current epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_current_epoch_subsidy_amount">current_epoch_subsidy_amount</a>(self: &<a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">mys_system::stake_subsidy::StakeSubsidy</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_current_epoch_subsidy_amount">current_epoch_subsidy_amount</a>(self: &<a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>): u64 {
    self.current_distribution_amount.min(self.balance.value())
}
</code></pre>



</details>

<a name="mys_system_stake_subsidy_get_distribution_counter"></a>

## Function `get_distribution_counter`

Returns the number of distributions that have occurred.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_get_distribution_counter">get_distribution_counter</a>(self: &<a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">mys_system::stake_subsidy::StakeSubsidy</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_get_distribution_counter">get_distribution_counter</a>(self: &<a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>): u64 {
    self.distribution_counter
}
</code></pre>



</details>
