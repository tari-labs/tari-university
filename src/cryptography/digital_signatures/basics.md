
## Public and Private keys

The first thing we'll do is create a public and private key.

On secp256k1, a private key is simply a scalar integer value between 0 and ~2<sup>256</sup>. That's roughly how many
atoms there are in the universe, so we have a big sandbox to play in.

We have a special point on the secp256k1 curve called _G_, that acts as the 'origin'. A public key is calculated
from the private key by multiplying it by _G_:

Let's take an example from [this post](https://chuckbatson.wordpress.com/2014/11/26/secp256k1-test-vectors/).

\\[
  P_a = k_a G
\\]

{{#playpen src/pubkey.rs}}

# Creating a signature

ECC math multiplication cannot be undone easily. This fact is used to create digital signatures. A valid digital signature is
a _proof_ that the person providing the signature knows the private key corresponding to the public key the message
is associated with.

The approach to creating signatures always follows this recipe:

1. Generate a secret once-off number, _r_
1. Create a public key, _R_ from _r_ (_R = r.G_)
1. Send the following to Bob, your recipient: your message (_m_), _R_, and your public key (_P = k.G_)

The actual signature is created by hashing the combination of all the public information above to create a _challenge_, e:

\\[
    e = H(R || P || m)
\\]

The hashing function is chosen so that _e_ has the same range as your private keys. In our case, SHA256 is a good choice.

Now the signature is constructed using your private information:

\\[
    s = r + ke 
\\]

Now Bob can also calculate _e_, since he already knows _m, R, P_. But he doesn't know your private key, or nonce.
_Note:_ When you construct the signature like this, it's known as a _Schnorr signature_, which we'll discuss in more 
detail in the next section. There are other ways of constructing _s_, such as ECDSA [WP1], which is used in Bitcoin.

But see this:

\\[
    \begin{align}
      sG &= (r + ke)G \\\\
      sG &= rG + (kG)e \\\\
      sG &= R + Pe
    \end{align}
\\]

So Bob, must just calculate the Public key corresponding to the signature and check that it equals the RHS of the last
equation above (R + Pe), all of which Bob knows.

## Why do we need the nonce?

Why do we need a nonce in the standard signature?

Let's say we naïvely sign a message m with

\\[
e = H(R || m)
\\]

and then the signature would be \\(s = ek \\). 

Now as before, we can check that the signature is valid:

\\[
\begin{align}
  s &= ek \\\\
 \therefore sG &= (ek)G = e(kG) = eP
\end{align}
\\]

So far so good. But anyone can read your private key now because s is a scalar, so \\(k = e/s\\)
 is not hard to do.
With the nonce you have to solve \\( k = (s - r)/e \\), but r is unknown

{{#playpen src/no-nonce.rs}}

# Secure key exchange

We can descrive the Elliptic Curve Diffie-Hellmam exchange (ECDH) which is a simple way for two parties
to generate a shared secret.

Alice and Bob want to communicate securely. A simple way to do this is to use each other's public keys and
calculate

\\[
\begin{align}
  S_a &= k_a Pb \tag{Alice} \\\\
  S_b &= k_b Pa \tag{Bob} \\\\
  \implies S_a = k_a k_b G &\equiv S_b = k_b k_a G
\end{align}
\\]

For security reasons, the private keys are usually chosen at random for each session (you'll see the term
_ephemeral_ keys being used), but then we have the problem of not being sure the other party is who they say they
are (perhaps due to a [MITM attack](https://en.wikipedia.org/wiki/Man-in-the-middle_attack)).

Various additional authentication steps can be employed to resolve this problem, which we won't get into here. 

[WP1]: https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm 'Wikipedia: ECDSA'