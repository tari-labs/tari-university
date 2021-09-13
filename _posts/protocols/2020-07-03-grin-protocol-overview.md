---
layout: post
title:  Mimblewimble-Grin Blockchain Protocol Overview
date:   2020-07-03 15:00:00 +0300
image:  '/images/banner-07.jpg'
category: protocols
tags:   [protocols, mimblewimble, grin]
featured:
excerpttext: Depending on whom you ask, Mimblewimble is either a tongue-tying curse or a blockchain protocol designed to be private and scalable.
---

## Table of Contents

- [Introduction](#introduction)
- [Mimblewimble Protocol Overview](#mimblewimble-protocol-overview)
  - [Commitments](#commitments)
  - [Cut-through and Pruning](#cut-through-and-pruning)
    - [Cut-through](#cut-through)
    - [Pruning](#pruning)
  - [Grin Blocks](#grin-blocks)
- [Trustless Transactions](#trustless-transactions)
- [Contracts](#contracts)
  - [Time-locked](#time-locked)
    - [Absolute](#absolute)
    - [Relative](#relative)
  - [Multisig](#multisig)
- [Atomic Swaps](#atomic-swaps)
- [References](#references)
- [Appendices](#appendices)
  - [Appendix A: Example of Grin Block](#appendix-a-example-of-grin-block)
- [Contributors](#contributors)



## Introduction

Depending on whom you ask, Mimblewimble is either a tongue-tying curse or a blockchain protocol designed to be private
and scalable. The transactions in Mimblewimble are derived from confidential transactions by Greg Maxwell [[1]], which
in turn are based on the Pedersen commitment scheme. On 19&nbsp;July&nbsp;2016, Tom Elvis Jedusor left a white paper
on the tor network describing how Mimblewimble could work. As the potential for this was realized, work was done to make
this a reality. One of these projects is Grin, which is a minimalistic implementation of Mimblewimble. Further
information can be found in [[2]] and [[3]].



## Mimblewimble Protocol Overview

### Commitments

Mimblewimble publishes all transactions as confidential transactions. All inputs, outputs and change are expressed in
the following form:

​	$ r \cdot G + v \cdot H $

where $ G $ and $ H $ are elliptic curves, $ r $ a private key used as a blinding factor, $ v $ the value and
"$ \cdot $" is Elliptic-curve cryptography (ECC) multiplication.  

An example transaction can be expressed as input = output + change.

​	$ (r_i \cdot G + v_i \cdot H) = (r_o \cdot G + v_o \cdot H) + (r_c \cdot G + v_c + \cdot H) $  

But this requires that

​	$ r_i = r_o + r_c $

A more detailed explanation of how Mimblewimble works can be found in the Grin GitHub documents [[4]].

### Cut-through and Pruning

#### 	Cut-through

Grin includes something called "cut-through" in the transaction pool. Cut-through removes outputs from the transaction
pool, which have already been spent as new inputs, using the fact that every transaction in a block should sum to zero.
This is shown below:

​	$ output - inputs = kernel_-excess +(part \mspace{3mu} of)kernel_- offset $

The kernel offset is used to hide which kernel belongs to which transaction and we only have a summed kernel offset
stored in the header of each block.

We don't have to record these transactions inside the block, although we still have to record the kernel as the kernel
proof transfer of ownership to make sure that the whole block sums to zero, as expressed in the following formula:

​	$ sum(ouputs) - sum(inputs) = sum(kernel_-excess) + kernel_-offset $

An example of cut-through follows:

```text
 I1(x1)    +---> O1
           +---> O2

 I2(x2,O2) +---> O3

 I3(O3)    +---> O4
           +---> O5
```

After cut-through:

```text
 I1(x1)    +---> O1
 I2(x2)    +---> O4
           +---> O5
```

In the preceding examples, "I" represents new inputs, "X" represents inputs from previous blocks and "O" represents
outputs.

This causes Mimblewimble blocks to be much smaller than normal bitcoin blocks, as the cut-through transactions are
no longer listed under inputs and outputs. In practice, after this we can still see there was a transaction, because
the kernel excess still remains, but the actual hidden values are not recorded.

#### 	Pruning

Pruning takes this same concept but goes into past blocks. Therefore, if an output in a previous block gets spent, it
will be removed from that block. Pruning removes the leaves from the Merkle Mountain Range (MMR) as well. Thus, it
allows the ledger to be small and scalable. According to the Grin team [[3]], assuming 10&nbsp;million transactions with
100,000&nbsp; unspent outputs, the ledger will be roughly 130GB, which can be divided into the following parts:

- 128GB transaction data (inputs and outputs);
- 1GB transaction proof data;
- 250MB block headers.

The total storage requirements can be reduced if cut-through and pruning are applied. The ledger will shrink to
approximately 1.8GB and will result in the following:

- 1GB transaction proof data;
- UTXO size 520MB;
- 250MB block headers.

### Grin Blocks

The grin block contains the following data:

1. Transaction outputs, which include for each output:
   - A Pedersen commitment (33&nbsp;bytes).
   - A range proof (over 5KB at this time).
2. Transaction inputs, which are just output references (32&nbsp;bytes).
3. Transaction fees, in clear text.
4. Transaction "proofs", which include for each transaction:
   - The excess commitment sum for the transaction (33&nbsp;bytes).
   - A signature generated with the excess (71&nbsp;bytes average).
5. A block header that includes Merkle trees and proof of work (approximately 250&nbsp;bytes).

The Grin header:

| Header Field        | Description                               |
| ------------------- | ----------------------------------------- |
| Hash                | Unique hash of block                      |
| Version             | Grin version                              |
| Previous Block      | Unique hash of previous block             |
| Age                 | Time the block was mined                  |
| Cuckoo Solution     | The wining cuckoo solution                |
| Difficulty          | Difficulty of the solved cuckoo           |
| Target Difficulty   | Difficulty of this block                  |
| Total Difficulty    | Total difficulty of mined chain up to age |
| Total Kernel Offset | Kernel offset                             |
| Nonce               | Random number for cuckoo                  |
| Block Reward        | Coinbase + fee reward for block           |

The rest of the block contains a list of kernels, inputs and outputs. [Appendix A](#appendix-a-example-of-grin-block)
contains an example of a grin block.



## Trustless Transactions

Schnorr signatures have been done in Tari Labs University (TLU). Please look
[here](/cryptography/introduction-schnorr-signatures) for a more detailed explanation [[7]].

Since Grin transactions are obscured by Pedersen Commitments, there is no proof that money was actually transferred. To
solve this problem, we require the receiver to collaborate with the sender in building a transaction and, more
specifically, the kernel signature [[4]].

When Alice wants to pay Bob, the transaction will be performed using the following steps:

1. Alice selects her inputs and her change. The sum of all blinding factors (change output minus inputs) is $ r_s $.
2. Alice picks a random nonce ks and sends her partial transaction, $ k_s\cdot G $ and $ r_s\cdot G $ to Bob.
3. Bob picks his own random nonce $ k_r $ and the blinding factor for his output $ r_r $. Using $ r_r $, Bob adds his
    output to the transaction.
4. Bob computes the following:
   - message $ m = fee \Vert lock_-height $;
   - Schnorr challenge $ e = SHA256(m \Vert k_r \cdot G + k_s\cdot  G \Vert r_r\cdot G + r_s\cdot G) $; and
   - his side of the signature, $ s_r = k_r + e\cdot G $.
5. Bob sends $ s_r $ and $ k_r\cdot G $ and $ r_r\cdot G $  to Alice.
6. Alice computes $ e $, just like Bob did, and can check that $ s_r\cdot G = k_r\cdot G + e\cdot r_r \cdot G $.
7. Alice sends her side of the signature, $ s_s = k_s + e\cdot r_s $, to Bob.
8. Bob validates $ s_s\cdot G $, just like Alice did for $ s_r\cdot G $ in step 5, and can produce the final signature
    $ sig = (s_s + s_r , \mspace{6mu} k_s\cdot G + k_s\cdot G) $ as well as the final transaction kernel, including $ sig $ and the public key
    $ r_r\cdot G + r_s\cdot G$.


## Contracts

### Time-locked

#### 	Absolute

In a normal Grin transaction, the signature [[4]], just the normal fee, gets signed as the message. But to get an absolute
time-locked transaction, the message can be modified taking the block height and appending the fee to that. A block with
a kernel that includes a lock height greater than the current block height is then rejected.

​	$ m = fee \Vert h $

#### 	Relative

Taking into account how an absolute time-locked transaction is constructed, the same idea can be extended by taking the
relative block height and not the absolute height, but also adding a specific kernel commitment. In this way, the
signature references a specific block as height. The same principle counts as with absolute time-locked transactions in
that a block with a kernel containing a relative time-locked transaction that has not passed, is rejected.

​	$ m = fee \Vert h \Vert c $

### Multisig

Multi-signatures (multisigs) are also known as N-of-M signatures. This means that N amount out of M amount of peers
need to agree before a transaction can be spent.

When Bob and Alice [[6]] want to do a 2&#8209;of&#8209;2 multisig contract, the contract can be done using the following
steps:

1. Bob picks a blinding factor $ r_b $ and sends $ r_b\cdot G $ to Alice.
2. Alice picks a blinding factor $ r_a $ and builds the commitment $ C= r_a\cdot G + r_b\cdot G + v\cdot H $; she
sends the commitment to Bob.
3. Bob creates a range proof for $ v $ using $ C $ and $ r_b $, and sends it to Alice.
4. Alice generates her own range proof and aggregates it with Bob, finalizing the multiparty output $ O_{ab} $.
5. The kernel is built following the same procedure as used with [Trustless Transactions](#trustless-transactions).

We observe that the output $ O_{ab} $ is unknown to both parties, because neither knows the whole blinding factor. To be
able to build a transaction spending $ O_{ab} $, someone would need to know $ r_a + r_b $ to produce a kernel
signature. To produce the original spending kernel, Alice and Bob need to collaborate.

## Atomic Swaps

Atomic swaps can be used to exchange coins from different blockchains in a trustless environment. In the Grin
documentation, this is handled in the contracts documentation [[6]] and in the contract ideas documentation
[[8]]. In practice, there has already been an atomic swap between Grin and Ethereum [[9]], but this only used the Grin
testnet with a modified Grin implementation, as the release version of Grin did not yet support the required contracts.
TLU has a section about [Atomic Swaps](/protocols/AtomicSwaps) [[7]].

Atomic swaps work with 2&#8209;of&#8209;2 multisig contracts, one public key being Alice's, the other being the hash of
a preimage
that Bob has to reveal. Consider public key derivation $ x\cdot G $ to be the hash function and by Bob revealing
$ x $, Alice can then produce an adequate signature, proving she knows $ x $ (in addition to her own private key).

Alice will swap Grin with Bob for Bitcoin. We assume Bob created an output on the Bitcoin blockchain that allows
spending by Alice if she learns a hash preimage $ x $, or by Bob after time $ T_b $. Alice is ready to send her Grin
to Bob if he reveals $ x $.

Alice will send her Grin to a multiparty timelock contract with a refund time $ T_a < T_b $. To send the
2&#8209;of&#8209;2 output
to Bob and execute the swap, Alice and Bob start as if they were building a normal trustless transaction:

1. Alice picks a random nonce $ k_s $ and her blinding sum $ r_s $ and sends $ k_s\cdot G $ and $ r_s\cdot G $ to Bob.
2. Bob picks a random blinding factor $ r_r $ and a random nonce $ k_r $. However, this time, instead of simply sending
$ s_r = k_r + e\cdot r_r $ with his $ r_r\cdot G $ and $ k_r\cdot G $, Bob sends $ s_r' = k_r + x + e\cdot r_r $ as
well as $ x\cdot G $.
3. Alice can validate that $ s_r'\cdot G = k_r\cdot G + x\cdot G + r_r\cdot G $. She can also check that Bob has money
locked with $ x\cdot G $ on the other chain.
4. Alice sends back her $ s_s = k_s + e\cdot x_s $ as she normally would, now that she can also compute
$ e = SHA256(m \Vert k_s\cdot G+k_r\cdot G) $.
5. To complete the signature, Bob computes $ s_r = k_r + e\cdot r_r $ and the final signature is
$ (s_r + s_s, \mspace{6mu} k_r\cdot G + k_s\cdot G) $.
6. As soon as Bob broadcasts the final transaction to get his Grin, Alice can compute $ s_r' - s_r $ to get $ x $.

Prior to completing the atomic swap, Bob needs to know Alice's public key. Bob would then create an output on the
Bitcoin blockchain with a 2&#8209;of&#8209;2 multisig similar to `alice_pubkey secret_pubkey 2 OP_CHECKMULTISIG`. This
should be
wrapped in an `OP_IF` so Bob can get his money back after an agreed-upon time. All of this can even be wrapped in a
Pays To Script Hash (P2SH). Here, `secret_pubkey` is $x\cdot G$ from the previous section.

To verify the output, Alice would take $x\cdot G$, recreate the bitcoin script, hash it and check that her hash matches
what's in theP2SH (step&nbsp;2 in the [Multisig](#multisig) section). Once she gets $x$ (step&nbsp;6), she can build the
two signatures necessary
to spend the 2&#8209;of&#8209;2, having both private keys, and get her bitcoin.

## References

[[1]]  G. Maxwell, "Confidential Transactions", 2017 [online].  Available:
<https://people.xiph.org/~greg/confidential_values.txt>. Accessed: 2018&#8209;10&#8209;24.

[1]: https://people.xiph.org/~greg/confidential_values.txt
"Original confidential transaction paper"

[[2]] P. Robinson, H. Odendaal, S. W. van Heerden and A. Zaidelson, "Grin vs. BEAM, a Comparison", 2018 [online].
Available:&nbsp;<https://tari&#8209;labs.github.io/tari-university/protocols/grin-beam-comparison/MainReport.html#grin-vs-beam-a-comparison>.
Accessed: 2018&#8209;10&#8209;08.

[2]: https://tari-labs.github.io/tari-university/protocols/grin-beam-comparison/MainReport.html#grin-vs-beam-a-comparison
"Grin vs. BEAM, a Comparison"

[[3]] Y. Roodt, H. Odendaal, S. W. van Heerden, R. Robinson and A. Kamal, "Grin Design Choice Criticisms - Truth or
Fiction", 2018 [online]. Available:
<https://tari-labs.github.io/tari-university/protocols/grin-design-choice-criticisms/MainReport.html>.
Accessed: 2018&#8209;10&#8209;08.

[3]: https://tari-labs.github.io/tari-university/protocols/grin-design-choice-criticisms/MainReport.html
"Grin Design Choice Criticisms"

[[4]] B. Simon et al., "Grin Document Structure", 2018 [online]. Available:
<https://github.com/mimblewimble/grin/blob/master/doc/table_of_contents.md>. Accessed: 2018&#8209;10&#8209;24).

[4]: https://github.com/mimblewimble/grin/blob/master/doc/table_of_contents.md
"Main Grin Document Structure"

[[5]] I. Peverell et al., "Pruning Blockchain Data", 2016 [online]. Available:
<https://github.com/mimblewimble/grin/blob/master/doc/pruning.md>. Accessed: 2018&#8209;10&#8209;26.

[5]: https://github.com/mimblewimble/grin/blob/master/doc/pruning.md
"Grin Pruning"

[[6]] I. Peverell et al., "Contracts", 2018 [online]. Available:
<https://github.com/mimblewimble/grin/blob/master/doc/contracts.md>.
Accessed: 2018&#8209;10&#8209;26.

[6]: https://github.com/mimblewimble/grin/blob/master/doc/contracts.md
"Grin Contracts"

[[7]] "Tari Labs University".  Tari Labs, 2018 [online]. Available: <https://tari-labs.github.io/tari-university/>.
Accessed: 2018&#8209;10&#8209;27.

[7]: https://tari-labs.github.io/tari-university/
"Tari Labs University"

[[8]] Q. Le Sceller, "Contract Ideas", 2018 [online]. Available:
<https://github.com/mimblewimble/grin/blob/master/doc/contract_ideas.md>. (Accessed: 2018&#8209;10&#8209;27.)

[8]: https://github.com/mimblewimble/grin/blob/master/doc/contract_ideas.md
"Grin Contract Ideas Discussion Document"

[[9]] Jasper, "First Grin Atomic Swap!" (2018) [online]. Available:
<https://medium.com/grinswap/first-grin-atomic-swap-a16b4cc19196>. Accessed: 2018&#8209;10&#8209;27.

[9]: https://medium.com/grinswap/first-grin-atomic-swap-a16b4cc19196
"First ever Grin atomic swap"



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

`4 x Inputs`

| No.  | Commit                                                             |
| ---- | ------------------------------------------------------------------ |
| 1    | 0898a4b53964ada66aa16de3d44ff02228c168a23c0bd71b162f4366ce0dae24b0 |
| 2    | 09a173023e9c39c923e626317ffd384c7bce44109fea91a9c142723bfa700fce27 |
| 3    | 086e0d164fe92d837b5365465a6b37af496a4f8520a2c1fccbb9f736521631ba96 |
| 4    | 087a00d61f8ada399f170953c6cc7336c6a0b22518a8b02fd8dd3e28c01ee51fdb |

`5 x Outputs`


| No.  | Output Type   |Commit                                    | Spent |
| ---- | ---------------------------------------------------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------------- |
| 1    | Transaction |09eac33dfdeb84da698c6c17329e4a06020238d9bb31435a4abd9d2ffc122f6879|False|
| 2    | Transaction |0860e9cf37a94668c842738a5acc8abd628c122608f48a50bbb7728f46a3d50673|False|
| 3    | Coinbase |08324cdbf7443b6253bb0cdf314fce39117dcafbddda36ed37f2c209fc651802d6|False|
| 4    | Transaction |0873f0da4ce164e2597800bf678946aad1cd2d7e2371c4eed471fecf9571942b4f|False|
| 5    | Transaction |09774ee77edaaa81b3c6ee31f471f014db86c4b3345f739472cb12ecc8f40401df|False|

`3 x Kernels`

| No.  | Features   |Fee                                    | Lock Height |
| ---- | ---------------------------------------------------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------------- |
| 1    | DEFAULT_KERNEL |6 mg|7477|
| 2    | DEFAULT_KERNEL |8 mg|7477|
| 3    | COINBASE_KERNEL |0 grin|7482|

Apart from the header information, we can only see that this block contains two transactions from the two kernels
present. Between these two transactions, we only know that there were four inputs and four outputs. Because of the way
in which Mimblewimble obfuscates the transaction, we don't know the values or which input and output belong to which
transaction.



## Contributors

- <https://github.com/SWvheerden>
- <https://github.com/neonknight64>
- <https://github.com/hansieodendaal>
- <https://github.com/anselld>
