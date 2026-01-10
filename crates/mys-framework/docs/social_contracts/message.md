---
title: Module `social_contracts::message`
---

On-chain messaging protocol for MySocial blockchain
Stores only message digests (hashes) while keeping full content off-chain
Features: idempotency, message ordering, replay protection, access control, rate limiting


-  [Struct `Registry`](#social_contracts_message_Registry)
-  [Struct `Conversation`](#social_contracts_message_Conversation)
-  [Struct `MessageDigest`](#social_contracts_message_MessageDigest)
-  [Struct `PaidMessageEscrow`](#social_contracts_message_PaidMessageEscrow)
-  [Struct `ConversationCreated`](#social_contracts_message_ConversationCreated)
-  [Struct `ParticipantsAdded`](#social_contracts_message_ParticipantsAdded)
-  [Struct `ParticipantsRemoved`](#social_contracts_message_ParticipantsRemoved)
-  [Struct `RoleChanged`](#social_contracts_message_RoleChanged)
-  [Struct `ConversationMetaSet`](#social_contracts_message_ConversationMetaSet)
-  [Struct `MessageSent`](#social_contracts_message_MessageSent)
-  [Struct `MessageEdited`](#social_contracts_message_MessageEdited)
-  [Struct `MessageTombstoned`](#social_contracts_message_MessageTombstoned)
-  [Struct `DeliveredAck`](#social_contracts_message_DeliveredAck)
-  [Struct `ReadAck`](#social_contracts_message_ReadAck)
-  [Struct `Reacted`](#social_contracts_message_Reacted)
-  [Struct `Pinned`](#social_contracts_message_Pinned)
-  [Struct `KeyRotated`](#social_contracts_message_KeyRotated)
-  [Struct `MemberKeySubmitted`](#social_contracts_message_MemberKeySubmitted)
-  [Struct `Moderation`](#social_contracts_message_Moderation)
-  [Struct `Paused`](#social_contracts_message_Paused)
-  [Struct `VersionSet`](#social_contracts_message_VersionSet)
-  [Struct `RelayerUpdated`](#social_contracts_message_RelayerUpdated)
-  [Struct `AdminTransferred`](#social_contracts_message_AdminTransferred)
-  [Struct `RateLimitsSet`](#social_contracts_message_RateLimitsSet)
-  [Struct `PaidMessageSent`](#social_contracts_message_PaidMessageSent)
-  [Struct `PaidMessageReplied`](#social_contracts_message_PaidMessageReplied)
-  [Struct `PaymentClaimed`](#social_contracts_message_PaymentClaimed)
-  [Struct `PaymentRefunded`](#social_contracts_message_PaymentRefunded)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_message_bootstrap_init)
-  [Function `set_relayer`](#social_contracts_message_set_relayer)
-  [Function `pause`](#social_contracts_message_pause)
-  [Function `set_version`](#social_contracts_message_set_version)
-  [Function `transfer_admin`](#social_contracts_message_transfer_admin)
-  [Function `create_conversation`](#social_contracts_message_create_conversation)
-  [Function `add_participants`](#social_contracts_message_add_participants)
-  [Function `remove_participants`](#social_contracts_message_remove_participants)
-  [Function `leave_conversation`](#social_contracts_message_leave_conversation)
-  [Function `set_role`](#social_contracts_message_set_role)
-  [Function `set_conv_meta`](#social_contracts_message_set_conv_meta)
-  [Function `use_nonce`](#social_contracts_message_use_nonce)
-  [Function `mark_dedupe`](#social_contracts_message_mark_dedupe)
-  [Function `send_message`](#social_contracts_message_send_message)
-  [Function `edit_message`](#social_contracts_message_edit_message)
-  [Function `tombstone_message`](#social_contracts_message_tombstone_message)
-  [Function `ack_delivery`](#social_contracts_message_ack_delivery)
-  [Function `ack_read`](#social_contracts_message_ack_read)
-  [Function `react`](#social_contracts_message_react)
-  [Function `pin`](#social_contracts_message_pin)
-  [Function `unpin`](#social_contracts_message_unpin)
-  [Function `rotate_conv_key`](#social_contracts_message_rotate_conv_key)
-  [Function `submit_member_key`](#social_contracts_message_submit_member_key)
-  [Function `mod_remove`](#social_contracts_message_mod_remove)
-  [Function `set_rate_limits`](#social_contracts_message_set_rate_limits)
-  [Function `export_range`](#social_contracts_message_export_range)
-  [Function `is_member`](#social_contracts_message_is_member)
-  [Function `next_seq`](#social_contracts_message_next_seq)
-  [Function `meta_hash`](#social_contracts_message_meta_hash)
-  [Function `get_role`](#social_contracts_message_get_role)
-  [Function `assert_role_at_least`](#social_contracts_message_assert_role_at_least)
-  [Function `enforce_rate_limits`](#social_contracts_message_enforce_rate_limits)
-  [Function `send_paid_message`](#social_contracts_message_send_paid_message)
-  [Function `reply_to_paid_message`](#social_contracts_message_reply_to_paid_message)
-  [Function `claim_payment_internal`](#social_contracts_message_claim_payment_internal)
-  [Function `refund_paid_message`](#social_contracts_message_refund_paid_message)


<pre><code><b>use</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption">mydata::bf_hmac_encryption</a>;
<b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr">mydata::hmac256ctr</a>;
<b>use</b> <a href="../mydata/kdf.md#mydata_kdf">mydata::kdf</a>;
<b>use</b> <a href="../mydata/polynomial.md#mydata_polynomial">mydata::polynomial</a>;
<b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bag.md#mys_bag">mys::bag</a>;
<b>use</b> <a href="../mys/balance.md#mys_balance">mys::balance</a>;
<b>use</b> <a href="../mys/bcs.md#mys_bcs">mys::bcs</a>;
<b>use</b> <a href="../mys/bls12381.md#mys_bls12381">mys::bls12381</a>;
<b>use</b> <a href="../mys/clock.md#mys_clock">mys::clock</a>;
<b>use</b> <a href="../mys/coin.md#mys_coin">mys::coin</a>;
<b>use</b> <a href="../mys/config.md#mys_config">mys::config</a>;
<b>use</b> <a href="../mys/deny_list.md#mys_deny_list">mys::deny_list</a>;
<b>use</b> <a href="../mys/dynamic_field.md#mys_dynamic_field">mys::dynamic_field</a>;
<b>use</b> <a href="../mys/dynamic_object_field.md#mys_dynamic_object_field">mys::dynamic_object_field</a>;
<b>use</b> <a href="../mys/event.md#mys_event">mys::event</a>;
<b>use</b> <a href="../mys/group_ops.md#mys_group_ops">mys::group_ops</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/hmac.md#mys_hmac">mys::hmac</a>;
<b>use</b> <a href="../mys/mys.md#mys_mys">mys::mys</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/package.md#mys_package">mys::package</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_map.md#mys_vec_map">mys::vec_map</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
<b>use</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">social_contracts::subscription</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/hash.md#std_hash">std::hash</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_message_Registry"></a>

## Struct `Registry`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a> <b>has</b> key
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
<code>admin: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>relayer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>paused: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>rl_window_secs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>rl_per_user_per_window: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>rl_per_conv_per_window: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_Conversation"></a>

## Struct `Conversation`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a> <b>has</b> key
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
<code>kind: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>roles: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>members: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>messages: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u64, <a href="../social_contracts/message.md#social_contracts_message_MessageDigest">social_contracts::message::MessageDigest</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>used_dedupe: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;vector&lt;u8&gt;, bool&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>nonces: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u128, bool&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>reactions: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u64, <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u32, u32&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>pins: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u64, bool&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>delivered: <a href="../mys/vec_map.md#mys_vec_map_VecMap">mys::vec_map::VecMap</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>read: <a href="../mys/vec_map.md#mys_vec_map_VecMap">mys::vec_map::VecMap</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>flags: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>rl_window_secs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>rl_per_user_per_window: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>rl_per_conv_per_window: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>rl_window_start: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>rl_conv_count: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>rl_user_counts: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, u32&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>rl_active_users: vector&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>paid_msg_escrow: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u64, <a href="../social_contracts/message.md#social_contracts_message_PaidMessageEscrow">social_contracts::message::PaidMessageEscrow</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_MessageDigest"></a>

## Struct `MessageDigest`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_MessageDigest">MessageDigest</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>kind: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>parent: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>digest_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>media_batch_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>key_ref: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>edit_seq: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>client_ts: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>server_ts: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>flags: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>char_count: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u32&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>is_paid: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_PaidMessageEscrow"></a>

## Struct `PaidMessageEscrow`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_PaidMessageEscrow">PaidMessageEscrow</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>payer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>escrowed_balance: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>created_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>parent_seq: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_ConversationCreated"></a>

## Struct `ConversationCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_ConversationCreated">ConversationCreated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>kind: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_ParticipantsAdded"></a>

## Struct `ParticipantsAdded`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_ParticipantsAdded">ParticipantsAdded</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>count: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_ParticipantsRemoved"></a>

## Struct `ParticipantsRemoved`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_ParticipantsRemoved">ParticipantsRemoved</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>count: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_RoleChanged"></a>

## Struct `RoleChanged`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_RoleChanged">RoleChanged</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>who: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>role: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_ConversationMetaSet"></a>

## Struct `ConversationMetaSet`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_ConversationMetaSet">ConversationMetaSet</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_MessageSent"></a>

## Struct `MessageSent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_MessageSent">MessageSent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>kind: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_MessageEdited"></a>

## Struct `MessageEdited`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_MessageEdited">MessageEdited</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>edit_seq: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_MessageTombstoned"></a>

## Struct `MessageTombstoned`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_MessageTombstoned">MessageTombstoned</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reason: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_DeliveredAck"></a>

## Struct `DeliveredAck`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_DeliveredAck">DeliveredAck</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>upto: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_ReadAck"></a>

## Struct `ReadAck`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_ReadAck">ReadAck</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>upto: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_Reacted"></a>

## Struct `Reacted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_Reacted">Reacted</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>code: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>op: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_Pinned"></a>

## Struct `Pinned`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_Pinned">Pinned</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>on: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_KeyRotated"></a>

## Struct `KeyRotated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_KeyRotated">KeyRotated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>version: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_MemberKeySubmitted"></a>

## Struct `MemberKeySubmitted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_MemberKeySubmitted">MemberKeySubmitted</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_Moderation"></a>

## Struct `Moderation`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_Moderation">Moderation</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reason: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_Paused"></a>

## Struct `Paused`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_Paused">Paused</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>on: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_VersionSet"></a>

## Struct `VersionSet`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_VersionSet">VersionSet</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>version: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_RelayerUpdated"></a>

## Struct `RelayerUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_RelayerUpdated">RelayerUpdated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>old_relayer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_relayer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_AdminTransferred"></a>

## Struct `AdminTransferred`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_AdminTransferred">AdminTransferred</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>old_admin: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_admin: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>transferred_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_RateLimitsSet"></a>

## Struct `RateLimitsSet`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_RateLimitsSet">RateLimitsSet</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>window_secs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>per_user: u32</code>
</dt>
<dd>
</dd>
<dt>
<code>per_conv: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_PaidMessageSent"></a>

## Struct `PaidMessageSent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_PaidMessageSent">PaidMessageSent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>payer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>created_epoch: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_PaidMessageReplied"></a>

## Struct `PaidMessageReplied`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_PaidMessageReplied">PaidMessageReplied</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>paid_msg_seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reply_seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reply_char_count: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_PaymentClaimed"></a>

## Struct `PaymentClaimed`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_PaymentClaimed">PaymentClaimed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>net_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_epoch: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_message_PaymentRefunded"></a>

## Struct `PaymentRefunded`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/message.md#social_contracts_message_PaymentRefunded">PaymentRefunded</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conv: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>payer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>refunded_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reason: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_message_CONV_CHANNEL"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_CONV_CHANNEL">CONV_CHANNEL</a>: u8 = 2;
</code></pre>



<a name="social_contracts_message_CONV_DM"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_CONV_DM">CONV_DM</a>: u8 = 0;
</code></pre>



<a name="social_contracts_message_CONV_GROUP"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_CONV_GROUP">CONV_GROUP</a>: u8 = 1;
</code></pre>



<a name="social_contracts_message_E_ALREADY_MEMBER"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_ALREADY_MEMBER">E_ALREADY_MEMBER</a>: u64 = 4;
</code></pre>



<a name="social_contracts_message_E_BAD_SEQ"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_BAD_SEQ">E_BAD_SEQ</a>: u64 = 7;
</code></pre>



<a name="social_contracts_message_E_DEDUPE_USED"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_DEDUPE_USED">E_DEDUPE_USED</a>: u64 = 5;
</code></pre>



<a name="social_contracts_message_E_FORBIDDEN"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_FORBIDDEN">E_FORBIDDEN</a>: u64 = 11;
</code></pre>



<a name="social_contracts_message_E_INSUFFICIENT_PAYMENT"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_INSUFFICIENT_PAYMENT">E_INSUFFICIENT_PAYMENT</a>: u64 = 13;
</code></pre>



<a name="social_contracts_message_E_NONCE_USED"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_NONCE_USED">E_NONCE_USED</a>: u64 = 6;
</code></pre>



<a name="social_contracts_message_E_NOT_ADMIN"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_NOT_ADMIN">E_NOT_ADMIN</a>: u64 = 3;
</code></pre>



<a name="social_contracts_message_E_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_NOT_FOUND">E_NOT_FOUND</a>: u64 = 9;
</code></pre>



<a name="social_contracts_message_E_NOT_MEMBER"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>: u64 = 2;
</code></pre>



<a name="social_contracts_message_E_NOT_PAID_MESSAGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_NOT_PAID_MESSAGE">E_NOT_PAID_MESSAGE</a>: u64 = 14;
</code></pre>



<a name="social_contracts_message_E_NO_MSG"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_NO_MSG">E_NO_MSG</a>: u64 = 8;
</code></pre>



<a name="social_contracts_message_E_PAUSED"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_PAUSED">E_PAUSED</a>: u64 = 1;
</code></pre>



<a name="social_contracts_message_E_PAYMENT_ALREADY_CLAIMED"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_PAYMENT_ALREADY_CLAIMED">E_PAYMENT_ALREADY_CLAIMED</a>: u64 = 16;
</code></pre>



<a name="social_contracts_message_E_PAYMENT_EXPIRED"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_PAYMENT_EXPIRED">E_PAYMENT_EXPIRED</a>: u64 = 15;
</code></pre>



<a name="social_contracts_message_E_RATE_LIMIT"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_RATE_LIMIT">E_RATE_LIMIT</a>: u64 = 12;
</code></pre>



<a name="social_contracts_message_E_REPLY_TOO_SHORT"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_REPLY_TOO_SHORT">E_REPLY_TOO_SHORT</a>: u64 = 17;
</code></pre>



<a name="social_contracts_message_E_ROLE_INVALID"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_ROLE_INVALID">E_ROLE_INVALID</a>: u64 = 10;
</code></pre>



<a name="social_contracts_message_E_WRONG_VERSION"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_E_WRONG_VERSION">E_WRONG_VERSION</a>: u64 = 18;
</code></pre>



<a name="social_contracts_message_MIN_REPLY_CHARS"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_MIN_REPLY_CHARS">MIN_REPLY_CHARS</a>: u32 = 6;
</code></pre>



<a name="social_contracts_message_MSG_REPLY"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_MSG_REPLY">MSG_REPLY</a>: u8 = 1;
</code></pre>



<a name="social_contracts_message_MSG_SYSTEM"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_MSG_SYSTEM">MSG_SYSTEM</a>: u8 = 2;
</code></pre>



<a name="social_contracts_message_MSG_TEXT"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_MSG_TEXT">MSG_TEXT</a>: u8 = 0;
</code></pre>



<a name="social_contracts_message_PAID_MSG_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_PAID_MSG_PLATFORM_FEE_BPS">PAID_MSG_PLATFORM_FEE_BPS</a>: u64 = 250;
</code></pre>



<a name="social_contracts_message_PAID_MSG_TOTAL_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_PAID_MSG_TOTAL_FEE_BPS">PAID_MSG_TOTAL_FEE_BPS</a>: u64 = 500;
</code></pre>



<a name="social_contracts_message_PAID_MSG_TREASURY_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_PAID_MSG_TREASURY_FEE_BPS">PAID_MSG_TREASURY_FEE_BPS</a>: u64 = 250;
</code></pre>



<a name="social_contracts_message_PAYMENT_EXPIRATION_EPOCHS"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_PAYMENT_EXPIRATION_EPOCHS">PAYMENT_EXPIRATION_EPOCHS</a>: u64 = 30;
</code></pre>



<a name="social_contracts_message_ROLE_ADMIN"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_ROLE_ADMIN">ROLE_ADMIN</a>: u8 = 2;
</code></pre>



<a name="social_contracts_message_ROLE_MEMBER"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_ROLE_MEMBER">ROLE_MEMBER</a>: u8 = 0;
</code></pre>



<a name="social_contracts_message_ROLE_MOD"></a>



<pre><code><b>const</b> <a href="../social_contracts/message.md#social_contracts_message_ROLE_MOD">ROLE_MOD</a>: u8 = 1;
</code></pre>



<a name="social_contracts_message_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization - creates the messaging registry (called during genesis)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> admin = tx_context::sender(ctx);
    <b>let</b> registry = <a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a> {
        id: object::new(ctx),
        admin,
        relayer: admin, // Default to admin, can be changed later
        paused: <b>false</b>,
        version: 1,
        // Default rate limits (can be changed by admin)
        rl_window_secs: 60,          // 60 second windows
        rl_per_user_per_window: 10,  // 10 messages per user per window
        rl_per_conv_per_window: 100, // 100 messages per conversation per window
    };
    transfer::share_object(registry);
}
</code></pre>



</details>

<a name="social_contracts_message_set_relayer"></a>

## Function `set_relayer`

Update relayer address (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_set_relayer">set_relayer</a>(registry: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Registry">social_contracts::message::Registry</a>, new_relayer: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_set_relayer">set_relayer</a>(
    registry: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>,
    new_relayer: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == registry.admin, <a href="../social_contracts/message.md#social_contracts_message_E_NOT_ADMIN">E_NOT_ADMIN</a>);
    <b>let</b> old_relayer = registry.relayer;
    registry.relayer = new_relayer;
    // Emit relayer updated event
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_RelayerUpdated">RelayerUpdated</a> {
        old_relayer,
        new_relayer,
        updated_by: tx_context::sender(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_message_pause"></a>

## Function `pause`

Pause/unpause the protocol (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_pause">pause</a>(registry: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Registry">social_contracts::message::Registry</a>, on: bool, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_pause">pause</a>(
    registry: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>,
    on: bool,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == registry.admin, <a href="../social_contracts/message.md#social_contracts_message_E_NOT_ADMIN">E_NOT_ADMIN</a>);
    registry.paused = on;
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_Paused">Paused</a> { on });
}
</code></pre>



</details>

<a name="social_contracts_message_set_version"></a>

## Function `set_version`

Update protocol version (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_set_version">set_version</a>(registry: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Registry">social_contracts::message::Registry</a>, version: u32, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_set_version">set_version</a>(
    registry: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>,
    version: u32,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tx_context::sender(ctx) == registry.admin, <a href="../social_contracts/message.md#social_contracts_message_E_NOT_ADMIN">E_NOT_ADMIN</a>);
    registry.version = version;
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_VersionSet">VersionSet</a> { version });
}
</code></pre>



</details>

<a name="social_contracts_message_transfer_admin"></a>

## Function `transfer_admin`

Transfer admin privileges (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_transfer_admin">transfer_admin</a>(registry: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Registry">social_contracts::message::Registry</a>, new_admin: <b>address</b>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_transfer_admin">transfer_admin</a>(
    registry: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>,
    new_admin: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility (<a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a> uses u32, <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> uses u64)
    <b>assert</b>!((registry.version <b>as</b> u64) == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/message.md#social_contracts_message_E_WRONG_VERSION">E_WRONG_VERSION</a>);
    <b>assert</b>!(tx_context::sender(ctx) == registry.admin, <a href="../social_contracts/message.md#social_contracts_message_E_NOT_ADMIN">E_NOT_ADMIN</a>);
    <b>let</b> old_admin = registry.admin;
    registry.admin = new_admin;
    // Emit admin transferred event
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_AdminTransferred">AdminTransferred</a> {
        old_admin,
        new_admin,
        transferred_by: tx_context::sender(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_message_create_conversation"></a>

## Function `create_conversation`

Create a new conversation (shared object, publicly accessible)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_create_conversation">create_conversation</a>(registry: &<a href="../social_contracts/message.md#social_contracts_message_Registry">social_contracts::message::Registry</a>, kind: u8, <a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_create_conversation">create_conversation</a>(
    registry: &<a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>,
    kind: u8,
    <a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(!registry.paused, <a href="../social_contracts/message.md#social_contracts_message_E_PAUSED">E_PAUSED</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>let</b> <b>mut</b> conv = <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a> {
        id: object::new(ctx),
        kind,
        <a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a>: 1,
        <a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>,
        roles: table::new(ctx),
        members: table::new(ctx),
        messages: table::new(ctx),
        used_dedupe: table::new(ctx),
        nonces: table::new(ctx),
        reactions: table::new(ctx),
        pins: table::new(ctx),
        delivered: vec_map::empty(),
        read: vec_map::empty(),
        flags: 0,
        rl_window_secs: 0, // Inherit from <a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>
        rl_per_user_per_window: 0,
        rl_per_conv_per_window: 0,
        rl_window_start: 0,
        rl_conv_count: 0,
        rl_user_counts: table::new(ctx),
        rl_active_users: vector::empty(),
        paid_msg_escrow: table::new(ctx),
    };
    <b>let</b> conv_addr = object::uid_to_address(&conv.id);
    // Add creator <b>as</b> admin
    table::add(&<b>mut</b> conv.roles, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_ADMIN">ROLE_ADMIN</a>);
    table::add(&<b>mut</b> conv.members, sender, <b>true</b>);
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_ConversationCreated">ConversationCreated</a> {
        conv: conv_addr,
        kind,
        creator: sender,
    });
    transfer::share_object(conv);
}
</code></pre>



</details>

<a name="social_contracts_message_add_participants"></a>

## Function `add_participants`

Add participants to conversation (admin/mod only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_add_participants">add_participants</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, participants: vector&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_add_participants">add_participants</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    participants: vector&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_MOD">ROLE_MOD</a>);
    <b>let</b> len = vector::length(&participants);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> added = 0u32;
    <b>while</b> (i &lt; len) {
        <b>let</b> addr = *vector::borrow(&participants, i);
        <b>if</b> (!table::contains(&conv.members, addr)) {
            table::add(&<b>mut</b> conv.roles, addr, <a href="../social_contracts/message.md#social_contracts_message_ROLE_MEMBER">ROLE_MEMBER</a>);
            table::add(&<b>mut</b> conv.members, addr, <b>true</b>);
            added = added + 1;
        };
        i = i + 1;
    };
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_ParticipantsAdded">ParticipantsAdded</a> {
        conv: object::uid_to_address(&conv.id),
        count: added,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_remove_participants"></a>

## Function `remove_participants`

Remove participants from conversation (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_remove_participants">remove_participants</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, participants: vector&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_remove_participants">remove_participants</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    participants: vector&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_ADMIN">ROLE_ADMIN</a>);
    <b>let</b> len = vector::length(&participants);
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> removed = 0u32;
    <b>while</b> (i &lt; len) {
        <b>let</b> addr = *vector::borrow(&participants, i);
        <b>if</b> (table::contains(&conv.members, addr)) {
            table::remove(&<b>mut</b> conv.roles, addr);
            table::remove(&<b>mut</b> conv.members, addr);
            removed = removed + 1;
        };
        i = i + 1;
    };
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_ParticipantsRemoved">ParticipantsRemoved</a> {
        conv: object::uid_to_address(&conv.id),
        count: removed,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_leave_conversation"></a>

## Function `leave_conversation`

Leave a conversation (self-removal)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_leave_conversation">leave_conversation</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_leave_conversation">leave_conversation</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&conv.members, sender), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>);
    table::remove(&<b>mut</b> conv.roles, sender);
    table::remove(&<b>mut</b> conv.members, sender);
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_ParticipantsRemoved">ParticipantsRemoved</a> {
        conv: object::uid_to_address(&conv.id),
        count: 1,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_set_role"></a>

## Function `set_role`

Set member role (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_set_role">set_role</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, member: <b>address</b>, role: u8, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_set_role">set_role</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    member: <b>address</b>,
    role: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_ADMIN">ROLE_ADMIN</a>);
    <b>assert</b>!(table::contains(&conv.members, member), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>);
    <b>assert</b>!(role &lt;= <a href="../social_contracts/message.md#social_contracts_message_ROLE_ADMIN">ROLE_ADMIN</a>, <a href="../social_contracts/message.md#social_contracts_message_E_ROLE_INVALID">E_ROLE_INVALID</a>);
    *table::borrow_mut(&<b>mut</b> conv.roles, member) = role;
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_RoleChanged">RoleChanged</a> {
        conv: object::uid_to_address(&conv.id),
        who: member,
        role,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_set_conv_meta"></a>

## Function `set_conv_meta`

Update conversation metadata hash (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_set_conv_meta">set_conv_meta</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, <a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_set_conv_meta">set_conv_meta</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    <a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_ADMIN">ROLE_ADMIN</a>);
    conv.<a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a> = <a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>;
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_ConversationMetaSet">ConversationMetaSet</a> {
        conv: object::uid_to_address(&conv.id),
        <a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_use_nonce"></a>

## Function `use_nonce`

Pre-register a nonce for batch operations


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_use_nonce">use_nonce</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, nonce: u128, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_use_nonce">use_nonce</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    nonce: u128,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&conv.members, sender), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>);
    // Initialize per-member nonce table <b>if</b> needed
    <b>if</b> (!table::contains(&conv.nonces, sender)) {
        table::add(&<b>mut</b> conv.nonces, sender, table::new(ctx));
    };
    <b>let</b> member_nonces = table::borrow_mut(&<b>mut</b> conv.nonces, sender);
    <b>assert</b>!(!table::contains(member_nonces, nonce), <a href="../social_contracts/message.md#social_contracts_message_E_NONCE_USED">E_NONCE_USED</a>);
    table::add(member_nonces, nonce, <b>true</b>);
}
</code></pre>



</details>

<a name="social_contracts_message_mark_dedupe"></a>

## Function `mark_dedupe`

Pre-mark dedupe key as used


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_mark_dedupe">mark_dedupe</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, dedupe_key: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_mark_dedupe">mark_dedupe</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    dedupe_key: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_MOD">ROLE_MOD</a>);
    <b>assert</b>!(!table::contains(&conv.used_dedupe, dedupe_key), <a href="../social_contracts/message.md#social_contracts_message_E_DEDUPE_USED">E_DEDUPE_USED</a>);
    table::add(&<b>mut</b> conv.used_dedupe, dedupe_key, <b>true</b>);
}
</code></pre>



</details>

<a name="social_contracts_message_send_message"></a>

## Function `send_message`

Send a new message with rate limit enforcement


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_send_message">send_message</a>(registry: &<a href="../social_contracts/message.md#social_contracts_message_Registry">social_contracts::message::Registry</a>, conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, kind: u8, parent: u64, digest_hash: vector&lt;u8&gt;, media_batch_hash: vector&lt;u8&gt;, key_ref: vector&lt;u8&gt;, client_ts: u64, dedupe_key: vector&lt;u8&gt;, nonce: u128, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_send_message">send_message</a>(
    registry: &<a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>,
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    kind: u8,
    parent: u64,
    digest_hash: vector&lt;u8&gt;,
    media_batch_hash: vector&lt;u8&gt;,
    key_ref: vector&lt;u8&gt;,
    client_ts: u64,
    dedupe_key: vector&lt;u8&gt;,
    nonce: u128,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(!registry.paused, <a href="../social_contracts/message.md#social_contracts_message_E_PAUSED">E_PAUSED</a>);
    // Note: <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a> doesn't have version field - this would require structural change
    // For now, we rely on <a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a> version check which is checked at creation
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&conv.members, sender), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>);
    // Check dedupe
    <b>assert</b>!(!table::contains(&conv.used_dedupe, dedupe_key), <a href="../social_contracts/message.md#social_contracts_message_E_DEDUPE_USED">E_DEDUPE_USED</a>);
    table::add(&<b>mut</b> conv.used_dedupe, dedupe_key, <b>true</b>);
    // Check nonce
    <b>if</b> (!table::contains(&conv.nonces, sender)) {
        table::add(&<b>mut</b> conv.nonces, sender, table::new(ctx));
    };
    <b>let</b> member_nonces = table::borrow_mut(&<b>mut</b> conv.nonces, sender);
    <b>assert</b>!(!table::contains(member_nonces, nonce), <a href="../social_contracts/message.md#social_contracts_message_E_NONCE_USED">E_NONCE_USED</a>);
    table::add(member_nonces, nonce, <b>true</b>);
    // Enforce rate limits with tumbling window
    <a href="../social_contracts/message.md#social_contracts_message_enforce_rate_limits">enforce_rate_limits</a>(registry, conv, sender, clock);
    // Assign sequence number
    <b>let</b> seq = conv.<a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a>;
    conv.<a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a> = seq + 1;
    // Create <a href="../social_contracts/message.md#social_contracts_message">message</a> digest
    <b>let</b> msg = <a href="../social_contracts/message.md#social_contracts_message_MessageDigest">MessageDigest</a> {
        seq,
        sender,
        kind,
        parent,
        digest_hash,
        media_batch_hash,
        key_ref,
        edit_seq: 0,
        client_ts,
        server_ts: clock::timestamp_ms(clock) / 1000,
        flags: 0,
        char_count: option::none(),
        is_paid: <b>false</b>,
    };
    table::add(&<b>mut</b> conv.messages, seq, msg);
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_MessageSent">MessageSent</a> {
        conv: object::uid_to_address(&conv.id),
        seq,
        sender,
        kind,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_edit_message"></a>

## Function `edit_message`

Edit an existing message (sender only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_edit_message">edit_message</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, seq: u64, new_digest_hash: vector&lt;u8&gt;, new_media_batch_hash: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_edit_message">edit_message</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    seq: u64,
    new_digest_hash: vector&lt;u8&gt;,
    new_media_batch_hash: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&conv.messages, seq), <a href="../social_contracts/message.md#social_contracts_message_E_NO_MSG">E_NO_MSG</a>);
    <b>let</b> msg = table::borrow_mut(&<b>mut</b> conv.messages, seq);
    <b>assert</b>!(msg.sender == sender, <a href="../social_contracts/message.md#social_contracts_message_E_FORBIDDEN">E_FORBIDDEN</a>);
    msg.digest_hash = new_digest_hash;
    msg.media_batch_hash = new_media_batch_hash;
    msg.edit_seq = msg.edit_seq + 1;
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_MessageEdited">MessageEdited</a> {
        conv: object::uid_to_address(&conv.id),
        seq,
        edit_seq: msg.edit_seq,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_tombstone_message"></a>

## Function `tombstone_message`

Tombstone (soft-delete) a message (sender, mod, or admin)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_tombstone_message">tombstone_message</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, seq: u64, reason: u8, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_tombstone_message">tombstone_message</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    seq: u64,
    reason: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&conv.messages, seq), <a href="../social_contracts/message.md#social_contracts_message_E_NO_MSG">E_NO_MSG</a>);
    <b>let</b> msg = table::borrow(&conv.messages, seq);
    <b>let</b> is_owner = msg.sender == sender;
    <b>let</b> is_mod_or_admin = table::contains(&conv.roles, sender) &&
        *table::borrow(&conv.roles, sender) &gt;= <a href="../social_contracts/message.md#social_contracts_message_ROLE_MOD">ROLE_MOD</a>;
    <b>assert</b>!(is_owner || is_mod_or_admin, <a href="../social_contracts/message.md#social_contracts_message_E_FORBIDDEN">E_FORBIDDEN</a>);
    // Mark <b>as</b> tombstoned (set digest to empty)
    <b>let</b> msg_mut = table::borrow_mut(&<b>mut</b> conv.messages, seq);
    msg_mut.digest_hash = vector::empty();
    msg_mut.media_batch_hash = vector::empty();
    msg_mut.flags = msg_mut.flags | 1; // Set tombstone flag
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_MessageTombstoned">MessageTombstoned</a> {
        conv: object::uid_to_address(&conv.id),
        seq,
        reason,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_ack_delivery"></a>

## Function `ack_delivery`

Acknowledge delivery up to a sequence number


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_ack_delivery">ack_delivery</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, upto: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_ack_delivery">ack_delivery</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    upto: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&conv.members, sender), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>);
    <b>if</b> (vec_map::contains(&conv.delivered, &sender)) {
        <b>let</b> (_key, current) = vec_map::remove(&<b>mut</b> conv.delivered, &sender);
        <b>if</b> (upto &gt; current) {
            vec_map::insert(&<b>mut</b> conv.delivered, sender, upto);
        } <b>else</b> {
            vec_map::insert(&<b>mut</b> conv.delivered, sender, current);
        };
    } <b>else</b> {
        vec_map::insert(&<b>mut</b> conv.delivered, sender, upto);
    };
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_DeliveredAck">DeliveredAck</a> {
        conv: object::uid_to_address(&conv.id),
        member: sender,
        upto,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_ack_read"></a>

## Function `ack_read`

Acknowledge read up to a sequence number


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_ack_read">ack_read</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, upto: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_ack_read">ack_read</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    upto: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&conv.members, sender), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>);
    <b>if</b> (vec_map::contains(&conv.read, &sender)) {
        <b>let</b> (_key, current) = vec_map::remove(&<b>mut</b> conv.read, &sender);
        <b>if</b> (upto &gt; current) {
            vec_map::insert(&<b>mut</b> conv.read, sender, upto);
        } <b>else</b> {
            vec_map::insert(&<b>mut</b> conv.read, sender, current);
        };
    } <b>else</b> {
        vec_map::insert(&<b>mut</b> conv.read, sender, upto);
    };
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_ReadAck">ReadAck</a> {
        conv: object::uid_to_address(&conv.id),
        member: sender,
        upto,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_react"></a>

## Function `react`

Add or remove a reaction to a message


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_react">react</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, seq: u64, emoji_code: u32, add: bool, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_react">react</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    seq: u64,
    emoji_code: u32,
    add: bool,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&conv.members, sender), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>);
    <b>assert</b>!(table::contains(&conv.messages, seq), <a href="../social_contracts/message.md#social_contracts_message_E_NO_MSG">E_NO_MSG</a>);
    // Initialize reaction table <b>for</b> this <a href="../social_contracts/message.md#social_contracts_message">message</a> <b>if</b> needed
    <b>if</b> (!table::contains(&conv.reactions, seq)) {
        table::add(&<b>mut</b> conv.reactions, seq, table::new(ctx));
    };
    <b>let</b> msg_reactions = table::borrow_mut(&<b>mut</b> conv.reactions, seq);
    <b>if</b> (add) {
        // Add reaction
        <b>if</b> (table::contains(msg_reactions, emoji_code)) {
            <b>let</b> count = table::borrow_mut(msg_reactions, emoji_code);
            *count = *count + 1;
        } <b>else</b> {
            table::add(msg_reactions, emoji_code, 1);
        };
    } <b>else</b> {
        // Remove reaction
        <b>if</b> (table::contains(msg_reactions, emoji_code)) {
            <b>let</b> count = table::borrow_mut(msg_reactions, emoji_code);
            <b>if</b> (*count &gt; 1) {
                *count = *count - 1;
            } <b>else</b> {
                table::remove(msg_reactions, emoji_code);
            };
        };
    };
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_Reacted">Reacted</a> {
        conv: object::uid_to_address(&conv.id),
        seq,
        member: sender,
        code: emoji_code,
        op: <b>if</b> (add) 1 <b>else</b> 0,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_pin"></a>

## Function `pin`

Pin a message (mod/admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_pin">pin</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, seq: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_pin">pin</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    seq: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_MOD">ROLE_MOD</a>);
    <b>assert</b>!(table::contains(&conv.messages, seq), <a href="../social_contracts/message.md#social_contracts_message_E_NO_MSG">E_NO_MSG</a>);
    <b>if</b> (!table::contains(&conv.pins, seq)) {
        table::add(&<b>mut</b> conv.pins, seq, <b>true</b>);
    };
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_Pinned">Pinned</a> {
        conv: object::uid_to_address(&conv.id),
        seq,
        on: <b>true</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_unpin"></a>

## Function `unpin`

Unpin a message (mod/admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_unpin">unpin</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, seq: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_unpin">unpin</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    seq: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_MOD">ROLE_MOD</a>);
    <b>if</b> (table::contains(&conv.pins, seq)) {
        table::remove(&<b>mut</b> conv.pins, seq);
    };
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_Pinned">Pinned</a> {
        conv: object::uid_to_address(&conv.id),
        seq,
        on: <b>false</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_rotate_conv_key"></a>

## Function `rotate_conv_key`

Rotate conversation encryption key (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_rotate_conv_key">rotate_conv_key</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, _new_key_hash: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_rotate_conv_key">rotate_conv_key</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    _new_key_hash: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_ADMIN">ROLE_ADMIN</a>);
    // Increment version (stored in flags upper bits)
    <b>let</b> version = ((conv.flags &gt;&gt; 32) + 1) <b>as</b> u32;
    conv.flags = (conv.flags & 0xFFFFFFFF) | ((version <b>as</b> u64) &lt;&lt; 32);
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_KeyRotated">KeyRotated</a> {
        conv: object::uid_to_address(&conv.id),
        version,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_submit_member_key"></a>

## Function `submit_member_key`

Submit member-specific key bundle (member only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_submit_member_key">submit_member_key</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, _key_bundle_hash: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_submit_member_key">submit_member_key</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    _key_bundle_hash: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&conv.members, sender), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>);
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_MemberKeySubmitted">MemberKeySubmitted</a> {
        conv: object::uid_to_address(&conv.id),
        member: sender,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_mod_remove"></a>

## Function `mod_remove`

Moderator removes a message


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_mod_remove">mod_remove</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, seq: u64, reason: u8, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_mod_remove">mod_remove</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    seq: u64,
    reason: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_MOD">ROLE_MOD</a>);
    <b>assert</b>!(table::contains(&conv.messages, seq), <a href="../social_contracts/message.md#social_contracts_message_E_NO_MSG">E_NO_MSG</a>);
    // Tombstone the <a href="../social_contracts/message.md#social_contracts_message">message</a>
    <b>let</b> msg = table::borrow_mut(&<b>mut</b> conv.messages, seq);
    msg.digest_hash = vector::empty();
    msg.media_batch_hash = vector::empty();
    msg.flags = msg.flags | 1; // Set tombstone flag
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_Moderation">Moderation</a> {
        conv: object::uid_to_address(&conv.id),
        seq,
        reason,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_set_rate_limits"></a>

## Function `set_rate_limits`

Set rate limits for a conversation (admin/mod only)
0 values inherit from Registry defaults


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_set_rate_limits">set_rate_limits</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, window_secs: u64, per_user: u32, per_conv: u32, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_set_rate_limits">set_rate_limits</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    window_secs: u64,
    per_user: u32,
    per_conv: u32,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv, sender, <a href="../social_contracts/message.md#social_contracts_message_ROLE_MOD">ROLE_MOD</a>);
    conv.rl_window_secs = window_secs;
    conv.rl_per_user_per_window = per_user;
    conv.rl_per_conv_per_window = per_conv;
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_RateLimitsSet">RateLimitsSet</a> {
        conv: object::uid_to_address(&conv.id),
        window_secs,
        per_user,
        per_conv,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_export_range"></a>

## Function `export_range`

Export a range of messages for pagination


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_export_range">export_range</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, from_seq: u64, limit: u16): vector&lt;<a href="../social_contracts/message.md#social_contracts_message_MessageDigest">social_contracts::message::MessageDigest</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_export_range">export_range</a>(
    conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    from_seq: u64,
    limit: u16
): vector&lt;<a href="../social_contracts/message.md#social_contracts_message_MessageDigest">MessageDigest</a>&gt; {
    <b>let</b> <b>mut</b> result = vector::empty&lt;<a href="../social_contracts/message.md#social_contracts_message_MessageDigest">MessageDigest</a>&gt;();
    <b>let</b> <b>mut</b> seq = from_seq;
    <b>let</b> end_seq = from_seq + (limit <b>as</b> u64);
    <b>let</b> max_seq = conv.<a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a>;
    <b>while</b> (seq &lt; end_seq && seq &lt; max_seq) {
        <b>if</b> (table::contains(&conv.messages, seq)) {
            <b>let</b> msg = *table::borrow(&conv.messages, seq);
            vector::push_back(&<b>mut</b> result, msg);
        };
        seq = seq + 1;
    };
    result
}
</code></pre>



</details>

<a name="social_contracts_message_is_member"></a>

## Function `is_member`

Check if an address is a member


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_is_member">is_member</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, who: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_is_member">is_member</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>, who: <b>address</b>): bool {
    table::contains(&conv.members, who)
}
</code></pre>



</details>

<a name="social_contracts_message_next_seq"></a>

## Function `next_seq`

Get the next sequence number


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>): u64 {
    conv.<a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a>
}
</code></pre>



</details>

<a name="social_contracts_message_meta_hash"></a>

## Function `meta_hash`

Get conversation metadata hash


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>): vector&lt;u8&gt; {
    conv.<a href="../social_contracts/message.md#social_contracts_message_meta_hash">meta_hash</a>
}
</code></pre>



</details>

<a name="social_contracts_message_get_role"></a>

## Function `get_role`

Get member role (or ROLE_MEMBER if not found)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_get_role">get_role</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, who: <b>address</b>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_get_role">get_role</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>, who: <b>address</b>): u8 {
    <b>if</b> (table::contains(&conv.roles, who)) {
        *table::borrow(&conv.roles, who)
    } <b>else</b> {
        <a href="../social_contracts/message.md#social_contracts_message_ROLE_MEMBER">ROLE_MEMBER</a>
    }
}
</code></pre>



</details>

<a name="social_contracts_message_assert_role_at_least"></a>

## Function `assert_role_at_least`

Assert member has at least the specified role


<pre><code><b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, who: <b>address</b>, min_role: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_assert_role_at_least">assert_role_at_least</a>(conv: &<a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>, who: <b>address</b>, min_role: u8) {
    <b>assert</b>!(table::contains(&conv.roles, who), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>);
    <b>let</b> role = *table::borrow(&conv.roles, who);
    <b>assert</b>!(role &gt;= min_role, <a href="../social_contracts/message.md#social_contracts_message_E_FORBIDDEN">E_FORBIDDEN</a>);
}
</code></pre>



</details>

<a name="social_contracts_message_enforce_rate_limits"></a>

## Function `enforce_rate_limits`

Enforce rate limits with tumbling window


<pre><code><b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_enforce_rate_limits">enforce_rate_limits</a>(registry: &<a href="../social_contracts/message.md#social_contracts_message_Registry">social_contracts::message::Registry</a>, conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, sender: <b>address</b>, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_enforce_rate_limits">enforce_rate_limits</a>(
    registry: &<a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>,
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    sender: <b>address</b>,
    clock: &Clock
) {
    <b>let</b> now = clock::timestamp_ms(clock) / 1000; // Convert to seconds
    // Determine effective limits (0 = inherit from <a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>)
    <b>let</b> window_secs = <b>if</b> (conv.rl_window_secs == 0) {
        registry.rl_window_secs
    } <b>else</b> {
        conv.rl_window_secs
    };
    <b>let</b> per_user_limit = <b>if</b> (conv.rl_per_user_per_window == 0) {
        registry.rl_per_user_per_window
    } <b>else</b> {
        conv.rl_per_user_per_window
    };
    <b>let</b> per_conv_limit = <b>if</b> (conv.rl_per_conv_per_window == 0) {
        registry.rl_per_conv_per_window
    } <b>else</b> {
        conv.rl_per_conv_per_window
    };
    // Check <b>if</b> we need to reset the window
    <b>if</b> (now - conv.rl_window_start &gt;= window_secs) {
        // Reset window
        conv.rl_window_start = now;
        conv.rl_conv_count = 0;
        // Clear all user counts using tracked active users
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&conv.rl_active_users);
        <b>while</b> (i &lt; len) {
            <b>let</b> user = *vector::borrow(&conv.rl_active_users, i);
            <b>if</b> (table::contains(&conv.rl_user_counts, user)) {
                table::remove(&<b>mut</b> conv.rl_user_counts, user);
            };
            i = i + 1;
        };
        // Clear the active users list
        conv.rl_active_users = vector::empty();
    };
    // Check conversation limit
    <b>assert</b>!(conv.rl_conv_count + 1 &lt;= per_conv_limit, <a href="../social_contracts/message.md#social_contracts_message_E_RATE_LIMIT">E_RATE_LIMIT</a>);
    // Check per-user limit
    <b>let</b> user_count = <b>if</b> (table::contains(&conv.rl_user_counts, sender)) {
        *table::borrow(&conv.rl_user_counts, sender)
    } <b>else</b> {
        0
    };
    <b>assert</b>!(user_count + 1 &lt;= per_user_limit, <a href="../social_contracts/message.md#social_contracts_message_E_RATE_LIMIT">E_RATE_LIMIT</a>);
    // Increment counters
    conv.rl_conv_count = conv.rl_conv_count + 1;
    <b>if</b> (table::contains(&conv.rl_user_counts, sender)) {
        <b>let</b> count = table::borrow_mut(&<b>mut</b> conv.rl_user_counts, sender);
        *count = *count + 1;
    } <b>else</b> {
        table::add(&<b>mut</b> conv.rl_user_counts, sender, 1);
        // Track this user <b>as</b> active in the current window
        vector::push_back(&<b>mut</b> conv.rl_active_users, sender);
    };
}
</code></pre>



</details>

<a name="social_contracts_message_send_paid_message"></a>

## Function `send_paid_message`

Send a paid message to a profile owner


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_send_paid_message">send_paid_message</a>(registry: &<a href="../social_contracts/message.md#social_contracts_message_Registry">social_contracts::message::Registry</a>, conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, recipient_profile: &<a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, kind: u8, parent: u64, digest_hash: vector&lt;u8&gt;, media_batch_hash: vector&lt;u8&gt;, key_ref: vector&lt;u8&gt;, client_ts: u64, char_count: u32, dedupe_key: vector&lt;u8&gt;, nonce: u128, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_send_paid_message">send_paid_message</a>(
    registry: &<a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>,
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    recipient_profile: &Profile,
    <b>mut</b> payment: Coin&lt;MYS&gt;,
    kind: u8,
    parent: u64,
    digest_hash: vector&lt;u8&gt;,
    media_batch_hash: vector&lt;u8&gt;,
    key_ref: vector&lt;u8&gt;,
    client_ts: u64,
    char_count: u32,
    dedupe_key: vector&lt;u8&gt;,
    nonce: u128,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(!registry.paused, <a href="../social_contracts/message.md#social_contracts_message_E_PAUSED">E_PAUSED</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>let</b> recipient = <a href="../social_contracts/profile.md#social_contracts_profile_get_owner">profile::get_owner</a>(recipient_profile);
    // Check <b>if</b> <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> requires paid messaging
    <b>assert</b>!(<a href="../social_contracts/profile.md#social_contracts_profile_requires_paid_message">profile::requires_paid_message</a>(recipient_profile), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_PAID_MESSAGE">E_NOT_PAID_MESSAGE</a>);
    <b>let</b> min_cost = <a href="../social_contracts/profile.md#social_contracts_profile_get_min_message_cost">profile::get_min_message_cost</a>(recipient_profile);
    <b>assert</b>!(option::is_some(&min_cost), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_PAID_MESSAGE">E_NOT_PAID_MESSAGE</a>);
    <b>let</b> required_amount = *option::borrow(&min_cost);
    // Check payment is sufficient
    <b>assert</b>!(coin::value(&payment) &gt;= required_amount, <a href="../social_contracts/message.md#social_contracts_message_E_INSUFFICIENT_PAYMENT">E_INSUFFICIENT_PAYMENT</a>);
    // Check dedupe
    <b>assert</b>!(!table::contains(&conv.used_dedupe, dedupe_key), <a href="../social_contracts/message.md#social_contracts_message_E_DEDUPE_USED">E_DEDUPE_USED</a>);
    table::add(&<b>mut</b> conv.used_dedupe, dedupe_key, <b>true</b>);
    // Check nonce
    <b>if</b> (!table::contains(&conv.nonces, sender)) {
        table::add(&<b>mut</b> conv.nonces, sender, table::new(ctx));
    };
    <b>let</b> member_nonces = table::borrow_mut(&<b>mut</b> conv.nonces, sender);
    <b>assert</b>!(!table::contains(member_nonces, nonce), <a href="../social_contracts/message.md#social_contracts_message_E_NONCE_USED">E_NONCE_USED</a>);
    table::add(member_nonces, nonce, <b>true</b>);
    // Enforce rate limits (paid messages also subject to rate limits)
    <a href="../social_contracts/message.md#social_contracts_message_enforce_rate_limits">enforce_rate_limits</a>(registry, conv, sender, clock);
    // Assign sequence number
    <b>let</b> seq = conv.<a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a>;
    conv.<a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a> = seq + 1;
    // Extract payment and hold in escrow
    <b>let</b> escrow_payment = coin::split(&<b>mut</b> payment, required_amount, ctx);
    <b>let</b> escrow_balance = coin::into_balance(escrow_payment);
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    // Create escrow <b>entry</b>
    <b>let</b> escrow = <a href="../social_contracts/message.md#social_contracts_message_PaidMessageEscrow">PaidMessageEscrow</a> {
        payer: sender,
        recipient,
        amount: required_amount,
        escrowed_balance: escrow_balance,
        created_epoch: current_epoch,
        claimed: <b>false</b>,
        parent_seq: seq,
    };
    table::add(&<b>mut</b> conv.paid_msg_escrow, seq, escrow);
    // Create <a href="../social_contracts/message.md#social_contracts_message">message</a> digest
    <b>let</b> msg = <a href="../social_contracts/message.md#social_contracts_message_MessageDigest">MessageDigest</a> {
        seq,
        sender,
        kind,
        parent,
        digest_hash,
        media_batch_hash,
        key_ref,
        edit_seq: 0,
        client_ts,
        server_ts: clock::timestamp_ms(clock) / 1000,
        flags: 0,
        char_count: option::some(char_count),
        is_paid: <b>true</b>,
    };
    table::add(&<b>mut</b> conv.messages, seq, msg);
    // Emit events
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_MessageSent">MessageSent</a> {
        conv: object::uid_to_address(&conv.id),
        seq,
        sender,
        kind,
    });
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_PaidMessageSent">PaidMessageSent</a> {
        conv: object::uid_to_address(&conv.id),
        seq,
        payer: sender,
        recipient,
        amount: required_amount,
        created_epoch: current_epoch,
    });
    // Return excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, sender);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
}
</code></pre>



</details>

<a name="social_contracts_message_reply_to_paid_message"></a>

## Function `reply_to_paid_message`

Reply to a paid message and trigger payment release if conditions are met


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_reply_to_paid_message">reply_to_paid_message</a>(registry: &<a href="../social_contracts/message.md#social_contracts_message_Registry">social_contracts::message::Registry</a>, conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, paid_msg_seq: u64, kind: u8, digest_hash: vector&lt;u8&gt;, media_batch_hash: vector&lt;u8&gt;, key_ref: vector&lt;u8&gt;, client_ts: u64, char_count: u32, dedupe_key: vector&lt;u8&gt;, nonce: u128, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_reply_to_paid_message">reply_to_paid_message</a>(
    registry: &<a href="../social_contracts/message.md#social_contracts_message_Registry">Registry</a>,
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    treasury: &EcosystemTreasury,
    paid_msg_seq: u64,
    kind: u8,
    digest_hash: vector&lt;u8&gt;,
    media_batch_hash: vector&lt;u8&gt;,
    key_ref: vector&lt;u8&gt;,
    client_ts: u64,
    char_count: u32,
    dedupe_key: vector&lt;u8&gt;,
    nonce: u128,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(!registry.paused, <a href="../social_contracts/message.md#social_contracts_message_E_PAUSED">E_PAUSED</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&conv.members, sender), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_MEMBER">E_NOT_MEMBER</a>);
    // Verify the paid <a href="../social_contracts/message.md#social_contracts_message">message</a> exists and <b>has</b> escrow
    <b>assert</b>!(table::contains(&conv.paid_msg_escrow, paid_msg_seq), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_PAID_MESSAGE">E_NOT_PAID_MESSAGE</a>);
    <b>let</b> escrow = table::borrow(&conv.paid_msg_escrow, paid_msg_seq);
    // Verify sender is the recipient
    <b>assert</b>!(sender == escrow.recipient, <a href="../social_contracts/message.md#social_contracts_message_E_FORBIDDEN">E_FORBIDDEN</a>);
    // Verify payment not already claimed
    <b>assert</b>!(!escrow.claimed, <a href="../social_contracts/message.md#social_contracts_message_E_PAYMENT_ALREADY_CLAIMED">E_PAYMENT_ALREADY_CLAIMED</a>);
    // Verify payment not expired (with underflow protection)
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    // Check <b>for</b> clock issues - <b>if</b> created_epoch is in the future, treat <b>as</b> expired
    <b>if</b> (current_epoch &lt; escrow.created_epoch) {
        <b>abort</b> <a href="../social_contracts/message.md#social_contracts_message_E_PAYMENT_EXPIRED">E_PAYMENT_EXPIRED</a>
    };
    <b>assert</b>!(current_epoch - escrow.created_epoch &lt;= <a href="../social_contracts/message.md#social_contracts_message_PAYMENT_EXPIRATION_EPOCHS">PAYMENT_EXPIRATION_EPOCHS</a>, <a href="../social_contracts/message.md#social_contracts_message_E_PAYMENT_EXPIRED">E_PAYMENT_EXPIRED</a>);
    // Check dedupe
    <b>assert</b>!(!table::contains(&conv.used_dedupe, dedupe_key), <a href="../social_contracts/message.md#social_contracts_message_E_DEDUPE_USED">E_DEDUPE_USED</a>);
    table::add(&<b>mut</b> conv.used_dedupe, dedupe_key, <b>true</b>);
    // Check nonce
    <b>if</b> (!table::contains(&conv.nonces, sender)) {
        table::add(&<b>mut</b> conv.nonces, sender, table::new(ctx));
    };
    <b>let</b> member_nonces = table::borrow_mut(&<b>mut</b> conv.nonces, sender);
    <b>assert</b>!(!table::contains(member_nonces, nonce), <a href="../social_contracts/message.md#social_contracts_message_E_NONCE_USED">E_NONCE_USED</a>);
    table::add(member_nonces, nonce, <b>true</b>);
    // Verify reply meets minimum character requirement
    <b>assert</b>!(char_count &gt;= <a href="../social_contracts/message.md#social_contracts_message_MIN_REPLY_CHARS">MIN_REPLY_CHARS</a>, <a href="../social_contracts/message.md#social_contracts_message_E_REPLY_TOO_SHORT">E_REPLY_TOO_SHORT</a>);
    // Assign sequence number <b>for</b> reply
    <b>let</b> seq = conv.<a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a>;
    conv.<a href="../social_contracts/message.md#social_contracts_message_next_seq">next_seq</a> = seq + 1;
    // Create reply <a href="../social_contracts/message.md#social_contracts_message">message</a> digest
    <b>let</b> msg = <a href="../social_contracts/message.md#social_contracts_message_MessageDigest">MessageDigest</a> {
        seq,
        sender,
        kind,
        parent: paid_msg_seq, // Link to paid <a href="../social_contracts/message.md#social_contracts_message">message</a>
        digest_hash,
        media_batch_hash,
        key_ref,
        edit_seq: 0,
        client_ts,
        server_ts: clock::timestamp_ms(clock) / 1000,
        flags: 0,
        char_count: option::some(char_count),
        is_paid: <b>false</b>,
    };
    table::add(&<b>mut</b> conv.messages, seq, msg);
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_MessageSent">MessageSent</a> {
        conv: object::uid_to_address(&conv.id),
        seq,
        sender,
        kind,
    });
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_PaidMessageReplied">PaidMessageReplied</a> {
        conv: object::uid_to_address(&conv.id),
        paid_msg_seq,
        reply_seq: seq,
        recipient: sender,
        reply_char_count: char_count,
    });
    // Automatically claim the payment
    <a href="../social_contracts/message.md#social_contracts_message_claim_payment_internal">claim_payment_internal</a>(conv, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, treasury, paid_msg_seq, ctx);
}
</code></pre>



</details>

<a name="social_contracts_message_claim_payment_internal"></a>

## Function `claim_payment_internal`

Claim payment from a replied paid message (internal helper)


<pre><code><b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_claim_payment_internal">claim_payment_internal</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, paid_msg_seq: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_claim_payment_internal">claim_payment_internal</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    treasury: &EcosystemTreasury,
    paid_msg_seq: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Get mutable escrow
    <b>let</b> escrow = table::borrow_mut(&<b>mut</b> conv.paid_msg_escrow, paid_msg_seq);
    <b>assert</b>!(!escrow.claimed, <a href="../social_contracts/message.md#social_contracts_message_E_PAYMENT_ALREADY_CLAIMED">E_PAYMENT_ALREADY_CLAIMED</a>);
    <b>let</b> total_amount = escrow.amount;
    // Calculate fees with overflow protection using u128 intermediate values
    <b>let</b> platform_fee = (((total_amount <b>as</b> u128) * (<a href="../social_contracts/message.md#social_contracts_message_PAID_MSG_PLATFORM_FEE_BPS">PAID_MSG_PLATFORM_FEE_BPS</a> <b>as</b> u128)) / 10000) <b>as</b> u64;
    <b>let</b> treasury_fee = (((total_amount <b>as</b> u128) * (<a href="../social_contracts/message.md#social_contracts_message_PAID_MSG_TREASURY_FEE_BPS">PAID_MSG_TREASURY_FEE_BPS</a> <b>as</b> u128)) / 10000) <b>as</b> u64;
    <b>let</b> net_amount = total_amount - platform_fee - treasury_fee;
    // Split and distribute payments
    <b>let</b> <b>mut</b> escrow_coin = coin::from_balance(balance::withdraw_all(&<b>mut</b> escrow.escrowed_balance), ctx);
    // Platform fee
    <b>if</b> (platform_fee &gt; 0) {
        <b>let</b> <b>mut</b> platform_fee_coin = coin::split(&<b>mut</b> escrow_coin, platform_fee, ctx);
        <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_fee_coin, platform_fee, ctx);
        coin::destroy_zero(platform_fee_coin);
    };
    // Treasury fee
    <b>if</b> (treasury_fee &gt; 0) {
        <b>let</b> treasury_fee_coin = coin::split(&<b>mut</b> escrow_coin, treasury_fee, ctx);
        transfer::public_transfer(treasury_fee_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    // Net amount to recipient
    transfer::public_transfer(escrow_coin, escrow.recipient);
    // Mark <b>as</b> claimed
    escrow.claimed = <b>true</b>;
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_PaymentClaimed">PaymentClaimed</a> {
        conv: object::uid_to_address(&conv.id),
        seq: paid_msg_seq,
        recipient: escrow.recipient,
        amount: total_amount,
        platform_fee,
        treasury_fee,
        net_amount,
        claimed_epoch: current_epoch,
    });
}
</code></pre>



</details>

<a name="social_contracts_message_refund_paid_message"></a>

## Function `refund_paid_message`

Refund an expired or unclaimed paid message payment


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_refund_paid_message">refund_paid_message</a>(conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">social_contracts::message::Conversation</a>, paid_msg_seq: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/message.md#social_contracts_message_refund_paid_message">refund_paid_message</a>(
    conv: &<b>mut</b> <a href="../social_contracts/message.md#social_contracts_message_Conversation">Conversation</a>,
    paid_msg_seq: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    // Verify escrow exists
    <b>assert</b>!(table::contains(&conv.paid_msg_escrow, paid_msg_seq), <a href="../social_contracts/message.md#social_contracts_message_E_NOT_PAID_MESSAGE">E_NOT_PAID_MESSAGE</a>);
    <b>let</b> escrow = table::borrow_mut(&<b>mut</b> conv.paid_msg_escrow, paid_msg_seq);
    // Only payer can request refund
    <b>assert</b>!(sender == escrow.payer, <a href="../social_contracts/message.md#social_contracts_message_E_FORBIDDEN">E_FORBIDDEN</a>);
    // Verify not already claimed
    <b>assert</b>!(!escrow.claimed, <a href="../social_contracts/message.md#social_contracts_message_E_PAYMENT_ALREADY_CLAIMED">E_PAYMENT_ALREADY_CLAIMED</a>);
    // Verify payment is expired (&gt;= to include the expiration epoch) with underflow protection
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    // Check <b>for</b> clock issues - <b>if</b> created_epoch is in the future, allow refund
    <b>if</b> (current_epoch &lt; escrow.created_epoch) {
        // Clock issue - allow refund <b>as</b> a safety measure
    } <b>else</b> {
        <b>assert</b>!(current_epoch - escrow.created_epoch &gt;= <a href="../social_contracts/message.md#social_contracts_message_PAYMENT_EXPIRATION_EPOCHS">PAYMENT_EXPIRATION_EPOCHS</a>, <a href="../social_contracts/message.md#social_contracts_message_E_PAYMENT_EXPIRED">E_PAYMENT_EXPIRED</a>);
    };
    <b>let</b> refund_amount = escrow.amount;
    // Refund the payment
    <b>let</b> refund_coin = coin::from_balance(balance::withdraw_all(&<b>mut</b> escrow.escrowed_balance), ctx);
    transfer::public_transfer(refund_coin, escrow.payer);
    // Mark <b>as</b> claimed (to prevent double refund)
    escrow.claimed = <b>true</b>;
    event::emit(<a href="../social_contracts/message.md#social_contracts_message_PaymentRefunded">PaymentRefunded</a> {
        conv: object::uid_to_address(&conv.id),
        seq: paid_msg_seq,
        payer: escrow.payer,
        amount: refund_amount,
        refunded_epoch: current_epoch,
        reason: 0, // 0 = expired
    });
}
</code></pre>



</details>
