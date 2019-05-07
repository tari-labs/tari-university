# Mimblewimble Multiparty Bulletproof UTXO

- [Introduction](#introduction)
- [Notation Used](#notation-used)
- [Bitcoin $ m\text{-of-}n $ Multisig in a Nutshell](#bitcoin--mtext-of-n--multisig-in-a-nutshell)
- [Conclusions, Observations and Recommendations](#conclusions-observations-and-recommendations)
- [References](#references)
- [Appendices](#appendices)
  - [Appendix A: ???](#appendix-a-???)
- [Contributors](#contributors)



## Introduction

In [Mimblewimble](../mimblewimble-1/MainReport.md) the concept of a Bitcoin type multi-signature (multisig) applied to an Unspent Transaction Output (UTXO) does not really exist. 

In Bitcoin, multisig payments are usually combined with Pay to Script Hash (P2SH) functionality as a means to send funds to a P2SH payment address and then to manage its expenditure from there. The redeem script itself sets the conditions that must be fulfilled in order for the UTXOs linked to the P2SH payment address to be spent [[[1]], [[2]]). 

Unlike Bitcoin, Mimblewimble transactions do not involve payment addresses as all transactions are confidential. The only requirement for a Mimblewimble UTXO to be spent is the ability to open (or unlock) the [Pederson Commitment](../../cryptography/bulletproofs-protocols/MainReport.md#pedersen-commitments-and-elliptic-curve-pedersen-commitments) and does not require an "owner" signature.



## Notation Used

This section gives the general notation of mathematical expressions used. It provides important pre-knowledge for the remainder of the report.

- All Pederson Commitments will be of the [elliptic derivative]((../../cryptography/bulletproofs-protocols/MainReport.md#pedersen-commitments-and-elliptic-curve-pedersen-commitments)) depicted by $  C(v,k) = (vH + kG)  $ with $ v $ being the value committed to and $ k $ being the blinding factor.
- Scalar multiplication will be depicted by "$ \cdot $", as an example $ e \cdot (vH + kG) = e \cdot vH + e \cdot kG  $.
- A Pederson Commitments to the value of $ 0 $ will be depicted by $ C(0,k) = (0H + kG) = (kG) = (\mathbf{0}) $.



## Bitcoin $ m\text{-of-}n $ Multisig in a Nutshell

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



## Formation of the Multiparty Bulletproof UTXO

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
where the unspent-hiding-blinding commitment from Alice is $ (v_1 H + k_1 G) $ and the value of $ (v_2 H + k_2 G) $ is equal to $ fee $ to be paid to the miner. The newly created commitment $ (v_1 H + (k_1 + k_x) G) $ would be equally unspendable by the Alice and Bob because neither of them will know the total blinding factor $ k_1 + k_x $. Fortunately, in order to construct a Bulletproof range proof for the new output $ (v_1 H + (k_1 + k_x) G) $ as required by transaction validation rules, the values of $ v_1 $ and $ k_1 + k_x $ must be known otherwise the prover (i.e. Bob) cannot convince an honest verifier (i.e. the miner) that $ v_1 $ is non-negative (i.e. in the range $ [0,2^n - 1] $).

In the event that Bob can convince Alice that she must create a fund that both of them have signing powers over ($ 2\text{-of-}2 $ multisig), it would theoretically be possible to create the required Bulletproof range proof for relation (1) if they work together to create it.



### Setting up the Multi-party Funding Transaction

Alice, Bob and Carol agree to set up a multi-party $ 3\text{-of-}3 $ multisig fund that they can control together. The transaction they need to set up looks like this:
$$
\begin{aligned} 
C_m(v_1, k_1 + k_2 + k_3) - C_a(v_a, k_a) - C_b(v_b, k_b) - C_c(v_c, k_c) + fee &= (\mathbf{0}) \\\\
(v_1H + (k_1 + k_2 + k_3)G) - (v_aH + k_aG) - (v_bH + k_bG) - (v_cH + k_cG) + fee &= (\mathbf{0})
\end{aligned}
$$
In order for this to work, they have to keep their portion of the shared blinding factor secret, so each of them creates their private blinding factor $ k_n $ and shares the public blinding factor $ k_nG $ with the group. In addition, each of them create an offset $ o_n $ that will be subtracted from their input commitments' blinding factors to prevent someone else linking this transaction's inputs and outputs when analyzing the Mimblewimble block. Each of calculates their total excess that will be used to sign the transaction: 
$$
\begin{aligned} 
x_{sa} &= k_a - o_a \\\\
x_{sb} &= k_b - o_b \\\\
x_{sc} &= k_c - o_c
\end{aligned}
$$
Consequently they share the public value of the excess with each other:
$$
P_{agg} = x_{sa}G + x_{sb}G + x_{sc}G
$$
In order for them to sign the transaction, they also need to share the public values of their shared blinding factor with each other:
$$
R_{agg} = k_1G + k_2G + k_3G
$$
They now have enough information to calculate the same challenge for the signature,
$$
\begin{aligned} 
m &= fee||height \\\\
e &= \text{Hash}(R_{agg} || P_{agg} || m)
\end{aligned}
$$
and proceed to calculate a combined signature for the transaction. Each use their own private nonce $ r_n $ and secret shared blinding factor $ k_n $ and calculate partial a signatures $ s_n $ that will be shared and aggregated:
$$
\begin{aligned} 
s\_a &= r_a + e \cdot k\_1 \\\\
s\_b &= r_b + e \cdot k\_2 \\\\
s\_c &= r_c + e \cdot k\_3 \\\\
s\_{agg} &= s\_a + s\_b + s\_c
\end{aligned}
$$
The resulting signature for the transaction is the tuple $ (s_{agg},R_{agg}) $. In order to validate the signature, all publicly shared values will be needed:
$$
s_{agg}G \overset{?}{=} R_{agg} + e \cdot P_{agg}
$$
In order to validate that no funds are created the total offset must also be shared and stored in the transaction kernel:
$$
o_{tot} = o_a + o_b + o_c
$$
The transaction is then validated to be equal to a commitment to the value $ 0 $ as follows:
$$
(v_1H + (k_1 + k_2 + k_3)G) - (v_aH + k_aG) - (v_bH + k_bG) - (v_cH + k_cG) + fee \overset{?}{=} (0H + o_{tot}G)
$$


### Creating the Multi-party Bulletproof

One crucial aspect in validating the transaction is still missing, that is each new UTXO must also include a Bulletproof range proof. Up to now Alice, Bob and Carol could each keep their portion of the shared blinding factor secret. The new combined commitment they created, $ (v_1H + (k_1 + k_2 + k_3)G) $, cannot be used as is to calculate the Bulletproof range proof, otherwise the three parties will have to give up their portion of the shared blinding factor.

???



## Conclusions, Observations and Recommendations

- ???



## References

[[1]] "The Best Step-by-Step Bitcoin Script Guide Part 2" [online]. Available: https://blockgeeks.com/guides/bitcoin-script-guide-part-2. Date accessed: 2019&#8209;05&#8209;02.

[1]: https://blockgeeks.com/guides/bitcoin-script-guide-part-2
"The Best Step-by-Step Bitcoin Script Guide Part 2"

[[2]] "Script" [online]. Available: https://en.bitcoin.it/wiki/Script. Date accessed: 2019&#8209;05&#8209;06.

[2]: https://en.bitcoin.it/wiki/Script
"Script"

[[3]] "Multisignature" [online]. Available: https://en.bitcoin.it/wiki/Multisignature. Date accessed: 2019&#8209;05&#8209;06.

[3]: https://en.bitcoin.it/wiki/Multisignature
"Multisignature"

[[4]] "Transaction" [online]. Available: https://en.bitcoin.it/wiki/Transaction. Date accessed: 2019&#8209;05&#8209;06.

[4]: https://en.bitcoin.it/wiki/Transaction
"Transaction"

[[5]] S. Pour, "Bitcoin multisig the hard way: Understanding raw P2SH multisig transactions" [online]. Available: https://www.soroushjp.com/2014/12/20/bitcoin-multisig-the-hard-way-understanding-raw-multisignature-bitcoin-transactions. Date accessed: 2019&#8209;05&#8209;06.

[5]: https://www.soroushjp.com/2014/12/20/bitcoin-multisig-the-hard-way-understanding-raw-multisignature-bitcoin-transactions
"Bitcoin multisig the hard way: 
Understanding raw P2SH multisig transactions"

[[6]] "GitHub: gavinandresen/TwoOfThree.sh" [online]. Available: https://gist.github.com/gavinandresen/3966071. Date accessed: 2019&#8209;05&#8209;06.

[6]: https://gist.github.com/gavinandresen/3966071
"GitHub: gavinandresen/TwoOfThree.sh"




[[?]] B. Bünz, J. Bootle, D. Boneh, A. Poelstra, P. Wuille and G. Maxwell, "Bulletproofs: Short Proofs for Confidential Transactions and More", Blockchain Protocol Analysis and Security Engineering 2018 [online]. Available: <http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf>. Date accessed: 2018&#8209;09&#8209;18.

[?]: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf "Bulletproofs: Short Proofs for Confidential Transactions and 
More" 





## Appendices

### Appendix A: ???

??? 



## Contributors

- <https://github.com/hansieodendaal>
- <https://github.com/anselld>
