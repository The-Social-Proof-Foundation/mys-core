---
title: Module `mys_system::mys_system`
---

MySocial System State Type Upgrade Guide
<code><a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a></code> is a thin wrapper around <code>MysSystemStateInner</code> that provides a versioned interface.
The <code><a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a></code> object has a fixed ID 0x5, and the <code>MysSystemStateInner</code> object is stored as a dynamic field.
There are a few different ways to upgrade the <code>MysSystemStateInner</code> type:

The simplest and one that doesn't involve a real upgrade is to just add dynamic fields to the <code>extra_fields</code> field
of <code>MysSystemStateInner</code> or any of its sub type. This is useful when we are in a rush, or making a small change,
or still experimenting a new field.

To properly upgrade the <code>MysSystemStateInner</code> type, we need to ship a new framework that does the following:
1. Define a new <code>MysSystemStateInner</code>type (e.g. <code>MysSystemStateInnerV2</code>).
2. Define a data migration function that migrates the old <code>MysSystemStateInner</code> to the new one (i.e. MysSystemStateInnerV2).
3. Replace all uses of <code>MysSystemStateInner</code> with <code>MysSystemStateInnerV2</code> in both mys_system.move and mys_system_state_inner.move,
with the exception of the <code><a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner_create">mys_system_state_inner::create</a></code> function, which should always return the genesis type.
4. Inside <code><a href="../mys_system/mys_system.md#mys_system_mys_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a></code> function, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade the inner object. Make sure to also update the version in the wrapper.
A detailed example can be found in mys/tests/framework_upgrades/mock_mys_systems/shallow_upgrade.
Along with the Move change, we also need to update the Rust code to support the new type. This includes:
1. Define a new <code>MysSystemStateInner</code> struct type that matches the new Move type, and implement the MysSystemStateTrait.
2. Update the <code><a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a></code> struct to include the new version as a new enum variant.
3. Update the <code>get_mys_system_state</code> function to handle the new version.
To test that the upgrade will be successful, we need to modify <code>mys_system_state_production_upgrade_test</code> test in
protocol_version_tests and trigger a real upgrade using the new framework. We will need to keep this directory as old version,
put the new framework in a new directory, and run the test to exercise the upgrade.

To upgrade Validator type, besides everything above, we also need to:
1. Define a new Validator type (e.g. ValidatorV2).
2. Define a data migration function that migrates the old Validator to the new one (i.e. ValidatorV2).
3. Replace all uses of Validator with ValidatorV2 except the genesis creation function.
4. In validator_wrapper::upgrade_to_latest, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade it.
In Rust, we also need to add a new case in <code>get_validator_from_table</code>.
Note that it is possible to upgrade MysSystemStateInner without upgrading Validator, but not the other way around.
And when we only upgrade MysSystemStateInner, the version of Validator in the wrapper will not be updated, and hence may become
inconsistent with the version of MysSystemStateInner. This is fine as long as we don't use the Validator version to determine
the MysSystemStateInner version, or vice versa.


