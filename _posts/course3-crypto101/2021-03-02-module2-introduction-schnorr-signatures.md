---
layout: module
title:  Introduction to Schnorr Signatures
date:   2021-03-02 15:01:35 +0300
postid: cr02
format: article
level: intermediate
redirect_from: /cryptography/digital_signatures/introduction_schnorr_signatures.html
image:  '/images/banner-01.jpg'
course: cryptography-101
category: cryptography-101
time: 15
tags:   [cryptography]
featured:
description: Private-public key pairs are the cornerstone of much of the cryptographic security underlying everything from secure web browsing to banking to cryptocurrencies.
selftest:
  - title: A Schnorr signature requires two pieces of secret information. What are they?
    content: The private key and the nonce.
  - title: What is the difference between a nonce and a private key?
    content: A nonce is used only once, whereas a private key is used many times.
  - title: What does _nonce_ mean?
    content: A nonce is a **n**umber used only **once**.
  - title: Why is re-using a nonce a bad idea?
    content: Because it can _trivially_ lead to the private key being exposed. To see this, create two signatures with
      the same nonce, but different messages. Then subtract the two signatures from each other. The result will be a
      multiple of the private key, which can then easily be calculated.
  - title: What does it mean when we say that ECC keys are homomorphic?
    content: It means that adding or multiplying two secret values together, and performing the same operation with
      their public values, will result in a secret value that is the same as if the operation had been performed on the
      two secret values.
  - title: Why can Schnorr signatures be aggregated?
    content: Schnorr signatures are _linear_ in their terms (_r + ek_). Since, ECC keys are homomorphic, it is possible
      to add the two signatures together, and the result will be a valid signature for the sum of the two messages.

---

{% include content/cryptography/02-introduction-schnorr-signatures.md %}
