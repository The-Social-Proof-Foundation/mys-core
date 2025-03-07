---
title: Module `mys_system::staking_pool`
---



-  [Struct `StakingPool`](#mys_system_staking_pool_StakingPool)
-  [Struct `PoolTokenExchangeRate`](#mys_system_staking_pool_PoolTokenExchangeRate)
-  [Struct `StakedMys`](#mys_system_staking_pool_StakedMys)
-  [Struct `FungibleStakedMys`](#mys_system_staking_pool_FungibleStakedMys)
-  [Struct `FungibleStakedMysData`](#mys_system_staking_pool_FungibleStakedMysData)
-  [Struct `FungibleStakedMysDataKey`](#mys_system_staking_pool_FungibleStakedMysDataKey)
-  [Constants](#@Constants_0)
-  [Function `new`](#mys_system_staking_pool_new)
-  [Function `request_add_stake`](#mys_system_staking_pool_request_add_stake)
-  [Function `request_withdraw_stake`](#mys_system_staking_pool_request_withdraw_stake)
-  [Function `redeem_fungible_staked_mys`](#mys_system_staking_pool_redeem_fungible_staked_mys)
-  [Function `calculate_fungible_staked_mys_withdraw_amount`](#mys_system_staking_pool_calculate_fungible_staked_mys_withdraw_amount)
-  [Function `convert_to_fungible_staked_mys`](#mys_system_staking_pool_convert_to_fungible_staked_mys)
-  [Function `withdraw_from_principal`](#mys_system_staking_pool_withdraw_from_principal)
-  [Function `unwrap_staked_mys`](#mys_system_staking_pool_unwrap_staked_mys)
-  [Function `deposit_rewards`](#mys_system_staking_pool_deposit_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#mys_system_staking_pool_process_pending_stakes_and_withdraws)
-  [Function `process_pending_stake_withdraw`](#mys_system_staking_pool_process_pending_stake_withdraw)
-  [Function `process_pending_stake`](#mys_system_staking_pool_process_pending_stake)
-  [Function `withdraw_rewards`](#mys_system_staking_pool_withdraw_rewards)
-  [Function `activate_staking_pool`](#mys_system_staking_pool_activate_staking_pool)
-  [Function `deactivate_staking_pool`](#mys_system_staking_pool_deactivate_staking_pool)
-  [Function `mys_balance`](#mys_system_staking_pool_mys_balance)
-  [Function `pool_id`](#mys_system_staking_pool_pool_id)
-  [Function `fungible_staked_mys_pool_id`](#mys_system_staking_pool_fungible_staked_mys_pool_id)
-  [Function `staked_mys_amount`](#mys_system_staking_pool_staked_mys_amount)
-  [Function `stake_activation_epoch`](#mys_system_staking_pool_stake_activation_epoch)
-  [Function `is_preactive`](#mys_system_staking_pool_is_preactive)
-  [Function `is_inactive`](#mys_system_staking_pool_is_inactive)
-  [Function `fungible_staked_mys_value`](#mys_system_staking_pool_fungible_staked_mys_value)
-  [Function `split_fungible_staked_mys`](#mys_system_staking_pool_split_fungible_staked_mys)
-  [Function `join_fungible_staked_mys`](#mys_system_staking_pool_join_fungible_staked_mys)
-  [Function `split`](#mys_system_staking_pool_split)
-  [Function `split_staked_mys`](#mys_system_staking_pool_split_staked_mys)
-  [Function `join_staked_mys`](#mys_system_staking_pool_join_staked_mys)
-  [Function `is_equal_staking_metadata`](#mys_system_staking_pool_is_equal_staking_metadata)
-  [Function `pool_token_exchange_rate_at_epoch`](#mys_system_staking_pool_pool_token_exchange_rate_at_epoch)
-  [Function `pending_stake_amount`](#mys_system_staking_pool_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#mys_system_staking_pool_pending_stake_withdraw_amount)
-  [Function `exchange_rates`](#mys_system_staking_pool_exchange_rates)
-  [Function `mys_amount`](#mys_system_staking_pool_mys_amount)
-  [Function `pool_token_amount`](#mys_system_staking_pool_pool_token_amount)
-  [Function `is_preactive_at_epoch`](#mys_system_staking_pool_is_preactive_at_epoch)
-  [Function `get_mys_amount`](#mys_system_staking_pool_get_mys_amount)
-  [Function `get_token_amount`](#mys_system_staking_pool_get_token_amount)
-  [Function `initial_exchange_rate`](#mys_system_staking_pool_initial_exchange_rate)
-  [Function `check_balance_invariants`](#mys_system_staking_pool_check_balance_invariants)


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



<a name="mys_system_staking_pool_StakingPool"></a>

## Struct `StakingPool`

A staking pool embedded in each validator struct in the system state object.


<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a> <b>has</b> key, store
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
<code>activation_epoch: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this pool became active.
 The value is <code>None</code> if the pool is pre-active and <code>Some(&lt;epoch_number&gt;)</code> if active or inactive.
</dd>
<dt>
<code>deactivation_epoch: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this staking pool ceased to be active. <code>None</code> = {pre-active, active},
 <code>Some(&lt;epoch_number&gt;)</code> if in-active, and it was de-activated at epoch <code>&lt;epoch_number&gt;</code>.
</dd>
<dt>
<code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a>: u64</code>
</dt>
<dd>
 The total number of MYS tokens in this pool, including the MYS in the rewards_pool, as well as in all the principal
 in the <code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a></code> object, updated at epoch boundaries.
</dd>
<dt>
<code>rewards_pool: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 The epoch stake rewards will be added here at the end of each epoch.
</dd>
<dt>
<code>pool_token_balance: u64</code>
</dt>
<dd>
 Total number of pool tokens issued by the pool.
</dd>
<dt>
<code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_exchange_rates">exchange_rates</a>: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u64, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">mys_system::staking_pool::PoolTokenExchangeRate</a>&gt;</code>
</dt>
<dd>
 Exchange rate history of previous epochs. Key is the epoch number.
 The entries start from the <code>activation_epoch</code> of this pool and contains exchange rates at the beginning of each epoch,
 i.e., right after the rewards for the previous epoch have been deposited into the pool.
</dd>
<dt>
<code>pending_stake: u64</code>
</dt>
<dd>
 Pending stake amount for this epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>pending_total_mys_withdraw: u64</code>
</dt>
<dd>
 Pending stake withdrawn during the current epoch, emptied at epoch boundaries.
 This includes both the principal and rewards MYS withdrawn.
</dd>
<dt>
<code>pending_pool_token_withdraw: u64</code>
</dt>
<dd>
 Pending pool token withdrawn during the current epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>extra_fields: <a href="../mys/bag.md#mys_bag_Bag">mys::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="mys_system_staking_pool_PoolTokenExchangeRate"></a>

## Struct `PoolTokenExchangeRate`

Struct representing the exchange rate of the stake pool token to MYS.


<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_system_staking_pool_StakedMys"></a>

## Struct `StakedMys`

A self-custodial object holding the staked MYS tokens.


<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a> <b>has</b> key, store
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
<code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
 ID of the staking pool we are staking with.
</dd>
<dt>
<code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: u64</code>
</dt>
<dd>
 The epoch at which the stake becomes active.
</dd>
<dt>
<code>principal: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 The staked MYS tokens.
</dd>
</dl>


</details>

<a name="mys_system_staking_pool_FungibleStakedMys"></a>

## Struct `FungibleStakedMys`

An alternative to <code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a></code> that holds the pool token amount instead of the MYS balance.
StakedMys objects can be converted to FungibleStakedMyss after the initial warmup period.
The advantage of this is that you can now merge multiple StakedMys objects from different
activation epochs into a single FungibleStakedMys object.


<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a> <b>has</b> key, store
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
<code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
 ID of the staking pool we are staking with.
</dd>
<dt>
<code>value: u64</code>
</dt>
<dd>
 The pool token amount.
</dd>
</dl>


</details>

<a name="mys_system_staking_pool_FungibleStakedMysData"></a>

## Struct `FungibleStakedMysData`

Holds useful information


<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMysData">FungibleStakedMysData</a> <b>has</b> key, store
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
<code>total_supply: u64</code>
</dt>
<dd>
 fungible_staked_mys supply
</dd>
<dt>
<code>principal: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 principal balance. Rewards are withdrawn from the reward pool
</dd>
</dl>


</details>

<a name="mys_system_staking_pool_FungibleStakedMysDataKey"></a>

## Struct `FungibleStakedMysDataKey`



<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMysDataKey">FungibleStakedMysDataKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_system_staking_pool_EActivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>: u64 = 16;
</code></pre>



<a name="mys_system_staking_pool_ECannotMintFungibleStakedMysYet"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_ECannotMintFungibleStakedMysYet">ECannotMintFungibleStakedMysYet</a>: u64 = 19;
</code></pre>



<a name="mys_system_staking_pool_EDeactivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>: u64 = 11;
</code></pre>



<a name="mys_system_staking_pool_EDelegationOfZeroMys"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EDelegationOfZeroMys">EDelegationOfZeroMys</a>: u64 = 17;
</code></pre>



<a name="mys_system_staking_pool_EDelegationToInactivePool"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>: u64 = 10;
</code></pre>



<a name="mys_system_staking_pool_EDestroyNonzeroBalance"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EDestroyNonzeroBalance">EDestroyNonzeroBalance</a>: u64 = 5;
</code></pre>



<a name="mys_system_staking_pool_EIncompatibleStakedMys"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EIncompatibleStakedMys">EIncompatibleStakedMys</a>: u64 = 12;
</code></pre>



<a name="mys_system_staking_pool_EInsufficientPoolTokenBalance"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>: u64 = 0;
</code></pre>



<a name="mys_system_staking_pool_EInsufficientRewardsPoolBalance"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EInsufficientRewardsPoolBalance">EInsufficientRewardsPoolBalance</a>: u64 = 4;
</code></pre>



<a name="mys_system_staking_pool_EInsufficientMysTokenBalance"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EInsufficientMysTokenBalance">EInsufficientMysTokenBalance</a>: u64 = 3;
</code></pre>



<a name="mys_system_staking_pool_EInvariantFailure"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EInvariantFailure">EInvariantFailure</a>: u64 = 20;
</code></pre>



<a name="mys_system_staking_pool_EPendingDelegationDoesNotExist"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EPendingDelegationDoesNotExist">EPendingDelegationDoesNotExist</a>: u64 = 8;
</code></pre>



<a name="mys_system_staking_pool_EPoolAlreadyActive"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>: u64 = 14;
</code></pre>



<a name="mys_system_staking_pool_EPoolNotPreactive"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EPoolNotPreactive">EPoolNotPreactive</a>: u64 = 15;
</code></pre>



<a name="mys_system_staking_pool_EStakedMysBelowThreshold"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EStakedMysBelowThreshold">EStakedMysBelowThreshold</a>: u64 = 18;
</code></pre>



<a name="mys_system_staking_pool_ETokenBalancesDoNotMatchExchangeRate"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>: u64 = 9;
</code></pre>



<a name="mys_system_staking_pool_ETokenTimeLockIsSome"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_ETokenTimeLockIsSome">ETokenTimeLockIsSome</a>: u64 = 6;
</code></pre>



<a name="mys_system_staking_pool_EWithdrawAmountCannotBeZero"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EWithdrawAmountCannotBeZero">EWithdrawAmountCannotBeZero</a>: u64 = 2;
</code></pre>



<a name="mys_system_staking_pool_EWithdrawalInSameEpoch"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EWithdrawalInSameEpoch">EWithdrawalInSameEpoch</a>: u64 = 13;
</code></pre>



<a name="mys_system_staking_pool_EWrongDelegation"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EWrongDelegation">EWrongDelegation</a>: u64 = 7;
</code></pre>



<a name="mys_system_staking_pool_EWrongPool"></a>



<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EWrongPool">EWrongPool</a>: u64 = 1;
</code></pre>



<a name="mys_system_staking_pool_MIN_STAKING_THRESHOLD"></a>

StakedMys objects cannot be split to below this amount.


<pre><code><b>const</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="mys_system_staking_pool_new"></a>

## Function `new`

Create a new, empty staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_new">new</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_new">new</a>(ctx: &<b>mut</b> TxContext) : <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a> {
    <b>let</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_exchange_rates">exchange_rates</a> = table::new(ctx);
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a> {
        id: object::new(ctx),
        activation_epoch: option::none(),
        deactivation_epoch: option::none(),
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a>: 0,
        rewards_pool: balance::zero(),
        pool_token_balance: 0,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_exchange_rates">exchange_rates</a>,
        pending_stake: 0,
        pending_total_mys_withdraw: 0,
        pending_pool_token_withdraw: 0,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="mys_system_staking_pool_request_add_stake"></a>

## Function `request_add_stake`

Request to stake to a staking pool. The stake starts counting at the beginning of the next epoch,


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_request_add_stake">request_add_stake</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, stake: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_request_add_stake">request_add_stake</a>(
    pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>,
    stake: Balance&lt;MYS&gt;,
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: u64,
    ctx: &<b>mut</b> TxContext
) : <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a> {
    <b>let</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a> = stake.value();
    <b>assert</b>!(!<a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_inactive">is_inactive</a>(pool), <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>);
    <b>assert</b>!(<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a> &gt; 0, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EDelegationOfZeroMys">EDelegationOfZeroMys</a>);
    <b>let</b> staked_mys = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a> {
        id: object::new(ctx),
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>: object::id(pool),
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>,
        principal: stake,
    };
    pool.pending_stake = pool.pending_stake + <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>;
    staked_mys
}
</code></pre>



</details>

<a name="mys_system_staking_pool_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw the given stake plus rewards from a staking pool.
Both the principal and corresponding rewards in MYS are withdrawn.
A proportional amount of pool token withdraw is recorded and processed at epoch change time.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(
    pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>,
    staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>,
    ctx: &TxContext
) : Balance&lt;MYS&gt; {
    // stake is inactive
    <b>if</b> (staked_mys.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a> &gt; ctx.epoch()) {
        <b>let</b> principal = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_unwrap_staked_mys">unwrap_staked_mys</a>(staked_mys);
        pool.pending_stake = pool.pending_stake - principal.value();
        <b>return</b> principal
    };
    <b>let</b> (pool_token_withdraw_amount, <b>mut</b> principal_withdraw) =
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(pool, staked_mys);
    <b>let</b> principal_withdraw_amount = principal_withdraw.value();
    <b>let</b> rewards_withdraw = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_withdraw_rewards">withdraw_rewards</a>(
        pool, principal_withdraw_amount, pool_token_withdraw_amount, ctx.epoch()
    );
    <b>let</b> total_mys_withdraw_amount = principal_withdraw_amount + rewards_withdraw.value();
    pool.pending_total_mys_withdraw = pool.pending_total_mys_withdraw + total_mys_withdraw_amount;
    pool.pending_pool_token_withdraw = pool.pending_pool_token_withdraw + pool_token_withdraw_amount;
    // If the pool is inactive, we immediately process the withdrawal.
    <b>if</b> (<a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_inactive">is_inactive</a>(pool)) <a href="../mys_system/staking_pool.md#mys_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool);
    // TODO: implement withdraw bonding period here.
    principal_withdraw.join(rewards_withdraw);
    principal_withdraw
}
</code></pre>



</details>

<a name="mys_system_staking_pool_redeem_fungible_staked_mys"></a>

## Function `redeem_fungible_staked_mys`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_redeem_fungible_staked_mys">redeem_fungible_staked_mys</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, fungible_staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">mys_system::staking_pool::FungibleStakedMys</a>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_redeem_fungible_staked_mys">redeem_fungible_staked_mys</a>(
    pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>,
    fungible_staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a>,
    ctx: &TxContext
) : Balance&lt;MYS&gt; {
    <b>let</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a> { id, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>, value } = fungible_staked_mys;
    <b>assert</b>!(<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a> == object::id(pool), <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EWrongPool">EWrongPool</a>);
    object::delete(id);
    <b>let</b> latest_exchange_rate = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, tx_context::epoch(ctx));
    <b>let</b> fungible_staked_mys_data: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMysData">FungibleStakedMysData</a> = bag::borrow_mut(
        &<b>mut</b> pool.extra_fields,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMysDataKey">FungibleStakedMysDataKey</a> {}
    );
    <b>let</b> (principal_amount, rewards_amount) = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_calculate_fungible_staked_mys_withdraw_amount">calculate_fungible_staked_mys_withdraw_amount</a>(
        latest_exchange_rate,
        value,
        balance::value(&fungible_staked_mys_data.principal),
        fungible_staked_mys_data.total_supply
    );
    fungible_staked_mys_data.total_supply = fungible_staked_mys_data.total_supply - value;
    <b>let</b> <b>mut</b> mys_out = balance::split(&<b>mut</b> fungible_staked_mys_data.principal, principal_amount);
    balance::join(
        &<b>mut</b> mys_out,
        balance::split(&<b>mut</b> pool.rewards_pool, rewards_amount)
    );
    pool.pending_total_mys_withdraw = pool.pending_total_mys_withdraw + balance::value(&mys_out);
    pool.pending_pool_token_withdraw = pool.pending_pool_token_withdraw + value;
    mys_out
}
</code></pre>



</details>

<a name="mys_system_staking_pool_calculate_fungible_staked_mys_withdraw_amount"></a>

## Function `calculate_fungible_staked_mys_withdraw_amount`

written in separate function so i can test with random values
returns (principal_withdraw_amount, rewards_withdraw_amount)


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_calculate_fungible_staked_mys_withdraw_amount">calculate_fungible_staked_mys_withdraw_amount</a>(latest_exchange_rate: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">mys_system::staking_pool::PoolTokenExchangeRate</a>, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_fungible_staked_mys_value">fungible_staked_mys_value</a>: u64, fungible_staked_mys_data_principal_amount: u64, fungible_staked_mys_data_total_supply: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_calculate_fungible_staked_mys_withdraw_amount">calculate_fungible_staked_mys_withdraw_amount</a>(
    latest_exchange_rate: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>,
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_fungible_staked_mys_value">fungible_staked_mys_value</a>: u64,
    fungible_staked_mys_data_principal_amount: u64, // fungible_staked_mys_data.principal.value()
    fungible_staked_mys_data_total_supply: u64, // fungible_staked_mys_data.total_supply
) : (u64, u64) {
    // 1. <b>if</b> the entire <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMysData">FungibleStakedMysData</a> supply is redeemed, how much mys should we receive?
    <b>let</b> total_mys_amount = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_mys_amount">get_mys_amount</a>(&latest_exchange_rate, fungible_staked_mys_data_total_supply);
    // min with total_mys_amount to prevent underflow
    <b>let</b> fungible_staked_mys_data_principal_amount = <a href="../std/u64.md#std_u64_min">std::u64::min</a>(
        fungible_staked_mys_data_principal_amount,
        total_mys_amount
    );
    // 2. how much do we need to withdraw from the rewards pool?
    <b>let</b> total_rewards = total_mys_amount - fungible_staked_mys_data_principal_amount;
    // 3. proportionally withdraw from both wrt the <a href="../mys_system/staking_pool.md#mys_system_staking_pool_fungible_staked_mys_value">fungible_staked_mys_value</a>.
    <b>let</b> principal_withdraw_amount = ((<a href="../mys_system/staking_pool.md#mys_system_staking_pool_fungible_staked_mys_value">fungible_staked_mys_value</a> <b>as</b> u128)
        * (fungible_staked_mys_data_principal_amount <b>as</b> u128)
        / (fungible_staked_mys_data_total_supply <b>as</b> u128)) <b>as</b> u64;
    <b>let</b> rewards_withdraw_amount = ((<a href="../mys_system/staking_pool.md#mys_system_staking_pool_fungible_staked_mys_value">fungible_staked_mys_value</a> <b>as</b> u128)
        * (total_rewards <b>as</b> u128)
        / (fungible_staked_mys_data_total_supply <b>as</b> u128)) <b>as</b> u64;
    // <b>invariant</b> check, just in case
    <b>let</b> expected_mys_amount = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_mys_amount">get_mys_amount</a>(&latest_exchange_rate, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_fungible_staked_mys_value">fungible_staked_mys_value</a>);
    <b>assert</b>!(principal_withdraw_amount + rewards_withdraw_amount &lt;= expected_mys_amount, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EInvariantFailure">EInvariantFailure</a>);
    (principal_withdraw_amount, rewards_withdraw_amount)
}
</code></pre>



</details>

<a name="mys_system_staking_pool_convert_to_fungible_staked_mys"></a>

## Function `convert_to_fungible_staked_mys`

Convert the given staked MYS to an FungibleStakedMys object


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_convert_to_fungible_staked_mys">convert_to_fungible_staked_mys</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">mys_system::staking_pool::FungibleStakedMys</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_convert_to_fungible_staked_mys">convert_to_fungible_staked_mys</a>(
    pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>,
    staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>,
    ctx: &<b>mut</b> TxContext
) : <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a> {
    <b>let</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a> { id, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>, principal } = staked_mys;
    <b>assert</b>!(<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a> == object::id(pool), <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EWrongPool">EWrongPool</a>);
    <b>assert</b>!(
        tx_context::epoch(ctx) &gt;= <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_ECannotMintFungibleStakedMysYet">ECannotMintFungibleStakedMysYet</a>
    );
    object::delete(id);
    <b>let</b> exchange_rate_at_staking_epoch = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
        pool,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>
    );
    <b>let</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a> = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_token_amount">get_token_amount</a>(
        &exchange_rate_at_staking_epoch,
        balance::value(&principal)
    );
    <b>if</b> (!bag::contains(&pool.extra_fields, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMysDataKey">FungibleStakedMysDataKey</a> {})) {
        bag::add(
            &<b>mut</b> pool.extra_fields,
            <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMysDataKey">FungibleStakedMysDataKey</a> {},
            <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMysData">FungibleStakedMysData</a> {
                id: object::new(ctx),
                total_supply: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a>,
                principal
            }
        );
    }
    <b>else</b> {
        <b>let</b> fungible_staked_mys_data: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMysData">FungibleStakedMysData</a> = bag::borrow_mut(
            &<b>mut</b> pool.extra_fields,
            <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMysDataKey">FungibleStakedMysDataKey</a> {}
        );
        fungible_staked_mys_data.total_supply = fungible_staked_mys_data.total_supply + <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a>;
        balance::join(&<b>mut</b> fungible_staked_mys_data.principal, principal);
    };
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a> {
        id: object::new(ctx),
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>,
        value: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a>,
    }
}
</code></pre>



</details>

<a name="mys_system_staking_pool_withdraw_from_principal"></a>

## Function `withdraw_from_principal`

Withdraw the principal MYS stored in the StakedMys object, and calculate the corresponding amount of pool
tokens using exchange rate at staking epoch.
Returns values are amount of pool tokens withdrawn and withdrawn principal portion of MYS.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>): (u64, <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(
    pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>,
    staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>,
) : (u64, Balance&lt;MYS&gt;) {
    // Check that the stake information matches the pool.
    <b>assert</b>!(staked_mys.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a> == object::id(pool), <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EWrongPool">EWrongPool</a>);
    <b>let</b> exchange_rate_at_staking_epoch = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, staked_mys.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>);
    <b>let</b> principal_withdraw = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_unwrap_staked_mys">unwrap_staked_mys</a>(staked_mys);
    <b>let</b> pool_token_withdraw_amount = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_token_amount">get_token_amount</a>(
		&exchange_rate_at_staking_epoch,
		principal_withdraw.value()
	);
    (
        pool_token_withdraw_amount,
        principal_withdraw,
    )
}
</code></pre>



</details>

<a name="mys_system_staking_pool_unwrap_staked_mys"></a>

## Function `unwrap_staked_mys`



<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_unwrap_staked_mys">unwrap_staked_mys</a>(staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>): <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_unwrap_staked_mys">unwrap_staked_mys</a>(staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>): Balance&lt;MYS&gt; {
    <b>let</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a> {
        id,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>: _,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: _,
        principal,
    } = staked_mys;
    object::delete(id);
    principal
}
</code></pre>



</details>

<a name="mys_system_staking_pool_deposit_rewards"></a>

## Function `deposit_rewards`

Called at epoch advancement times to add rewards (in MYS) to the staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, rewards: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>, rewards: Balance&lt;MYS&gt;) {
    pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a> = pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a> + rewards.value();
    pool.rewards_pool.join(rewards);
}
</code></pre>



</details>

<a name="mys_system_staking_pool_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>, ctx: &TxContext) {
    <b>let</b> new_epoch = ctx.epoch() + 1;
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool);
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_process_pending_stake">process_pending_stake</a>(pool);
    pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_exchange_rates">exchange_rates</a>.add(
        new_epoch,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>: pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a>, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a>: pool.pool_token_balance },
    );
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool, new_epoch);
}
</code></pre>



</details>

<a name="mys_system_staking_pool_process_pending_stake_withdraw"></a>

## Function `process_pending_stake_withdraw`

Called at epoch boundaries to process pending stake withdraws requested during the epoch.
Also called immediately upon withdrawal if the pool is inactive.


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>) {
    pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a> = pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a> - pool.pending_total_mys_withdraw;
    pool.pool_token_balance = pool.pool_token_balance - pool.pending_pool_token_withdraw;
    pool.pending_total_mys_withdraw = 0;
    pool.pending_pool_token_withdraw = 0;
}
</code></pre>



</details>

<a name="mys_system_staking_pool_process_pending_stake"></a>

## Function `process_pending_stake`

Called at epoch boundaries to process the pending stake.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>) {
    // Use the most up to date exchange rate with the rewards deposited and withdraws effectuated.
    <b>let</b> latest_exchange_rate =
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>: pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a>, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a>: pool.pool_token_balance };
    pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a> = pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a> + pool.pending_stake;
    pool.pool_token_balance = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_token_amount">get_token_amount</a>(&latest_exchange_rate, pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a>);
    pool.pending_stake = 0;
}
</code></pre>



</details>

<a name="mys_system_staking_pool_withdraw_rewards"></a>

## Function `withdraw_rewards`

This function does the following:
1. Calculates the total amount of MYS (including principal and rewards) that the provided pool tokens represent
at the current exchange rate.
2. Using the above number and the given <code>principal_withdraw_amount</code>, calculates the rewards portion of the
stake we should withdraw.
3. Withdraws the rewards portion from the rewards pool at the current exchange rate. We only withdraw the rewards
portion because the principal portion was already taken out of the staker's self custodied StakedMys.


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_withdraw_rewards">withdraw_rewards</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, principal_withdraw_amount: u64, pool_token_withdraw_amount: u64, epoch: u64): <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_withdraw_rewards">withdraw_rewards</a>(
    pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>,
    principal_withdraw_amount: u64,
    pool_token_withdraw_amount: u64,
    epoch: u64,
) : Balance&lt;MYS&gt; {
    <b>let</b> exchange_rate = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, epoch);
    <b>let</b> total_mys_withdraw_amount = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_mys_amount">get_mys_amount</a>(&exchange_rate, pool_token_withdraw_amount);
    <b>let</b> <b>mut</b> reward_withdraw_amount =
        <b>if</b> (total_mys_withdraw_amount &gt;= principal_withdraw_amount)
            total_mys_withdraw_amount - principal_withdraw_amount
        <b>else</b> 0;
    // This may happen when we are withdrawing everything from the pool and
    // the rewards pool balance may be less than reward_withdraw_amount.
    // TODO: FIGURE OUT EXACTLY WHY THIS CAN HAPPEN.
    reward_withdraw_amount = reward_withdraw_amount.min(pool.rewards_pool.value());
    pool.rewards_pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_split">split</a>(reward_withdraw_amount)
}
</code></pre>



</details>

<a name="mys_system_staking_pool_activate_staking_pool"></a>

## Function `activate_staking_pool`

Called by <code><a href="../mys_system/validator.md#mys_system_validator">validator</a></code> module to activate a staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, activation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>, activation_epoch: u64) {
    // Add the initial exchange rate to the table.
    pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_exchange_rates">exchange_rates</a>.add(
        activation_epoch,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
    );
    // Check that the pool is preactive and not inactive.
    <b>assert</b>!(<a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_preactive">is_preactive</a>(pool), <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>);
    <b>assert</b>!(!<a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_inactive">is_inactive</a>(pool), <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>);
    // Fill in the active epoch.
    pool.activation_epoch.fill(activation_epoch);
}
</code></pre>



</details>

<a name="mys_system_staking_pool_deactivate_staking_pool"></a>

## Function `deactivate_staking_pool`

Deactivate a staking pool by setting the <code>deactivation_epoch</code>. After
this pool deactivation, the pool stops earning rewards. Only stake
withdraws can be made to the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>, deactivation_epoch: u64) {
    // We can't deactivate an already deactivated pool.
    <b>assert</b>!(!<a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_inactive">is_inactive</a>(pool), <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>);
    pool.deactivation_epoch = option::some(deactivation_epoch);
}
</code></pre>



</details>

<a name="mys_system_staking_pool_mys_balance"></a>

## Function `mys_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>): u64 { pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a> }
</code></pre>



</details>

<a name="mys_system_staking_pool_pool_id"></a>

## Function `pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>(staked_mys: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>): <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>(staked_mys: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>): ID { staked_mys.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a> }
</code></pre>



</details>

<a name="mys_system_staking_pool_fungible_staked_mys_pool_id"></a>

## Function `fungible_staked_mys_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_fungible_staked_mys_pool_id">fungible_staked_mys_pool_id</a>(fungible_staked_mys: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">mys_system::staking_pool::FungibleStakedMys</a>): <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_fungible_staked_mys_pool_id">fungible_staked_mys_pool_id</a>(fungible_staked_mys: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a>): ID { fungible_staked_mys.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a> }
</code></pre>



</details>

<a name="mys_system_staking_pool_staked_mys_amount"></a>

## Function `staked_mys_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_staked_mys_amount">staked_mys_amount</a>(staked_mys: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_staked_mys_amount">staked_mys_amount</a>(staked_mys: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>): u64 { staked_mys.principal.value() }
</code></pre>



</details>

<a name="mys_system_staking_pool_stake_activation_epoch"></a>

## Function `stake_activation_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>(staked_mys: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>(staked_mys: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>): u64 {
    staked_mys.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>
}
</code></pre>



</details>

<a name="mys_system_staking_pool_is_preactive"></a>

## Function `is_preactive`

Returns true if the input staking pool is preactive.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_preactive">is_preactive</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_preactive">is_preactive</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>): bool{
    pool.activation_epoch.is_none()
}
</code></pre>



</details>

<a name="mys_system_staking_pool_is_inactive"></a>

## Function `is_inactive`

Returns true if the input staking pool is inactive.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>): bool {
    pool.deactivation_epoch.is_some()
}
</code></pre>



</details>

<a name="mys_system_staking_pool_fungible_staked_mys_value"></a>

## Function `fungible_staked_mys_value`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_fungible_staked_mys_value">fungible_staked_mys_value</a>(fungible_staked_mys: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">mys_system::staking_pool::FungibleStakedMys</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_fungible_staked_mys_value">fungible_staked_mys_value</a>(fungible_staked_mys: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a>): u64 { fungible_staked_mys.value }
</code></pre>



</details>

<a name="mys_system_staking_pool_split_fungible_staked_mys"></a>

## Function `split_fungible_staked_mys`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_split_fungible_staked_mys">split_fungible_staked_mys</a>(fungible_staked_mys: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">mys_system::staking_pool::FungibleStakedMys</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">mys_system::staking_pool::FungibleStakedMys</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_split_fungible_staked_mys">split_fungible_staked_mys</a>(
    fungible_staked_mys: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a>,
    split_amount: u64,
    ctx: &<b>mut</b> TxContext
): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a> {
    <b>assert</b>!(split_amount &lt;= fungible_staked_mys.value, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>);
    fungible_staked_mys.value = fungible_staked_mys.value - split_amount;
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a> {
        id: object::new(ctx),
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>: fungible_staked_mys.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>,
        value: split_amount,
    }
}
</code></pre>



</details>

<a name="mys_system_staking_pool_join_fungible_staked_mys"></a>

## Function `join_fungible_staked_mys`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_join_fungible_staked_mys">join_fungible_staked_mys</a>(self: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">mys_system::staking_pool::FungibleStakedMys</a>, other: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">mys_system::staking_pool::FungibleStakedMys</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_join_fungible_staked_mys">join_fungible_staked_mys</a>(self: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a>, other: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a>) {
    <b>let</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">FungibleStakedMys</a> { id, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>, value } = other;
    <b>assert</b>!(self.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a> == <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EWrongPool">EWrongPool</a>);
    object::delete(id);
    self.value = self.value + value;
}
</code></pre>



</details>

<a name="mys_system_staking_pool_split"></a>

## Function `split`

Split StakedMys <code>self</code> to two parts, one with principal <code>split_amount</code>,
and the remaining principal is left in <code>self</code>.
All the other parameters of the StakedMys like <code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a></code> or <code><a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a></code> remain the same.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_split">split</a>(self: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_split">split</a>(self: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>, split_amount: u64, ctx: &<b>mut</b> TxContext): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a> {
    <b>let</b> original_amount = self.principal.value();
    <b>assert</b>!(split_amount &lt;= original_amount, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EInsufficientMysTokenBalance">EInsufficientMysTokenBalance</a>);
    <b>let</b> remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least <a href="../mys_system/staking_pool.md#mys_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(remaining_amount &gt;= <a href="../mys_system/staking_pool.md#mys_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EStakedMysBelowThreshold">EStakedMysBelowThreshold</a>);
    <b>assert</b>!(split_amount &gt;= <a href="../mys_system/staking_pool.md#mys_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EStakedMysBelowThreshold">EStakedMysBelowThreshold</a>);
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a> {
        id: object::new(ctx),
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>: self.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: self.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>,
        principal: self.principal.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_split">split</a>(split_amount),
    }
}
</code></pre>



</details>

<a name="mys_system_staking_pool_split_staked_mys"></a>

## Function `split_staked_mys`

Split the given StakedMys to the two parts, one with principal <code>split_amount</code>,
transfer the newly split part to the sender address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_split_staked_mys">split_staked_mys</a>(stake: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_split_staked_mys">split_staked_mys</a>(stake: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    transfer::transfer(<a href="../mys_system/staking_pool.md#mys_system_staking_pool_split">split</a>(stake, split_amount, ctx), ctx.sender());
}
</code></pre>



</details>

<a name="mys_system_staking_pool_join_staked_mys"></a>

## Function `join_staked_mys`

Consume the staked mys <code>other</code> and add its value to <code>self</code>.
Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_join_staked_mys">join_staked_mys</a>(self: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>, other: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_join_staked_mys">join_staked_mys</a>(self: &<b>mut</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>, other: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>) {
    <b>assert</b>!(<a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self, &other), <a href="../mys_system/staking_pool.md#mys_system_staking_pool_EIncompatibleStakedMys">EIncompatibleStakedMys</a>);
    <b>let</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a> {
        id,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>: _,
        <a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: _,
        principal,
    } = other;
    id.delete();
    self.principal.join(principal);
}
</code></pre>



</details>

<a name="mys_system_staking_pool_is_equal_staking_metadata"></a>

## Function `is_equal_staking_metadata`

Returns true if all the staking parameters of the staked mys except the principal are identical


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>, other: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>, other: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">StakedMys</a>): bool {
    (self.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a> == other.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_id">pool_id</a>) &&
    (self.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a> == other.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>)
}
</code></pre>



</details>

<a name="mys_system_staking_pool_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, epoch: u64): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">mys_system::staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>, epoch: u64): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    // If the pool is preactive then the exchange rate is always 1:1.
    <b>if</b> (<a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool, epoch)) {
        <b>return</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
    };
    <b>let</b> clamped_epoch = pool.deactivation_epoch.get_with_default(epoch);
    <b>let</b> <b>mut</b> epoch = clamped_epoch.min(epoch);
    <b>let</b> activation_epoch = *pool.activation_epoch.borrow();
    // Find the latest epoch that's earlier than the given epoch with an <b>entry</b> in the table
    <b>while</b> (epoch &gt;= activation_epoch) {
        <b>if</b> (pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_exchange_rates">exchange_rates</a>.contains(epoch)) {
            <b>return</b> pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_exchange_rates">exchange_rates</a>[epoch]
        };
        epoch = epoch - 1;
    };
    // This line really should be unreachable. Do we want an <b>assert</b> <b>false</b> here?
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
}
</code></pre>



</details>

<a name="mys_system_staking_pool_pending_stake_amount"></a>

## Function `pending_stake_amount`

Returns the total value of the pending staking requests for this staking pool.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="../mys_system/staking_pool.md#mys_system_staking_pool">staking_pool</a>: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="../mys_system/staking_pool.md#mys_system_staking_pool">staking_pool</a>: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>): u64 {
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool">staking_pool</a>.pending_stake
}
</code></pre>



</details>

<a name="mys_system_staking_pool_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`

Returns the total withdrawal from the staking pool this epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="../mys_system/staking_pool.md#mys_system_staking_pool">staking_pool</a>: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="../mys_system/staking_pool.md#mys_system_staking_pool">staking_pool</a>: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>): u64 {
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool">staking_pool</a>.pending_total_mys_withdraw
}
</code></pre>



</details>

<a name="mys_system_staking_pool_exchange_rates"></a>

## Function `exchange_rates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>): &<a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u64, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">mys_system::staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>): &Table&lt;u64, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>&gt; {
    &pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_exchange_rates">exchange_rates</a>
}
</code></pre>



</details>

<a name="mys_system_staking_pool_mys_amount"></a>

## Function `mys_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>(exchange_rate: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">mys_system::staking_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>(exchange_rate: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>
}
</code></pre>



</details>

<a name="mys_system_staking_pool_pool_token_amount"></a>

## Function `pool_token_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">mys_system::staking_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a>
}
</code></pre>



</details>

<a name="mys_system_staking_pool_is_preactive_at_epoch"></a>

## Function `is_preactive_at_epoch`

Returns true if the provided staking pool is preactive at the provided epoch.


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>, epoch: u64): bool{
    // Either the pool is currently preactive or the pool's starting epoch is later than the provided epoch.
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_is_preactive">is_preactive</a>(pool) || (*pool.activation_epoch.borrow() &gt; epoch)
}
</code></pre>



</details>

<a name="mys_system_staking_pool_get_mys_amount"></a>

## Function `get_mys_amount`



<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_mys_amount">get_mys_amount</a>(exchange_rate: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">mys_system::staking_pool::PoolTokenExchangeRate</a>, token_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_mys_amount">get_mys_amount</a>(exchange_rate: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, token_amount: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a> == 0 || exchange_rate.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a> == 0) {
        <b>return</b> token_amount
    };
    <b>let</b> res = exchange_rate.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a> <b>as</b> u128
            * (token_amount <b>as</b> u128)
            / (exchange_rate.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a> <b>as</b> u128);
    res <b>as</b> u64
}
</code></pre>



</details>

<a name="mys_system_staking_pool_get_token_amount"></a>

## Function `get_token_amount`



<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">mys_system::staking_pool::PoolTokenExchangeRate</a>, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a> == 0 || exchange_rate.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a> == 0) {
        <b>return</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>
    };
    <b>let</b> res = exchange_rate.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a> <b>as</b> u128
            * (<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a> <b>as</b> u128)
            / (exchange_rate.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a> <b>as</b> u128);
    res <b>as</b> u64
}
</code></pre>



</details>

<a name="mys_system_staking_pool_initial_exchange_rate"></a>

## Function `initial_exchange_rate`



<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">mys_system::staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { <a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_amount">mys_amount</a>: 0, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_amount">pool_token_amount</a>: 0 }
}
</code></pre>



</details>

<a name="mys_system_staking_pool_check_balance_invariants"></a>

## Function `check_balance_invariants`



<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">mys_system::staking_pool::StakingPool</a>, epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakingPool">StakingPool</a>, epoch: u64) {
    <b>let</b> exchange_rate = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, epoch);
    // check that the pool token balance and mys balance ratio matches the exchange rate stored.
    <b>let</b> expected = <a href="../mys_system/staking_pool.md#mys_system_staking_pool_get_token_amount">get_token_amount</a>(&exchange_rate, pool.<a href="../mys_system/staking_pool.md#mys_system_staking_pool_mys_balance">mys_balance</a>);
    <b>let</b> actual = pool.pool_token_balance;
    <b>assert</b>!(expected == actual, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>)
}
</code></pre>



</details>
