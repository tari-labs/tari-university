# Schnorr signatures

If you follow the crypto news, you'll know that that new hotness in Bitcoin is Schnorr Signatures.

![schnorr](./img/schnorr-meme.jpg)

But in actual fact, they're old news! The Schnorr signature is considered the simplest digital signature scheme 
to be provably secure in a random oracle model. It is efficient and generates short signatures. 
It was covered by U.S. Patent 4,995,082 which expired in February 2008 [WP1].

## So why all the fuss?

What makes Schnorr signatures to interesting (and potentially dangerous) is their simplicity. 
Schnorr signatures are _linear_, so you have some nice properties.

Given two private keys _x, y_ with corresponding public keys, _X, Y_, the following holds:

\\[
  (x + y)G = xG + yG = X + Y 
\\]

You saw this multiplicative property in the previous section, when we were verifying the signature. It's this 
linear property of Schnorr signatures that makes it very attractive for things like

* signature aggregation
* atomic swaps
* 'scriptless' scripts

# (Naïve) Signature aggregation

Let's see how the linearity property of Schnorr signatures can be used to construct a 2-of-2 multi-signature.

Alice and Bob want to co-sign something (a Tari transaction, say) without having to trust each other; 
i.e. they need to be able to prove ownership of their respective keys, and the aggregate signature is
only valid if _both_ Alice and Bob provide their part of the signature. 

Assuming private keys are denoted \\( k_i \\) and public keys \\( P_i \\). If we ask Alice and Bob to each 
supply a nonce, we can try

\\[
\begin{align}
  P_{agg} &= P_a + P_b \\\\
  e &= H(R_a || R_b || P_a || P_b || m) \\\\
  s_{agg} &= r_a + r_b + (k_a + k_b)e \\\\
        &= (r_a + k_ae) + (r_b + k_ae) \\\\
        &= s_a + s_b
\end{align}
\\]

So it looks like Alice and Bob can supply their own _R_, and anyone can construct the 2-of-2 signature 
from the sum of the _R_s and public keys. This does work:

{{#playpen src/aggregation_1.rs}}

But this scheme is not secure!

# Key cancellation attack

Let's take the previous scenario again, but this time, Bob knows Alice's public key and nonce ahead of time, by
waiting until she reveals them. 

Now Bob lies, and says that his public key is \\( P_b' = P_b - P_a \\) and public nonce is \\( R_b' = R_b - R_a \\).

Note that Bob doesn't know the private keys for these faked values, but that doesn't matter.

Everyone assumes that \\(s_{agg} = R_a + R_b' + e(P_a + P_b') \\) as per the aggregation scheme.

But Bob can create this signature himself: 

\\[
\begin{align}
  s_{agg}G &= R_a + R_b' + e(P_a + P_b') \\\\
    &= R_a + (R_b - R_a) + e(P_a + P_b - P_a) \\\\
    &= R_b + eP_b \\\\
    &= r_bG + ek_bG \\\\
  \therefore s_{agg} &= r_b + ek_b = s_b
\end{align}
\\]

{{#playpen src/cancellation.rs}}

[WP1]: https://en.wikipedia.org/wiki/Schnorr_signature 'Wikipedia:Schnorr signature'