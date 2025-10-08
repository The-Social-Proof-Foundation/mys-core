---
title: Module `mydata::kdf`
---



-  [Constants](#@Constants_0)
-  [Function `kdf`](#mydata_kdf_kdf)
-  [Function `hash_to_g1_with_dst`](#mydata_kdf_hash_to_g1_with_dst)


<pre><code><b>use</b> <a href="../mys/address.md#mys_address">mys::address</a>;
<b>use</b> <a href="../mys/bcs.md#mys_bcs">mys::bcs</a>;
<b>use</b> <a href="../mys/bls12381.md#mys_bls12381">mys::bls12381</a>;
<b>use</b> <a href="../mys/group_ops.md#mys_group_ops">mys::group_ops</a>;
<b>use</b> <a href="../mys/hex.md#mys_hex">mys::hex</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/hash.md#std_hash">std::hash</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="mydata_kdf_DST_ID"></a>



<pre><code><b>const</b> <a href="../mydata/kdf.md#mydata_kdf_DST_ID">DST_ID</a>: vector&lt;u8&gt; = vector[77, 89, 83, 45, 77, 89, 68, 65, 84, 65, 45, 73, 66, 69, 45, 66, 76, 83, 49, 50, 51, 56, 49, 45, 48, 48];
</code></pre>



<a name="mydata_kdf_DST_KDF"></a>



<pre><code><b>const</b> <a href="../mydata/kdf.md#mydata_kdf_DST_KDF">DST_KDF</a>: vector&lt;u8&gt; = vector[77, 89, 83, 45, 77, 89, 68, 65, 84, 65, 45, 73, 66, 69, 45, 66, 76, 83, 49, 50, 51, 56, 49, 45, 72, 50, 45, 48, 48];
</code></pre>



<a name="mydata_kdf_kdf"></a>

## Function `kdf`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../mydata/kdf.md#mydata_kdf">kdf</a>(input: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_GT">mys::bls12381::GT</a>&gt;, nonce: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G2">mys::bls12381::G2</a>&gt;, gid: &<a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G1">mys::bls12381::G1</a>&gt;, object_id: <b>address</b>, index: u8): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mydata/kdf.md#mydata_kdf">kdf</a>(
    input: &Element&lt;GT&gt;,
    nonce: &Element&lt;G2&gt;,
    gid: &Element&lt;G1&gt;,
    object_id: <b>address</b>,
    index: u8,
): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> bytes = <a href="../mydata/kdf.md#mydata_kdf_DST_KDF">DST_KDF</a>;
    bytes.append(*input.bytes());
    bytes.append(*nonce.bytes());
    bytes.append(*gid.bytes());
    bytes.append(object_id.to_bytes());
    bytes.push_back(index);
    sha3_256(bytes)
}
</code></pre>



</details>

<a name="mydata_kdf_hash_to_g1_with_dst"></a>

## Function `hash_to_g1_with_dst`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../mydata/kdf.md#mydata_kdf_hash_to_g1_with_dst">hash_to_g1_with_dst</a>(id: &vector&lt;u8&gt;): <a href="../mys/group_ops.md#mys_group_ops_Element">mys::group_ops::Element</a>&lt;<a href="../mys/bls12381.md#mys_bls12381_G1">mys::bls12381::G1</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../mydata/kdf.md#mydata_kdf_hash_to_g1_with_dst">hash_to_g1_with_dst</a>(id: &vector&lt;u8&gt;): Element&lt;G1&gt; {
    <b>let</b> <b>mut</b> bytes = <a href="../mydata/kdf.md#mydata_kdf_DST_ID">DST_ID</a>;
    bytes.append(*id);
    bls12381::hash_to_g1(&bytes)
}
</code></pre>



</details>
