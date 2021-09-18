---
layout: post
title:  Introduction to Scriptless Scripts
date:   2020-02-03 15:01:35 +0300
image:  '/images/banner-01.jpg'
category: cryptography
tags:   [cryptography]
featured:
excerpttext: Scriptless Scripts are a means to execute smart contracts off-chain, through the use of Schnorr signatures.
---

## Table of Contents

- [Definition of Scriptless Scripts](#definition-of-scriptless-scripts)
- [Benefits of Scriptless Scripts](#benefits-of-scriptless-scripts)
- [List of Scriptless Scripts](#list-of-scriptless-scripts)
- [Role of Schnorr Signatures](#role-of-schnorr-signatures)
- [Schnorr Multi-signatures](#schnorr-multi-signatures)
- [Adaptor Signatures](#adaptor-signatures)
- [Simultaneous Scriptless Scripts](#simultaneous-scriptless-scripts)
  - [Preimages](#preimages)
  - [Difference of Two Schnorr Signatures](#difference-of-two-schnorr-signatures)
- [Atomic (Cross-chain Swaps) Example with Adaptor Signatures](#atomic-cross-chain-swaps-example-with-adaptor-signatures)
- [Zero Knowledge Contingent Payments](#zero-knowledge-contingent-payments)
- [Mimblewimble's Core Scriptless Script](#mimblewimbles-core-scriptless-script)
- [References](#references)
- [Contributors](#contributors)

## Definition of Scriptless Scripts

Scriptless Scripts are a means to execute smart contracts off-chain, through the use of Schnorr signatures [[1]].  

The concept of Scriptless Scripts was born from Mimblewimble, which is a blockchain design that with the exception of
kernels and their signatures, does not store permanent data. Fundamental properties of Mimblewimble
include both privacy and scaling, both of which require the implementation of Scriptless Scripts [[2]].

A brief introduction is also given in
[Scriptless Scripts, Layer 2 Scaling Survey](/scaling/layer2scaling-survey#a5-scriptless-scripts).


## Benefits of Scriptless Scripts

The benefits of Scriptless Scripts are functionality, privacy and efficiency.


### Functionality

With regard to functionality, Scriptless Scripts are said to increase the range and complexity of smart contracts.
Currently, as within Bitcoin Script, limitations stem from the number of ```OP_CODES``` that have been enabled by the
network. Scriptless Scripts move the specification and execution of smart contracts from the network to a discussion
that only involves the participants of the smart contract.

### Privacy

With regard to privacy, moving the specification and execution of smart contracts from on-chain to off-chain increases
privacy. When on-chain, many details of the smart contract are shared to the entire network. These details include the
number and addresses of participants, and the amounts transferred. By moving smart contracts off-chain, the network only
knows that the participants agree that the terms of their contract have been satisfied and that the transaction in
question is valid.

### Efficiency

With regard to efficiency, Scriptless Scripts minimize the amount of data that requires verification and storage
on-chain. By moving smart contracts off-chain, there are fewer overheads for full nodes and lower transaction fees for
users [[1]].   



## List of Scriptless Scripts

In this report, various forms of scripts will be covered, including [[3]]:
- Simultaneous Scriptless Scripts
- Adaptor Signatures
- Zero Knowledge Contingent Payments


## Role of Schnorr Signatures

To begin with, the fundamentals of Schnorr signatures must be defined. The signer has a private key $x$ and random private
nonce $r$. $G$ is the generator of a discrete log hard group, $R=rG$ is the public nonce and $P=xG$ is the public key [[4]].

The signature, $s$, for message $m$,  can then be computed as a simple linear transaction

$$
s = r + e \cdot x
$$

where

$$
e=H(P||R||m) \text{, } P=xG
$$

The position on the line chosen is taken as the hash of all the data that one needs to commit to, the digital signature.
The verification equation involves the multiplication of each of the terms in the equation by $G$ and takes into account
the cryptographic assumption (discrete log) where $G$ can be multiplied in but not divided out, thus preventing
deciphering.

$$
sG = rG + e \cdot xG
$$

Elliptic Curve Digital Signature Algorithm (ECDSA) signatures (used in Bitcoin) are not linear in $x$ and $r$, and are
thus less useful [[2]].



## Schnorr Multi-signatures

A multi-signature (multisig) has multiple participants that produce a signature. Every participant might produce a
separate signature and concatenate them, forming a multisig.

With Schnorr Signatures, one can have a single public key, which is the sum of many different people's public keys. The
resulting key is one against which signatures will be verifiable [[5]].

The formulation of a multisig involves taking the sum of all components; thus all nonces and $s$ values result in the
formulation of a multisig [[4]]:
$$
s=\sum s(i)
$$
It can therefore be seen that these signatures are essentially Scriptless Scripts. Independent public keys of several
participants are joint to form a single key and signature, which, when published, do not divulge details of the
number of participants involved or the original public keys.



## Adaptor Signatures  

This multisig protocol can be modified to produce an adaptor signature, which serves as the building block for all
Scriptless Script functions [[5]].

Instead of functioning as a full valid signature on a message with a key, an adaptor signature is a promise that a
signature agreed to be published, will reveal a secret.

This concept is similar to that of atomic swaps. However, no scripts are implemented. Since this is elliptic curve
cryptography, there is only scalar multiplication of elliptic curve points. Fortunately, similar to a hash function,
elliptic curves function in one way, so an elliptic curve point ($T$), can simply be shared and the secret will be its
corresponding private key.  

If two parties are considered, rather than providing their nonce $R$ in the multisig protocol, a blinding factor, taken
as an elliptic curve point $T$, is conceived and sent in addition to $R$ (i.e. $R+T$). It can therefore be seen that
$R$ is not
blinded; it has instead been offset by the secret value $T$.

Here, the Schnorr multisig construction is modified such that the first party generates
$$
T=tG, R=rG
$$
where $t$ is the shared secret, $G$ is the generator of discrete log hard group and $r$ is the random nonce.

Using this information, the second party generates
$$
H(P||R+T||m)x
$$
where the coins to be swapped are contained within message $m$. The first party can now calculate the complete
signature $s$ such that  
$$
s=r+t+H(P||R+T||m)x
$$
The first party then calculates and publishes the adaptor signature $s'$ to the second party (and anyone else listening)
$$
s'=s-t
$$
The second party can verify the adaptor signature $s'$ by asserting $s'G$
$$
s'G \overset{?}{=} R+H(P||R+T||m)P
$$
However, this is not a valid signature, as the hashed nonce point is $R+T$ and not $R$.

The second party cannot retrieve a valid signature from this and requires ECDLP solving to recover $s'+t$, which is
virtually impossible.

After the first party broadcasts $s$ to claim the coins within message $m$, the second party can calculate the
secret $t$ from
$$
t=s-s'
$$
The above is very general. However, by attaching auxiliary proofs too, an adaptor signature can be derived that will
allow the translation of correct movement of the auxiliary protocol into a valid signature.



## Simultaneous Scriptless Scripts

### Preimages

The execution of separate transactions in an atomic fashion is achieved through preimages. If two transactions require
the preimage to the same hash, once one is executed, the preimage is exposed so that the other one can be as well.
Atomic swaps and Lightning channels use this construction [[4]].

### Difference of Two Schnorr Signatures

If we consider the difference of two Schnorr signatures:
$$
d=s-s'=k-k'+e \cdot x-e' \cdot x'
$$
The above equation can be verified in a similar manner to that of a single Schnorr signature, by multiplying each term
by $G$ and confirming algebraic correctness:
$$
dG=kG-k'G+e \cdot xG-e' \cdot x'G
$$
It must be noted that the difference $d$ is being verified, and not the Schnorr signature itself. $d$ functions as the
translating key between two separate independent Schnorr signatures. Given $d$ and either $s$ or $s'$, the other can be
computed. So possession of $d$ makes these two signatures atomic. This scheme does not link the two signatures or
compromise their security.

For an atomic transaction, during the setup stage, someone provides the opposing party with the value $d$, and asserts
it as the correct value. Once the transaction is signed, it can be adjusted to complete the other transaction. Atomicity
is achieved, but can only be used by the person who possesses this $d$ value. Generally, the party that stands to lose
money requires the $d$ value.

The $d$ value provides an interesting property with regard to atomicity. It is shared before signatures are public,
which in turn allows the two transactions to be atomic once the transactions are published. By taking the difference of
any two Schnorr signatures, one is able to construct transcripts, such as an atomic swap multisig contract.

This is a critical feature for Mimblewimble, which was previously thought to be unable to support atomic swaps or
Lightning channels [[4]].



## Atomic (Cross-chain Swaps) Example with Adaptor Signatures

Alice has a certain number of coins on a particular blockchain; Bob also has a certain number of coins on another
blockchain. Alice and Bob want to engage in an atomic exchange. However, neither blockchain is aware of the other, nor
are they able to verify each other's transactions.

The  classical way of achieving this involves the use of the blockchain's script system to put a hash preimage challenge
and then reveal the same preimage on both sides. Once Alice knows the preimage, she reveals it to take her coins. Bob
then copies it off one chain to the other chain to take his coins.

Using adaptor signatures, the same result can be achieved through simpler means. In this case, both Alice and Bob put
up their coins on two of two outputs on each blockchain. They sign the multisig protocols in parallel, where Bob then
gives Alice the adaptor signatures for each side using the same value $T$ . This means that for Bob to take his coins,
he needs to reveal $t$; and for Alice to take her coins, she needs to reveal $T$. Bob then replaces one of the
signatures and publishes $t$, taking his coins. Alice computes $t$  from the final signature, visible on the blockchain,
and uses that to reveal another signature, giving Alice her coins.

Thus it can be seen that atomicity is achieved. One is still able to exchange information, but now there are no explicit
hashes or preimages on the blockchain. No script properties are necessary and privacy is achieved [[4]].



## Zero Knowledge Contingent Payments

Zero Knowledge Contingent Payments (ZKCP) is a transaction protocol. This protocol allows a buyer to purchase
information from a seller using coins in a manner that is private, scalable, secure and, importantly, in a trustless
environment. The expected information is transferred only when  payment is made. The buyer and seller do not need to
trust each other or depend on arbitration by a third party [[6]].



## Mimblewimble's Core Scriptless Script

As previously stated, Mimblewimble is a blockchain design. Built similarly to Bitcoin, every transaction has inputs and
outputs. Each input and output has a confidential transaction commitment. Confidential commitments have an interesting
property where, in a valid balanced transaction, one can subtract the input from the output commitments, ensuring that
all of the values of the Pedersen values balance out. Taking the difference of these inputs and outputs results in the
multisig key of the owners of every output and every input in the transaction. This is referred to as the kernel.

Mimblewimble blocks will only have a list of new inputs, new outputs and signatures that are created
from the aforementioned excess value [[7]].

Since the values are homomorphically encrypted, nodes can verify that no coins are being created or destroyed.



## References

[[1]] "Crypto Innovation Spotlight 2: Scriptless Scripts" [online].
Available: <https://medium.com/blockchain-capital/crypto-innovation-spotlight-2-scriptless-scripts-306c4eb6b3a8>.
Date accessed: 2018&#8209;02&#8209;27.

[1]: https://medium.com/blockchain-capital/crypto-innovation-spotlight-2-scriptless-scripts-306c4eb6b3a8
"Crypto Innovation Spotlight 2: Scriptless Scripts"

[[2]] A. Poelstra, "Mimblewimble and Scriptless Scripts". Presented at Real World Crypto, 2018 [online].
Available: <https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s>. Date accessed: 2018&#8209;01&#8209;11.

[2]: https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s
"Mimblewimble and Scriptless Scripts"

[[3]] A. Poelstra, "Scriptless Scripts". Presented at Layer 2 Summit Hosted by MIT DCI and Fidelity Labs on
18 May 2018 [online]. Available: <https://www.youtube.com/watch?v=jzoS0tPUAiQ&t=3h36m>. Date Accessed: 2018&#8209;05&#8209;25.

[3]: https://www.youtube.com/watch?v=jzoS0tPUAiQ&t=3h36m
"Scriptless Scripts"

[[4]] A. Poelstra, "Mimblewimble and Scriptless Scripts". Presented at MIT Bitcoin Expo&nbsp;2017 Day&nbsp;1 [online].
Available: <https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20>. Date accessed: 2017&#8209;03&#8209;04.

[4]: https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20
"Mimblewimble and Scriptless Scripts"

[[5]] "Flipping the Scriptless Script on Schnorr" [online].
Available: <https://joinmarket.me/blog/blog/flipping-the-scriptless-script-on-schnorr/>. Date accessed: November&nbsp;2017.

[5]: https://joinmarket.me/blog/blog/flipping-the-scriptless-script-on-schnorr/
"Flipping the Scriptless Script on Schnorr"

[[6]] "The First Successful Zero-knowledge Contingent Payment" [online].
Available: <https://bitcoincore.org/en/2016/02/26/zero-knowledge-contingent-payments-announcement/>. Date accessed:
2016&#8209;02&#8209;26.

[6]: https://bitcoincore.org/en/2016/02/26/zero-knowledge-contingent-payments-announcement/
"The First Successful Zero-knowledge Contingent Payment, 26&nbsp;Feb&nbsp;2016"

[[7]] "What is Mimblewimble?" [Online.]
Available: <https://www.cryptocompare.com/coins/guides/what-is-mimblewimble/>. Date accessed: 2018&#8209;06&#8209;30.

[7]: https://www.cryptocompare.com/coins/guides/what-is-mimblewimble/
"What is Mimblewimble?"



## Contributors

- <https://github.com/kevoulee>
- <https://github.com/hansieodendaal>
- <https://github.com/anselld>
