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
- As a recap: Mimblewimble is a blockchain design with no permanent data except kernels and their signatures; supports only script less scripts (anything that supports Schnorr signatures will support scriptless scripts-which is how it derives its privacy and scaling properties

Note: Scriptless scripts are magic digital signatures that can only be created by faithful execution of a smart contract. You might think this is limited in power, and it is, in an academic sense, but there's a lot we can do with this paradigm. Verifying digital signatures is a very general function, but it is powerful.  Also to recap, Mimblewimble is a blockchain design with no permanent data except kernels and their signatures. MimbleWimble only supports scriptless scripts, which is how it derives its privacy and scaling properties

---

## Rationale for Scriptless Scripts 

- In order to describe smart contracts and enforce there execution, Bitcoin (and Ethereum, etc.) use a **scripting language**
- The scripts must be downloaded--> parsed-->validated by all the nodes present on the network 
- There is little intrinsic structure to be compressed 
- Script details are visible and thus compromise privacy and fungibility 

*Conversely, with scriptless scripts, the only components visible are the public keys (i.e uniformly random curve points) and the digital signatures

Note: Bitcoin, Ethereum and other blockchains make use of a scripting language which is a way of describing under which conditions coins can be spent. These scripting languages allow you to do smart contracts where coins can be spent under conditions like time lock, multiparty, delay, random numbers etc. Here, scripts must be downloaded, witnessed and validated and this prevents compression and aggregation. These public cryptocurrencies have every transaction published and downloadable by everybody- they are visible to everyone forever and they have to be stored. This means different outputs in Bitcoin, or different accounts in ethereum, are not very fungible or private because you can tell what the rules are on each individual coin and you can distinguish between coins. Obviously this is bad for commercial confidentiality-especially if your amounts are recorded on the blockchain-this makes it extremely difficult to do business when you are revealing all of your financial transactions. It is no good and compromises the usability of these systems for real purposes. Conversely, with scriptless scripts, the only components visible are the public keys and the digital signatures- These are faceless cryptographic functions. They just look like random data that has some simple algebraic relationship . It erases so much of the semantic information. It is also a consistent data structure, so you can have a lot of compression. Thus, by using scriptless scripts one can avoid revealing any contracts.

---

## The Role of Schnorr Signatures 

- For Schnorr signatures the signer has a secret key ***x***, ephemeral secret key ***k*** -- he publishes a public key ***xG*** (*G* is base point, of generator of the group)

- A signature is the ephemeral pubic key kG as well as 

  $$
  s=k-ex
  $$
  Where:

  *k*= secret nonce 

  *x*=private key

  $$
  e=H(kG||xG||message)
  $$

- Verified by checking 

  $$
  sG=kG-exG
  $$

- ECDSA signatures (used in Bitcoin) have the same shape, but *s* lacks some structure and *e* commits to only the message 

Note: I know it has been mentioned before, but I would like to digress a little and discuss Schnorr signatures. You have an ephemeral key pair, like k and kg here, you compute this value s which is a simple linear transaction. The position on the line you choose is taken as the hash of all the data you want to commit to, which is what a digital signature is. The verification equation is simple, one can multiply each of the terms in the equation by G-there is a cryptographic assumption called the discrete log problem which is where one can put G's in there and they cannot be divided out and this prevents you from deciphering.  

---
## Schnorr multi-signatures=Scriptless Scripts

- The multisignature is (s,R) 
  $$
  s=Σs(i)
  $$


- It can be seen that a multisignature is already a scriptless script
- It can be generalised to m-of-n by linear secret sharing 
- In general, scriptless scripts will see their power from these signatures being linear in all secret inputs 

Note: For a multisignature, you have multiple parties and you want to create a signature that every participant needs to contribute to to produce a signature. To do a multisignature you just sum everything. So your nonces are a sum of contributed nonces and your s values are a sum of everyones contributing s values and the result is a multisignature  So kind of a philosophical point is that these signatures are kind of a script less script, in the sense that you have a bunch of people who all have their own independent public keys so they add together to get a joint key and what they publish to the wider public is just a joint key and joint signatures using that key, public verifiers who weren’t party to that don’t know that how many people were involved, they don’t even know that more than one person was involved they certainly do not know the original public keys. There is a standard in the literature where you look for threshold signatures find a generalisation this to m of n signatures using linear secret sharing. And this nice property that because the multi signature came from adding up everyones nonces and adding up everyones s values if you put a linear secret sharing scheme among there you can basically do the same thing, where you are contributing shares of signatures rather than entire signatures and this all sort of works magically.  

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

Note: Now we look at the difference of two Schnorr signatures. This is the equation for an earlier slide- with a few changes- allowing for the subtraction of one from the other. You can verify this equation the same as verifying an ordinary Schnorr signature, you multiply every term by G and you verify whether the algebra works out. This is not verifying your Schnorr signature this is verifying a special property of this value d- this difference d. What is being verified is that if this verification equation checks out, then given d, a valid Schnorr signature s or s' , you can figure out the other one s' or s just given the first signature. d is essentially a translating key between two separate independent Schnorr signatures. So to make two transactions atomic, one is happening from the signature of one party, the other transaction is happening with the other, or more likely they are both  2-of-2, each party has a half that they are contributing to each transaction. During the setup phase, someone gives this other party the value d, and asserts it as the correct value and then once they give that value away, as soon as they sign one transaction, that signature can be used to tweak and make that other transaction. You get this atomicity property from hash preimages here. But the atomicity can only be used by the person who has this value d-this is fine both in lighting and general atomic swaps. There is only really one party, the person who stands to lose money that needs this value d and they can just assure that they have it during the setup phase. And what's exciting about this value d is that it's the difference between two Schnorr signatures. So the magic here is that someone is sharing the value d before the signatures are public and this allows the two transactions to be atomic once the transactions are out there and public, then d is just the difference in signatures. You can take any two Schnorr signatures in the world and take their difference, and it will be a valid d, and you can go make up a transcripts with people- like an atomic swap multisig contract. So this value d doesn't provide any linkages. These remain independent separate signatures. The magic is that d is public before the signatures are public- that is the because of the atomicity. And this is critical because in MimbleWimble there is no support for scripts, and it was actually thought for a long time after MimbleWimble came out that we wouldn't be able to do atomic swaps or lightning channels- which has massive implications. MimbleWimble’s claim to fame was that it was simple and scaled well, but if it could not do lightning channels and off-chain payment channels then this blockchain scalability gets undermined. People would say that Bitcoin in principle can setup many transactions into a single transaction, and its all well and good that MimbleWimble transactions are small and compatible but if you don’t need transactions in the first place then it is not better. It turns out one can do lightning channels on MimbleWimble using this and extra components for lock time. We can achieve this on MimbleWimble and any blockchain that uses Schnorr signatures. If Bitcoin had Schnorr signatures, you could do lightning channels- this closing scheme where currently in lightning everyone has a hash preimage and every channel in the path has to have a preimage of the same hash, you could use the linkage between each piece of the path to have a different d value, and so you still have the atomicity and channels or path of channels which can only be created/destroyed in one go. You no longer have this public link-ability and you no longer have these hash preimages in places that people have to store and validate forever. So you get a win for scalability, fungibility, privacy, etc. An overarching theme has been what hits the blockchain and here it is just signatures and keys and they all look random, there's nothing for people to really track here and attaching taint or censoring. Everything sort of looks the same, which was the goal of scriptless scripts and that we achieved here. 

---

## Adaptor Signatures 

- Consider the Schnorr multi signature construction, modified such that the first party generates T<sub>1</sub> =t<sub>1</sub>G. In place of R<sub>1</sub> it passes R<sub>1</sub> +T<sub>1</sub>  to the other parties. Alongside s<sub>1</sub> it passes T<sub>1</sub> 
- Adaptive signature= (T<sub>1</sub> , T<sub>1</sub>  + R<sub>1</sub>, s<sub>1</sub>  ) 
- (s, R) is not valid but (s+t<sub>1</sub>,R ) is valid
- Before signing, the other parties need to check s<sub>1</sub>G=R<sub>1</sub>+eP<sub>1</sub>   
- So the knowledge of t<sub>1</sub> will be equivalent to knowledge of a valid signature 

Note:So this multisignature protocol is modified to produce an adaptive signature- and this is going to be the building block for all of this scriptless script functions. So for two parties-somebody rather than giving their nonce R in the multisignature protocol they think of this blinding factor T and they send R+T (where R is the nonce) instead-they do everything else in the same way and they are also going to send T (so it is a strange thing to do, because you are not really blinding the nonce-you are not trying to hide it-all you are doing is offsetting it by some secret value that you know. So you do all of this following the protocol, what you get is a signature that is almost valid-it is valid if you offset its some secret value t that only one party knows, and it is easy for everyone involved to verify that this is happening correctly. And so what you have is a signature (adaptive signature) where if you know the adaptive signature, knowledge of a valid signature with the same nonce-which is enforced by the hash being the same-knowledge of a valid signature is equivalent to knowledge of the secret value t-this is a building block

---

## Features of Adaptor Signatures 

- By attaching auxiliary proofs to T<sub>1</sub>  to ensure t<sub>1</sub> is some necessary data for a separate protocol, "arbitrary steps of arbitrary protocols can be made equivalent to signature production"
- ==> parties can be paid for honest participation in a trestles manner
- Using the same T<sub>1</sub>  in multiple adaptor signatures results in the possibility to set signatures atomically 
- After a signature hits the chain, anyone can make up a T<sub>1</sub> and compute a corresponding "adaptor signature" for it, so such schemes are deniable/private

Note: Now this is very general and a lot can be done from this. T was said,  but if you attaching other artillery proofs to T- or if you are doing any other complicated multiparty protocol in parallel- T is something important to the protocol, what you have got now is an adaptive signature that will let you translate correct movement of the auxiliary protocol into a valid signature. In particular if you are doing any blockchain work you can create transactions that can only be completed by correct execution of this auxiliary protocol- and this is extremely cheap to do. This is a very general framework for attaching valid signatures to arbitrary discreet log based protocols and more generally any protocol where your primitives are linear. 

---

## Atomic (Cross-chain Swaps) Example with Adaptive Signatures

- Parties Alice and Bob send coins to their respective chains
- They each move their coins to outputs (can only be spent by a 2-of-2 multi signature)
- They sign the multisignature protocols in parallel, where Bob gives Alice adaptor signatures using the same T<sub>1</sub> 
- Bob then replaces one of the signatures (s, R) with (s+t<sub>1</sub>,R )and publishes it--> taking his coins
- Alice sees this, learns t<sub>1</sub>, and proceeds to do the same thing on the other chain, taking her coins

Note: So one example of this : atomic swaps- standard typical prototypical smart contract that you think about when you are working with blockchain. So we have Alice and Bob- Alice has A coins on the A coin blockchain and Bob has B coins on the B-coin blockchain. And they want to exchange these. These two blockchain do not know about each other and cannot verify each others transactions but some how they want to make these two transactions happen atomically on each chain- either both go through or neither there is no risk that Alice takes Bob’s coins and then does not put up her own - so there is a classical way to do this- where you use the blockchain’s script system to put a hash preimage challenge and then you have to reveal the same preimage on both sides, and then Alice knows the preimage, she reveals it to take her coins, Bob copies it of one chain to the other chain to take his coins - thats how the atomicity works. Here, using adaptive signatures we can do something simpler, where both Alice and Bob put up their coins on two of two outputs on each blockchain then Bob gives Alice the adaptive signatures for each side using the same T value meaning that for Bob to take his coins he need to reveal T and for Alice to take her coins she needs to reveal T. And Bob actually knows T. So they do this whole set up and there are some ordering constraints and in the end nobody has the opportunity to take their coins without every thing being set up such when Bob takes his coins he publishes t, Alice can compute t from the final signature that hits the blockchain and use that to get another signature giving her her coins and you get atomicity that way- you still have exchange of information but now we don’t have explicit hashes on the blockchain or explicit preimages on the blockchain- no body can even see that this is happening- it is a much smaller system and you have a much smaller system used to verify publicly that these signatures are valid and this is cool, because you basically change the support signatures you do not need any other script properties and as a bonus you get privacy 

---

## MimbleWimble's Scriptless Script

- MimbleWimble can then be considered the ultimate scriptless script 
- Every input and output has a key (referred to as a Pedersen commitment-but the transactions balances exactly when these commitment behave like keys; this trick is Confidential Transactions)
- A transaction signature uses the multi signature key of all input and output keys (called a "kernel" in MimbleWimble). It is irrelevant what gets signed, just that something is 
- Transaction validity is now contained in a scriptless script; further the signature has be used with other scriptless script constructions (atomic swaps, ZKCP, etc.) to add additional validity requirements with zero overhead

Note: To begin and end with a recap of MimbleWimble, which Andrew Poelstra claims to be the ultimate scriptless script. The way that MimbleWimble transactions work- MimbleWimble is a blockchain design, like in Bitcoin where every transaction has inputs and outputs. Every input and output has a key- a confidential transaction commitment- There is an interesting property of confidential transactions where in a valid balance transaction (where the input value equals the output value) you can take the output commitments subtract the input commitments- all of the values of the Pederson values balance out, so it is like you subtracted a lot of keys from a lot of keys which is a multisignature key. If you take this difference (outputs minus inputs) you get a multisignature key of the owners of every outputs and the owners of every input in the transaction. We call this the "kernel" in MimbleWimble-but it is just a multisignature key. To make the transaction valid, we require a signature with this key. This does two things: the signature authorizes the transaction and proves that the owners of the outputs and inputs wanted the transaction to happen, and secondly the fact that the signature is possible, means this difference is actually a multisignature key, which in turn means that the transaction balanced, so it's using this proof of non-inflation. So what this is, is that the validity of a transaction, meaning all relevant parties authorized it, where no coins were created from nowhere or were destroyed are both validated in scriptless scripts. The validity comes down to a single key and a single signature and ultimately everything in MimbleWimble except for these keys and signatures is just for witnessing what's going on. The core of MimbleWimble is basically every transaction is compressed into a single key and a single signature. The signature is simple enough, there's no real requirement, there's one thing that you need to add, you need to sign a locktime-this is necessary to make a lot of this protocols safer as it provides cover if people bale. This very simple signature, can be used in mulitple cases, one could make the preimage the encryption key of something, or link the signatures, using an atomic swap mechanism,  so even though these MimbleWimble transactions are really just single signatures, one can enforce arbitrary multiparty smart contracts into these single signatures. Which is the real magic of MimbleWimble and scriptless scripts. 

---

## References 

Andrew Poelstra: Presentations at [MIT Bitcoin Expo Day 2017](https://www.youtube.com/watch?v=0mVOq1jaR1U&feature=youtu.be&t=39m20s) and [Real World Crypto](https://www.youtube.com/watch?v=ovCBT1gyk9c&t=0s) 