-  [Struct `MysSystemState`](#mys_system_mys_system_MysSystemState)
-  [Constants](#@Constants_0)
-  [Function `create`](#mys_system_mys_system_create)
-  [Function `request_add_validator_candidate`](#mys_system_mys_system_request_add_validator_candidate)
-  [Function `request_remove_validator_candidate`](#mys_system_mys_system_request_remove_validator_candidate)
-  [Function `request_add_validator`](#mys_system_mys_system_request_add_validator)
-  [Function `request_remove_validator`](#mys_system_mys_system_request_remove_validator)
-  [Function `request_set_gas_price`](#mys_system_mys_system_request_set_gas_price)
-  [Function `set_candidate_validator_gas_price`](#mys_system_mys_system_set_candidate_validator_gas_price)
-  [Function `request_set_commission_rate`](#mys_system_mys_system_request_set_commission_rate)
-  [Function `set_candidate_validator_commission_rate`](#mys_system_mys_system_set_candidate_validator_commission_rate)
-  [Function `request_add_stake`](#mys_system_mys_system_request_add_stake)
-  [Function `request_add_stake_non_entry`](#mys_system_mys_system_request_add_stake_non_entry)
-  [Function `request_add_stake_mul_coin`](#mys_system_mys_system_request_add_stake_mul_coin)
-  [Function `request_withdraw_stake`](#mys_system_mys_system_request_withdraw_stake)
-  [Function `convert_to_fungible_staked_mys`](#mys_system_mys_system_convert_to_fungible_staked_mys)
-  [Function `redeem_fungible_staked_mys`](#mys_system_mys_system_redeem_fungible_staked_mys)
-  [Function `request_withdraw_stake_non_entry`](#mys_system_mys_system_request_withdraw_stake_non_entry)
-  [Function `report_validator`](#mys_system_mys_system_report_validator)
-  [Function `undo_report_validator`](#mys_system_mys_system_undo_report_validator)
-  [Function `rotate_operation_cap`](#mys_system_mys_system_rotate_operation_cap)
-  [Function `update_validator_name`](#mys_system_mys_system_update_validator_name)
-  [Function `update_validator_description`](#mys_system_mys_system_update_validator_description)
-  [Function `update_validator_image_url`](#mys_system_mys_system_update_validator_image_url)
-  [Function `update_validator_project_url`](#mys_system_mys_system_update_validator_project_url)
-  [Function `update_validator_next_epoch_network_address`](#mys_system_mys_system_update_validator_next_epoch_network_address)
-  [Function `update_candidate_validator_network_address`](#mys_system_mys_system_update_candidate_validator_network_address)
-  [Function `update_validator_next_epoch_p2p_address`](#mys_system_mys_system_update_validator_next_epoch_p2p_address)
-  [Function `update_candidate_validator_p2p_address`](#mys_system_mys_system_update_candidate_validator_p2p_address)
-  [Function `update_validator_next_epoch_primary_address`](#mys_system_mys_system_update_validator_next_epoch_primary_address)
-  [Function `update_candidate_validator_primary_address`](#mys_system_mys_system_update_candidate_validator_primary_address)
-  [Function `update_validator_next_epoch_worker_address`](#mys_system_mys_system_update_validator_next_epoch_worker_address)
-  [Function `update_candidate_validator_worker_address`](#mys_system_mys_system_update_candidate_validator_worker_address)
-  [Function `update_validator_next_epoch_protocol_pubkey`](#mys_system_mys_system_update_validator_next_epoch_protocol_pubkey)
-  [Function `update_candidate_validator_protocol_pubkey`](#mys_system_mys_system_update_candidate_validator_protocol_pubkey)
-  [Function `update_validator_next_epoch_worker_pubkey`](#mys_system_mys_system_update_validator_next_epoch_worker_pubkey)
-  [Function `update_candidate_validator_worker_pubkey`](#mys_system_mys_system_update_candidate_validator_worker_pubkey)
-  [Function `update_validator_next_epoch_network_pubkey`](#mys_system_mys_system_update_validator_next_epoch_network_pubkey)
-  [Function `update_candidate_validator_network_pubkey`](#mys_system_mys_system_update_candidate_validator_network_pubkey)
-  [Function `validator_address_by_pool_id`](#mys_system_mys_system_validator_address_by_pool_id)
-  [Function `pool_exchange_rates`](#mys_system_mys_system_pool_exchange_rates)
-  [Function `active_validator_addresses`](#mys_system_mys_system_active_validator_addresses)
-  [Function `advance_epoch`](#mys_system_mys_system_advance_epoch)
-  [Function `load_system_state`](#mys_system_mys_system_load_system_state)
-  [Function `load_system_state_mut`](#mys_system_mys_system_load_system_state_mut)
-  [Function `load_inner_maybe_upgrade`](#mys_system_mys_system_load_inner_maybe_upgrade)
-  [Function `validator_voting_powers`](#mys_system_mys_system_validator_voting_powers)


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
<b>use</b> <a href="../mys/pay.md#mys_pay">mys::pay</a>;
<b>use</b> <a href="../mys/priority_queue.md#mys_priority_queue">mys::priority_queue</a>;
<b>use</b> <a href="../mys/mys.md#mys_mys">mys::mys</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/table_vec.md#mys_table_vec">mys::table_vec</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_map.md#mys_vec_map">mys::vec_map</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../mys/versioned.md#mys_versioned">mys::versioned</a>;
<b>use</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy">mys_system::stake_subsidy</a>;
<b>use</b> <a href="../mys_system/staking_pool.md#mys_system_staking_pool">mys_system::staking_pool</a>;
<b>use</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund">mys_system::storage_fund</a>;
<b>use</b> <a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner">mys_system::mys_system_state_inner</a>;
<b>use</b> <a href="../mys_system/validator.md#mys_system_validator">mys_system::validator</a>;
<b>use</b> <a href="../mys_system/validator_cap.md#mys_system_validator_cap">mys_system::validator_cap</a>;
<b>use</b> <a href="../mys_system/validator_set.md#mys_system_validator_set">mys_system::validator_set</a>;
<b>use</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper">mys_system::validator_wrapper</a>;
<b>use</b> <a href="../mys_system/voting_power.md#mys_system_voting_power">mys_system::voting_power</a>;
</code></pre>



<a name="mys_system_mys_system_MysSystemState"></a>

## Struct `MysSystemState`



<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a> <b>has</b> key
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
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_system_mys_system_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="mys_system_mys_system_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 1;
</code></pre>



<a name="mys_system_mys_system_create"></a>

## Function `create`

Create a new MysSystemState object and make it shared.
This function will be called only once in genesis.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_create">create</a>(id: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>, validators: vector&lt;<a href="../mys_system/validator.md#mys_system_validator_Validator">mys_system::validator::Validator</a>&gt;, <a href="../mys_system/storage_fund.md#mys_system_storage_fund">storage_fund</a>: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, protocol_version: u64, epoch_start_timestamp_ms: u64, parameters: <a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner_SystemParameters">mys_system::mys_system_state_inner::SystemParameters</a>, <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy">stake_subsidy</a>: <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_StakeSubsidy">mys_system::stake_subsidy::StakeSubsidy</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_create">create</a>(
    id: UID,
    validators: vector&lt;Validator&gt;,
    <a href="../mys_system/storage_fund.md#mys_system_storage_fund">storage_fund</a>: Balance&lt;MYS&gt;,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    parameters: SystemParameters,
    <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy">stake_subsidy</a>: StakeSubsidy,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner_create">mys_system_state_inner::create</a>(
        validators,
        <a href="../mys_system/storage_fund.md#mys_system_storage_fund">storage_fund</a>,
        protocol_version,
        epoch_start_timestamp_ms,
        parameters,
        <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy">stake_subsidy</a>,
        ctx,
    );
    <b>let</b> version = <a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner_genesis_system_state_version">mys_system_state_inner::genesis_system_state_version</a>();
    <b>let</b> <b>mut</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a> {
        id,
        version,
    };
    dynamic_field::add(&<b>mut</b> self.id, version, system_state);
    transfer::share_object(self);
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Can be called by anyone who wishes to become a validator candidate and starts accruing delegated
stakes in their staking pool. Once they have at least <code>MIN_VALIDATOR_JOINING_STAKE</code> amount of stake they
can call <code><a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_validator">request_add_validator</a></code> to officially become an active validator at the next epoch.
Aborts if the caller is already a pending or active validator, or a validator candidate.
Note: <code>proof_of_possession</code> MUST be a valid signature using mys_address and protocol_pubkey_bytes.
To produce a valid PoP, run [fn test_proof_of_possession].


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_validator_candidate">request_add_validator_candidate</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, worker_pubkey_bytes: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, image_url: vector&lt;u8&gt;, project_url: vector&lt;u8&gt;, net_address: vector&lt;u8&gt;, p2p_address: vector&lt;u8&gt;, primary_address: vector&lt;u8&gt;, worker_address: vector&lt;u8&gt;, gas_price: u64, commission_rate: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_validator_candidate">request_add_validator_candidate</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    pubkey_bytes: vector&lt;u8&gt;,
    network_pubkey_bytes: vector&lt;u8&gt;,
    worker_pubkey_bytes: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    image_url: vector&lt;u8&gt;,
    project_url: vector&lt;u8&gt;,
    net_address: vector&lt;u8&gt;,
    p2p_address: vector&lt;u8&gt;,
    primary_address: vector&lt;u8&gt;,
    worker_address: vector&lt;u8&gt;,
    gas_price: u64,
    commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_validator_candidate">request_add_validator_candidate</a>(
        pubkey_bytes,
        network_pubkey_bytes,
        worker_pubkey_bytes,
        proof_of_possession,
        name,
        description,
        image_url,
        project_url,
        net_address,
        p2p_address,
        primary_address,
        worker_address,
        gas_price,
        commission_rate,
        ctx,
    )
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by a validator candidate to remove themselves from the candidacy. After this call
their staking pool becomes deactivate.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_add_validator"></a>

## Function `request_add_validator`

Called by a validator candidate to add themselves to the active validator set beginning next epoch.
Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
epoch has already reached the maximum.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_validator">request_add_validator</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_validator">request_add_validator</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_validator">request_add_validator</a>(ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_remove_validator"></a>

## Function `request_remove_validator`

A validator can call this function to request a removal in the next epoch.
We use the sender of <code>ctx</code> to look up the validator
(i.e. sender must match the mys_address in the validator).
At the end of the epoch, the <code><a href="../mys_system/validator.md#mys_system_validator">validator</a></code> object will be returned to the mys_address
of the validator.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_remove_validator">request_remove_validator</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_remove_validator">request_remove_validator</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_request_remove_validator">request_remove_validator</a>(ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_set_gas_price"></a>

## Function `request_set_gas_price`

A validator can call this entry function to submit a new gas price quote, to be
used for the reference gas price calculation at the end of the epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_set_gas_price">request_set_gas_price</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, cap: &<a href="../mys_system/validator_cap.md#mys_system_validator_cap_UnverifiedValidatorOperationCap">mys_system::validator_cap::UnverifiedValidatorOperationCap</a>, new_gas_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_set_gas_price">request_set_gas_price</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    new_gas_price: u64,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_request_set_gas_price">request_set_gas_price</a>(cap, new_gas_price)
}
</code></pre>



</details>

<a name="mys_system_mys_system_set_candidate_validator_gas_price"></a>

## Function `set_candidate_validator_gas_price`

This entry function is used to set new gas price for candidate validators


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, cap: &<a href="../mys_system/validator_cap.md#mys_system_validator_cap_UnverifiedValidatorOperationCap">mys_system::validator_cap::UnverifiedValidatorOperationCap</a>, new_gas_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    new_gas_price: u64,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(cap, new_gas_price)
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

A validator can call this entry function to set a new commission rate, updated at the end of
the epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_set_commission_rate">request_set_commission_rate</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, new_commission_rate: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_set_commission_rate">request_set_commission_rate</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    new_commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_request_set_commission_rate">request_set_commission_rate</a>(new_commission_rate, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_set_candidate_validator_commission_rate"></a>

## Function `set_candidate_validator_commission_rate`

This entry function is used to set new commission rate for candidate validators


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, new_commission_rate: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    new_commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(new_commission_rate, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_add_stake"></a>

## Function `request_add_stake`

Add stake to a validator's staking pool.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_stake">request_add_stake</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, stake: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_stake">request_add_stake</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    stake: Coin&lt;MYS&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> staked_mys = <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(wrapper, stake, validator_address, ctx);
    transfer::public_transfer(staked_mys, ctx.sender());
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_add_stake_non_entry"></a>

## Function `request_add_stake_non_entry`

The non-entry version of <code><a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_stake">request_add_stake</a></code>, which returns the staked MYS instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, stake: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    stake: Coin&lt;MYS&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
): StakedMys {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_stake">request_add_stake</a>(stake, validator_address, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_add_stake_mul_coin"></a>

## Function `request_add_stake_mul_coin`

Add stake to a validator's staking pool using multiple coins.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, stakes: vector&lt;<a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;&gt;, stake_amount: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    stakes: vector&lt;Coin&lt;MYS&gt;&gt;,
    stake_amount: option::Option&lt;u64&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <b>let</b> staked_mys = self.<a href="../mys_system/mys_system.md#mys_system_mys_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(stakes, stake_amount, validator_address, ctx);
    transfer::public_transfer(staked_mys, ctx.sender());
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Withdraw stake from a validator's staking pool.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_withdraw_stake">request_withdraw_stake</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_withdraw_stake">request_withdraw_stake</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    staked_mys: StakedMys,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> withdrawn_stake = <a href="../mys_system/mys_system.md#mys_system_mys_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(wrapper, staked_mys, ctx);
    transfer::public_transfer(withdrawn_stake.into_coin(ctx), ctx.sender());
}
</code></pre>



</details>

<a name="mys_system_mys_system_convert_to_fungible_staked_mys"></a>

## Function `convert_to_fungible_staked_mys`

Convert StakedMys into a FungibleStakedMys object.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_convert_to_fungible_staked_mys">convert_to_fungible_staked_mys</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">mys_system::staking_pool::FungibleStakedMys</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_convert_to_fungible_staked_mys">convert_to_fungible_staked_mys</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    staked_mys: StakedMys,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedMys {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_convert_to_fungible_staked_mys">convert_to_fungible_staked_mys</a>(staked_mys, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_redeem_fungible_staked_mys"></a>

## Function `redeem_fungible_staked_mys`

Convert FungibleStakedMys into a StakedMys object.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_redeem_fungible_staked_mys">redeem_fungible_staked_mys</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, fungible_staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_FungibleStakedMys">mys_system::staking_pool::FungibleStakedMys</a>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_redeem_fungible_staked_mys">redeem_fungible_staked_mys</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    fungible_staked_mys: FungibleStakedMys,
    ctx: &TxContext,
): Balance&lt;MYS&gt; {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_redeem_fungible_staked_mys">redeem_fungible_staked_mys</a>(fungible_staked_mys, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_request_withdraw_stake_non_entry"></a>

## Function `request_withdraw_stake_non_entry`

Non-entry version of <code><a href="../mys_system/mys_system.md#mys_system_mys_system_request_withdraw_stake">request_withdraw_stake</a></code> that returns the withdrawn MYS instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, staked_mys: <a href="../mys_system/staking_pool.md#mys_system_staking_pool_StakedMys">mys_system::staking_pool::StakedMys</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    staked_mys: StakedMys,
    ctx: &<b>mut</b> TxContext,
) : Balance&lt;MYS&gt; {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_request_withdraw_stake">request_withdraw_stake</a>(staked_mys, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_report_validator"></a>

## Function `report_validator`

Report a validator as a bad or non-performant actor in the system.
Succeeds if all the following are satisfied:
1. both the reporter in <code>cap</code> and the input <code>reportee_addr</code> are active validators.
2. reporter and reportee not the same address.
3. the cap object is still valid.
This function is idempotent.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_report_validator">report_validator</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, cap: &<a href="../mys_system/validator_cap.md#mys_system_validator_cap_UnverifiedValidatorOperationCap">mys_system::validator_cap::UnverifiedValidatorOperationCap</a>, reportee_addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_report_validator">report_validator</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    reportee_addr: <b>address</b>,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_report_validator">report_validator</a>(cap, reportee_addr)
}
</code></pre>



</details>

<a name="mys_system_mys_system_undo_report_validator"></a>

## Function `undo_report_validator`

Undo a <code><a href="../mys_system/mys_system.md#mys_system_mys_system_report_validator">report_validator</a></code> action. Aborts if
1. the reportee is not a currently active validator or
2. the sender has not previously reported the <code>reportee_addr</code>, or
3. the cap is not valid


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_undo_report_validator">undo_report_validator</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, cap: &<a href="../mys_system/validator_cap.md#mys_system_validator_cap_UnverifiedValidatorOperationCap">mys_system::validator_cap::UnverifiedValidatorOperationCap</a>, reportee_addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_undo_report_validator">undo_report_validator</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    reportee_addr: <b>address</b>,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_undo_report_validator">undo_report_validator</a>(cap, reportee_addr)
}
</code></pre>



</details>

<a name="mys_system_mys_system_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>UnverifiedValidatorOperationCap</code>, transfer it to the
validator and registers it. The original object is thus revoked.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_rotate_operation_cap">rotate_operation_cap</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_rotate_operation_cap">rotate_operation_cap</a>(ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_name"></a>

## Function `update_validator_name`

Update a validator's name.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_name">update_validator_name</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, name: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_name">update_validator_name</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    name: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_name">update_validator_name</a>(name, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_description"></a>

## Function `update_validator_description`

Update a validator's description


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_description">update_validator_description</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, description: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_description">update_validator_description</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    description: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_description">update_validator_description</a>(description, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_image_url"></a>

## Function `update_validator_image_url`

Update a validator's image url


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_image_url">update_validator_image_url</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, image_url: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_image_url">update_validator_image_url</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    image_url: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_image_url">update_validator_image_url</a>(image_url, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_project_url"></a>

## Function `update_validator_project_url`

Update a validator's project url


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_project_url">update_validator_project_url</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, project_url: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_project_url">update_validator_project_url</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    project_url: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_project_url">update_validator_project_url</a>(project_url, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_next_epoch_network_address"></a>

## Function `update_validator_next_epoch_network_address`

Update a validator's network address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, network_address: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    network_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(network_address, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_candidate_validator_network_address"></a>

## Function `update_candidate_validator_network_address`

Update candidate validator's network address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, network_address: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    network_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(network_address, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_next_epoch_p2p_address"></a>

## Function `update_validator_next_epoch_p2p_address`

Update a validator's p2p address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, p2p_address: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    p2p_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(p2p_address, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_candidate_validator_p2p_address"></a>

## Function `update_candidate_validator_p2p_address`

Update candidate validator's p2p address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, p2p_address: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    p2p_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(p2p_address, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_next_epoch_primary_address"></a>

## Function `update_validator_next_epoch_primary_address`

Update a validator's narwhal primary address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, primary_address: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    primary_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(primary_address, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_candidate_validator_primary_address"></a>

## Function `update_candidate_validator_primary_address`

Update candidate validator's narwhal primary address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, primary_address: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    primary_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(primary_address, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_next_epoch_worker_address"></a>

## Function `update_validator_next_epoch_worker_address`

Update a validator's narwhal worker address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, worker_address: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    worker_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(worker_address, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_candidate_validator_worker_address"></a>

## Function `update_candidate_validator_worker_address`

Update candidate validator's narwhal worker address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, worker_address: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    worker_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(worker_address, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_next_epoch_protocol_pubkey"></a>

## Function `update_validator_next_epoch_protocol_pubkey`

Update a validator's public key of protocol key and proof of possession.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, protocol_pubkey: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(protocol_pubkey, proof_of_possession, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_candidate_validator_protocol_pubkey"></a>

## Function `update_candidate_validator_protocol_pubkey`

Update candidate validator's public key of protocol key and proof of possession.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, protocol_pubkey: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(protocol_pubkey, proof_of_possession, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_next_epoch_worker_pubkey"></a>

## Function `update_validator_next_epoch_worker_pubkey`

Update a validator's public key of worker key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, worker_pubkey: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    worker_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(worker_pubkey, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_candidate_validator_worker_pubkey"></a>

## Function `update_candidate_validator_worker_pubkey`

Update candidate validator's public key of worker key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, worker_pubkey: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    worker_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(worker_pubkey, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_validator_next_epoch_network_pubkey"></a>

## Function `update_validator_next_epoch_network_pubkey`

Update a validator's public key of network key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, network_pubkey: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    network_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(network_pubkey, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_update_candidate_validator_network_pubkey"></a>

## Function `update_candidate_validator_network_pubkey`

Update candidate validator's public key of network key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, network_pubkey: vector&lt;u8&gt;, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(
    self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    network_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(network_pubkey, ctx)
}
</code></pre>



</details>

<a name="mys_system_mys_system_validator_address_by_pool_id"></a>

## Function `validator_address_by_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_validator_address_by_pool_id">validator_address_by_pool_id</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, pool_id: &<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_validator_address_by_pool_id">validator_address_by_pool_id</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>, pool_id: &ID): <b>address</b> {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_validator_address_by_pool_id">validator_address_by_pool_id</a>(pool_id)
}
</code></pre>



</details>

<a name="mys_system_mys_system_pool_exchange_rates"></a>

## Function `pool_exchange_rates`

Getter of the pool token exchange rate of a staking pool. Works for both active and inactive pools.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_pool_exchange_rates">pool_exchange_rates</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, pool_id: &<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>): &<a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u64, <a href="../mys_system/staking_pool.md#mys_system_staking_pool_PoolTokenExchangeRate">mys_system::staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_pool_exchange_rates">pool_exchange_rates</a>(
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    pool_id: &ID
): &Table&lt;u64, PoolTokenExchangeRate&gt;  {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_pool_exchange_rates">pool_exchange_rates</a>(pool_id)
}
</code></pre>



</details>

<a name="mys_system_mys_system_active_validator_addresses"></a>

## Function `active_validator_addresses`

Getter returning addresses of the currently active validators.


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_active_validator_addresses">active_validator_addresses</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_active_validator_addresses">active_validator_addresses</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>): vector&lt;<b>address</b>&gt; {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state">load_system_state</a>(wrapper);
    self.<a href="../mys_system/mys_system.md#mys_system_mys_system_active_validator_addresses">active_validator_addresses</a>()
}
</code></pre>



</details>

<a name="mys_system_mys_system_advance_epoch"></a>

## Function `advance_epoch`

This function should be called at the end of an epoch, and advances the system to the next epoch.
It does the following things:
1. Add storage charge to the storage fund.
2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
gas coins.
3. Distribute computation charge to validator stake.
4. Update all validators.


<pre><code><b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_advance_epoch">advance_epoch</a>(storage_reward: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, computation_reward: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>, new_epoch: u64, next_protocol_version: u64, storage_rebate: u64, non_refundable_storage_fee: u64, storage_fund_reinvest_rate: u64, reward_slashing_rate: u64, epoch_start_timestamp_ms: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_advance_epoch">advance_epoch</a>(
    storage_reward: Balance&lt;MYS&gt;,
    computation_reward: Balance&lt;MYS&gt;,
    wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>,
    new_epoch: u64,
    next_protocol_version: u64,
    storage_rebate: u64,
    non_refundable_storage_fee: u64,
    storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
                                     // into storage fund, in basis point.
    reward_slashing_rate: u64, // how much rewards are slashed to punish a <a href="../mys_system/validator.md#mys_system_validator">validator</a>, in bps.
    epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
    ctx: &<b>mut</b> TxContext,
) : Balance&lt;MYS&gt; {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    // Validator will make a special system call with sender set <b>as</b> 0x0.
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../mys_system/mys_system.md#mys_system_mys_system_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> storage_rebate = self.<a href="../mys_system/mys_system.md#mys_system_mys_system_advance_epoch">advance_epoch</a>(
        new_epoch,
        next_protocol_version,
        storage_reward,
        computation_reward,
        storage_rebate,
        non_refundable_storage_fee,
        storage_fund_reinvest_rate,
        reward_slashing_rate,
        epoch_start_timestamp_ms,
        ctx,
    );
    storage_rebate
}
</code></pre>



</details>

<a name="mys_system_mys_system_load_system_state"></a>

## Function `load_system_state`



<pre><code><b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state">load_system_state</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>): &<a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner_MysSystemStateInnerV2">mys_system::mys_system_state_inner::MysSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state">load_system_state</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>): &MysSystemStateInnerV2 {
    <a href="../mys_system/mys_system.md#mys_system_mys_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self)
}
</code></pre>



</details>

<a name="mys_system_mys_system_load_system_state_mut"></a>

## Function `load_system_state_mut`



<pre><code><b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>): &<b>mut</b> <a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner_MysSystemStateInnerV2">mys_system::mys_system_state_inner::MysSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state_mut">load_system_state_mut</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>): &<b>mut</b> MysSystemStateInnerV2 {
    <a href="../mys_system/mys_system.md#mys_system_mys_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self)
}
</code></pre>



</details>

<a name="mys_system_mys_system_load_inner_maybe_upgrade"></a>

## Function `load_inner_maybe_upgrade`



<pre><code><b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>): &<b>mut</b> <a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner_MysSystemStateInnerV2">mys_system::mys_system_state_inner::MysSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>): &<b>mut</b> MysSystemStateInnerV2 {
    <b>if</b> (self.version == 1) {
      <b>let</b> v1: MysSystemStateInner = dynamic_field::remove(&<b>mut</b> self.id, self.version);
      <b>let</b> v2 = v1.v1_to_v2();
      self.version = 2;
      dynamic_field::add(&<b>mut</b> self.id, self.version, v2);
    };
    <b>let</b> inner: &<b>mut</b> MysSystemStateInnerV2 = dynamic_field::borrow_mut(
        &<b>mut</b> self.id,
        self.version
    );
    <b>assert</b>!(inner.system_state_version() == self.version, <a href="../mys_system/mys_system.md#mys_system_mys_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="mys_system_mys_system_validator_voting_powers"></a>

## Function `validator_voting_powers`

Returns the voting power of the active validators, values are voting power in the scale of 10000.


<pre><code><b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_validator_voting_powers">validator_voting_powers</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">mys_system::mys_system::MysSystemState</a>): <a href="../mys/vec_map.md#mys_vec_map_VecMap">mys::vec_map::VecMap</a>&lt;<b>address</b>, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_validator_voting_powers">validator_voting_powers</a>(wrapper: &<b>mut</b> <a href="../mys_system/mys_system.md#mys_system_mys_system_MysSystemState">MysSystemState</a>): VecMap&lt;<b>address</b>, u64&gt; {
    <b>let</b> self = <a href="../mys_system/mys_system.md#mys_system_mys_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner_active_validator_voting_powers">mys_system_state_inner::active_validator_voting_powers</a>(self)
}
</code></pre>



</details>
