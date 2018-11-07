# Mimblewimble-Grin Blockchain Protocol Overview

## Introduction

Depending on who you ask, MimbleWimble is either a tongue-tying curse or blockchain protocol designed to be private and scalable. The transactions in MimbleWimble is derived from confidential transactions by Greg Maxwell[1], that is in turn based on the Pedersen commitment scheme. On 19 July 2016 someone with the name Tom Elvis Jedusor left a whitepaper on the tor network describing how MimbleWimble could work. As the potential for this was realised work was done to make this a reality. One of these projects is Grin, which is a minimalistic implementation of MimbleWimble. 

## Contents

- Mimblewimble-Grin Blockchain Protocol Overview
  - Mimblewimble protocol overview
    - Commitments
    - Cut-through and Pruning
    - Grin blocks
  - Trustless transactions
  - Contracts
    - Time-locked
    - Multisig
  - Atomic swaps
  - References

## Mimblewimble protocol overview

## Commitments

MimbleWimble publishes all transaction as confidential transactions.  All inputs, outputs and change are expressed in the following form:

```
r*G + v*H
```

where G and H are elliptic curves, r a private key used as a blinding factor, v the value and * being Elliptic-curve cryptography (ECC) multiplication.  

An example transaction can be expressed as input = output + change. 

```
(ri*G + vi*H) = (ro*G + vo*H) + (rc*G + vc*H)
```

But this requires that

```
ri = ro + rc
```

A more detail explanation of how MimbleWimble works can be found in the Grin GitHub documents [2].

### Cut-through and Pruning

#### 	Cut-through

Grin includes something that is called cut-through in the transaction pool, and this removes outputs in the transaction pool that are already spent as new inputs. Using the fact that every transaction in a block should sum to zero. This is shown below:

```
outputs - inputs = kernel_excess + (part of)kernel_offset
```

The kernel offset is used to hide which kernel belongs to which transaction and we only have a summed kernel offset stored in the header of each block. 

We don't have to record these transactions inside the block, although we still have to record the kernel as the kernel proof transfer of ownership to make sure that the whole block sums to zero, this is expressed in the formula below:

```
sum(outputs) - sum(inputs) = sum(kernel_excess) + kernel_offset
```

An example of cut-through can be seen below:

```
I1(x1) --- O1
        |- O2

I2(x2,O2) --- O3

I3(O3) --- O4
        |- O5
```

After cut-through

```
I1(x1) | O1
I2(x2) | O4
       | O5
```

In the diagrams: I are new inputs, X are inputs from previous blocks and O are outputs. 

This causes that MimbleWimble blocks be much smaller than normal bitcoin blocks as the cut-through transactions are not listed under inputs and outputs anymore. In practice after this we can still see there was a transaction because the kernel excess still remains, but the actual hidden values are not recorded. 

#### 	Pruning

Pruning takes this same concept but goes into past blocks. So, if an output in a previous block gets spent it will be removed from that block. Pruning removes the leaves from the Merkle Mountain Range (MMR) as well. Thus, it allows the ledger to be small and scalable. According to the Grin team [3] assuming 10 million transactions with 100&nbsp;000 unspent outputs the ledger will be roughly 130GB, this can be divided into the following parts:

- 128GB of transaction data (inputs and outputs).
- 1 GB of transaction proof data.
- 250MB of block headers.

The total storage requirements can be reduced if cut-through and pruning is applied, the ledger will shrink to approximately 1.8GB and will result in the following:

- 1 GB of transaction proof data.
- UTXO size of 520MB.
- 250MB of block headers.

### Grin blocks

The grin block contains the following data:

1. Transaction outputs, which include for each output:
   1. A Pedersen commitment (33 bytes).
   2. A range proof (over 5KB at this time).
2. Transaction inputs, which are just output references (32 bytes).
3. Transaction fees in clear text
4. Transaction "proofs", which include for each transaction:
   1. The excess commitment sum for the transaction (33 bytes).
   2. A signature generated with the excess (71 bytes average).
5. A block header includes Merkle trees and proof of work (about 250 bytes).

## Trustless transactions

Schnorr signatures have been done in Tari Labs University (TLU), please have a look there for a more detailed explanation [5]. 

Since Grin transactions are obscured by Pedersen Commitments, there is no prove that money was actually transferred. To solve this problem, we require the receiver to collaborate with the sender in building a transaction and more specifically the kernel signature [4].

When Alice wants to pay Bob, the transaction will be performed using the following steps::

1. Alice selects her inputs and her change. The sum of all blinding factors (change output minus inputs) is `rs`.
2. Alice picks a random nonce ks and sends her partial transaction, `ks*G` and `rs*G` to Bob.
3. Bob picks his own random nonce `kr` and the blinding factor for his output `rr`. Using `rr`, Bob adds his output to the transaction.
4. Bob computes the message `M = fee | lock_height`, the Schnorr challenge `e = SHA256(M | kr*G + ks*G | rr*G + rs*G)` and finally his side of the signature `sr = kr + e * rr`.
5. Bob sends `sr`, `kr*G` and `rr*G` to Alice.
6. Alice computes `e` just like Bob did and can check that `sr*G = kr*G + e*rr*G`.
7. Alice sends her side of the signature `ss = ks + e * rs` to Bob.
8. Bob validates `ss*G` just like Alice did for `sr*G` in step 5 and can produce the final signature `s = (ss + sr, ks*G + kr*G)` as well as the final transaction kernel including `s` and the public key `rr*G + rs*G`.

### 

## Contracts

### Time-locked

#### 	Absolute

