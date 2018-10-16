# Introduction

Private-public key pairs are the cornerstone of much of the
cryptographic security underlying everything from secure web browsing to banking to cryptocurrencies. Private-public key pairs
are _asymmetric_. This means that given one of the numbers (the private key), it's possible to derive the other one 
(the public key). However, doing the reverse is not feasible. 
It's this asymmetry that allows one to share the public key, uh, publicly and be confident that no-one can
figure out our private key (which we keep very secret and secure).

Asymmetric key pairs are employed in two main applications: In _authentication_, where you prove that you have knowledge of the private
key; and _encryption_ where messages can be encoded and only the person possessing the private key can decrypt and read the message.

In this introduction on digital signatures, we'll be talking about a particular class of keys: those derived from 
elliptic curves. There are other asymmetric schemes, not least of which those based on products of prime numbers, 
including RSA keys [[1]].

We're going to assume you know the basics of elliptic curve cryptography (ECC). If not, don't stress, there's a
[gentle introduction](../crypto-1/sources/PITCHME.link.md) in a previous chapter.

# Let's get started

This is an interactive introduction to digital signatures. It uses Rust code to demonstrate some of 
the idea presented here, so you can see them at work. The code for this introduction makes use 
of the [libsecp256k-rs](https://github.com/tari-labs/libsecp256k1) library. 

That's a mouthful, but secp256k1 is the name of the elliptic curve that secures a *lot* of things in many
cryptocurrencies' transactions, including Bitcoin. 

This particular library has some nice features. We've overridden the `+` (addition) and `*` (multiplication)
operators so that the Rust code looks a lot more like mathematical formulae. This makes it much easier
to play with the ideas we'll be exploring.

**WARNING!** _Don't use this library in production code_. It hasn't been battle-hardened, so [use this one in
production instead](https://github.com/rust-bitcoin/rust-secp256k1)

# References

[1]: https://en.wikipedia.org/wiki/RSA_(cryptosystem) 'Wikipedia RSA cryptography'

[[1]]: RSA (cryptosystem). https://en.wikipedia.org/wiki/RSA_(cryptosystem). Accessed 11 Oct 2018.
