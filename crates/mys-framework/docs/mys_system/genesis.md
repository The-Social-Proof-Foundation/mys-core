---
title: Module `mys_system::genesis`
---



-  [Struct `GenesisValidatorMetadata`](#mys_system_genesis_GenesisValidatorMetadata)
-  [Struct `GenesisChainParameters`](#mys_system_genesis_GenesisChainParameters)
-  [Struct `TokenDistributionSchedule`](#mys_system_genesis_TokenDistributionSchedule)
-  [Struct `TokenAllocation`](#mys_system_genesis_TokenAllocation)
-  [Constants](#@Constants_0)
-  [Function `create`](#mys_system_genesis_create)
-  [Function `allocate_tokens`](#mys_system_genesis_allocate_tokens)
-  [Function `activate_validators`](#mys_system_genesis_activate_validators)


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
<b>use</b> <a href="../mys_system/mys_system.md#mys_system_mys_system">mys_system::mys_system</a>;
<b>use</b> <a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner">mys_system::mys_system_state_inner</a>;
<b>use</b> <a href="../mys_system/validator.md#mys_system_validator">mys_system::validator</a>;
<b>use</b> <a href="../mys_system/validator_cap.md#mys_system_validator_cap">mys_system::validator_cap</a>;
<b>use</b> <a href="../mys_system/validator_set.md#mys_system_validator_set">mys_system::validator_set</a>;
<b>use</b> <a href="../mys_system/validator_wrapper.md#mys_system_validator_wrapper">mys_system::validator_wrapper</a>;
<b>use</b> <a href="../mys_system/voting_power.md#mys_system_voting_power">mys_system::voting_power</a>;
</code></pre>



<a name="mys_system_genesis_GenesisValidatorMetadata"></a>

## Struct `GenesisValidatorMetadata`



<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/genesis.md#mys_system_genesis_GenesisValidatorMetadata">GenesisValidatorMetadata</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>name: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>description: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>image_url: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>project_url: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>mys_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>gas_price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>commission_rate: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>protocol_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>proof_of_possession: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>network_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>worker_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>network_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>p2p_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>primary_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>worker_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_system_genesis_GenesisChainParameters"></a>

## Struct `GenesisChainParameters`



<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/genesis.md#mys_system_genesis_GenesisChainParameters">GenesisChainParameters</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>protocol_version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>chain_start_timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>epoch_duration_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_start_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_initial_distribution_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_period_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_decrease_rate: u16</code>
</dt>
<dd>
</dd>
<dt>
<code>max_validator_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_validator_joining_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>validator_low_stake_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>validator_very_low_stake_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>validator_low_stake_grace_period: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_system_genesis_TokenDistributionSchedule"></a>

## Struct `TokenDistributionSchedule`



<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/genesis.md#mys_system_genesis_TokenDistributionSchedule">TokenDistributionSchedule</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>stake_subsidy_fund_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>allocations: vector&lt;<a href="../mys_system/genesis.md#mys_system_genesis_TokenAllocation">mys_system::genesis::TokenAllocation</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_system_genesis_TokenAllocation"></a>

## Struct `TokenAllocation`



<pre><code><b>public</b> <b>struct</b> <a href="../mys_system/genesis.md#mys_system_genesis_TokenAllocation">TokenAllocation</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>recipient_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>staked_with_validator: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Indicates if this allocation should be staked at genesis and with which validator
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_system_genesis_EDuplicateValidator"></a>

The <code><a href="../mys_system/genesis.md#mys_system_genesis_create">create</a></code> function was called with duplicate validators.


<pre><code><b>const</b> <a href="../mys_system/genesis.md#mys_system_genesis_EDuplicateValidator">EDuplicateValidator</a>: u64 = 1;
</code></pre>



<a name="mys_system_genesis_ENotCalledAtGenesis"></a>

The <code><a href="../mys_system/genesis.md#mys_system_genesis_create">create</a></code> function was called at a non-genesis epoch.


<pre><code><b>const</b> <a href="../mys_system/genesis.md#mys_system_genesis_ENotCalledAtGenesis">ENotCalledAtGenesis</a>: u64 = 0;
</code></pre>



<a name="mys_system_genesis_create"></a>

## Function `create`

This function will be explicitly called once at genesis.
It will create a singleton MysSystemState object, which contains
all the information we need in the system.


<pre><code><b>fun</b> <a href="../mys_system/genesis.md#mys_system_genesis_create">create</a>(mys_system_state_id: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>, mys_supply: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, genesis_chain_parameters: <a href="../mys_system/genesis.md#mys_system_genesis_GenesisChainParameters">mys_system::genesis::GenesisChainParameters</a>, genesis_validators: vector&lt;<a href="../mys_system/genesis.md#mys_system_genesis_GenesisValidatorMetadata">mys_system::genesis::GenesisValidatorMetadata</a>&gt;, token_distribution_schedule: <a href="../mys_system/genesis.md#mys_system_genesis_TokenDistributionSchedule">mys_system::genesis::TokenDistributionSchedule</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/genesis.md#mys_system_genesis_create">create</a>(
    mys_system_state_id: UID,
    <b>mut</b> mys_supply: Balance&lt;MYS&gt;,
    genesis_chain_parameters: <a href="../mys_system/genesis.md#mys_system_genesis_GenesisChainParameters">GenesisChainParameters</a>,
    genesis_validators: vector&lt;<a href="../mys_system/genesis.md#mys_system_genesis_GenesisValidatorMetadata">GenesisValidatorMetadata</a>&gt;,
    token_distribution_schedule: <a href="../mys_system/genesis.md#mys_system_genesis_TokenDistributionSchedule">TokenDistributionSchedule</a>,
    ctx: &<b>mut</b> TxContext,
) {
    // Ensure this is only called at <a href="../mys_system/genesis.md#mys_system_genesis">genesis</a>
    <b>assert</b>!(ctx.epoch() == 0, <a href="../mys_system/genesis.md#mys_system_genesis_ENotCalledAtGenesis">ENotCalledAtGenesis</a>);
    <b>let</b> <a href="../mys_system/genesis.md#mys_system_genesis_TokenDistributionSchedule">TokenDistributionSchedule</a> {
        stake_subsidy_fund_mist,
        allocations,
    } = token_distribution_schedule;
    <b>let</b> subsidy_fund = mys_supply.split(stake_subsidy_fund_mist);
    <b>let</b> <a href="../mys_system/storage_fund.md#mys_system_storage_fund">storage_fund</a> = balance::zero();
    // Create all the `Validator` structs
    <b>let</b> <b>mut</b> validators = vector[];
    <b>let</b> count = genesis_validators.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; count) {
        <b>let</b> <a href="../mys_system/genesis.md#mys_system_genesis_GenesisValidatorMetadata">GenesisValidatorMetadata</a> {
            name,
            description,
            image_url,
            project_url,
            mys_address,
            gas_price,
            commission_rate,
            protocol_public_key,
            proof_of_possession,
            network_public_key,
            worker_public_key,
            network_address,
            p2p_address,
            primary_address,
            worker_address,
        } = genesis_validators[i];
        <b>let</b> <a href="../mys_system/validator.md#mys_system_validator">validator</a> = <a href="../mys_system/validator.md#mys_system_validator_new">validator::new</a>(
            mys_address,
            protocol_public_key,
            network_public_key,
            worker_public_key,
            proof_of_possession,
            name,
            description,
            image_url,
            project_url,
            network_address,
            p2p_address,
            primary_address,
            worker_address,
            gas_price,
            commission_rate,
            ctx
        );
        // Ensure that each <a href="../mys_system/validator.md#mys_system_validator">validator</a> is unique
        <b>assert</b>!(
            !<a href="../mys_system/validator_set.md#mys_system_validator_set_is_duplicate_validator">validator_set::is_duplicate_validator</a>(&validators, &<a href="../mys_system/validator.md#mys_system_validator">validator</a>),
            <a href="../mys_system/genesis.md#mys_system_genesis_EDuplicateValidator">EDuplicateValidator</a>,
        );
        validators.push_back(<a href="../mys_system/validator.md#mys_system_validator">validator</a>);
        i = i + 1;
    };
    // Allocate tokens and staking operations
    <a href="../mys_system/genesis.md#mys_system_genesis_allocate_tokens">allocate_tokens</a>(
        mys_supply,
        allocations,
        &<b>mut</b> validators,
        ctx
    );
    // Activate all validators
    <a href="../mys_system/genesis.md#mys_system_genesis_activate_validators">activate_validators</a>(&<b>mut</b> validators);
    <b>let</b> system_parameters = <a href="../mys_system/mys_system_state_inner.md#mys_system_mys_system_state_inner_create_system_parameters">mys_system_state_inner::create_system_parameters</a>(
        genesis_chain_parameters.epoch_duration_ms,
        genesis_chain_parameters.stake_subsidy_start_epoch,
        // Validator committee parameters
        genesis_chain_parameters.max_validator_count,
        genesis_chain_parameters.min_validator_joining_stake,
        genesis_chain_parameters.validator_low_stake_threshold,
        genesis_chain_parameters.validator_very_low_stake_threshold,
        genesis_chain_parameters.validator_low_stake_grace_period,
        ctx,
    );
    <b>let</b> <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy">stake_subsidy</a> = <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy_create">stake_subsidy::create</a>(
        subsidy_fund,
        genesis_chain_parameters.stake_subsidy_initial_distribution_amount,
        genesis_chain_parameters.stake_subsidy_period_length,
        genesis_chain_parameters.stake_subsidy_decrease_rate,
        ctx,
    );
    mys_system::create(
        mys_system_state_id,
        validators,
        <a href="../mys_system/storage_fund.md#mys_system_storage_fund">storage_fund</a>,
        genesis_chain_parameters.protocol_version,
        genesis_chain_parameters.chain_start_timestamp_ms,
        system_parameters,
        <a href="../mys_system/stake_subsidy.md#mys_system_stake_subsidy">stake_subsidy</a>,
        ctx,
    );
}
</code></pre>



</details>

<a name="mys_system_genesis_allocate_tokens"></a>

## Function `allocate_tokens`



<pre><code><b>fun</b> <a href="../mys_system/genesis.md#mys_system_genesis_allocate_tokens">allocate_tokens</a>(mys_supply: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, allocations: vector&lt;<a href="../mys_system/genesis.md#mys_system_genesis_TokenAllocation">mys_system::genesis::TokenAllocation</a>&gt;, validators: &<b>mut</b> vector&lt;<a href="../mys_system/validator.md#mys_system_validator_Validator">mys_system::validator::Validator</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/genesis.md#mys_system_genesis_allocate_tokens">allocate_tokens</a>(
    <b>mut</b> mys_supply: Balance&lt;MYS&gt;,
    <b>mut</b> allocations: vector&lt;<a href="../mys_system/genesis.md#mys_system_genesis_TokenAllocation">TokenAllocation</a>&gt;,
    validators: &<b>mut</b> vector&lt;Validator&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>while</b> (!allocations.is_empty()) {
        <b>let</b> <a href="../mys_system/genesis.md#mys_system_genesis_TokenAllocation">TokenAllocation</a> {
            recipient_address,
            amount_mist,
            staked_with_validator,
        } = allocations.pop_back();
        <b>let</b> allocation_balance = mys_supply.split(amount_mist);
        <b>if</b> (staked_with_validator.is_some()) {
            <b>let</b> validator_address = staked_with_validator.destroy_some();
            <b>let</b> <a href="../mys_system/validator.md#mys_system_validator">validator</a> = <a href="../mys_system/validator_set.md#mys_system_validator_set_get_validator_mut">validator_set::get_validator_mut</a>(validators, validator_address);
            <a href="../mys_system/validator.md#mys_system_validator">validator</a>.request_add_stake_at_genesis(
                allocation_balance,
                recipient_address,
                ctx
            );
        } <b>else</b> {
            <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>(
                allocation_balance.into_coin(ctx),
                recipient_address,
            );
        };
    };
    allocations.destroy_empty();
    // Provided allocations must fully allocate the mys_supply and there
    // should be none left at this point.
    mys_supply.destroy_zero();
}
</code></pre>



</details>

<a name="mys_system_genesis_activate_validators"></a>

## Function `activate_validators`



<pre><code><b>fun</b> <a href="../mys_system/genesis.md#mys_system_genesis_activate_validators">activate_validators</a>(validators: &<b>mut</b> vector&lt;<a href="../mys_system/validator.md#mys_system_validator_Validator">mys_system::validator::Validator</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mys_system/genesis.md#mys_system_genesis_activate_validators">activate_validators</a>(validators: &<b>mut</b> vector&lt;Validator&gt;) {
    // Activate all <a href="../mys_system/genesis.md#mys_system_genesis">genesis</a> validators
    <b>let</b> count = validators.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; count) {
        <b>let</b> <a href="../mys_system/validator.md#mys_system_validator">validator</a> = &<b>mut</b> validators[i];
        <a href="../mys_system/validator.md#mys_system_validator">validator</a>.activate(0);
        i = i + 1;
    };
}
</code></pre>



</details>
