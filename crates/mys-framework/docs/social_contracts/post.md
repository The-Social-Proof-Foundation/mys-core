---
title: Module `social_contracts::post`
---

Post module for the MySocial network
Handles creation and management of posts and comments
Implements features like comments, reposts, quotes, and predictions


-  [Struct `Post`](#social_contracts_post_Post)
-  [Struct `Comment`](#social_contracts_post_Comment)
-  [Struct `Repost`](#social_contracts_post_Repost)
-  [Struct `PredictionOption`](#social_contracts_post_PredictionOption)
-  [Struct `PredictionBet`](#social_contracts_post_PredictionBet)
-  [Struct `PredictionData`](#social_contracts_post_PredictionData)
-  [Struct `PromotionView`](#social_contracts_post_PromotionView)
-  [Struct `PromotionData`](#social_contracts_post_PromotionData)
-  [Struct `PostAdminCap`](#social_contracts_post_PostAdminCap)
-  [Struct `PostConfig`](#social_contracts_post_PostConfig)
-  [Struct `PostParametersUpdatedEvent`](#social_contracts_post_PostParametersUpdatedEvent)
-  [Struct `PostCreatedEvent`](#social_contracts_post_PostCreatedEvent)
-  [Struct `CommentCreatedEvent`](#social_contracts_post_CommentCreatedEvent)
-  [Struct `RepostEvent`](#social_contracts_post_RepostEvent)
-  [Struct `ReactionEvent`](#social_contracts_post_ReactionEvent)
-  [Struct `RemoveReactionEvent`](#social_contracts_post_RemoveReactionEvent)
-  [Struct `TipEvent`](#social_contracts_post_TipEvent)
-  [Struct `OwnershipTransferEvent`](#social_contracts_post_OwnershipTransferEvent)
-  [Struct `PostModerationEvent`](#social_contracts_post_PostModerationEvent)
-  [Struct `PostUpdatedEvent`](#social_contracts_post_PostUpdatedEvent)
-  [Struct `CommentUpdatedEvent`](#social_contracts_post_CommentUpdatedEvent)
-  [Struct `PostReportedEvent`](#social_contracts_post_PostReportedEvent)
-  [Struct `CommentReportedEvent`](#social_contracts_post_CommentReportedEvent)
-  [Struct `PostDeletedEvent`](#social_contracts_post_PostDeletedEvent)
-  [Struct `CommentDeletedEvent`](#social_contracts_post_CommentDeletedEvent)
-  [Struct `PredictionCreatedEvent`](#social_contracts_post_PredictionCreatedEvent)
-  [Struct `PredictionBetPlacedEvent`](#social_contracts_post_PredictionBetPlacedEvent)
-  [Struct `PredictionResolvedEvent`](#social_contracts_post_PredictionResolvedEvent)
-  [Struct `PredictionPayoutEvent`](#social_contracts_post_PredictionPayoutEvent)
-  [Struct `PredictionBetWithdrawnEvent`](#social_contracts_post_PredictionBetWithdrawnEvent)
-  [Struct `PromotedPostCreatedEvent`](#social_contracts_post_PromotedPostCreatedEvent)
-  [Struct `PromotedPostViewConfirmedEvent`](#social_contracts_post_PromotedPostViewConfirmedEvent)
-  [Struct `PromotionStatusToggledEvent`](#social_contracts_post_PromotionStatusToggledEvent)
-  [Struct `PromotionFundsWithdrawnEvent`](#social_contracts_post_PromotionFundsWithdrawnEvent)
-  [Struct `ModerationRecord`](#social_contracts_post_ModerationRecord)
-  [Constants](#@Constants_0)
-  [Function `is_auto_pool_disabled`](#social_contracts_post_is_auto_pool_disabled)
-  [Function `set_auto_pool_disabled`](#social_contracts_post_set_auto_pool_disabled)
-  [Function `bootstrap_init`](#social_contracts_post_bootstrap_init)
-  [Function `set_predictions_enabled`](#social_contracts_post_set_predictions_enabled)
-  [Function `set_prediction_fee`](#social_contracts_post_set_prediction_fee)
-  [Function `is_predictions_enabled`](#social_contracts_post_is_predictions_enabled)
-  [Function `create_prediction_post`](#social_contracts_post_create_prediction_post)
-  [Function `place_prediction_bet`](#social_contracts_post_place_prediction_bet)
-  [Function `withdraw_prediction_bet`](#social_contracts_post_withdraw_prediction_bet)
-  [Function `resolve_prediction`](#social_contracts_post_resolve_prediction)
-  [Function `create_post_internal`](#social_contracts_post_create_post_internal)
-  [Function `create_post`](#social_contracts_post_create_post)
-  [Function `create_comment`](#social_contracts_post_create_comment)
-  [Function `create_repost`](#social_contracts_post_create_repost)
-  [Function `delete_post`](#social_contracts_post_delete_post)
-  [Function `delete_comment`](#social_contracts_post_delete_comment)
-  [Function `react_to_post`](#social_contracts_post_react_to_post)
-  [Function `tip_post`](#social_contracts_post_tip_post)
-  [Function `apply_poc_redirection_and_transfer`](#social_contracts_post_apply_poc_redirection_and_transfer)
-  [Function `update_poc_result`](#social_contracts_post_update_poc_result)
-  [Function `clear_poc_data`](#social_contracts_post_clear_poc_data)
-  [Function `tip_repost`](#social_contracts_post_tip_repost)
-  [Function `tip_comment`](#social_contracts_post_tip_comment)
-  [Function `transfer_post_ownership`](#social_contracts_post_transfer_post_ownership)
-  [Function `admin_transfer_post_ownership`](#social_contracts_post_admin_transfer_post_ownership)
-  [Function `moderate_post`](#social_contracts_post_moderate_post)
-  [Function `moderate_comment`](#social_contracts_post_moderate_comment)
-  [Function `update_post`](#social_contracts_post_update_post)
-  [Function `update_comment`](#social_contracts_post_update_comment)
-  [Function `report_post`](#social_contracts_post_report_post)
-  [Function `report_comment`](#social_contracts_post_report_comment)
-  [Function `react_to_comment`](#social_contracts_post_react_to_comment)
-  [Function `get_post_content`](#social_contracts_post_get_post_content)
-  [Function `get_post_owner`](#social_contracts_post_get_post_owner)
-  [Function `get_post_id`](#social_contracts_post_get_post_id)
-  [Function `get_post_comment_count`](#social_contracts_post_get_post_comment_count)
-  [Function `get_comment_owner`](#social_contracts_post_get_comment_owner)
-  [Function `get_comment_post_id`](#social_contracts_post_get_comment_post_id)
-  [Function `get_id_address`](#social_contracts_post_get_id_address)
-  [Function `get_reaction_count`](#social_contracts_post_get_reaction_count)
-  [Function `get_tips_received`](#social_contracts_post_get_tips_received)
-  [Function `get_poc_badge_id`](#social_contracts_post_get_poc_badge_id)
-  [Function `get_revenue_redirect_to`](#social_contracts_post_get_revenue_redirect_to)
-  [Function `get_revenue_redirect_percentage`](#social_contracts_post_get_revenue_redirect_percentage)
-  [Function `get_total_bet_amount`](#social_contracts_post_get_total_bet_amount)
-  [Function `get_bets_count`](#social_contracts_post_get_bets_count)
-  [Function `get_bet_user`](#social_contracts_post_get_bet_user)
-  [Function `get_bet_option_id`](#social_contracts_post_get_bet_option_id)
-  [Function `get_bet_amount`](#social_contracts_post_get_bet_amount)
-  [Function `version`](#social_contracts_post_version)
-  [Function `borrow_version_mut`](#social_contracts_post_borrow_version_mut)
-  [Function `comment_version`](#social_contracts_post_comment_version)
-  [Function `borrow_comment_version_mut`](#social_contracts_post_borrow_comment_version_mut)
-  [Function `repost_version`](#social_contracts_post_repost_version)
-  [Function `borrow_repost_version_mut`](#social_contracts_post_borrow_repost_version_mut)
-  [Function `migrate_post`](#social_contracts_post_migrate_post)
-  [Function `migrate_comment`](#social_contracts_post_migrate_comment)
-  [Function `migrate_repost`](#social_contracts_post_migrate_repost)
-  [Function `update_post_parameters`](#social_contracts_post_update_post_parameters)
-  [Function `create_promoted_post`](#social_contracts_post_create_promoted_post)
-  [Function `confirm_promoted_post_view`](#social_contracts_post_confirm_promoted_post_view)
-  [Function `toggle_promotion_status`](#social_contracts_post_toggle_promotion_status)
-  [Function `withdraw_promotion_funds`](#social_contracts_post_withdraw_promotion_funds)
-  [Function `get_promotion_stats`](#social_contracts_post_get_promotion_stats)
-  [Function `has_user_viewed_promoted_post`](#social_contracts_post_has_user_viewed_promoted_post)
-  [Function `get_promotion_id`](#social_contracts_post_get_promotion_id)
-  [Function `set_moderation_status`](#social_contracts_post_set_moderation_status)
-  [Function `is_content_approved`](#social_contracts_post_is_content_approved)
-  [Function `create_post_admin_cap`](#social_contracts_post_create_post_admin_cap)


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
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
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



<a name="social_contracts_post_Post"></a>

## Struct `Post`

Post object that contains content information


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a> <b>has</b> key, store
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
<code>owner: <b>address</b></code>
</dt>
<dd>
 Owner's wallet address (the true owner)
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
 Author's profile ID (reference only, not ownership)
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Post content
</dd>
<dt>
<code>media: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;&gt;</code>
</dt>
<dd>
 Optional media URLs (multiple supported)
</dd>
<dt>
<code>mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
 Optional mentioned users (profile IDs)
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Optional metadata in JSON format
</dd>
<dt>
<code>post_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Post type (standard, comment, repost, quote_repost)
</dd>
<dt>
<code>parent_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Optional parent post ID for replies or quote reposts
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
<dt>
<code>reaction_count: u64</code>
</dt>
<dd>
 Total number of reactions
</dd>
<dt>
<code>comment_count: u64</code>
</dt>
<dd>
 Number of comments
</dd>
<dt>
<code>repost_count: u64</code>
</dt>
<dd>
 Number of reposts
</dd>
<dt>
<code>tips_received: u64</code>
</dt>
<dd>
 Total tips received in MYS (tracking only, not actual balance)
</dd>
<dt>
<code>removed_from_platform: bool</code>
</dt>
<dd>
 Whether the post has been removed from its platform
</dd>
<dt>
<code>user_reactions: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Table of user wallet addresses to their reactions (emoji or text)
</dd>
<dt>
<code>reaction_counts: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, u64&gt;</code>
</dt>
<dd>
 Table to count reactions by type
</dd>
<dt>
<code>allow_comments: bool</code>
</dt>
<dd>
 Direct permission toggles for post interactions
</dd>
<dt>
<code>allow_reactions: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>allow_reposts: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>allow_quotes: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>allow_tips: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>poc_badge_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;</code>
</dt>
<dd>
 Optional Proof of Creativity badge ID (for original content that passed verification)
</dd>
<dt>
<code>revenue_redirect_to: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Optional revenue redirection to original creator (for derivative content)
</dd>
<dt>
<code>revenue_redirect_percentage: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 Optional revenue redirection percentage (0-100)
</dd>
<dt>
<code>my_ip_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Reference to the intellectual property license for the post
</dd>
<dt>
<code>promotion_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Optional promotion data ID for promoted posts
</dd>
<dt>
<code>disable_auto_pool: bool</code>
</dt>
<dd>
 Opt-out flag to disable auto SPT pool initialization by SPoT
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_version">version</a>: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_post_Comment"></a>

## Struct `Comment`

Comment object for posts, supporting nested comments


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a> <b>has</b> key, store
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
 The post this comment belongs to
</dd>
<dt>
<code>parent_comment_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Optional parent comment ID for nested comments
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
 Owner's wallet address (the true owner)
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
 Commenter's profile ID (reference only, not ownership)
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Comment content
</dd>
<dt>
<code>media: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;&gt;</code>
</dt>
<dd>
 Optional media URLs
</dd>
<dt>
<code>mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
 Optional mentioned users (profile IDs)
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Optional metadata in JSON format
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
<dt>
<code>reaction_count: u64</code>
</dt>
<dd>
 Total number of reactions
</dd>
<dt>
<code>comment_count: u64</code>
</dt>
<dd>
 Number of nested comments
</dd>
<dt>
<code>repost_count: u64</code>
</dt>
<dd>
 Number of reposts
</dd>
<dt>
<code>tips_received: u64</code>
</dt>
<dd>
 Total tips received in MYS (tracking only, not actual balance)
</dd>
<dt>
<code>removed_from_platform: bool</code>
</dt>
<dd>
 Whether the comment has been removed from its platform
</dd>
<dt>
<code>user_reactions: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Table of user wallet addresses to their reactions (emoji or text)
</dd>
<dt>
<code>reaction_counts: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, u64&gt;</code>
</dt>
<dd>
 Table to count reactions by type
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_version">version</a>: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_post_Repost"></a>

## Struct `Repost`

Repost reference


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a> <b>has</b> key, store
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
<code>original_id: <b>address</b></code>
</dt>
<dd>
 The post/comment being reposted
</dd>
<dt>
<code>is_original_post: bool</code>
</dt>
<dd>
 Whether the original is a post (true) or comment (false)
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
 Owner's wallet address (the true owner)
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
 Reposter's profile ID (reference only, not ownership)
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
<dt>
<code><a href="../social_contracts/post.md#social_contracts_post_version">version</a>: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_post_PredictionOption"></a>

## Struct `PredictionOption`

Prediction option structure


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionOption">PredictionOption</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>total_bet: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PredictionBet"></a>

## Struct `PredictionBet`

Prediction bet record


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionBet">PredictionBet</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
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

<a name="social_contracts_post_PredictionData"></a>

## Struct `PredictionData`

Prediction metadata


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionData">PredictionData</a> <b>has</b> key, store
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
</dd>
<dt>
<code>options: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_PredictionOption">social_contracts::post::PredictionOption</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>bets: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_PredictionBet">social_contracts::post::PredictionBet</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>resolved: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>winning_option_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>betting_end_time: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>total_bet_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PromotionView"></a>

## Struct `PromotionView`

Promoted post view record


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionView">PromotionView</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>viewer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>view_duration: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>view_timestamp: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PromotionData"></a>

## Struct `PromotionData`

Promoted post metadata


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a> <b>has</b> key, store
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
</dd>
<dt>
<code>payment_per_view: u64</code>
</dt>
<dd>
 Amount of MYS to pay per view
</dd>
<dt>
<code>promotion_budget: <a href="../mys/balance.md#mys_balance_Balance">mys::balance::Balance</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;</code>
</dt>
<dd>
 MYS balance available for payments
</dd>
<dt>
<code>paid_viewers: <a href="../mys/table.md#mys_table_Table">mys::table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>
 Table tracking which users have already been paid for viewing
</dd>
<dt>
<code>views: vector&lt;<a href="../social_contracts/post.md#social_contracts_post_PromotionView">social_contracts::post::PromotionView</a>&gt;</code>
</dt>
<dd>
 List of all views for analytics
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
 Whether the promotion is currently active
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
 Promotion creation timestamp
</dd>
</dl>


</details>

<a name="social_contracts_post_PostAdminCap"></a>

## Struct `PostAdminCap`

Admin capability for resolving predictions


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_post_PostConfig"></a>

## Struct `PostConfig`

Global post feature configuration


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a> <b>has</b> key
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
<code>predictions_enabled: bool</code>
</dt>
<dd>
 Indicates if prediction posts are enabled
</dd>
<dt>
<code>prediction_fee_bps: u64</code>
</dt>
<dd>
 Prediction platform fee in basis points (100 = 1%)
</dd>
<dt>
<code>prediction_treasury: <b>address</b></code>
</dt>
<dd>
 Treasury address for prediction fees
</dd>
<dt>
<code>max_content_length: u64</code>
</dt>
<dd>
 Maximum character length for post content
</dd>
<dt>
<code>max_media_urls: u64</code>
</dt>
<dd>
 Maximum number of media URLs per post
</dd>
<dt>
<code>max_mentions: u64</code>
</dt>
<dd>
 Maximum number of mentions in a post
</dd>
<dt>
<code>max_metadata_size: u64</code>
</dt>
<dd>
 Maximum size for post metadata in bytes
</dd>
<dt>
<code>max_description_length: u64</code>
</dt>
<dd>
 Maximum length for report descriptions
</dd>
<dt>
<code>max_reaction_length: u64</code>
</dt>
<dd>
 Maximum length for reactions
</dd>
<dt>
<code>commenter_tip_percentage: u64</code>
</dt>
<dd>
 Percentage of tip that goes to commenter (remainder to post owner)
</dd>
<dt>
<code>repost_tip_percentage: u64</code>
</dt>
<dd>
 Percentage of tip that goes to reposter (remainder to original post owner)
</dd>
<dt>
<code>max_prediction_options: u64</code>
</dt>
<dd>
 Maximum number of prediction options
</dd>
</dl>


</details>

<a name="social_contracts_post_PostParametersUpdatedEvent"></a>

## Struct `PostParametersUpdatedEvent`

Event emitted when post parameters are updated


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostParametersUpdatedEvent">PostParametersUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
 Who performed the update
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
 When the update occurred
</dd>
<dt>
<code>max_content_length: u64</code>
</dt>
<dd>
 New max content length value
</dd>
<dt>
<code>max_media_urls: u64</code>
</dt>
<dd>
 New max media URLs value
</dd>
<dt>
<code>max_mentions: u64</code>
</dt>
<dd>
 New max mentions value
</dd>
<dt>
<code>max_metadata_size: u64</code>
</dt>
<dd>
 New max metadata size value
</dd>
<dt>
<code>max_description_length: u64</code>
</dt>
<dd>
 New max description length value
</dd>
<dt>
<code>max_reaction_length: u64</code>
</dt>
<dd>
 New max reaction length value
</dd>
<dt>
<code>commenter_tip_percentage: u64</code>
</dt>
<dd>
 New commenter tip percentage value
</dd>
<dt>
<code>repost_tip_percentage: u64</code>
</dt>
<dd>
 New repost tip percentage value
</dd>
<dt>
<code>max_prediction_options: u64</code>
</dt>
<dd>
 New max prediction options value
</dd>
</dl>


</details>

<a name="social_contracts_post_PostCreatedEvent"></a>

## Struct `PostCreatedEvent`

Post created event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>post_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>parent_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_CommentCreatedEvent"></a>

## Struct `CommentCreatedEvent`

Comment created event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_CommentCreatedEvent">CommentCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>comment_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>parent_comment_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_RepostEvent"></a>

## Struct `RepostEvent`

Repost event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_RepostEvent">RepostEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>repost_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>original_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>is_original_post: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_ReactionEvent"></a>

## Struct `ReactionEvent`

Reaction event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_ReactionEvent">ReactionEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>object_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reaction: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>is_post: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_RemoveReactionEvent"></a>

## Struct `RemoveReactionEvent`

Remove reaction event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_RemoveReactionEvent">RemoveReactionEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>object_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reaction: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>is_post: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_TipEvent"></a>

## Struct `TipEvent`

Tip event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>object_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>from: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>to: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>is_post: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_OwnershipTransferEvent"></a>

## Struct `OwnershipTransferEvent`

Post ownership transfer event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_OwnershipTransferEvent">OwnershipTransferEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>object_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>previous_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>is_post: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PostModerationEvent"></a>

## Struct `PostModerationEvent`

Post moderation event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostModerationEvent">PostModerationEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>removed: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>moderated_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PostUpdatedEvent"></a>

## Struct `PostUpdatedEvent`

Post updated event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostUpdatedEvent">PostUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_CommentUpdatedEvent"></a>

## Struct `CommentUpdatedEvent`

Comment updated event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_CommentUpdatedEvent">CommentUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>comment_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>updated_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PostReportedEvent"></a>

## Struct `PostReportedEvent`

Post reported event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostReportedEvent">PostReportedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>reporter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reason_code: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reported_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_CommentReportedEvent"></a>

## Struct `CommentReportedEvent`

Comment reported event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_CommentReportedEvent">CommentReportedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>comment_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reporter: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reason_code: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>reported_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PostDeletedEvent"></a>

## Struct `PostDeletedEvent`

Post deleted event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PostDeletedEvent">PostDeletedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>deleted_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_CommentDeletedEvent"></a>

## Struct `CommentDeletedEvent`

Comment deleted event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_CommentDeletedEvent">CommentDeletedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>comment_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>post_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>deleted_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PredictionCreatedEvent"></a>

## Struct `PredictionCreatedEvent`

Prediction creation event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionCreatedEvent">PredictionCreatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>prediction_data_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>content: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>options: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>betting_end_time: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PredictionBetPlacedEvent"></a>

## Struct `PredictionBetPlacedEvent`

Prediction bet placed event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionBetPlacedEvent">PredictionBetPlacedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PredictionResolvedEvent"></a>

## Struct `PredictionResolvedEvent`

Prediction resolved event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionResolvedEvent">PredictionResolvedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>winning_option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>total_bet_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>winning_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>resolved_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PredictionPayoutEvent"></a>

## Struct `PredictionPayoutEvent`

Prediction payout event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionPayoutEvent">PredictionPayoutEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PredictionBetWithdrawnEvent"></a>

## Struct `PredictionBetWithdrawnEvent`

Prediction bet withdrawn event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionBetWithdrawnEvent">PredictionBetWithdrawnEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>option_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>original_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>withdrawal_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PromotedPostCreatedEvent"></a>

## Struct `PromotedPostCreatedEvent`

Event emitted when a promoted post is created


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotedPostCreatedEvent">PromotedPostCreatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>payment_per_view: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_budget: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_post_PromotedPostViewConfirmedEvent"></a>

## Struct `PromotedPostViewConfirmedEvent`

Event emitted when a promoted post view is confirmed and payment is made


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotedPostViewConfirmedEvent">PromotedPostViewConfirmedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>viewer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>payment_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>view_duration: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <b>address</b></code>
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

<a name="social_contracts_post_PromotionStatusToggledEvent"></a>

## Struct `PromotionStatusToggledEvent`

Event emitted when promotion status is toggled


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionStatusToggledEvent">PromotionStatusToggledEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>toggled_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_status: bool</code>
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

<a name="social_contracts_post_PromotionFundsWithdrawnEvent"></a>

## Struct `PromotionFundsWithdrawnEvent`

Event emitted when promotion funds are withdrawn


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionFundsWithdrawnEvent">PromotionFundsWithdrawnEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>withdrawn_amount: u64</code>
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

<a name="social_contracts_post_ModerationRecord"></a>

## Struct `ModerationRecord`

Simple moderation record for tracking moderation decisions


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/post.md#social_contracts_post_ModerationRecord">ModerationRecord</a> <b>has</b> key
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
</dd>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>moderation_state: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>moderator: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>moderation_timestamp: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
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

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_post_COMMENTER_TIP_PERCENTAGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_COMMENTER_TIP_PERCENTAGE">COMMENTER_TIP_PERCENTAGE</a>: u64 = 80;
</code></pre>



<a name="social_contracts_post_ECommentsNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ECommentsNotAllowed">ECommentsNotAllowed</a>: u64 = 23;
</code></pre>



<a name="social_contracts_post_EContentTooLarge"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>: u64 = 5;
</code></pre>



<a name="social_contracts_post_EInsufficientPromotionFunds"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInsufficientPromotionFunds">EInsufficientPromotionFunds</a>: u64 = 36;
</code></pre>



<a name="social_contracts_post_EInvalidConfig"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>: u64 = 28;
</code></pre>



<a name="social_contracts_post_EInvalidParentReference"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>: u64 = 4;
</code></pre>



<a name="social_contracts_post_EInvalidPostType"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidPostType">EInvalidPostType</a>: u64 = 7;
</code></pre>



<a name="social_contracts_post_EInvalidTipAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>: u64 = 2;
</code></pre>



<a name="social_contracts_post_EInvalidViewDuration"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EInvalidViewDuration">EInvalidViewDuration</a>: u64 = 38;
</code></pre>



<a name="social_contracts_post_ENoEncryptedContent"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ENoEncryptedContent">ENoEncryptedContent</a>: u64 = 30;
</code></pre>



<a name="social_contracts_post_ENoSubscriptionService"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ENoSubscriptionService">ENoSubscriptionService</a>: u64 = 29;
</code></pre>



<a name="social_contracts_post_ENotPredictionPost"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ENotPredictionPost">ENotPredictionPost</a>: u64 = 16;
</code></pre>



<a name="social_contracts_post_ENotPromotedPost"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>: u64 = 34;
</code></pre>



<a name="social_contracts_post_EPostNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPostNotFound">EPostNotFound</a>: u64 = 1;
</code></pre>



<a name="social_contracts_post_EPredictionAlreadyResolved"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPredictionAlreadyResolved">EPredictionAlreadyResolved</a>: u64 = 14;
</code></pre>



<a name="social_contracts_post_EPredictionBettingClosed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPredictionBettingClosed">EPredictionBettingClosed</a>: u64 = 17;
</code></pre>



<a name="social_contracts_post_EPredictionDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPredictionDisabled">EPredictionDisabled</a>: u64 = 18;
</code></pre>



<a name="social_contracts_post_EPredictionOptionInvalid"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPredictionOptionInvalid">EPredictionOptionInvalid</a>: u64 = 15;
</code></pre>



<a name="social_contracts_post_EPredictionOptionsEmpty"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPredictionOptionsEmpty">EPredictionOptionsEmpty</a>: u64 = 13;
</code></pre>



<a name="social_contracts_post_EPredictionOptionsTooMany"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPredictionOptionsTooMany">EPredictionOptionsTooMany</a>: u64 = 12;
</code></pre>



<a name="social_contracts_post_EPriceMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPriceMismatch">EPriceMismatch</a>: u64 = 31;
</code></pre>



<a name="social_contracts_post_EPromotionAmountTooHigh"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPromotionAmountTooHigh">EPromotionAmountTooHigh</a>: u64 = 33;
</code></pre>



<a name="social_contracts_post_EPromotionAmountTooLow"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPromotionAmountTooLow">EPromotionAmountTooLow</a>: u64 = 32;
</code></pre>



<a name="social_contracts_post_EPromotionInactive"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EPromotionInactive">EPromotionInactive</a>: u64 = 37;
</code></pre>



<a name="social_contracts_post_EQuotesNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EQuotesNotAllowed">EQuotesNotAllowed</a>: u64 = 25;
</code></pre>



<a name="social_contracts_post_EReactionContentTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EReactionContentTooLong">EReactionContentTooLong</a>: u64 = 11;
</code></pre>



<a name="social_contracts_post_EReactionsNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EReactionsNotAllowed">EReactionsNotAllowed</a>: u64 = 22;
</code></pre>



<a name="social_contracts_post_EReportDescriptionTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EReportDescriptionTooLong">EReportDescriptionTooLong</a>: u64 = 10;
</code></pre>



<a name="social_contracts_post_EReportReasonInvalid"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EReportReasonInvalid">EReportReasonInvalid</a>: u64 = 9;
</code></pre>



<a name="social_contracts_post_ERepostsNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ERepostsNotAllowed">ERepostsNotAllowed</a>: u64 = 24;
</code></pre>



<a name="social_contracts_post_ESelfTipping"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ESelfTipping">ESelfTipping</a>: u64 = 3;
</code></pre>



<a name="social_contracts_post_ETipsNotAllowed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>: u64 = 26;
</code></pre>



<a name="social_contracts_post_ETooManyMediaUrls"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>: u64 = 6;
</code></pre>



<a name="social_contracts_post_EUnauthorized"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_post_EUnauthorizedTransfer"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EUnauthorizedTransfer">EUnauthorizedTransfer</a>: u64 = 8;
</code></pre>



<a name="social_contracts_post_EUserAlreadyViewed"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EUserAlreadyViewed">EUserAlreadyViewed</a>: u64 = 35;
</code></pre>



<a name="social_contracts_post_EUserBlockedByPlatform"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EUserBlockedByPlatform">EUserBlockedByPlatform</a>: u64 = 20;
</code></pre>



<a name="social_contracts_post_EUserNotJoinedPlatform"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>: u64 = 19;
</code></pre>



<a name="social_contracts_post_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>: u64 = 21;
</code></pre>



<a name="social_contracts_post_MAX_CONTENT_LENGTH"></a>

Constants for size limits


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_CONTENT_LENGTH">MAX_CONTENT_LENGTH</a>: u64 = 5000;
</code></pre>



<a name="social_contracts_post_MAX_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>: u64 = 500;
</code></pre>



<a name="social_contracts_post_MAX_MEDIA_URLS"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_MEDIA_URLS">MAX_MEDIA_URLS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_post_MAX_MENTIONS"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_MENTIONS">MAX_MENTIONS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_post_MAX_METADATA_SIZE"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_METADATA_SIZE">MAX_METADATA_SIZE</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_post_MAX_PREDICTION_OPTIONS"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_PREDICTION_OPTIONS">MAX_PREDICTION_OPTIONS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_post_MAX_PROMOTION_AMOUNT"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_PROMOTION_AMOUNT">MAX_PROMOTION_AMOUNT</a>: u64 = 100000000;
</code></pre>



<a name="social_contracts_post_MAX_REACTION_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_REACTION_LENGTH">MAX_REACTION_LENGTH</a>: u64 = 20;
</code></pre>



<a name="social_contracts_post_MAX_U64"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_post_MIN_PROMOTION_AMOUNT"></a>

Constants for promoted posts


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MIN_PROMOTION_AMOUNT">MIN_PROMOTION_AMOUNT</a>: u64 = 1000;
</code></pre>



<a name="social_contracts_post_MIN_VIEW_DURATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MIN_VIEW_DURATION">MIN_VIEW_DURATION</a>: u64 = 3000;
</code></pre>



<a name="social_contracts_post_MODERATION_APPROVED"></a>

Constants for moderation states


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MODERATION_APPROVED">MODERATION_APPROVED</a>: u8 = 1;
</code></pre>



<a name="social_contracts_post_MODERATION_FLAGGED"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_MODERATION_FLAGGED">MODERATION_FLAGGED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_post_POST_TYPE_PREDICTION"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_PREDICTION">POST_TYPE_PREDICTION</a>: vector&lt;u8&gt; = vector[112, 114, 101, 100, 105, 99, 116, 105, 111, 110];
</code></pre>



<a name="social_contracts_post_POST_TYPE_QUOTE_REPOST"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_QUOTE_REPOST">POST_TYPE_QUOTE_REPOST</a>: vector&lt;u8&gt; = vector[113, 117, 111, 116, 101, 95, 114, 101, 112, 111, 115, 116];
</code></pre>



<a name="social_contracts_post_POST_TYPE_REPOST"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_REPOST">POST_TYPE_REPOST</a>: vector&lt;u8&gt; = vector[114, 101, 112, 111, 115, 116];
</code></pre>



<a name="social_contracts_post_POST_TYPE_STANDARD"></a>

Valid post types


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>: vector&lt;u8&gt; = vector[115, 116, 97, 110, 100, 97, 114, 100];
</code></pre>



<a name="social_contracts_post_REPORT_REASON_HARASSMENT"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_HARASSMENT">REPORT_REASON_HARASSMENT</a>: u8 = 6;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_ILLEGAL"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_ILLEGAL">REPORT_REASON_ILLEGAL</a>: u8 = 4;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_IMPERSONATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_IMPERSONATION">REPORT_REASON_IMPERSONATION</a>: u8 = 5;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_MISINFORMATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_MISINFORMATION">REPORT_REASON_MISINFORMATION</a>: u8 = 3;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_OFFENSIVE"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OFFENSIVE">REPORT_REASON_OFFENSIVE</a>: u8 = 2;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_OTHER"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OTHER">REPORT_REASON_OTHER</a>: u8 = 99;
</code></pre>



<a name="social_contracts_post_REPORT_REASON_SPAM"></a>

Constants for report reason codes


<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_SPAM">REPORT_REASON_SPAM</a>: u8 = 1;
</code></pre>



<a name="social_contracts_post_REPOST_TIP_PERCENTAGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/post.md#social_contracts_post_REPOST_TIP_PERCENTAGE">REPOST_TIP_PERCENTAGE</a>: u64 = 50;
</code></pre>



<a name="social_contracts_post_is_auto_pool_disabled"></a>

## Function `is_auto_pool_disabled`

Query: per-post opt-out for auto SPT pool init


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_is_auto_pool_disabled">is_auto_pool_disabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_is_auto_pool_disabled">is_auto_pool_disabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool { <a href="../social_contracts/post.md#social_contracts_post">post</a>.disable_auto_pool }
</code></pre>



</details>

<a name="social_contracts_post_set_auto_pool_disabled"></a>

## Function `set_auto_pool_disabled`

Owner-only: set per-post opt-out flag


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_auto_pool_disabled">set_auto_pool_disabled</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, disabled: bool, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_auto_pool_disabled">set_auto_pool_disabled</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    disabled: bool,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(caller == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.disable_auto_pool = disabled;
}
</code></pre>



</details>

<a name="social_contracts_post_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the post configuration


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> admin = tx_context::sender(ctx);
    // Create and share <a href="../social_contracts/post.md#social_contracts_post">post</a> configuration with proper treasury
    transfer::share_object(
        <a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a> {
            id: object::new(ctx),
            predictions_enabled: <b>false</b>, // Predictions disabled by default
            prediction_fee_bps: 500, // Default 5% fee
            prediction_treasury: admin, // Auto-configured to admin during <a href="../social_contracts/bootstrap.md#social_contracts_bootstrap">bootstrap</a>
            max_content_length: <a href="../social_contracts/post.md#social_contracts_post_MAX_CONTENT_LENGTH">MAX_CONTENT_LENGTH</a>,
            max_media_urls: <a href="../social_contracts/post.md#social_contracts_post_MAX_MEDIA_URLS">MAX_MEDIA_URLS</a>,
            max_mentions: <a href="../social_contracts/post.md#social_contracts_post_MAX_MENTIONS">MAX_MENTIONS</a>,
            max_metadata_size: <a href="../social_contracts/post.md#social_contracts_post_MAX_METADATA_SIZE">MAX_METADATA_SIZE</a>,
            max_description_length: <a href="../social_contracts/post.md#social_contracts_post_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>,
            max_reaction_length: <a href="../social_contracts/post.md#social_contracts_post_MAX_REACTION_LENGTH">MAX_REACTION_LENGTH</a>,
            commenter_tip_percentage: <a href="../social_contracts/post.md#social_contracts_post_COMMENTER_TIP_PERCENTAGE">COMMENTER_TIP_PERCENTAGE</a>,
            repost_tip_percentage: <a href="../social_contracts/post.md#social_contracts_post_REPOST_TIP_PERCENTAGE">REPOST_TIP_PERCENTAGE</a>,
            max_prediction_options: <a href="../social_contracts/post.md#social_contracts_post_MAX_PREDICTION_OPTIONS">MAX_PREDICTION_OPTIONS</a>,
        }
    );
}
</code></pre>



</details>

<a name="social_contracts_post_set_predictions_enabled"></a>

## Function `set_predictions_enabled`

Enable or disable prediction functionality (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_predictions_enabled">set_predictions_enabled</a>(_: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">social_contracts::post::PostAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, enabled: bool, _ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_predictions_enabled">set_predictions_enabled</a>(
    _: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    enabled: bool,
    _ctx: &<b>mut</b> TxContext
) {
    // Admin capability verification is handled by type system
    // Update configuration
    config.predictions_enabled = enabled;
}
</code></pre>



</details>

<a name="social_contracts_post_set_prediction_fee"></a>

## Function `set_prediction_fee`

Set prediction fee (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_prediction_fee">set_prediction_fee</a>(_: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">social_contracts::post::PostAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, fee_bps: u64, treasury: <b>address</b>, _ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_prediction_fee">set_prediction_fee</a>(
    _: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    fee_bps: u64,
    treasury: <b>address</b>,
    _ctx: &<b>mut</b> TxContext
) {
    // Admin capability verification is handled by type system
    // Ensure fee is reasonable (max 25%)
    <b>assert</b>!(fee_bps &lt;= 2500, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    // Update configuration
    config.prediction_fee_bps = fee_bps;
    config.prediction_treasury = treasury;
}
</code></pre>



</details>

<a name="social_contracts_post_is_predictions_enabled"></a>

## Function `is_predictions_enabled`

Check if predictions are enabled


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_is_predictions_enabled">is_predictions_enabled</a>(config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_is_predictions_enabled">is_predictions_enabled</a>(config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>): bool {
    config.predictions_enabled
}
</code></pre>



</details>

<a name="social_contracts_post_create_prediction_post"></a>

## Function `create_prediction_post`

Create a new prediction post


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_prediction_post">create_prediction_post</a>(config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, _admin_cap: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">social_contracts::post::PostAdminCap</a>, registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, options: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, betting_end_time: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, allow_comments: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_reactions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_reposts: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_quotes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_tips: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_prediction_post">create_prediction_post</a>(
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    _admin_cap: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a>,
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    content: String,
    options: vector&lt;String&gt;,
    <b>mut</b> media_urls: Option&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    betting_end_time: Option&lt;u64&gt;,
    allow_comments: Option&lt;bool&gt;,
    allow_reactions: Option&lt;bool&gt;,
    allow_reposts: Option&lt;bool&gt;,
    allow_quotes: Option&lt;bool&gt;,
    allow_tips: Option&lt;bool&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Verify predictions are enabled
    <b>assert</b>!(config.predictions_enabled, <a href="../social_contracts/post.md#social_contracts_post_EPredictionDisabled">EPredictionDisabled</a>);
    <b>let</b> owner = tx_context::sender(ctx);
    // Look up the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID <b>for</b> the sender
    <b>let</b> <b>mut</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">social_contracts::profile::lookup_profile_by_owner</a>(registry, owner);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> profile_id = option::extract(&<b>mut</b> profile_id_option);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> user <b>has</b> joined the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>let</b> profile_id_obj = object::id_from_address(profile_id);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, profile_id_obj), <a href="../social_contracts/post.md#social_contracts_post_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    // Check <b>if</b> the user is blocked by the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>let</b> platform_address = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_address, owner), <a href="../social_contracts/post.md#social_contracts_post_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    // Validate content length
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate options
    <b>let</b> options_length = vector::length(&options);
    <b>assert</b>!(options_length &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EPredictionOptionsEmpty">EPredictionOptionsEmpty</a>);
    <b>assert</b>!(options_length &lt;= config.max_prediction_options, <a href="../social_contracts/post.md#social_contracts_post_EPredictionOptionsTooMany">EPredictionOptionsTooMany</a>);
    // Validate metadata size <b>if</b> provided
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_ref = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_ref) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Convert and validate media URLs <b>if</b> provided
    <b>let</b> media_option = <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> urls_bytes = option::extract(&<b>mut</b> media_urls);
        // Validate media URLs count
        <b>assert</b>!(vector::length(&urls_bytes) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        // Convert media URL bytes to Url
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&urls_bytes);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_bytes = *vector::borrow(&urls_bytes, i);
            vector::push_back(&<b>mut</b> urls, url::new_unsafe_from_bytes(url_bytes));
            i = i + 1;
        };
        option::some(urls)
    } <b>else</b> {
        option::none&lt;vector&lt;Url&gt;&gt;()
    };
    // Validate mentions <b>if</b> provided
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Set defaults <b>for</b> optional boolean parameters
    <b>let</b> final_allow_comments = <b>if</b> (option::is_some(&allow_comments)) {
        *option::borrow(&allow_comments)
    } <b>else</b> {
        <b>true</b> // Default to allowing comments
    };
    <b>let</b> final_allow_reactions = <b>if</b> (option::is_some(&allow_reactions)) {
        *option::borrow(&allow_reactions)
    } <b>else</b> {
        <b>true</b> // Default to allowing reactions
    };
    <b>let</b> final_allow_reposts = <b>if</b> (option::is_some(&allow_reposts)) {
        *option::borrow(&allow_reposts)
    } <b>else</b> {
        <b>true</b> // Default to allowing reposts
    };
    <b>let</b> final_allow_quotes = <b>if</b> (option::is_some(&allow_quotes)) {
        *option::borrow(&allow_quotes)
    } <b>else</b> {
        <b>true</b> // Default to allowing quotes
    };
    <b>let</b> final_allow_tips = <b>if</b> (option::is_some(&allow_tips)) {
        *option::borrow(&allow_tips)
    } <b>else</b> {
        <b>true</b> // Default to allowing tips
    };
    // Create the <a href="../social_contracts/post.md#social_contracts_post">post</a> with prediction type
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(
        owner,
        profile_id,
        content,
        media_option,
        mentions,
        metadata_json,
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_PREDICTION">POST_TYPE_PREDICTION</a>),
        option::none(),
        final_allow_comments,
        final_allow_reactions,
        final_allow_reposts,
        final_allow_quotes,
        final_allow_tips,
        option::none(), // poc_badge_id
        option::none(), // revenue_redirect_to
        option::none(), // revenue_redirect_percentage
        option::none(), // my_ip_id
        option::none(), // promotion_id
        ctx
    );
    // Create prediction options
    <b>let</b> <b>mut</b> prediction_options = vector::empty&lt;<a href="../social_contracts/post.md#social_contracts_post_PredictionOption">PredictionOption</a>&gt;();
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> options_len = vector::length(&options);
    <b>while</b> (i &lt; options_len) {
        <b>let</b> option_desc = *vector::borrow(&options, i);
        <b>let</b> prediction_option = <a href="../social_contracts/post.md#social_contracts_post_PredictionOption">PredictionOption</a> {
            id: (i <b>as</b> u8),
            description: option_desc,
            total_bet: 0
        };
        vector::push_back(&<b>mut</b> prediction_options, prediction_option);
        i = i + 1;
    };
    // Create prediction data
    <b>let</b> prediction_data = <a href="../social_contracts/post.md#social_contracts_post_PredictionData">PredictionData</a> {
        id: object::new(ctx),
        post_id,
        options: prediction_options,
        bets: vector::empty(),
        resolved: <b>false</b>,
        winning_option_id: option::none(),
        betting_end_time,
        total_bet_amount: 0,
    };
    <b>let</b> prediction_data_id = object::uid_to_address(&prediction_data.id);
    // Extract just the descriptions <b>for</b> the event
    <b>let</b> <b>mut</b> option_descriptions = vector::empty&lt;String&gt;();
    i = 0;
    <b>while</b> (i &lt; options_len) {
        <b>let</b> option = *vector::borrow(&prediction_options, i);
        vector::push_back(&<b>mut</b> option_descriptions, option.description);
        i = i + 1;
    };
    // Emit prediction created event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PredictionCreatedEvent">PredictionCreatedEvent</a> {
        post_id,
        prediction_data_id,
        owner,
        profile_id,
        content,
        options: option_descriptions,
        betting_end_time,
    });
    // Emit standard <a href="../social_contracts/post.md#social_contracts_post">post</a> created event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> {
        post_id,
        owner,
        profile_id,
        content,
        post_type: string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_PREDICTION">POST_TYPE_PREDICTION</a>),
        parent_post_id: option::none(),
        mentions,
    });
    // Share prediction data
    transfer::share_object(prediction_data);
}
</code></pre>



</details>

<a name="social_contracts_post_place_prediction_bet"></a>

## Function `place_prediction_bet`

Place a bet on a prediction post


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_place_prediction_bet">place_prediction_bet</a>(config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, prediction_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionData">social_contracts::post::PredictionData</a>, option_id: u8, coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_place_prediction_bet">place_prediction_bet</a>(
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    prediction_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionData">PredictionData</a>,
    option_id: u8,
    coin: &<b>mut</b> Coin&lt;MYS&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Verify predictions are enabled
    <b>assert</b>!(config.predictions_enabled, <a href="../social_contracts/post.md#social_contracts_post_EPredictionDisabled">EPredictionDisabled</a>);
    <b>let</b> bettor = tx_context::sender(ctx);
    // Verify this is a prediction <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_PREDICTION">POST_TYPE_PREDICTION</a>) == <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type, <a href="../social_contracts/post.md#social_contracts_post_ENotPredictionPost">ENotPredictionPost</a>);
    // Verify post_id matches
    <b>assert</b>!(object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id) == prediction_data.post_id, <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>);
    // Verify prediction is not resolved yet
    <b>assert</b>!(!prediction_data.resolved, <a href="../social_contracts/post.md#social_contracts_post_EPredictionAlreadyResolved">EPredictionAlreadyResolved</a>);
    // Check <b>if</b> betting period <b>has</b> ended
    <b>if</b> (option::is_some(&prediction_data.betting_end_time)) {
        <b>let</b> end_time = *option::borrow(&prediction_data.betting_end_time);
        <b>assert</b>!(tx_context::epoch(ctx) &lt;= end_time, <a href="../social_contracts/post.md#social_contracts_post_EPredictionBettingClosed">EPredictionBettingClosed</a>);
    };
    // Verify option_id is valid
    <b>let</b> <b>mut</b> option_valid = <b>false</b>;
    <b>let</b> <b>mut</b> option_index = 0;
    <b>let</b> options_len = vector::length(&prediction_data.options);
    <b>while</b> (option_index &lt; options_len) {
        <b>let</b> option = vector::borrow_mut(&<b>mut</b> prediction_data.options, option_index);
        <b>if</b> (option.id == option_id) {
            option_valid = <b>true</b>;
            // Update total bet <b>for</b> this option
            option.total_bet = option.total_bet + amount;
            <b>break</b>
        };
        option_index = option_index + 1;
    };
    <b>assert</b>!(option_valid, <a href="../social_contracts/post.md#social_contracts_post_EPredictionOptionInvalid">EPredictionOptionInvalid</a>);
    // Take bet amount from user's coin
    <b>let</b> bet_coin = coin::split(coin, amount, ctx);
    // Transfer bet to <a href="../social_contracts/post.md#social_contracts_post">post</a> owner (held until resolution)
    transfer::public_transfer(bet_coin, <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner);
    // Record bet
    <b>let</b> bet = <a href="../social_contracts/post.md#social_contracts_post_PredictionBet">PredictionBet</a> {
        user: bettor,
        option_id,
        amount,
        timestamp: tx_context::epoch(ctx),
    };
    // Add bet to prediction data
    vector::push_back(&<b>mut</b> prediction_data.bets, bet);
    // Update total bet amount
    prediction_data.total_bet_amount = prediction_data.total_bet_amount + amount;
    // Emit bet placed event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PredictionBetPlacedEvent">PredictionBetPlacedEvent</a> {
        post_id: prediction_data.post_id,
        user: bettor,
        option_id,
        amount,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_withdraw_prediction_bet"></a>

## Function `withdraw_prediction_bet`

Withdraw a prediction bet with adjusted returns based on current odds


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_withdraw_prediction_bet">withdraw_prediction_bet</a>(config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, prediction_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionData">social_contracts::post::PredictionData</a>, repayment_coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_withdraw_prediction_bet">withdraw_prediction_bet</a>(
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    prediction_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionData">PredictionData</a>,
    repayment_coin: &<b>mut</b> Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Verify predictions are enabled
    <b>assert</b>!(config.predictions_enabled, <a href="../social_contracts/post.md#social_contracts_post_EPredictionDisabled">EPredictionDisabled</a>);
    <b>let</b> withdrawer = tx_context::sender(ctx);
    // Verify this is a prediction <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_PREDICTION">POST_TYPE_PREDICTION</a>) == <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type, <a href="../social_contracts/post.md#social_contracts_post_ENotPredictionPost">ENotPredictionPost</a>);
    // Verify post_id matches
    <b>assert</b>!(object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id) == prediction_data.post_id, <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>);
    // Verify prediction is not resolved yet
    <b>assert</b>!(!prediction_data.resolved, <a href="../social_contracts/post.md#social_contracts_post_EPredictionAlreadyResolved">EPredictionAlreadyResolved</a>);
    // Check <b>if</b> betting period <b>has</b> ended
    <b>if</b> (option::is_some(&prediction_data.betting_end_time)) {
        <b>let</b> end_time = *option::borrow(&prediction_data.betting_end_time);
        <b>assert</b>!(tx_context::epoch(ctx) &lt;= end_time, <a href="../social_contracts/post.md#social_contracts_post_EPredictionBettingClosed">EPredictionBettingClosed</a>);
    };
    // Find the user's bet
    <b>let</b> bets_len = vector::length(&prediction_data.bets);
    <b>let</b> <b>mut</b> bet_index = 0;
    <b>let</b> <b>mut</b> found_bet = <b>false</b>;
    <b>let</b> <b>mut</b> user_bet_amount = 0;
    <b>let</b> <b>mut</b> user_option_id = 0;
    <b>while</b> (bet_index &lt; bets_len) {
        <b>let</b> bet = vector::borrow(&prediction_data.bets, bet_index);
        <b>if</b> (bet.user == withdrawer) {
            user_bet_amount = bet.amount;
            user_option_id = bet.option_id;
            found_bet = <b>true</b>;
            <b>break</b>
        };
        bet_index = bet_index + 1;
    };
    // Ensure the user <b>has</b> a bet to withdraw
    <b>assert</b>!(found_bet, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Calculate the current odds and determine the fair withdrawal amount
    // Get the total amount bet across all options
    <b>let</b> total_bet_amount = prediction_data.total_bet_amount;
    // Get current amount betting settings
    <b>let</b> options_len = vector::length(&prediction_data.options);
    <b>let</b> <b>mut</b> option_index = 0;
    <b>while</b> (option_index &lt; options_len) {
        <b>let</b> option = vector::borrow(&prediction_data.options, option_index);
        <b>if</b> (option.id == user_option_id) {
            <b>break</b>
        };
        option_index = option_index + 1;
    };
    // Calculate the fair withdrawal amount based on current odds
    // Formula: withdrawal_amount = user_bet_amount * (total_bet_amount - user_bet_amount) / (total_bet_amount)
    // Remove the user's bet from the calculation to get actual current market
    <b>let</b> adjusted_total_bet = total_bet_amount - user_bet_amount;
    // Calculate the withdrawal amount (using proportion of current odds)
    <b>let</b> <b>mut</b> withdrawal_amount = user_bet_amount;
    // Only adjust <b>if</b> there are other bets in the market
    <b>if</b> (adjusted_total_bet &gt; 0) {
        // Calculate fair value based on current odds
        // This formula ensures users get less <b>if</b> odds worsened, more <b>if</b> odds improved
        withdrawal_amount = (((user_bet_amount <b>as</b> u128) * (adjusted_total_bet <b>as</b> u128)) /
            (adjusted_total_bet <b>as</b> u128)) <b>as</b> u64;
    };
    // Ensure there's enough balance in the repayment coin
    <b>assert</b>!(coin::value(repayment_coin) &gt;= withdrawal_amount, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    // Update prediction data
    // 1. Decrease the total bet amount
    prediction_data.total_bet_amount = prediction_data.total_bet_amount - user_bet_amount;
    // 2. Decrease the option's total bet amount
    option_index = 0;
    <b>while</b> (option_index &lt; options_len) {
        <b>let</b> option = vector::borrow_mut(&<b>mut</b> prediction_data.options, option_index);
        <b>if</b> (option.id == user_option_id) {
            option.total_bet = option.total_bet - user_bet_amount;
            <b>break</b>
        };
        option_index = option_index + 1;
    };
    // 3. Remove the bet from the vector
    <b>if</b> (bet_index &lt; bets_len - 1) {
        // If not the last element, swap with last and pop
        vector::swap(&<b>mut</b> prediction_data.bets, bet_index, bets_len - 1);
    };
    vector::pop_back(&<b>mut</b> prediction_data.bets);
    // Transfer the withdrawal amount to the user
    <b>let</b> withdrawal_coin = coin::split(repayment_coin, withdrawal_amount, ctx);
    transfer::public_transfer(withdrawal_coin, withdrawer);
    // Emit prediction bet withdrawn event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PredictionBetWithdrawnEvent">PredictionBetWithdrawnEvent</a> {
        post_id: prediction_data.post_id,
        user: withdrawer,
        option_id: user_option_id,
        original_amount: user_bet_amount,
        withdrawal_amount,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_resolve_prediction"></a>

## Function `resolve_prediction`

Resolve a prediction (admin only) and distribute winnings


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_resolve_prediction">resolve_prediction</a>(config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, _admin_cap: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">social_contracts::post::PostAdminCap</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, prediction_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionData">social_contracts::post::PredictionData</a>, winning_option_id: u8, payout_funds: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_resolve_prediction">resolve_prediction</a>(
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    _admin_cap: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    prediction_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PredictionData">PredictionData</a>,
    winning_option_id: u8,
    payout_funds: &<b>mut</b> Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Verify predictions are enabled
    <b>assert</b>!(config.predictions_enabled, <a href="../social_contracts/post.md#social_contracts_post_EPredictionDisabled">EPredictionDisabled</a>);
    // Verify this is a prediction <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_PREDICTION">POST_TYPE_PREDICTION</a>) == <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type, <a href="../social_contracts/post.md#social_contracts_post_ENotPredictionPost">ENotPredictionPost</a>);
    // Verify post_id matches
    <b>assert</b>!(object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id) == prediction_data.post_id, <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>);
    // Verify prediction is not already resolved
    <b>assert</b>!(!prediction_data.resolved, <a href="../social_contracts/post.md#social_contracts_post_EPredictionAlreadyResolved">EPredictionAlreadyResolved</a>);
    // Verify option_id is valid
    <b>let</b> <b>mut</b> option_valid = <b>false</b>;
    <b>let</b> <b>mut</b> option_index = 0;
    <b>let</b> options_len = vector::length(&prediction_data.options);
    <b>let</b> <b>mut</b> winning_amount = 0;
    <b>while</b> (option_index &lt; options_len) {
        <b>let</b> option = vector::borrow(&prediction_data.options, option_index);
        <b>if</b> (option.id == winning_option_id) {
            option_valid = <b>true</b>;
            winning_amount = option.total_bet;
            <b>break</b>
        };
        option_index = option_index + 1;
    };
    <b>assert</b>!(option_valid, <a href="../social_contracts/post.md#social_contracts_post_EPredictionOptionInvalid">EPredictionOptionInvalid</a>);
    // Mark prediction <b>as</b> resolved
    prediction_data.resolved = <b>true</b>;
    prediction_data.winning_option_id = option::some(winning_option_id);
    // Emit prediction resolved event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PredictionResolvedEvent">PredictionResolvedEvent</a> {
        post_id: prediction_data.post_id,
        winning_option_id,
        total_bet_amount: prediction_data.total_bet_amount,
        winning_amount,
        resolved_by: tx_context::sender(ctx),
    });
    // Distribute all winnings automatically
    // Calculate <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee
    <b>let</b> total_bet_amount = prediction_data.total_bet_amount;
    <b>let</b> fee_amount = (total_bet_amount * config.prediction_fee_bps) / 10000;
    <b>let</b> distributable_amount = total_bet_amount - fee_amount;
    // Get all winners and their bet amounts
    <b>let</b> <b>mut</b> winners = vector::empty&lt;<b>address</b>&gt;();
    <b>let</b> <b>mut</b> winner_amounts = vector::empty&lt;u64&gt;();
    <b>let</b> <b>mut</b> winner_payouts = vector::empty&lt;u64&gt;();
    <b>let</b> <b>mut</b> total_payout = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> bets_len = vector::length(&prediction_data.bets);
    // First pass - identify winners and their bet amounts
    <b>while</b> (i &lt; bets_len) {
        <b>let</b> bet = vector::borrow(&prediction_data.bets, i);
        <b>if</b> (bet.option_id == winning_option_id) {
            <b>let</b> winner = bet.user;
            <b>let</b> bet_amount = bet.amount;
            // Check <b>if</b> this user is already in the winners list
            <b>let</b> <b>mut</b> found = <b>false</b>;
            <b>let</b> <b>mut</b> winner_index = 0;
            <b>let</b> winners_len = vector::length(&winners);
            <b>while</b> (winner_index &lt; winners_len && !found) {
                <b>if</b> (*vector::borrow(&winners, winner_index) == winner) {
                    found = <b>true</b>;
                    // Add to their existing bet amount
                    <b>let</b> current_amount = vector::borrow_mut(&<b>mut</b> winner_amounts, winner_index);
                    *current_amount = *current_amount + bet_amount;
                };
                winner_index = winner_index + 1;
            };
            <b>if</b> (!found) {
                // Add new winner
                vector::push_back(&<b>mut</b> winners, winner);
                vector::push_back(&<b>mut</b> winner_amounts, bet_amount);
            };
        };
        i = i + 1;
    };
    // Calculate payouts based on proportion of winning bets
    i = 0;
    <b>let</b> winners_len = vector::length(&winners);
    // Calculate payout ratios
    <b>while</b> (i &lt; winners_len) {
        <b>let</b> bet_amount = *vector::borrow(&winner_amounts, i);
        // Calculate payout based on proportion of winning bets
        <b>let</b> payout = <b>if</b> (winning_amount == 0) {
            0 // Avoid division by zero
        } <b>else</b> {
            (((bet_amount <b>as</b> u128) * (distributable_amount <b>as</b> u128)) / (winning_amount <b>as</b> u128)) <b>as</b> u64
        };
        vector::push_back(&<b>mut</b> winner_payouts, payout);
        total_payout = total_payout + payout;
        i = i + 1;
    };
    // Ensure we have enough funds to distribute, including fee
    <b>assert</b>!(coin::value(payout_funds) &gt;= total_bet_amount, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    // First send the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> fee <b>if</b> applicable
    <b>if</b> (fee_amount &gt; 0) {
        <b>let</b> fee_coin = coin::split(payout_funds, fee_amount, ctx);
        transfer::public_transfer(fee_coin, config.prediction_treasury);
    };
    // Distribute to all winners
    i = 0;
    <b>while</b> (i &lt; winners_len) {
        <b>let</b> winner = *vector::borrow(&winners, i);
        <b>let</b> amount = *vector::borrow(&winner_payouts, i);
        <b>if</b> (amount &gt; 0) {
            <b>let</b> payment = coin::split(payout_funds, amount, ctx);
            transfer::public_transfer(payment, winner);
            // Emit payout event
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_PredictionPayoutEvent">PredictionPayoutEvent</a> {
                post_id: prediction_data.post_id,
                user: winner,
                amount,
            });
        };
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_post_create_post_internal"></a>

## Function `create_post_internal`

Internal function to create a post and return its ID


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(owner: <b>address</b>, profile_id: <b>address</b>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, media_option: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, post_type: <a href="../std/string.md#std_string_String">std::string::String</a>, parent_post_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, allow_comments: bool, allow_reactions: bool, allow_reposts: bool, allow_quotes: bool, allow_tips: bool, poc_badge_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;, revenue_redirect_to: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, revenue_redirect_percentage: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, my_ip_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, promotion_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(
    owner: <b>address</b>,
    profile_id: <b>address</b>,
    content: String,
    media_option: Option&lt;vector&lt;Url&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    post_type: String,
    parent_post_id: Option&lt;<b>address</b>&gt;,
    allow_comments: bool,
    allow_reactions: bool,
    allow_reposts: bool,
    allow_quotes: bool,
    allow_tips: bool,
    poc_badge_id: Option&lt;ID&gt;,
    revenue_redirect_to: Option&lt;<b>address</b>&gt;,
    revenue_redirect_percentage: Option&lt;u64&gt;,
    my_ip_id: Option&lt;<b>address</b>&gt;,
    promotion_id: Option&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext
): <b>address</b> {
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> = <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a> {
        id: object::new(ctx),
        owner,
        profile_id,
        content,
        media: media_option,
        mentions,
        metadata_json,
        post_type,
        parent_post_id,
        created_at: tx_context::epoch(ctx),
        reaction_count: 0,
        comment_count: 0,
        repost_count: 0,
        tips_received: 0,
        removed_from_platform: <b>false</b>,
        user_reactions: table::new(ctx),
        reaction_counts: table::new(ctx),
        allow_comments,
        allow_reactions,
        allow_reposts,
        allow_quotes,
        allow_tips,
        poc_badge_id,
        revenue_redirect_to,
        revenue_redirect_percentage,
        my_ip_id,
        promotion_id,
        disable_auto_pool: <b>false</b>,
        <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Get <a href="../social_contracts/post.md#social_contracts_post">post</a> ID before sharing
    <b>let</b> post_id = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    // Share object
    transfer::share_object(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    // Return the <a href="../social_contracts/post.md#social_contracts_post">post</a> ID
    post_id
}
</code></pre>



</details>

<a name="social_contracts_post_create_post"></a>

## Function `create_post`

Create a new post with interaction permissions


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post">create_post</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, allow_comments: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_reactions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_reposts: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_quotes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_tips: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post">create_post</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    content: String,
    <b>mut</b> media_urls: Option&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    allow_comments: Option&lt;bool&gt;,
    allow_reactions: Option&lt;bool&gt;,
    allow_reposts: Option&lt;bool&gt;,
    allow_quotes: Option&lt;bool&gt;,
    allow_tips: Option&lt;bool&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> owner = tx_context::sender(ctx);
    // Look up the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID <b>for</b> the sender (<b>for</b> reference, not ownership)
    <b>let</b> <b>mut</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">social_contracts::profile::lookup_profile_by_owner</a>(registry, owner);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> profile_id = option::extract(&<b>mut</b> profile_id_option);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> user <b>has</b> joined the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>let</b> profile_id_obj = object::id_from_address(profile_id);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, profile_id_obj), <a href="../social_contracts/post.md#social_contracts_post_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    // Check <b>if</b> the user is blocked by the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>let</b> platform_address = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_address, owner), <a href="../social_contracts/post.md#social_contracts_post_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    // Validate content length using config
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate metadata size <b>if</b> provided
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_ref = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_ref) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Convert and validate media URLs <b>if</b> provided
    <b>let</b> media_option = <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> urls_bytes = option::extract(&<b>mut</b> media_urls);
        // Validate media URLs count using config
        <b>assert</b>!(vector::length(&urls_bytes) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        // Convert media URL bytes to Url
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&urls_bytes);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_bytes = *vector::borrow(&urls_bytes, i);
            vector::push_back(&<b>mut</b> urls, url::new_unsafe_from_bytes(url_bytes));
            i = i + 1;
        };
        option::some(urls)
    } <b>else</b> {
        option::none&lt;vector&lt;Url&gt;&gt;()
    };
    // Validate mentions <b>if</b> provided using config
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Set defaults <b>for</b> optional boolean parameters
    <b>let</b> final_allow_comments = <b>if</b> (option::is_some(&allow_comments)) {
        *option::borrow(&allow_comments)
    } <b>else</b> {
        <b>true</b> // Default to allowing comments
    };
    <b>let</b> final_allow_reactions = <b>if</b> (option::is_some(&allow_reactions)) {
        *option::borrow(&allow_reactions)
    } <b>else</b> {
        <b>true</b> // Default to allowing reactions
    };
    <b>let</b> final_allow_reposts = <b>if</b> (option::is_some(&allow_reposts)) {
        *option::borrow(&allow_reposts)
    } <b>else</b> {
        <b>true</b> // Default to allowing reposts
    };
    <b>let</b> final_allow_quotes = <b>if</b> (option::is_some(&allow_quotes)) {
        *option::borrow(&allow_quotes)
    } <b>else</b> {
        <b>true</b> // Default to allowing quotes
    };
    <b>let</b> final_allow_tips = <b>if</b> (option::is_some(&allow_tips)) {
        *option::borrow(&allow_tips)
    } <b>else</b> {
        <b>true</b> // Default to allowing tips
    };
    // Create and share the <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(
        owner,
        profile_id,
        content,
        media_option,
        mentions,
        metadata_json,
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>),
        option::none(),
        final_allow_comments,
        final_allow_reactions,
        final_allow_reposts,
        final_allow_quotes,
        final_allow_tips,
        option::none(), // poc_badge_id
        option::none(), // revenue_redirect_to
        option::none(), // revenue_redirect_percentage
        option::none(), // my_ip_id
        option::none(), // promotion_id
        ctx
    );
    // Emit <a href="../social_contracts/post.md#social_contracts_post">post</a> created event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> {
        post_id,
        owner,
        profile_id,
        content,
        post_type: string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>),
        parent_post_id: option::none(),
        mentions,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_create_comment"></a>

## Function `create_comment`

Create a comment on a post or a reply to another comment
Returns the ID of the created comment


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_comment">create_comment</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, parent_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, parent_comment_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, content: <a href="../std/string.md#std_string_String">std::string::String</a>, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_comment">create_comment</a>(
    registry: &UsernameRegistry,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &BlockListRegistry,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    parent_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    parent_comment_id: Option&lt;<b>address</b>&gt;,
    content: String,
    <b>mut</b> media_urls: Option&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    ctx: &<b>mut</b> TxContext
): <b>address</b> {
    <b>let</b> owner = tx_context::sender(ctx);
    // Look up the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID <b>for</b> the sender
    <b>let</b> <b>mut</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">social_contracts::profile::lookup_profile_by_owner</a>(registry, owner);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> profile_id = option::extract(&<b>mut</b> profile_id_option);
    // Check <b>if</b> user <b>has</b> joined the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>let</b> profile_id_obj = object::id_from_address(profile_id);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, profile_id_obj), <a href="../social_contracts/post.md#social_contracts_post_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    // Check <b>if</b> the user is blocked by the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>let</b> platform_address = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_address, owner), <a href="../social_contracts/post.md#social_contracts_post_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    // Check <b>if</b> the caller is blocked by the <a href="../social_contracts/post.md#social_contracts_post">post</a> creator
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, parent_post.owner, owner), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> comments are allowed on the parent <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(parent_post.allow_comments, <a href="../social_contracts/post.md#social_contracts_post_ECommentsNotAllowed">ECommentsNotAllowed</a>);
    // Validate content length using config
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate metadata size <b>if</b> provided
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_ref = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_ref) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Convert and validate media URLs <b>if</b> provided
    <b>let</b> media_option = <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> urls_bytes = option::extract(&<b>mut</b> media_urls);
        // Validate media URLs count using config
        <b>assert</b>!(vector::length(&urls_bytes) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        // Convert media URL bytes to Url objects
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&urls_bytes);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_bytes = *vector::borrow(&urls_bytes, i);
            vector::push_back(&<b>mut</b> urls, url::new_unsafe_from_bytes(url_bytes));
            i = i + 1;
        };
        option::some(urls)
    } <b>else</b> {
        option::none&lt;vector&lt;Url&gt;&gt;()
    };
    // Validate mentions <b>if</b> provided using config
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Get parent <a href="../social_contracts/post.md#social_contracts_post">post</a> ID
    <b>let</b> parent_post_id = object::uid_to_address(&parent_post.id);
    // Create a proper <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a> object instead of reusing <a href="../social_contracts/post.md#social_contracts_post">post</a> structure
    <b>let</b> comment = <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a> {
        id: object::new(ctx),
        post_id: parent_post_id,
        parent_comment_id,
        owner,
        profile_id,
        content,
        media: media_option,
        mentions,
        metadata_json,
        created_at: tx_context::epoch(ctx),
        reaction_count: 0,
        comment_count: 0,
        repost_count: 0,
        tips_received: 0,
        removed_from_platform: <b>false</b>,
        user_reactions: table::new(ctx),
        reaction_counts: table::new(ctx),
        <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    // Get comment ID before sharing
    <b>let</b> comment_id = object::uid_to_address(&comment.id);
    // Increment the parent <a href="../social_contracts/post.md#social_contracts_post">post</a>'s comment count with overflow protection
    // Stop incrementing at max but allow commenting to <b>continue</b>
    <b>if</b> (parent_post.comment_count &lt; <a href="../social_contracts/post.md#social_contracts_post_MAX_U64">MAX_U64</a>) {
        parent_post.comment_count = parent_post.comment_count + 1;
    };
    // Emit comment created event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_CommentCreatedEvent">CommentCreatedEvent</a> {
        comment_id,
        post_id: parent_post_id,
        parent_comment_id,
        owner,
        profile_id,
        content,
        mentions,
    });
    // Share the comment object
    transfer::share_object(comment);
    // Return the comment ID to the caller
    comment_id
}
</code></pre>



</details>

<a name="social_contracts_post_create_repost"></a>

## Function `create_repost`

Create a repost or quote repost depending on provided parameters
If content is provided, it's treated as a quote repost
If content is empty/none, it's treated as a standard repost


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_repost">create_repost</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, original_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, content: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, allow_comments: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_reactions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_reposts: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_quotes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, allow_tips: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_repost">create_repost</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    original_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    <b>mut</b> content: Option&lt;String&gt;,
    <b>mut</b> media_urls: Option&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    allow_comments: Option&lt;bool&gt;,
    allow_reactions: Option&lt;bool&gt;,
    allow_reposts: Option&lt;bool&gt;,
    allow_quotes: Option&lt;bool&gt;,
    allow_tips: Option&lt;bool&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> owner = tx_context::sender(ctx);
    // Look up the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID <b>for</b> the sender (<b>for</b> reference, not ownership)
    <b>let</b> <b>mut</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">social_contracts::profile::lookup_profile_by_owner</a>(registry, owner);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> profile_id = option::extract(&<b>mut</b> profile_id_option);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> user <b>has</b> joined the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>let</b> profile_id_obj = object::id_from_address(profile_id);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">platform::has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, profile_id_obj), <a href="../social_contracts/post.md#social_contracts_post_EUserNotJoinedPlatform">EUserNotJoinedPlatform</a>);
    // Check <b>if</b> the user is blocked by the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>let</b> platform_address = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_address, owner), <a href="../social_contracts/post.md#social_contracts_post_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    <b>let</b> original_post_id = object::uid_to_address(&original_post.id);
    // Determine <b>if</b> this is a quote repost or standard repost
    <b>let</b> is_quote_repost = option::is_some(&content) && string::length(option::borrow(&content)) &gt; 0;
    // Check <a href="../social_contracts/post.md#social_contracts_post">post</a> permissions directly
    <b>if</b> (is_quote_repost) {
        // For quote reposts, check <b>if</b> quoting is allowed
        <b>assert</b>!(original_post.allow_quotes, <a href="../social_contracts/post.md#social_contracts_post_EQuotesNotAllowed">EQuotesNotAllowed</a>);
    } <b>else</b> {
        // For regular reposts, check <b>if</b> reposting is allowed
        <b>assert</b>!(original_post.allow_reposts, <a href="../social_contracts/post.md#social_contracts_post_ERepostsNotAllowed">ERepostsNotAllowed</a>);
    };
    // Initialize content string
    <b>let</b> content_string = <b>if</b> (is_quote_repost) {
        // Validate content length <b>for</b> quote reposts
        <b>let</b> content_value = option::extract(&<b>mut</b> content);
        // Use config value instead of hardcoded constant
        <b>assert</b>!(string::length(&content_value) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        content_value
    } <b>else</b> {
        // Empty string <b>for</b> standard reposts
        string::utf8(b"")
    };
    // Validate and process media URLs <b>if</b> provided
    <b>let</b> media_option = <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> urls_bytes = option::extract(&<b>mut</b> media_urls);
        // Validate media URLs count
        <b>assert</b>!(vector::length(&urls_bytes) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        // Convert media URL bytes to Url
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&urls_bytes);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_bytes = *vector::borrow(&urls_bytes, i);
            vector::push_back(&<b>mut</b> urls, url::new_unsafe_from_bytes(url_bytes));
            i = i + 1;
        };
        option::some(urls)
    } <b>else</b> {
        option::none&lt;vector&lt;Url&gt;&gt;()
    };
    // Validate metadata size <b>if</b> provided
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_ref = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_ref) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Validate mentions <b>if</b> provided
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Create repost <b>as</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> with appropriate type
    <b>let</b> post_type = <b>if</b> (is_quote_repost) {
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_QUOTE_REPOST">POST_TYPE_QUOTE_REPOST</a>)
    } <b>else</b> {
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_REPOST">POST_TYPE_REPOST</a>)
    };
    // For standard reposts, also create a <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a> object
    <b>if</b> (!is_quote_repost) {
        <b>let</b> repost = <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a> {
            id: object::new(ctx),
            original_id: original_post_id,
            is_original_post: <b>true</b>,
            owner,
            profile_id,
            created_at: tx_context::epoch(ctx),
            <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        };
        // Get repost ID before sharing
        <b>let</b> repost_id = object::uid_to_address(&repost.id);
        // Emit repost event before sharing
        event::emit(<a href="../social_contracts/post.md#social_contracts_post_RepostEvent">RepostEvent</a> {
            repost_id,
            original_id: original_post_id,
            is_original_post: <b>true</b>,
            owner,
            profile_id,
        });
        // Share repost object
        transfer::share_object(repost);
    };
    // Increment original <a href="../social_contracts/post.md#social_contracts_post">post</a> repost count
    original_post.repost_count = original_post.repost_count + 1;
    // Set defaults <b>for</b> optional boolean parameters
    <b>let</b> final_allow_comments = <b>if</b> (option::is_some(&allow_comments)) {
        *option::borrow(&allow_comments)
    } <b>else</b> {
        <b>true</b> // Default to allowing comments
    };
    <b>let</b> final_allow_reactions = <b>if</b> (option::is_some(&allow_reactions)) {
        *option::borrow(&allow_reactions)
    } <b>else</b> {
        <b>true</b> // Default to allowing reactions
    };
    <b>let</b> final_allow_reposts = <b>if</b> (option::is_some(&allow_reposts)) {
        *option::borrow(&allow_reposts)
    } <b>else</b> {
        <b>true</b> // Default to allowing reposts
    };
    <b>let</b> final_allow_quotes = <b>if</b> (option::is_some(&allow_quotes)) {
        *option::borrow(&allow_quotes)
    } <b>else</b> {
        <b>true</b> // Default to allowing quotes
    };
    <b>let</b> final_allow_tips = <b>if</b> (option::is_some(&allow_tips)) {
        *option::borrow(&allow_tips)
    } <b>else</b> {
        <b>true</b> // Default to allowing tips
    };
    // Create and share the repost <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>let</b> repost_post_id = <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(
        owner,
        profile_id,
        content_string,
        media_option,
        mentions,
        metadata_json,
        post_type,
        option::some(original_post_id),
        final_allow_comments,
        final_allow_reactions,
        final_allow_reposts,
        final_allow_quotes,
        final_allow_tips,
        option::none(), // poc_badge_id
        option::none(), // revenue_redirect_to
        option::none(), // revenue_redirect_percentage
        option::none(), // No MyIP <b>for</b> reposts
        option::none(), // promotion_id
        ctx
    );
    // Emit <a href="../social_contracts/post.md#social_contracts_post">post</a> created event <b>for</b> the repost
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostCreatedEvent">PostCreatedEvent</a> {
        post_id: repost_post_id,
        owner,
        profile_id,
        content: content_string,
        post_type,
        parent_post_id: option::some(original_post_id),
        mentions,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_delete_post"></a>

## Function `delete_post`

Delete a post owned by the caller


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_delete_post">delete_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_delete_post">delete_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(sender == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Emit event <b>for</b> the <a href="../social_contracts/post.md#social_contracts_post">post</a> deletion
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostDeletedEvent">PostDeletedEvent</a> {
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        owner: <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
        profile_id: <a href="../social_contracts/post.md#social_contracts_post">post</a>.profile_id,
        post_type: <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type,
        deleted_at: tx_context::epoch(ctx)
    });
    // Extract UID to delete the <a href="../social_contracts/post.md#social_contracts_post">post</a> object
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a> {
        id,
        owner: _,
        profile_id: _,
        content: _,
        media: _,
        mentions: _,
        metadata_json: _,
        post_type: _,
        parent_post_id: _,
        created_at: _,
        reaction_count: _,
        comment_count: _,
        repost_count: _,
        tips_received: _,
        removed_from_platform: _,
        user_reactions,
        reaction_counts,
        allow_comments: _,
        allow_reactions: _,
        allow_reposts: _,
        allow_quotes: _,
        allow_tips: _,
        poc_badge_id: _,
        revenue_redirect_to: _,
        revenue_redirect_percentage: _,
        my_ip_id: _,
        promotion_id: _,
        disable_auto_pool: _,
        <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: _,
    } = <a href="../social_contracts/post.md#social_contracts_post">post</a>;
    // Clean up associated data structures
    table::drop(user_reactions);
    table::drop(reaction_counts);
    // Delete the <a href="../social_contracts/post.md#social_contracts_post">post</a> object
    object::delete(id);
}
</code></pre>



</details>

<a name="social_contracts_post_delete_comment"></a>

## Function `delete_comment`

Delete a comment owned by the caller


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_delete_comment">delete_comment</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, comment: <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_delete_comment">delete_comment</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    comment: <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(sender == comment.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Verify the comment belongs to this <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>let</b> comment_post_id = comment.post_id;
    <b>let</b> post_id = object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id);
    <b>assert</b>!(comment_post_id == post_id, <a href="../social_contracts/post.md#social_contracts_post_EPostNotFound">EPostNotFound</a>);
    // Decrement the <a href="../social_contracts/post.md#social_contracts_post">post</a>'s comment count
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.comment_count = <a href="../social_contracts/post.md#social_contracts_post">post</a>.comment_count - 1;
    // Emit event <b>for</b> the comment deletion
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_CommentDeletedEvent">CommentDeletedEvent</a> {
        comment_id: object::uid_to_address(&comment.id),
        post_id,
        owner: comment.owner,
        profile_id: comment.profile_id,
        deleted_at: tx_context::epoch(ctx)
    });
    // Extract UID to delete the comment object
    <b>let</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a> {
        id,
        post_id: _,
        parent_comment_id: _,
        owner: _,
        profile_id: _,
        content: _,
        media: _,
        mentions: _,
        metadata_json: _,
        created_at: _,
        reaction_count: _,
        comment_count: _,
        repost_count: _,
        tips_received: _,
        removed_from_platform: _,
        user_reactions,
        reaction_counts,
        <a href="../social_contracts/post.md#social_contracts_post_version">version</a>: _,
    } = comment;
    // Clean up associated data structures
    table::drop(user_reactions);
    table::drop(reaction_counts);
    // Delete the comment object
    object::delete(id);
}
</code></pre>



</details>

<a name="social_contracts_post_react_to_post"></a>

## Function `react_to_post`

React to a post with a specific reaction (emoji or text)
If the user already has the exact same reaction, it will be removed (toggle behavior)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_react_to_post">react_to_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, reaction: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_react_to_post">react_to_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    reaction: String,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> user = tx_context::sender(ctx);
    // Validate reaction length using config
    <b>assert</b>!(string::length(&reaction) &lt;= config.max_reaction_length, <a href="../social_contracts/post.md#social_contracts_post_EReactionContentTooLong">EReactionContentTooLong</a>);
    // Check <b>if</b> reactions are allowed on this <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.allow_reactions, <a href="../social_contracts/post.md#social_contracts_post_EReactionsNotAllowed">EReactionsNotAllowed</a>);
    // Check <b>if</b> user already reacted to the <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>if</b> (table::contains(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, user)) {
        // Get the previous reaction
        <b>let</b> previous_reaction = *table::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, user);
        // If the reaction is the same, remove it (toggle behavior)
        <b>if</b> (reaction == previous_reaction) {
            // Remove user's reaction
            table::remove(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, user);
            // Decrease count <b>for</b> this reaction type
            <b>let</b> count = *table::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction);
            <b>if</b> (count &lt;= 1) {
                table::remove(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction);
            } <b>else</b> {
                *table::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction) = count - 1;
            };
            // Decrement <a href="../social_contracts/post.md#social_contracts_post">post</a> reaction count
            <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count = <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count - 1;
            // Emit remove reaction event
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_RemoveReactionEvent">RemoveReactionEvent</a> {
                object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
                user,
                reaction,
                is_post: <b>true</b>,
            });
            <b>return</b>
        };
        // Different reaction, update existing one
        // Decrease count <b>for</b> previous reaction
        <b>let</b> previous_count = *table::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, previous_reaction);
        <b>if</b> (previous_count &lt;= 1) {
            table::remove(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, previous_reaction);
        } <b>else</b> {
            *table::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, previous_reaction) = previous_count - 1;
        };
        // Update user's reaction
        *table::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, user) = reaction;
    } <b>else</b> {
        // New reaction from this user
        table::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.user_reactions, user, reaction);
        // Increment <a href="../social_contracts/post.md#social_contracts_post">post</a> reaction count
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count = <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count + 1;
    };
    // Increment count <b>for</b> the reaction
    <b>if</b> (table::contains(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction)) {
        <b>let</b> count = *table::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction);
        *table::borrow_mut(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction) = count + 1;
    } <b>else</b> {
        table::add(&<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_counts, reaction, 1);
    };
    // Emit reaction event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_ReactionEvent">ReactionEvent</a> {
        object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        user,
        reaction,
        is_post: <b>true</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_tip_post"></a>

## Function `tip_post`

Tip a post creator with MySo tokens (with PoC revenue redirection support)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_post">tip_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, coins: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_post">tip_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    coins: &<b>mut</b> Coin&lt;MYS&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Basic validation
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    <b>let</b> tipper = tx_context::sender(ctx);
    <b>assert</b>!(tipper != <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_ESelfTipping">ESelfTipping</a>);
    // Verify this is not a repost or quote repost (those should <b>use</b> <a href="../social_contracts/post.md#social_contracts_post_tip_repost">tip_repost</a> instead)
    <b>assert</b>!(
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_REPOST">POST_TYPE_REPOST</a>) != <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type &&
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_QUOTE_REPOST">POST_TYPE_QUOTE_REPOST</a>) != <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type,
        <a href="../social_contracts/post.md#social_contracts_post_EInvalidPostType">EInvalidPostType</a>
    );
    // Check <b>if</b> tips are allowed on this <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.allow_tips, <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>);
    // Apply PoC redirection and transfer
    <b>let</b> actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_poc_redirection_and_transfer">apply_poc_redirection_and_transfer</a>(
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
        amount,
        coins,
        tipper,
        object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        <b>true</b>,
        ctx
    );
    // Record total tips received <b>for</b> this <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received = <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received + actual_received;
    // Emit tip event <b>for</b> amount actually received by <a href="../social_contracts/post.md#social_contracts_post">post</a> owner
    <b>if</b> (actual_received &gt; 0) {
        event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
            object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
            from: tipper,
            to: <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
            amount: actual_received,
            is_post: <b>true</b>,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_post_apply_poc_redirection_and_transfer"></a>

## Function `apply_poc_redirection_and_transfer`

Helper function to apply PoC revenue redirection and transfer coins
Returns the amount actually received by the intended recipient


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_apply_poc_redirection_and_transfer">apply_poc_redirection_and_transfer</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, intended_recipient: <b>address</b>, amount: u64, coins: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, tipper: <b>address</b>, object_id: <b>address</b>, is_post_event: bool, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_apply_poc_redirection_and_transfer">apply_poc_redirection_and_transfer</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    intended_recipient: <b>address</b>,
    amount: u64,
    coins: &<b>mut</b> Coin&lt;MYS&gt;,
    tipper: <b>address</b>,
    object_id: <b>address</b>,
    is_post_event: bool,
    ctx: &<b>mut</b> TxContext
): u64 {
    // Check <b>if</b> this <a href="../social_contracts/post.md#social_contracts_post">post</a> <b>has</b> revenue redirection <b>for</b> the intended recipient
    <b>if</b> (intended_recipient == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner &&
        option::is_some(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_to) &&
        option::is_some(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_percentage)) {
        <b>let</b> redirect_percentage = *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_percentage);
        <b>let</b> original_creator = *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_to);
        <b>if</b> (redirect_percentage &gt; 0) {
            // Calculate tip split
            <b>let</b> redirected_amount = (amount * redirect_percentage) / 100;
            <b>let</b> remaining_amount = amount - redirected_amount;
            // Take the tip amount out of the provided coin
            <b>let</b> <b>mut</b> tip_coins = coin::split(coins, amount, ctx);
            <b>if</b> (redirected_amount &gt; 0) {
                // Split tip <b>for</b> redirection
                <b>let</b> redirected_coins = coin::split(&<b>mut</b> tip_coins, redirected_amount, ctx);
                // Transfer redirected amount to original creator
                transfer::public_transfer(redirected_coins, original_creator);
                // Emit redirection event
                event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
                    object_id,
                    from: tipper,
                    to: original_creator,
                    amount: redirected_amount,
                    is_post: is_post_event,
                });
            };
            <b>if</b> (remaining_amount &gt; 0) {
                // Transfer remaining amount to intended recipient
                transfer::public_transfer(tip_coins, intended_recipient);
            } <b>else</b> {
                coin::destroy_zero(tip_coins);
            };
            <b>return</b> remaining_amount
        };
    };
    // No redirection - normal transfer
    <b>let</b> tip_coins = coin::split(coins, amount, ctx);
    transfer::public_transfer(tip_coins, intended_recipient);
    amount
}
</code></pre>



</details>

<a name="social_contracts_post_update_poc_result"></a>

## Function `update_poc_result`

Internal function to update PoC result (called only from proof_of_creativity module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_poc_result">update_poc_result</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, result_type: u8, badge_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;, redirect_to: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, redirect_percentage: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_poc_result">update_poc_result</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    result_type: u8, // 1 = badge issued, 2 = redirection applied
    badge_id: Option&lt;ID&gt;,
    redirect_to: Option&lt;<b>address</b>&gt;,
    redirect_percentage: Option&lt;u64&gt;
) {
    <b>if</b> (result_type == 1) {
        // PoC badge issued - content is original
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.poc_badge_id = badge_id;
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_to = option::none();
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_percentage = option::none();
    } <b>else</b> <b>if</b> (result_type == 2) {
        // Revenue redirection applied - content is derivative
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.poc_badge_id = option::none();
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_to = redirect_to;
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_percentage = redirect_percentage;
    };
}
</code></pre>



</details>

<a name="social_contracts_post_clear_poc_data"></a>

## Function `clear_poc_data`

Internal function to clear PoC data after dispute resolution


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_clear_poc_data">clear_poc_data</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_clear_poc_data">clear_poc_data</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>) {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.poc_badge_id = option::none();
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_to = option::none();
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_percentage = option::none();
}
</code></pre>



</details>

<a name="social_contracts_post_tip_repost"></a>

## Function `tip_repost`

Tip a repost with MySo tokens - applies 50/50 split between repost owner and original post owner


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_repost">tip_repost</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, original_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_repost">tip_repost</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, // The repost
    original_post: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>, // The original <a href="../social_contracts/post.md#social_contracts_post">post</a>
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    coin: &<b>mut</b> Coin&lt;MYS&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> tipper = tx_context::sender(ctx);
    // Check <b>if</b> amount is valid
    <b>assert</b>!(amount &gt; 0 && coin::value(coin) &gt;= amount, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    // Prevent self-tipping
    <b>assert</b>!(tipper != <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_ESelfTipping">ESelfTipping</a>);
    // Verify this is a repost or quote repost
    <b>assert</b>!(
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_REPOST">POST_TYPE_REPOST</a>) == <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type ||
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_QUOTE_REPOST">POST_TYPE_QUOTE_REPOST</a>) == <a href="../social_contracts/post.md#social_contracts_post">post</a>.post_type,
        <a href="../social_contracts/post.md#social_contracts_post_EInvalidPostType">EInvalidPostType</a>
    );
    // Verify the <a href="../social_contracts/post.md#social_contracts_post">post</a> <b>has</b> a parent_post_id
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.parent_post_id), <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>);
    // Verify the original_post ID matches the parent_post_id
    <b>let</b> parent_id = *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.parent_post_id);
    <b>assert</b>!(parent_id == object::uid_to_address(&original_post.id), <a href="../social_contracts/post.md#social_contracts_post_EInvalidParentReference">EInvalidParentReference</a>);
    // Check <b>if</b> tips are allowed on both posts
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.allow_tips, <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>);
    <b>assert</b>!(original_post.allow_tips, <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>);
    // Skip split <b>if</b> repost owner and original <a href="../social_contracts/post.md#social_contracts_post">post</a> owner are the same
    <b>if</b> (<a href="../social_contracts/post.md#social_contracts_post">post</a>.owner == original_post.owner) {
        // Standard flow - apply PoC redirection <b>for</b> unified owner
        <b>let</b> actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_poc_redirection_and_transfer">apply_poc_redirection_and_transfer</a>(
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
            amount,
            coin,
            tipper,
            object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
            <b>true</b>,
            ctx
        );
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received = <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received + actual_received;
        // Emit tip event <b>for</b> amount actually received
        <b>if</b> (actual_received &gt; 0) {
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
                object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
                from: tipper,
                to: <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
                amount: actual_received,
                is_post: <b>true</b>,
            });
        };
    } <b>else</b> {
        // Calculate split using config
        <b>let</b> repost_owner_amount = (amount * config.repost_tip_percentage) / 100;
        <b>let</b> original_owner_amount = amount - repost_owner_amount;
        // Apply PoC redirection <b>for</b> repost owner's share
        <b>let</b> repost_actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_poc_redirection_and_transfer">apply_poc_redirection_and_transfer</a>(
            <a href="../social_contracts/post.md#social_contracts_post">post</a>,
            <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
            repost_owner_amount,
            coin,
            tipper,
            object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
            <b>true</b>,
            ctx
        );
        // Apply PoC redirection <b>for</b> original <a href="../social_contracts/post.md#social_contracts_post">post</a> owner's share
        <b>let</b> original_actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_poc_redirection_and_transfer">apply_poc_redirection_and_transfer</a>(
            original_post,
            original_post.owner,
            original_owner_amount,
            coin,
            tipper,
            object::uid_to_address(&original_post.id),
            <b>true</b>,
            ctx
        );
        // Update tip counters
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received = <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received + repost_actual_received;
        original_post.tips_received = original_post.tips_received + original_actual_received;
        // Emit tip events <b>for</b> amounts actually received
        <b>if</b> (repost_actual_received &gt; 0) {
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
                object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
                from: tipper,
                to: <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
                amount: repost_actual_received,
                is_post: <b>true</b>,
            });
        };
        <b>if</b> (original_actual_received &gt; 0) {
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
                object_id: object::uid_to_address(&original_post.id),
                from: tipper,
                to: original_post.owner,
                amount: original_actual_received,
                is_post: <b>true</b>,
            });
        };
    }
}
</code></pre>



</details>

<a name="social_contracts_post_tip_comment"></a>

## Function `tip_comment`

Tip a comment with MySo tokens
Split is 80% to commenter, 20% to post owner (with PoC redirection on post owner's share)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_comment">tip_comment</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, coin: &<b>mut</b> <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_tip_comment">tip_comment</a>(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    coin: &<b>mut</b> Coin&lt;MYS&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> tipper = tx_context::sender(ctx);
    // Check <b>if</b> amount is valid
    <b>assert</b>!(amount &gt; 0 && coin::value(coin) &gt;= amount, <a href="../social_contracts/post.md#social_contracts_post_EInvalidTipAmount">EInvalidTipAmount</a>);
    // Prevent self-tipping
    <b>assert</b>!(tipper != comment.owner, <a href="../social_contracts/post.md#social_contracts_post_ESelfTipping">ESelfTipping</a>);
    // Check <b>if</b> tips are allowed on the <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.allow_tips, <a href="../social_contracts/post.md#social_contracts_post_ETipsNotAllowed">ETipsNotAllowed</a>);
    // Calculate split based on config percentage
    <b>let</b> commenter_amount = (amount * config.commenter_tip_percentage) / 100;
    <b>let</b> post_owner_amount = amount - commenter_amount;
    // Transfer commenter's share directly (no PoC redirection <b>for</b> commenters)
    <b>let</b> commenter_tip = coin::split(coin, commenter_amount, ctx);
    transfer::public_transfer(commenter_tip, comment.owner);
    // Apply PoC redirection <b>for</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> owner's share
    <b>let</b> post_owner_actual_received = <a href="../social_contracts/post.md#social_contracts_post_apply_poc_redirection_and_transfer">apply_poc_redirection_and_transfer</a>(
        <a href="../social_contracts/post.md#social_contracts_post">post</a>,
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
        post_owner_amount,
        coin,
        tipper,
        object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        <b>true</b>,
        ctx
    );
    // Update tip counters
    comment.tips_received = comment.tips_received + commenter_amount;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received = <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received + post_owner_actual_received;
    // Emit tip event <b>for</b> commenter
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
        object_id: object::uid_to_address(&comment.id),
        from: tipper,
        to: comment.owner,
        amount: commenter_amount,
        is_post: <b>false</b>,
    });
    // Emit tip event <b>for</b> <a href="../social_contracts/post.md#social_contracts_post">post</a> owner (amount actually received)
    <b>if</b> (post_owner_actual_received &gt; 0) {
        event::emit(<a href="../social_contracts/post.md#social_contracts_post_TipEvent">TipEvent</a> {
            object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
            from: tipper,
            to: <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
            amount: post_owner_actual_received,
            is_post: <b>true</b>,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_post_transfer_post_ownership"></a>

## Function `transfer_post_ownership`

Transfer post ownership to another user (by post owner)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_transfer_post_ownership">transfer_post_ownership</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, new_owner: <b>address</b>, registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_transfer_post_ownership">transfer_post_ownership</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    new_owner: <b>address</b>,
    registry: &UsernameRegistry,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_owner = tx_context::sender(ctx);
    // Verify current owner is authorized
    <b>assert</b>!(current_owner == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorizedTransfer">EUnauthorizedTransfer</a>);
    // Look up the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID <b>for</b> the new owner (<b>for</b> reference, not ownership)
    <b>let</b> <b>mut</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">social_contracts::profile::lookup_profile_by_owner</a>(registry, new_owner);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> new_profile_id = option::extract(&<b>mut</b> profile_id_option);
    // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> ownership
    <b>let</b> previous_owner = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner = new_owner;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.profile_id = new_profile_id;
    // Emit ownership transfer event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_OwnershipTransferEvent">OwnershipTransferEvent</a> {
        object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        previous_owner,
        new_owner,
        is_post: <b>true</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_admin_transfer_post_ownership"></a>

## Function `admin_transfer_post_ownership`

Admin function to transfer post ownership (requires PostAdminCap)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_admin_transfer_post_ownership">admin_transfer_post_ownership</a>(_: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">social_contracts::post::PostAdminCap</a>, <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, new_owner: <b>address</b>, registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, _ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_admin_transfer_post_ownership">admin_transfer_post_ownership</a>(
    _: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a>,
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    new_owner: <b>address</b>,
    registry: &UsernameRegistry,
    _ctx: &<b>mut</b> TxContext
) {
    // Admin capability verification is handled by type system
    // Look up the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID <b>for</b> the new owner (<b>for</b> reference, not ownership)
    <b>let</b> <b>mut</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">social_contracts::profile::lookup_profile_by_owner</a>(registry, new_owner);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> new_profile_id = option::extract(&<b>mut</b> profile_id_option);
    // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> ownership
    <b>let</b> previous_owner = <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner = new_owner;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.profile_id = new_profile_id;
    // Emit ownership transfer event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_OwnershipTransferEvent">OwnershipTransferEvent</a> {
        object_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        previous_owner,
        new_owner,
        is_post: <b>true</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_moderate_post"></a>

## Function `moderate_post`

Moderate a post (remove/restore from platform)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_moderate_post">moderate_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, remove: bool, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_moderate_post">moderate_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    remove: bool,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> developer or moderator
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">platform::is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> status
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.removed_from_platform = remove;
    // Emit moderation event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostModerationEvent">PostModerationEvent</a> {
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        platform_id: object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>)),
        removed: remove,
        moderated_by: caller,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_moderate_comment"></a>

## Function `moderate_comment`

Moderate a comment (remove/restore from platform)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_moderate_comment">moderate_comment</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, remove: bool, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_moderate_comment">moderate_comment</a>(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    remove: bool,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> developer or moderator
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">platform::is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Update comment status
    comment.removed_from_platform = remove;
    // Emit moderation event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostModerationEvent">PostModerationEvent</a> {
        post_id: object::uid_to_address(&comment.id),
        platform_id: object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>)),
        removed: remove,
        moderated_by: caller,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_update_post"></a>

## Function `update_post`

Update an existing post


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_post">update_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_post">update_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    content: String,
    <b>mut</b> media_urls: Option&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is the owner
    <b>let</b> owner = tx_context::sender(ctx);
    <b>assert</b>!(owner == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate content length using config
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate and update metadata <b>if</b> provided
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_string = option::borrow(& metadata_json);
        <b>assert</b>!(string::length(metadata_string) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        // Clear the current value and set the new one
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.metadata_json = option::some(*metadata_string);
    };
    // Convert and validate media URLs <b>if</b> provided
    <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> urls_bytes = option::extract(&<b>mut</b> media_urls);
        // Validate media URLs count
        <b>assert</b>!(vector::length(&urls_bytes) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        // Convert media URL bytes to Url
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(&urls_bytes);
        <b>while</b> (i &lt; len) {
            <b>let</b> url_bytes = *vector::borrow(&urls_bytes, i);
            vector::push_back(&<b>mut</b> urls, url::new_unsafe_from_bytes(url_bytes));
            i = i + 1;
        };
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.media = option::some(urls);
    };
    // Validate mentions <b>if</b> provided using config
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.mentions = mentions;
    };
    // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> content
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.content = content;
    // Emit <a href="../social_contracts/post.md#social_contracts_post">post</a> updated event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostUpdatedEvent">PostUpdatedEvent</a> {
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        owner: <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner,
        profile_id: <a href="../social_contracts/post.md#social_contracts_post">post</a>.profile_id,
        content: <a href="../social_contracts/post.md#social_contracts_post">post</a>.content,
        metadata_json: <a href="../social_contracts/post.md#social_contracts_post">post</a>.metadata_json,
        updated_at: tx_context::epoch(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_update_comment"></a>

## Function `update_comment`

Update an existing comment


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_comment">update_comment</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_comment">update_comment</a>(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    content: String,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is the owner
    <b>let</b> owner = tx_context::sender(ctx);
    <b>assert</b>!(owner == comment.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate content length using config
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate mentions <b>if</b> provided using config
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
        comment.mentions = mentions;
    };
    // Update comment content
    comment.content = content;
    // Emit comment updated event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_CommentUpdatedEvent">CommentUpdatedEvent</a> {
        comment_id: object::uid_to_address(&comment.id),
        post_id: comment.post_id,
        owner: comment.owner,
        profile_id: comment.profile_id,
        content: comment.content,
        updated_at: tx_context::epoch(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_report_post"></a>

## Function `report_post`

Report a post


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_report_post">report_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, reason_code: u8, description: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_report_post">report_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    reason_code: u8,
    description: String,
    ctx: &<b>mut</b> TxContext
) {
    // Validate reason code
    <b>assert</b>!(
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_SPAM">REPORT_REASON_SPAM</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OFFENSIVE">REPORT_REASON_OFFENSIVE</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_MISINFORMATION">REPORT_REASON_MISINFORMATION</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_ILLEGAL">REPORT_REASON_ILLEGAL</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_IMPERSONATION">REPORT_REASON_IMPERSONATION</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_HARASSMENT">REPORT_REASON_HARASSMENT</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OTHER">REPORT_REASON_OTHER</a>,
        <a href="../social_contracts/post.md#social_contracts_post_EReportReasonInvalid">EReportReasonInvalid</a>
    );
    // Validate description length using config
    <b>assert</b>!(string::length(&description) &lt;= config.max_description_length, <a href="../social_contracts/post.md#social_contracts_post_EReportDescriptionTooLong">EReportDescriptionTooLong</a>);
    // Get reporter's <b>address</b>
    <b>let</b> reporter = tx_context::sender(ctx);
    // Emit <a href="../social_contracts/post.md#social_contracts_post">post</a> reported event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostReportedEvent">PostReportedEvent</a> {
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        reporter,
        reason_code,
        description,
        reported_at: tx_context::epoch(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_report_comment"></a>

## Function `report_comment`

Report a comment


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_report_comment">report_comment</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, reason_code: u8, description: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_report_comment">report_comment</a>(
    comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    reason_code: u8,
    description: String,
    ctx: &<b>mut</b> TxContext
) {
    // Validate reason code
    <b>assert</b>!(
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_SPAM">REPORT_REASON_SPAM</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OFFENSIVE">REPORT_REASON_OFFENSIVE</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_MISINFORMATION">REPORT_REASON_MISINFORMATION</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_ILLEGAL">REPORT_REASON_ILLEGAL</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_IMPERSONATION">REPORT_REASON_IMPERSONATION</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_HARASSMENT">REPORT_REASON_HARASSMENT</a> ||
        reason_code == <a href="../social_contracts/post.md#social_contracts_post_REPORT_REASON_OTHER">REPORT_REASON_OTHER</a>,
        <a href="../social_contracts/post.md#social_contracts_post_EReportReasonInvalid">EReportReasonInvalid</a>
    );
    // Validate description length using config
    <b>assert</b>!(string::length(&description) &lt;= config.max_description_length, <a href="../social_contracts/post.md#social_contracts_post_EReportDescriptionTooLong">EReportDescriptionTooLong</a>);
    // Get reporter's <b>address</b>
    <b>let</b> reporter = tx_context::sender(ctx);
    // Emit comment reported event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_CommentReportedEvent">CommentReportedEvent</a> {
        comment_id: object::uid_to_address(&comment.id),
        reporter,
        reason_code,
        description,
        reported_at: tx_context::epoch(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_react_to_comment"></a>

## Function `react_to_comment`

React to a comment with a specific reaction (emoji or text)
If the user already has the exact same reaction, it will be removed (toggle behavior)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_react_to_comment">react_to_comment</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, reaction: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_react_to_comment">react_to_comment</a>(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    reaction: String,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> user = tx_context::sender(ctx);
    // Validate reaction length using config
    <b>assert</b>!(string::length(&reaction) &lt;= config.max_reaction_length, <a href="../social_contracts/post.md#social_contracts_post_EReactionContentTooLong">EReactionContentTooLong</a>);
    // Check <b>if</b> user already reacted to the comment
    <b>if</b> (table::contains(&comment.user_reactions, user)) {
        // Get the previous reaction
        <b>let</b> previous_reaction = *table::borrow(&comment.user_reactions, user);
        // If the reaction is the same, remove it (toggle behavior)
        <b>if</b> (reaction == previous_reaction) {
            // Remove user's reaction
            table::remove(&<b>mut</b> comment.user_reactions, user);
            // Decrease count <b>for</b> this reaction type
            <b>let</b> count = *table::borrow(&comment.reaction_counts, reaction);
            <b>if</b> (count &lt;= 1) {
                table::remove(&<b>mut</b> comment.reaction_counts, reaction);
            } <b>else</b> {
                *table::borrow_mut(&<b>mut</b> comment.reaction_counts, reaction) = count - 1;
            };
            // Decrement comment reaction count
            comment.reaction_count = comment.reaction_count - 1;
            // Emit remove reaction event
            event::emit(<a href="../social_contracts/post.md#social_contracts_post_RemoveReactionEvent">RemoveReactionEvent</a> {
                object_id: object::uid_to_address(&comment.id),
                user,
                reaction,
                is_post: <b>false</b>,
            });
            <b>return</b>
        };
        // Different reaction, update existing one
        // Decrease count <b>for</b> previous reaction
        <b>let</b> previous_count = *table::borrow(&comment.reaction_counts, previous_reaction);
        <b>if</b> (previous_count &lt;= 1) {
            table::remove(&<b>mut</b> comment.reaction_counts, previous_reaction);
        } <b>else</b> {
            *table::borrow_mut(&<b>mut</b> comment.reaction_counts, previous_reaction) = previous_count - 1;
        };
        // Update user's reaction
        *table::borrow_mut(&<b>mut</b> comment.user_reactions, user) = reaction;
    } <b>else</b> {
        // New reaction from this user
        table::add(&<b>mut</b> comment.user_reactions, user, reaction);
        // Increment comment reaction count
        comment.reaction_count = comment.reaction_count + 1;
    };
    // Increment count <b>for</b> the reaction
    <b>if</b> (table::contains(&comment.reaction_counts, reaction)) {
        <b>let</b> count = *table::borrow(&comment.reaction_counts, reaction);
        *table::borrow_mut(&<b>mut</b> comment.reaction_counts, reaction) = count + 1;
    } <b>else</b> {
        table::add(&<b>mut</b> comment.reaction_counts, reaction, 1);
    };
    // Emit reaction event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_ReactionEvent">ReactionEvent</a> {
        object_id: object::uid_to_address(&comment.id),
        user,
        reaction,
        is_post: <b>false</b>,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_get_post_content"></a>

## Function `get_post_content`

Get post content


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_content">get_post_content</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_content">get_post_content</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): String {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.content
}
</code></pre>



</details>

<a name="social_contracts_post_get_post_owner"></a>

## Function `get_post_owner`

Get post owner


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">get_post_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_owner">get_post_owner</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): <b>address</b> {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner
}
</code></pre>



</details>

<a name="social_contracts_post_get_post_id"></a>

## Function `get_post_id`

Get post ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_id">get_post_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_id">get_post_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): &UID {
    &<a href="../social_contracts/post.md#social_contracts_post">post</a>.id
}
</code></pre>



</details>

<a name="social_contracts_post_get_post_comment_count"></a>

## Function `get_post_comment_count`

Get post comment count


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_comment_count">get_post_comment_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_post_comment_count">get_post_comment_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u64 {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.comment_count
}
</code></pre>



</details>

<a name="social_contracts_post_get_comment_owner"></a>

## Function `get_comment_owner`

Get comment owner


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_comment_owner">get_comment_owner</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_comment_owner">get_comment_owner</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): <b>address</b> {
    comment.owner
}
</code></pre>



</details>

<a name="social_contracts_post_get_comment_post_id"></a>

## Function `get_comment_post_id`

Get comment post ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_comment_post_id">get_comment_post_id</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_comment_post_id">get_comment_post_id</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): <b>address</b> {
    comment.post_id
}
</code></pre>



</details>

<a name="social_contracts_post_get_id_address"></a>

## Function `get_id_address`

Get the ID address of a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_id_address">get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_id_address">get_id_address</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): <b>address</b> {
    object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id)
}
</code></pre>



</details>

<a name="social_contracts_post_get_reaction_count"></a>

## Function `get_reaction_count`

Get the reaction count of a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_reaction_count">get_reaction_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_reaction_count">get_reaction_count</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u64 {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.reaction_count
}
</code></pre>



</details>

<a name="social_contracts_post_get_tips_received"></a>

## Function `get_tips_received`

Get the tips received for a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_tips_received">get_tips_received</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_tips_received">get_tips_received</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u64 {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.tips_received
}
</code></pre>



</details>

<a name="social_contracts_post_get_poc_badge_id"></a>

## Function `get_poc_badge_id`

Get the PoC badge ID for a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_poc_badge_id">get_poc_badge_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/object.md#mys_object_ID">mys::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_poc_badge_id">get_poc_badge_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): &Option&lt;ID&gt; {
    &<a href="../social_contracts/post.md#social_contracts_post">post</a>.poc_badge_id
}
</code></pre>



</details>

<a name="social_contracts_post_get_revenue_redirect_to"></a>

## Function `get_revenue_redirect_to`

Get the revenue redirect address for a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_to">get_revenue_redirect_to</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_to">get_revenue_redirect_to</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): &Option&lt;<b>address</b>&gt; {
    &<a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_to
}
</code></pre>



</details>

<a name="social_contracts_post_get_revenue_redirect_percentage"></a>

## Function `get_revenue_redirect_percentage`

Get the revenue redirect percentage for a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_percentage">get_revenue_redirect_percentage</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_revenue_redirect_percentage">get_revenue_redirect_percentage</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): &Option&lt;u64&gt; {
    &<a href="../social_contracts/post.md#social_contracts_post">post</a>.revenue_redirect_percentage
}
</code></pre>



</details>

<a name="social_contracts_post_get_total_bet_amount"></a>

## Function `get_total_bet_amount`

Get total bet amount for a prediction


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_total_bet_amount">get_total_bet_amount</a>(prediction_data: &<a href="../social_contracts/post.md#social_contracts_post_PredictionData">social_contracts::post::PredictionData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_total_bet_amount">get_total_bet_amount</a>(prediction_data: &<a href="../social_contracts/post.md#social_contracts_post_PredictionData">PredictionData</a>): u64 {
    prediction_data.total_bet_amount
}
</code></pre>



</details>

<a name="social_contracts_post_get_bets_count"></a>

## Function `get_bets_count`

Get number of bets for a prediction


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_bets_count">get_bets_count</a>(prediction_data: &<a href="../social_contracts/post.md#social_contracts_post_PredictionData">social_contracts::post::PredictionData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_bets_count">get_bets_count</a>(prediction_data: &<a href="../social_contracts/post.md#social_contracts_post_PredictionData">PredictionData</a>): u64 {
    vector::length(&prediction_data.bets)
}
</code></pre>



</details>

<a name="social_contracts_post_get_bet_user"></a>

## Function `get_bet_user`

Get bet user at index


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_bet_user">get_bet_user</a>(prediction_data: &<a href="../social_contracts/post.md#social_contracts_post_PredictionData">social_contracts::post::PredictionData</a>, index: u64): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_bet_user">get_bet_user</a>(prediction_data: &<a href="../social_contracts/post.md#social_contracts_post_PredictionData">PredictionData</a>, index: u64): <b>address</b> {
    <b>let</b> bet = vector::borrow(&prediction_data.bets, index);
    bet.user
}
</code></pre>



</details>

<a name="social_contracts_post_get_bet_option_id"></a>

## Function `get_bet_option_id`

Get bet option id at index


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_bet_option_id">get_bet_option_id</a>(prediction_data: &<a href="../social_contracts/post.md#social_contracts_post_PredictionData">social_contracts::post::PredictionData</a>, index: u64): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_bet_option_id">get_bet_option_id</a>(prediction_data: &<a href="../social_contracts/post.md#social_contracts_post_PredictionData">PredictionData</a>, index: u64): u8 {
    <b>let</b> bet = vector::borrow(&prediction_data.bets, index);
    bet.option_id
}
</code></pre>



</details>

<a name="social_contracts_post_get_bet_amount"></a>

## Function `get_bet_amount`

Get bet amount at index


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_bet_amount">get_bet_amount</a>(prediction_data: &<a href="../social_contracts/post.md#social_contracts_post_PredictionData">social_contracts::post::PredictionData</a>, index: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_bet_amount">get_bet_amount</a>(prediction_data: &<a href="../social_contracts/post.md#social_contracts_post_PredictionData">PredictionData</a>, index: u64): u64 {
    <b>let</b> bet = vector::borrow(&prediction_data.bets, index);
    bet.amount
}
</code></pre>



</details>

<a name="social_contracts_post_version"></a>

## Function `version`

Get the version of a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_version">version</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_version">version</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): u64 {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_borrow_version_mut"></a>

## Function `borrow_version_mut`

Get a mutable reference to the post version (for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): &<b>mut</b> u64 {
    &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_comment_version"></a>

## Function `comment_version`

Get the version of a comment


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_version">comment_version</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_comment_version">comment_version</a>(comment: &<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): u64 {
    comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_borrow_comment_version_mut"></a>

## Function `borrow_comment_version_mut`

Get a mutable reference to the comment version (for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_comment_version_mut">borrow_comment_version_mut</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_comment_version_mut">borrow_comment_version_mut</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>): &<b>mut</b> u64 {
    &<b>mut</b> comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_repost_version"></a>

## Function `repost_version`

Get the version of a repost


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_repost_version">repost_version</a>(repost: &<a href="../social_contracts/post.md#social_contracts_post_Repost">social_contracts::post::Repost</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_repost_version">repost_version</a>(repost: &<a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a>): u64 {
    repost.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_borrow_repost_version_mut"></a>

## Function `borrow_repost_version_mut`

Get a mutable reference to the repost version (for upgrade module)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_repost_version_mut">borrow_repost_version_mut</a>(repost: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">social_contracts::post::Repost</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_borrow_repost_version_mut">borrow_repost_version_mut</a>(repost: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a>): &<b>mut</b> u64 {
    &<b>mut</b> repost.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_post_migrate_post"></a>

## Function `migrate_post`

Migration function for Post


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_post">migrate_post</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_post">migrate_post</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/post.md#social_contracts_post_version">version</a> &gt; current <a href="../social_contracts/post.md#social_contracts_post_version">version</a>)
    <b>assert</b>!(<a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> &lt; current_version, <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/post.md#social_contracts_post_version">version</a> and update to new <a href="../social_contracts/post.md#social_contracts_post_version">version</a>
    <b>let</b> old_version = <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>;
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> post_id = object::id(<a href="../social_contracts/post.md#social_contracts_post">post</a>);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        post_id,
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_post_migrate_comment"></a>

## Function `migrate_comment`

Migration function for Comment


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_comment">migrate_comment</a>(comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">social_contracts::post::Comment</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_comment">migrate_comment</a>(
    comment: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/post.md#social_contracts_post_version">version</a> &gt; current <a href="../social_contracts/post.md#social_contracts_post_version">version</a>)
    <b>assert</b>!(comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> &lt; current_version, <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/post.md#social_contracts_post_version">version</a> and update to new <a href="../social_contracts/post.md#social_contracts_post_version">version</a>
    <b>let</b> old_version = comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>;
    comment.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> comment_id = object::id(comment);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        comment_id,
        string::utf8(b"<a href="../social_contracts/post.md#social_contracts_post_Comment">Comment</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_post_migrate_repost"></a>

## Function `migrate_repost`

Migration function for Repost


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_repost">migrate_repost</a>(repost: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">social_contracts::post::Repost</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_migrate_repost">migrate_repost</a>(
    repost: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/post.md#social_contracts_post_version">version</a> &gt; current <a href="../social_contracts/post.md#social_contracts_post_version">version</a>)
    <b>assert</b>!(repost.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> &lt; current_version, <a href="../social_contracts/post.md#social_contracts_post_EWrongVersion">EWrongVersion</a>);
    // Remember old <a href="../social_contracts/post.md#social_contracts_post_version">version</a> and update to new <a href="../social_contracts/post.md#social_contracts_post_version">version</a>
    <b>let</b> old_version = repost.<a href="../social_contracts/post.md#social_contracts_post_version">version</a>;
    repost.<a href="../social_contracts/post.md#social_contracts_post_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> repost_id = object::id(repost);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        repost_id,
        string::utf8(b"<a href="../social_contracts/post.md#social_contracts_post_Repost">Repost</a>"),
        old_version,
        tx_context::sender(ctx)
    );
    // Any migration logic can be added here <b>for</b> future upgrades
}
</code></pre>



</details>

<a name="social_contracts_post_update_post_parameters"></a>

## Function `update_post_parameters`

Update post parameters (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_post_parameters">update_post_parameters</a>(_admin_cap: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">social_contracts::post::PostAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, max_content_length: u64, max_media_urls: u64, max_mentions: u64, max_metadata_size: u64, max_description_length: u64, max_reaction_length: u64, commenter_tip_percentage: u64, repost_tip_percentage: u64, max_prediction_options: u64, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_update_post_parameters">update_post_parameters</a>(
    _admin_cap: &<a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    max_content_length: u64,
    max_media_urls: u64,
    max_mentions: u64,
    max_metadata_size: u64,
    max_description_length: u64,
    max_reaction_length: u64,
    commenter_tip_percentage: u64,
    repost_tip_percentage: u64,
    max_prediction_options: u64,
    ctx: &<b>mut</b> TxContext
) {
    // Validation
    <b>assert</b>!(commenter_tip_percentage &lt;= 100, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(repost_tip_percentage &lt;= 100, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_content_length &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_media_urls &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_mentions &gt; 0, <a href="../social_contracts/post.md#social_contracts_post_EInvalidConfig">EInvalidConfig</a>);
    // Update config
    config.max_content_length = max_content_length;
    config.max_media_urls = max_media_urls;
    config.max_mentions = max_mentions;
    config.max_metadata_size = max_metadata_size;
    config.max_description_length = max_description_length;
    config.max_reaction_length = max_reaction_length;
    config.commenter_tip_percentage = commenter_tip_percentage;
    config.repost_tip_percentage = repost_tip_percentage;
    config.max_prediction_options = max_prediction_options;
    // Emit update event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostParametersUpdatedEvent">PostParametersUpdatedEvent</a> {
        updated_by: tx_context::sender(ctx),
        timestamp: tx_context::epoch_timestamp_ms(ctx),
        max_content_length,
        max_media_urls,
        max_mentions,
        max_metadata_size,
        max_description_length,
        max_reaction_length,
        commenter_tip_percentage,
        repost_tip_percentage,
        max_prediction_options,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_create_promoted_post"></a>

## Function `create_promoted_post`

Create a promoted post with MYS tokens for viewer payments


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_promoted_post">create_promoted_post</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, _block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">social_contracts::post::PostConfig</a>, content: <a href="../std/string.md#std_string_String">std::string::String</a>, media_urls: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;, mentions: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<b>address</b>&gt;&gt;, metadata_json: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, my_ip_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, payment_per_view: u64, promotion_budget: <a href="../mys/coin.md#mys_coin_Coin">mys::coin::Coin</a>&lt;<a href="../mys/mys.md#mys_mys_MYS">mys::mys::MYS</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_promoted_post">create_promoted_post</a>(
    registry: &UsernameRegistry,
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">platform::PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    _block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    config: &<a href="../social_contracts/post.md#social_contracts_post_PostConfig">PostConfig</a>,
    content: String,
    <b>mut</b> media_urls: Option&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;,
    mentions: Option&lt;vector&lt;<b>address</b>&gt;&gt;,
    metadata_json: Option&lt;String&gt;,
    my_ip_id: Option&lt;<b>address</b>&gt;,
    payment_per_view: u64,
    promotion_budget: Coin&lt;MYS&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> owner = tx_context::sender(ctx);
    // Validate promotion parameters
    <b>assert</b>!(payment_per_view &gt;= <a href="../social_contracts/post.md#social_contracts_post_MIN_PROMOTION_AMOUNT">MIN_PROMOTION_AMOUNT</a>, <a href="../social_contracts/post.md#social_contracts_post_EPromotionAmountTooLow">EPromotionAmountTooLow</a>);
    <b>assert</b>!(payment_per_view &lt;= <a href="../social_contracts/post.md#social_contracts_post_MAX_PROMOTION_AMOUNT">MAX_PROMOTION_AMOUNT</a>, <a href="../social_contracts/post.md#social_contracts_post_EPromotionAmountTooHigh">EPromotionAmountTooHigh</a>);
    <b>assert</b>!(coin::value(&promotion_budget) &gt;= payment_per_view, <a href="../social_contracts/post.md#social_contracts_post_EInsufficientPromotionFunds">EInsufficientPromotionFunds</a>);
    // Look up the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a> ID <b>for</b> the sender
    <b>let</b> <b>mut</b> profile_id_option = <a href="../social_contracts/profile.md#social_contracts_profile_lookup_profile_by_owner">social_contracts::profile::lookup_profile_by_owner</a>(registry, owner);
    <b>assert</b>!(option::is_some(&profile_id_option), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    <b>let</b> profile_id = option::extract(&<b>mut</b> profile_id_option);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">platform::is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate block list - simplified <b>for</b> this implementation
    // <b>assert</b>!(!block_list::is_profile_blocked(block_list_registry, profile_id), <a href="../social_contracts/post.md#social_contracts_post_EUserBlockedByPlatform">EUserBlockedByPlatform</a>);
    // Validate content length using config
    <b>assert</b>!(string::length(&content) &lt;= config.max_content_length, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    // Validate and convert media URLs <b>if</b> provided
    <b>let</b> media_option = <b>if</b> (option::is_some(&media_urls)) {
        <b>let</b> urls_bytes = option::extract(&<b>mut</b> media_urls);
        <b>assert</b>!(vector::length(&urls_bytes) &lt;= config.max_media_urls, <a href="../social_contracts/post.md#social_contracts_post_ETooManyMediaUrls">ETooManyMediaUrls</a>);
        <b>let</b> <b>mut</b> urls = vector::empty&lt;Url&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>while</b> (i &lt; vector::length(&urls_bytes)) {
            <b>let</b> url_bytes = vector::borrow(&urls_bytes, i);
            <b>let</b> url = url::new_unsafe_from_bytes(*url_bytes);
            vector::push_back(&<b>mut</b> urls, url);
            i = i + 1;
        };
        option::some(urls)
    } <b>else</b> {
        option::none()
    };
    // Validate mentions <b>if</b> provided using config
    <b>if</b> (option::is_some(&mentions)) {
        <b>let</b> mentions_ref = option::borrow(&mentions);
        <b>assert</b>!(vector::length(mentions_ref) &lt;= config.max_mentions, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Validate metadata <b>if</b> provided using config
    <b>if</b> (option::is_some(&metadata_json)) {
        <b>let</b> metadata_string = option::borrow(&metadata_json);
        <b>assert</b>!(string::length(metadata_string) &lt;= config.max_metadata_size, <a href="../social_contracts/post.md#social_contracts_post_EContentTooLarge">EContentTooLarge</a>);
    };
    // Create promotion data (starts <b>as</b> inactive until <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> activates it)
    <b>let</b> <b>mut</b> promotion_data = <a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a> {
        id: object::new(ctx),
        post_id: @0x0, // Will be set after <a href="../social_contracts/post.md#social_contracts_post">post</a> creation
        payment_per_view,
        promotion_budget: coin::into_balance(promotion_budget),
        paid_viewers: table::new(ctx),
        views: vector::empty(),
        active: <b>false</b>, // Starts inactive until <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> approves
        created_at: tx_context::epoch_timestamp_ms(ctx),
    };
    <b>let</b> promotion_id = object::uid_to_address(&promotion_data.id);
    // Create and share the <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>let</b> post_id = <a href="../social_contracts/post.md#social_contracts_post_create_post_internal">create_post_internal</a>(
        owner,
        profile_id,
        content,
        media_option,
        mentions,
        metadata_json,
        string::utf8(<a href="../social_contracts/post.md#social_contracts_post_POST_TYPE_STANDARD">POST_TYPE_STANDARD</a>),
        option::none(),
        <b>true</b>, // allow_comments
        <b>true</b>, // allow_reactions
        <b>true</b>, // allow_reposts
        <b>true</b>, // allow_quotes
        <b>true</b>, // allow_tips
        option::none(), // poc_badge_id
        option::none(), // revenue_redirect_to
        option::none(), // revenue_redirect_percentage
        my_ip_id,
        option::some(promotion_id),
        ctx
    );
    // Update promotion data with <a href="../social_contracts/post.md#social_contracts_post">post</a> ID
    promotion_data.post_id = post_id;
    // Get budget value before moving the promotion_data
    <b>let</b> total_budget = balance::value(&promotion_data.promotion_budget);
    // Share promotion data
    transfer::share_object(promotion_data);
    // Emit promoted <a href="../social_contracts/post.md#social_contracts_post">post</a> creation event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PromotedPostCreatedEvent">PromotedPostCreatedEvent</a> {
        post_id,
        owner,
        profile_id,
        payment_per_view,
        total_budget,
        created_at: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_confirm_promoted_post_view"></a>

## Function `confirm_promoted_post_view`

Confirm a user has viewed a promoted post and pay them (platform only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_confirm_promoted_post_view">confirm_promoted_post_view</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, promotion_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">social_contracts::post::PromotionData</a>, platform_obj: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, viewer_address: <b>address</b>, view_duration: u64, clock: &<a href="../mys/clock.md#mys_clock_Clock">mys::clock::Clock</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_confirm_promoted_post_view">confirm_promoted_post_view</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    promotion_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a>,
    platform_obj: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    viewer_address: <b>address</b>,
    view_duration: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Verify this is a <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> call (<a href="../social_contracts/platform.md#social_contracts_platform">platform</a> developer or moderator)
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">platform::is_developer_or_moderator</a>(platform_obj, caller), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Verify the <a href="../social_contracts/post.md#social_contracts_post">post</a> is promoted
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id), <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>);
    <b>let</b> post_promotion_id = *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id);
    <b>assert</b>!(post_promotion_id == object::uid_to_address(&promotion_data.id), <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>);
    // Verify promotion is active
    <b>assert</b>!(promotion_data.active, <a href="../social_contracts/post.md#social_contracts_post_EPromotionInactive">EPromotionInactive</a>);
    // Verify view duration meets minimum requirement
    <b>assert</b>!(view_duration &gt;= <a href="../social_contracts/post.md#social_contracts_post_MIN_VIEW_DURATION">MIN_VIEW_DURATION</a>, <a href="../social_contracts/post.md#social_contracts_post_EInvalidViewDuration">EInvalidViewDuration</a>);
    // Verify user hasn't already been paid <b>for</b> viewing this <a href="../social_contracts/post.md#social_contracts_post">post</a>
    <b>assert</b>!(!table::contains(&promotion_data.paid_viewers, viewer_address), <a href="../social_contracts/post.md#social_contracts_post_EUserAlreadyViewed">EUserAlreadyViewed</a>);
    // Verify sufficient budget remains
    <b>assert</b>!(balance::value(&promotion_data.promotion_budget) &gt;= promotion_data.payment_per_view, <a href="../social_contracts/post.md#social_contracts_post_EInsufficientPromotionFunds">EInsufficientPromotionFunds</a>);
    // Record the view
    <b>let</b> view_record = <a href="../social_contracts/post.md#social_contracts_post_PromotionView">PromotionView</a> {
        viewer: viewer_address,
        view_duration,
        view_timestamp: clock::timestamp_ms(clock),
        platform_id: caller, // Use caller <b>as</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> identifier
    };
    vector::push_back(&<b>mut</b> promotion_data.views, view_record);
    // Mark user <b>as</b> paid
    table::add(&<b>mut</b> promotion_data.paid_viewers, viewer_address, <b>true</b>);
    // Split payment from promotion budget and transfer to viewer
    <b>let</b> payment_balance = balance::split(&<b>mut</b> promotion_data.promotion_budget, promotion_data.payment_per_view);
    <b>let</b> payment_coin = coin::from_balance(payment_balance, ctx);
    transfer::public_transfer(payment_coin, viewer_address);
    // If budget is exhausted, deactivate promotion
    <b>if</b> (balance::value(&promotion_data.promotion_budget) &lt; promotion_data.payment_per_view) {
        promotion_data.active = <b>false</b>;
    };
    // Emit view confirmation event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PromotedPostViewConfirmedEvent">PromotedPostViewConfirmedEvent</a> {
        post_id: post_promotion_id,
        viewer: viewer_address,
        payment_amount: promotion_data.payment_per_view,
        view_duration,
        platform_id: caller, // Use caller <b>as</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> identifier
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_toggle_promotion_status"></a>

## Function `toggle_promotion_status`

Toggle promotion status (platform can activate, both platform and owner can deactivate)
Use with activate: false to deactivate promotions


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_toggle_promotion_status">toggle_promotion_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, promotion_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">social_contracts::post::PromotionData</a>, platform_obj: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, activate: bool, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_toggle_promotion_status">toggle_promotion_status</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    promotion_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a>,
    platform_obj: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    activate: bool,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    // Verify the <a href="../social_contracts/post.md#social_contracts_post">post</a> is promoted
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id), <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>);
    <b>let</b> post_promotion_id = *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id);
    <b>assert</b>!(post_promotion_id == object::uid_to_address(&promotion_data.id), <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>);
    <b>if</b> (activate) {
        // Only <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> can activate promotions
        <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">platform::is_developer_or_moderator</a>(platform_obj, caller), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    } <b>else</b> {
        // Both <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> and <a href="../social_contracts/post.md#social_contracts_post">post</a> owner can deactivate
        <b>let</b> is_platform = <a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">platform::is_developer_or_moderator</a>(platform_obj, caller);
        <b>let</b> is_owner = caller == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner;
        <b>assert</b>!(is_platform || is_owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    };
    promotion_data.active = activate;
    // Emit status change event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PromotionStatusToggledEvent">PromotionStatusToggledEvent</a> {
        post_id: post_promotion_id,
        toggled_by: caller,
        new_status: activate,
        timestamp: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_withdraw_promotion_funds"></a>

## Function `withdraw_promotion_funds`

Withdraw all MYS tokens from promotion (owner only, deactivates promotion)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_withdraw_promotion_funds">withdraw_promotion_funds</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, promotion_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">social_contracts::post::PromotionData</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_withdraw_promotion_funds">withdraw_promotion_funds</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    promotion_data: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    // Verify caller is <a href="../social_contracts/post.md#social_contracts_post">post</a> owner
    <b>assert</b>!(caller == <a href="../social_contracts/post.md#social_contracts_post">post</a>.owner, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Verify the <a href="../social_contracts/post.md#social_contracts_post">post</a> is promoted
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id), <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>);
    <b>let</b> post_promotion_id = *option::borrow(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id);
    <b>assert</b>!(post_promotion_id == object::uid_to_address(&promotion_data.id), <a href="../social_contracts/post.md#social_contracts_post_ENotPromotedPost">ENotPromotedPost</a>);
    // Get remaining funds
    <b>let</b> remaining_amount = balance::value(&promotion_data.promotion_budget);
    // Extract all remaining balance and transfer to owner
    <b>let</b> withdrawn_balance = balance::withdraw_all(&<b>mut</b> promotion_data.promotion_budget);
    <b>let</b> withdrawn_coins = coin::from_balance(withdrawn_balance, ctx);
    transfer::public_transfer(withdrawn_coins, caller);
    // Deactivate promotion
    promotion_data.active = <b>false</b>;
    // Emit withdrawal event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PromotionFundsWithdrawnEvent">PromotionFundsWithdrawnEvent</a> {
        post_id: post_promotion_id,
        owner: caller,
        withdrawn_amount: remaining_amount,
        timestamp: tx_context::epoch_timestamp_ms(ctx),
    });
}
</code></pre>



</details>

<a name="social_contracts_post_get_promotion_stats"></a>

## Function `get_promotion_stats`

Get promotion statistics for a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_promotion_stats">get_promotion_stats</a>(promotion_data: &<a href="../social_contracts/post.md#social_contracts_post_PromotionData">social_contracts::post::PromotionData</a>): (u64, u64, bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_promotion_stats">get_promotion_stats</a>(promotion_data: &<a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a>): (u64, u64, bool, u64) {
    (
        promotion_data.payment_per_view,
        balance::value(&promotion_data.promotion_budget),
        promotion_data.active,
        vector::length(&promotion_data.views)
    )
}
</code></pre>



</details>

<a name="social_contracts_post_has_user_viewed_promoted_post"></a>

## Function `has_user_viewed_promoted_post`

Check if a user has already been paid for viewing a promoted post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_has_user_viewed_promoted_post">has_user_viewed_promoted_post</a>(promotion_data: &<a href="../social_contracts/post.md#social_contracts_post_PromotionData">social_contracts::post::PromotionData</a>, user: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_has_user_viewed_promoted_post">has_user_viewed_promoted_post</a>(promotion_data: &<a href="../social_contracts/post.md#social_contracts_post_PromotionData">PromotionData</a>, user: <b>address</b>): bool {
    table::contains(&promotion_data.paid_viewers, user)
}
</code></pre>



</details>

<a name="social_contracts_post_get_promotion_id"></a>

## Function `get_promotion_id`

Get the promotion ID from a post


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_promotion_id">get_promotion_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_get_promotion_id">get_promotion_id</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): Option&lt;<b>address</b>&gt; {
    <a href="../social_contracts/post.md#social_contracts_post">post</a>.promotion_id
}
</code></pre>



</details>

<a name="social_contracts_post_set_moderation_status"></a>

## Function `set_moderation_status`

Set moderation status for a post (platform devs/mods only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_moderation_status">set_moderation_status</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, status: u8, reason: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_set_moderation_status">set_moderation_status</a>(
    <a href="../social_contracts/post.md#social_contracts_post">post</a>: &<b>mut</b> <a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">platform::Platform</a>,
    status: u8, // <a href="../social_contracts/post.md#social_contracts_post_MODERATION_APPROVED">MODERATION_APPROVED</a> or <a href="../social_contracts/post.md#social_contracts_post_MODERATION_FLAGGED">MODERATION_FLAGGED</a>
    reason: Option&lt;String&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> developer or moderator
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">platform::is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, caller), <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Validate status
    <b>assert</b>!(status == <a href="../social_contracts/post.md#social_contracts_post_MODERATION_APPROVED">MODERATION_APPROVED</a> || status == <a href="../social_contracts/post.md#social_contracts_post_MODERATION_FLAGGED">MODERATION_FLAGGED</a>, <a href="../social_contracts/post.md#social_contracts_post_EUnauthorized">EUnauthorized</a>);
    // Update <a href="../social_contracts/post.md#social_contracts_post">post</a> status based on moderation decision
    <b>if</b> (status == <a href="../social_contracts/post.md#social_contracts_post_MODERATION_FLAGGED">MODERATION_FLAGGED</a>) {
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.removed_from_platform = <b>true</b>;
    } <b>else</b> {
        <a href="../social_contracts/post.md#social_contracts_post">post</a>.removed_from_platform = <b>false</b>;
    };
    // Create or update moderation record
    <b>let</b> moderation_record = <a href="../social_contracts/post.md#social_contracts_post_ModerationRecord">ModerationRecord</a> {
        id: object::new(ctx),
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        platform_id: object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>)),
        moderation_state: status,
        moderator: option::some(caller),
        moderation_timestamp: option::some(tx_context::epoch_timestamp_ms(ctx)),
        reason,
    };
    transfer::share_object(moderation_record);
    // Emit moderation event
    event::emit(<a href="../social_contracts/post.md#social_contracts_post_PostModerationEvent">PostModerationEvent</a> {
        post_id: object::uid_to_address(&<a href="../social_contracts/post.md#social_contracts_post">post</a>.id),
        platform_id: object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>)),
        removed: (status == <a href="../social_contracts/post.md#social_contracts_post_MODERATION_FLAGGED">MODERATION_FLAGGED</a>),
        moderated_by: caller,
    });
}
</code></pre>



</details>

<a name="social_contracts_post_is_content_approved"></a>

## Function `is_content_approved`

Check if content is approved (not flagged)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_is_content_approved">is_content_approved</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">social_contracts::post::Post</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_is_content_approved">is_content_approved</a>(<a href="../social_contracts/post.md#social_contracts_post">post</a>: &<a href="../social_contracts/post.md#social_contracts_post_Post">Post</a>): bool {
    !<a href="../social_contracts/post.md#social_contracts_post">post</a>.removed_from_platform
}
</code></pre>



</details>

<a name="social_contracts_post_create_post_admin_cap"></a>

## Function `create_post_admin_cap`

Create a PostAdminCap for bootstrap (package visibility only)
This function is only callable by other modules in the same package


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_admin_cap">create_post_admin_cap</a>(ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">social_contracts::post::PostAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/post.md#social_contracts_post_create_post_admin_cap">create_post_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a> {
    <a href="../social_contracts/post.md#social_contracts_post_PostAdminCap">PostAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>
