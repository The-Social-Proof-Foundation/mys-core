---
title: Module `mydata::hmac256ctr`
---



-  [Constants](#@Constants_0)
-  [Function `decrypt`](#mydata_hmac256ctr_decrypt)
-  [Function `mac`](#mydata_hmac256ctr_mac)


<pre><code><b>use</b> <a href="../mys/hmac.md#mys_hmac">mys::hmac</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="mydata_hmac256ctr_ENC_TAG"></a>



<pre><code><b>const</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_ENC_TAG">ENC_TAG</a>: vector&lt;u8&gt; = vector[72, 77, 65, 67, 45, 67, 84, 82, 45, 69, 78, 67];
</code></pre>



<a name="mydata_hmac256ctr_MAC_TAG"></a>



<pre><code><b>const</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_MAC_TAG">MAC_TAG</a>: vector&lt;u8&gt; = vector[72, 77, 65, 67, 45, 67, 84, 82, 45, 77, 65, 67];
</code></pre>



<a name="mydata_hmac256ctr_decrypt"></a>

## Function `decrypt`

Decrypt a message that was encrypted in Hmac256Ctr mode.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_decrypt">decrypt</a>(ciphertext: &vector&lt;u8&gt;, <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_mac">mac</a>: &vector&lt;u8&gt;, aad: &vector&lt;u8&gt;, key: &vector&lt;u8&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_decrypt">decrypt</a>(
    ciphertext: &vector&lt;u8&gt;,
    <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_mac">mac</a>: &vector&lt;u8&gt;,
    aad: &vector&lt;u8&gt;,
    key: &vector&lt;u8&gt;,
): Option&lt;vector&lt;u8&gt;&gt; {
    <b>if</b> (<a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_mac">mac</a>(key, aad, ciphertext) != <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_mac">mac</a>) {
        <b>return</b> option::none()
    };
    <b>let</b> <b>mut</b> next_block = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> current_mask = vector[];
    option::some(ciphertext.map_ref!(|b| {
        <b>if</b> (i == 0) {
            current_mask =
                hmac_sha3_256(key, &vector[<a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_ENC_TAG">ENC_TAG</a>, bcs::to_bytes(&(next_block <b>as</b> u64))].flatten());
            next_block = next_block + 1;
        };
        <b>let</b> result = *b ^ current_mask[i];
        i = (i + 1) % 32;
        result
    }))
}
</code></pre>



</details>

<a name="mydata_hmac256ctr_mac"></a>

## Function `mac`



<pre><code><b>fun</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_mac">mac</a>(key: &vector&lt;u8&gt;, aux: &vector&lt;u8&gt;, ciphertext: &vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_mac">mac</a>(key: &vector&lt;u8&gt;, aux: &vector&lt;u8&gt;, ciphertext: &vector&lt;u8&gt;): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> mac_input = <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr_MAC_TAG">MAC_TAG</a>;
    mac_input.append(bcs::to_bytes(&aux.length()));
    mac_input.append(*aux);
    mac_input.append(*ciphertext);
    hmac_sha3_256(key, &mac_input)
}
</code></pre>



</details>
