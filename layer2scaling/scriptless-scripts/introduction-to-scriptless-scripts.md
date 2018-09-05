# Introduction to Scriptless Scripts

- Definition of Scriptless Scripts 
- Rationale for Scriptless Scripts 
- The Role of Schnorr Signatures
- Schnorr multi-signatures=Scriptless Scripts
- Simultaneous Scriptless Scripts
- Adaptor Signatures
- Features of Adaptor Signatures
- Atomic (Cross-chain Swaps) Example with Adaptive Signatures
- Mimblewimble's Scriptless Scripts 

---

## Definition of Scriptless Scripts 

Scriptless Scripts are digital signatures, created only through the execution of a smart contract through the implementation of Schnorr signatures. 

The concept of Scriptless Scripts was borne from Mimblewimble, which is a block chain design that with the exception of kernels and their signatures does not store permanent data. Fundamental properties of Mimblewimble include both privacy and scaling both of which require the implementation of Scriptless Scripts. 

---

## Rationale for Scriptless Scripts 

Bitcoin, Ethereum and other block chains have implemented a scripting language as a means to describe smart contracts and enforce their execution. These scripting languages allow one to execute smart contracts where coins can be spent under specified conditions, namely lock, multiparty, delay etc.

The script must be downloaded, witnessed and validated, preventing compression and aggregation; and since these cryptocurrencies have every transaction published and downloadable by all, they are not very fungible or private, as one is able to discern the rules applied to each individual coin and thus distinguish between coins. This has ramifications for commercial confidentiality, as the details of financial transactions are revealed, which compromises the real world purpose of these systems. 

Conversely, with scriptless scripts, the only components visible are the public keys and the digital signatures. These faceless cryptographic functions remove much of the semantic information. Thus, by using scriptless scripts one can avoid revealing any contracts.

---

## The Role of Schnorr Signatures 

With Schnorr signatures the signer has a private key *x*, an ephemeral secret nonce *k*, and a public key *xG* (where *G* is base point, of generator of the group) 

S can then be computed as a simple linear transaction

$$
s=k-ex
$$

Where:

$$
k= secret nonce 
$$

$$
x=private key and 
$$

$$
e=H(kG||xG||message)
$$

The position on the line chosen is taken as the hash of all the data that one needs to commit to, the digital signature. The verification equation involves the multiplication of each of the terms in the equation by G and takes in account the cryptographic assumption (discrete log) where G can be multiplied in but not divided out, thus preventing deciphering. 

$$
sG=kG-exG
$$

ECDSA signatures (used in Bitcoin) have the same shape, but *s* lacks some structure and *e* commits to only the message 

---

## Schnorr multi-signatures

For a multisignature, there are multiple parties and single signature. Every participant needs to contribute to produce the signature. The formulation of a multisignature involves taking the sum of all components; thus all nonces and *s* values result in the formulation of a multisignature. 
 
$$
s=Σs(i)
$$

It can therefore be seen that these signatures are essentially scriptless scripts. Independent public keys of several participants are joint to form a single key and signature, which when published do not divulge the details as to the number of participants involved or the original public keys. 

---

## Adaptive Signatures 

This multisignature protocol can be modified to produce an adaptive signature, which serves as the building block for all scriptless script functions. 

If two parties are considered: rather than providing their nonce *R* in the multisignature protocol, a blinding factor *T* is conceived and sent in addition to *R* (*R+T*). So it can be seen that *R* is not blinded, it has instead been offset by the secret value *T*. 

Here, the Schnorr multisignature construction is modified such that the first party generates $$T<sub>1</sub> =t<sub>1</sub>G$$ 

In place of $$R<sub>1</sub>$$ it passes $$R<sub>1</sub> +T<sub>1</sub>$$  to the other parties. 

Alongside $$s<sub>1</sub>$$ it passes $$T<sub>1</sub>$$ 

$$Adaptive signature=(T<sub>1</sub> , T<sub>1</sub>  + R<sub>1</sub>, s<sub>1</sub>)$$

So one follows the protocol, and receives a signature that is partially valid: valid if it is offset by a secret value *t* that only one party knows. It then becomes simple for everyone involved to verify that this occurrence $$(s, R)$$ is not valid but $$(s+t<sub>1</sub>,R)$$ is valid. 

Before signing, the other parties need to check $$s<sub>1</sub>G=R<sub>1</sub>+eP<sub>1</sub>$$ 

