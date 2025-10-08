---
title: Module `social_contracts::governance`
---

Governance module for the MySocial network
Manages the decentralized governance system with delegate council and community assembly
Implements proposal submission, voting, and execution processes


-  [Struct `GovernanceAdminCap`](#social_contracts_governance_GovernanceAdminCap)
-  [Struct `GovernanceDAO`](#social_contracts_governance_GovernanceDAO)
-  [Struct `Delegate`](#social_contracts_governance_Delegate)
-  [Struct `NominatedDelegate`](#social_contracts_governance_NominatedDelegate)
-  [Struct `Proposal`](#social_contracts_governance_Proposal)
-  [Struct `Vote`](#social_contracts_governance_Vote)
-  [Struct `DelegateElectedEvent`](#social_contracts_governance_DelegateElectedEvent)
-  [Struct `DelegateNominatedEvent`](#social_contracts_governance_DelegateNominatedEvent)
-  [Struct `DelegateVotedEvent`](#social_contracts_governance_DelegateVotedEvent)
-  [Struct `ProposalSubmittedEvent`](#social_contracts_governance_ProposalSubmittedEvent)
-  [Struct `DelegateVoteEvent`](#social_contracts_governance_DelegateVoteEvent)
-  [Struct `CommunityVoteEvent`](#social_contracts_governance_CommunityVoteEvent)
-  [Struct `AnonymousVoteEvent`](#social_contracts_governance_AnonymousVoteEvent)
-  [Struct `ProposalApprovedForVotingEvent`](#social_contracts_governance_ProposalApprovedForVotingEvent)
-  [Struct `ProposalRejectedEvent`](#social_contracts_governance_ProposalRejectedEvent)
-  [Struct `ProposalApprovedEvent`](#social_contracts_governance_ProposalApprovedEvent)
-  [Struct `ProposalRejectedByCommunityEvent`](#social_contracts_governance_ProposalRejectedByCommunityEvent)
-  [Struct `ProposalImplementedEvent`](#social_contracts_governance_ProposalImplementedEvent)
-  [Struct `RewardsDistributedEvent`](#social_contracts_governance_RewardsDistributedEvent)
-  [Struct `VoteDecryptionFailedEvent`](#social_contracts_governance_VoteDecryptionFailedEvent)
-  [Struct `ProposalRescindedEvent`](#social_contracts_governance_ProposalRescindedEvent)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_governance_bootstrap_init)
-  [Function `initialize_registry_tables`](#social_contracts_governance_initialize_registry_tables)
-  [Function `update_governance_parameters`](#social_contracts_governance_update_governance_parameters)
-  [Function `nominate_delegate`](#social_contracts_governance_nominate_delegate)
-  [Function `vote_for_delegate`](#social_contracts_governance_vote_for_delegate)
-  [Function `update_delegate_panel`](#social_contracts_governance_update_delegate_panel)
-  [Function `submit_proposal`](#social_contracts_governance_submit_proposal)
-  [Function `submit_ecosystem_proposal`](#social_contracts_governance_submit_ecosystem_proposal)
-  [Function `submit_proof_of_creativity_proposal`](#social_contracts_governance_submit_proof_of_creativity_proposal)
-  [Function `submit_proposal_internal`](#social_contracts_governance_submit_proposal_internal)
-  [Function `rescind_proposal`](#social_contracts_governance_rescind_proposal)
-  [Function `delegate_vote_on_proposal`](#social_contracts_governance_delegate_vote_on_proposal)
-  [Function `move_to_community_voting_by_id`](#social_contracts_governance_move_to_community_voting_by_id)
-  [Function `reject_proposal_by_id`](#social_contracts_governance_reject_proposal_by_id)
-  [Function `community_vote_on_proposal`](#social_contracts_governance_community_vote_on_proposal)
-  [Function `community_vote_anonymous`](#social_contracts_governance_community_vote_anonymous)
-  [Function `finalize_proposal`](#social_contracts_governance_finalize_proposal)
-  [Function `finalize_proposal_anonymous`](#social_contracts_governance_finalize_proposal_anonymous)
-  [Function `distribute_rewards`](#social_contracts_governance_distribute_rewards)
-  [Function `mark_proposal_implemented`](#social_contracts_governance_mark_proposal_implemented)
-  [Function `get_proposals_by_type`](#social_contracts_governance_get_proposals_by_type)
-  [Function `get_proposals_by_status`](#social_contracts_governance_get_proposals_by_status)
-  [Function `get_delegate_count`](#social_contracts_governance_get_delegate_count)
-  [Function `get_delegate_info`](#social_contracts_governance_get_delegate_info)
-  [Function `get_proposal_info`](#social_contracts_governance_get_proposal_info)
-  [Function `treasury_balance`](#social_contracts_governance_treasury_balance)
-  [Function `calculate_vote_cost`](#social_contracts_governance_calculate_vote_cost)
-  [Function `is_delegate`](#social_contracts_governance_is_delegate)
-  [Function `is_delegate_term_expired`](#social_contracts_governance_is_delegate_term_expired)
-  [Function `get_governance_parameters`](#social_contracts_governance_get_governance_parameters)
-  [Function `reject_proposal_manually`](#social_contracts_governance_reject_proposal_manually)
-  [Function `create_platform_governance`](#social_contracts_governance_create_platform_governance)
-  [Function `version`](#social_contracts_governance_version)
-  [Function `set_version`](#social_contracts_governance_set_version)
-  [Function `migrate_registry`](#social_contracts_governance_migrate_registry)
-  [Function `create_governance_admin_cap`](#social_contracts_governance_create_governance_admin_cap)


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
<b>use</b> <a href="../mys/vec_set.md#mys_vec_set">mys::vec_set</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
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



<a name="social_contracts_governance_GovernanceAdminCap"></a>

## Struct `GovernanceAdminCap`

Admin capability for Governance system management


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">GovernanceAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_governance_GovernanceDAO"></a>

## Struct `GovernanceDAO`

Governance registry that keeps track of all delegates and proposals


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a> <b>has</b> key
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
<code>registry_type: u8</code>
</dt>
<dd>
 Registry type identifier (ecosystem, proof of creativity, platform)
</dd>
<dt>
<code>delegate_count: u64</code>
</dt>
<dd>
 Configuration parameters
</dd>
<dt>
<code>delegate_term_epochs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_submission_cost: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_on_chain_age_days: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_votes_per_user: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quadratic_base_cost: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_period_epochs: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum_votes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegates: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">social_contracts::governance::Delegate</a>&gt;</code>
</dt>
<dd>
 Tables and mappings
</dd>
<dt>
<code>proposals: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>proposals_by_status: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;u8, vector&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>treasury: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>nominated_delegates: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">social_contracts::governance::NominatedDelegate</a>&gt;</code>
</dt>
<dd>
 Treasury for proposal costs and rewards
</dd>
<dt>
<code>delegate_addresses: <a href="../mys/vec_set.md#mys_vec_set_VecSet">mys::vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>nominee_addresses: <a href="../mys/vec_set.md#mys_vec_set_VecSet">mys::vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>voters: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, bool&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_Delegate"></a>

## Struct `Delegate`

Delegate struct representing a member of the delegate council


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><b>address</b>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>upvotes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>downvotes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>proposals_reviewed: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>proposals_submitted: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sided_winning_proposals: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sided_losing_proposals: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>term_start: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>term_end: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_NominatedDelegate"></a>

## Struct `NominatedDelegate`

Nominee struct representing a user nominated but not yet active


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><b>address</b>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>scheduled_term_start_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>upvotes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>downvotes: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_Proposal"></a>

## Struct `Proposal`

Proposal struct representing a governance proposal


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a> <b>has</b> key, store
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
<code>title: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>reference_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>submitter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>submission_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_approval_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_rejection_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>community_votes_for: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>community_votes_against: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_start_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_end_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reward_pool: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_Vote"></a>

## Struct `Vote`

Vote struct for tracking individual votes - unused fields removed


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_Vote">Vote</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_governance_DelegateElectedEvent"></a>

## Struct `DelegateElectedEvent`

Event emitted when a new delegate is elected


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegateElectedEvent">DelegateElectedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>delegate_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>term_start: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>term_end: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>registry_type: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_DelegateNominatedEvent"></a>

## Struct `DelegateNominatedEvent`

Event emitted when a user is nominated to become a delegate in a future term


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegateNominatedEvent">DelegateNominatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>nominee_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>scheduled_term_start_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>registry_type: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_DelegateVotedEvent"></a>

## Struct `DelegateVotedEvent`

Event emitted when a delegate or nominee is voted for/against


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegateVotedEvent">DelegateVotedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>target_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>is_active_delegate: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>upvote: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>new_upvote_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_downvote_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>registry_type: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalSubmittedEvent"></a>

## Struct `ProposalSubmittedEvent`

Event emitted when a proposal is submitted


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalSubmittedEvent">ProposalSubmittedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>title: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>reference_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>submitter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reward_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>submission_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_DelegateVoteEvent"></a>

## Struct `DelegateVoteEvent`

Event emitted when a delegate votes on a proposal


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_DelegateVoteEvent">DelegateVoteEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>approve: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>vote_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reason: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_CommunityVoteEvent"></a>

## Struct `CommunityVoteEvent`

Event emitted when a community member votes on a proposal


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_CommunityVoteEvent">CommunityVoteEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>vote_weight: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>approve: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>vote_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>vote_cost: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_AnonymousVoteEvent"></a>

## Struct `AnonymousVoteEvent`

Event emitted when an anonymous vote is submitted


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_AnonymousVoteEvent">AnonymousVoteEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>vote_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>encrypted_vote_data: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalApprovedForVotingEvent"></a>

## Struct `ProposalApprovedForVotingEvent`

Event emitted when a proposal is approved by the delegate council


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalApprovedForVotingEvent">ProposalApprovedForVotingEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>voting_start_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_end_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalRejectedEvent"></a>

## Struct `ProposalRejectedEvent`

Event emitted when a proposal is rejected by the delegate council


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedEvent">ProposalRejectedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>rejection_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalApprovedEvent"></a>

## Struct `ProposalApprovedEvent`

Event emitted when a proposal is approved by the community


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalApprovedEvent">ProposalApprovedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>approval_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>votes_for: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>votes_against: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalRejectedByCommunityEvent"></a>

## Struct `ProposalRejectedByCommunityEvent`

Event emitted when a proposal is rejected by the community


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedByCommunityEvent">ProposalRejectedByCommunityEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>rejection_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>votes_for: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>votes_against: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_ProposalImplementedEvent"></a>

## Struct `ProposalImplementedEvent`

Event emitted when a proposal is implemented


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalImplementedEvent">ProposalImplementedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>implementation_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_RewardsDistributedEvent"></a>

## Struct `RewardsDistributedEvent`

Event emitted when rewards are distributed to voters


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_RewardsDistributedEvent">RewardsDistributedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>total_reward: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>recipient_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>distribution_time: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_governance_VoteDecryptionFailedEvent"></a>

## Struct `VoteDecryptionFailedEvent`

Event emitted when vote decryption fails


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_VoteDecryptionFailedEvent">VoteDecryptionFailedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>failure_reason: <a href="../std/string.md#std_string_String">std::string::String</a></code>
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

<a name="social_contracts_governance_ProposalRescindedEvent"></a>

## Struct `ProposalRescindedEvent`

Event emitted when a proposal is rescinded by its submitter


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/governance.md#social_contracts_governance_ProposalRescindedEvent">ProposalRescindedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>submitter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>rescind_time: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>refund_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_governance_ANON_VOTERS_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>: vector&lt;u8&gt; = vector[97, 110, 111, 110, 95, 118, 111, 116, 101, 114, 115];
</code></pre>



<a name="social_contracts_governance_DELEGATE_REASONS_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_REASONS_FIELD">DELEGATE_REASONS_FIELD</a>: vector&lt;u8&gt; = vector[100, 101, 108, 101, 103, 97, 116, 101, 95, 114, 101, 97, 115, 111, 110, 115];
</code></pre>



<a name="social_contracts_governance_DELEGATE_VOTES_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTES_FIELD">DELEGATE_VOTES_FIELD</a>: vector&lt;u8&gt; = vector[100, 101, 108, 101, 103, 97, 116, 101, 95, 118, 111, 116, 101, 115];
</code></pre>



<a name="social_contracts_governance_EAlreadyDelegate"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyDelegate">EAlreadyDelegate</a>: u64 = 4;
</code></pre>



<a name="social_contracts_governance_EAlreadyNominated"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyNominated">EAlreadyNominated</a>: u64 = 16;
</code></pre>



<a name="social_contracts_governance_EAlreadyVoted"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyVoted">EAlreadyVoted</a>: u64 = 8;
</code></pre>



<a name="social_contracts_governance_EDelegateAnonNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EDelegateAnonNotAllowed">EDelegateAnonNotAllowed</a>: u64 = 18;
</code></pre>



<a name="social_contracts_governance_EExceedsMaxVotes"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EExceedsMaxVotes">EExceedsMaxVotes</a>: u64 = 13;
</code></pre>



<a name="social_contracts_governance_EInvalidAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidAmount">EInvalidAmount</a>: u64 = 1;
</code></pre>



<a name="social_contracts_governance_EInvalidParameter"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>: u64 = 2;
</code></pre>



<a name="social_contracts_governance_EInvalidProposalStatus"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>: u64 = 7;
</code></pre>



<a name="social_contracts_governance_EInvalidRegistry"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidRegistry">EInvalidRegistry</a>: u64 = 15;
</code></pre>



<a name="social_contracts_governance_EInvalidVoteCount"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidVoteCount">EInvalidVoteCount</a>: u64 = 14;
</code></pre>



<a name="social_contracts_governance_ENCRYPTED_VOTES_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>: vector&lt;u8&gt; = vector[101, 110, 99, 114, 121, 112, 116, 101, 100, 95, 118, 111, 116, 101, 115];
</code></pre>



<a name="social_contracts_governance_ENotDelegate"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>: u64 = 3;
</code></pre>



<a name="social_contracts_governance_ENotVotingPhase"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENotVotingPhase">ENotVotingPhase</a>: u64 = 9;
</code></pre>



<a name="social_contracts_governance_EProposalNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>: u64 = 6;
</code></pre>



<a name="social_contracts_governance_EUnauthorized"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_governance_EVotingPeriodEnded"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodEnded">EVotingPeriodEnded</a>: u64 = 12;
</code></pre>



<a name="social_contracts_governance_EVotingPeriodNotEnded"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodNotEnded">EVotingPeriodNotEnded</a>: u64 = 11;
</code></pre>



<a name="social_contracts_governance_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>: u64 = 17;
</code></pre>



<a name="social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM"></a>

Proposal type constants


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM">PROPOSAL_TYPE_ECOSYSTEM</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_PROPOSAL_TYPE_PLATFORM"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>: u8 = 3;
</code></pre>



<a name="social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_STATUS_APPROVED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_governance_STATUS_COMMUNITY_VOTING"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>: u8 = 2;
</code></pre>



<a name="social_contracts_governance_STATUS_DELEGATE_REVIEW"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>: u8 = 1;
</code></pre>



<a name="social_contracts_governance_STATUS_IMPLEMENTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>: u8 = 5;
</code></pre>



<a name="social_contracts_governance_STATUS_OWNER_RESCIND"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_OWNER_RESCIND">STATUS_OWNER_RESCIND</a>: u8 = 6;
</code></pre>



<a name="social_contracts_governance_STATUS_REJECTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>: u8 = 4;
</code></pre>



<a name="social_contracts_governance_STATUS_SUBMITTED"></a>

Proposal status constants


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_SUBMITTED">STATUS_SUBMITTED</a>: u8 = 0;
</code></pre>



<a name="social_contracts_governance_VOTED_AGAINST_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_AGAINST_FIELD">VOTED_AGAINST_FIELD</a>: vector&lt;u8&gt; = vector[118, 111, 116, 101, 100, 95, 97, 103, 97, 105, 110, 115, 116];
</code></pre>



<a name="social_contracts_governance_VOTED_COMMUNITY_FIELD"></a>

Field names for dynamic fields


<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_COMMUNITY_FIELD">VOTED_COMMUNITY_FIELD</a>: vector&lt;u8&gt; = vector[118, 111, 116, 101, 100, 95, 99, 111, 109, 109, 117, 110, 105, 116, 121];
</code></pre>



<a name="social_contracts_governance_VOTED_DELEGATES_LIST_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>: vector&lt;u8&gt; = vector[118, 111, 116, 101, 100, 95, 100, 101, 108, 101, 103, 97, 116, 101, 115, 95, 108, 105, 115, 116];
</code></pre>



<a name="social_contracts_governance_VOTED_FOR_FIELD"></a>



<pre><code><b>const</b> <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_FOR_FIELD">VOTED_FOR_FIELD</a>: vector&lt;u8&gt; = vector[118, 111, 116, 101, 100, 95, 102, 111, 114];
</code></pre>



<a name="social_contracts_governance_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the governance registries
This function has the same logic as init() but can be called by bootstrap


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    // Create MySocial Ecosystem Governance Registry
    <b>let</b> <b>mut</b> ecosystem_registry = <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a> {
        id: object::new(ctx),
        registry_type: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM">PROPOSAL_TYPE_ECOSYSTEM</a>,
        // Configuration parameters specific to ecosystem <a href="../social_contracts/governance.md#social_contracts_governance">governance</a>
        delegate_count: 3, // Larger council <b>for</b> ecosystem decisions
        delegate_term_epochs: 90, // 3 months <b>for</b> ecosystem delegates
        proposal_submission_cost: 100_000_000, // 100 MYS <b>for</b> ecosystem proposals
        min_on_chain_age_days: 30, // 1 month minimum <b>for</b> ecosystem voting
        max_votes_per_user: 10, // Up to 10 votes per user
        quadratic_base_cost: 10_000_000, // 10 MYS per additional vote
        voting_period_epochs: 7, // 7 epochs <b>for</b> ecosystem votes
        quorum_votes: 20, // 20 votes required <b>for</b> ecosystem proposals
        // Tables
        delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a>&gt;(ctx),
        proposals: table::new&lt;ID, <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>&gt;(ctx),
        proposals_by_status: table::new&lt;u8, vector&lt;ID&gt;&gt;(ctx),
        treasury: balance::zero(),
        nominated_delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a>&gt;(ctx),
        delegate_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        nominee_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        voters: table::new&lt;<b>address</b>, Table&lt;<b>address</b>, bool&gt;&gt;(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Create Proof of Creativity Governance Registry
    <b>let</b> <b>mut</b> proof_of_creativity_registry = <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a> {
        id: object::new(ctx),
        registry_type: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>,
        // Configuration parameters specific to proof of creativity <a href="../social_contracts/governance.md#social_contracts_governance">governance</a>
        delegate_count: 2, // Smaller council <b>for</b> proof of creativity
        delegate_term_epochs: 180, // 3 months <b>for</b> proof of creativity delegates
        proposal_submission_cost: 25_000_000, // 25 MYS <b>for</b> proof of creativity
        min_on_chain_age_days: 1, // 1 day minimum <b>for</b> proof of creativity voting
        max_votes_per_user: 3, // Up to 3 votes per user
        quadratic_base_cost: 2_500_000, // 2.5 MYS per additional vote
        voting_period_epochs: 1, // 1 epoch <b>for</b> proof of creativity votes
        quorum_votes: 10, // 10 votes required <b>for</b> proof of creativity proposals
        // Tables
        delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a>&gt;(ctx),
        proposals: table::new&lt;ID, <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>&gt;(ctx),
        proposals_by_status: table::new&lt;u8, vector&lt;ID&gt;&gt;(ctx),
        treasury: balance::zero(),
        nominated_delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a>&gt;(ctx),
        delegate_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        nominee_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        voters: table::new&lt;<b>address</b>, Table&lt;<b>address</b>, bool&gt;&gt;(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Initialize each registry's status tables
    <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(&<b>mut</b> ecosystem_registry, ctx);
    <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(&<b>mut</b> proof_of_creativity_registry, ctx);
    // Share the registry objects
    transfer::share_object(ecosystem_registry);
    transfer::share_object(proof_of_creativity_registry);
}
</code></pre>



</details>

<a name="social_contracts_governance_initialize_registry_tables"></a>

## Function `initialize_registry_tables`



<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, _ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, _ctx: &<b>mut</b> TxContext) {
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_SUBMITTED">STATUS_SUBMITTED</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>, vector::empty&lt;ID&gt;());
    table::add(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_OWNER_RESCIND">STATUS_OWNER_RESCIND</a>, vector::empty&lt;ID&gt;());
}
</code></pre>



</details>

<a name="social_contracts_governance_update_governance_parameters"></a>

## Function `update_governance_parameters`

Update governance parameters
Can only be called by the governance admin


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_governance_parameters">update_governance_parameters</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, _: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">social_contracts::governance::GovernanceAdminCap</a>, delegate_count: u64, delegate_term_epochs: u64, proposal_submission_cost: u64, min_on_chain_age_days: u64, max_votes_per_user: u64, quadratic_base_cost: u64, voting_period_epochs: u64, quorum_votes: u64, _ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_governance_parameters">update_governance_parameters</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    _: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">GovernanceAdminCap</a>,
    delegate_count: u64,
    delegate_term_epochs: u64,
    proposal_submission_cost: u64,
    min_on_chain_age_days: u64,
    max_votes_per_user: u64,
    quadratic_base_cost: u64,
    voting_period_epochs: u64,
    quorum_votes: u64,
    _ctx: &<b>mut</b> TxContext
) {
    // Admin capability verification is handled by type system
    // Ensure parameters are sensible
    <b>assert</b>!(delegate_count &gt; 1, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    <b>assert</b>!(delegate_term_epochs &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    // proposal_submission_cost can be 0
    // min_on_chain_age_days can be 0
    <b>assert</b>!(max_votes_per_user &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    // quadratic_base_cost can be 0 (<b>if</b> voting is free)
    <b>assert</b>!(voting_period_epochs &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    <b>assert</b>!(quorum_votes &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    // Update parameters
    registry.delegate_count = delegate_count;
    registry.delegate_term_epochs = delegate_term_epochs;
    registry.proposal_submission_cost = proposal_submission_cost;
    registry.min_on_chain_age_days = min_on_chain_age_days;
    registry.max_votes_per_user = max_votes_per_user;
    registry.quadratic_base_cost = quadratic_base_cost;
    registry.voting_period_epochs = voting_period_epochs;
    registry.quorum_votes = quorum_votes;
}
</code></pre>



</details>

<a name="social_contracts_governance_nominate_delegate"></a>

## Function `nominate_delegate`

Nominate self as a delegate
Requires the caller to have a valid profile


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_nominate_delegate">nominate_delegate</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_nominate_delegate">nominate_delegate</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    profile_registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">profile::UsernameRegistry</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    // Check <b>if</b> already a delegate or nominee delegate
    <b>assert</b>!(!table::contains(&registry.delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyDelegate">EAlreadyDelegate</a>);
    <b>assert</b>!(!table::contains(&registry.nominated_delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyNominated">EAlreadyNominated</a>);
    <b>let</b> <b>mut</b> profile_id_opt = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">profile::lookup_profile_by_owner</a>(profile_registry, caller);
    <b>assert</b>!(option::is_some(&profile_id_opt), <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>);
    // Get <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID
    <b>let</b> profile_id_addr = option::extract(&<b>mut</b> profile_id_opt);
    <b>let</b> profile_id = object::id_from_address(profile_id_addr);
    <b>let</b> scheduled_term_start_epoch = ((current_epoch / registry.delegate_term_epochs) + 1) * registry.delegate_term_epochs;
    <b>let</b> nominated_delegate = <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a> {
        <b>address</b>: caller,
        profile_id,
        scheduled_term_start_epoch,
        upvotes: 0,
        downvotes: 0,
    };
    table::add(&<b>mut</b> registry.nominated_delegates, caller, nominated_delegate);
    vec_set::insert(&<b>mut</b> registry.nominee_addresses, caller);
    // Initialize voter tracking table <b>for</b> this nominee
    table::add(&<b>mut</b> registry.voters, caller, table::new&lt;<b>address</b>, bool&gt;(ctx));
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateNominatedEvent">DelegateNominatedEvent</a> {
        nominee_address: caller,
        profile_id,
        scheduled_term_start_epoch,
        registry_type: registry.registry_type,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_vote_for_delegate"></a>

## Function `vote_for_delegate`

Vote for or against a delegate or nominee delegate
Positive votes support the delegate, negative votes express disapproval
Users can change their vote at any time


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_for_delegate">vote_for_delegate</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, target_address: <b>address</b>, upvote: bool, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_vote_for_delegate">vote_for_delegate</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    target_address: <b>address</b>,
    upvote: bool,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    // Don't allow self-voting
    <b>assert</b>!(caller != target_address, <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>);
    // Variables <b>for</b> event emission
    <b>let</b> is_active_delegate: bool;
    <b>let</b> upvote_count: u64;
    <b>let</b> downvote_count: u64;
    // Handle voting <b>for</b> active delegates
    <b>if</b> (table::contains(&registry.delegates, target_address)) {
        is_active_delegate = <b>true</b>;
        <b>let</b> delegate = table::borrow_mut(&<b>mut</b> registry.delegates, target_address);
        // Create voter table <b>for</b> this target <b>if</b> it doesn't exist
        <b>if</b> (!table::contains(&registry.voters, target_address)) {
            table::add(&<b>mut</b> registry.voters, target_address, table::new&lt;<b>address</b>, bool&gt;(ctx));
        };
        <b>let</b> voter_table = table::borrow_mut(&<b>mut</b> registry.voters, target_address);
        // Check <b>if</b> user <b>has</b> already voted
        <b>if</b> (table::contains(voter_table, caller)) {
            <b>let</b> previous_vote = *table::borrow(voter_table, caller);
            // If same vote, do nothing
            <b>if</b> (previous_vote == upvote) {
                <b>return</b>
            };
            // Different vote - remove previous vote
            <b>if</b> (previous_vote) {
                delegate.upvotes = delegate.upvotes - 1;
            } <b>else</b> {
                delegate.downvotes = delegate.downvotes - 1;
            };
            // Add new vote
            <b>if</b> (upvote) {
                delegate.upvotes = delegate.upvotes + 1;
            } <b>else</b> {
                delegate.downvotes = delegate.downvotes + 1;
            };
            // Update record
            *table::borrow_mut(voter_table, caller) = upvote;
        } <b>else</b> {
            // First time voting <b>for</b> this target
            <b>if</b> (upvote) {
                delegate.upvotes = delegate.upvotes + 1;
            } <b>else</b> {
                delegate.downvotes = delegate.downvotes + 1;
            };
            // Record vote
            table::add(voter_table, caller, upvote);
        };
        upvote_count = delegate.upvotes;
        downvote_count = delegate.downvotes;
    }
    // Handle voting <b>for</b> nominee delegates
    <b>else</b> <b>if</b> (table::contains(&registry.nominated_delegates, target_address)) {
        is_active_delegate = <b>false</b>;
        <b>let</b> nominee = table::borrow_mut(&<b>mut</b> registry.nominated_delegates, target_address);
        // Create voter table <b>for</b> this target <b>if</b> it doesn't exist
        <b>if</b> (!table::contains(&registry.voters, target_address)) {
            table::add(&<b>mut</b> registry.voters, target_address, table::new&lt;<b>address</b>, bool&gt;(ctx));
        };
        <b>let</b> voter_table = table::borrow_mut(&<b>mut</b> registry.voters, target_address);
        // Check <b>if</b> user <b>has</b> already voted
        <b>if</b> (table::contains(voter_table, caller)) {
            <b>let</b> previous_vote = *table::borrow(voter_table, caller);
            // If same vote, do nothing
            <b>if</b> (previous_vote == upvote) {
                <b>return</b>
            };
            // Different vote - remove previous vote
            <b>if</b> (previous_vote) {
                nominee.upvotes = nominee.upvotes - 1;
            } <b>else</b> {
                nominee.downvotes = nominee.downvotes - 1;
            };
            // Add new vote
            <b>if</b> (upvote) {
                nominee.upvotes = nominee.upvotes + 1;
            } <b>else</b> {
                nominee.downvotes = nominee.downvotes + 1;
            };
            // Update record
            *table::borrow_mut(voter_table, caller) = upvote;
        } <b>else</b> {
            // First time voting <b>for</b> this nominee
            <b>if</b> (upvote) {
                nominee.upvotes = nominee.upvotes + 1;
            } <b>else</b> {
                nominee.downvotes = nominee.downvotes + 1;
            };
            // Record vote
            table::add(voter_table, caller, upvote);
        };
        upvote_count = nominee.upvotes;
        downvote_count = nominee.downvotes;
    }
    <b>else</b> { <b>abort</b> <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a> };
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateVotedEvent">DelegateVotedEvent</a> {
        target_address,
        voter: caller,
        is_active_delegate,
        upvote,
        new_upvote_count: upvote_count,
        new_downvote_count: downvote_count,
        registry_type: registry.registry_type,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_update_delegate_panel"></a>

## Function `update_delegate_panel`

Updates delegate panel at the end of a delegate term cycle.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_delegate_panel">update_delegate_panel</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_update_delegate_panel">update_delegate_panel</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    <b>let</b> delegate_term_epochs = registry.delegate_term_epochs;
    <b>assert</b>!(delegate_term_epochs &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    <b>if</b> (current_epoch == 0 || current_epoch % delegate_term_epochs != 0) {
        <b>return</b>
    };
    // --- Gather Candidates Data ---
    <b>let</b> <b>mut</b> candidate_addresses = vector::empty&lt;<b>address</b>&gt;();
    <b>let</b> <b>mut</b> candidate_profile_ids = vector::empty&lt;ID&gt;();
    <b>let</b> <b>mut</b> candidate_upvotes = vector::empty&lt;u64&gt;();
    <b>let</b> <b>mut</b> candidate_downvotes = vector::empty&lt;u64&gt;();
    <b>let</b> <b>mut</b> candidate_net_votes = vector::empty&lt;u64&gt;(); // For sorting calculations
    <b>let</b> <b>mut</b> candidate_is_incumbent = vector::empty&lt;bool&gt;(); // Track incumbent status <b>for</b> tie-breaking
    // Use 1000 <b>as</b> the baseline score where upvotes and downvotes are equal
    <b>let</b> baseline_score: u64 = 1000;
    // 1. Gather Active Delegates (using the tracking set)
    <b>let</b> active_delegate_keys_vec = vec_set::into_keys(registry.delegate_addresses); // Consumes the set
    <b>let</b> len_active = vector::length(&active_delegate_keys_vec);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len_active) {
        <b>let</b> addr = *vector::borrow(&active_delegate_keys_vec, i);
        // Need to borrow from the actual table to get full data
        <b>if</b> (table::contains(&registry.delegates, addr)) { // Check existence before borrow
            <b>let</b> delegate: &<a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a> = table::borrow(&registry.delegates, addr);
            vector::push_back(&<b>mut</b> candidate_addresses, addr);
            vector::push_back(&<b>mut</b> candidate_profile_ids, delegate.profile_id);
            vector::push_back(&<b>mut</b> candidate_upvotes, delegate.upvotes);
            vector::push_back(&<b>mut</b> candidate_downvotes, delegate.downvotes);
            vector::push_back(&<b>mut</b> candidate_is_incumbent, <b>true</b>); // Mark <b>as</b> incumbent
            // Calculate net votes with baseline offset (allows negative effective scores)
            <b>let</b> net_votes = <b>if</b> (delegate.upvotes &gt;= delegate.downvotes) {
                // If more upvotes than downvotes, add the difference to baseline
                baseline_score + (delegate.upvotes - delegate.downvotes)
            } <b>else</b> {
                // If more downvotes than upvotes, subtract the difference from baseline
                // But make sure we don't underflow
                <b>if</b> (delegate.downvotes - delegate.upvotes &gt;= baseline_score) {
                    0 // Minimum score
                } <b>else</b> {
                    baseline_score - (delegate.downvotes - delegate.upvotes)
                }
            };
            vector::push_back(&<b>mut</b> candidate_net_votes, net_votes);
        };
        // Note: If delegate wasn't found in table (somehow desynced), it's skipped.
        i = i + 1;
    };
    // Recreate the delegate_addresses set <b>as</b> empty, it will be repopulated with winners
    registry.delegate_addresses = vec_set::empty&lt;<b>address</b>&gt;();
    // 2. Gather Nominated Delegates (using the tracking set)
    <b>let</b> nominee_keys_vec = vec_set::into_keys(registry.nominee_addresses); // Consumes the set
    <b>let</b> len_nominated = vector::length(&nominee_keys_vec);
    <b>let</b> <b>mut</b> j = 0;
    <b>while</b> (j &lt; len_nominated) {
        <b>let</b> addr = *vector::borrow(&nominee_keys_vec, j);
        // Check <b>if</b> it was already added <b>as</b> an active delegate (shouldn't happen <b>if</b> sets are correct)
        // We can skip this check <b>if</b> we ensure sets are managed correctly.
        // Need to borrow from the actual table to get full data
        <b>if</b> (table::contains(&registry.nominated_delegates, addr)) { // Check existence
            <b>let</b> nominee: &<a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a> = table::borrow(&registry.nominated_delegates, addr);
            vector::push_back(&<b>mut</b> candidate_addresses, addr);
            vector::push_back(&<b>mut</b> candidate_profile_ids, nominee.profile_id);
            vector::push_back(&<b>mut</b> candidate_upvotes, nominee.upvotes);
            vector::push_back(&<b>mut</b> candidate_downvotes, nominee.downvotes);
            vector::push_back(&<b>mut</b> candidate_is_incumbent, <b>false</b>); // Mark <b>as</b> non-incumbent
            // Calculate net votes with baseline offset (allows negative effective scores)
            <b>let</b> net_votes = <b>if</b> (nominee.upvotes &gt;= nominee.downvotes) {
                // If more upvotes than downvotes, add the difference to baseline
                baseline_score + (nominee.upvotes - nominee.downvotes)
            } <b>else</b> {
                // If more downvotes than upvotes, subtract the difference from baseline
                // But make sure we don't underflow
                <b>if</b> (nominee.downvotes - nominee.upvotes &gt;= baseline_score) {
                    0 // Minimum score
                } <b>else</b> {
                    baseline_score - (nominee.downvotes - nominee.upvotes)
                }
            };
            vector::push_back(&<b>mut</b> candidate_net_votes, net_votes);
        };
        j = j + 1;
    };
    // Recreate the nominee_addresses set <b>as</b> empty, losers will not be re-added.
    registry.nominee_addresses = vec_set::empty&lt;<b>address</b>&gt;();
    // --- Sort Candidate Data by net votes (higher is better) ---
    <b>let</b> candidate_count = vector::length(&candidate_addresses);
    <b>let</b> <b>mut</b> a = 0;
    <b>while</b> (a &lt; candidate_count) {
        <b>let</b> <b>mut</b> b = a + 1;
        <b>while</b> (b &lt; candidate_count) {
            // Compare net votes
            <b>let</b> a_votes = *vector::borrow(&candidate_net_votes, a);
            <b>let</b> b_votes = *vector::borrow(&candidate_net_votes, b);
            // If b <b>has</b> more net votes than a, or they're tied but b is an incumbent and a is not, swap them
            <b>let</b> should_swap = <b>if</b> (b_votes &gt; a_votes) {
                <b>true</b>
            } <b>else</b> <b>if</b> (b_votes == a_votes) {
                // Tie-breaking: prefer incumbent delegates
                <b>let</b> a_incumbent = *vector::borrow(&candidate_is_incumbent, a);
                <b>let</b> b_incumbent = *vector::borrow(&candidate_is_incumbent, b);
                // Only swap <b>if</b> b is incumbent and a is not
                b_incumbent && !a_incumbent
            } <b>else</b> {
                <b>false</b>
            };
            <b>if</b> (should_swap) {
                // Swap addresses
                <b>let</b> temp_addr = *vector::borrow(&candidate_addresses, a);
                *vector::borrow_mut(&<b>mut</b> candidate_addresses, a) = *vector::borrow(&candidate_addresses, b);
                *vector::borrow_mut(&<b>mut</b> candidate_addresses, b) = temp_addr;
                // Swap <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> IDs
                <b>let</b> temp_id = *vector::borrow(&candidate_profile_ids, a);
                *vector::borrow_mut(&<b>mut</b> candidate_profile_ids, a) = *vector::borrow(&candidate_profile_ids, b);
                *vector::borrow_mut(&<b>mut</b> candidate_profile_ids, b) = temp_id;
                // Swap upvotes
                <b>let</b> temp_up = *vector::borrow(&candidate_upvotes, a);
                *vector::borrow_mut(&<b>mut</b> candidate_upvotes, a) = *vector::borrow(&candidate_upvotes, b);
                *vector::borrow_mut(&<b>mut</b> candidate_upvotes, b) = temp_up;
                // Swap downvotes
                <b>let</b> temp_down = *vector::borrow(&candidate_downvotes, a);
                *vector::borrow_mut(&<b>mut</b> candidate_downvotes, a) = *vector::borrow(&candidate_downvotes, b);
                *vector::borrow_mut(&<b>mut</b> candidate_downvotes, b) = temp_down;
                // Swap net votes
                <b>let</b> temp_net = *vector::borrow(&candidate_net_votes, a);
                *vector::borrow_mut(&<b>mut</b> candidate_net_votes, a) = *vector::borrow(&candidate_net_votes, b);
                *vector::borrow_mut(&<b>mut</b> candidate_net_votes, b) = temp_net;
                // Swap incumbent status
                <b>let</b> temp_inc = *vector::borrow(&candidate_is_incumbent, a);
                *vector::borrow_mut(&<b>mut</b> candidate_is_incumbent, a) = *vector::borrow(&candidate_is_incumbent, b);
                *vector::borrow_mut(&<b>mut</b> candidate_is_incumbent, b) = temp_inc;
            };
            b = b + 1;
        };
        a = a + 1;
    };
    // --- Select Winners and Update State ---
    <b>let</b> num_winners_to_select = registry.delegate_count;
    <b>let</b> actual_candidate_count = vector::length(&candidate_addresses);
    <b>let</b> final_winner_count = <b>if</b> (actual_candidate_count &lt; num_winners_to_select) { actual_candidate_count } <b>else</b> { num_winners_to_select };
    // --- Cleanup Old State ---
    // 1. Remove *all* previously active delegates from the table
    // We iterate using the keys vector we got earlier
    <b>let</b> <b>mut</b> k = 0;
    <b>while</b> (k &lt; len_active) {
        <b>let</b> addr = *vector::borrow(&active_delegate_keys_vec, k);
        // Check <b>if</b> it exists before removing (might have been removed <b>if</b> somehow duplicated)
        <b>if</b> (table::contains(&registry.delegates, addr)) {
             <b>let</b> old_delegate = table::remove(&<b>mut</b> registry.delegates, addr);
             <b>let</b> <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a> { <b>address</b>: _, profile_id: _, upvotes: _, downvotes: _, proposals_reviewed: _, proposals_submitted: _, sided_winning_proposals: _, sided_losing_proposals: _, term_start: _, term_end: _ } = old_delegate;
        };
        k = k + 1;
    };
    // 2. Remove *all* previously nominated delegates from the table
    <b>let</b> <b>mut</b> l = 0;
    <b>while</b> (l &lt; len_nominated) {
        <b>let</b> addr = *vector::borrow(&nominee_keys_vec, l);
        <b>if</b> (table::contains(&registry.nominated_delegates, addr)) {
            <b>let</b> old_nominee = table::remove(&<b>mut</b> registry.nominated_delegates, addr);
            <b>let</b> <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a> { <b>address</b>: _, profile_id: _, scheduled_term_start_epoch: _, upvotes: _, downvotes: _ } = old_nominee;
         };
         l = l + 1;
    };
    // --- Add Winners to the Delegates Table ---
    // The delegates table should now be empty. Add the winners.
    <b>let</b> <b>mut</b> m = 0;
    <b>while</b> (m &lt; final_winner_count) {
        <b>let</b> winner_addr = *vector::borrow(&candidate_addresses, m);
        <b>let</b> winner_profile_id = *vector::borrow(&candidate_profile_ids, m);
        <b>let</b> winner_upvotes = *vector::borrow(&candidate_upvotes, m);
        <b>let</b> winner_downvotes = *vector::borrow(&candidate_downvotes, m);
        <b>let</b> term_start = current_epoch;
        <b>let</b> term_end = term_start + delegate_term_epochs;
        <b>let</b> new_delegate = <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a> {
            <b>address</b>: winner_addr,
            profile_id: winner_profile_id,
            upvotes: winner_upvotes,
            downvotes: winner_downvotes,
            proposals_reviewed: 0, // Reset counters
            proposals_submitted: 0,
            sided_winning_proposals: 0,
            sided_losing_proposals: 0,
            term_start: term_start,
            term_end: term_end,
        };
        // Add winner to the (previously emptied) delegates table
        table::add(&<b>mut</b> registry.delegates, winner_addr, new_delegate);
        // Add winner's <b>address</b> back to the tracking set
        vec_set::insert(&<b>mut</b> registry.delegate_addresses, winner_addr);
        event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateElectedEvent">DelegateElectedEvent</a> {
            delegate_address: winner_addr,
            profile_id: winner_profile_id,
            term_start: term_start,
            term_end: term_end,
            registry_type: registry.registry_type,
        });
        m = m + 1;
    };
    // Destroy helper vectors used <b>for</b> candidate data
    vector::destroy_empty(candidate_addresses);
    vector::destroy_empty(candidate_profile_ids);
    vector::destroy_empty(candidate_upvotes);
    vector::destroy_empty(candidate_downvotes);
    vector::destroy_empty(candidate_net_votes);
    vector::destroy_empty(candidate_is_incumbent);
    // Destroy key vectors obtained from into_keys
    vector::destroy_empty(active_delegate_keys_vec);
    vector::destroy_empty(nominee_keys_vec);
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_proposal"></a>

## Function `submit_proposal`

Universal function to submit any type of proposal
Handles proposal types: ecosystem and proof of creativity


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal">submit_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_type: u8, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, disputed_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;, reference_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal">submit_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_type: u8,
    title: String,
    description: String,
    disputed_id: Option&lt;ID&gt;, // Optional ID <b>for</b> disputes (<a href="../social_contracts/profile.md#social_contracts_profile">profile</a> or content)
    reference_id: Option&lt;ID&gt;, // Optional reference
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Verify proposal type is valid
    <b>assert</b>!(proposal_type &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    // Verify registry type matches proposal type
    <b>assert</b>!(registry.registry_type == proposal_type, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidRegistry">EInvalidRegistry</a>);
    // Handle reference ID based on proposal type
    <b>let</b> actual_reference_id = <b>if</b> (proposal_type == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM">PROPOSAL_TYPE_ECOSYSTEM</a>) {
        // Ecosystem proposals <b>use</b> reference_id <b>as</b> provided
        reference_id
    } <b>else</b> <b>if</b> (proposal_type == <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>) {
        // Proof of creativity proposals should have either reference_id or disputed_id
        <b>if</b> (option::is_some(&reference_id)) {
            reference_id
        } <b>else</b> <b>if</b> (option::is_some(&disputed_id)) {
            disputed_id
        } <b>else</b> {
            // Proof of creativity proposals should reference creative content
            <b>assert</b>!(<b>false</b>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
            option::none&lt;ID&gt;()
        }
    } <b>else</b> {
        // For other proposal types (like <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>), <b>use</b> reference_id <b>if</b> provided
        reference_id
    };
    // Submit the proposal using the internal implementation
    <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal_internal">submit_proposal_internal</a>(
        registry,
        title,
        description,
        proposal_type,
        actual_reference_id,
        metadata_json,
        coin,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_ecosystem_proposal"></a>

## Function `submit_ecosystem_proposal`

Submit a new proposal to the ecosystem registry
Requires staking MYS tokens equal to the proposal submission cost


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_ecosystem_proposal">submit_ecosystem_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, reference_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_ecosystem_proposal">submit_ecosystem_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    title: String,
    description: String,
    reference_id: Option&lt;ID&gt;,
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal">submit_proposal</a>(
        registry,
        <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_ECOSYSTEM">PROPOSAL_TYPE_ECOSYSTEM</a>,
        title,
        description,
        option::none&lt;ID&gt;(), // No disputed ID <b>for</b> ecosystem proposals
        reference_id,
        metadata_json,
        coin,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_proof_of_creativity_proposal"></a>

## Function `submit_proof_of_creativity_proposal`

Submit a proof of creativity proposal


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proof_of_creativity_proposal">submit_proof_of_creativity_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, creative_content_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proof_of_creativity_proposal">submit_proof_of_creativity_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    title: String,
    description: String,
    creative_content_id: ID,
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal">submit_proposal</a>(
        registry,
        <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PROOF_OF_CREATIVITY">PROPOSAL_TYPE_PROOF_OF_CREATIVITY</a>,
        title,
        description,
        option::none&lt;ID&gt;(), // No disputed ID <b>for</b> proof of creativity
        option::some(creative_content_id), // Reference to creative content
        metadata_json,
        coin,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_governance_submit_proposal_internal"></a>

## Function `submit_proposal_internal`

Internal function for submitting proposals


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal_internal">submit_proposal_internal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, title: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, proposal_type: u8, reference_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_submit_proposal_internal">submit_proposal_internal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    title: String,
    description: String,
    proposal_type: u8,
    reference_id: Option&lt;ID&gt;,
    metadata_json: Option&lt;String&gt;,
    coin: &<b>mut</b> Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    // Check stake amount
    <b>assert</b>!(coin::value(coin) &gt;= registry.proposal_submission_cost, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidAmount">EInvalidAmount</a>);
    // Create proposal
    <b>let</b> proposal_id = object::new(ctx);
    <b>let</b> <b>mut</b> proposal = <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a> {
        id: proposal_id,
        title: title,
        description: description,
        proposal_type: proposal_type,
        reference_id: reference_id,
        metadata_json: metadata_json,
        submitter: caller,
        submission_time: current_time,
        delegate_approval_count: 0,
        delegate_rejection_count: 0,
        community_votes_for: 0,
        community_votes_against: 0,
        status: <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>,
        voting_start_time: 0,
        voting_end_time: 0,
        reward_pool: balance::zero(),
    };
    // Split coin and add to proposal's reward pool
    <b>let</b> proposal_coin = coin::split(coin, registry.proposal_submission_cost, ctx);
    balance::join(&<b>mut</b> proposal.reward_pool, coin::into_balance(proposal_coin));
    // Initialize dynamic fields <b>for</b> vote tracking
    <b>let</b> delegate_votes = table::new&lt;<b>address</b>, bool&gt;(ctx);
    dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTES_FIELD">DELEGATE_VOTES_FIELD</a>, delegate_votes);
    <b>let</b> voted_community = vec_set::empty&lt;<b>address</b>&gt;();
    dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_COMMUNITY_FIELD">VOTED_COMMUNITY_FIELD</a>, voted_community);
    // Add "voted <b>for</b>" and "voted against" tracking <b>for</b> easier reward distribution
    <b>let</b> voted_for = vec_set::empty&lt;<b>address</b>&gt;();
    dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_FOR_FIELD">VOTED_FOR_FIELD</a>, voted_for);
    <b>let</b> voted_against = vec_set::empty&lt;<b>address</b>&gt;();
    dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_AGAINST_FIELD">VOTED_AGAINST_FIELD</a>, voted_against);
    // Add proposal to registry
    <b>let</b> proposal_id_copy = object::id(&proposal);
    table::add(&<b>mut</b> registry.proposals, proposal_id_copy, proposal);
    // Add to proposals by status
    <b>let</b> proposals_of_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>);
    vector::push_back(proposals_of_status, proposal_id_copy);
    // Increment proposal count <b>for</b> delegate <b>if</b> applicable
    <b>if</b> (table::contains(&registry.delegates, caller)) {
        <b>let</b> delegate = table::borrow_mut(&<b>mut</b> registry.delegates, caller);
        delegate.proposals_submitted = delegate.proposals_submitted + 1;
    };
    // Emit event
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalSubmittedEvent">ProposalSubmittedEvent</a> {
        proposal_id: proposal_id_copy,
        title: title,
        description: description,
        proposal_type: proposal_type,
        reference_id: reference_id,
        metadata_json: metadata_json,
        submitter: caller,
        reward_amount: registry.proposal_submission_cost,
        submission_time: current_time,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_rescind_proposal"></a>

## Function `rescind_proposal`

Allow a proposal owner to rescind their proposal if it's still in the delegate review stage


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_rescind_proposal">rescind_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_rescind_proposal">rescind_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    // Verify proposal exists and is in delegate review phase
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>let</b> proposal = table::borrow(&registry.proposals, proposal_id);
    // Verify caller is the proposal submitter
    <b>assert</b>!(proposal.submitter == caller, <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>);
    // Verify proposal is still in delegate review stage
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    // Remove proposal from the registry to modify it
    <b>let</b> <b>mut</b> proposal = table::remove(&<b>mut</b> registry.proposals, proposal_id);
    // Update proposals by status - remove from delegate review
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    // Return staked tokens to submitter
    <b>let</b> refund_amount = balance::value(&proposal.reward_pool);
    <b>if</b> (refund_amount &gt; 0) {
        <b>let</b> refund_coin = coin::from_balance(balance::withdraw_all(&<b>mut</b> proposal.reward_pool), ctx);
        transfer::public_transfer(refund_coin, caller);
    } <b>else</b> {
        // Empty the balance even <b>if</b> zero, <b>for</b> consistency
        balance::destroy_zero(balance::withdraw_all(&<b>mut</b> proposal.reward_pool));
    };
    // Emit event <b>for</b> the rescinded proposal
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRescindedEvent">ProposalRescindedEvent</a> {
        proposal_id,
        submitter: caller,
        rescind_time: current_time,
        refund_amount,
    });
    // Update proposal status to owner rescinded
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_OWNER_RESCIND">STATUS_OWNER_RESCIND</a>;
    // Add to rejected proposals list (we still <b>use</b> the rejected table <b>for</b> tracking)
    <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_OWNER_RESCIND">STATUS_OWNER_RESCIND</a>);
    vector::push_back(to_status, proposal_id);
    // Add the modified proposal back to the registry
    table::add(&<b>mut</b> registry.proposals, proposal_id, proposal);
}
</code></pre>



</details>

<a name="social_contracts_governance_delegate_vote_on_proposal"></a>

## Function `delegate_vote_on_proposal`

Delegate votes on a proposal if it should move to community voting


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_delegate_vote_on_proposal">delegate_vote_on_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, approve: bool, reason: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_delegate_vote_on_proposal">delegate_vote_on_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_id: ID,
    approve: bool,
    <b>mut</b> reason: Option&lt;String&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    // Verify caller is a delegate
    <b>assert</b>!(table::contains(&registry.delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>);
    // Verify proposal exists and is in delegate review phase
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>let</b> proposal = table::borrow_mut(&<b>mut</b> registry.proposals, proposal_id);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    // Check <b>if</b> delegate <b>has</b> already voted
    <b>let</b> delegate_votes: &<b>mut</b> Table&lt;<b>address</b>, bool&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTES_FIELD">DELEGATE_VOTES_FIELD</a>);
    <b>assert</b>!(!table::contains(delegate_votes, caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyVoted">EAlreadyVoted</a>);
    // Record vote
    table::add(delegate_votes, caller, approve);
    // Store delegate's reason <b>if</b> provided
    <b>if</b> (option::is_some(&reason)) {
        // Initialize reason table <b>if</b> it doesn't exist
        <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_REASONS_FIELD">DELEGATE_REASONS_FIELD</a>)) {
            <b>let</b> reason_table = table::new&lt;<b>address</b>, String&gt;(ctx);
            dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_REASONS_FIELD">DELEGATE_REASONS_FIELD</a>, reason_table);
        };
        // Add delegate's reason to the table
        <b>let</b> reason_table: &<b>mut</b> Table&lt;<b>address</b>, String&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_REASONS_FIELD">DELEGATE_REASONS_FIELD</a>);
        table::add(reason_table, caller, option::extract(&<b>mut</b> reason));
    };
    // Add delegate to the list of voted delegates (<b>for</b> later tracking)
    <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>)) {
        <b>let</b> delegates_list = vector::empty&lt;<b>address</b>&gt;();
        dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>, delegates_list);
    };
    <b>let</b> delegates_list: &<b>mut</b> vector&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>);
    vector::push_back(delegates_list, caller);
    // Update vote counts - one vote per delegate (no multiplier)
    <b>if</b> (approve) {
        proposal.delegate_approval_count = proposal.delegate_approval_count + 1;
    } <b>else</b> {
        proposal.delegate_rejection_count = proposal.delegate_rejection_count + 1;
    };
    // Update delegate stats
    <b>let</b> delegate = table::borrow_mut(&<b>mut</b> registry.delegates, caller);
    delegate.proposals_reviewed = delegate.proposals_reviewed + 1;
    // Capture vote counts <b>for</b> decision making after event emission
    <b>let</b> delegate_approval_count = proposal.delegate_approval_count;
    <b>let</b> delegate_rejection_count = proposal.delegate_rejection_count;
    <b>let</b> total_delegates = table::length(&registry.delegates);
    // Emit event - restore the original reason <b>for</b> event emission
    <b>let</b> reason_for_event = reason;
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_DelegateVoteEvent">DelegateVoteEvent</a> {
        proposal_id,
        delegate_address: caller,
        approve,
        vote_time: current_time,
        reason: reason_for_event,
    });
    // If more than half of delegates approve, <b>move</b> to community voting
    <b>if</b> (delegate_approval_count &gt; total_delegates / 2) {
        <a href="../social_contracts/governance.md#social_contracts_governance_move_to_community_voting_by_id">move_to_community_voting_by_id</a>(registry, proposal_id, ctx);
    }
    // If more than half of delegates reject, reject the proposal
    <b>else</b> <b>if</b> (delegate_rejection_count &gt; total_delegates / 2) {
        <a href="../social_contracts/governance.md#social_contracts_governance_reject_proposal_by_id">reject_proposal_by_id</a>(registry, proposal_id, current_time, ctx);
    };
}
</code></pre>



</details>

<a name="social_contracts_governance_move_to_community_voting_by_id"></a>

## Function `move_to_community_voting_by_id`

Move a proposal to community voting phase


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_move_to_community_voting_by_id">move_to_community_voting_by_id</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, ctx: &<a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_move_to_community_voting_by_id">move_to_community_voting_by_id</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_id: ID,
    ctx: &TxContext
) {
    // Get proposal from registry
    <b>let</b> <b>mut</b> proposal = table::remove(&<b>mut</b> registry.proposals, proposal_id);
    // Update status
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>;
    // Get current epoch <b>for</b> voting period calculation
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    // Set voting period based on registry voting period (using epochs)
    // This ensures voting ends on epoch boundaries, not millisecond timestamps
    proposal.voting_start_time = current_epoch;
    proposal.voting_end_time = current_epoch + registry.voting_period_epochs;
    // Update proposals by status
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>);
    vector::push_back(to_status, proposal_id);
    // Put proposal back in registry
    table::add(&<b>mut</b> registry.proposals, proposal_id, proposal);
    // Emit event - <b>use</b> the precise timestamp <b>for</b> the event
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalApprovedForVotingEvent">ProposalApprovedForVotingEvent</a> {
        proposal_id,
        voting_start_time: current_epoch,
        voting_end_time: current_epoch + registry.voting_period_epochs,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_reject_proposal_by_id"></a>

## Function `reject_proposal_by_id`

Reject a proposal by ID (avoids reference issues)


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_reject_proposal_by_id">reject_proposal_by_id</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, current_time: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_reject_proposal_by_id">reject_proposal_by_id</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_id: ID,
    current_time: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Get proposal from registry
    <b>let</b> <b>mut</b> proposal = table::remove(&<b>mut</b> registry.proposals, proposal_id);
    // Update status
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>;
    // Update proposals by status
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>);
    vector::push_back(to_status, proposal_id);
    // Return funds to submitter
    <b>let</b> submitter = proposal.submitter;
    <b>let</b> refund_amount = balance::value(&proposal.reward_pool);
    <b>if</b> (refund_amount &gt; 0) {
        <b>let</b> refund_coin = coin::from_balance(balance::withdraw_all(&<b>mut</b> proposal.reward_pool), ctx);
        transfer::public_transfer(refund_coin, submitter);
    } <b>else</b> {
        // Empty the balance even <b>if</b> zero, <b>for</b> consistency
        balance::destroy_zero(balance::withdraw_all(&<b>mut</b> proposal.reward_pool));
    };
    // Put modified proposal back in registry
    table::add(&<b>mut</b> registry.proposals, proposal_id, proposal);
    // Emit event
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedEvent">ProposalRejectedEvent</a> {
        proposal_id,
        rejection_time: current_time,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_community_vote_on_proposal"></a>

## Function `community_vote_on_proposal`

Community vote on a proposal with quadratic voting
Users can cast multiple votes by paying a quadratically increasing cost


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_on_proposal">community_vote_on_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, vote_count: u64, approve: bool, coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_on_proposal">community_vote_on_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_id: ID,
    vote_count: u64,
    approve: bool,
    coin: &<b>mut</b> Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    // Also get the current epoch <b>for</b> timing checks
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    // Calculate vote cost before borrowing from registry
    <b>let</b> quadratic_base_cost = registry.quadratic_base_cost;
    <b>let</b> vote_cost = <b>if</b> (vote_count &lt;= 1) {
        0
    } <b>else</b> {
        // Quadratic cost formula: base_cost * (vote_count^2 - 1)
        quadratic_base_cost * ((vote_count * vote_count) - 1)
    };
    // Verify proposal exists and is in community voting phase
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>let</b> proposal = table::borrow_mut(&<b>mut</b> registry.proposals, proposal_id);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_ENotVotingPhase">ENotVotingPhase</a>);
    // Verify voting period hasn't ended (check using epochs, not milliseconds)
    <b>assert</b>!(current_epoch &lt;= proposal.voting_end_time, <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodEnded">EVotingPeriodEnded</a>);
    // Verify vote count is valid (at least 1, no more than max)
    <b>assert</b>!(vote_count &gt; 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidVoteCount">EInvalidVoteCount</a>);
    <b>assert</b>!(vote_count &lt;= registry.max_votes_per_user, <a href="../social_contracts/governance.md#social_contracts_governance_EExceedsMaxVotes">EExceedsMaxVotes</a>);
    // Check <b>if</b> user <b>has</b> already voted
    <b>let</b> voted_community: &<b>mut</b> VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_COMMUNITY_FIELD">VOTED_COMMUNITY_FIELD</a>);
    <b>assert</b>!(!vec_set::contains(voted_community, &caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyVoted">EAlreadyVoted</a>);
    // Check <b>if</b> user <b>has</b> enough funds <b>for</b> votes
    <b>if</b> (vote_cost &gt; 0) {
        <b>assert</b>!(coin::value(coin) &gt;= vote_cost, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidAmount">EInvalidAmount</a>);
        // Split coin and add to proposal's reward pool <b>for</b> the additional votes
        <b>let</b> vote_coin = coin::split(coin, vote_cost, ctx);
        balance::join(&<b>mut</b> proposal.reward_pool, coin::into_balance(vote_coin));
    };
    // Record vote
    vec_set::insert(voted_community, caller);
    // Update vote tally
    <b>if</b> (approve) {
        proposal.community_votes_for = proposal.community_votes_for + vote_count;
        // Add to voted_for set <b>for</b> reward distribution
        <b>let</b> voted_for: &<b>mut</b> VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_FOR_FIELD">VOTED_FOR_FIELD</a>);
        vec_set::insert(voted_for, caller);
    } <b>else</b> {
        proposal.community_votes_against = proposal.community_votes_against + vote_count;
        // Add to voted_against set <b>for</b> reward distribution
        <b>let</b> voted_against: &<b>mut</b> VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_AGAINST_FIELD">VOTED_AGAINST_FIELD</a>);
        vec_set::insert(voted_against, caller);
    };
    // Emit event (<b>use</b> millisecond timestamp <b>for</b> the event record)
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_CommunityVoteEvent">CommunityVoteEvent</a> {
        proposal_id,
        voter: caller,
        vote_weight: vote_count,
        approve,
        vote_time: current_time,
        vote_cost,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_community_vote_anonymous"></a>

## Function `community_vote_anonymous`

Submit an anonymous encrypted vote on a proposal


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_anonymous">community_vote_anonymous</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, encrypted_vote: <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_community_vote_anonymous">community_vote_anonymous</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_id: ID,
    encrypted_vote: EncryptedObject,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>let</b> proposal = table::borrow_mut(&<b>mut</b> registry.proposals, proposal_id);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_ENotVotingPhase">ENotVotingPhase</a>);
    <b>assert</b>!(current_epoch &lt;= proposal.voting_end_time, <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodEnded">EVotingPeriodEnded</a>);
    <b>assert</b>!(!table::contains(&registry.delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_EDelegateAnonNotAllowed">EDelegateAnonNotAllowed</a>);
    <b>let</b> voted_community: &<b>mut</b> VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_COMMUNITY_FIELD">VOTED_COMMUNITY_FIELD</a>);
    <b>assert</b>!(!vec_set::contains(voted_community, &caller), <a href="../social_contracts/governance.md#social_contracts_governance_EAlreadyVoted">EAlreadyVoted</a>);
    vec_set::insert(voted_community, caller);
    <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>)) {
        <b>let</b> tbl = table::new&lt;<b>address</b>, EncryptedObject&gt;(ctx);
        dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>, tbl);
    };
    <b>let</b> enc_tbl: &<b>mut</b> Table&lt;<b>address</b>, EncryptedObject&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>);
    table::add(enc_tbl, caller, encrypted_vote);
    <b>if</b> (!dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>)) {
        <b>let</b> set = vec_set::empty&lt;<b>address</b>&gt;();
        dynamic_field::add(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>, set);
    };
    <b>let</b> anon_set: &<b>mut</b> VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>);
    vec_set::insert(anon_set, caller);
    // Serialize the entire EncryptedObject <b>for</b> indexer storage
    <b>let</b> <b>mut</b> serialized_vote = vector::empty&lt;u8&gt;();
    serialized_vote.append(*encrypted_vote.blob());
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_AnonymousVoteEvent">AnonymousVoteEvent</a> {
        proposal_id,
        voter: caller,
        vote_time: current_time,
        encrypted_vote_data: serialized_vote, // Emit encrypted blob <b>for</b> indexer
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_finalize_proposal"></a>

## Function `finalize_proposal`

Finalize a proposal after the voting period ends


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal">finalize_proposal</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal">finalize_proposal</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    // Verify proposal exists and is in community voting phase
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>let</b> proposal = table::borrow_mut(&<b>mut</b> registry.proposals, proposal_id);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    // Verify voting period <b>has</b> ended (using epoch-based timing)
    <b>assert</b>!(current_epoch &gt; proposal.voting_end_time, <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodNotEnded">EVotingPeriodNotEnded</a>);
    // Get total votes
    <b>let</b> total_votes = proposal.community_votes_for + proposal.community_votes_against;
    // Update proposals by status
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    // Check <b>if</b> quorum is reached (using registry's quorum_votes)
    <b>let</b> quorum_met = total_votes &gt;= registry.quorum_votes;
    <b>let</b> majority_approve = proposal.community_votes_for &gt; proposal.community_votes_against;
    <b>if</b> (quorum_met) {
        // Update delegate winning/losing votes <b>if</b> they exist in the proposal
        <b>if</b> (dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>)) {
            <b>let</b> delegates_list: &vector&lt;<b>address</b>&gt; = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_DELEGATES_LIST_FIELD">VOTED_DELEGATES_LIST_FIELD</a>);
            <b>let</b> delegate_votes: &Table&lt;<b>address</b>, bool&gt; = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_DELEGATE_VOTES_FIELD">DELEGATE_VOTES_FIELD</a>);
            <b>let</b> <b>mut</b> i = 0;
            <b>let</b> delegates_count = vector::length(delegates_list);
            <b>while</b> (i &lt; delegates_count) {
                <b>let</b> delegate_addr = *vector::borrow(delegates_list, i);
                // Only update <b>if</b> the delegate still exists in the registry
                <b>if</b> (table::contains(&registry.delegates, delegate_addr)) {
                    <b>let</b> delegate = table::borrow_mut(&<b>mut</b> registry.delegates, delegate_addr);
                    <b>let</b> delegate_approved = *table::borrow(delegate_votes, delegate_addr);
                    // Update winning/losing counts based on matching the majority
                    <b>if</b> ((delegate_approved && majority_approve) || (!delegate_approved && !majority_approve)) {
                        delegate.sided_winning_proposals = delegate.sided_winning_proposals + 1;
                    } <b>else</b> {
                        delegate.sided_losing_proposals = delegate.sided_losing_proposals + 1;
                    };
                };
                i = i + 1;
            };
        };
        // Determine outcome
        <b>if</b> (majority_approve) {
            // <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a> approved
            proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>;
            <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>);
            vector::push_back(to_status, proposal_id);
            // Emit approval event
            event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalApprovedEvent">ProposalApprovedEvent</a> {
                proposal_id,
                approval_time: current_time,
                votes_for: proposal.community_votes_for,
                votes_against: proposal.community_votes_against,
            });
            // Distribute rewards to winning voters
            <a href="../social_contracts/governance.md#social_contracts_governance_distribute_rewards">distribute_rewards</a>(proposal, <b>true</b>, ctx);
        } <b>else</b> {
            // <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a> rejected
            proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>;
            <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>);
            vector::push_back(to_status, proposal_id);
            // Emit rejection event
            event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedByCommunityEvent">ProposalRejectedByCommunityEvent</a> {
                proposal_id,
                rejection_time: current_time,
                votes_for: proposal.community_votes_for,
                votes_against: proposal.community_votes_against,
            });
            // Distribute rewards to losing voters
            <a href="../social_contracts/governance.md#social_contracts_governance_distribute_rewards">distribute_rewards</a>(proposal, <b>false</b>, ctx);
        }
    } <b>else</b> {
        // Quorum not reached, proposal rejected
        proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>;
        <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>);
        vector::push_back(to_status, proposal_id);
        // Emit rejection event
        event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalRejectedByCommunityEvent">ProposalRejectedByCommunityEvent</a> {
            proposal_id,
            rejection_time: current_time,
            votes_for: proposal.community_votes_for,
            votes_against: proposal.community_votes_against,
        });
        // Return funds to proposer since quorum wasn't reached
        <b>let</b> submitter = proposal.submitter;
        <b>let</b> refund_amount = balance::value(&proposal.reward_pool);
        <b>if</b> (refund_amount &gt; 0) {
            <b>let</b> refund_coin = coin::from_balance(balance::withdraw_all(&<b>mut</b> proposal.reward_pool), ctx);
            transfer::public_transfer(refund_coin, submitter);
        } <b>else</b> {
            // Empty the balance even <b>if</b> zero, <b>for</b> consistency
            balance::destroy_zero(balance::withdraw_all(&<b>mut</b> proposal.reward_pool));
        };
    }
}
</code></pre>



</details>

<a name="social_contracts_governance_finalize_proposal_anonymous"></a>

## Function `finalize_proposal_anonymous`

Finalize a proposal with anonymous votes by decrypting them first


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal_anonymous">finalize_proposal_anonymous</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">mydata::bf_hmac_encryption::VerifiedDerivedKey</a>&gt;, public_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">mydata::bf_hmac_encryption::PublicKey</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal_anonymous">finalize_proposal_anonymous</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_id: ID,
    keys: &vector&lt;VerifiedDerivedKey&gt;,
    public_keys: &vector&lt;PublicKey&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_epoch = tx_context::epoch(ctx);
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    // First, collect all the decrypted votes
    <b>let</b> <b>mut</b> votes_for = vector::empty&lt;<b>address</b>&gt;();
    <b>let</b> <b>mut</b> votes_against = vector::empty&lt;<b>address</b>&gt;();
    <b>let</b> <b>mut</b> invalid_votes = vector::empty&lt;<b>address</b>&gt;(); // Track invalid votes
    {
        <b>let</b> proposal = table::borrow_mut(&<b>mut</b> registry.proposals, proposal_id);
        <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
        <b>assert</b>!(current_epoch &gt; proposal.voting_end_time, <a href="../social_contracts/governance.md#social_contracts_governance_EVotingPeriodNotEnded">EVotingPeriodNotEnded</a>);
        <b>if</b> (dynamic_field::exists_(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>)) {
            <b>let</b> votes_tbl: &Table&lt;<b>address</b>, EncryptedObject&gt; = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ENCRYPTED_VOTES_FIELD">ENCRYPTED_VOTES_FIELD</a>);
            <b>let</b> anon_set: &VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow(&proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_ANON_VOTERS_FIELD">ANON_VOTERS_FIELD</a>);
            <b>let</b> voters_vec = vec_set::into_keys(*anon_set);
            <b>let</b> <b>mut</b> i = 0;
            <b>let</b> len = vector::length(&voters_vec);
            // Decrypt all votes and collect results with comprehensive error handling
            <b>while</b> (i &lt; len) {
                <b>let</b> addr = *vector::borrow(&voters_vec, i);
                <b>let</b> enc = table::borrow(votes_tbl, addr);
                <b>let</b> dec = decrypt(enc, keys, public_keys);
                <b>if</b> (option::is_some(&dec)) {
                    <b>let</b> b = option::borrow(&dec);
                    // Validate vote format: must be exactly 1 byte with value 0 or 1
                    <b>if</b> (vector::length(b) == 1) {
                        <b>let</b> vote_value = *vector::borrow(b, 0);
                        <b>if</b> (vote_value == 1) {
                            vector::push_back(&<b>mut</b> votes_for, addr);
                        } <b>else</b> <b>if</b> (vote_value == 0) {
                            vector::push_back(&<b>mut</b> votes_against, addr);
                        } <b>else</b> {
                            // Invalid vote value (not 0 or 1) - possible attack
                            vector::push_back(&<b>mut</b> invalid_votes, addr);
                            event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_VoteDecryptionFailedEvent">VoteDecryptionFailedEvent</a> {
                                proposal_id,
                                voter: addr,
                                failure_reason: string::utf8(b"Invalid vote value - not 0 or 1"),
                                timestamp: tx_context::epoch_timestamp_ms(ctx),
                            });
                        }
                    } <b>else</b> {
                        // Invalid vote format (wrong length) - possible corruption
                        vector::push_back(&<b>mut</b> invalid_votes, addr);
                        event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_VoteDecryptionFailedEvent">VoteDecryptionFailedEvent</a> {
                            proposal_id,
                            voter: addr,
                            failure_reason: string::utf8(b"Invalid vote format - wrong byte length"),
                            timestamp: tx_context::epoch_timestamp_ms(ctx),
                        });
                    }
                } <b>else</b> {
                    // Failed to decrypt - could be malicious, corrupted, or wrong keys
                    vector::push_back(&<b>mut</b> invalid_votes, addr);
                    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_VoteDecryptionFailedEvent">VoteDecryptionFailedEvent</a> {
                        proposal_id,
                        voter: addr,
                        failure_reason: string::utf8(b"Decryption failed - invalid keys or corrupted data"),
                        timestamp: tx_context::epoch_timestamp_ms(ctx),
                    });
                };
                i = i + 1;
            };
            vector::destroy_empty(voters_vec);
        };
    };
    // Log invalid votes <b>for</b> transparency but don't fail the entire process
    // In production, you might want to emit events <b>for</b> invalid votes
    vector::destroy_empty(invalid_votes);
    // Now apply all the valid votes
    {
        <b>let</b> proposal = table::borrow_mut(&<b>mut</b> registry.proposals, proposal_id);
        // Process votes <b>for</b>
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&votes_for);
        <b>while</b> (i &lt; len) {
            <b>let</b> addr = *vector::borrow(&votes_for, i);
            proposal.community_votes_for = proposal.community_votes_for + 1;
            <b>let</b> voted_for: &<b>mut</b> VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_FOR_FIELD">VOTED_FOR_FIELD</a>);
            vec_set::insert(voted_for, addr);
            i = i + 1;
        };
        // Process votes against
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&votes_against);
        <b>while</b> (i &lt; len) {
            <b>let</b> addr = *vector::borrow(&votes_against, i);
            proposal.community_votes_against = proposal.community_votes_against + 1;
            <b>let</b> voted_against: &<b>mut</b> VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow_mut(&<b>mut</b> proposal.id, <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_AGAINST_FIELD">VOTED_AGAINST_FIELD</a>);
            vec_set::insert(voted_against, addr);
            i = i + 1;
        };
    };
    // Clean up temporary vectors
    vector::destroy_empty(votes_for);
    vector::destroy_empty(votes_against);
    // All encrypted votes processed
    <a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal">finalize_proposal</a>(registry, proposal_id, ctx);
}
</code></pre>



</details>

<a name="social_contracts_governance_distribute_rewards"></a>

## Function `distribute_rewards`

Distribute rewards to winning voters
Rewards are distributed equally among all voters who voted for the winning side


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_distribute_rewards">distribute_rewards</a>(proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, approve_won: bool, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_distribute_rewards">distribute_rewards</a>(
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>,
    approve_won: bool,
    ctx: &<b>mut</b> TxContext
) {
    // Get the total reward amount
    <b>let</b> total_reward = balance::value(&proposal.reward_pool);
    <b>if</b> (total_reward == 0) {
        <b>return</b> // No rewards to distribute
    };
    // Get the field name based on which side won
    <b>let</b> field_name = <b>if</b> (approve_won) <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_FOR_FIELD">VOTED_FOR_FIELD</a> <b>else</b> <a href="../social_contracts/governance.md#social_contracts_governance_VOTED_AGAINST_FIELD">VOTED_AGAINST_FIELD</a>;
    <b>let</b> winner_voters: &VecSet&lt;<b>address</b>&gt; = dynamic_field::borrow(&proposal.id, field_name);
    <b>let</b> voters_vec = vec_set::into_keys(*winner_voters);
    <b>let</b> voter_count = vector::length(&voters_vec);
    <b>if</b> (voter_count == 0) {
        // No winners, <b>return</b> funds to proposer
        <b>let</b> reward_coin = coin::from_balance(balance::withdraw_all(&<b>mut</b> proposal.reward_pool), ctx);
        transfer::public_transfer(reward_coin, proposal.submitter);
        vector::destroy_empty(voters_vec);
        <b>return</b>
    };
    // Calculate per voter reward
    <b>let</b> per_voter_reward = total_reward / voter_count;
    // There could be dust left due to division
    <b>let</b> total_distributed = per_voter_reward * voter_count;
    <b>let</b> dust = <b>if</b> (total_distributed &lt; total_reward) {
        total_reward - total_distributed
    } <b>else</b> {
        0
    };
    // Distribute the bulk of the rewards
    <b>let</b> <b>mut</b> idx = 0;
    <b>while</b> (idx &lt; voter_count) {
        <b>let</b> voter = *vector::borrow(&voters_vec, idx);
        <b>let</b> reward_amount = <b>if</b> (idx == voter_count - 1) { per_voter_reward + dust } <b>else</b> { per_voter_reward };
        <b>if</b> (reward_amount &gt; 0) {
            <b>let</b> balance = balance::split(&<b>mut</b> proposal.reward_pool, reward_amount);
            <b>let</b> reward_coin = coin::from_balance(balance, ctx);
            transfer::public_transfer(reward_coin, voter);
        };
        idx = idx + 1;
    };
    // Emit event <b>for</b> reward distribution
    <b>assert</b>!(balance::value(&proposal.reward_pool) == 0, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidAmount">EInvalidAmount</a>);
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_RewardsDistributedEvent">RewardsDistributedEvent</a> {
        proposal_id: object::id(proposal),
        total_reward,
        recipient_count: voter_count,
        distribution_time: tx_context::epoch_timestamp_ms(ctx),
    });
    vector::destroy_empty(voters_vec);
}
</code></pre>



</details>

<a name="social_contracts_governance_mark_proposal_implemented"></a>

## Function `mark_proposal_implemented`

Mark a proposal as implemented


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented">mark_proposal_implemented</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented">mark_proposal_implemented</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_id: ID,
    description: Option&lt;String&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    // Verify proposal exists and is approved
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>let</b> proposal = table::borrow_mut(&<b>mut</b> registry.proposals, proposal_id);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    // Only the submitter or a delegate can mark <b>as</b> implemented
    <b>assert</b>!(
        proposal.submitter == caller || table::contains(&registry.delegates, caller),
        <a href="../social_contracts/governance.md#social_contracts_governance_EUnauthorized">EUnauthorized</a>
    );
    // Update status
    proposal.status = <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>;
    // Update proposals by status
    <b>let</b> from_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>);
    <b>let</b> <b>mut</b> index = 0;
    <b>let</b> len = vector::length(from_status);
    <b>while</b> (index &lt; len) {
        <b>if</b> (*vector::borrow(from_status, index) == proposal_id) {
            vector::remove(from_status, index);
            <b>break</b>
        };
        index = index + 1;
    };
    <b>let</b> to_status = table::borrow_mut(&<b>mut</b> registry.proposals_by_status, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>);
    vector::push_back(to_status, proposal_id);
    // Emit event
    event::emit(<a href="../social_contracts/governance.md#social_contracts_governance_ProposalImplementedEvent">ProposalImplementedEvent</a> {
        proposal_id,
        implementation_time: current_time,
        description,
    });
}
</code></pre>



</details>

<a name="social_contracts_governance_get_proposals_by_type"></a>

## Function `get_proposals_by_type`

Get all proposals of a specific type


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposals_by_type">get_proposals_by_type</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_type: u8): vector&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposals_by_type">get_proposals_by_type</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_type: u8
): vector&lt;ID&gt; {
    <b>assert</b>!(proposal_type &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    <b>let</b> <b>mut</b> result = vector::empty&lt;ID&gt;();
    <b>let</b> statuses = vector[ <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_SUBMITTED">STATUS_SUBMITTED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_COMMUNITY_VOTING">STATUS_COMMUNITY_VOTING</a>, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_APPROVED">STATUS_APPROVED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_REJECTED">STATUS_REJECTED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a> ];
    <b>let</b> <b>mut</b> s = 0;
    <b>while</b> (s &lt; vector::length(&statuses)) {
        <b>let</b> status = *vector::borrow(&statuses, s);
        <b>let</b> proposals_of_status: &vector&lt;ID&gt; = table::borrow(&registry.proposals_by_status, status);
        <b>let</b> <b>mut</b> p = 0;
        <b>let</b> num_proposals = vector::length(proposals_of_status);
        <b>while</b> (p &lt; num_proposals) {
            <b>let</b> proposal_id = *vector::borrow(proposals_of_status, p);
            <b>let</b> proposal: &<a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a> = table::borrow(&registry.proposals, proposal_id);
            <b>if</b> (proposal.proposal_type == proposal_type) {
                vector::push_back(&<b>mut</b> result, proposal_id);
            };
            p = p + 1;
        };
        s = s + 1;
    };
    vector::destroy_empty(statuses);
    result
}
</code></pre>



</details>

<a name="social_contracts_governance_get_proposals_by_status"></a>

## Function `get_proposals_by_status`

Get all proposals with a specific status


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposals_by_status">get_proposals_by_status</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, status: u8): vector&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposals_by_status">get_proposals_by_status</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    status: u8
): vector&lt;ID&gt; {
    <b>assert</b>!(status &lt;= <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_IMPLEMENTED">STATUS_IMPLEMENTED</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidParameter">EInvalidParameter</a>);
    *table::borrow(&registry.proposals_by_status, status)
}
</code></pre>



</details>

<a name="social_contracts_governance_get_delegate_count"></a>

## Function `get_delegate_count`

Get number of delegates


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_delegate_count">get_delegate_count</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_delegate_count">get_delegate_count</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): u64 {
    table::length(&registry.delegates)
}
</code></pre>



</details>

<a name="social_contracts_governance_get_delegate_info"></a>

## Function `get_delegate_info`

Get delegate information


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_delegate_info">get_delegate_info</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, addr: <b>address</b>): (<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, u64, u64, u64, u64, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_delegate_info">get_delegate_info</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    addr: <b>address</b>
): (ID, u64, u64, u64, u64, u64, u64, u64, u64) {
    <b>assert</b>!(table::contains(&registry.delegates, addr), <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>);
    <b>let</b> delegate = table::borrow(&registry.delegates, addr);
    (
        delegate.profile_id,
        delegate.upvotes,
        delegate.downvotes,
        delegate.proposals_reviewed,
        delegate.proposals_submitted,
        delegate.sided_winning_proposals,
        delegate.sided_losing_proposals,
        delegate.term_start,
        delegate.term_end
    )
}
</code></pre>



</details>

<a name="social_contracts_governance_get_proposal_info"></a>

## Function `get_proposal_info`

Get proposal information


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposal_info">get_proposal_info</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>): (<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../std/string.md#std_string_String">std::string::String</a>, u8, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <b>address</b>, u64, u8, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_proposal_info">get_proposal_info</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    id: ID
): (String, String, u8, Option&lt;ID&gt;, Option&lt;String&gt;, <b>address</b>, u64, u8, u64, u64) {
    <b>assert</b>!(table::contains(&registry.proposals, id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>let</b> proposal = table::borrow(&registry.proposals, id);
    (
        proposal.title,
        proposal.description,
        proposal.proposal_type,
        proposal.reference_id,
        proposal.metadata_json,
        proposal.submitter,
        proposal.submission_time,
        proposal.status,
        proposal.community_votes_for,
        proposal.community_votes_against
    )
}
</code></pre>



</details>

<a name="social_contracts_governance_treasury_balance"></a>

## Function `treasury_balance`

Get the current treasury balance


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_treasury_balance">treasury_balance</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_treasury_balance">treasury_balance</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): u64 {
    balance::value(&registry.treasury)
}
</code></pre>



</details>

<a name="social_contracts_governance_calculate_vote_cost"></a>

## Function `calculate_vote_cost`

Calculate cost for additional votes beyond the first free vote


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_calculate_vote_cost">calculate_vote_cost</a>(vote_count: u64, registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_calculate_vote_cost">calculate_vote_cost</a>(
    vote_count: u64,
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>
): u64 {
    <b>if</b> (vote_count &lt;= 1) {
        <b>return</b> 0
    };
    // Quadratic cost formula: base_cost * (vote_count^2 - 1)
    registry.quadratic_base_cost * ((vote_count * vote_count) - 1)
}
</code></pre>



</details>

<a name="social_contracts_governance_is_delegate"></a>

## Function `is_delegate`

Check if an address is a delegate


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_is_delegate">is_delegate</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_is_delegate">is_delegate</a>( registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, addr: <b>address</b> ): bool {
    table::contains(&registry.delegates, addr)
}
</code></pre>



</details>

<a name="social_contracts_governance_is_delegate_term_expired"></a>

## Function `is_delegate_term_expired`

Check if delegate term has expired


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_is_delegate_term_expired">is_delegate_term_expired</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, addr: <b>address</b>, current_epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_is_delegate_term_expired">is_delegate_term_expired</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    addr: <b>address</b>,
    current_epoch: u64
): bool {
    <b>if</b> (!table::contains(&registry.delegates, addr)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> delegate = table::borrow(&registry.delegates, addr);
    delegate.term_end &lt;= current_epoch
}
</code></pre>



</details>

<a name="social_contracts_governance_get_governance_parameters"></a>

## Function `get_governance_parameters`

Get governance parameters


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_governance_parameters">get_governance_parameters</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): (u64, u64, u64, u64, u64, u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_get_governance_parameters">get_governance_parameters</a>(
    registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>
): (u64, u64, u64, u64, u64, u64, u64, u64) {
    (
        registry.delegate_count,
        registry.delegate_term_epochs,
        registry.proposal_submission_cost,
        registry.min_on_chain_age_days,
        registry.max_votes_per_user,
        registry.quadratic_base_cost,
        registry.voting_period_epochs,
        registry.quorum_votes
    )
}
</code></pre>



</details>

<a name="social_contracts_governance_reject_proposal_manually"></a>

## Function `reject_proposal_manually`

If more than half of delegates reject, reject the proposal manually


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_reject_proposal_manually">reject_proposal_manually</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_reject_proposal_manually">reject_proposal_manually</a>(
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>,
    proposal_id: ID,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is a delegate
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(table::contains(&registry.delegates, caller), <a href="../social_contracts/governance.md#social_contracts_governance_ENotDelegate">ENotDelegate</a>);
    // Verify proposal exists and is in delegate review phase
    <b>assert</b>!(table::contains(&registry.proposals, proposal_id), <a href="../social_contracts/governance.md#social_contracts_governance_EProposalNotFound">EProposalNotFound</a>);
    <b>let</b> proposal = table::borrow(&registry.proposals, proposal_id);
    <b>assert</b>!(proposal.status == <a href="../social_contracts/governance.md#social_contracts_governance_STATUS_DELEGATE_REVIEW">STATUS_DELEGATE_REVIEW</a>, <a href="../social_contracts/governance.md#social_contracts_governance_EInvalidProposalStatus">EInvalidProposalStatus</a>);
    // Reject the proposal
    <b>let</b> current_time = tx_context::epoch_timestamp_ms(ctx);
    <a href="../social_contracts/governance.md#social_contracts_governance_reject_proposal_by_id">reject_proposal_by_id</a>(registry, proposal_id, current_time, ctx);
}
</code></pre>



</details>

<a name="social_contracts_governance_create_platform_governance"></a>

## Function `create_platform_governance`

Create a platform-specific governance registry when a platform is approved
This function can only be called by the platform toggle_platform_approval function
and only the package publisher can call it


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_create_platform_governance">create_platform_governance</a>(delegate_count: u64, delegate_term_epochs: u64, proposal_submission_cost: u64, min_on_chain_age_days: u64, max_votes_per_user: u64, quadratic_base_cost: u64, voting_period_epochs: u64, quorum_votes: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/object.md#mys_object_ID">mys::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_create_platform_governance">create_platform_governance</a>(
    delegate_count: u64,
    delegate_term_epochs: u64,
    proposal_submission_cost: u64,
    min_on_chain_age_days: u64,
    max_votes_per_user: u64,
    quadratic_base_cost: u64,
    voting_period_epochs: u64,
    quorum_votes: u64,
    ctx: &<b>mut</b> TxContext
): ID {
    // Create Platform Governance Registry with parameters
    <b>let</b> <b>mut</b> platform_registry = <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a> {
        id: object::new(ctx),
        registry_type: <a href="../social_contracts/governance.md#social_contracts_governance_PROPOSAL_TYPE_PLATFORM">PROPOSAL_TYPE_PLATFORM</a>,
        // Configuration parameters specific to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/governance.md#social_contracts_governance">governance</a>
        delegate_count,
        delegate_term_epochs,
        proposal_submission_cost,
        min_on_chain_age_days,
        max_votes_per_user,
        quadratic_base_cost,
        voting_period_epochs,
        quorum_votes,
        // Tables
        delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_Delegate">Delegate</a>&gt;(ctx),
        proposals: table::new&lt;ID, <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">Proposal</a>&gt;(ctx),
        proposals_by_status: table::new&lt;u8, vector&lt;ID&gt;&gt;(ctx),
        treasury: balance::zero(),
        nominated_delegates: table::new&lt;<b>address</b>, <a href="../social_contracts/governance.md#social_contracts_governance_NominatedDelegate">NominatedDelegate</a>&gt;(ctx),
        delegate_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        nominee_addresses: vec_set::empty&lt;<b>address</b>&gt;(),
        voters: table::new&lt;<b>address</b>, Table&lt;<b>address</b>, bool&gt;&gt;(ctx),
        <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Initialize registry tables
    <a href="../social_contracts/governance.md#social_contracts_governance_initialize_registry_tables">initialize_registry_tables</a>(&<b>mut</b> platform_registry, ctx);
    // Get the ID before sharing
    <b>let</b> registry_id = object::id(&platform_registry);
    // Share the registry object
    transfer::share_object(platform_registry);
    // Return the registry ID
    registry_id
}
</code></pre>



</details>

<a name="social_contracts_governance_version"></a>

## Function `version`

Get version of GovernanceDAO


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>(registry: &<a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>): u64 {
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_governance_set_version"></a>

## Function `set_version`

Set version of GovernanceDAO


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_set_version">set_version</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, new_version: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_set_version">set_version</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, new_version: u64) {
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> = new_version;
}
</code></pre>



</details>

<a name="social_contracts_governance_migrate_registry"></a>

## Function `migrate_registry`

Public entry function that migrates registry to the latest version


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_migrate_registry">migrate_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, _ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_migrate_registry">migrate_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">GovernanceDAO</a>, _ctx: &<b>mut</b> TxContext) {
    <b>let</b> current_version = registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a>;
    <b>let</b> latest_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(current_version &lt; latest_version, <a href="../social_contracts/governance.md#social_contracts_governance_EWrongVersion">EWrongVersion</a>);
    // Version-specific migrations would go here when needed
    registry.<a href="../social_contracts/governance.md#social_contracts_governance_version">version</a> = latest_version;
}
</code></pre>



</details>

<a name="social_contracts_governance_create_governance_admin_cap"></a>

## Function `create_governance_admin_cap`

Create a GovernanceAdminCap for bootstrap (package visibility only)
This function is only callable by other modules in the same package


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_create_governance_admin_cap">create_governance_admin_cap</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">social_contracts::governance::GovernanceAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/governance.md#social_contracts_governance_create_governance_admin_cap">create_governance_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">GovernanceAdminCap</a> {
    <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceAdminCap">GovernanceAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>
