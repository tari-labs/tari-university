
## Public and Private keys

The first thing we'll do is create a public and private key.

On secp256k1, a private key is simply a scalar integer value between 0 and ~2<sup>256</sup>. That's roughly how many
atoms there are in the universe, so we have a big sandbox to play in.

We have a special point on the secp256k1 curve called _G_, that acts as the 'origin'. A public key is calculated
from the private key by multiplying it by _G_:

$$
  P_a = k_a G
$$

Let's take an example from [this post](https://chuckbatson.wordpress.com/2014/11/26/secp256k1-test-vectors/), where
it is known that the public key for `1`, when written in uncompressed format is `0479BE667...C47D08FFB10D4B8`.


{{#playpen src/pubkey.rs}}

# Creating a signature

Reversing ECC math multiplication (i.e. division) is pretty much infeasible when using properly random values for your scalars.
 This property is called the _Discrete Log Problem_ and is used as the principle behind a lot of cryptography and digital signatures. 
 A valid digital signature is evidence that the person providing the signature knows the private key corresponding to the public key the message
is associated with, or that they have solved the Discrete Log Problem. 

The approach to creating signatures always follows this recipe:

1. Generate a secret once-off number, _r_
1. Create a public key, _R_ from _r_ (_R = r.G_)
1. Send the following to Bob, your recipient: your message (_m_), _R_, and your public key (_P = k.G_)

The actual signature is created by hashing the combination of all the public information above to create a _challenge_, e:

$$
    e = H(R || P || m)
$$

The hashing function is chosen so that _e_ has the same range as your private keys. In our case, SHA256 is a good choice.

Now the signature is constructed using your private information:

$$
    s = r + ke 
$$

Now Bob can also calculate _e_, since he already knows _m, R, P_. But he doesn't know your private key, or nonce.
_Note:_ When you construct the signature like this, it's known as a _Schnorr signature_, which we'll discuss in more 
detail in the next section. There are other ways of constructing _s_, such as ECDSA [[1]], which is used in Bitcoin.

But see this:

$$
    \begin{align}
      sG &= (r + ke)G \\\\
      sG &= rG + (kG)e \\\\
      sG &= R + Pe
    \end{align}
$$

So Bob, must just calculate the public key corresponding to the signature and check that it equals the RHS of the last
equation above (R + Pe), all of which Bob knows WOOOHOO.

## Why do we need the nonce?

Why do we need a nonce in the standard signature?

Let's say we naïvely sign a message m with

$$
    e = H(R || m)
$$

and then the signature would be \\(s = ek \\). 

Now as before, we can check that the signature is valid:

$$
\begin{align}
  s &= ek \\\\
 \therefore sG &= (ek)G = e(kG) = eP
\end{align}
$$

So far so good. But anyone can read your private key now because s is a scalar, so \\(k = \frac{s}{e} \\)
 is not hard to do.
With the nonce you have to solve \\( k = (s - r)/e \\), but r is unknown, so this is not a feasible calculation as long
 as _r_ has been chosen randomly.
 
We can show that leaving off the nonce is indeed highly insecure:

{{#playpen src/no-nonce.rs}}

# ECDH

How do parties that want to communicate securely generate a shared secret for encrypting messages? One way is called
the Elliptic Curve Diffie-Hellmam exchange (ECDH) which is a simple method for doing just this.

ECDH is used in many places, including the Lightning Network during channel negotiation [[2]].

Here's how it works. Alice and Bob want to communicate securely. A simple way to do this is to use each other's public keys and
calculate

$$
\begin{align}
  S_a &= k_a P_b \tag{Alice} \\\\
  S_b &= k_b P_a \tag{Bob} \\\\
  \implies S_a = k_a k_b G &\equiv S_b = k_b k_a G
\end{align}
$$

{{#playpen src/ecdh.rs}}

For security reasons, the private keys are usually chosen at random for each session (you'll see the term
_ephemeral_ keys being used), but then we have the problem of not being sure the other party is who they say they
are (perhaps due to a MITM attack [[3]]).

Various additional authentication steps can be employed to resolve this problem, which we won't get into here. 

[1]: https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm 'Wikipedia: ECDSA'
[2]: https://github.com/lightningnetwork/lightning-rfc/blob/master/08-transport.md 'BOLT #8: Encrypted and Authenticated Transport'
[3]: https://en.wikipedia.org/wiki/Man-in-the-middle_attack 'Wikipedia: Man in the Middle Attack'
# References

* [[1]] Elliptic Curve Digital Signature Algorithm, Wikipedia, https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm.
* [[2]] BOLT #8: Encrypted and Authenticated Transport, Lightning RFC, Github. https://github.com/lightningnetwork/lightning-rfc/blob/master/08-transport.md
* [[3]] Man in the Middle Attack, Wikipedia, https://en.wikipedia.org/wiki/Man-in-the-middle_attack