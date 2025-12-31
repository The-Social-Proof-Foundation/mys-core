---
title: Module `social_contracts::enclave`
---



-  [Struct `Pcrs`](#social_contracts_enclave_Pcrs)
-  [Struct `EnclaveConfig`](#social_contracts_enclave_EnclaveConfig)
-  [Struct `Enclave`](#social_contracts_enclave_Enclave)
-  [Struct `Cap`](#social_contracts_enclave_Cap)
-  [Struct `IntentMessage`](#social_contracts_enclave_IntentMessage)
-  [Constants](#@Constants_0)
-  [Function `new_cap`](#social_contracts_enclave_new_cap)
-  [Function `create_enclave_config`](#social_contracts_enclave_create_enclave_config)
-  [Function `register_enclave`](#social_contracts_enclave_register_enclave)
-  [Function `verify_signature`](#social_contracts_enclave_verify_signature)
-  [Function `update_pcrs`](#social_contracts_enclave_update_pcrs)
-  [Function `update_name`](#social_contracts_enclave_update_name)
-  [Function `pcr0`](#social_contracts_enclave_pcr0)
-  [Function `pcr1`](#social_contracts_enclave_pcr1)
-  [Function `pcr2`](#social_contracts_enclave_pcr2)
-  [Function `pk`](#social_contracts_enclave_pk)
-  [Function `destroy_old_enclave`](#social_contracts_enclave_destroy_old_enclave)
-  [Function `deploy_old_enclave_by_owner`](#social_contracts_enclave_deploy_old_enclave_by_owner)
-  [Function `assert_is_valid_for_config`](#social_contracts_enclave_assert_is_valid_for_config)
-  [Function `load_pk`](#social_contracts_enclave_load_pk)
-  [Function `to_pcrs`](#social_contracts_enclave_to_pcrs)
-  [Function `create_intent_message`](#social_contracts_enclave_create_intent_message)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/clock.md#mys_clock">mys::clock</a>;
<b>use</b> <a href="../mys/ed25519.md#mys_ed25519">mys::ed25519</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../mys/nitro_attestation.md#mys_nitro_attestation">mys::nitro_attestation</a>;
<b>use</b> <a href="../mys/object.md#mys_object">mys::object</a>;
<b>use</b> <a href="../mys/transfer.md#mys_transfer">mys::transfer</a>;
<b>use</b> <a href="../mys/tx_context.md#mys_tx_context">mys::tx_context</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_enclave_Pcrs"></a>

## Struct `Pcrs`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_Pcrs">Pcrs</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>0: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>1: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>2: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_enclave_EnclaveConfig"></a>

## Struct `EnclaveConfig`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;<b>phantom</b> T&gt; <b>has</b> key
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
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>pcrs: <a href="../social_contracts/enclave.md#social_contracts_enclave_Pcrs">social_contracts::enclave::Pcrs</a></code>
</dt>
<dd>
</dd>
<dt>
<code>capability_id: <a href="../mys/object.md#mys_object_ID">mys::object::ID</a></code>
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

<a name="social_contracts_enclave_Enclave"></a>

## Struct `Enclave`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">Enclave</a>&lt;<b>phantom</b> T&gt; <b>has</b> key
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
<code><a href="../social_contracts/enclave.md#social_contracts_enclave_pk">pk</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>config_version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_enclave_Cap"></a>

## Struct `Cap`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">Cap</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
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

<a name="social_contracts_enclave_IntentMessage"></a>

## Struct `IntentMessage`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_IntentMessage">IntentMessage</a>&lt;T: drop&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>intent: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>payload: T</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_enclave_EInvalidCap"></a>



<pre><code><b>const</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_EInvalidCap">EInvalidCap</a>: u64 = 2;
</code></pre>



<a name="social_contracts_enclave_EInvalidConfigVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_EInvalidConfigVersion">EInvalidConfigVersion</a>: u64 = 1;
</code></pre>



<a name="social_contracts_enclave_EInvalidOwner"></a>



<pre><code><b>const</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_EInvalidOwner">EInvalidOwner</a>: u64 = 3;
</code></pre>



<a name="social_contracts_enclave_EInvalidPCRs"></a>



<pre><code><b>const</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_EInvalidPCRs">EInvalidPCRs</a>: u64 = 0;
</code></pre>



<a name="social_contracts_enclave_new_cap"></a>

## Function `new_cap`

Create a new <code><a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">Cap</a></code> using a <code>witness</code> T from a module.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_new_cap">new_cap</a>&lt;T: drop&gt;(_: T, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>): <a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">social_contracts::enclave::Cap</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_new_cap">new_cap</a>&lt;T: drop&gt;(_: T, ctx: &<b>mut</b> TxContext): <a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">Cap</a>&lt;T&gt; {
    <a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">Cap</a> {
        id: object::new(ctx),
    }
}
</code></pre>



</details>

<a name="social_contracts_enclave_create_enclave_config"></a>

## Function `create_enclave_config`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_create_enclave_config">create_enclave_config</a>&lt;T: drop&gt;(cap: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">social_contracts::enclave::Cap</a>&lt;T&gt;, name: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr0">pcr0</a>: vector&lt;u8&gt;, <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr1">pcr1</a>: vector&lt;u8&gt;, <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr2">pcr2</a>: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_create_enclave_config">create_enclave_config</a>&lt;T: drop&gt;(
    cap: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">Cap</a>&lt;T&gt;,
    name: String,
    <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr0">pcr0</a>: vector&lt;u8&gt;,
    <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr1">pcr1</a>: vector&lt;u8&gt;,
    <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr2">pcr2</a>: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> enclave_config = <a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;T&gt; {
        id: object::new(ctx),
        name,
        pcrs: <a href="../social_contracts/enclave.md#social_contracts_enclave_Pcrs">Pcrs</a>(<a href="../social_contracts/enclave.md#social_contracts_enclave_pcr0">pcr0</a>, <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr1">pcr1</a>, <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr2">pcr2</a>),
        capability_id: cap.id.to_inner(),
        version: 0,
    };
    transfer::share_object(enclave_config);
}
</code></pre>



</details>

<a name="social_contracts_enclave_register_enclave"></a>

## Function `register_enclave`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_register_enclave">register_enclave</a>&lt;T&gt;(enclave_config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">social_contracts::enclave::EnclaveConfig</a>&lt;T&gt;, document: <a href="../mys/nitro_attestation.md#mys_nitro_attestation_NitroAttestationDocument">mys::nitro_attestation::NitroAttestationDocument</a>, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_register_enclave">register_enclave</a>&lt;T&gt;(
    enclave_config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;T&gt;,
    document: NitroAttestationDocument,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_pk">pk</a> = <a href="../social_contracts/enclave.md#social_contracts_enclave_load_pk">load_pk</a>(enclave_config, &document);
    <b>let</b> _enclave = <a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">Enclave</a>&lt;T&gt; {
        id: object::new(ctx),
        <a href="../social_contracts/enclave.md#social_contracts_enclave_pk">pk</a>,
        config_version: enclave_config.version,
        owner: tx_context::sender(ctx),
    };
    transfer::share_object(_enclave);
}
</code></pre>



</details>

<a name="social_contracts_enclave_verify_signature"></a>

## Function `verify_signature`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_verify_signature">verify_signature</a>&lt;T, P: drop&gt;(<a href="../social_contracts/enclave.md#social_contracts_enclave">enclave</a>: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">social_contracts::enclave::Enclave</a>&lt;T&gt;, intent_scope: u8, timestamp_ms: u64, payload: P, signature: &vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_verify_signature">verify_signature</a>&lt;T, P: drop&gt;(
    <a href="../social_contracts/enclave.md#social_contracts_enclave">enclave</a>: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">Enclave</a>&lt;T&gt;,
    intent_scope: u8,
    timestamp_ms: u64,
    payload: P,
    signature: &vector&lt;u8&gt;,
): bool {
    <b>let</b> intent_message = <a href="../social_contracts/enclave.md#social_contracts_enclave_create_intent_message">create_intent_message</a>(intent_scope, timestamp_ms, payload);
    <b>let</b> payload = bcs::to_bytes(&intent_message);
    <b>return</b> ed25519::ed25519_verify(signature, &<a href="../social_contracts/enclave.md#social_contracts_enclave">enclave</a>.<a href="../social_contracts/enclave.md#social_contracts_enclave_pk">pk</a>, &payload)
}
</code></pre>



</details>

<a name="social_contracts_enclave_update_pcrs"></a>

## Function `update_pcrs`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_update_pcrs">update_pcrs</a>&lt;T: drop&gt;(config: &<b>mut</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">social_contracts::enclave::EnclaveConfig</a>&lt;T&gt;, cap: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">social_contracts::enclave::Cap</a>&lt;T&gt;, <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr0">pcr0</a>: vector&lt;u8&gt;, <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr1">pcr1</a>: vector&lt;u8&gt;, <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr2">pcr2</a>: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_update_pcrs">update_pcrs</a>&lt;T: drop&gt;(
    config: &<b>mut</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;T&gt;,
    cap: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">Cap</a>&lt;T&gt;,
    <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr0">pcr0</a>: vector&lt;u8&gt;,
    <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr1">pcr1</a>: vector&lt;u8&gt;,
    <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr2">pcr2</a>: vector&lt;u8&gt;,
) {
    cap.<a href="../social_contracts/enclave.md#social_contracts_enclave_assert_is_valid_for_config">assert_is_valid_for_config</a>(config);
    config.pcrs = <a href="../social_contracts/enclave.md#social_contracts_enclave_Pcrs">Pcrs</a>(<a href="../social_contracts/enclave.md#social_contracts_enclave_pcr0">pcr0</a>, <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr1">pcr1</a>, <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr2">pcr2</a>);
    config.version = config.version + 1;
}
</code></pre>



</details>

<a name="social_contracts_enclave_update_name"></a>

## Function `update_name`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_update_name">update_name</a>&lt;T: drop&gt;(config: &<b>mut</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">social_contracts::enclave::EnclaveConfig</a>&lt;T&gt;, cap: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">social_contracts::enclave::Cap</a>&lt;T&gt;, name: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_update_name">update_name</a>&lt;T: drop&gt;(config: &<b>mut</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;T&gt;, cap: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">Cap</a>&lt;T&gt;, name: String) {
    cap.<a href="../social_contracts/enclave.md#social_contracts_enclave_assert_is_valid_for_config">assert_is_valid_for_config</a>(config);
    config.name = name;
}
</code></pre>



</details>

<a name="social_contracts_enclave_pcr0"></a>

## Function `pcr0`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr0">pcr0</a>&lt;T&gt;(config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">social_contracts::enclave::EnclaveConfig</a>&lt;T&gt;): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr0">pcr0</a>&lt;T&gt;(config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;T&gt;): &vector&lt;u8&gt; {
    &config.pcrs.0
}
</code></pre>



</details>

<a name="social_contracts_enclave_pcr1"></a>

## Function `pcr1`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr1">pcr1</a>&lt;T&gt;(config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">social_contracts::enclave::EnclaveConfig</a>&lt;T&gt;): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr1">pcr1</a>&lt;T&gt;(config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;T&gt;): &vector&lt;u8&gt; {
    &config.pcrs.1
}
</code></pre>



</details>

<a name="social_contracts_enclave_pcr2"></a>

## Function `pcr2`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr2">pcr2</a>&lt;T&gt;(config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">social_contracts::enclave::EnclaveConfig</a>&lt;T&gt;): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_pcr2">pcr2</a>&lt;T&gt;(config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;T&gt;): &vector&lt;u8&gt; {
    &config.pcrs.2
}
</code></pre>



</details>

<a name="social_contracts_enclave_pk"></a>

## Function `pk`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_pk">pk</a>&lt;T&gt;(<a href="../social_contracts/enclave.md#social_contracts_enclave">enclave</a>: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">social_contracts::enclave::Enclave</a>&lt;T&gt;): &vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_pk">pk</a>&lt;T&gt;(<a href="../social_contracts/enclave.md#social_contracts_enclave">enclave</a>: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">Enclave</a>&lt;T&gt;): &vector&lt;u8&gt; {
    &<a href="../social_contracts/enclave.md#social_contracts_enclave">enclave</a>.<a href="../social_contracts/enclave.md#social_contracts_enclave_pk">pk</a>
}
</code></pre>



</details>

<a name="social_contracts_enclave_destroy_old_enclave"></a>

## Function `destroy_old_enclave`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_destroy_old_enclave">destroy_old_enclave</a>&lt;T&gt;(e: <a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">social_contracts::enclave::Enclave</a>&lt;T&gt;, config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">social_contracts::enclave::EnclaveConfig</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_destroy_old_enclave">destroy_old_enclave</a>&lt;T&gt;(e: <a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">Enclave</a>&lt;T&gt;, config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;T&gt;) {
    <b>assert</b>!(e.config_version &lt; config.version, <a href="../social_contracts/enclave.md#social_contracts_enclave_EInvalidConfigVersion">EInvalidConfigVersion</a>);
    <b>let</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">Enclave</a> { id, .. } = e;
    id.delete();
}
</code></pre>



</details>

<a name="social_contracts_enclave_deploy_old_enclave_by_owner"></a>

## Function `deploy_old_enclave_by_owner`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_deploy_old_enclave_by_owner">deploy_old_enclave_by_owner</a>&lt;T&gt;(e: <a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">social_contracts::enclave::Enclave</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../mys/tx_context.md#mys_tx_context_TxContext">mys::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_deploy_old_enclave_by_owner">deploy_old_enclave_by_owner</a>&lt;T&gt;(e: <a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">Enclave</a>&lt;T&gt;, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(e.owner == tx_context::sender(ctx), <a href="../social_contracts/enclave.md#social_contracts_enclave_EInvalidOwner">EInvalidOwner</a>);
    <b>let</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_Enclave">Enclave</a> { id, .. } = e;
    id.delete();
}
</code></pre>



</details>

<a name="social_contracts_enclave_assert_is_valid_for_config"></a>

## Function `assert_is_valid_for_config`



<pre><code><b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_assert_is_valid_for_config">assert_is_valid_for_config</a>&lt;T&gt;(cap: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">social_contracts::enclave::Cap</a>&lt;T&gt;, enclave_config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">social_contracts::enclave::EnclaveConfig</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_assert_is_valid_for_config">assert_is_valid_for_config</a>&lt;T&gt;(cap: &<a href="../social_contracts/enclave.md#social_contracts_enclave_Cap">Cap</a>&lt;T&gt;, enclave_config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;T&gt;) {
    <b>assert</b>!(cap.id.to_inner() == enclave_config.capability_id, <a href="../social_contracts/enclave.md#social_contracts_enclave_EInvalidCap">EInvalidCap</a>);
}
</code></pre>



</details>

<a name="social_contracts_enclave_load_pk"></a>

## Function `load_pk`



<pre><code><b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_load_pk">load_pk</a>&lt;T&gt;(enclave_config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">social_contracts::enclave::EnclaveConfig</a>&lt;T&gt;, document: &<a href="../mys/nitro_attestation.md#mys_nitro_attestation_NitroAttestationDocument">mys::nitro_attestation::NitroAttestationDocument</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_load_pk">load_pk</a>&lt;T&gt;(enclave_config: &<a href="../social_contracts/enclave.md#social_contracts_enclave_EnclaveConfig">EnclaveConfig</a>&lt;T&gt;, document: &NitroAttestationDocument): vector&lt;u8&gt; {
    <b>assert</b>!(<a href="../social_contracts/enclave.md#social_contracts_enclave_to_pcrs">to_pcrs</a>(document) == enclave_config.pcrs, <a href="../social_contracts/enclave.md#social_contracts_enclave_EInvalidPCRs">EInvalidPCRs</a>);
    <b>let</b> public_key_option = nitro_attestation::public_key(document);
    <b>assert</b>!(option::is_some(public_key_option), <a href="../social_contracts/enclave.md#social_contracts_enclave_EInvalidPCRs">EInvalidPCRs</a>);
    *option::borrow(public_key_option)
}
</code></pre>



</details>

<a name="social_contracts_enclave_to_pcrs"></a>

## Function `to_pcrs`



<pre><code><b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_to_pcrs">to_pcrs</a>(document: &<a href="../mys/nitro_attestation.md#mys_nitro_attestation_NitroAttestationDocument">mys::nitro_attestation::NitroAttestationDocument</a>): <a href="../social_contracts/enclave.md#social_contracts_enclave_Pcrs">social_contracts::enclave::Pcrs</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_to_pcrs">to_pcrs</a>(document: &NitroAttestationDocument): <a href="../social_contracts/enclave.md#social_contracts_enclave_Pcrs">Pcrs</a> {
    <b>let</b> pcrs_vec = nitro_attestation::pcrs(document);
    // PCRs are stored in fixed order [0, 1, 2, 3, 4, 8] per <b>native</b> implementation
    <b>let</b> pcr0_entry = vector::borrow(pcrs_vec, 0);
    <b>let</b> pcr1_entry = vector::borrow(pcrs_vec, 1);
    <b>let</b> pcr2_entry = vector::borrow(pcrs_vec, 2);
    <a href="../social_contracts/enclave.md#social_contracts_enclave_Pcrs">Pcrs</a>(*nitro_attestation::value(pcr0_entry), *nitro_attestation::value(pcr1_entry), *nitro_attestation::value(pcr2_entry))
}
</code></pre>



</details>

<a name="social_contracts_enclave_create_intent_message"></a>

## Function `create_intent_message`



<pre><code><b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_create_intent_message">create_intent_message</a>&lt;P: drop&gt;(intent: u8, timestamp_ms: u64, payload: P): <a href="../social_contracts/enclave.md#social_contracts_enclave_IntentMessage">social_contracts::enclave::IntentMessage</a>&lt;P&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/enclave.md#social_contracts_enclave_create_intent_message">create_intent_message</a>&lt;P: drop&gt;(intent: u8, timestamp_ms: u64, payload: P): <a href="../social_contracts/enclave.md#social_contracts_enclave_IntentMessage">IntentMessage</a>&lt;P&gt; {
    <a href="../social_contracts/enclave.md#social_contracts_enclave_IntentMessage">IntentMessage</a> {
        intent,
        timestamp_ms,
        payload,
    }
}
</code></pre>



</details>
