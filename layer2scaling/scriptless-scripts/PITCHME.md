# Scriptless Scripts

- Definition of Scriptless Scripts 
- Rationale for Scriptless Scripts 
- The Role of Schnorr Signatures
- Schnorr multi-signatures=Scriptless Scripts
- Simultaneous Scriptless Scripts
- Adaptor Signatures
- Features of Adaptor Signatures
- Atomic (Cross-chain Swaps) Example with Adaptive Signatures
- MimbleWimble's Scriptless Script

---

## Definition of Scriptless Scripts 

- Scriptless scripts involve the 'magicking' of digital signatures so that they can only be created by faithful execution of a smart contract (using Schnorr signatures)
- They have been considered to be limited in power
- As a recap: Mimblewimble is a block chain design with no permanent data except kernels and their signatures; supports only script less scripts (anything that supports Schnorr signatures will support scriptless scripts-which is how it derives its privacy and scaling properties

---

## Rationale for Scriptless Scripts 

- In order to describe smart contracts and enforce there execution, Bitcoin (and Ethereum, etc.) use a **scripting language**
- The scripts must be downloaded--> parsed-->validated by all the nodes present on the network 
- There is little intrinsic structure to be compressed 
- Script details are visible and thus compromise privacy and fungibility 

*Conversely, with scriptless scripts, the only components visible are the public keys (i.e uniformly random curve points) and the digital signatures

---

## The Role of Schnorr Signatures 

- For Schnorr signatures the signer has a secret key ***x***, ephemeral secret key ***k*** -- he publishes a public key ***xG*** (*G* is base point, of generator of the group)

- A signature is the ephemeral public key *kG* as well as 

  $$
  s=k-ex
  $$
  Where *k* is the secret nonce and *x* the private key

  $$
  e=H(kG||xG||message)
  $$

- Verified by checking 

  $$
  sG=kG-exG
  $$

- ECDSA signatures (used in Bitcoin) are not linear in *x* and *r*, and thus less useful  

---

## Schnorr multi-signatures = Scriptless Scripts

- The multisignature is (s,R) 
  $$
  s=Σs(i)
  $$

- It can be seen that a multisig is already a scriptless script
- It can be generalized to m-of-n by linear secret sharing 
- In general, scriptless scripts will see their power from these signatures being linear in all secret inputs 

---

## Simultaneous Scriptless Scripts 

- Executing separate transactions in an atomic fashion is traditionally done with preimages: if two transactions require the preimage to the same hash, once one is executed, the preimage is exposed so that the other one can be too 
- Atomic Swaps and Lightning channels use this contruction 

+++

## Simultaneous Scriptless Scripts cont.

- Instead what is done is the difference of two Schnorr signature signatures is considered

  $$
  d=s-s'=k-k'+ex-e'x'
  $$

- Given *kG, k'G, e, e'* this construction can be verified as 

  $$
  dG=kG-k'G+exG-e'x'G
  $$



- Given *d* and either *s* or *s'*, the other can be computed. So possession of d makes these two signatures atomic
- But since *d* is computable by anybody after *s, s'* are available, this scheme does nothing to link the two signatures or harm their security

---

## Adaptor Signatures 

- Consider the Schnorr multi signature construction, modified such that the first party generates T<sub>1</sub> =t<sub>1</sub>G. In place of R<sub>1</sub> it passes R<sub>1</sub> +T<sub>1</sub>  to the other parties. Alongside s<sub>1</sub> it passes T<sub>1</sub> 
- Adaptive signature= (T<sub>1</sub> , T<sub>1</sub>  + R<sub>1</sub>, s<sub>1</sub>  ) 
- (s, R) is not valid but (s+t<sub>1</sub>,R ) is valid
- Before signing, the other parties need to check s<sub>1</sub>G=R<sub>1</sub>+eP<sub>1</sub>   
- So the knowledge of t<sub>1</sub> will be equivalent to knowledge of a valid signature 

---

## Features of Adaptor Signatures 

- By attaching auxiliary proofs to T<sub>1</sub>  to ensure t<sub>1</sub> is some necessary data for a separate protocol, "arbitrary steps of arbitrary protocols can be made equivalent to signature production"
- ==> parties can be paid for honest participation in a trestles manner
- Using the same T<sub>1</sub>  in multiple adaptor signatures results in the possibility to set signatures atomically 
- After a signature hits the chain, anyone can make up a T<sub>1</sub> and compute a corresponding "adaptor signature" for it, so such schemes are deniable/private

---

## Atomic (Cross-chain Swaps) Example with Adaptive Signatures

- Parties Alice and Bob send coins to their respective chains
- They each move their coins to outputs (can only be spent by a 2-of-2 multi signature)
- They sign the multisignature protocols in parallel, where Bob gives Alice adaptor signatures using the same T<sub>1</sub> 
- Bob then replaces one of the signatures (s, R) with (s+t<sub>1</sub>,R )and publishes it--> taking his coins
- Alice sees this, learns t<sub>1</sub>, and proceeds to do the same thing on the other chain, taking her coins

---

## MimbleWimble's Scriptless Script

- MimbleWimble can then be considered the ultimate scriptless script 
- Every input and output has a key (referred to as a Pedersen commitment-but the transactions balances exactly when these commitment behave like keys; this trick is Confidential Transactions)
- A transaction signature uses the multi signature key of all input and output keys (called a "kernel" in Mimblewimble). It is irrelevant what gets signed, just that something is 
- Transaction validity is now contained in a scriptless script; further the signature has be used with other scriptless script constructions (atomic swaps, ZKCP, etc.) to add additional validity requirements with zero overhead

---

## References 

Andrew Poelstra: Presentations at [MIT Bitcoin Expo Day 2017](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20s) and [Real World Crypto](https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s) 
