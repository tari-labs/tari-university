# Introduction

This is an interactive introduction to digital signatures. It uses Rust code to demonstrate some of 
the idea presented here, so you can see them at work. The code for this introduction makes use 
of the [libsecp256k-rs](https://github.com/tari-labs/libsecp256k1) library. 

That's a mouthful, but secp256k1 is the name of the elliptic curve that secures a *lot* of things in many
cryptocurrencies' transactions, including Bitcoin. 

This particular library has some nice features. I've overridden the `+` (addition) and `*` (multiplication)
operators so that the Rust code looks a lot more like mathematical formulae. This makes it much easier
to play with the ideas we'll be exploring.

**WARNING!** _Don't use this library in production code_. It hasn't been battle-hardened, so [use this one in
production instead](https://github.com/rust-bitcoin/rust-secp256k1)

# Let's get started

I'm going to assume you know the basics of elliptic curve cryptography (ECC). If not, don't stress, there's a
[gentle introduction](https://gitpitch.com/tari-labs/tari-university/master?p=/cryptography/crypto-1#/) 
on Tari Labs University archives.