In a normal Grin transaction the signature [4] just the normal fee gets signed as the message. But to get an absolute time locked transaction the message can be modified taking the block height and appending the fee to that. A block with a kernel that includes a lock height greater than the current block height is then rejected.

#### 	Relative

Taking into account how an absolute time-locked transaction is constructed the same idea can be extended by taking the relative block height and not the absolute height, but also adding a specific kernel commitment. In this way the signature references a specific block as height. The same principle counts as with absolute time-locked transactions in that a block with a kernel containing a relative time-locked transaction that has not passed is rejected. 

### Multisig

Multisigs (Multi-signatures) are also known as N-of-M signatures, and this means that N amount out of M amount of peers need to agree before a transaction can be spend.

When Bob and Alice [4] wants to do a 2-of-2 multisig contract, the contract can be done with the following steps:

1. Bob picks a blinding factor `rb` and sends `rb*G` to Alice.
2. Alice picks a blinding factor `ra` and builds the commitment `C = ra*G + rb*G + v*H`. She sends the commitment to Bob.
3. Bob creates a range proof for `v` using `C` and `rb` and sends it to Alice.
4. Alice generates her own range proof, aggregates it with Bob, finalising the multiparty output `Oab`.
5. The kernel is built following the same procedure as for Trustless Transactions.

We observe that the output `Oab`, is unknown to both party because neither knows the whole blinding factor. To be able to build a transaction spending `Oab`, someone would need to know `ra + rb` to produce a kernel signature. To produce the original spending kernel, Alice and Bob need to collaborate.

## Atomic swaps

Atomic swaps can be used to exchange coins from different blockchains in a trustless environment. In the Grin documentation this is handled in length by contracts documentation [4] and in the contracts ideas documentation [6]. In practice there has already been an atomic swap between Grin and Ethureum [7], but this used the Grin test-net with modified Grin code as the release version Grindid not yet support the contracts required at that time. TLU has a section about Atomic swaps [5].

Atomic swaps work with 2-of-2 multisig contracts, one public key being Alice's, the second being the hash of a preimage that Bob has to reveal. Consider public key derivation `x*G` to be the hash function and by Bob revealing `x`, Alice can then produce an adequate signature proving she knows `x` (in addition to her own private key).

Alice will swap Grin with Bob for Bitcoin. We assume Bob created an output on the Bitcoin blockchain that allows spending with by Alice if she learns a hash pre-image `x`, or by Bob after time `Tb`. Alice is ready to send her Grin to Bob if he reveals `x`.

Alice will send her Grin to a multiparty timelock contract with a refund time `Ta < Tb`. To send the 2-of-2 output to Bob and execute the swap, Alice and Bob start as if they were building a normal trustless transaction.

1. Alice picks a random nonce `ks` and her blinding sum `rs` and sends `ks*G` and `rs*G` to Bob.
2. Bob picks a random blinding factor `rr` and a random nonce `kr`. However, this time, instead of simply sending `sr = kr + e * rr` with his `rr*G` and `kr*G`, Bob sends `sr' = kr + x + e * rr` as well as `x*G`.
3. Alice can validate that `sr'*G = kr*G + x*G + rr*G`. She can also check that Bob has money locked with `x*G` on the other chain.
4. Alice sends back her `ss = ks + e * xs` as she normally would, now that she can also compute `e = SHA256(M | ks*G + kr*G)`.
5. To complete the signature, Bob computes `sr = kr + e * rr` and the final signature is `(sr + ss, kr*G + ks*G)`.
6. As soon as Bob broadcasts the final transaction to get his Grin, Alice can compute `sr' - sr` to get `x`.

Prior to completing the atomic swap, Bob needs to know Alice's public key. Bob would then create an output on the Bitcoin blockchain with a 2-of-2 multisig similar to `alice_pubkey secret_pubkey 2 OP_CHECKMULTISIG`. This should be wrapped in an `OP_IF` so Bob can get his money back after an agreed-upon time. All of this can even be wrapped in a  Pays To Script Hash (P2SH) . Here `secret_pubkey` is `x*G` from the previous section.

To verify the output, Alice would take `x*G`, recreate the bitcoin script, hash it and check that her hash matches what's in theP2SH  (step 2 in previous section). Once she gets `x` (step 6), she can build the 2 signatures necessary to spend the 2-of-2, having both private keys, and get her bitcoin.

## References

1. Maxwell, G. (2017) *Confidential Transactions*. Available at: https://people.xiph.org/~greg/confidential_values.txt (Accessed: 24 October 2018).
2. Simon B and Et al (2018) *Grin document structure*. Available at: https://github.com/mimblewimble/grin/blob/master/doc/table_of_contents.md (Accessed: 24 October 2018).
3. Peverell, I. and et al (2016) *Pruning Blockchain Data*. Available at: https://github.com/mimblewimble/grin/blob/master/doc/pruning.md (Accessed: 26 October 2018).
4. Peverell, I. and Et al (2018) *Contracts*. Available at: https://github.com/mimblewimble/grin/blob/master/doc/contracts.md (Accessed: 26 October 2018).
5. Tari Labs (2018) *Tari Labs University*. Available at: https://tari-labs.github.io/tari-university/ (Accessed: 27 October 2018).
6. Sceller, Q. Le (2018) *Contract ideas*. Available at: https://github.com/mimblewimble/grin/blob/master/doc/contract_ideas.md (Accessed: 27 October 2018).
7. Jasper (2018) *First Grin atomic swap!* Available at: https://medium.com/grinswap/first-grin-atomic-swap-a16b4cc19196 (Accessed: 27 October 2018).

## Contributors

https://github.com/SWvheerden

https://github.com/neonknight64

https://github.com/hansieodendaal