Thus, knowledge of $$t<sub>1</sub>$$ will be equivalent to knowledge of a valid signature. 

The above is very general however, by attaching auxiliary proofs to $$T<sub>1</sub>$$ one can derive an adaptive signature that will let one translate correct movement of the auxiliary protocol into a valid signature. 

---

## Simultaneous Scriptless Scripts 

The execution of separate transactions in an atomic fashion is achieved through preimages. If two transactions require the preimage to the same hash, once one is executed, the preimage is exposed so that the other one can be too. Atomic Swaps and Lightning channels use this construction. 

#### The Difference

If we consider the difference of two Schnorr signatures. 

$$
d=s-s'=k-k'+ex-e'x'
$$

The above equation can be verified in a similar manner to that of a single Schorr signature, by multiplying each term by G and confirming algebraic correctness. 

$$
dG=kG-k'G+exG-e'x'G
$$

It must be noted that the Schnorr signature itself is not being verified, but instead the difference *d*. *d* functions as the translating key between two separate independent Schnorr signatures. Given *d* and either *s* or *s'*, the other can be computed. So possession of *d* makes these two signatures atomic. This scheme does not link the two signatures or compromise their security. 

For an atomic transaction, during the setup stage, someone provides the opposing party with the value *d*, and asserts it as the correct value. Once the transaction is signed it can be adjusted to complete the other transaction. Atomicity is achieved; but can only be used by the person who possesses this *d* value. Generally the party that stands to lose money requires the *d* value. 

The *d* value provides an interesting property with regards to atomicity, it is shared before signatures are public which in turn allows the two transactions to be atomic once the transactions are published and by taking difference of any two Schnorr signatures one is able to construct transcripts, such as an atomic swap multisig contract. 

This is a critical feature for Mimblewimble, which was previously thought to be unable to support atomic swaps or lightning channels. 

---

## Atomic (Cross-chain Swaps) Example with Adaptive Signatures

Alice has a certain number of coins on a particular block chain; Bob also has a certain number of coins on another block chain. Alice and Bob want to engage in an atomic exchange, however neither of the block chains are aware of each other nor are they able to verify each others transactions. 

The  classical way of achieving this involves the use of the block chain's script system to put a hash preimage challenge and then reveal the same preimage on both sides: Once Alice knows the preimage, she reveals it to take her coins; Bob then copies it of one chain to the other chain to take his coins. 

Using adaptive signatures, the same result can be achieved through simpler means. In this case, both Alice and Bob put up their coins on two of two outputs on each block chain. They sign the multisignature protocols in parallel, where Bob then gives Alice the adaptive signatures for each side using the same value (T<sub>1</sub>); Meaning that for Bob to take his coins he needs to reveal t and for Alice to take her coins she needs to reveal T. Bob then replaces one of the signatures (s, R) with (s+t<sub>1</sub>,R )and publishes t, taking his coins. Alice computes t<sub>1</sub>  from the final signature, visible on the block chain and uses that to reveal another signature, giving her her coins. 

Thus it can be seen that atomicity is achieved. One is still able to exchange information but now there are no explicit hashes or preimages on the block chain: No script properties are necessary and privacy is achieved. 

---

## Mimblewimble's Core Scriptless Script

As previously stated, Mimblewimble is a block chain design. Built similarly to Bitcoin, every transaction has inputs and outputs. Each input and output has a corresponding key, referred to as a confidential transition commitment. Confidential commitments have an interesting property where in a valid balanced transaction  one can subtract the input from the output commitments, ensuring that all of the values of the Pedersen values balance out. Taking the difference results in the multisignature key of the owners of every output and every input in the transaction. This is referred to as the kernel. 

In order for a transaction to be considered valid, a signature is required with this key. This ensures two things 

1. The signature authorises the transaction and proves that the owners of the outputs and inputs are in agreement of the transaction
2. Secondly, the fact that the signature is possible, suggests that this difference is actually a multisignature key, which in turn means that the transaction balanced, using this proof of non-inflation. This highlights the validity of a transaction, where all relevant parties authorise it and no coins were created or destroyed.  

The core of Mimblewimble is that every transaction is compressed into a single key and a single signature, which has far reaching potential. 

---

## References 

Andrew Poelstra: Presentations at [MIT Bitcoin Expo Day 2017](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20s) and [Real World Crypto](https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s) 
