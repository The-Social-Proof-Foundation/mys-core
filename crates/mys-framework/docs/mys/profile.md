---
title: Module `mys::profile`
---

Profile module for the MySocial network
Handles user identity, profile creation and management


-  [Struct `Profile`](#mys_profile_Profile)
-  [Struct `ProfileCreatedEvent`](#mys_profile_ProfileCreatedEvent)
-  [Struct `ProfileUpdatedEvent`](#mys_profile_ProfileUpdatedEvent)
-  [Struct `UsernameUpdatedEvent`](#mys_profile_UsernameUpdatedEvent)
-  [Struct `UsernameNFTAssignedEvent`](#mys_profile_UsernameNFTAssignedEvent)
-  [Struct `UsernameNFTRemovedEvent`](#mys_profile_UsernameNFTRemovedEvent)
-  [Constants](#@Constants_0)
-  [Function `create_profile`](#mys_profile_create_profile)
-  [Function `create_and_register_profile`](#mys_profile_create_and_register_profile)
-  [Function `update_profile`](#mys_profile_update_profile)
-  [Function `display_name`](#mys_profile_display_name)
-  [Function `bio`](#mys_profile_bio)
-  [Function `profile_picture`](#mys_profile_profile_picture)
-  [Function `created_at`](#mys_profile_created_at)
-  [Function `owner`](#mys_profile_owner)
-  [Function `id`](#mys_profile_id)
-  [Function `has_username_nft`](#mys_profile_has_username_nft)
-  [Function `username_nft_id`](#mys_profile_username_nft_id)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/dynamic_field.md#mys_dynamic_field">mys::dynamic_field</a>;
<b>use</b> <a href="../mys/event.md#mys_event">mys::event</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../mys/url.md#mys_url">mys::url</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="mys_profile_Profile"></a>

## Struct `Profile`

Profile object that contains user information
Note: Profile is deliberately not transferable (no 'store' ability)


<pre><code><b>public</b> <b>struct</b> <a href="../mys/profile.md#mys_profile_Profile">Profile</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../mys/profile.md#mys_profile_id">id</a>: <a href="../mys/object.md#mys_object_UID">mys::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mys/profile.md#mys_profile_display_name">display_name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Display name of the profile
</dd>
<dt>
<code><a href="../mys/profile.md#mys_profile_bio">bio</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Bio of the profile
</dd>
<dt>
<code><a href="../mys/profile.md#mys_profile_profile_picture">profile_picture</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;</code>
</dt>
<dd>
 Profile picture URL
</dd>
<dt>
<code><a href="../mys/profile.md#mys_profile_created_at">created_at</a>: u64</code>
</dt>
<dd>
 Profile creation timestamp
</dd>
<dt>
<code><a href="../mys/profile.md#mys_profile_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
 Profile owner address
</dd>
</dl>


</details>

<a name="mys_profile_ProfileCreatedEvent"></a>

## Struct `ProfileCreatedEvent`

Profile created event


<pre><code><b>public</b> <b>struct</b> <a href="../mys/profile.md#mys_profile_ProfileCreatedEvent">ProfileCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mys/profile.md#mys_profile_display_name">display_name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mys/profile.md#mys_profile_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_profile_ProfileUpdatedEvent"></a>

## Struct `ProfileUpdatedEvent`

Profile updated event


<pre><code><b>public</b> <b>struct</b> <a href="../mys/profile.md#mys_profile_ProfileUpdatedEvent">ProfileUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mys/profile.md#mys_profile_display_name">display_name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mys/profile.md#mys_profile_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_profile_UsernameUpdatedEvent"></a>

## Struct `UsernameUpdatedEvent`

Username updated event


<pre><code><b>public</b> <b>struct</b> <a href="../mys/profile.md#mys_profile_UsernameUpdatedEvent">UsernameUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>old_username: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>new_username: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mys/profile.md#mys_profile_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_profile_UsernameNFTAssignedEvent"></a>

## Struct `UsernameNFTAssignedEvent`

Username NFT assigned event


<pre><code><b>public</b> <b>struct</b> <a href="../mys/profile.md#mys_profile_UsernameNFTAssignedEvent">UsernameNFTAssignedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>username_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>username: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>assigned_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mys_profile_UsernameNFTRemovedEvent"></a>

## Struct `UsernameNFTRemovedEvent`

Username NFT removed event


<pre><code><b>public</b> <b>struct</b> <a href="../mys/profile.md#mys_profile_UsernameNFTRemovedEvent">UsernameNFTRemovedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>username_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>removed_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mys_profile_EInvalidUsername"></a>



<pre><code><b>const</b> <a href="../mys/profile.md#mys_profile_EInvalidUsername">EInvalidUsername</a>: u64 = 4;
</code></pre>



<a name="mys_profile_ENameRegistryMismatch"></a>



<pre><code><b>const</b> <a href="../mys/profile.md#mys_profile_ENameRegistryMismatch">ENameRegistryMismatch</a>: u64 = 5;
</code></pre>



<a name="mys_profile_EProfileAlreadyExists"></a>

Error codes


<pre><code><b>const</b> <a href="../mys/profile.md#mys_profile_EProfileAlreadyExists">EProfileAlreadyExists</a>: u64 = 0;
</code></pre>



<a name="mys_profile_EUnauthorized"></a>



<pre><code><b>const</b> <a href="../mys/profile.md#mys_profile_EUnauthorized">EUnauthorized</a>: u64 = 1;
</code></pre>



<a name="mys_profile_EUsernameAlreadySet"></a>



<pre><code><b>const</b> <a href="../mys/profile.md#mys_profile_EUsernameAlreadySet">EUsernameAlreadySet</a>: u64 = 2;
</code></pre>



<a name="mys_profile_EUsernameNotRegistered"></a>



<pre><code><b>const</b> <a href="../mys/profile.md#mys_profile_EUsernameNotRegistered">EUsernameNotRegistered</a>: u64 = 3;
</code></pre>



<a name="mys_profile_USERNAME_NFT_FIELD"></a>

Field names for dynamic fields


<pre><code><b>const</b> <a href="../mys/profile.md#mys_profile_USERNAME_NFT_FIELD">USERNAME_NFT_FIELD</a>: vector&lt;u8&gt; = vector[117, 115, 101, 114, 110, 97, 109, 101, 95, 110, 102, 116];
</code></pre>



<a name="mys_profile_create_profile"></a>

## Function `create_profile`

Create a new profile


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_create_profile">create_profile</a>(<a href="../mys/profile.md#mys_profile_display_name">display_name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../mys/profile.md#mys_profile_bio">bio</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../mys/profile.md#mys_profile_profile_picture">profile_picture</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../mys/profile.md#mys_profile_Profile">mys::profile::Profile</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_create_profile">create_profile</a>(
    <a href="../mys/profile.md#mys_profile_display_name">display_name</a>: String,
    <a href="../mys/profile.md#mys_profile_bio">bio</a>: String,
    <a href="../mys/profile.md#mys_profile_profile_picture">profile_picture</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;Url&gt;,
    ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">tx_context::TxContext</a>
): <a href="../mys/profile.md#mys_profile_Profile">Profile</a> {
    <b>let</b> <a href="../mys/profile.md#mys_profile_owner">owner</a> = <a href="../mys/tx_context.md#mys_tx_context_sender">tx_context::sender</a>(ctx);
    <b>let</b> now = <a href="../mys/tx_context.md#mys_tx_context_epoch">tx_context::epoch</a>(ctx);
    <b>let</b> <a href="../mys/profile.md#mys_profile">profile</a> = <a href="../mys/profile.md#mys_profile_Profile">Profile</a> {
        <a href="../mys/profile.md#mys_profile_id">id</a>: <a href="../mys/object.md#mys_object_new">object::new</a>(ctx),
        <a href="../mys/profile.md#mys_profile_display_name">display_name</a>,
        <a href="../mys/profile.md#mys_profile_bio">bio</a>,
        <a href="../mys/profile.md#mys_profile_profile_picture">profile_picture</a>,
        <a href="../mys/profile.md#mys_profile_created_at">created_at</a>: now,
        <a href="../mys/profile.md#mys_profile_owner">owner</a>,
    };
    <a href="../mys/event.md#mys_event_emit">event::emit</a>(<a href="../mys/profile.md#mys_profile_ProfileCreatedEvent">ProfileCreatedEvent</a> {
        profile_id: <a href="../mys/object.md#mys_object_uid_to_address">object::uid_to_address</a>(&<a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_id">id</a>),
        <a href="../mys/profile.md#mys_profile_display_name">display_name</a>: <a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_display_name">display_name</a>,
        <a href="../mys/profile.md#mys_profile_owner">owner</a>,
    });
    <a href="../mys/profile.md#mys_profile">profile</a>
}
</code></pre>



</details>

<a name="mys_profile_create_and_register_profile"></a>

## Function `create_and_register_profile`

Create a new profile and transfer to sender


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_create_and_register_profile">create_and_register_profile</a>(<a href="../mys/profile.md#mys_profile_display_name">display_name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../mys/profile.md#mys_profile_bio">bio</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, profile_picture_url: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_create_and_register_profile">create_and_register_profile</a>(
    <a href="../mys/profile.md#mys_profile_display_name">display_name</a>: String,
    <a href="../mys/profile.md#mys_profile_bio">bio</a>: String,
    profile_picture_url: vector&lt;u8&gt;,
    ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">tx_context::TxContext</a>
) {
    <b>let</b> <a href="../mys/profile.md#mys_profile_profile_picture">profile_picture</a> = <b>if</b> (<a href="../std/vector.md#std_vector_length">std::vector::length</a>(&profile_picture_url) &gt; 0) {
        <a href="../std/option.md#std_option_some">std::option::some</a>(<a href="../mys/url.md#mys_url_new_unsafe_from_bytes">url::new_unsafe_from_bytes</a>(profile_picture_url))
    } <b>else</b> {
        <a href="../std/option.md#std_option_none">std::option::none</a>()
    };
    <b>let</b> <a href="../mys/profile.md#mys_profile">profile</a> = <a href="../mys/profile.md#mys_profile_create_profile">create_profile</a>(
        <a href="../mys/profile.md#mys_profile_display_name">display_name</a>,
        <a href="../mys/profile.md#mys_profile_bio">bio</a>,
        <a href="../mys/profile.md#mys_profile_profile_picture">profile_picture</a>,
        ctx
    );
    <a href="../mys/transfer.md#mys_transfer_transfer">transfer::transfer</a>(<a href="../mys/profile.md#mys_profile">profile</a>, <a href="../mys/tx_context.md#mys_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="mys_profile_update_profile"></a>

## Function `update_profile`

Update profile information


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_update_profile">update_profile</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<b>mut</b> <a href="../mys/profile.md#mys_profile_Profile">mys::profile::Profile</a>, new_display_name: <a href="../std/string.md#std_string_String">std::string::String</a>, new_bio: <a href="../std/string.md#std_string_String">std::string::String</a>, new_profile_picture_url: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_update_profile">update_profile</a>(
    <a href="../mys/profile.md#mys_profile">profile</a>: &<b>mut</b> <a href="../mys/profile.md#mys_profile_Profile">Profile</a>,
    new_display_name: String,
    new_bio: String,
    new_profile_picture_url: vector&lt;u8&gt;,
    ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">tx_context::TxContext</a>
) {
    <b>assert</b>!(<a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_owner">owner</a> == <a href="../mys/tx_context.md#mys_tx_context_sender">tx_context::sender</a>(ctx), <a href="../mys/profile.md#mys_profile_EUnauthorized">EUnauthorized</a>);
    <a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_display_name">display_name</a> = new_display_name;
    <a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_bio">bio</a> = new_bio;
    <b>if</b> (<a href="../std/vector.md#std_vector_length">std::vector::length</a>(&new_profile_picture_url) &gt; 0) {
        <a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_profile_picture">profile_picture</a> = <a href="../std/option.md#std_option_some">std::option::some</a>(<a href="../mys/url.md#mys_url_new_unsafe_from_bytes">url::new_unsafe_from_bytes</a>(new_profile_picture_url));
    };
    <a href="../mys/event.md#mys_event_emit">event::emit</a>(<a href="../mys/profile.md#mys_profile_ProfileUpdatedEvent">ProfileUpdatedEvent</a> {
        profile_id: <a href="../mys/object.md#mys_object_uid_to_address">object::uid_to_address</a>(&<a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_id">id</a>),
        <a href="../mys/profile.md#mys_profile_display_name">display_name</a>: <a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_display_name">display_name</a>,
        <a href="../mys/profile.md#mys_profile_owner">owner</a>: <a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_owner">owner</a>,
    });
}
</code></pre>



</details>

<a name="mys_profile_display_name"></a>

## Function `display_name`

Get the display name of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_display_name">display_name</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">mys::profile::Profile</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_display_name">display_name</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">Profile</a>): String {
    <a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_display_name">display_name</a>
}
</code></pre>



</details>

<a name="mys_profile_bio"></a>

## Function `bio`

Get the bio of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_bio">bio</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">mys::profile::Profile</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_bio">bio</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">Profile</a>): String {
    <a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_bio">bio</a>
}
</code></pre>



</details>

<a name="mys_profile_profile_picture"></a>

## Function `profile_picture`

Get the profile picture URL of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_profile_picture">profile_picture</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">mys::profile::Profile</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mys/url.md#mys_url_Url">mys::url::Url</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_profile_picture">profile_picture</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">Profile</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;Url&gt; {
    &<a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_profile_picture">profile_picture</a>
}
</code></pre>



</details>

<a name="mys_profile_created_at"></a>

## Function `created_at`

Get the creation timestamp of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_created_at">created_at</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">mys::profile::Profile</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_created_at">created_at</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">Profile</a>): u64 {
    <a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_created_at">created_at</a>
}
</code></pre>



</details>

<a name="mys_profile_owner"></a>

## Function `owner`

Get the owner of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_owner">owner</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">mys::profile::Profile</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_owner">owner</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">Profile</a>): <b>address</b> {
    <a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_owner">owner</a>
}
</code></pre>



</details>

<a name="mys_profile_id"></a>

## Function `id`

Get the ID of a profile


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_id">id</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">mys::profile::Profile</a>): &<a href="../mys/object.md#mys_object_UID">mys::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_id">id</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">Profile</a>): &<a href="../mys/object.md#mys_object_UID">object::UID</a> {
    &<a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_id">id</a>
}
</code></pre>



</details>

<a name="mys_profile_has_username_nft"></a>

## Function `has_username_nft`

Check if a profile has a username NFT reference


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_has_username_nft">has_username_nft</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">mys::profile::Profile</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_has_username_nft">has_username_nft</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">Profile</a>): bool {
    <a href="../mys/dynamic_field.md#mys_dynamic_field_exists_">dynamic_field::exists_</a>(&<a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_id">id</a>, <a href="../mys/profile.md#mys_profile_USERNAME_NFT_FIELD">USERNAME_NFT_FIELD</a>)
}
</code></pre>



</details>

<a name="mys_profile_username_nft_id"></a>

## Function `username_nft_id`

Get the username NFT ID associated with this profile


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_username_nft_id">username_nft_id</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">mys::profile::Profile</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mys/profile.md#mys_profile_username_nft_id">username_nft_id</a>(<a href="../mys/profile.md#mys_profile">profile</a>: &<a href="../mys/profile.md#mys_profile_Profile">Profile</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt; {
    <b>if</b> (<a href="../mys/dynamic_field.md#mys_dynamic_field_exists_">dynamic_field::exists_</a>(&<a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_id">id</a>, <a href="../mys/profile.md#mys_profile_USERNAME_NFT_FIELD">USERNAME_NFT_FIELD</a>)) {
        <a href="../std/option.md#std_option_some">std::option::some</a>(*<a href="../mys/dynamic_field.md#mys_dynamic_field_borrow">dynamic_field::borrow</a>&lt;vector&lt;u8&gt;, <b>address</b>&gt;(&<a href="../mys/profile.md#mys_profile">profile</a>.<a href="../mys/profile.md#mys_profile_id">id</a>, <a href="../mys/profile.md#mys_profile_USERNAME_NFT_FIELD">USERNAME_NFT_FIELD</a>))
    } <b>else</b> {
        <a href="../std/option.md#std_option_none">std::option::none</a>()
    }
}
</code></pre>



</details>
