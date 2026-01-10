---
title: Module `social_contracts::proof_of_creativity`
---

Proof of Creativity module for the MySocial network
Manages content originality verification through oracle analysis,
PoC badge issuance, revenue redirection, and community dispute voting


-  [Struct `PoCAdminCap`](#social_contracts_proof_of_creativity_PoCAdminCap)
-  [Struct `PoCConfig`](#social_contracts_proof_of_creativity_PoCConfig)
-  [Struct `Vote`](#social_contracts_proof_of_creativity_Vote)
-  [Struct `PoCDispute`](#social_contracts_proof_of_creativity_PoCDispute)
-  [Struct `PoCRegistry`](#social_contracts_proof_of_creativity_PoCRegistry)
-  [Struct `AnalysisSubmittedEvent`](#social_contracts_proof_of_creativity_AnalysisSubmittedEvent)
-  [Struct `PoCBadgeIssuedEvent`](#social_contracts_proof_of_creativity_PoCBadgeIssuedEvent)
-  [Struct `RevenueRedirectionActivatedEvent`](#social_contracts_proof_of_creativity_RevenueRedirectionActivatedEvent)
-  [Struct `PoCDisputeSubmittedEvent`](#social_contracts_proof_of_creativity_PoCDisputeSubmittedEvent)
-  [Struct `DisputeVoteCastEvent`](#social_contracts_proof_of_creativity_DisputeVoteCastEvent)
-  [Struct `PoCDisputeResolvedEvent`](#social_contracts_proof_of_creativity_PoCDisputeResolvedEvent)
-  [Struct `VotingRewardClaimedEvent`](#social_contracts_proof_of_creativity_VotingRewardClaimedEvent)
-  [Struct `PoCConfigUpdatedEvent`](#social_contracts_proof_of_creativity_PoCConfigUpdatedEvent)
-  [Struct `TokenPoolSyncNeededEvent`](#social_contracts_proof_of_creativity_TokenPoolSyncNeededEvent)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_proof_of_creativity_bootstrap_init)
-  [Function `update_poc_config`](#social_contracts_proof_of_creativity_update_poc_config)
-  [Function `analyze_and_update_post`](#social_contracts_proof_of_creativity_analyze_and_update_post)
-  [Function `update_token_pool_if_exists`](#social_contracts_proof_of_creativity_update_token_pool_if_exists)
-  [Function `submit_poc_dispute`](#social_contracts_proof_of_creativity_submit_poc_dispute)
-  [Function `vote_on_dispute`](#social_contracts_proof_of_creativity_vote_on_dispute)
-  [Function `resolve_dispute_voting`](#social_contracts_proof_of_creativity_resolve_dispute_voting)
-  [Function `claim_voting_reward`](#social_contracts_proof_of_creativity_claim_voting_reward)
-  [Function `get_threshold_for_media_type`](#social_contracts_proof_of_creativity_get_threshold_for_media_type)
-  [Function `is_authorized_oracle`](#social_contracts_proof_of_creativity_is_authorized_oracle)
-  [Function `get_registry_stats`](#social_contracts_proof_of_creativity_get_registry_stats)
-  [Function `has_poc_data`](#social_contracts_proof_of_creativity_has_poc_data)
-  [Function `get_dispute_voting_status`](#social_contracts_proof_of_creativity_get_dispute_voting_status)
-  [Function `get_dispute_stakes`](#social_contracts_proof_of_creativity_get_dispute_stakes)
-  [Function `has_user_voted`](#social_contracts_proof_of_creativity_has_user_voted)
-  [Function `config_version`](#social_contracts_proof_of_creativity_config_version)
-  [Function `dispute_version`](#social_contracts_proof_of_creativity_dispute_version)
-  [Function `registry_version`](#social_contracts_proof_of_creativity_registry_version)
-  [Function `migrate_poc_config`](#social_contracts_proof_of_creativity_migrate_poc_config)
-  [Function `migrate_poc_dispute`](#social_contracts_proof_of_creativity_migrate_poc_dispute)
-  [Function `migrate_poc_registry`](#social_contracts_proof_of_creativity_migrate_poc_registry)
-  [Function `create_poc_admin_cap`](#social_contracts_proof_of_creativity_create_poc_admin_cap)


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
<b>use</b> <a href="../mys/math.md#mys_math">mys::math</a>;
<b>use</b> <a href="../mys/mys.md#mys_mys">mys::mys</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/package.md#mys_package">mys::package</a>;
<b>use</b> <a href="../mys/table.md#mys_table">mys::table</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/types.md#mys_types">mys::types</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/post.md#social_contracts_post">social_contracts::post</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
<b>use</b> <a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens">social_contracts::social_proof_tokens</a>;
<b>use</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">social_contracts::subscription</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/hash.md#std_hash">std::hash</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_proof_of_creativity_PoCAdminCap"></a>

## Struct `PoCAdminCap`

Admin capability for Proof of Creativity system management


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCAdminCap">PoCAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_PoCConfig"></a>

## Struct `PoCConfig`

Global configuration for Proof of Creativity system


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a> <b>has</b> key
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
<code>oracle_address: <b>address</b></code>
</dt>
<dd>
 Oracle address authorized to submit analysis results
</dd>
<dt>
<code>image_threshold: u64</code>
</dt>
<dd>
 Similarity thresholds for different media types (stored as percentages 0-100)
</dd>
<dt>
<code>video_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>audio_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>revenue_redirect_percentage: u64</code>
</dt>
<dd>
 Percentage of revenue to redirect when similarity detected (0-100)
</dd>
<dt>
<code>dispute_cost: u64</code>
</dt>
<dd>
 Cost to submit a dispute
</dd>
<dt>
<code>dispute_protocol_fee: u64</code>
</dt>
<dd>
 Protocol fee for disputes (goes to ecosystem treasury)
</dd>
<dt>
<code>min_vote_stake: u64</code>
</dt>
<dd>
 Minimum stake amount required to vote on disputes
</dd>
<dt>
<code>max_vote_stake: u64</code>
</dt>
<dd>
 Maximum stake amount allowed per vote
</dd>
<dt>
<code>voting_duration_epochs: u64</code>
</dt>
<dd>
 Voting duration in epochs
</dd>
<dt>
<code>max_reasoning_length: u64</code>
</dt>
<dd>
 Maximum length for reasoning text
</dd>
<dt>
<code>max_evidence_urls: u64</code>
</dt>
<dd>
 Maximum number of evidence URLs allowed
</dd>
<dt>
<code>max_votes_per_dispute: u64</code>
</dt>
<dd>
 Maximum number of votes allowed per dispute
</dd>
<dt>
<code>dispute_governance_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
 Governance registry ID for PoC disputes
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_Vote"></a>

## Struct `Vote`

Individual vote record in a dispute


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_Vote">Vote</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
 Voter's address
</dd>
<dt>
<code>vote_choice: u8</code>
</dt>
<dd>
 Vote choice (VOTE_UPHOLD or VOTE_OVERTURN)
</dd>
<dt>
<code>stake_amount: u64</code>
</dt>
<dd>
 Amount of MySo staked with this vote
</dd>
<dt>
<code>voted_at: u64</code>
</dt>
<dd>
 Epoch when vote was cast
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_PoCDispute"></a>

## Struct `PoCDispute`

Dispute challenging a PoC badge or revenue redirection with community voting


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a> <b>has</b> key
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
<code>post_id: <b>address</b></code>
</dt>
<dd>
 Post being disputed
</dd>
<dt>
<code>disputer: <b>address</b></code>
</dt>
<dd>
 Address that submitted the dispute (post owner)
</dd>
<dt>
<code>dispute_type: u8</code>
</dt>
<dd>
 Type of dispute (challenging badge or redirection)
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
 Current status of dispute
</dd>
<dt>
<code>evidence: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Evidence or reasoning provided by disputer
</dd>
<dt>
<code>submitted_at: u64</code>
</dt>
<dd>
 Dispute submission timestamp
</dd>
<dt>
<code>voting_start_epoch: u64</code>
</dt>
<dd>
 Epoch when voting starts
</dd>
<dt>
<code>voting_end_epoch: u64</code>
</dt>
<dd>
 Epoch when voting ends
</dd>
<dt>
<code>votes: vector&lt;<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_Vote">social_contracts::proof_of_creativity::Vote</a>&gt;</code>
</dt>
<dd>
 All votes cast on this dispute
</dd>
<dt>
<code>uphold_stake: u64</code>
</dt>
<dd>
 Total stake on uphold side
</dd>
<dt>
<code>overturn_stake: u64</code>
</dt>
<dd>
 Total stake on overturn side
</dd>
<dt>
<code>voter_records: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>
 Mapping of voter addresses to prevent double voting
</dd>
<dt>
<code>reward_pool: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 Total reward pool from losing side (set after resolution)
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_PoCRegistry"></a>

## Struct `PoCRegistry`

Simplified registry to track PoC statistics


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">PoCRegistry</a> <b>has</b> key
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
<code>total_badges_issued: u64</code>
</dt>
<dd>
 Total badges issued
</dd>
<dt>
<code>total_redirections_created: u64</code>
</dt>
<dd>
 Total redirections created
</dd>
<dt>
<code>total_disputes_submitted: u64</code>
</dt>
<dd>
 Total disputes submitted
</dd>
<dt>
<code>total_votes_cast: u64</code>
</dt>
<dd>
 Total votes cast across all disputes
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_AnalysisSubmittedEvent"></a>

## Struct `AnalysisSubmittedEvent`

Event emitted when oracle submits analysis results


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_AnalysisSubmittedEvent">AnalysisSubmittedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>media_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>similarity_detected: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>highest_similarity_score: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>oracle_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>evidence_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_PoCBadgeIssuedEvent"></a>

## Struct `PoCBadgeIssuedEvent`

Event emitted when a PoC badge is issued


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCBadgeIssuedEvent">PoCBadgeIssuedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>badge_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>media_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>issued_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_RevenueRedirectionActivatedEvent"></a>

## Struct `RevenueRedirectionActivatedEvent`

Event emitted when revenue redirection is activated


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_RevenueRedirectionActivatedEvent">RevenueRedirectionActivatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>redirection_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>accused_post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>original_post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>redirect_percentage: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>similarity_score: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_PoCDisputeSubmittedEvent"></a>

## Struct `PoCDisputeSubmittedEvent`

Event emitted when a PoC dispute is submitted


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDisputeSubmittedEvent">PoCDisputeSubmittedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dispute_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>disputer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>dispute_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_start_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_end_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_DisputeVoteCastEvent"></a>

## Struct `DisputeVoteCastEvent`

Event emitted when a vote is cast on a dispute


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DisputeVoteCastEvent">DisputeVoteCastEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dispute_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>vote_choice: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_uphold_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_overturn_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_PoCDisputeResolvedEvent"></a>

## Struct `PoCDisputeResolvedEvent`

Event emitted when a dispute is resolved


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDisputeResolvedEvent">PoCDisputeResolvedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dispute_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>resolution: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>winning_side: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>total_winning_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_losing_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>badge_revoked: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>redirection_removed: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_VotingRewardClaimedEvent"></a>

## Struct `VotingRewardClaimedEvent`

Event emitted when voting rewards are claimed


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VotingRewardClaimedEvent">VotingRewardClaimedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dispute_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>original_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reward_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_payout: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_PoCConfigUpdatedEvent"></a>

## Struct `PoCConfigUpdatedEvent`

Event emitted when PoC configuration is updated


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfigUpdatedEvent">PoCConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>image_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>video_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>audio_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>revenue_redirect_percentage: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>dispute_cost: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_vote_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_vote_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_duration_epochs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_reasoning_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_evidence_urls: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_votes_per_dispute: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_proof_of_creativity_TokenPoolSyncNeededEvent"></a>

## Struct `TokenPoolSyncNeededEvent`

Event emitted when token pool synchronization is needed


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_TokenPoolSyncNeededEvent">TokenPoolSyncNeededEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_proof_of_creativity_DEFAULT_AUDIO_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_AUDIO_THRESHOLD">DEFAULT_AUDIO_THRESHOLD</a>: u64 = 95;
</code></pre>



<a name="social_contracts_proof_of_creativity_DEFAULT_DISPUTE_COST"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_DISPUTE_COST">DEFAULT_DISPUTE_COST</a>: u64 = 5000000000;
</code></pre>



<a name="social_contracts_proof_of_creativity_DEFAULT_DISPUTE_PROTOCOL_FEE"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_DISPUTE_PROTOCOL_FEE">DEFAULT_DISPUTE_PROTOCOL_FEE</a>: u64 = 1000000000;
</code></pre>



<a name="social_contracts_proof_of_creativity_DEFAULT_IMAGE_THRESHOLD"></a>

Configuration constants (default values)


<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_IMAGE_THRESHOLD">DEFAULT_IMAGE_THRESHOLD</a>: u64 = 95;
</code></pre>



<a name="social_contracts_proof_of_creativity_DEFAULT_MAX_VOTES_PER_DISPUTE"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_MAX_VOTES_PER_DISPUTE">DEFAULT_MAX_VOTES_PER_DISPUTE</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_proof_of_creativity_DEFAULT_MAX_VOTE_STAKE"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_MAX_VOTE_STAKE">DEFAULT_MAX_VOTE_STAKE</a>: u64 = 100000000000;
</code></pre>



<a name="social_contracts_proof_of_creativity_DEFAULT_MIN_VOTE_STAKE"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_MIN_VOTE_STAKE">DEFAULT_MIN_VOTE_STAKE</a>: u64 = 1000000000;
</code></pre>



<a name="social_contracts_proof_of_creativity_DEFAULT_REVENUE_REDIRECT_PERCENTAGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_REVENUE_REDIRECT_PERCENTAGE">DEFAULT_REVENUE_REDIRECT_PERCENTAGE</a>: u64 = 100;
</code></pre>



<a name="social_contracts_proof_of_creativity_DEFAULT_VIDEO_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_VIDEO_THRESHOLD">DEFAULT_VIDEO_THRESHOLD</a>: u64 = 95;
</code></pre>



<a name="social_contracts_proof_of_creativity_DEFAULT_VOTING_DURATION_EPOCHS"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_VOTING_DURATION_EPOCHS">DEFAULT_VOTING_DURATION_EPOCHS</a>: u64 = 7;
</code></pre>



<a name="social_contracts_proof_of_creativity_DISPUTE_STATUS_RESOLVED_OVERTURNED"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DISPUTE_STATUS_RESOLVED_OVERTURNED">DISPUTE_STATUS_RESOLVED_OVERTURNED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_proof_of_creativity_DISPUTE_STATUS_RESOLVED_UPHELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DISPUTE_STATUS_RESOLVED_UPHELD">DISPUTE_STATUS_RESOLVED_UPHELD</a>: u8 = 2;
</code></pre>



<a name="social_contracts_proof_of_creativity_DISPUTE_STATUS_VOTING"></a>

Dispute status constants


<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DISPUTE_STATUS_VOTING">DISPUTE_STATUS_VOTING</a>: u8 = 1;
</code></pre>



<a name="social_contracts_proof_of_creativity_EAlreadyVoted"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EAlreadyVoted">EAlreadyVoted</a>: u64 = 17;
</code></pre>



<a name="social_contracts_proof_of_creativity_EDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EDisabled">EDisabled</a>: u64 = 21;
</code></pre>



<a name="social_contracts_proof_of_creativity_EInsufficientFunds"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInsufficientFunds">EInsufficientFunds</a>: u64 = 9;
</code></pre>



<a name="social_contracts_proof_of_creativity_EInvalidEvidenceUrls"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidEvidenceUrls">EInvalidEvidenceUrls</a>: u64 = 20;
</code></pre>



<a name="social_contracts_proof_of_creativity_EInvalidMediaType"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidMediaType">EInvalidMediaType</a>: u64 = 7;
</code></pre>



<a name="social_contracts_proof_of_creativity_EInvalidReasoning"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidReasoning">EInvalidReasoning</a>: u64 = 19;
</code></pre>



<a name="social_contracts_proof_of_creativity_EInvalidStakeAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidStakeAmount">EInvalidStakeAmount</a>: u64 = 14;
</code></pre>



<a name="social_contracts_proof_of_creativity_EInvalidThreshold"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidThreshold">EInvalidThreshold</a>: u64 = 2;
</code></pre>



<a name="social_contracts_proof_of_creativity_ENoVotesToResolve"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_ENoVotesToResolve">ENoVotesToResolve</a>: u64 = 18;
</code></pre>



<a name="social_contracts_proof_of_creativity_ENotOracle"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_ENotOracle">ENotOracle</a>: u64 = 12;
</code></pre>



<a name="social_contracts_proof_of_creativity_EPostNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EPostNotFound">EPostNotFound</a>: u64 = 3;
</code></pre>



<a name="social_contracts_proof_of_creativity_ETooManyVotes"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_ETooManyVotes">ETooManyVotes</a>: u64 = 22;
</code></pre>



<a name="social_contracts_proof_of_creativity_EUnauthorized"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EUnauthorized">EUnauthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_proof_of_creativity_EVotingEnded"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EVotingEnded">EVotingEnded</a>: u64 = 16;
</code></pre>



<a name="social_contracts_proof_of_creativity_EVotingNotActive"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EVotingNotActive">EVotingNotActive</a>: u64 = 15;
</code></pre>



<a name="social_contracts_proof_of_creativity_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EWrongVersion">EWrongVersion</a>: u64 = 11;
</code></pre>



<a name="social_contracts_proof_of_creativity_MAX_EVIDENCE_URLS"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MAX_EVIDENCE_URLS">MAX_EVIDENCE_URLS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_proof_of_creativity_MAX_REASONING_LENGTH"></a>

Validation constants


<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MAX_REASONING_LENGTH">MAX_REASONING_LENGTH</a>: u64 = 5000;
</code></pre>



<a name="social_contracts_proof_of_creativity_MEDIA_TYPE_AUDIO"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MEDIA_TYPE_AUDIO">MEDIA_TYPE_AUDIO</a>: u8 = 3;
</code></pre>



<a name="social_contracts_proof_of_creativity_MEDIA_TYPE_IMAGE"></a>

Media type constants


<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MEDIA_TYPE_IMAGE">MEDIA_TYPE_IMAGE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_proof_of_creativity_MEDIA_TYPE_VIDEO"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MEDIA_TYPE_VIDEO">MEDIA_TYPE_VIDEO</a>: u8 = 2;
</code></pre>



<a name="social_contracts_proof_of_creativity_VOTE_OVERTURN"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_OVERTURN">VOTE_OVERTURN</a>: u8 = 2;
</code></pre>



<a name="social_contracts_proof_of_creativity_VOTE_UPHOLD"></a>

Vote option constants


<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_UPHOLD">VOTE_UPHOLD</a>: u8 = 1;
</code></pre>



<a name="social_contracts_proof_of_creativity_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the PoC configuration and registry


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> sender = tx_context::sender(ctx);
    // Create and share PoC configuration
    transfer::share_object(
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a> {
            id: object::new(ctx),
            oracle_address: sender, // Initially set to deployer, should be updated
            image_threshold: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_IMAGE_THRESHOLD">DEFAULT_IMAGE_THRESHOLD</a>,
            video_threshold: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_VIDEO_THRESHOLD">DEFAULT_VIDEO_THRESHOLD</a>,
            audio_threshold: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_AUDIO_THRESHOLD">DEFAULT_AUDIO_THRESHOLD</a>,
            revenue_redirect_percentage: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_REVENUE_REDIRECT_PERCENTAGE">DEFAULT_REVENUE_REDIRECT_PERCENTAGE</a>,
            dispute_cost: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_DISPUTE_COST">DEFAULT_DISPUTE_COST</a>,
            dispute_protocol_fee: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_DISPUTE_PROTOCOL_FEE">DEFAULT_DISPUTE_PROTOCOL_FEE</a>,
            min_vote_stake: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_MIN_VOTE_STAKE">DEFAULT_MIN_VOTE_STAKE</a>,
            max_vote_stake: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_MAX_VOTE_STAKE">DEFAULT_MAX_VOTE_STAKE</a>,
            voting_duration_epochs: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_VOTING_DURATION_EPOCHS">DEFAULT_VOTING_DURATION_EPOCHS</a>,
            max_reasoning_length: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MAX_REASONING_LENGTH">MAX_REASONING_LENGTH</a>,
            max_evidence_urls: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MAX_EVIDENCE_URLS">MAX_EVIDENCE_URLS</a>,
            max_votes_per_dispute: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DEFAULT_MAX_VOTES_PER_DISPUTE">DEFAULT_MAX_VOTES_PER_DISPUTE</a>,
            dispute_governance_id: object::id_from_address(@0x0), // Placeholder <b>for</b> future <a href="../social_contracts/governance.md#social_contracts_governance">governance</a>
            version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        }
    );
    // Create and share PoC registry
    transfer::share_object(
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">PoCRegistry</a> {
            id: object::new(ctx),
            total_badges_issued: 0,
            total_redirections_created: 0,
            total_disputes_submitted: 0,
            total_votes_cast: 0,
            version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        }
    );
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_update_poc_config"></a>

## Function `update_poc_config`

Update PoC configuration (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_update_poc_config">update_poc_config</a>(_: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCAdminCap">social_contracts::proof_of_creativity::PoCAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">social_contracts::proof_of_creativity::PoCConfig</a>, oracle_address: <b>address</b>, image_threshold: u64, video_threshold: u64, audio_threshold: u64, revenue_redirect_percentage: u64, dispute_cost: u64, dispute_protocol_fee: u64, min_vote_stake: u64, max_vote_stake: u64, voting_duration_epochs: u64, max_reasoning_length: u64, max_evidence_urls: u64, max_votes_per_dispute: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_update_poc_config">update_poc_config</a>(
    _: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCAdminCap">PoCAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a>,
    oracle_address: <b>address</b>,
    image_threshold: u64,
    video_threshold: u64,
    audio_threshold: u64,
    revenue_redirect_percentage: u64,
    dispute_cost: u64,
    dispute_protocol_fee: u64,
    min_vote_stake: u64,
    max_vote_stake: u64,
    voting_duration_epochs: u64,
    max_reasoning_length: u64,
    max_evidence_urls: u64,
    max_votes_per_dispute: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Admin capability verification is handled by type system
    // Validate thresholds (0-100)
    <b>assert</b>!(image_threshold &lt;= 100, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidThreshold">EInvalidThreshold</a>);
    <b>assert</b>!(video_threshold &lt;= 100, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidThreshold">EInvalidThreshold</a>);
    <b>assert</b>!(audio_threshold &lt;= 100, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidThreshold">EInvalidThreshold</a>);
    <b>assert</b>!(revenue_redirect_percentage &lt;= 100, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidThreshold">EInvalidThreshold</a>);
    // Validate voting parameters
    <b>assert</b>!(min_vote_stake &gt; 0 && min_vote_stake &lt;= max_vote_stake, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidStakeAmount">EInvalidStakeAmount</a>);
    <b>assert</b>!(voting_duration_epochs &gt; 0, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidThreshold">EInvalidThreshold</a>);
    // Validate reasoning and evidence URL parameters
    <b>assert</b>!(max_reasoning_length &gt; 0, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidThreshold">EInvalidThreshold</a>);
    <b>assert</b>!(max_evidence_urls &gt; 0, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidThreshold">EInvalidThreshold</a>);
    <b>assert</b>!(max_votes_per_dispute &gt; 0, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidThreshold">EInvalidThreshold</a>);
    // Update configuration
    config.oracle_address = oracle_address;
    config.image_threshold = image_threshold;
    config.video_threshold = video_threshold;
    config.audio_threshold = audio_threshold;
    config.revenue_redirect_percentage = revenue_redirect_percentage;
    config.dispute_cost = dispute_cost;
    config.dispute_protocol_fee = dispute_protocol_fee;
    config.min_vote_stake = min_vote_stake;
    config.max_vote_stake = max_vote_stake;
    config.voting_duration_epochs = voting_duration_epochs;
    config.max_reasoning_length = max_reasoning_length;
    config.max_evidence_urls = max_evidence_urls;
    config.max_votes_per_dispute = max_votes_per_dispute;
    // Emit configuration update event
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfigUpdatedEvent">PoCConfigUpdatedEvent</a> {
        updated_by: tx_context::sender(ctx),
        image_threshold,
        video_threshold,
        audio_threshold,
        revenue_redirect_percentage,
        dispute_cost,
        min_vote_stake,
        max_vote_stake,
        voting_duration_epochs,
        max_reasoning_length,
        max_evidence_urls,
        max_votes_per_dispute,
        timestamp: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_analyze_and_update_post"></a>

## Function `analyze_and_update_post`

SINGLE ENTRY POINT: Oracle analyzes content and updates post PoC status
This is the ONLY function the PoC server needs to call
Automatically updates token pool if it exists
Reasoning and evidence URLs are optional for transparency and accountability


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_analyze_and_update_post">analyze_and_update_post</a>(config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">social_contracts::proof_of_creativity::PoCConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">social_contracts::proof_of_creativity::PoCRegistry</a>, token_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, media_type: u8, highest_similarity_score: u64, original_creator: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, evidence_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_analyze_and_update_post">analyze_and_update_post</a>(
    config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">PoCRegistry</a>,
    token_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>,
    media_type: u8,
    highest_similarity_score: u64,
    <b>mut</b> original_creator: Option&lt;<b>address</b>&gt;,
    reasoning: Option&lt;String&gt;,
    evidence_urls: Option&lt;vector&lt;String&gt;&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> timestamp = tx_context::epoch_timestamp_ms(ctx);
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">social_contracts::post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    // Verify caller is authorized oracle
    <b>assert</b>!(caller == config.oracle_address, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_ENotOracle">ENotOracle</a>);
    // Verify media type is valid
    <b>assert</b>!(
        media_type == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MEDIA_TYPE_IMAGE">MEDIA_TYPE_IMAGE</a> ||
        media_type == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MEDIA_TYPE_VIDEO">MEDIA_TYPE_VIDEO</a> ||
        media_type == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MEDIA_TYPE_AUDIO">MEDIA_TYPE_AUDIO</a>,
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidMediaType">EInvalidMediaType</a>
    );
    // Validate reasoning <b>if</b> provided
    <b>if</b> (option::is_some(&reasoning)) {
        <b>let</b> reasoning_val = option::borrow(&reasoning);
        <b>let</b> reasoning_len = string::length(reasoning_val);
        <b>assert</b>!(reasoning_len &lt;= config.max_reasoning_length, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidReasoning">EInvalidReasoning</a>);
    };
    // Validate evidence URLs array <b>if</b> provided
    <b>if</b> (option::is_some(&evidence_urls)) {
        <b>let</b> urls = option::borrow(&evidence_urls);
        <b>assert</b>!(vector::length(urls) &lt;= config.max_evidence_urls, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidEvidenceUrls">EInvalidEvidenceUrls</a>);
    };
    // Get threshold <b>for</b> this media type
    <b>let</b> threshold = <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_get_threshold_for_media_type">get_threshold_for_media_type</a>(config, media_type);
    // Determine <b>if</b> similarity exceeds threshold and original creator exists
    <b>let</b> similarity_detected = highest_similarity_score &gt;= threshold && option::is_some(&original_creator);
    <b>if</b> (similarity_detected) {
        // Content is derivative - apply revenue redirection
        <b>let</b> original_creator_address = option::extract(&<b>mut</b> original_creator);
        // Calculate redirect percentage using the same formula <b>as</b> before
        <b>let</b> delta_numerator = highest_similarity_score - threshold;
        <b>let</b> delta_denominator = 100 - threshold;
        <b>let</b> delta_percentage = <b>if</b> (delta_denominator &gt; 0) {
            (delta_numerator * 100) / delta_denominator
        } <b>else</b> {
            100 // If threshold is 100, redirect 100%
        };
        <b>let</b> redirect_percentage = (config.revenue_redirect_percentage * delta_percentage) / 100;
        // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> with redirection info and PoC metadata
        <a href="../social_contracts/post.md#social_contracts_post_update_poc_result">social_contracts::post::update_poc_result</a>(
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            2, // redirection applied
            option::some(original_creator_address), // redirect to original creator
            option::some(redirect_percentage), // redirect percentage
            reasoning, // PoC reasoning
            evidence_urls, // PoC evidence URLs
            highest_similarity_score, // similarity score
            media_type, // media type analyzed
            caller, // oracle <b>address</b>
            timestamp // analysis timestamp
        );
        // Update registry tracking
        registry.total_redirections_created = registry.total_redirections_created + 1;
        // Emit simplified event
        event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_RevenueRedirectionActivatedEvent">RevenueRedirectionActivatedEvent</a> {
            redirection_id: post_id, // Use <a href="../social_contracts/post.md#social_contracts_post">post</a> ID <b>as</b> redirection ID
            accused_post_id: post_id,
            original_post_id: original_creator_address,
            redirect_percentage,
            similarity_score: highest_similarity_score,
            timestamp,
        });
    } <b>else</b> {
        // Content is original - issue PoC badge
        // Verify PoC is enabled <b>for</b> this <a href="../social_contracts/post.md#social_contracts_post">post</a>
        <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post_is_poc_enabled">social_contracts::post::is_poc_enabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EDisabled">EDisabled</a>);
        // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> with PoC metadata (badge issued)
        <a href="../social_contracts/post.md#social_contracts_post_update_poc_result">social_contracts::post::update_poc_result</a>(
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            1, // badge issued
            option::none(), // no redirection
            option::none(), // no redirection percentage
            reasoning, // PoC reasoning
            evidence_urls, // PoC evidence URLs
            highest_similarity_score, // similarity score
            media_type, // media type analyzed
            caller, // oracle <b>address</b>
            timestamp // analysis timestamp
        );
        // Update registry tracking
        registry.total_badges_issued = registry.total_badges_issued + 1;
        // Emit event - indexers track badge status from this event
        event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCBadgeIssuedEvent">PoCBadgeIssuedEvent</a> {
            badge_id: post_id, // Use <a href="../social_contracts/post.md#social_contracts_post">post</a> ID <b>as</b> badge identifier
            post_id,
            media_type,
            issued_by: caller,
            timestamp,
        });
    };
    // Emit analysis event with reasoning and evidence URLs
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_AnalysisSubmittedEvent">AnalysisSubmittedEvent</a> {
        post_id,
        media_type,
        similarity_detected,
        highest_similarity_score,
        oracle_address: caller,
        timestamp,
        reasoning,
        evidence_urls,
    });
    // Automatically update token pool <b>if</b> it exists
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_update_token_pool_if_exists">update_token_pool_if_exists</a>(token_registry, <a href="../social_contracts/post.md#social_contracts_post">post</a>, ctx);
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_update_token_pool_if_exists"></a>

## Function `update_token_pool_if_exists`

Helper function to check if token pool sync is needed
This ensures token pools are automatically synchronized with PoC results


<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_update_token_pool_if_exists">update_token_pool_if_exists</a>(token_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, _ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_update_token_pool_if_exists">update_token_pool_if_exists</a>(
    token_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>,
    _ctx: &<b>mut</b> TxContext
) {
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">social_contracts::post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    // Check <b>if</b> a token pool exists <b>for</b> this <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>if</b> (<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_token_exists">social_contracts::social_proof_tokens::token_exists</a>(token_registry, post_id)) {
        // Token pool exists - emit event <b>for</b> automatic synchronization
        // The off-chain system can listen <b>for</b> this event and call update_token_poc_data
        event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_TokenPoolSyncNeededEvent">TokenPoolSyncNeededEvent</a> {
            post_id,
            timestamp: tx_context::epoch_timestamp_ms(_ctx),
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_submit_poc_dispute"></a>

## Function `submit_poc_dispute`

Submit a PoC dispute with community voting


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_submit_poc_dispute">submit_poc_dispute</a>(config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">social_contracts::proof_of_creativity::PoCConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">social_contracts::proof_of_creativity::PoCRegistry</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, evidence: <a href="../std/string.md#std_string_String">std::string::String</a>, payment: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_submit_poc_dispute">submit_poc_dispute</a>(
    config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">PoCRegistry</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>,
    evidence: String,
    <b>mut</b> payment: Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> disputer = tx_context::sender(ctx);
    <b>let</b> timestamp = tx_context::epoch_timestamp_ms(ctx);
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_get_id_address">social_contracts::post::get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    // Verify sufficient payment
    <b>let</b> total_cost = config.dispute_cost + config.dispute_protocol_fee;
    <b>assert</b>!(coin::value(&payment) &gt;= total_cost, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInsufficientFunds">EInsufficientFunds</a>);
    // Validate evidence length
    <b>let</b> evidence_len = string::length(&evidence);
    <b>assert</b>!(evidence_len &lt;= config.max_reasoning_length, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidReasoning">EInvalidReasoning</a>);
    // Verify only <a href="../social_contracts/post.md#social_contracts_post">post</a> owner can dispute their <a href="../social_contracts/post.md#social_contracts_post">post</a>'s PoC status
    <b>assert</b>!(disputer == <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">social_contracts::post::get_post_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>), <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EUnauthorized">EUnauthorized</a>);
    // Verify the <a href="../social_contracts/post.md#social_contracts_post">post</a> <b>has</b> PoC data to dispute (badge or redirection)
    <b>let</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_has_poc_data">has_poc_data</a> = option::is_some(<a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_to">social_contracts::post::get_revenue_redirect_to</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) ||
                        <a href="../social_contracts/post.md#social_contracts_post_has_poc_badge">social_contracts::post::has_poc_badge</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <b>assert</b>!(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_has_poc_data">has_poc_data</a>, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EPostNotFound">EPostNotFound</a>);
    // Extract dispute fee and send to ecosystem treasury
    <b>let</b> dispute_fee = coin::split(&<b>mut</b> payment, total_cost, ctx);
    transfer::public_transfer(dispute_fee, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">social_contracts::profile::get_treasury_address</a>(treasury));
    // Return excess payment
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, disputer);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Calculate voting period - start next epoch, end after voting duration
    <b>let</b> voting_start_epoch = current_epoch + 1;
    <b>let</b> voting_end_epoch = voting_start_epoch + config.voting_duration_epochs;
    // Create dispute with voting mechanism
    <b>let</b> dispute = <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a> {
        id: object::new(ctx),
        post_id,
        disputer,
        dispute_type: 1, // Generic PoC dispute
        status: <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DISPUTE_STATUS_VOTING">DISPUTE_STATUS_VOTING</a>,
        evidence,
        submitted_at: timestamp,
        voting_start_epoch,
        voting_end_epoch,
        votes: vector::empty(),
        uphold_stake: 0,
        overturn_stake: 0,
        voter_records: table::new(ctx),
        reward_pool: balance::zero(),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> dispute_id = object::uid_to_address(&dispute.id);
    // Update registry tracking
    registry.total_disputes_submitted = registry.total_disputes_submitted + 1;
    // Emit dispute submitted event
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDisputeSubmittedEvent">PoCDisputeSubmittedEvent</a> {
        dispute_id,
        post_id,
        disputer,
        dispute_type: 1,
        stake_amount: total_cost,
        voting_start_epoch,
        voting_end_epoch,
        timestamp,
    });
    // Share dispute <b>for</b> <b>public</b> voting
    transfer::share_object(dispute);
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_vote_on_dispute"></a>

## Function `vote_on_dispute`

Cast a vote on a PoC dispute (community voting)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_vote_on_dispute">vote_on_dispute</a>(config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">social_contracts::proof_of_creativity::PoCConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">social_contracts::proof_of_creativity::PoCRegistry</a>, dispute: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">social_contracts::proof_of_creativity::PoCDispute</a>, vote_choice: u8, stake_coin: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_vote_on_dispute">vote_on_dispute</a>(
    config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">PoCRegistry</a>,
    dispute: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a>,
    vote_choice: u8, // <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_UPHOLD">VOTE_UPHOLD</a> or <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_OVERTURN">VOTE_OVERTURN</a>
    stake_coin: Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> voter = tx_context::sender(ctx);
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    <b>let</b> timestamp = tx_context::epoch_timestamp_ms(ctx);
    <b>let</b> stake_amount = coin::value(&stake_coin);
    // Validate vote choice
    <b>assert</b>!(vote_choice == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_UPHOLD">VOTE_UPHOLD</a> || vote_choice == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_OVERTURN">VOTE_OVERTURN</a>, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EUnauthorized">EUnauthorized</a>);
    // Check vote limit
    <b>let</b> current_votes = vector::length(&dispute.votes);
    <b>assert</b>!(current_votes &lt; config.max_votes_per_dispute, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_ETooManyVotes">ETooManyVotes</a>);
    // Validate stake amount is within bounds
    <b>assert</b>!(stake_amount &gt;= config.min_vote_stake && stake_amount &lt;= config.max_vote_stake, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidStakeAmount">EInvalidStakeAmount</a>);
    // Verify voting period is active
    <b>assert</b>!(current_epoch &gt;= dispute.voting_start_epoch, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EVotingNotActive">EVotingNotActive</a>);
    <b>assert</b>!(current_epoch &lt;= dispute.voting_end_epoch, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EVotingEnded">EVotingEnded</a>);
    // Verify voter hasn't already voted
    <b>assert</b>!(!table::contains(&dispute.voter_records, voter), <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EAlreadyVoted">EAlreadyVoted</a>);
    // Record the vote
    <b>let</b> vote = <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_Vote">Vote</a> {
        voter,
        vote_choice,
        stake_amount,
        voted_at: current_epoch,
    };
    vector::push_back(&<b>mut</b> dispute.votes, vote);
    table::add(&<b>mut</b> dispute.voter_records, voter, <b>true</b>);
    // Update stake totals
    <b>if</b> (vote_choice == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_UPHOLD">VOTE_UPHOLD</a>) {
        dispute.uphold_stake = dispute.uphold_stake + stake_amount;
    } <b>else</b> {
        dispute.overturn_stake = dispute.overturn_stake + stake_amount;
    };
    // Take stake and hold it in the dispute
    <b>let</b> stake_balance = coin::into_balance(stake_coin);
    balance::join(&<b>mut</b> dispute.reward_pool, stake_balance);
    // Update registry tracking
    registry.total_votes_cast = registry.total_votes_cast + 1;
    // Emit vote event
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DisputeVoteCastEvent">DisputeVoteCastEvent</a> {
        dispute_id: object::uid_to_address(&dispute.id),
        voter,
        vote_choice,
        stake_amount,
        total_uphold_stake: dispute.uphold_stake,
        total_overturn_stake: dispute.overturn_stake,
        timestamp,
    });
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_resolve_dispute_voting"></a>

## Function `resolve_dispute_voting`

Resolve PoC dispute after voting period ends


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_resolve_dispute_voting">resolve_dispute_voting</a>(dispute: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">social_contracts::proof_of_creativity::PoCDispute</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, token_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_resolve_dispute_voting">resolve_dispute_voting</a>(
    dispute: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>,
    token_registry: &<a href="../social_contracts/social_proof_tokens.md#social_contracts_social_proof_tokens_TokenRegistry">social_contracts::social_proof_tokens::TokenRegistry</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    <b>let</b> timestamp = tx_context::epoch_timestamp_ms(ctx);
    <b>let</b> dispute_id = object::uid_to_address(&dispute.id);
    // Verify voting period <b>has</b> ended
    <b>assert</b>!(current_epoch &gt; dispute.voting_end_epoch, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EVotingNotActive">EVotingNotActive</a>);
    // Verify there are votes to resolve
    <b>assert</b>!(vector::length(&dispute.votes) &gt; 0, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_ENoVotesToResolve">ENoVotesToResolve</a>);
    // Determine winning side
    <b>let</b> winning_side = <b>if</b> (dispute.uphold_stake &gt; dispute.overturn_stake) {
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_UPHOLD">VOTE_UPHOLD</a>
    } <b>else</b> {
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_OVERTURN">VOTE_OVERTURN</a>
    };
    <b>let</b> (total_winning_stake, total_losing_stake) = <b>if</b> (winning_side == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_UPHOLD">VOTE_UPHOLD</a>) {
        (dispute.uphold_stake, dispute.overturn_stake)
    } <b>else</b> {
        (dispute.overturn_stake, dispute.uphold_stake)
    };
    // Apply dispute resolution to <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>let</b> (badge_revoked, redirection_removed) = <b>if</b> (winning_side == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_OVERTURN">VOTE_OVERTURN</a>) {
        // Challenger wins - clear PoC data
        <a href="../social_contracts/post.md#social_contracts_post_clear_poc_data">social_contracts::post::clear_poc_data</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
        (<b>true</b>, <b>true</b>)
    } <b>else</b> {
        // Original decision stands - no changes needed
        (<b>false</b>, <b>false</b>)
    };
    // Update dispute status
    dispute.status = <b>if</b> (winning_side == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_UPHOLD">VOTE_UPHOLD</a>) {
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DISPUTE_STATUS_RESOLVED_UPHELD">DISPUTE_STATUS_RESOLVED_UPHELD</a>
    } <b>else</b> {
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DISPUTE_STATUS_RESOLVED_OVERTURNED">DISPUTE_STATUS_RESOLVED_OVERTURNED</a>
    };
    // Emit resolution event
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDisputeResolvedEvent">PoCDisputeResolvedEvent</a> {
        dispute_id,
        post_id: dispute.post_id,
        resolution: dispute.status,
        winning_side,
        total_winning_stake,
        total_losing_stake,
        badge_revoked,
        redirection_removed,
        timestamp,
    });
    // Automatically update token pool <b>if</b> it exists
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_update_token_pool_if_exists">update_token_pool_if_exists</a>(token_registry, <a href="../social_contracts/post.md#social_contracts_post">post</a>, ctx);
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_claim_voting_reward"></a>

## Function `claim_voting_reward`

Claim voting rewards after dispute resolution


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_claim_voting_reward">claim_voting_reward</a>(dispute: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">social_contracts::proof_of_creativity::PoCDispute</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_claim_voting_reward">claim_voting_reward</a>(
    dispute: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> claimer = tx_context::sender(ctx);
    <b>let</b> timestamp = tx_context::epoch_timestamp_ms(ctx);
    <b>let</b> dispute_id = object::uid_to_address(&dispute.id);
    // Verify dispute is resolved
    <b>assert</b>!(
        dispute.status == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DISPUTE_STATUS_RESOLVED_UPHELD">DISPUTE_STATUS_RESOLVED_UPHELD</a> ||
        dispute.status == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_DISPUTE_STATUS_RESOLVED_OVERTURNED">DISPUTE_STATUS_RESOLVED_OVERTURNED</a>,
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EVotingNotActive">EVotingNotActive</a>
    );
    // Find the voter's vote and verify they voted on winning side
    <b>let</b> votes_len = vector::length(&dispute.votes);
    <b>let</b> <b>mut</b> vote_index = 0;
    <b>let</b> <b>mut</b> found_vote = <b>false</b>;
    <b>let</b> <b>mut</b> voter_stake = 0;
    <b>let</b> <b>mut</b> voter_choice = 0;
    <b>while</b> (vote_index &lt; votes_len && !found_vote) {
        <b>let</b> vote = vector::borrow(&dispute.votes, vote_index);
        <b>if</b> (vote.voter == claimer) {
            found_vote = <b>true</b>;
            voter_stake = vote.stake_amount;
            voter_choice = vote.vote_choice;
        };
        vote_index = vote_index + 1;
    };
    <b>assert</b>!(found_vote, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EUnauthorized">EUnauthorized</a>);
    // Determine winning side and verify voter was on winning side
    <b>let</b> winning_side = <b>if</b> (dispute.uphold_stake &gt; dispute.overturn_stake) {
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_UPHOLD">VOTE_UPHOLD</a>
    } <b>else</b> {
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_OVERTURN">VOTE_OVERTURN</a>
    };
    <b>assert</b>!(voter_choice == winning_side, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EUnauthorized">EUnauthorized</a>);
    // Calculate reward
    <b>let</b> (total_winning_stake, total_losing_stake) = <b>if</b> (winning_side == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VOTE_UPHOLD">VOTE_UPHOLD</a>) {
        (dispute.uphold_stake, dispute.overturn_stake)
    } <b>else</b> {
        (dispute.overturn_stake, dispute.uphold_stake)
    };
    // Calculate proportional reward: original stake + share of losing side
    <b>let</b> reward_from_losers = <b>if</b> (total_winning_stake &gt; 0) {
        (((voter_stake <b>as</b> u128) * (total_losing_stake <b>as</b> u128)) / (total_winning_stake <b>as</b> u128)) <b>as</b> u64
    } <b>else</b> {
        0
    };
    <b>let</b> total_payout = voter_stake + reward_from_losers;
    // Verify sufficient balance in reward pool
    <b>assert</b>!(balance::value(&dispute.reward_pool) &gt;= total_payout, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInsufficientFunds">EInsufficientFunds</a>);
    // Transfer reward to voter
    <b>let</b> reward_coin = coin::from_balance(
        balance::split(&<b>mut</b> dispute.reward_pool, total_payout),
        ctx
    );
    transfer::public_transfer(reward_coin, claimer);
    // Emit reward event
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_VotingRewardClaimedEvent">VotingRewardClaimedEvent</a> {
        dispute_id,
        voter: claimer,
        original_stake: voter_stake,
        reward_amount: reward_from_losers,
        total_payout,
        timestamp,
    });
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_get_threshold_for_media_type"></a>

## Function `get_threshold_for_media_type`

Get similarity threshold for a media type


<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_get_threshold_for_media_type">get_threshold_for_media_type</a>(config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">social_contracts::proof_of_creativity::PoCConfig</a>, media_type: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_get_threshold_for_media_type">get_threshold_for_media_type</a>(config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a>, media_type: u8): u64 {
    <b>if</b> (media_type == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MEDIA_TYPE_IMAGE">MEDIA_TYPE_IMAGE</a>) {
        config.image_threshold
    } <b>else</b> <b>if</b> (media_type == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MEDIA_TYPE_VIDEO">MEDIA_TYPE_VIDEO</a>) {
        config.video_threshold
    } <b>else</b> <b>if</b> (media_type == <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_MEDIA_TYPE_AUDIO">MEDIA_TYPE_AUDIO</a>) {
        config.audio_threshold
    } <b>else</b> {
        <b>abort</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EInvalidMediaType">EInvalidMediaType</a>
    }
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_is_authorized_oracle"></a>

## Function `is_authorized_oracle`

Check if an address is the authorized oracle


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_is_authorized_oracle">is_authorized_oracle</a>(config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">social_contracts::proof_of_creativity::PoCConfig</a>, caller: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_is_authorized_oracle">is_authorized_oracle</a>(config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a>, caller: <b>address</b>): bool {
    caller == config.oracle_address
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_get_registry_stats"></a>

## Function `get_registry_stats`

Get registry statistics


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_get_registry_stats">get_registry_stats</a>(registry: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">social_contracts::proof_of_creativity::PoCRegistry</a>): (u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_get_registry_stats">get_registry_stats</a>(registry: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">PoCRegistry</a>): (u64, u64, u64, u64) {
    (
        registry.total_badges_issued,
        registry.total_redirections_created,
        registry.total_disputes_submitted,
        registry.total_votes_cast
    )
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_has_poc_data"></a>

## Function `has_poc_data`

Check if a post has PoC data that can be disputed


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_has_poc_data">has_poc_data</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_has_poc_data">has_poc_data</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool {
    option::is_some(<a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_to">social_contracts::post::get_revenue_redirect_to</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)) ||
    <a href="../social_contracts/post.md#social_contracts_post_has_poc_badge">social_contracts::post::has_poc_badge</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>)
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_get_dispute_voting_status"></a>

## Function `get_dispute_voting_status`

Get dispute voting status


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_get_dispute_voting_status">get_dispute_voting_status</a>(dispute: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">social_contracts::proof_of_creativity::PoCDispute</a>, current_epoch: u64): (bool, bool, u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_get_dispute_voting_status">get_dispute_voting_status</a>(dispute: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a>, current_epoch: u64): (bool, bool, u8) {
    <b>let</b> voting_active = current_epoch &gt;= dispute.voting_start_epoch && current_epoch &lt;= dispute.voting_end_epoch;
    <b>let</b> voting_ended = current_epoch &gt; dispute.voting_end_epoch;
    (voting_active, voting_ended, dispute.status)
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_get_dispute_stakes"></a>

## Function `get_dispute_stakes`

Get dispute stake totals


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_get_dispute_stakes">get_dispute_stakes</a>(dispute: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">social_contracts::proof_of_creativity::PoCDispute</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_get_dispute_stakes">get_dispute_stakes</a>(dispute: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a>): (u64, u64, u64) {
    (dispute.uphold_stake, dispute.overturn_stake, vector::length(&dispute.votes))
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_has_user_voted"></a>

## Function `has_user_voted`

Check if user has already voted on dispute


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_has_user_voted">has_user_voted</a>(dispute: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">social_contracts::proof_of_creativity::PoCDispute</a>, user: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_has_user_voted">has_user_voted</a>(dispute: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a>, user: <b>address</b>): bool {
    table::contains(&dispute.voter_records, user)
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_config_version"></a>

## Function `config_version`

Get the version of the PoC config


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_config_version">config_version</a>(config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">social_contracts::proof_of_creativity::PoCConfig</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_config_version">config_version</a>(config: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a>): u64 {
    config.version
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_dispute_version"></a>

## Function `dispute_version`

Get the version of a PoC dispute


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_dispute_version">dispute_version</a>(dispute: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">social_contracts::proof_of_creativity::PoCDispute</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_dispute_version">dispute_version</a>(dispute: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a>): u64 {
    dispute.version
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_registry_version"></a>

## Function `registry_version`

Get the version of the PoC registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_registry_version">registry_version</a>(registry: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">social_contracts::proof_of_creativity::PoCRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_registry_version">registry_version</a>(registry: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">PoCRegistry</a>): u64 {
    registry.version
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_migrate_poc_config"></a>

## Function `migrate_poc_config`

Migration function for PoCConfig


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_migrate_poc_config">migrate_poc_config</a>(config: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">social_contracts::proof_of_creativity::PoCConfig</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_migrate_poc_config">migrate_poc_config</a>(
    config: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a>
    <b>assert</b>!(config.version &lt; current_version, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EWrongVersion">EWrongVersion</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = config.version;
    config.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> config_id = object::id(config);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        config_id,
        string::utf8(b"<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCConfig">PoCConfig</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_migrate_poc_dispute"></a>

## Function `migrate_poc_dispute`

Migration function for PoCDispute


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_migrate_poc_dispute">migrate_poc_dispute</a>(dispute: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">social_contracts::proof_of_creativity::PoCDispute</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_migrate_poc_dispute">migrate_poc_dispute</a>(
    dispute: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a>
    <b>assert</b>!(dispute.version &lt; current_version, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EWrongVersion">EWrongVersion</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = dispute.version;
    dispute.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> dispute_id = object::id(dispute);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        dispute_id,
        string::utf8(b"<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCDispute">PoCDispute</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_migrate_poc_registry"></a>

## Function `migrate_poc_registry`

Migration function for PoCRegistry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_migrate_poc_registry">migrate_poc_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">social_contracts::proof_of_creativity::PoCRegistry</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_migrate_poc_registry">migrate_poc_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">PoCRegistry</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a>
    <b>assert</b>!(registry.version &lt; current_version, <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_EWrongVersion">EWrongVersion</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = registry.version;
    registry.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> registry_id = object::id(registry);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        registry_id,
        string::utf8(b"<a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCRegistry">PoCRegistry</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_proof_of_creativity_create_poc_admin_cap"></a>

## Function `create_poc_admin_cap`

Create a PoCAdminCap for bootstrap (package visibility only)
This function is only callable by other modules in the same package


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_create_poc_admin_cap">create_poc_admin_cap</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCAdminCap">social_contracts::proof_of_creativity::PoCAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_create_poc_admin_cap">create_poc_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCAdminCap">PoCAdminCap</a> {
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_proof_of_creativity_PoCAdminCap">PoCAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>
