---
title: Module `seal::polynomial`
---



-  [Struct `Polynomial`](#seal_polynomial_Polynomial)
-  [Function `get_constant_term`](#seal_polynomial_get_constant_term)
-  [Function `add`](#seal_polynomial_add)
-  [Function `degree`](#seal_polynomial_degree)
-  [Function `reduce`](#seal_polynomial_reduce)
-  [Function `mul`](#seal_polynomial_mul)
-  [Function `div`](#seal_polynomial_div)
-  [Function `scale`](#seal_polynomial_scale)
-  [Function `monic_linear`](#seal_polynomial_monic_linear)
-  [Function `interpolate`](#seal_polynomial_interpolate)
-  [Function `evaluate`](#seal_polynomial_evaluate)


<pre><code><b>use</b> <a href="../seal/gf256.md#seal_gf256">seal::gf256</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="seal_polynomial_Polynomial"></a>

## Struct `Polynomial`

This represents a polynomial over GF(2^8).
The first coefficient is the constant term.


<pre><code><b>public</b> <b>struct</b> <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coefficients: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="seal_polynomial_get_constant_term"></a>

## Function `get_constant_term`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_get_constant_term">get_constant_term</a>(p: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_get_constant_term">get_constant_term</a>(p: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a>): u8 {
    <b>if</b> (p.coefficients.is_empty()) {
        <b>return</b> 0
    };
    p.coefficients[0]
}
</code></pre>



</details>

<a name="seal_polynomial_add"></a>

## Function `add`



<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_add">add</a>(x: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>, y: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_add">add</a>(x: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a>, y: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a>): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> {
    <b>let</b> x_length: u64 = x.coefficients.length();
    <b>let</b> y_length: u64 = y.coefficients.length();
    <b>if</b> (x_length &lt; y_length) {
        // We assume that x is the longer vector
        <b>return</b> <a href="../seal/polynomial.md#seal_polynomial_add">add</a>(y, x)
    };
    <b>let</b> <b>mut</b> coefficients: vector&lt;u8&gt; = vector::empty&lt;u8&gt;();
    y_length.do!(|i| coefficients.push_back(<a href="../seal/gf256.md#seal_gf256_add">gf256::add</a>(x.coefficients[i], y.coefficients[i])));
    (x_length - y_length).do!(|i| coefficients.push_back(x.coefficients[i + y_length]));
    <b>let</b> result = <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> { coefficients };
    <a href="../seal/polynomial.md#seal_polynomial_reduce">reduce</a>(result);
    result
}
</code></pre>



</details>

<a name="seal_polynomial_degree"></a>

## Function `degree`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_degree">degree</a>(x: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_degree">degree</a>(x: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a>): u64 {
    x.coefficients.length() - 1
}
</code></pre>



</details>

<a name="seal_polynomial_reduce"></a>

## Function `reduce`



<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_reduce">reduce</a>(x: <a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_reduce">reduce</a>(<b>mut</b> x: <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a>) {
    <b>while</b> (x.coefficients.length() &gt; 0 && x.coefficients[x.coefficients.length() - 1] == 0) {
        x.coefficients.pop_back();
    };
}
</code></pre>



</details>

<a name="seal_polynomial_mul"></a>

## Function `mul`



<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_mul">mul</a>(x: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>, y: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_mul">mul</a>(x: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a>, y: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a>): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> {
    <b>let</b> <a href="../seal/polynomial.md#seal_polynomial_degree">degree</a> = x.<a href="../seal/polynomial.md#seal_polynomial_degree">degree</a>() + y.<a href="../seal/polynomial.md#seal_polynomial_degree">degree</a>();
    <b>let</b> coefficients = vector::tabulate!(<a href="../seal/polynomial.md#seal_polynomial_degree">degree</a> + 1, |i| {
        <b>let</b> <b>mut</b> sum = 0;
        i.do_eq!(|j| {
            <b>if</b> (j &lt;= x.<a href="../seal/polynomial.md#seal_polynomial_degree">degree</a>() && i - j &lt;= y.<a href="../seal/polynomial.md#seal_polynomial_degree">degree</a>()) {
                sum = <a href="../seal/gf256.md#seal_gf256_add">gf256::add</a>(sum, <a href="../seal/gf256.md#seal_gf256_mul">gf256::mul</a>(x.coefficients[j], y.coefficients[i - j]));
            }
        });
        sum
    });
    <b>let</b> result = <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> { coefficients };
    <a href="../seal/polynomial.md#seal_polynomial_reduce">reduce</a>(result);
    result
}
</code></pre>



</details>

<a name="seal_polynomial_div"></a>

## Function `div`



<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_div">div</a>(x: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>, s: u8): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_div">div</a>(x: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a>, s: u8): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> {
    <a href="../seal/polynomial.md#seal_polynomial_scale">scale</a>(x, <a href="../seal/gf256.md#seal_gf256_div">gf256::div</a>(1, s))
}
</code></pre>



</details>

<a name="seal_polynomial_scale"></a>

## Function `scale`



<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_scale">scale</a>(x: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>, s: u8): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_scale">scale</a>(x: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a>, s: u8): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> {
    <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> { coefficients: x.coefficients.map_ref!(|c| <a href="../seal/gf256.md#seal_gf256_mul">gf256::mul</a>(*c, s)) }
}
</code></pre>



</details>

<a name="seal_polynomial_monic_linear"></a>

## Function `monic_linear`

Return x - c


<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_monic_linear">monic_linear</a>(c: &u8): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_monic_linear">monic_linear</a>(c: &u8): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> {
    <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> { coefficients: vector[<a href="../seal/gf256.md#seal_gf256_sub">gf256::sub</a>(0, *c), 1] }
}
</code></pre>



</details>

<a name="seal_polynomial_interpolate"></a>

## Function `interpolate`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_interpolate">interpolate</a>(x: &vector&lt;u8&gt;, y: &vector&lt;u8&gt;): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_interpolate">interpolate</a>(x: &vector&lt;u8&gt;, y: &vector&lt;u8&gt;): <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> {
    <b>assert</b>!(x.length() == y.length());
    <b>let</b> n = x.length();
    <b>let</b> <b>mut</b> sum = <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> { coefficients: vector::empty&lt;u8&gt;() };
    n.do!(|j| {
        <b>let</b> <b>mut</b> product = <a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a> { coefficients: vector[1] };
        n.do!(|i| {
            <b>if</b> (i != j) {
                product =
                    <a href="../seal/polynomial.md#seal_polynomial_mul">mul</a>(
                        &product,
                        &<a href="../seal/polynomial.md#seal_polynomial_div">div</a>(&<a href="../seal/polynomial.md#seal_polynomial_monic_linear">monic_linear</a>(&x[i]), <a href="../seal/gf256.md#seal_gf256_sub">gf256::sub</a>(x[j], x[i])),
                    );
            };
        });
        sum = <a href="../seal/polynomial.md#seal_polynomial_add">add</a>(&sum, &<a href="../seal/polynomial.md#seal_polynomial_scale">scale</a>(&product, y[j]));
    });
    sum
}
</code></pre>



</details>

<a name="seal_polynomial_evaluate"></a>

## Function `evaluate`



<pre><code><b>public</b> <b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_evaluate">evaluate</a>(p: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">seal::polynomial::Polynomial</a>, x: u8): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../seal/polynomial.md#seal_polynomial_evaluate">evaluate</a>(p: &<a href="../seal/polynomial.md#seal_polynomial_Polynomial">Polynomial</a>, x: u8): u8 {
    <b>let</b> <b>mut</b> result = 0;
    <b>let</b> n = p.coefficients.length();
    n.do!(|i| {
        result = <a href="../seal/gf256.md#seal_gf256_add">gf256::add</a>(<a href="../seal/gf256.md#seal_gf256_mul">gf256::mul</a>(result, x), p.coefficients[n - i - 1]);
    });
    result
}
</code></pre>



</details>
