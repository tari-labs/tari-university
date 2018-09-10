# Introduction to Scriptless Scripts 
See [Layer 2 Scaling Survey (part 2)](https://gitpitch.com/tari-labs/tari-university/master?p=/layer2scaling/more-landscape#/5)

- Definition of Scriptless Scripts 
- Rationale for Scriptless Scripts 
- List of Scriptless Scripts 
- The Role of Schnorr Signatures
- Schnorr multi-signatures=Scriptless Scripts
- Simultaneous Scriptless Scripts
- Adaptor Signatures
- Features of Adaptor Signatures
- Atomic (Cross-chain Swaps) Example with Adaptive Signatures
- Mimblewimble's Scriptless Scripts 

---

## Definition of Scriptless Scripts 

Scriptless Scripts are a means to execute smart contracts off-chain, through the use of Schnorr signatures. [[1]](https://medium.com/blockchain-capital/crypto-innovation-spotlight-2-scriptless-scripts-306c4eb6b3a8)  

The concept of Scriptless Scripts was borne from Mimblewimble, which is a block chain design that with the exception of kernels and their signatures does not store permanent data. Fundamental properties of Mimblewimble include both privacy and scaling both of which require the implementation of Scriptless Scripts. [[2]](https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s)

---

## The benefit of Scriptless Scripts 

The benefit of Scriptless Scripts are functionality, privacy and efficiency. 

With regards to functionality, Scriptless Scripts are said to increase the range and complexity of smart contracts. Currently, as within Bitcoin Script limitations stem from the number of ```OP_CODES ```that have been enabled bu the network. Scriptless scripts move the specification and execution of smart contractions from the network to a discussion that only involves the participants of the smart contract. 

With regards to privacy, moving the specification and execution of smart contracts from on-chain to off-chain increases privacy. When on-chain, many details of the smart contract are shared to the entire network including the number and addresses of participants and the amounts transferred. By moving smart contracts off-chain, the network only knows that the participants agree that the terms of their contract have been satisfied and that the transaction in question is valid. 

With regards to efficiency, Scriptless Scripts minimize the amount of data that requires verification and storage on-chain. By moving smart contracts off-chain, there are less overheads for full nodes and lower transaction fees for users. [[1]](https://medium.com/blockchain-capital/crypto-innovation-spotlight-2-scriptless-scripts-306c4eb6b3a8)   

---

## A list of Scriptless Scripts 

In this report various forms of Scripts will be covered. These include: [[3]](https://www.youtube.com/watch?v=jzoS0tPUAiQ&t=3h36m)
- Simultaneous Scriptless Scripts
- Adaptive Signatures 
- Zero Knowledge Contingent Payments

---

## The Role of Schnorr Signatures 

To begin with, the fundamentals of Schnorr signatures must be defined . The signer has a private key *x*, random nonce *r*, and *G* is the generator of a discreet log hard group. *P* is the public key. [[4]](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20)

s, the signature, can then be computed as a simple linear transaction

$$
s=r+ex
$$

Where:

$$
e=H(P||r||message)
$$


The position on the line chosen is taken as the hash of all the data that one needs to commit to, the digital signature. The verification equation involves the multiplication of each of the terms in the equation by G and takes in account the cryptographic assumption (discrete log) where G can be multiplied in but not divided out, thus preventing deciphering. 

$$
sG=rG+exG
$$

ECDSA signatures (used in Bitcoin) are not linear in *x* and *r*, and thus less useful [[2]](https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s) 

---

## Schnorr multi-signatures

First, a mulitsignature has multiple participants that produce a signature. Every participant might product a separate signature and concatenate them forming a multisignature. 

With Schnorr Signatures, one can have a single public key, which is the sum of many different people's public keys. the resulting key is one that signatures will be verifiable against. [[5]](https://joinmarket.me/blog/blog/flipping-the-scriptless-script-on-schnorr/)

The formulation of a multisignature involves taking the sum of all components; thus all nonces and *s* values result in the formulation of a multisignature. [[4]](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20)
 
$$
s=Σs(i)
$$

It can therefore be seen that these signatures are essentially scriptless scripts. Independent public keys of several participants are joint to form a single key and signature, which when published do not divulge the details as to the number of participants involved or the original public keys. 

---

## Adaptive Signatures  

This multisignature protocol can be modified to produce an adaptive signature, which serves as the building block for all scriptless script functions. [[5]](https://joinmarket.me/blog/blog/flipping-the-scriptless-script-on-schnorr/) 

Instead of functioning as full valid signature on a message with a key, an adapter signature is a promise that a signature agreed to be published, will reveal a secret. 

This concept is similar to that of atomic swaps, however no scrips are implemented. Since this is elliptic curve cryptography, there is only scalar multiplication of elliptic curve points. Fortunately, like a hash function, elliptic curve function in one way, so an elliptic curve point (T), can simply be shared and the secret will be it's corresponding private key.  

If two parties are considered: rather than providing their nonce *R* in the multisignature protocol, a blinding factor, taken as an elliptic curve point *T* is conceived and sent in addition to *R* (ie. *R+T*). So it can be seen that *R* is not blinded, it has instead been offset by the secret value *T*. 

Here, the Schnorr multisignature construction is modified such that the first party generates $$T=tG$$ where *t* is the shared secret and $$R=rG$$(this is all with respect to the first participant of the 2 of 2 case swap) where *G*= generator of discreet log hard group and *r*=random nonce 

$$
s=r+t+H(P||R+T||message)x
$$

The first participant, publishes to the second participant and other the following 
$$s'=s-t$$, $$T=tG$$ and $$R=rG$$ where *s'* is the adaptive signature

The second participant can verify the adapter signature *s'* 

$$
s'G?+R+H(P||R+T||message)P
$$

However this is not a valid signature as the hashed nonce point is *R+T* and not *R*

The second participant cannot retrieve a valid signature and requires ECDLP solving to recover *s'+t*

After validation of adaptive signature by the second participant, it is known:
Receipt of *t* ==> receipt of valid signature $$s=s'+t$$

The above is very general however, by attaching auxiliary proofs to one can derive an adaptive signature that will let one translate correct movement of the auxiliary protocol into a valid signature. 

---

## Simultaneous Scriptless Scripts 

The execution of separate transactions in an atomic fashion is achieved through preimages. If two transactions require the preimage to the same hash, once one is executed, the preimage is exposed so that the other one can be too. Atomic Swaps and Lightning channels use this construction. [[4]](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20)

#### The Difference

If we consider the difference of two Schnorr signatures. 

$$
d=s-s'=k-k'+ex-e'x'
$$

The above equation can be verified in a similar manner to that of a single Schnorr signature, by multiplying each term by G and confirming algebraic correctness. 

$$
dG=kG-k'G+exG-e'x'G
$$

It must be noted that the Schnorr signature itself is not being verified, but instead the difference *d*. *d* functions as the translating key between two separate independent Schnorr signatures. Given *d* and either *s* or *s'*, the other can be computed. So possession of *d* makes these two signatures atomic. This scheme does not link the two signatures or compromise their security. 

For an atomic transaction, during the setup stage, someone provides the opposing party with the value *d*, and asserts it as the correct value. Once the transaction is signed it can be adjusted to complete the other transaction. Atomicity is achieved; but can only be used by the person who possesses this *d* value. Generally the party that stands to lose money requires the *d* value. 

The *d* value provides an interesting property with regards to atomicity, it is shared before signatures are public which in turn allows the two transactions to be atomic once the transactions are published and by taking difference of any two Schnorr signatures one is able to construct transcripts, such as an atomic swap multisig contract. 

This is a critical feature for Mimblewimble, which was previously thought to be unable to support atomic swaps or lightning channels. [[4]](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20)

---

## Atomic (Cross-chain Swaps) Example with Adaptive Signatures

Alice has a certain number of coins on a particular block chain; Bob also has a certain number of coins on another block chain. Alice and Bob want to engage in an atomic exchange, however neither of the block chains are aware of each other nor are they able to verify each others transactions. 

The  classical way of achieving this involves the use of the block chain's script system to put a hash preimage challenge and then reveal the same preimage on both sides: Once Alice knows the preimage, she reveals it to take her coins; Bob then copies it of one chain to the other chain to take his coins. 

Using adaptive signatures, the same result can be achieved through simpler means. In this case, both Alice and Bob put up their coins on two of two outputs on each block chain. They sign the multisignature protocols in parallel, where Bob then gives Alice the adaptive signatures for each side using the same value *T* ; Meaning that for Bob to take his coins he needs to reveal t and for Alice to take her coins she needs to reveal T. Bob then replaces one of the signatures and publishes *t*, taking his coins. Alice computes *t*  from the final signature, visible on the block chain and uses that to reveal another signature, giving her her coins. 

Thus it can be seen that atomicity is achieved. One is still able to exchange information but now there are no explicit hashes or preimages on the block chain: No script properties are necessary and privacy is achieved. [[4]](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20)

---

## Zero Knowledge Contingent Payments 

ZKCP is a transaction protocol. This protocol allows a buyer to purchase information from a seller using coins in a manner which is private, scalable, secure, and importantly in a trustless environment. The expected information is transferred only when  payment is made. The buyer and seller do not need to trust each other or depend on arbitration by a third party.[[6]](https://bitcoincore.org/en/2016/02/26/zero-knowledge-contingent-payments-announcement/)

---

## Mimblewimble's Core Scriptless Script

As previously stated, Mimblewimble is a block chain design. Built similarly to Bitcoin, every transaction has inputs and outputs. Each input and output has a confidential transition commitment. Confidential commitments have an interesting property where in a valid balanced transaction one can subtract the input from the output commitments, ensuring that all of the values of the Pedersen values balance out. Taking the difference of these inputs and outputs results in the multisignature key of the owners of every output and every input in the transaction. This is referred to as the kernel.

Mimblewimble blocks will only have a list of new inputs, a list of new outputs and a list of signatures which are created from the aforementioned excess value. [[7]](https://www.cryptocompare.com/coins/guides/what-is-mimblewimble/)

Since the values are homomorphically encrypted, nodes can verify that no coin are being created or destroyed. 

---

## References 
1. Crypto Innovation Spotlight 2: Scriptless Scripts, 27 Feb 2018. https://medium.com/blockchain-capital/crypto-innovation-spotlight-2-scriptless-scripts-306c4eb6b3a8
2.  Andrew Poelstra: Presentation at [Real World Crypto](https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s) 
3.  Andrew Poelstra: Presentation at [Layer 2 Summit Hosted by MIT DCI and Fidelity Labs](https://www.youtube.com/watch?v=jzoS0tPUAiQ&t=3h36m)
4.  Andrew Poelstra: Presentation at [MIT Bitcoin Expo Day 2017](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20)
5.  Flipping the scriptless script on Schnorr, Nov 2017. (https://joinmarket.me/blog/blog/flipping-the-scriptless-script-on-schnorr/)
6.  The first successful Zero-Knowledge Contingent Payment, 26 Feb 2016. (https://bitcoincore.org/en/2016/02/26/zero-knowledge-contingent-payments-announcement/)
7.  What is Mimblewimble?, 30 Jun 2018. (https://www.cryptocompare.com/coins/guides/what-is-mimblewimble/)

