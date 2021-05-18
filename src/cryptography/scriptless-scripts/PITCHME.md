---
marp: true
theme: default
paginate: true
footer: © Tari Labs, 2018-2021. (License : CC BY-NC-SA 4.0)
---

<style>
section {
  font-size: 1.5em;
}
</style>
# Scriptless Scripts

- Definition of Scriptless Scripts
- The Benefit for Scriptless Scripts
- List of Scriptless Scripts
- The Role of Schnorr Signatures
- Schnorr multi-signatures=Scriptless Scripts
- Adaptor Signatures
- Simultaneous Scriptless Scripts
- Atomic (Cross-chain Swaps) Example with Adaptor Signatures
- Zero Knowledge Contingent Payments
- Mimblewimble's Core Scriptless Script

---

## Definition of Scriptless Scripts

- Scriptless Scripts are a means to execute smart contracts off-chain, through the use of Schnorr signatures. [[1]](https://medium.com/-capital/crypto-innovation-spotlight-2-scriptless-scripts-306c4eb6b3a8)

- The concept of Scriptless Scripts was borne from Mimblewimble, recap: Mimblewimble is a block chain design that with the exception of kernels and their signatures does not store permanent data. Fundamental properties of Mimblewimble include both privacy and scaling both of which require the implementation of Scriptless Scripts. [[2]](https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s)

*Also see [Scriptless scripts, Layer 2 Scaling Survey](../../scaling/layer2scaling-landscape/layer2scaling-survey.md#scriptless-scripts)*

---

##  The Benefit of Scriptless Scripts

The benefit of Scriptless Scripts are functionality, privacy and efficiency. [[1]](https://medium.com/blockchain-capital/crypto-innovation-spotlight-2-scriptless-scripts-306c4eb6b3a8)

- Functionality: Scriptless Scripts may increase the range and complexity of smart contracts. Scriptless scripts move the specification and execution of smart contracts from the network to a discussion that only involves the participants of the smart contract.
- Privacy: Moving the specification and execution of smart contracts from on-chain to off-chain increases privacy.
- Efficiency: Scriptless Scripts minimize the amount of data that requires verification and storage on-chain.

---

## List of Scriptless Scripts

These include: [[3]](https://www.youtube.com/watch?v=jzoS0tPUAiQ&t=3h36m)
- Simultaneous Scriptless Scripts
- Adaptor Signatures
- Zero Knowledge Contingent Payments

---

## The Role of Schnorr Signatures

The fundamentals of Schnorr signatures must be defined . The signer has a private key $x$, random nonce $r$, and $G$ is the generator of a discreet log hard group. $P$ is the public key. ([[4]](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20), [[2]](https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s))

$s$, the signature, can then be computed as a simple linear transaction

$$
s=r+ex
$$

Where:

$$
e=H(P||r||message)
$$

$$
P=xG
$$

The verification equation involves the multiplication of each of the terms in the equation by G and takes in account the cryptographic assumption (discrete log) where G can be multiplied in but not divided out, thus preventing deciphering.

$$
sG=rG+exG
$$

---

## Schnorr multi-signatures = Scriptless Scripts

A multisig has multiple participants that produce a signature. Every participant might produce a separate signature and concatenate them, forming a multisig.

$$
  s=Σs(i)
$$

- It can be seen that a multisig is already a scriptless script
- Independent public keys of several participants are joint to form a single key and signature
- This does not divulge the details as to the number of participants involved or the original public keys.

---

## Adaptor Signatures

- This multisig protocol can be modified to produce an adaptor signature, which serves as the building block for all scriptless script functions. [[5]](https://joinmarket.me/blog/blog/flipping-the-scriptless-script-on-schnorr/)

- Instead of functioning as full valid signature on a message with a key, an adaptor signature is a promise that a signature agreed to be published, will reveal a secret.

- This concept is similar to that of atomic swaps, however no scrips are implemented. Since this is elliptic curve cryptography, there is only scalar multiplication of elliptic curve points. Fortunately, like a hash function, elliptic curve function in one way, so an elliptic curve point (*T*), can simply be shared and the secret will be it's corresponding private key.

- If two parties are considered: rather than providing their nonce *R* in the multisig protocol, a blinding factor, taken as an elliptic curve point *T* is conceived and sent in addition to *R* (ie. *R+T*). So it can be seen that *R* is not blinded, it has instead been offset by the secret value *T*.

---

Here, the Schnorr multisig construction is modified such that the first party generates

$$
T=tG, R=rG
$$

where *t* is the shared secret, *G* is the generator of discreet log hard group and *r* the random nonce

Using this information the second party generates

$$
H(P||R+T||message)x
$$

where the coins to be swapped are contained within *message*. The first party can now calculate the complete signature *s* such that

$$
s=r+t+H(P||R+T||message)x
$$

---

The first party then calculates and publishes the adaptor signature *s'* to the second party (and any one else listening)

$$
s'=s-t
$$

The second party can verify the adaptor signature *s'* by asserting *s'G*

$$
s'G=?+R+H(P||R+T||message)P
$$

However this is not a valid signature as the hashed nonce point is *R+T* and not *R*

The second party cannot retrieve a valid signature from this and requires ECDLP solving to recover *s'+t*, which is virtually impossible.

After the first party broadcasts *s* to claim the coins within *message* the second party can calculate the secret *t* from

$$
t=s-s'
$$

By attaching auxiliary proofs one can derive an adaptor signature to translate correct movement of the auxiliary protocol into a valid signature.

---

## Simultaneous Scriptless Scripts

- The execution of separate transactions in an atomic fashion is achieved through preimages. If two transactions require the preimage to the same hash, once one is executed, the preimage is exposed so that the other one can be too. Atomic Swaps and Lightning channels use this construction. [[4]](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20)

---

## Simultaneous Scriptless Scripts cont.

- Instead what is done is the difference of two Schnorr signature signatures is considered

  $$
  d=s-s'=k-k'+ex-e'x'
  $$

- Given *kG, k'G, e, e'* this construction can be verified as

  $$
  dG=kG-k'G+exG-e'x'G
  $$






- The Schnorr signature itself is not being verified, but instead the difference *d*
- *d* functions as the translating key between two separate independent Schnorr signatures
- Given *d* and either *s* or *s'*, the other can be computed
- This scheme does not link the two signatures or compromise their security

---

## Atomic (Cross-chain Swaps) Example with Adaptive Signatures

- Parties Alice and Bob send coins to their respective chains
- They each move their coins to outputs (can only be spent by a 2-of-2 multi signature)
- They sign the multisignature protocols in parallel, where Bob then gives Alice the adaptor signatures for each side using the same value *T*
- Meaning that for Bob to take his coins he needs to reveal *t* and for Alice to take her coins she needs to reveal *T*
- Bob then replaces one of the signatures and publishes *t*, taking his coins
- Alice computes *t*  from the final signature, visible on the block chain and uses that to reveal another signature, giving her her coins. [[4]](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20)

---

## Zero Knowledge Contingent Payments

- ZKCP is a transaction protocol
- This protocol allows a buyer to purchase information from a seller using coins in a manner which is private, scalable, secure, and importantly in a trustless environment
- The expected information is transferred only when  payment is made
- The buyer and seller do not need to trust each other or depend on arbitration by a third party. [[6]](https://bitcoincore.org/en/2016/02/26/zero-knowledge-contingent-payments-announcement/)

---

## Mimblewimble's Core Scriptless Script

- Mimblewimble is a block chain design. Built similarly to Bitcoin, every transaction has inputs and outputs. Each input and output has a confidential transaction commitment.
- Confidential commitments have an interesting property where in a valid balanced transaction one can subtract the input from the output commitments, ensuring that all of the values of the Pedersen values balance out.
- Taking the difference of these inputs and outputs results in the multisig key of the owners of every output and every input in the transaction. This is referred to as the kernel.
- Mimblewimble blocks will only have a list of new inputs, a list of new outputs and a list of signatures which are created from the aforementioned excess value. [[7]](https://www.cryptocompare.com/coins/guides/what-is-mimblewimble/)
- Since the values are homomorphically encrypted, nodes can verify that no coins are being created or destroyed.

---

## References

[1] - [Crypto Innovation Spotlight 2: Scriptless Scripts, 27 Feb 2018](https://medium.com/blockchain-capital/crypto-innovation-spotlight-2-scriptless-scripts-306c4eb6b3a8)

[2] - [Andrew Poelstra: Presentation at Real World Crypto](https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s)

[3] - [Andrew Poelstra: Presentation at Layer 2 Summit Hosted by MIT DCI and Fidelity Labs](https://www.youtube.com/watch?v=jzoS0tPUAiQ&t=3h36m)

[4] - [Andrew Poelstra: Presentation at MIT Bitcoin Expo Day 2017](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20)

[5] - [Flipping the scriptless script on Schnorr, Nov 2017](https://joinmarket.me/blog/blog/flipping-the-scriptless-script-on-schnorr/)

[6] - [The first successful Zero-Knowledge Contingent Payment, 26 Feb 2016](https://bitcoincore.org/en/2016/02/26/zero-knowledge-contingent-payments-announcement/)

[7] - [What is Mimblewimble?, 30 Jun 2018](https://www.cryptocompare.com/coins/guides/what-is-mimblewimble/)