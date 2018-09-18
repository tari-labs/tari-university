# Introduction

This is an interactive introduction to digital signatures. It uses Rust code to demonstrate some of 
the idea presented here, so you can see them at work. The code for this introduction makes use 
of the [libsecp256k-rs](https://github.com/tari-labs/libsecp256k1) library. 

That's a mouthful, but secp256k1 is the name of the elliptic curve that secure a *lot* of Bitcoin, and
other cryptocurrencies' transactions. 

This particular library has some nice features. I've overridden the `+` (addition) and `*` (multiplication)
operators so that the Rust code looks a lot more like mathematical formulae. This makes it much easier
to play with the ideas we'll be exploring.

**WARNING!** _Don't use this library in production code_. It hasn't been battle-hardened, so [use this one in
production instead](https://github.com/rust-bitcoin/rust-secp256k1)

# Let's get started

I'm going to assume you know the basics of elliptic curve cryptography (ECC). If not, don't stress, there's a
[gentle introduction](https://gitpitch.com/tari-labs/tari-university/master?p=/cryptography/crypto-1#/) 
on Tari Labs University archives.

## Public and Private keys

The first thing we'll do is create a public and private key.

On secp256k1, a private key is simply a scalar integer value between 0 and ~2<sup>256</sup>. That's roughly how many
atoms there are in the universe, so we have a big sandbox to play in.

We have a special point on the secp256k1 curve called _G_, that acts as the 'origin'. A public key is calculated
from the private key by multiplying it by _G_:

Let's take an example [this post](https://chuckbatson.wordpress.com/2014/11/26/secp256k1-test-vectors/).

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

But see this:

\\[
    \begin{align}
      sG &= (r + ke)G \\\\
      sG &= rG + (kG)e \\\\
      sG &= R + Pe
    \end{align}
\\]

So Bob, must just calculate the Public key sorresponding to the signature and check that it equals the RHS of the last
equation above (R + Pe), all of which Bob knows.



{{#playpen src/signature.rs}}