# Mimblewimble Multiparty Bulletproof UTXO

- [Introduction](#introduction)
- [Notation Used](#notation-used)
- [Background](#background)
  - [Bitcoin $ m\text{-of-}n $ Multisig in a Nutshell](#bitcoin--mtext-of-n--multisig-in-a-nutshell)
  - [Security of the Mimblewimble Blockchain](#security-of-the-mimblewimble-blockchain)
  - [Secure Sharing Protocol](#secure-sharing-protocol)
- [Mimblewimble $ n\text{-of-}n $ Multiparty Bulletproof UTXO](#mimblewimble--ntext-of-n--multiparty-bulletproof-utxo)
  - [Simple Sharing Protocol](#simple-sharing-protocol)
  - [Setting up the Multiparty Funding Transaction](#setting-up-the-multiparty-funding-transaction)
  - [Creating the Multiparty Bulletproof](#creating-the-multiparty-bulletproof)
    - [Utilizing Bulletproofs MPC Protocol](#utilizing-bulletproofs-mpc-protocol)
    - [Utilizing Grin's Shared Bulletproof Computation](#utilizing-grins-shared-bulletproof-computation)
- [Mimblewimble $ m\text{-of-}n $ Multiparty Bulletproof UTXO](#mimblewimble--mtext-of-n--multiparty-bulletproof-utxo)
  - [Utilizing Shamir's Secret Sharing](#utilizing-shamirs-secret-sharing)
  - [Multiple Rounds Scheme](#multiple-rounds-scheme)
- [Comparison to Bitcoin](#comparison-to-bitcoin)
- [Conclusions, Observations and Recommendations](#conclusions-observations-and-recommendations)
- [References](#references)
- [Appendices](#appendices)
  - [Appendix A: ???](#appendix-a-???)
- [Contributors](#contributors)



## Introduction

In [Mimblewimble](../mimblewimble-1/MainReport.md) the concept of a Bitcoin type multi-signature (multisig) applied to an Unspent Transaction Output (UTXO) does not really exist. 

In Bitcoin, multisig payments are usually combined with Pay to Script Hash (P2SH) functionality as a means to send funds to a P2SH payment address, and then to manage its expenditure from there. The redeem script itself sets the conditions that must be fulfilled in order for the UTXOs linked to the P2SH payment address to be spent [[[1]], [[2]]). 

Unlike Bitcoin, Mimblewimble transactions do not involve payment addresses, as all transactions are confidential. The only requirement for a Mimblewimble UTXO to be spent is the ability to open (or unlock) the [Pederson Commitment](../../cryptography/bulletproofs-protocols/MainReport.md#pedersen-commitments-and-elliptic-curve-pedersen-commitments) that contain the tokens, and it does not require an "owner" signature. A typical Mimblewimble UTXO looks like this [[9]]:

```Text
08c15e94ddea81e6a0a31ed558ef5e0574e5369c4fcba92808fe992fbff68884cc
```

Another fundamental difference is that for any Mimblewimble transaction all parties, that is all senders and all receivers, must interact to conclude the transaction.



## Notation Used

This section gives the general notation of mathematical expressions used. It provides important pre-knowledge for the remainder of the report.

- All Pederson Commitments will be of the [elliptic derivative]((../../cryptography/bulletproofs-protocols/MainReport.md#pedersen-commitments-and-elliptic-curve-pedersen-commitments)) depicted by $  C(v,k) = (vH + kG)  $ with $ v $ being the value committed to and $ k $ being the blinding factor.
- Scalar multiplication will be depicted by "$ \cdot $", as an example $ e \cdot (vH + kG) = e \cdot vH + e \cdot kG  $.
- A Pederson Commitment to the value of $ 0 $ will be depicted by $ C(0,k) = (0H + kG) = (kG) = (\mathbf{0}) $.
- Let $ \text{H}_{s}(arg) $ be a collision-resistant hash function used in a sharing protocol where $ arg $ is the value being committed to.



## Background



### Bitcoin $ m\text{-of-}n $ Multisig in a Nutshell

Multiple use cases of $ m\text{-of-}n $ multisig applications exist, for example a $ 1\text{-of-}2 $ petty cash account, or a $ 2\text{-of-}2 $ two-factor authentication wallet, or a $ 2\text{-of-}3 $ board of directors account, etc [[3]]. A typical 2-of-3 Bitcoin P2SH multisig redeem script (where any 2 of the 3 predefined public keys must sign the transaction) has the following form:

```Text
redeemScript     = <OP_2> <A pubkey> <B pubkey> <C pubkey> <OP_3> OP_CHECKMULTISIG
```

The P2SH payment address is the result of the redeem script double hashed with SHA-256 and RIPEMD-160 and then Base58Check encoded with a prefix of `0x05`:

```Text
redeemScriptHash = RIPEMD160(SHA256(redeemScript))
P2SHAddress      = base58check.Encode("05", redeemScriptHash)
```

Multiple payments can now be sent to the P2SH payment address. A generic funding transaction's output script for the P2SH payment address has the following form, irrespective of the redeem script's contents,

```Text
scriptPubKey      =     OP_HASH160 <redeemScriptHash> OP_EQUAL
```


with `OP_HASH160` being the combination of SHA-256 and RIPEMD-160. The 2-of-3 multisig redeem transaction's input script would the have the following form

```Text
scriptSig         =  OP_0 <A sig> <C sig> <redeemScript>
```

and the combined spending and funding transaction script (validation script) would be

```Text
validationScript    = OP_0 <A sig> <C sig> <redeemScript> OP_HASH160 <redeemScriptHash> OP_EQUAL
```

When `validationScript` is executed all values are added to the execution stack in sequence. When opcode `OP_HASH160` is encountered the preceding value `<redeemScript>` is hashed and added to the stack and when opcode `OP_EQUAL` is encountered the previous two values, hash of the `<redeemScript>` and `<redeemScriptHash>`, are compared and removed from the stack if equal. The top of the stack then contains `<redeemScript>`, which is evaluated with the two entries on top of that, `<A sig>` and `<C sig>`. The last value in the stack, `OP_0`, is needed for a glitch in the `OP_CHECKMULTISIG` opcode implementation which makes it pop one more item than are available on the stack ([[1]], [[5]]).

*What is signed?*

Partial signatures are created in the same sequence as the public keys are defined in `redeemScript`. A simplified serialized hexadecimal version of the transaction - consisting of the input transaction ID and UTXO index, amount to be paid, `scriptPubKey` and transaction locktime - is signed. Each consecutive partial signature includes a serialization of the previous partial signature with the simplified transaction data to be signed, creating multiple cross-references in the signed data. This, combined with the public keys, proves the transaction was created by the real owners of the bitcoins in question ([[4]], [[5]], [[6]]).

*How is change redirected to the multisig P2SH?*

Bitcoin transactions are allowed to have multiple recipients, and one of the recipients of the funds from a P2SH multisig transaction can be the the original `P2SHAddress`, thus sending change back to itself. Circular payments to the same addresses are allowed, but it will suffer from a lack of confidentiality. Another way to do this would be to create a new `redeemScript` with a new set of public keys every time a P2SH multisig transaction is done to collect the change, but this will be more complex to manage [[4]].



### Security of the Mimblewimble Blockchain

A [Mimblewimble](../mimblewimble-1/MainReport.md) blockchain relies on two complimenting aspects to provide security; [Pederson Commitments](../../cryptography/bulletproofs-protocols/MainReport.md#pedersen-commitments-and-elliptic-curve-pedersen-commitments) and range proofs (in the form of [Bulletproof range proofs](../../cryptography/bulletproofs-and-mimblewimble/MainReport.md)). Pederson Commitments provide perfectly hiding and computationally binding commitments, i.e. the confidentiality aspect, and range proofs provide assurance that the currency cannot be inflated and that 3<sup>rd</sup> parties cannot lock away ones funds. Due to the fact that Mimblewimble commitments are totally confidential and that ownership cannot be proofed, anyone can try to spend or mess with unspent coins embedded in those commitments. Fortunately any new UTXO requires a range proof, and this is impossible to create if the input commitment cannot be opened.

The role that Bulletproof range proofs play in securing the blockchain can be demonstrated as follows. Let $ C_{A}(v_1 , k_1) $ be the "closed" input UTXO commitment from Alice that a bad actor, Bob, is trying to lock away by adding an additional blinding factor $ k_{x} $ to the commitment. A valid Mimblewimble transaction would have the following form
$$
\begin{aligned} 
C_{locked}(v_1 , k_1 + k_x) - C_{A}(v_1 , k_1) - C_{fee}(v_2 , k_2) + fee &= (\mathbf{0}) \\\\
(v_1 H + (k_1 + k_x) G) -  (v_1 H + k_1 G) - (v_2 H + k_2 G) + fee &= (\mathbf{0})
\end{aligned}
\mspace{70mu} (1)
$$
where the unspent-hiding-blinding commitment from Alice is $ (v_1 H + k_1 G) $ and the value of $ (v_2 H + k_2 G) $ is equal to $ fee $ to be paid to the miner. The newly created commitment $ (v_1 H + (k_1 + k_x) G) $ would be equally unspendable by Alice and Bob because neither of them will know the total blinding factor $ k_1 + k_x $. Fortunately, in order to construct a Bulletproof range proof for the new output $ (v_1 H + (k_1 + k_x) G) $ as required by transaction validation rules, the values of $ v_1 $ and $ k_1 + k_x $ must be known otherwise the prover (i.e. Bob) cannot convince an honest verifier (i.e. the miner) that $ v_1 $ is non-negative (i.e. in the range $ [0,2^n - 1] $).

In the event that Bob can convince Alice that she must create a fund that both of them have signing powers over ($ 2\text{-of-}2 $ multisig), it would theoretically be possible to create the required Bulletproof range proof for relation (1) if they work together to create it.



### Secure Sharing Protocol

Multiple parties working together to create a single transaction that involves multiple steps, need to share information in such a way that what they shared cannot work against them. Each step require a proof, and it should not be possible to replay an individual step's proof in a different context. Merlin transcripts [[7]] is an excellent example of a protocol implementation that achieves just that. For the purposes of this report a simple information sharing protocol is proposed that can be implemented with Merlin transcripts [[7]]. 



## Mimblewimble $ n\text{-of-}n $ Multiparty Bulletproof UTXO



### Simple Sharing Protocol

Alice, Bob and Carol agree to set up a multiparty $ 3\text{-of-}3 $ multisig fund that they can control together. They decide to use a sharing hash function $ val_H = \text{H}_{s}(arg) $ as a handshaking mechanism for all information they need to share. The 1<sup>st</sup> step is to calculate the hash $ val_H $ for the value $ arg $ they want to commit to in sharing and to distribute it to all parties. They then wait until all other parties' commitments have been received. The 2<sup>nd</sup> step is to send the actual value they committed to to all parties and to then verify each value against its commitment. If everything match up they proceed, otherwise they stop and discard everything they have done. 

This ensures that public values for each step is not exposed until all commitments have been received. They will apply this simple sharing protocol to all information they need to share with each other.



### Setting up the Multiparty Funding Transaction

In the transaction Alice, Bob and Carol want to set up, $ C_m $ is the multiparty shared commitment that contains the funds, and $ C_a $, $ C_b $ and $ C_c $ are their respective input contributions. This transaction looks as follows:
$$
\begin{aligned} 
C_m(v_1, k_1 + k_2 + k_3) - C_a(v_a, k_a) - C_b(v_b, k_b) - C_c(v_c, k_c) + fee &= (\mathbf{0}) \\\\
(v_1H + (k_1 + k_2 + k_3)G) - (v_aH + k_aG) - (v_bH + k_bG) - (v_cH + k_cG) + fee &= (\mathbf{0})
\end{aligned}
$$
In order for this scheme to work, they must be able to jointly sign the transaction with a Schnorr signature, and have to keep their portion of the shared blinding factor secret. Each of them creates their own private blinding factor $ k_n $ and shares the public blinding factor $ k_nG $ with the group.
$$
R_{agg} = k_1G + k_2G + k_3G
$$
In addition, each of them create an offset $ \phi_n $ that will be subtracted from their input commitments' blinding factors to prevent someone else linking this transaction's inputs and outputs when analyzing the Mimblewimble block. They then calculate their own total excess that will be used to partially sign the transaction: 
$$
\begin{aligned} 
x_{sa} &= k_a - \phi_a \\\\
x_{sb} &= k_b - \phi_b \\\\
x_{sc} &= k_c - \phi_c
\end{aligned}
$$
Consequently they share the public value of the excess $ x_{sn}G $ with each other:
$$
P_{agg} = x_{sa}G + x_{sb}G + x_{sc}G
$$
They now have enough information to calculate the same challenge $ e $ as
$$
e = \text{Hash}(R_{agg} || P_{agg} || m) \mspace{18mu} \text{ with } \mspace{18mu} m = fee||height
$$
for a combined (aggregated) signature for the transaction. Each use their own private nonce $ r_n $ and secret blinding factor $ k_n $, calculate their partial Schnorr signature $ s_n $ and share it with each other:
$$
\begin{aligned} 
s\_a &= r_a + e \cdot k\_1 \\\\
s\_b &= r_b + e \cdot k\_2 \\\\
s\_c &= r_c + e \cdot k\_3
\end{aligned}
$$
The aggregated Schnorr signature is then simply calculated as
$$
s\_{agg} = s\_a + s\_b + s\_c
$$
The resulting signature for the transaction is the tuple $ (s_{agg},R_{agg}) $. In order to validate the signature, the publicly shared aggregated values $ R_{agg} $ and $ P_{agg} $ will be needed:
$$
s_{agg}G \overset{?}{=} R_{agg} + e \cdot P_{agg}
$$
In order to validate that no funds are created the total offset must also be stored in the transaction kernel, so the parties also share the offset they calculated:
$$
\phi_{tot} = \phi_a + \phi_b + \phi_c
$$
The transaction balance is then validated to be equal to a commitment to the value $ 0 $ as follows:
$$
(v_1H + (k_1 + k_2 + k_3)G) - (v_aH + k_aG) - (v_bH + k_bG) - (v_cH + k_cG) + fee \overset{?}{=} (0H + o_{tot}G)
$$



### Creating the Multiparty Bulletproof

One crucial aspect in validating the transaction is still missing, that is each new UTXO must also include a Bulletproof range proof. Up to now, Alice, Bob and Carol could each keep their portion of the shared blinding factor $ k_n $ secret. The new combined commitment they created, $ (v_1H + (k_1 + k_2 + k_3)G) $, cannot be used as is to calculate the Bulletproof range proof, otherwise the three parties will have to give up their portion of the shared blinding factor. Now they need to use a secure method to calculate their combined Bulletproof range proof.

#### Utilizing Bulletproofs MPC Protocol

This scheme involves coloring the UTXO to enable attachment of additional proof data and a flag to let the miners know that they have to employ a different set of validation rules. The [Bulletproofs Multiparty Computation](../../cryptography/bulletproofs-protocols/MainReport.md#mpc-protocol-for-bulletproofs) (MPC) protocol can be used in special way to construct a range proof that can be validated by the miners. Aggregating the range proofs using this protocol provides a huge space saving; a single Bulletproof range proof consists of 672 bytes, whereas 10 only consist of 928 bytes. For this scheme the simple information sharing protocol will not be adequate; an efficient robust implementation of the Bulletproof MPC range proof like that done by Dalek Cryptography [[10]] is suggested. 

This scheme works as follows. Alice, Bob and Carol proceed to calculate an aggregated MPC Bulletproof range proof for the combined multiparty funds, but each using their own secret blinding factor in the commitment. They therefor construct fake commitments that will be used to calculate fake range proofs as follows:
$$
\begin{aligned} 
\text{Alice's fake commitment:} \mspace{18mu} C_1(\frac{v_1}{3},k_1) &= (\frac{v_1}{3}H + k_1G) \\\\
\text{Bob's fake commitment:} \mspace{18mu} C_2(\frac{v_1}{3},k_2) &= (\frac{v_1}{3}H + k_2G) \\\\
\text{Carol's fake commitment:} \mspace{18mu} C_3(\frac{v_1}{3},k_3) &= (\frac{v_1}{3}H + k_3G)
\end{aligned}
$$

Notice that 
$$
\begin{aligned} 
C_m(v_1, k_1 + k_2 + k_3) &= C_1(\frac{v_1}{3},k_1) + C_2(\frac{v_1}{3},k_2) + C_3(\frac{v_1}{3},k_3) \\\\
(v_1H + (k_1 + k_2 + k_3)G) &= (\frac{v_1}{3}H + k_1G) + (\frac{v_1}{3}H + k_1G) + (\frac{v_1}{3}H + k_1G)
\end{aligned}
$$


Running the Bulletproof MPC range proof will result in a proof share $ PS_n(C_n) $ for each party for their fake commitments, which will be aggregated by the dealer according to the protocol. Any one of the party members can be the dealer as the objective here is just to create the aggregated range proof. Let the aggregated range proof for the set $ \{ C_1, C_2, C_3 \} $ be depicted by $ RP_{m_{agg}} $. The UTXO will then consist of the tuple $ (C_m , RP_{m_{agg}}) $ and meta data $ flag, C_1, C_2, C_3 $. Validation by miners will involve
$$
C_m \overset{?}{=} C_1 + C_2 + C_3 \\\\
\text{verify }  RP_{m_{agg}}  \text{ for set }  \{ C_1, C_2, C_3 \}
$$
instead of 
$$
\text{verify }  RP_{m}  \text{ for }  C_m 
$$


#### Utilizing Grin's Shared Bulletproof Computation

???



## Mimblewimble $ m\text{-of-}n $ Multiparty Bulletproof UTXO

Shamir's Secret Sharing is needed

Amount of rounds can be pre-determined

Hash of the secret can be shared together with the shards so $ m\text{-of-}n $ parties can confirm its correctness

### Utilizing Shamir's Secret Sharing

???

<<https://iancoleman.io/shamir/>>

[[8]]

### Multiple Rounds Scheme

???



## Comparison to Bitcoin

???



## Conclusions, Observations and Recommendations

- ???



## References

[[1]] "The Best Step-by-Step Bitcoin Script Guide Part 2" [online]. Available: <https://blockgeeks.com/guides/bitcoin-script-guide-part-2>. Date accessed: 2019&#8209;05&#8209;02.

[1]: https://blockgeeks.com/guides/bitcoin-script-guide-part-2
"The Best Step-by-Step Bitcoin Script Guide Part 2"

[[2]] "Script" [online]. Available: <https://en.bitcoin.it/wiki/Script>. Date accessed: 2019&#8209;05&#8209;06.

[2]: https://en.bitcoin.it/wiki/Script
"Script"

[[3]] "Multisignature" [online]. Available: <https://en.bitcoin.it/wiki/Multisignature>. Date accessed: 2019&#8209;05&#8209;06.

[3]: https://en.bitcoin.it/wiki/Multisignature
"Multisignature"

[[4]] "Transaction" [online]. Available: <https://en.bitcoin.it/wiki/Transaction>. Date accessed: 2019&#8209;05&#8209;06.

[4]: https://en.bitcoin.it/wiki/Transaction
"Transaction"

[[5]] S. Pour, "Bitcoin multisig the hard way: Understanding raw P2SH multisig transactions" [online]. Available: <https://www.soroushjp.com/2014/12/20/bitcoin-multisig-the-hard-way-understanding-raw-multisignature-bitcoin-transactions>. Date accessed: 2019&#8209;05&#8209;06.

[5]: https://www.soroushjp.com/2014/12/20/bitcoin-multisig-the-hard-way-understanding-raw-multisignature-bitcoin-transactions
"Bitcoin multisig the hard way: 
Understanding raw P2SH multisig transactions"

[[6]] "GitHub: gavinandresen/TwoOfThree.sh" [online]. Available: <https://gist.github.com/gavinandresen/3966071>. Date accessed: 2019&#8209;05&#8209;06.

[6]: https://gist.github.com/gavinandresen/3966071
"GitHub: gavinandresen/TwoOfThree.sh"

[[7]] H.de Valence, I. Lovecruft and O. Andreev, "Merlin Transcripts" [online]. Available: <https://merlin.cool/index.html> and <https://doc-internal.dalek.rs/merlin/index.html>. Date accessed: 2019&#8209;05&#8209;10.

[7]: https://doc-internal.dalek.rs/merlin/index.html
"Merlin Transcripts" 

[[8]] T. Pedersen. "Non-interactive and Information-theoretic Secure Verifiable Secret Sharing" 
[online]. Available: <https://www.cs.cornell.edu/courses/cs754/2001fa/129.pdf>. Date accessed: 
2018-09-27.

[8]: https://www.cs.cornell.edu/courses/cs754/2001fa/129.pdf
"Non-interactive and information-theoretic
secure verifiable secret sharing, 
Pedersen T."

[[9]] "GrinExplorer, Block 164,690" [online]. Available: <https://grinexplorer.net/block/0000016c1ceb1cf588a45d0c167dbfb15d153c4d1d33a0fbfe0c55dbf7635410>. Date accessed: 2019&#8209;05&#8209;10.

[9]: https://grinexplorer.net/block/0000016c1ceb1cf588a45d0c167dbfb15d153c4d1d33a0fbfe0c55dbf7635410
"GitHub: gavinandresen/TwoOfThree.sh"

[[10]] "Dalek Cryptography - Crate Bulletproofs - Module bulletproofs::range_proof_mpc" [online]. Available: <https://doc-internal.dalek.rs/bulletproofs/range_proof_mpc/index.html>. 
Date accessed: 2018-11-12.

[10]: https://doc-internal.dalek.rs/bulletproofs/range_proof_mpc/index.html
"Dalek Cryptography - Crate Bulletproofs
Module bulletproofs::range_proof_mpc"




[[?]] B. Bünz, J. Bootle, D. Boneh, A. Poelstra, P. Wuille and G. Maxwell, "Bulletproofs: Short Proofs for Confidential Transactions and More", Blockchain Protocol Analysis and Security Engineering 2018 [online]. Available: <http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf>. Date accessed: 2018&#8209;09&#8209;18.

[?]: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf "Bulletproofs: Short Proofs for Confidential Transactions and 
More" 





## Appendices

### Appendix A: ???

??? 



## Contributors

- <https://github.com/hansieodendaal>
- <https://github.com/anselld>
