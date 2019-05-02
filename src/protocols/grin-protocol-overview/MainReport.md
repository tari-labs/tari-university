# Mimblewimble-Grin Blockchain Protocol Overview

- [Introduction](#introduction)
- [Contents](#contents)
- [Mimblewimble protocol overview](#mimblewimble-protocol-overview)
  - [Commitments](#commitments)
  - [Cut-through and pruning](#cut-through-and-pruning)
  - [Grin blocks](#grin-blocks)
- [Trustless transactions](#trustless-transactions)
- [Contracts](#contracts)
  - [Time-locked](#time-locked)
  - [Multisig](#multisig)
- [Atomic swaps](#atomic-swaps)
- [References](#references)
- [Appendices](#appendices)
  - [Appendix A: Example of Grin Block](#example-of-grin-block)
- [Contributors](#contributors)



## Introduction

Depending on who you ask, Mimblewimble is either a tongue-tying curse or blockchain protocol designed to be private and scalable. The transactions in Mimblewimble is derived from confidential transactions by Greg Maxwell [[1]], that is in turn based on the Pedersen commitment scheme. On 19 July 2016 someone with the name Tom Elvis Jedusor left a whitepaper on the tor network describing how Mimblewimble could work. As the potential for this was realized work was done to make this a reality. One of these projects is Grin, which is a minimalistic implementation of Mimblewimble.  Further information could be found on [Grin at Grin vs. BEAM, a Comparison](../grin-beam-comparison/MainReport.md) [[2]] and [Grin design choice criticisms - Truth or Fiction](../grin-design-choice-criticisms/MainReport.md) [[3]]. 



## Mimblewimble protocol overview

### Commitments

Mimblewimble publishes all transaction as confidential transactions.  All inputs, outputs and change are expressed in the following form:

​	$ r \cdot G + v \cdot H ​$

where $ G $ and $ H $ are elliptic curves, $ r $ a private key used as a blinding factor, $ v $ the value and "$ \cdot $" being Elliptic-curve cryptography (ECC) multiplication.  

An example transaction can be expressed as input = output + change. 

​	$ (r_i \cdot G + v_i \cdot H) = (r_c \cdot G + v_c \cdot H) + (r_c \cdot G + v_c + \cdot H) $  

But this requires that

​	$ r_i = r_o + r_c $

A more detail explanation of how Mimblewimble works can be found in the Grin GitHub documents [[4]].

### Cut-through and pruning

#### 	Cut-through

Grin includes something that is called cut-through in the transaction pool, and this removes outputs in the transaction pool that are already spent as new inputs. Using the fact that every transaction in a block should sum to zero. This is shown below:

​	$ output - inputs = kernel_-excess +(part \mspace{3mu} of)kernel_- offset $ 

The kernel offset is used to hide which kernel belongs to which transaction and we only have a summed kernel offset stored in the header of each block. 

We don't have to record these transactions inside the block, although we still have to record the kernel as the kernel proof transfer of ownership to make sure that the whole block sums to zero, this is expressed in the formula below:

​	$ sum(ouputs) - sum(inputs) = sum(kernel_-excess) + kernel_-offset $

An example of cut-through can be seen below:

```text
 I1(x1)    +---> O1
           +---> O2

 I2(x2,O2) +---> O3

 I3(O3)    +---> O4
           +---> O5
```

After cut-through

```text
 I1(x1)    +---> O1
 I2(x2)    +---> O4
           +---> O5
```

In the diagrams: I are new inputs, X are inputs from previous blocks and O are outputs. 

This causes that Mimblewimble blocks to be much smaller than normal bitcoin blocks as the cut-through transactions are not listed under inputs and outputs anymore. In practice after this we can still see there was a transaction because the kernel excess still remains, but the actual hidden values are not recorded. 

#### 	Pruning

Pruning takes this same concept but goes into past blocks. So, if an output in a previous block gets spent it will be removed from that block. Pruning removes the leaves from the Merkle Mountain Range (MMR) as well. Thus, it allows the ledger to be small and scalable. According to the Grin team [[3]] assuming 10 million transactions with 100&nbsp;000 unspent outputs the ledger will be roughly 130GB, this can be divided into the following parts:

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
3. Transaction fees, in clear text
4. Transaction "proofs", which include for each transaction:
   1. The excess commitment sum for the transaction (33 bytes).
   2. A signature generated with the excess (71 bytes average).
5. A block header that includes Merkle trees and proof of work (about 250 bytes).

The Grin header:

| Header field        |                                           |
| ------------------- | ----------------------------------------- |
| Hash                | Unique hash of block                      |
| Version             | Grin version                              |
| Previous block      | Unique hash of previous block             |
| Age                 | Time the block was mined                  |
| Cuckoo solution     | The wining cuckoo solution                |
| Difficulty          | Difficulty of the solved cuckoo           |
| Target Difficulty   | Difficulty of this block                  |
| Total difficulty    | Total difficulty of mined chain up to age |
| Total kernal offset | Kernel offset                             |
| Nonce               | Random number for cuckoo                  |
| Block reward        | Coinbase + fee reward for block           |

The rest of the block contains a list of kernels, inputs and outputs. An example of a grin block is shown in the appendix.



## Trustless transactions

Schnorr signatures have been done in Tari Labs University (TLU), please have a look [here](../../cryptography/digital_signatures/introduction.md) for a more detailed explanation [[7]]. 

Since Grin transactions are obscured by Pedersen Commitments, there is no prove that money was actually transferred. To solve this problem, we require the receiver to collaborate with the sender in building a transaction and more specifically the kernel signature [[4]].

When Alice wants to pay Bob, the transaction will be performed using the following steps:

1. Alice selects her inputs and her change. The sum of all blinding factors (change output minus inputs) is $ r_s $.

2. Alice picks a random nonce ks and sends her partial transaction, $ k_s\cdot G $ and $ r_s\cdot G $ to Bob.

3. Bob picks his own random nonce $ k_r $ and the blinding factor for his output $ r_r $. Using $ r_r $ Bob adds his output to the transaction.

4. Bob computes the message $ M= fee \Vert lock_-height $, 

   the Schnorr challenge $ e = SHA256(M \Vert K_r \cdot G + K_s\cdot  G \Vert r_r\cdot G + r_s\cdot G) $ 

   and finally his side of the signature $ s_r = k_r + e\cdot G $ 

5. Bob sends: $ s_r $ and $ k_r\cdot G $ and $ r_r\cdot G $  to Alice.

6. Alice computes $ e $ just like Bob did and can check that $ s_r\cdot G = k_r\cdot G + e\cdot r_r \cdot G $ 

7. Alice sends her side of the signature $ s_s = k_s + e\cdot r_s $  to Bob.

8. Bob validates $ s_s\cdot G $  just like Alice did for $ s_r\cdot G $ in step 5 and can produce the final signature $ s = s_s + s_r , k_s\cdot G + k_s\cdot G$ as well as the final transaction kernel including $ s $ and the public key $ r_r\cdot G + r_s\cdot G$ 



## Contracts

### Time-locked

#### 	Absolute

In a normal Grin transaction the signature [[4]] just the normal fee gets signed as the message. But to get an absolute time locked transaction the message can be modified taking the block height and appending the fee to that. A block with a kernel that includes a lock height greater than the current block height is then rejected.

​	$ M = fee \Vert h $

#### 	Relative

Taking into account how an absolute time-locked transaction is constructed the same idea can be extended by taking the relative block height and not the absolute height, but also adding a specific kernel commitment. In this way the signature references a specific block as height. The same principle counts as with absolute time-locked transactions in that a block with a kernel containing a relative time-locked transaction that has not passed is rejected. 

​	$ M = fee \Vert h \Vert c $

### Multisig

Multi-signatures (Multisigs) are also known as N-of-M signatures, and this means that N amount out of M amount of peers need to agree before a transaction can be spend.

When Bob and Alice [[6]] wants to do a 2-of-2 multisig contract, the contract can be done with the following steps:

1. Bob picks a blinding factor $ r_b $ and sends $ r_b\cdot G $ to Alice.
2. Alice picks a blinding factor $ r_a $  and builds the commitment $ C= r_a\cdot G + r_b\cdot G + v\cdot H $, she sends the commitment to Bob.
3. Bob creates a range proof for $ v $  using $ C $  and $ r_b $  and sends it to Alice.
4. Alice generates her own range proof, aggregates it with Bob, finalizing the multiparty output $ O_{ab} $ 
5. The kernel is built following the same procedure as used with Trustless Transactions.

We observe that the output $ O_{ab} $ , is unknown to both party because neither knows the whole blinding factor. To be able to build a transaction spending $ O_{ab} $, someone would need to know $ r_a + r_b $ to produce a kernel signature. To produce the original spending kernel, Alice and Bob need to collaborate.

## Atomic swaps

Atomic swaps can be used to exchange coins from different blockchains in a trustless environment. In the Grin documentation this is handled in length by the contracts documentation [[6]] and in the contracts ideas documentation [[8]]. In practice there has already been an atomic swap between Grin and Ethereum [[9]], but this only used the Grin test-net with a modified Grin implementation as the release version of Grin did not yet support the required contracts. TLU has a section about [Atomic swaps](../atomic-swaps/AtomicSwaps.md) [[7]].

Atomic swaps work with 2-of-2 multisig contracts, one public key being Alice's, the second being the hash of a preimage that Bob has to reveal. Consider public key derivation $ x\cdot G $ to be the hash function and by Bob revealing $ x $, Alice can then produce an adequate signature proving she knows $ x $  (in addition to her own private key).

Alice will swap Grin with Bob for Bitcoin. We assume Bob created an output on the Bitcoin blockchain that allows spending by Alice if she learns a hash pre-image $ x $, or by Bob after time $ T_b $ . Alice is ready to send her Grin to Bob if he reveals $ x $.

Alice will send her Grin to a multiparty timelock contract with a refund time $ T_a < T_b $. To send the 2-of-2 output to Bob and execute the swap, Alice and Bob start as if they were building a normal trustless transaction.

1. Alice picks a random nonce $ k_s $  and her blinding sum $ r_s $ and sends $ k_s\cdot G $ and $ r_s\cdot G $ to Bob.
2. Bob picks a random blinding factor $ r_r $ and a random nonce $ k_r $. However, this time, instead of simply sending $ s_r = k_r + e\cdot r_r $  with his $ r_r\cdot G $ and $ k_r\cdot G $, Bob sends $ s_r' = k_r + x + e\cdot r_r $ as well as $ x\cdot G $ 
3. Alice can validate that $ s_r'\cdot G = k_r\cdot G + x\cdot G + r_r\cdot G $. She can also check that Bob has money locked with $ x\cdot G $ on the other chain.
4. Alice sends back her $ s_s = k_s + e\cdot x_s $ as she normally would, now that she can also compute $ e = SHA256(M \Vert k_s\cdot G+k_r\cdot G) $
5. To complete the signature, Bob computes $ s_r = k_r + e\cdot r_r $ and the final signature is $ (s_r + s_s, k_r\cdot G + k_s\cdot G) $ 
6. As soon as Bob broadcasts the final transaction to get his Grin, Alice can compute $ s_r' - s_r $ to get $ x $.

Prior to completing the atomic swap, Bob needs to know Alice's public key. Bob would then create an output on the Bitcoin blockchain with a 2-of-2 multisig similar to `alice_pubkey secret_pubkey 2 OP_CHECKMULTISIG`. This should be wrapped in an `OP_IF` so Bob can get his money back after an agreed-upon time. All of this can even be wrapped in a  Pays To Script Hash (P2SH). Here `secret_pubkey` is $x\cdot G$ from the previous section.

To verify the output, Alice would take $x\cdot G$, recreate the bitcoin script, hash it and check that her hash matches what's in theP2SH  (step 2 in previous section). Once she gets $x$ (step 6), she can build the 2 signatures necessary to spend the 2-of-2, having both private keys, and get her bitcoin.

## References

[[1]] *Confidential Transactions*. Maxwell, G. (2017)  Available at: https://people.xiph.org/~greg/confidential_values.txt (Accessed: 24 October 2018).

[1]: https://people.xiph.org/~greg/confidential_values.txt
"Original confidential transaction paper"

[[2]] *Grin vs. BEAM, a Comparison*. Robinson, P. and et al (2018)Available at: https://tari-labs.github.io/tari-university/protocols/grin-beam-comparison/MainReport.html#grin-vs-beam-a-comparison (Accessed: 8 October 2018).

[2]: https://tari-labs.github.io/tari-university/protocols/grin-beam-comparison/MainReport.html#grin-vs-beam-a-comparison
"Grin vs. BEAM a Comparison"

[[3]] *Grin Design Choice Criticisms - Truth or Fiction*. Roodt, Y. and et al (2018) Available at: https://tari-labs.github.io/tari-university/protocols/grin-design-choice-criticisms/MainReport.html (Accessed: 8 October 2018).

[3]: https://tari-labs.github.io/tari-university/protocols/grin-design-choice-criticisms/MainReport.html
"Grin design choice criticisms"

[[4]] *Grin document structure*. Simon B and Et al (2018) Available at: https://github.com/mimblewimble/grin/blob/master/doc/table_of_contents.md (Accessed: 24 October 2018).

[4]: https://github.com/mimblewimble/grin/blob/master/doc/table_of_contents.md
"Main Grin document structure"

[[5]] *Pruning Blockchain Data*. Peverell, I. and et al (2016) Available at: https://github.com/mimblewimble/grin/blob/master/doc/pruning.md (Accessed: 26 October 2018).

[5]: https://github.com/mimblewimble/grin/blob/master/doc/pruning.md
"Grin pruning"

[[6]] *Contracts*. Peverell, I. and Et al (2018) Available at: https://github.com/mimblewimble/grin/blob/master/doc/contracts.md (Accessed: 26 October 2018).

[6]: https://github.com/mimblewimble/grin/blob/master/doc/contracts.md
"Grin contracts"

[[7]] *Tari Labs University*.  Tari Labs (2018) Available at: https://tari-labs.github.io/tari-university/ (Accessed: 27 October 2018).

[7]: https://tari-labs.github.io/tari-university/
"The Tari Labs university"

[[8]] *Contract ideas*. Sceller, Q. Le (2018) Available at: https://github.com/mimblewimble/grin/blob/master/doc/contract_ideas.md (Accessed: 27 October 2018).

[8]: https://github.com/mimblewimble/grin/blob/master/doc/contract_ideas.md
"Grin contract ideas discussion document"

[[9]] *First Grin atomic swap!* Jasper (2018) Available at: https://medium.com/grinswap/first-grin-atomic-swap-a16b4cc19196 (Accessed: 27 October 2018).

[9]: https://medium.com/grinswap/first-grin-atomic-swap-a16b4cc19196
"First ever Grin atomic swap"
## Contributors

https://github.com/SWvheerden

https://github.com/neonknight64

https://github.com/hansieodendaal



## Appendices

### Appendix A: Example of Grin Block

| Hash                | 02cb5e810857266609511699c8d222ed4e02883c6b6d3405c05a3caea9bb0f64 |
| ------------------- | ------------------------------------------------------------ |
| Version             | 1                                                            |
| Previous Block      | [0343597fe7c69f497177248913e6e485f3f23bb03b07a0b8a5b54f68187dbc1d](https://grinexplorer.net/block/0343597fe7c69f497177248913e6e485f3f23bb03b07a0b8a5b54f68187dbc1d) |
| Age                 | 2018-10-23, 08:03:46 UTC                                     |
| Cuckoo Solution     | Size:                                                        |
| Difficulty          | 37,652                                                       |
| Target Difficulty   | 17,736                                                       |
| Total Difficulty    | 290,138,524                                                  |
| Total Kernel Offset | b52ccdf119fe18d7bd12bcdf0642fcb479c6093dca566e0aed33eb538f410fb5 |
| Nonce               | 7262225957146290317                                          |
| Block Reward        | 60 grin                                                      |
| Fees                | 14 mg                                                        |

<details class="black" open="" style="box-sizing: border-box;"><summary style="box-sizing: border-box; touch-action: manipulation; display: list-item; background-color: rgb(153, 153, 153); color: rgb(255, 255, 255); font-weight: bold; line-height: 40px; padding-left: 8px;">Inputs (4)</summary><table class="table table-horizontal-bordered table-hover" style="box-sizing: border-box; border-collapse: collapse; width: 1110px; max-width: 100%; margin-bottom: 1rem; background-color: transparent;"><thead class="thead-light" style="box-sizing: border-box;"><tr style="box-sizing: border-box;"><th style="box-sizing: border-box; text-align: inherit; padding: 0.75rem; vertical-align: bottom; border-top: 1px solid rgb(233, 236, 239); border-bottom: 2px solid rgb(233, 236, 239); color: rgb(73, 80, 87); background-color: rgb(233, 236, 239); border-right-color: rgb(233, 236, 239); border-left-color: rgb(233, 236, 239);">Commit</th></tr></thead><tbody style="box-sizing: border-box;"><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221); font-family: monospace;">0898a4b53964ada66aa16de3d44ff02228c168a23c0bd71b162f4366ce0dae24b0</td></tr><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221); font-family: monospace;">09a173023e9c39c923e626317ffd384c7bce44109fea91a9c142723bfa700fce27</td></tr><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221); font-family: monospace;">086e0d164fe92d837b5365465a6b37af496a4f8520a2c1fccbb9f736521631ba96</td></tr><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221); font-family: monospace;">087a00d61f8ada399f170953c6cc7336c6a0b22518a8b02fd8dd3e28c01ee51fdb</td></tr></tbody></table></details>

<details class="black" open="" style="box-sizing: border-box;"><summary style="box-sizing: border-box; touch-action: manipulation; display: list-item; background-color: rgb(153, 153, 153); color: rgb(255, 255, 255); font-weight: bold; line-height: 40px; padding-left: 8px;">Outputs (5)</summary><table class="table table-horizontal-bordered table-hover" style="box-sizing: border-box; border-collapse: collapse; width: 1110px; max-width: 100%; margin-bottom: 1rem; background-color: transparent;"><thead class="thead-light" style="box-sizing: border-box;"><tr style="box-sizing: border-box;"><th style="box-sizing: border-box; text-align: inherit; padding: 0.75rem; vertical-align: bottom; border-top: 1px solid rgb(233, 236, 239); border-bottom: 2px solid rgb(233, 236, 239); color: rgb(73, 80, 87); background-color: rgb(233, 236, 239); border-right-color: rgb(233, 236, 239); border-left-color: rgb(233, 236, 239);">Output Type</th><th style="box-sizing: border-box; text-align: inherit; padding: 0.75rem; vertical-align: bottom; border-top: 1px solid rgb(233, 236, 239); border-bottom: 2px solid rgb(233, 236, 239); color: rgb(73, 80, 87); background-color: rgb(233, 236, 239); border-right-color: rgb(233, 236, 239); border-left-color: rgb(233, 236, 239);">Commit</th><th style="box-sizing: border-box; text-align: inherit; padding: 0.75rem; vertical-align: bottom; border-top: 1px solid rgb(233, 236, 239); border-bottom: 2px solid rgb(233, 236, 239); color: rgb(73, 80, 87); background-color: rgb(233, 236, 239); border-right-color: rgb(233, 236, 239); border-left-color: rgb(233, 236, 239);">Spent</th></tr></thead><tbody style="box-sizing: border-box;"><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">Transaction</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">09eac33dfdeb84da698c6c17329e4a06020238d9bb31435a4abd9d2ffc122f6879</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">False</td></tr><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">Transaction</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">0860e9cf37a94668c842738a5acc8abd628c122608f48a50bbb7728f46a3d50673</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">False</td></tr><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">Coinbase</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">08324cdbf7443b6253bb0cdf314fce39117dcafbddda36ed37f2c209fc651802d6</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">False</td></tr><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">Transaction</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">0873f0da4ce164e2597800bf678946aad1cd2d7e2371c4eed471fecf9571942b4f</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">False</td></tr><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">Transaction</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">09774ee77edaaa81b3c6ee31f471f014db86c4b3345f739472cb12ecc8f40401df</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">False</td></tr></tbody></table></details>

<details class="black" open="" style="box-sizing: border-box;"><summary style="box-sizing: border-box; touch-action: manipulation; display: list-item; background-color: rgb(153, 153, 153); color: rgb(255, 255, 255); font-weight: bold; line-height: 40px; padding-left: 8px;">Kernels (3)</summary><table class="table table-horizontal-bordered table-hover" style="box-sizing: border-box; border-collapse: collapse; width: 1110px; max-width: 100%; margin-bottom: 1rem; background-color: transparent;"><thead class="thead-light" style="box-sizing: border-box;"><tr style="box-sizing: border-box;"><th style="box-sizing: border-box; text-align: inherit; padding: 0.75rem; vertical-align: bottom; border-top: 1px solid rgb(233, 236, 239); border-bottom: 2px solid rgb(233, 236, 239); color: rgb(73, 80, 87); background-color: rgb(233, 236, 239); border-right-color: rgb(233, 236, 239); border-left-color: rgb(233, 236, 239);">Features</th><th style="box-sizing: border-box; text-align: inherit; padding: 0.75rem; vertical-align: bottom; border-top: 1px solid rgb(233, 236, 239); border-bottom: 2px solid rgb(233, 236, 239); color: rgb(73, 80, 87); background-color: rgb(233, 236, 239); border-right-color: rgb(233, 236, 239); border-left-color: rgb(233, 236, 239);">Fee</th><th style="box-sizing: border-box; text-align: inherit; padding: 0.75rem; vertical-align: bottom; border-top: 1px solid rgb(233, 236, 239); border-bottom: 2px solid rgb(233, 236, 239); color: rgb(73, 80, 87); background-color: rgb(233, 236, 239); border-right-color: rgb(233, 236, 239); border-left-color: rgb(233, 236, 239);">Lock Height</th></tr></thead><tbody style="box-sizing: border-box;"><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">DEFAULT_KERNEL</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">6 mg</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">7477</td></tr><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">DEFAULT_KERNEL</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">8 mg</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">7477</td></tr><tr style="box-sizing: border-box;"><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">COINBASE_KERNEL</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">0 grin</td><td style="box-sizing: border-box; padding: 0.75rem; vertical-align: top; border-top: 1px solid rgb(233, 236, 239); border-bottom: 1px solid rgb(221, 221, 221);">7482</td></tr></tbody></table></details>

Apart from the header information we can only see that this block contains 2 transaction from the 2 kernels present. Between those two transaction we only know that there were 4 inputs and 4 outputs. Because of the way Mimblewimble obfuscates the transaction we don't know the values or which input and output belongs to which transaction. 