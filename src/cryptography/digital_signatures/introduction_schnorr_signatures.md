# Introduction to Schnorr Signatures

[toc levels=4 bullet]: # " "

- [Overview](#overview)
- [Let's get Started](#lets-get-started)
- [Basics of Schnorr Signatures](#basics-of-schnorr-signatures)
  - [Public and Private Keys](#public-and-private-keys)
  - [Creating a Signature](#creating-a-signature)
    - [Approach Taken](#approach-taken)
    - [Why do we Need the Nonce?](#why-do-we-need-the-nonce)
    - [ECDH](#ecdh)
- [Schnorr Signatures](#schnorr-signatures)
  - [So why all the Fuss?](#so-why-all-the-fuss)
  - [(Naïve) Signature Aggregation](#naïve-signature-aggregation)
  - [Key Cancellation Attack](#key-cancellation-attack)
  - [Better Approaches to Aggregation](#better-approaches-to-aggregation)
- [MuSig](#musig)
  - [MuSig Demonstration](#musig-demonstration)
  - [Security Demonstration](#security-demonstration)
  - [Replay Attacks!](#replay-attacks)
- [References](#references)
- [Contributors](#contributors)

## Overview

Private-public key pairs are the cornerstone of much of the
cryptographic security underlying everything from secure web browsing to banking to cryptocurrencies. Private-public key pairs
are _asymmetric_. This means that given one of the numbers (the private key), it's possible to derive the other one
(the public key). However, doing the reverse is not feasible.
It's this asymmetry that allows one to share the public key, uh, publicly and be confident that no one can
figure out our private key (which we keep very secret and secure).

Asymmetric key pairs are employed in two main applications:

- in _authentication_, where you prove that you have knowledge of the private
  key; and
- in _encryption_, where messages can be encoded and only the person possessing the private key can decrypt and read the message.

In this introduction to digital signatures, we'll be talking about a particular class of keys: those derived from
elliptic curves. There are other asymmetric schemes, not least of which are those based on products of prime numbers,
including RSA keys [[1]].

We're going to assume you know the basics of elliptic curve cryptography (ECC). If not, don't stress, there's a
[gentle introduction](../crypto-1/sources/PITCHME.link.md) in a previous chapter.

## Let's get Started

This is an interactive introduction to digital signatures. It uses Rust code to demonstrate some of
the ideas presented here, so you can see them at work. The code for this introduction uses the
[libsecp256k-rs](https://github.com/tari-labs/libsecp256k1) library.

That's a mouthful, but secp256k1 is the name of the elliptic curve that secures a _lot_ of things in many
cryptocurrencies' transactions, including Bitcoin.

This particular library has some nice features. We've overridden the `+` (addition) and `*` (multiplication)
operators so that the Rust code looks a lot more like mathematical formulae. This makes it much easier
to play with the ideas we'll be exploring.

**WARNING!** _Don't use this library in production code_. It hasn't been battle-hardened, so [use this one in
production instead](https://github.com/rust-bitcoin/rust-secp256k1).

## Basics of Schnorr Signatures

### Public and Private Keys

The first thing we'll do is create a public and private key from an elliptic curve.

On secp256k1, a private key is simply a scalar integer value between 0 and ~2<sup>256</sup>. That's roughly how many
atoms there are in the universe, so we have a big sandbox to play in.

We have a special point on the secp256k1 curve called _G_, which acts as the "origin". A public key is calculated by
adding _G_ on the curve to itself, \\( k_a \\) times. This is the definition of multiplication by a scalar, and is
written as:

$$
  P_a = k_a G
$$

Let's take an example from [this post](https://chuckbatson.wordpress.com/2014/11/26/secp256k1-test-vectors/), where
it is known that the public key for `1`, when written in uncompressed format, is `0479BE667...C47D08FFB10D4B8`.
The following code snippet demonstrates this:

{{#playground src/pubkey.rs}}

### Creating a Signature

#### Approach Taken

Reversing ECC math multiplication (i.e. division) is pretty much infeasible when using properly chosen random values for your scalars ([[5]],[[6]]).
This property is called the _Discrete Log Problem_, and is used as the principle behind many cryptography and digital signatures.
A valid digital signature is evidence that the person providing the signature knows the private key corresponding to the public key with which the message
is associated, or that they have solved the Discrete Log Problem.

The approach to creating signatures always follows this recipe:

1. Generate a secret once-off number (called a _nonce_), $r$.
2. Create a public key, $R$ from $r$ (where $R = r.G$).
3. Send the following to Bob, your recipient - your message ($m$), $R$, and your public key ($P = k.G$).

The actual signature is created by hashing the combination of all the public information above to create a _challenge_, $e$:

$$
    e = H(R || P || m)
$$

The hashing function is chosen so that _e_ has the same range as your private keys. In our case, we want something that
returns a 256-bit number, so SHA256 is a good choice.

Now the signature is constructed using your private information:

$$
    s = r + ke
$$

Bob can now also calculate $e$, since he already knows $m, R, P$. But he doesn't know your private key, or nonce.

**Note:** When you construct the signature like this, it's known as a [Schnorr signature](#schnorr-signatures), which is discussed in
a following section. There are other ways of constructing $s$, such as ECDSA [[2]], which is used in Bitcoin.

But see this:

$$ sG = (r + ke)G $$

Multiply out the right-hand side:

$$ sG = rG + (kG)e ​$$

Substitute \\(R = rG \\) and \\(P = kG \\) and we have:
$$ sG = R + Pe ​$$

So Bob must just calculate the public key corresponding to the signature $\text{(}s.G\text{)}$ and check that it equals the right-hand side of the last
equation above $\text{(}R + P.e\text{)}$, all of which Bob already knows.

#### Why do we Need the Nonce?

Why do we need a nonce in the standard signature?

Let's say we naïvely sign a message $m$ with

$$
e = H(P || m)
$$

and then the signature would be \\(s = ek \\).

Now as before, we can check that the signature is valid:

$$
\begin{align}
  sG &= ekG \\\\
     &= e(kG) = eP
\end{align}
$$

So far so good. But anyone can read your private key now because $s$ is a scalar, so \\(k = {s}/{e} \\)
is not hard to do.
With the nonce you have to solve \\( k = (s - r)/e \\), but $r$ is unknown, so this is not a feasible calculation as long
as $r$ has been chosen randomly.

We can show that leaving off the nonce is indeed highly insecure:

{{#playground src/no-nonce.rs}}

#### ECDH

How do parties that want to communicate securely generate a shared secret for encrypting messages? One way is called
the Elliptic Curve Diffie-Hellman exchange (ECDH), which is a simple method for doing just this.

ECDH is used in many places, including the Lightning Network during channel negotiation [[3]].

Here's how it works. Alice and Bob want to communicate securely. A simple way to do this is to use each other's public keys and
calculate

$$
\begin{align}
  S_a &= k_a P_b \tag{Alice} \\\\
  S_b &= k_b P_a \tag{Bob} \\\\
  \implies S_a = k_a k_b G &\equiv S_b = k_b k_a G
\end{align}
$$

{{#playground src/ecdh.rs}}

For security reasons, the private keys are usually chosen at random for each session (you'll see the term
_ephemeral_ keys being used), but then we have the problem of not being sure the other party is who they say they
are (perhaps due to a man-in-the-middle attack [[4]]).

Various additional authentication steps can be employed to resolve this problem, which we won't get into here.

## Schnorr Signatures

If you follow the crypto news, you'll know that that the new hotness in Bitcoin is Schnorr Signatures.

<p align="center"><img src="./img/schnorr-meme.jpg" width="600" /></p>

But in fact, they're old news! The Schnorr signature is considered the simplest digital signature scheme
to be provably secure in a random oracle model. It is efficient and generates short signatures.
It was covered by U.S. Patent 4,995,082, which expired in February&nbsp;2008 [[7]].

### So why all the Fuss?

What makes Schnorr signatures so interesting and [potentially dangerous](#key-cancellation-attack), is their simplicity.
Schnorr signatures are _linear_, so you have some nice properties.

Elliptic curves have the multiplicative property. So if you have two scalars $x, y$ with corresponding points $X, Y$,
the following holds:

$$
  (x + y)G = xG + yG = X + Y
$$

Schnorr signatures are of the form \\( s = r + e.k \\). This construction is linear too, so it fits nicely with
the linearity of elliptic curve math.

You saw this property in a [previous section](#creating-a-signature), when we were verifying the signature. Schnorr signatures' linearity
makes it very attractive for, among others:

- signature aggregation;
- [atomic swaps](../../protocols/atomic-swaps/AtomicSwaps.md);
- ["scriptless" scripts](../scriptless-scripts/introduction-to-scriptless-scripts.md).

### Naïve Signature Aggregation

Let's see how the linearity property of Schnorr signatures can be used to construct a two-of-two multi-signature.

Alice and Bob want to cosign something (a Tari transaction, say) without having to trust each other;
i.e. they need to be able to prove ownership of their respective keys, and the aggregate signature is
only valid if _both_ Alice and Bob provide their part of the signature.

Assuming private keys are denoted \\( k_i \\) and public keys \\( P_i \\). If we ask Alice and Bob to each
supply a nonce, we can try:

$$
\begin{align}
  P_{agg} &= P_a + P_b \\\\
  e &= H(R_a || R_b || P_a || P_b || m) \\\\
  s_{agg} &= r_a + r_b + (k_a + k_b)e \\\\
        &= (r_a + k_ae) + (r_b + k_ae) \\\\
        &= s_a + s_b
\end{align}
$$

So it looks like Alice and Bob can supply their own $R$, and anyone can construct the two-of-two signature
from the sum of the $Rs$ and public keys. This does work:

{{#playground src/aggregation_1.rs}}

But this scheme is not secure!

### Key Cancellation Attack

Let's take the previous scenario again, but this time, Bob knows Alice's public key and nonce ahead of time, by
waiting until she reveals them.

Now Bob lies and says that his public key is \\( P_b' = P_b - P_a \\) and public nonce is \\( R_b' = R_b - R_a \\).

Note that Bob doesn't know the private keys for these faked values, but that doesn't matter.

Everyone assumes that \\(s\_{agg} = R_a + R_b' + e(P_a + P_b') \\) as per the aggregation scheme.

But Bob can create this signature himself:

$$
\begin{align}
  s_{agg}G &= R_a + R_b' + e(P_a + P_b') \\\\
    &= R_a + (R_b - R_a) + e(P_a + P_b - P_a) \\\\
    &= R_b + eP_b \\\\
    &= r_bG + ek_bG \\\\
  \therefore s_{agg} &= r_b + ek_b = s_b
\end{align}
$$

{{#playground src/cancellation.rs}}

### Better Approaches to Aggregation

In the [Key Cancellation Attack](#key-cancellation-attack), Bob didn't know the private keys for his published $R$ and $P$ values. We could defeat Bob
by asking him to sign a message proving that he _does_ know the private keys.

This works, but it requires another round of messaging between parties, which is not conducive to a great user experience.

A better approach would be one that incorporates one or more of the following features:

- It must be provably secure in the plain public-key model, without having to prove knowledge of secret keys, as we might have asked Bob to do in the [naïve](#naïve-signature-aggregation) approach.
- It should satisfy the normal Schnorr equation, i.e. the resulting signature can be verified with an expression of the form \\( R + e X \\).
- It allows for Interactive Aggregate Signatures (IAS), where the signers are required to cooperate.
- It allows for Non-interactive Aggregate Signatures (NAS), where the aggregation can be done by anyone.
- It allows each signer to sign the same message, $m$.
- It allows each signer to sign their own message, \\( m_i \\).

## MuSig

MuSig is a recently proposed ([[8]],[[9]]) simple signature aggregation scheme that satisfies all of the properties in the preceding section.

### MuSig Demonstration

We'll demonstrate the interactive MuSig scheme here, where each signatory signs the same message.
The scheme works as follows:

1. Each signer has a public-private key pair, as before.
2. Each signer shares a commitment to their public nonce (we'll skip this step in this demonstration). This step is
   necessary to prevent certain kinds of rogue key attacks [[10]].
3. Each signer publishes the public key of their nonce, \\( R_i \\).
4. Everyone calculates the same "shared public key", $X$ as follows:

$$
    \begin{align}
        \ell &= H(X_1 || \dots || X_n) \\\\
        a_i &= H(\ell || X_i) \\\\
        X &= \sum a_i X_i \\\\
    \end{align}
$$

Note that in the preceding ordering of public keys, some deterministic convention should be used, such as the lexicographical
order of the serialized keys.

1. Everyone also calculates the shared nonce, \\( R = \sum R_i \\).
2. The challenge, $e$ is \\( H(R || X || m) \\).
3. Each signer provides their contribution to the signature as:

$$
    s_i = r_i + k_i a_i e
$$

Notice that the only departure here from a standard Schnorr signature is the inclusion of the factor \\( a_i \\).

The aggregate signature is the usual summation, \\( s = \sum s_i \\).

Verification is done by confirming that as usual:

$$
  sG \equiv R + e X \\
$$

Proof:

$$
\begin{align}
  sG &= \sum s_i G \\\\
     &= \sum (r_i + k_i a_i e)G \\\\
     &= \sum r_iG + k_iG a_i e \\\\
     &= \sum R_i + X_i a_i e \\\\
     &= \sum R_i + e \sum a_i X_i \\\\
     &= R + e X \\\\
     \blacksquare
\end{align}
$$

Let's demonstrate this using a three-of-three multisig:

{{#playground src/musig.rs}}

### Security Demonstration

As a final demonstration, let's show how MuSig defeats the cancellation attack from the [naïve](#naïve-signature-aggregation) signature scheme.
Using the same idea as in the [Key Cancellation Attack](#key-cancellation-attack) section, Bob has provided fake values for his
nonce and public keys:

$$
\begin{align}
  R_f &= R_b - R_a \\\\
  X_f &= X_b - X_a \\\\
\end{align}
$$

This leads to both Alice and Bob calculating the following "shared" values:

$$
\begin{align}
  \ell &= H(X_a || X_f) \\\\
  a_a &= H(\ell || X_a) \\\\
  a_f &= H(\ell || X_f) \\\\
  X &= a_a X_a + a_f X_f \\\\
  R &= R_a + R_f (= R_b) \\\\
  e &= H(R || X || m)
\end{align}
$$

Bob then tries to construct a unilateral signature following MuSig:

$$
  s_b = r_b + k_s e
$$

Let's assume for now that \\( k_s \\) doesn't need to be Bob's private key, but that he can derive it using information
he knows. For this to be a valid signature, it must verify to \\( R + eX \\). So therefore:

$$
\begin{align}
  s_b G          &= R + eX \\\\
  (r_b + k_s e)G &= R_b + e(a_a X_a + a_f X_f) & \text{The first term looks good so far}\\\\
                 &= R_b + e(a_a X_a + a_f X_b - a_f X_a) \\\\
                 &= (r_b + e a_a k_a + e a_f k_b - e a_f k_a)G & \text{The r terms cancel as before} \\\\
  k_s e &=  e a_a k_a + e a_f k_b - e a_f k_a & \text{But nothing else is going away}\\\\
  k_s &= a_a k_a + a_f k_b - a_f k_a \\\\
\end{align}
$$

In the previous attack, Bob had all the information he needed on the right-hand side of the analogous calculation. In MuSig,
Bob must somehow know Alice's private key and the faked private key (the terms don't cancel anymore) in order to create a unilateral signature,
and so his cancellation attack is defeated.

### Replay Attacks!

It's critical that a new nonce be chosen for every signing ceremony. The best way to do this is to make use of a
cryptographically secure (pseudo-)random number generator (CSPRNG).

But even if this is the case, let's say an attacker can trick us into signing a new message by "rewinding" the signing
ceremony to the point where partial signatures are generated. At this point, the attacker provides a different message,
\\( e' = H(...||m') \\) to sign. Not suspecting any foul play, each party calculates their partial signature:

$$ s'\_i = r_i + a_i k_i e' $$
However, the attacker still has access to the first set of signatures: \\( s_i = r_i + a_i k_i e \\). He now simply
subtracts them:

$$
  \begin{align}
  s'_i - s_i &= (r_i + a_i k_i e') - (r_i + a_i k_i e) \\\\
             &= a_i k_i (e' - e) \\\\
  \therefore k_i &= \frac{s'_i - s_i}{a_i(e' - e)}
  \end{align}
$$

Everything on the right-hand side of the final equation is known by the attacker and thus he can trivially extract everybody's private key.
It's difficult to protect against this kind of attack. One way to is make it difficult (or impossible) to stop and
restart signing ceremonies. If a multi-sig ceremony gets interrupted, then you need to start from step one again. This
is fairly unergonomic, but until a more robust solution comes along, it may be the best we have!

## References

[[1]] Wikipedia: "RSA (Cryptosystem)" \[online\]. Available: <https://en.wikipedia.org/wiki/RSA_(cryptosystem)>. Date accessed:
2018&#8209;10&#8209;11.

[1]: https://en.wikipedia.org/wiki/RSA_(cryptosystem) "Wikipedia RSA Cryptography"

[[2]] Wikipedia: "Elliptic Curve Digital Signature Algorithm" \[online\]. Available:
<https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm>. Date accessed: 2018&#8209;10&#8209;11.

[2]: https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm "Wikipedia: ECDSA"

[[3]] Github: "BOLT #8: Encrypted and Authenticated Transport, Lightning RFC" \[online\].
Available: <https://github.com/lightningnetwork/lightning-rfc/blob/master/08-transport.md>. Date accessed: 2018&#8209;10&#8209;11.

[3]: https://github.com/lightningnetwork/lightning-rfc/blob/master/08-transport.md "BOLT #8: Encrypted and Authenticated Transport"

[[4]] Wikipedia: "Man in the Middle Attack" \[online\].
Available: <https://en.wikipedia.org/wiki/Man-in-the-middle_attack>. Date accessed: 2018&#8209;10&#8209;11.

[4]: https://en.wikipedia.org/wiki/Man-in-the-middle_attack "Wikipedia: Man in the Middle Attack"

[[5]] StackOverflow: "How does a Cryptographically Secure Random Number Generator Work?" [online\]. Available:
<https://stackoverflow.com/questions/2449594/how-does-a-cryptographically-secure-random-number-generator-work>.
Date accessed: 2018&#8209;10&#8209;11.

[5]: https://stackoverflow.com/questions/2449594/how-does-a-cryptographically-secure-random-number-generator-work "StackOverflow: How does a Cryptographically Secure Random Number Generator Work?"

[[6]] Wikipedia: "Cryptographically Secure Pseudorandom Number Generator" \[online\]. Available:
<https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator>. Date accessed: 2018&#8209;10&#8209;11.

[6]: https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator "Cryptographically Secure Pseudorandom Number Generator"

[[7]] Wikipedia: "Schnorr Signature" \[online\]. Available: <https://en.wikipedia.org/wiki/Schnorr_signature>.
Date accessed: 2018&#8209;09&#8209;19.

[7]: https://en.wikipedia.org/wiki/Schnorr_signature "Wikipedia: Schnorr Signature"

[[8]] Blockstream: "Key Aggregation for Schnorr Signatures" \[online\]. Available:
<https://blockstream.com/2018/01/23/musig-key-aggregation-schnorr-signatures.html>. Date accessed: 2018&#8209;09&#8209;19.

[8]: https://blockstream.com/2018/01/23/musig-key-aggregation-schnorr-signatures.html "Blockstream: Key Aggregation for Schnorr Signatures"

[[9]] G. Maxwell, A. Poelstra, Y. Seurin and P. Wuille, "Simple Schnorr Multi-signatures with Applications to Bitcoin" \[online\]. Available: <https://eprint.iacr.org/2018/068.pdf>. Date accessed: 2018&#8209;09&#8209;19.

[9]: https://eprint.iacr.org/2018/068.pdf "Simple Schnorr Multi-signatures with Applications to Bitcoin"

[[10]] M. Drijvers, K. Edalatnejad, B. Ford, E. Kiltz, J. Loss, G. Neven and I. Stepanovs,
"On the Security of Two-round Multi-signatures", Cryptology ePrint Archive, Report 2018/417 \[online\].
Available: <https://eprint.iacr.org/2018/417.pdf>. Date accessed: 2019&#8209;02&#8209;21.

[10]: https://eprint.iacr.org/2018/417.pdf "On the Security of Two-round Multi-signatures"

## Contributors

- <https://github.com/CjS77>
- <https://github.com/SWvHeerden>
- <https://github.com/hansieodendaal>
- <https://github.com/neonknight64>
- <https://github.com/anselld>
