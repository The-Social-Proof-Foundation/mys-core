---
title: Module `mys::transfer_policy`
---

Defines the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code> type and the logic to approve <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code>s.

- TransferPolicy - is a highly customizable primitive, which provides an
interface for the type owner to set custom transfer rules for every
deal performed in the <code>Kiosk</code> or a similar system that integrates with TP.

- Once a <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;</code> is created for and shared (or frozen), the
type <code>T</code> becomes tradable in <code>Kiosk</code>s. On every purchase operation, a
<code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code> is created and needs to be confirmed by the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>
hot potato or transaction will fail.

- Type owner (creator) can set any Rules as long as the ecosystem supports
them. All of the Rules need to be resolved within a single transaction (eg
pay royalty and pay fixed commission). Once required actions are performed,
the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code> can be "confirmed" via <code><a href="../mys/transfer_policy.md#mys_transfer_policy_confirm_request">confirm_request</a></code> call.

- <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code> aims to be the main interface for creators to control trades
of their types and collect profits if a fee is required on sales. Custom
policies can be removed at any moment, and the change will affect all instances
of the type at once.


-  [Struct `TransferRequest`](#mys_transfer_policy_TransferRequest)
-  [Struct `TransferPolicy`](#mys_transfer_policy_TransferPolicy)
-  [Struct `TransferPolicyCap`](#mys_transfer_policy_TransferPolicyCap)
-  [Struct `TransferPolicyCreated`](#mys_transfer_policy_TransferPolicyCreated)
-  [Struct `TransferPolicyDestroyed`](#mys_transfer_policy_TransferPolicyDestroyed)
-  [Struct `RuleKey`](#mys_transfer_policy_RuleKey)
-  [Constants](#@Constants_0)
-  [Function `new_request`](#mys_transfer_policy_new_request)
-  [Function `new`](#mys_transfer_policy_new)
-  [Function `default`](#mys_transfer_policy_default)
-  [Function `withdraw`](#mys_transfer_policy_withdraw)
-  [Function `destroy_and_withdraw`](#mys_transfer_policy_destroy_and_withdraw)
-  [Function `confirm_request`](#mys_transfer_policy_confirm_request)
-  [Function `add_rule`](#mys_transfer_policy_add_rule)
-  [Function `get_rule`](#mys_transfer_policy_get_rule)
-  [Function `add_to_balance`](#mys_transfer_policy_add_to_balance)
-  [Function `add_receipt`](#mys_transfer_policy_add_receipt)
-  [Function `has_rule`](#mys_transfer_policy_has_rule)
-  [Function `remove_rule`](#mys_transfer_policy_remove_rule)
-  [Function `uid`](#mys_transfer_policy_uid)
-  [Function `uid_mut_as_owner`](#mys_transfer_policy_uid_mut_as_owner)
-  [Function `rules`](#mys_transfer_policy_rules)
-  [Function `item`](#mys_transfer_policy_item)
-  [Function `paid`](#mys_transfer_policy_paid)
-  [Function `from`](#mys_transfer_policy_from)


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
<b>use</b> <a href="../mys/package.md#mys_package">mys::package</a>;
<b>use</b> <a href="../mys/mys.md#mys_mys">mys::mys</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
</code></pre>



<a name="mys_transfer_policy_TransferRequest"></a>

## Struct `TransferRequest`

A "Hot Potato" forcing the buyer to get a transfer permission
from the item type (<code>T</code>) owner on purchase attempt.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a>&lt;<b>phantom</b> T&gt;
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../mys/transfer_policy.md#mys_transfer_policy_item">item</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
 The ID of the transferred item. Although the <code>T</code> has no
 constraints, the main use case for this module is to work
 with Objects.
</dd>
<dt>
<code><a href="../mys/transfer_policy.md#mys_transfer_policy_paid">paid</a>: u64</code>
</dt>
<dd>
 Amount of MYS paid for the item. Can be used to
 calculate the fee / transfer policy enforcement.
</dd>
<dt>
<code><a href="../mys/transfer_policy.md#mys_transfer_policy_from">from</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
 The ID of the Kiosk / Safe the object is being sold from.
 Can be used by the TransferPolicy implementors.
</dd>
<dt>
<code>receipts: <a href="../mys/vec_set.md#mys_vec_set_VecSet">mys::vec_set::VecSet</a>&lt;<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>&gt;</code>
</dt>
<dd>
 Collected Receipts. Used to verify that all of the rules
 were followed and <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code> can be confirmed.
</dd>
</dl>


</details>

<a name="mys_transfer_policy_TransferPolicy"></a>

## Struct `TransferPolicy`

A unique capability that allows the owner of the <code>T</code> to authorize
transfers. Can only be created with the <code>Publisher</code> object. Although
there's no limitation to how many policies can be created, for most
of the cases there's no need to create more than one since any of the
policies can be used to confirm the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
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
<code><a href="../mys/balance.md#mys_balance">balance</a>: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 The Balance of the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code> which collects <code>MYS</code>.
 By default, transfer policy does not collect anything , and it's
 a matter of an implementation of a specific rule - whether to add
 to balance and how much.
</dd>
<dt>
<code><a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a>: <a href="../mys/vec_set.md#mys_vec_set_VecSet">mys::vec_set::VecSet</a>&lt;<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>&gt;</code>
</dt>
<dd>
 Set of types of attached rules - used to verify <code>receipts</code> when
 a <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code> is received in <code><a href="../mys/transfer_policy.md#mys_transfer_policy_confirm_request">confirm_request</a></code> function.
 Additionally provides a way to look up currently attached Rules.
</dd>
</dl>


</details>

<a name="mys_transfer_policy_TransferPolicyCap"></a>

## Struct `TransferPolicyCap`

A Capability granting the owner permission to add/remove rules as well
as to <code><a href="../mys/transfer_policy.md#mys_transfer_policy_withdraw">withdraw</a></code> and <code><a href="../mys/transfer_policy.md#mys_transfer_policy_destroy_and_withdraw">destroy_and_withdraw</a></code> the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
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
<code>policy_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_transfer_policy_TransferPolicyCreated"></a>

## Struct `TransferPolicyCreated`

Event that is emitted when a publisher creates a new <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a></code>
making the discoverability and tracking the supported types easier.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCreated">TransferPolicyCreated</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_transfer_policy_TransferPolicyDestroyed"></a>

## Struct `TransferPolicyDestroyed`

Event that is emitted when a publisher destroys a <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a></code>.
Allows for tracking supported policies.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyDestroyed">TransferPolicyDestroyed</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_transfer_policy_RuleKey"></a>

## Struct `RuleKey`

Key to store "Rule" configuration for a specific <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>.


<pre><code><b>public</b> <b>struct</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_RuleKey">RuleKey</a>&lt;<b>phantom</b> T: drop&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_transfer_policy_EIllegalRule"></a>

A completed rule is not set in the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>.


<pre><code><b>const</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_EIllegalRule">EIllegalRule</a>: u64 = 1;
</code></pre>



<a name="mys_transfer_policy_ENotEnough"></a>

Trying to <code><a href="../mys/transfer_policy.md#mys_transfer_policy_withdraw">withdraw</a></code> more than there is.


<pre><code><b>const</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_ENotEnough">ENotEnough</a>: u64 = 5;
</code></pre>



<a name="mys_transfer_policy_ENotOwner"></a>

Trying to <code><a href="../mys/transfer_policy.md#mys_transfer_policy_withdraw">withdraw</a></code> or <code>close_and_withdraw</code> with a wrong Cap.


<pre><code><b>const</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_ENotOwner">ENotOwner</a>: u64 = 4;
</code></pre>



<a name="mys_transfer_policy_EPolicyNotSatisfied"></a>

The number of receipts does not match the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code> requirement.


<pre><code><b>const</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_EPolicyNotSatisfied">EPolicyNotSatisfied</a>: u64 = 0;
</code></pre>



<a name="mys_transfer_policy_ERuleAlreadySet"></a>

Attempting to create a Rule that is already set.


<pre><code><b>const</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_ERuleAlreadySet">ERuleAlreadySet</a>: u64 = 3;
</code></pre>



<a name="mys_transfer_policy_EUnknownRequirement"></a>

A Rule is not set.


<pre><code><b>const</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_EUnknownRequirement">EUnknownRequirement</a>: u64 = 2;
</code></pre>



<a name="mys_transfer_policy_new_request"></a>

## Function `new_request`

Construct a new <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code> hot potato which requires an
approving action from the creator to be destroyed / resolved. Once
created, it must be confirmed in the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_confirm_request">confirm_request</a></code> call otherwise
the transaction will fail.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_new_request">new_request</a>&lt;T&gt;(<a href="../mys/transfer_policy.md#mys_transfer_policy_item">item</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, <a href="../mys/transfer_policy.md#mys_transfer_policy_paid">paid</a>: u64, <a href="../mys/transfer_policy.md#mys_transfer_policy_from">from</a>: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>): <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">mys::transfer_policy::TransferRequest</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_new_request">new_request</a>&lt;T&gt;(<a href="../mys/transfer_policy.md#mys_transfer_policy_item">item</a>: ID, <a href="../mys/transfer_policy.md#mys_transfer_policy_paid">paid</a>: u64, <a href="../mys/transfer_policy.md#mys_transfer_policy_from">from</a>: ID): <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a>&lt;T&gt; {
    <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a> { <a href="../mys/transfer_policy.md#mys_transfer_policy_item">item</a>, <a href="../mys/transfer_policy.md#mys_transfer_policy_paid">paid</a>, <a href="../mys/transfer_policy.md#mys_transfer_policy_from">from</a>, receipts: <a href="../mys/vec_set.md#mys_vec_set_empty">vec_set::empty</a>() }
}
</code></pre>



</details>

<a name="mys_transfer_policy_new"></a>

## Function `new`

Register a type in the Kiosk system and receive a <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code> and
a <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a></code> for the type. The <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code> is required to
confirm kiosk deals for the <code>T</code>. If there's no <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>
available for use, the type can not be traded in kiosks.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_new">new</a>&lt;T&gt;(pub: &<a href="../mys/package.md#mys_package_Publisher">mys::package::Publisher</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): (<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;, <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">mys::transfer_policy::TransferPolicyCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_new">new</a>&lt;T&gt;(pub: &Publisher, ctx: &<b>mut</b> TxContext): (<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;, <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a>&lt;T&gt;) {
    <b>assert</b>!(<a href="../mys/package.md#mys_package_from_package">package::from_package</a>&lt;T&gt;(pub), 0);
    <b>let</b> id = <a href="../mys/object.md#mys_object_new">object::new</a>(ctx);
    <b>let</b> policy_id = id.to_inner();
    <a href="../mys/event.md#mys_event_emit">event::emit</a>(<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCreated">TransferPolicyCreated</a>&lt;T&gt; { id: policy_id });
    (
        <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a> { id, <a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a>: <a href="../mys/vec_set.md#mys_vec_set_empty">vec_set::empty</a>(), <a href="../mys/balance.md#mys_balance">balance</a>: <a href="../mys/balance.md#mys_balance_zero">balance::zero</a>() },
        <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a> { id: <a href="../mys/object.md#mys_object_new">object::new</a>(ctx), policy_id },
    )
}
</code></pre>



</details>

<a name="mys_transfer_policy_default"></a>

## Function `default`

Initialize the Transfer Policy in the default scenario: Create and share
the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>, transfer <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a></code> to the transaction
sender.


<pre><code><b>entry</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_default">default</a>&lt;T&gt;(pub: &<a href="../mys/package.md#mys_package_Publisher">mys::package::Publisher</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_default">default</a>&lt;T&gt;(pub: &Publisher, ctx: &<b>mut</b> TxContext) {
    <b>let</b> (policy, cap) = <a href="../mys/transfer_policy.md#mys_transfer_policy_new">new</a>&lt;T&gt;(pub, ctx);
    <a href="../mys/transfer.md#mys_transfer_share_object">mys::transfer::share_object</a>(policy);
    <a href="../mys/transfer.md#mys_transfer_transfer">mys::transfer::transfer</a>(cap, ctx.sender());
}
</code></pre>



</details>

<a name="mys_transfer_policy_withdraw"></a>

## Function `withdraw`

Withdraw some amount of profits from the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>. If amount
is not specified, all profits are withdrawn.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_withdraw">withdraw</a>&lt;T&gt;(self: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;, cap: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">mys::transfer_policy::TransferPolicyCap</a>&lt;T&gt;, amount: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_withdraw">withdraw</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;,
    cap: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a>&lt;T&gt;,
    amount: Option&lt;u64&gt;,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;MYS&gt; {
    <b>assert</b>!(<a href="../mys/object.md#mys_object_id">object::id</a>(self) == cap.policy_id, <a href="../mys/transfer_policy.md#mys_transfer_policy_ENotOwner">ENotOwner</a>);
    <b>let</b> amount = <b>if</b> (amount.is_some()) {
        <b>let</b> amt = amount.destroy_some();
        <b>assert</b>!(amt &lt;= self.<a href="../mys/balance.md#mys_balance">balance</a>.value(), <a href="../mys/transfer_policy.md#mys_transfer_policy_ENotEnough">ENotEnough</a>);
        amt
    } <b>else</b> {
        self.<a href="../mys/balance.md#mys_balance">balance</a>.value()
    };
    <a href="../mys/coin.md#mys_coin_take">coin::take</a>(&<b>mut</b> self.<a href="../mys/balance.md#mys_balance">balance</a>, amount, ctx)
}
</code></pre>



</details>

<a name="mys_transfer_policy_destroy_and_withdraw"></a>

## Function `destroy_and_withdraw`

Destroy a TransferPolicyCap.
Can be performed by any party as long as they own it.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_destroy_and_withdraw">destroy_and_withdraw</a>&lt;T&gt;(self: <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;, cap: <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">mys::transfer_policy::TransferPolicyCap</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_destroy_and_withdraw">destroy_and_withdraw</a>&lt;T&gt;(
    self: <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;,
    cap: <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a>&lt;T&gt;,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;MYS&gt; {
    <b>assert</b>!(<a href="../mys/object.md#mys_object_id">object::id</a>(&self) == cap.policy_id, <a href="../mys/transfer_policy.md#mys_transfer_policy_ENotOwner">ENotOwner</a>);
    <b>let</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a> { id: cap_id, policy_id } = cap;
    <b>let</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a> { id, <a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a>: _, <a href="../mys/balance.md#mys_balance">balance</a> } = self;
    id.delete();
    cap_id.delete();
    <a href="../mys/event.md#mys_event_emit">event::emit</a>(<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyDestroyed">TransferPolicyDestroyed</a>&lt;T&gt; { id: policy_id });
    <a href="../mys/balance.md#mys_balance">balance</a>.into_coin(ctx)
}
</code></pre>



</details>

<a name="mys_transfer_policy_confirm_request"></a>

## Function `confirm_request`

Allow a <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code> for the type <code>T</code>. The call is protected
by the type constraint, as only the publisher of the <code>T</code> can get
<code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;</code>.

Note: unless there's a policy for <code>T</code> to allow transfers,
Kiosk trades will not be possible.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_confirm_request">confirm_request</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;, request: <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">mys::transfer_policy::TransferRequest</a>&lt;T&gt;): (<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, u64, <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_confirm_request">confirm_request</a>&lt;T&gt;(
    self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;,
    request: <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a>&lt;T&gt;,
): (ID, u64, ID) {
    <b>let</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a> { <a href="../mys/transfer_policy.md#mys_transfer_policy_item">item</a>, <a href="../mys/transfer_policy.md#mys_transfer_policy_paid">paid</a>, <a href="../mys/transfer_policy.md#mys_transfer_policy_from">from</a>, receipts } = request;
    <b>let</b> <b>mut</b> completed = receipts.into_keys();
    <b>let</b> <b>mut</b> total = completed.length();
    <b>assert</b>!(total == self.<a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a>.size(), <a href="../mys/transfer_policy.md#mys_transfer_policy_EPolicyNotSatisfied">EPolicyNotSatisfied</a>);
    <b>while</b> (total &gt; 0) {
        <b>let</b> rule_type = completed.pop_back();
        <b>assert</b>!(self.<a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a>.contains(&rule_type), <a href="../mys/transfer_policy.md#mys_transfer_policy_EIllegalRule">EIllegalRule</a>);
        total = total - 1;
    };
    (<a href="../mys/transfer_policy.md#mys_transfer_policy_item">item</a>, <a href="../mys/transfer_policy.md#mys_transfer_policy_paid">paid</a>, <a href="../mys/transfer_policy.md#mys_transfer_policy_from">from</a>)
}
</code></pre>



</details>

<a name="mys_transfer_policy_add_rule"></a>

## Function `add_rule`

Add a custom Rule to the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>. Once set, <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code> must
receive a confirmation of the rule executed so the hot potato can be unpacked.

- T: the type to which TransferPolicy<T> is applied.
- Rule: the witness type for the Custom rule
- Config: a custom configuration for the rule

Config requires <code>drop</code> to allow creators to remove any policy at any moment,
even if graceful unpacking has not been implemented in a "rule module".


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_add_rule">add_rule</a>&lt;T, Rule: drop, Config: drop, store&gt;(_: Rule, policy: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;, cap: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">mys::transfer_policy::TransferPolicyCap</a>&lt;T&gt;, cfg: Config)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_add_rule">add_rule</a>&lt;T, Rule: drop, Config: store + drop&gt;(
    _: Rule,
    policy: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;,
    cap: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a>&lt;T&gt;,
    cfg: Config,
) {
    <b>assert</b>!(<a href="../mys/object.md#mys_object_id">object::id</a>(policy) == cap.policy_id, <a href="../mys/transfer_policy.md#mys_transfer_policy_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(!<a href="../mys/transfer_policy.md#mys_transfer_policy_has_rule">has_rule</a>&lt;T, Rule&gt;(policy), <a href="../mys/transfer_policy.md#mys_transfer_policy_ERuleAlreadySet">ERuleAlreadySet</a>);
    df::add(&<b>mut</b> policy.id, <a href="../mys/transfer_policy.md#mys_transfer_policy_RuleKey">RuleKey</a>&lt;Rule&gt; {}, cfg);
    policy.<a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a>.insert(type_name::get&lt;Rule&gt;())
}
</code></pre>



</details>

<a name="mys_transfer_policy_get_rule"></a>

## Function `get_rule`

Get the custom Config for the Rule (can be only one per "Rule" type).


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_get_rule">get_rule</a>&lt;T, Rule: drop, Config: drop, store&gt;(_: Rule, policy: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;): &Config
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_get_rule">get_rule</a>&lt;T, Rule: drop, Config: store + drop&gt;(
    _: Rule,
    policy: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;,
): &Config {
    df::borrow(&policy.id, <a href="../mys/transfer_policy.md#mys_transfer_policy_RuleKey">RuleKey</a>&lt;Rule&gt; {})
}
</code></pre>



</details>

<a name="mys_transfer_policy_add_to_balance"></a>

## Function `add_to_balance`

Add some <code>MYS</code> to the balance of a <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_add_to_balance">add_to_balance</a>&lt;T, Rule: drop&gt;(_: Rule, policy: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;, <a href="../mys/coin.md#mys_coin">coin</a>: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_add_to_balance">add_to_balance</a>&lt;T, Rule: drop&gt;(_: Rule, policy: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;, <a href="../mys/coin.md#mys_coin">coin</a>: Coin&lt;MYS&gt;) {
    <b>assert</b>!(<a href="../mys/transfer_policy.md#mys_transfer_policy_has_rule">has_rule</a>&lt;T, Rule&gt;(policy), <a href="../mys/transfer_policy.md#mys_transfer_policy_EUnknownRequirement">EUnknownRequirement</a>);
    <a href="../mys/coin.md#mys_coin_put">coin::put</a>(&<b>mut</b> policy.<a href="../mys/balance.md#mys_balance">balance</a>, <a href="../mys/coin.md#mys_coin">coin</a>)
}
</code></pre>



</details>

<a name="mys_transfer_policy_add_receipt"></a>

## Function `add_receipt`

Adds a <code>Receipt</code> to the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code>, unblocking the request and
confirming that the policy requirements are satisfied.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_add_receipt">add_receipt</a>&lt;T, Rule: drop&gt;(_: Rule, request: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">mys::transfer_policy::TransferRequest</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_add_receipt">add_receipt</a>&lt;T, Rule: drop&gt;(_: Rule, request: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a>&lt;T&gt;) {
    request.receipts.insert(type_name::get&lt;Rule&gt;())
}
</code></pre>



</details>

<a name="mys_transfer_policy_has_rule"></a>

## Function `has_rule`

Check whether a custom rule has been added to the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_has_rule">has_rule</a>&lt;T, Rule: drop&gt;(policy: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_has_rule">has_rule</a>&lt;T, Rule: drop&gt;(policy: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;): bool {
    df::exists_(&policy.id, <a href="../mys/transfer_policy.md#mys_transfer_policy_RuleKey">RuleKey</a>&lt;Rule&gt; {})
}
</code></pre>



</details>

<a name="mys_transfer_policy_remove_rule"></a>

## Function `remove_rule`

Remove the Rule from the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_remove_rule">remove_rule</a>&lt;T, Rule: drop, Config: drop, store&gt;(policy: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;, cap: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">mys::transfer_policy::TransferPolicyCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_remove_rule">remove_rule</a>&lt;T, Rule: drop, Config: store + drop&gt;(
    policy: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;,
    cap: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a>&lt;T&gt;,
) {
    <b>assert</b>!(<a href="../mys/object.md#mys_object_id">object::id</a>(policy) == cap.policy_id, <a href="../mys/transfer_policy.md#mys_transfer_policy_ENotOwner">ENotOwner</a>);
    <b>let</b> _: Config = df::remove(&<b>mut</b> policy.id, <a href="../mys/transfer_policy.md#mys_transfer_policy_RuleKey">RuleKey</a>&lt;Rule&gt; {});
    policy.<a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a>.remove(&type_name::get&lt;Rule&gt;());
}
</code></pre>



</details>

<a name="mys_transfer_policy_uid"></a>

## Function `uid`

Allows reading custom attachments to the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code> if there are any.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_uid">uid</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;): &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_uid">uid</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;): &UID { &self.id }
</code></pre>



</details>

<a name="mys_transfer_policy_uid_mut_as_owner"></a>

## Function `uid_mut_as_owner`

Get a mutable reference to the <code>self.id</code> to enable custom attachments
to the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_uid_mut_as_owner">uid_mut_as_owner</a>&lt;T&gt;(self: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;, cap: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">mys::transfer_policy::TransferPolicyCap</a>&lt;T&gt;): &<b>mut</b> <a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_uid_mut_as_owner">uid_mut_as_owner</a>&lt;T&gt;(self: &<b>mut</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;, cap: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicyCap">TransferPolicyCap</a>&lt;T&gt;): &<b>mut</b> UID {
    <b>assert</b>!(<a href="../mys/object.md#mys_object_id">object::id</a>(self) == cap.policy_id, <a href="../mys/transfer_policy.md#mys_transfer_policy_ENotOwner">ENotOwner</a>);
    &<b>mut</b> self.id
}
</code></pre>



</details>

<a name="mys_transfer_policy_rules"></a>

## Function `rules`

Read the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a></code> field from the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">mys::transfer_policy::TransferPolicy</a>&lt;T&gt;): &<a href="../mys/vec_set.md#mys_vec_set_VecSet">mys::vec_set::VecSet</a>&lt;<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferPolicy">TransferPolicy</a>&lt;T&gt;): &VecSet&lt;TypeName&gt; {
    &self.<a href="../mys/transfer_policy.md#mys_transfer_policy_rules">rules</a>
}
</code></pre>



</details>

<a name="mys_transfer_policy_item"></a>

## Function `item`

Get the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_item">item</a></code> field of the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_item">item</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">mys::transfer_policy::TransferRequest</a>&lt;T&gt;): <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_item">item</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a>&lt;T&gt;): ID { self.<a href="../mys/transfer_policy.md#mys_transfer_policy_item">item</a> }
</code></pre>



</details>

<a name="mys_transfer_policy_paid"></a>

## Function `paid`

Get the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_paid">paid</a></code> field of the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_paid">paid</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">mys::transfer_policy::TransferRequest</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_paid">paid</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a>&lt;T&gt;): u64 { self.<a href="../mys/transfer_policy.md#mys_transfer_policy_paid">paid</a> }
</code></pre>



</details>

<a name="mys_transfer_policy_from"></a>

## Function `from`

Get the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_from">from</a></code> field of the <code><a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_from">from</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">mys::transfer_policy::TransferRequest</a>&lt;T&gt;): <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/transfer_policy.md#mys_transfer_policy_from">from</a>&lt;T&gt;(self: &<a href="../mys/transfer_policy.md#mys_transfer_policy_TransferRequest">TransferRequest</a>&lt;T&gt;): ID { self.<a href="../mys/transfer_policy.md#mys_transfer_policy_from">from</a> }
</code></pre>



</details>
