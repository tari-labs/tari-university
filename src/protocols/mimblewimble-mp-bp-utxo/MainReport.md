<head>
<style>
div.mywrap {
  width: 95%; 
  word-wrap: break-word;
  background: #f1f1f1;
  font-size: 0.875em;
  font-family: "Source Code Pro", Consolas, "Ubuntu Mono", Menlo, "DejaVu Sans Mono", monospace, monospace;
  padding: 2.0em;
  color: #6e6b5e;
}
</style>
</head>


# Mimblewimble Multiparty Bulletproof UTXO

- [Introduction](#introduction)
- [Background](#background)
  - [Bitcoin $ m\text{-of-}n $ Multisig in a Nutshell](#bitcoin--mtext-of-n--multisig-in-a-nutshell)
  - [Security of the Mimblewimble Blockchain](#security-of-the-mimblewimble-blockchain)
  - [Secure Sharing Protocol](#secure-sharing-protocol)
- [Mimblewimble $ n\text{-of-}n $ Multiparty Bulletproof UTXO](#mimblewimble--ntext-of-n--multiparty-bulletproof-utxo)
  - [Simple Sharing Protocol](#simple-sharing-protocol)
  - [Setting up the Multiparty Funding Transaction](#setting-up-the-multiparty-funding-transaction)
  - [Creating the Multiparty Bulletproof Range Proof](#creating-the-multiparty-bulletproof-range-proof)
    - [Utilizing Bulletproofs MPC Protocol](#utilizing-bulletproofs-mpc-protocol)
    - [Utilizing Grin's Shared Bulletproof Computation](#utilizing-grins-shared-bulletproof-computation)
    - [Comparison of the two Bulletproof Methods](#comparison-of-the-two-bulletproof-methods)
  - [Spending the Multiparty UTXO](#spending-the-multiparty-utxo)
- [Mimblewimble $ m\text{-of-}n $ Multiparty Bulletproof UTXO](#mimblewimble--mtext-of-n--multiparty-bulletproof-utxo)
  - [Secret Sharing](#secret-sharing)
  - [Multiple Rounds' Data](#multiple-rounds-data)
  - [How it Works](#how-it-works)
  - [Spending Protocol](#spending-protocol)
- [Comparison to Bitcoin](#comparison-to-bitcoin)
- [Conclusions, Observations and Recommendations](#conclusions-observations-and-recommendations)
- [References](#references)
- [Appendices](#appendices)
  - [Appendix A: Notation Used](#appendix-a-notation-used)
  - [Appendix B: Definition of Terms](#appendix-b-definition-of-terms)
  - [Appendix C: Shamir's Secret Sharing Example](#appendix-c-shamirs-secret-sharing-example)
- [Contributors](#contributors)



## Introduction

In [Mimblewimble](../mimblewimble-1/MainReport.md), the concept of a Bitcoin-type multi-signature (multisig) applied to 
an Unspent Transaction Output (UTXO) does not really exist. 

In Bitcoin, multisig payments are usually combined with Pay to Script Hash (P2SH) functionality as a means to send funds 
to a P2SH payment address, and then to manage its expenditure from there. The redeem script itself sets the conditions 
that must be fulfilled for the UTXOs linked to the P2SH payment address to be spent ([[1]], [[2]]). 

Unlike Bitcoin, Mimblewimble transactions do not involve payment addresses, as all transactions are confidential. The 
only requirement for a Mimblewimble UTXO to be spent is the ability to open (or unlock) the 
[Pederson Commitment](../../cryptography/bulletproofs-protocols/MainReport.md#pedersen-commitments-and-elliptic-curve-pedersen-commitments) 
that contains the tokens; it does not require an "owner" signature. A typical Mimblewimble UTXO looks like this [[9]]:

<div class="mywrap">08c15e94ddea81e6a0a31ed558ef5e0574e5369c4fcba92808fe992fbff68884cc</div>

Another fundamental difference is that for any Mimblewimble transaction, all parties, i.e. all senders and all 
receivers, must interact to conclude the transaction.



## Background



### Bitcoin $ m\text{-of-}n $ Multisig in a Nutshell

Multiple use cases of $ m\text{-of-}n $ multisig applications exist, e.g. a $ 1\text{-of-}2 $ petty cash account, 
a $ 2\text{-of-}2 $ two-factor authentication wallet and a $ 2\text{-of-}3 $ board of directors account [[3]]. 
A typical 2-of-3 Bitcoin P2SH multisig redeem script (where any two of the three predefined public keys must sign the 
transaction) has the following form:

```Text
redeemScript     = <OP_2> <A pubkey> <B pubkey> <C pubkey> <OP_3> OP_CHECKMULTISIG
```

The P2SH payment address is the result of the redeem script double hashed with SHA-256 and RIPEMD-160 and then 
Base58Check encoded with a prefix of `0x05`:

```Text
redeemScriptHash = RIPEMD160(SHA256(redeemScript))
P2SHAddress      = base58check.Encode("05", redeemScriptHash)
```

Multiple payments can now be sent to the P2SH payment address. A generic funding transaction's output script for the 
P2SH payment address has the following form, irrespective of the redeem script's contents:

```Text
scriptPubKey      =     OP_HASH160 <redeemScriptHash> OP_EQUAL
```

with `OP_HASH160` being the combination of SHA-256 and RIPEMD-160. The 2-of-3 multisig redeem transaction's input script 
would have the following form:

```Text
scriptSig         =  OP_0 <A sig> <C sig> <redeemScript>
```

and the combined spending and funding transaction script (validation script) would be

```Text
validationScript    = OP_0 <A sig> <C sig> <redeemScript> OP_HASH160 <redeemScriptHash> OP_EQUAL
```

When `validationScript` is executed, all values are added to the execution stack in sequence. When opcode `OP_HASH160` is 
encountered, the preceding value `<redeemScript>` is hashed and added to the stack; and when opcode `OP_EQUAL` is 
encountered, the previous two values, hash of the `<redeemScript>` and `<redeemScriptHash>`, are compared and removed 
from the stack if equal. The top of the stack then contains `<redeemScript>`, which is evaluated with the two entries on 
top of that, `<A sig>` and `<C sig>`. The last value in the stack, `OP_0`, is needed for a glitch in the `OP_CHECKMULTISIG` 
opcode implementation, which makes it pop one more item than those that are available on the stack ([[1]], [[5]]).

*What is signed?*

Partial signatures are created in the same sequence in which the public keys are defined in `redeemScript`. A simplified 
serialized hexadecimal version of the transaction - consisting of the input transaction ID and UTXO index, amount to be 
paid, `scriptPubKey` and transaction locktime - is signed. Each consecutive partial signature includes a serialization 
of the previous partial signature with the simplified transaction data to be signed, creating multiple cross-references 
in the signed data. This, combined with the public keys, proves the transaction was created by the real owners of the 
bitcoins in question ([[4]], [[5]], [[6]]).

*How is change redirected to the multisig P2SH?*

Bitcoin transactions can have multiple recipients, and one of the recipients of the funds from a P2SH multisig 
transaction can be the original `P2SHAddress`, thus sending change back to itself. Circular payments to the same 
addresses are allowed, but they will suffer from a lack of confidentiality. Another way to do this would be to create a 
new `redeemScript` with a new set of public keys every time a P2SH multisig transaction is done to collect the change, 
but this would be more complex to manage [[4]].



### Security of the Mimblewimble Blockchain

A [Mimblewimble](../mimblewimble-1/MainReport.md) blockchain relies on two complementary aspects to provide security: 
[Pederson Commitments](../../cryptography/bulletproofs-protocols/MainReport.md#pedersen-commitments-and-elliptic-curve-pedersen-commitments) 
and range proofs (in the form of [Bulletproof range proofs](../../cryptography/bulletproofs-and-mimblewimble/MainReport.md)). 
Pederson Commitments provide perfectly hiding and computationally binding commitments, i.e. the confidentiality aspect. Range proofs provide assurance that the currency cannot be inflated and that third parties cannot lock away 
one's funds. Since Mimblewimble commitments are totally confidential and ownership cannot be proved, anyone can try 
to spend or mess with unspent coins embedded in those commitments. Fortunately, any new UTXO requires a range proof, and 
this is impossible to create if the input commitment cannot be opened.

The role that Bulletproof range proofs play in securing the blockchain can be demonstrated as follows. Let 
$ C\_a(v\_1 , k\_1) $ be the "closed" input UTXO commitment from Alice that a bad actor, Bob, is trying to lock away by 
adding an additional blinding factor $ k\_{x} $ to the commitment. (Refer to [Appendix A](#appendix-a-notation-used) for all 
notation used.) A valid Mimblewimble transaction would have the following form:
$$
\begin{aligned} 
C\_{locked}(v\_1 , k\_1 + k\_x) - C\_a(v\_1 , k\_1) - C\_{fee}(v\_2 , k\_2) + fee &= (\mathbf{0}) \\\\
\\\\
(v\_1 H + (k\_1 + k\_x) G) -  (v\_1 H + k\_1 G) - (v\_2 H + k\_2 G) + fee &= (\mathbf{0})
\end{aligned}
\mspace{70mu} (1)
$$

where the unspent-hiding-blinding commitment from Alice is $ (v\_1 H + k\_1 G) $ and the value of $ (v\_2 H + k\_2 G) $ 
is equal to $ fee $ to be paid to the miner. The newly created commitment $ (v\_1 H + (k\_1 + k\_x) G) $ would be 
equally unspendable by Alice and Bob, because neither of them would know the total blinding factor $ k\_1 + k\_x $. 
Fortunately, in order to construct a Bulletproof range proof for the new output $ (v\_1 H + (k\_1 + k\_x) G) $ as 
required by transaction validation rules, the values of $ v\_1 $ and $ k\_1 + k\_x $ must be known, otherwise the prover 
(i.e. Bob) would not be able to convince an honest verifier (i.e. the miner) that $ v\_1 $ is non-negative (i.e. in the range 
$ [0,2^n - 1] $).

If Bob can convince Alice that she must create a fund over which both of them have signing powers ($ 2\text{-of-}2 $ 
multisig), it would theoretically be possible to create the required Bulletproof range proof for relation (1) if they 
work together to create it.



### Secure Sharing Protocol

Multiple parties working together to create a single transaction that involves multiple steps, need to share information 
in such a way that what they shared cannot work against them. Each step requires a proof, and it should not be possible 
to replay an individual step's proof in a different context. Merlin transcripts [[7]] is an excellent example of a 
protocol implementation that achieves this. For the purposes of this report, a simple information sharing protocol 
is proposed that can be implemented with Merlin transcripts [[7]]. 



## Mimblewimble $ n\text{-of-}n $ Multiparty Bulletproof UTXO



### Simple Sharing Protocol

Alice, Bob and Carol agree to set up a multiparty $ 3\text{-of-}3 $ multisig fund that they can control together. They 
decide to use a sharing hash function $ val\_H = \text{H}\_{s}(arg) $ as a handshaking mechanism for all information 
they need to share. The first step is to calculate the hash $ val\_H $ for the value $ arg $ they want to 
commit to in sharing, and to distribute it to all parties. They then wait until all other parties' commitments have been 
received. The second step is to send the actual value they committed to all parties, and to then verify each 
value against its commitment. If everything matches up, they proceed; otherwise they stop and discard everything they 
have done. 

This ensures that public values for each step are not exposed until all commitments have been received. They will apply 
this simple sharing protocol to all information they need to share with each other. This will be denoted by 
$ \text{share:} ​$.



### Setting up the Multiparty Funding Transaction

In the transaction Alice, Bob and Carol want to set up, $ C\_m $ is the multiparty shared commitment that contains the 
funds, and $ C\_a $, $ C\_b $ and $ C\_c $ are their respective input contributions. This transaction looks as follows:

$$
\begin{aligned} 
C\_m(v\_1, \sum \_{j=1}^3 k\_jG) - C\_a(v\_a, k\_a) - C\_b(v\_b, k\_b) - C\_c(v\_c, k\_c) + fee &= (\mathbf{0}) \\\\
(v\_1H + (k\_1 + k\_2 + k\_3)G) - (v\_aH + k\_aG) - (v\_bH + k\_bG) - (v\_cH + k\_cG) + fee &= (\mathbf{0})
\end{aligned}
$$

In order for this scheme to work, they must be able to jointly sign the transaction with a Schnorr signature, while 
keeping their portion of the shared blinding factor secret. Each of them creates their own private blinding factor 
$ k\_n ​$ for the multiparty shared commitment and shares the public blinding factor $ k\_nG ​$ with the group:
$$
\text{share:} \mspace{9mu} \lbrace k\_1G, k\_2G, k\_3G \rbrace
$$

They proceed to calculate their own total excess blinding factors as 
$ x\_{sn} = \sum k\_{n(change)} - \sum k\_{n(inputs)} - \phi\_n $, with $ \phi_n\ ​$ being a random offset of their own 
choosing (in this example there is no change): 
$$
\begin{aligned} 
x\_{sa} &= 0 - k\_a - \phi\_a \\\\
x\_{sb} &= 0 - k\_b - \phi\_b \\\\
x\_{sc} &= 0 - k\_c - \phi\_c
\end{aligned}
$$

The offset $ \phi\_n $ is introduced to prevent someone else linking this transaction's inputs and outputs when 
analyzing the Mimblewimble block, and will be used later on to balance the transaction. They consequently share the 
public value of the excess $ x\_{sn}G $ with each other: 

$$
\text{share:} \mspace{9mu} \lbrace x\_{sa}G, x\_{sb}G, x\_{sc}G \rbrace
$$

They now have enough information to calculate the aggregated public key for the signature:

$$
P\_{agg} = (k\_1G + x\_{sa}G) + (k\_2G + x\_{sb}G) + (k\_3G + x\_{sc}G) \\\\
P\_{agg} = (k\_1G - (k\_a + \phi\_a)G) + (k\_2G - (k\_b + \phi\_b)G) + (k\_3G - (k\_ca + \phi\_c)G)
$$

Each party also selects a private nonce $ r\_n $, shares the public value $  r\_nG $ with the group,

$$
\text{share:} \mspace{9mu} \lbrace r\_aG, r\_bG, r\_cG \rbrace
$$

and calculates the aggregated public nonce for the signature:

$$
R\_{agg} = r\_aG + r\_bG + r\_cG
$$

The signature challenge $ e $ can now be calculated:

$$
e = \text{Hash}(R\_{agg} || P\_{agg} || m) \mspace{18mu} \text{ with } \mspace{18mu} m = fee||height
$$

Each party now uses their private nonce $ r\_n $, secret blinding factor $ k\_n $ and excess $ x\_{sn} $ to calculate a 
partial Schnorr signature $ s\_n $:
$$
\begin{aligned} 
s\_a &= r_a + e \cdot (k\_1 + x\_{sa}) \\\\
s\_b &= r_b + e \cdot (k\_2 + x\_{sb}) \\\\
s\_c &= r_c + e \cdot (k\_3 + x\_{sc})
\end{aligned}
$$

These partial signatures are then shared with the group to be aggregated:

$$
\text{share:} \mspace{9mu} \lbrace s\_a, s\_b, s\_c \rbrace
$$

The aggregated Schnorr signature for the transaction is then simply calculated as

$$
s\_{agg} = s\_a + s\_b + s\_c
$$

The resulting signature for the transaction is the tuple $ (s\_{agg},R\_{agg}) $. To validate the signature, publicly 
shared aggregated values $ R\_{agg} $ and $ P\_{agg} $ will be needed:

$$
s\_{agg}G \overset{?}{=} R\_{agg} + e \cdot P\_{agg}
$$

To validate that no funds are created, the total offset must also be stored in the transaction kernel, so the parties 
also share their offset and calculate the total:
$$
\text{share:} \mspace{9mu} \lbrace \phi\_a, \phi\_b, \phi\_c \rbrace \\\\
\phi\_{tot} = \phi\_a + \phi\_b + \phi\_c
$$

The transaction balance can then be validated to be equal to a commitment to the value $ 0 $ as follows:

$$
(v\_1H + (k\_1 + k\_2 + k\_3)G) - (v\_aH + k\_aG) - (v\_bH + k\_bG) - (v\_cH + k\_cG) + fee \overset{?}{=} 
(0H + ( P\_{agg} + \phi\_{tot})G)
$$


### Creating the Multiparty Bulletproof Range Proof

One crucial aspect in validating the transaction is still missing, i.e. each new UTXO must also include a Bulletproof 
range proof. Up to now, Alice, Bob and Carol could each keep their portion of the shared blinding factor $ k\_n $ 
secret. The new combined commitment they created, $ (v\_1H + (k\_1 + k\_2 + k\_3)G) $, cannot be used as is to calculate 
the Bulletproof range proof, otherwise the three parties would have to give up their portion of the shared blinding 
factor. Now they need to use a secure method to calculate their combined Bulletproof range proof.


#### Utilizing Bulletproofs MPC Protocol

This scheme involves coloring the UTXO to enable attachment of additional proof data, a flag to let the miners know that 
they must employ a different set of validation rules and a hash of the UTXO and all metadata. The 
[Bulletproofs Multiparty Computation](../../cryptography/bulletproofs-protocols/MainReport.md#mpc-protocol-for-bulletproofs) 
(MPC) protocol can be used in a special way to construct a range proof that can be validated by the miners. Aggregating 
the range proofs using this protocol provides a huge space saving; a single Bulletproof range proof consists of 672&nbsp;bytes, whereas aggregating 16 only consists of 928&nbsp;bytes [[11]]. For this scheme, the simple information sharing protocol 
will not be adequate; an efficient, robust and secure implementation of the Bulletproof MPC range proof such as that done 
by Dalek Cryptography [[10]] is suggested. 

This scheme works as follows. Alice, Bob and Carol proceed to calculate an aggregated MPC Bulletproof range proof for 
the combined multiparty funds, but with each using their own secret blinding factor in the commitment. They therefore 
construct fake commitments that will be used to calculate fake range proofs as follows:
$$
\begin{aligned} 
\text{Alice's fake commitment:} \mspace{18mu} C\_1(\frac{v\_1}3,k\_1) &= (\frac{v\_1}3H + k\_1G) \\\\
\text{Bob's fake commitment:} \mspace{18mu} C\_2(\frac{v\_1}3,k\_2) &= (\frac{v\_1}3H + k\_2G) \\\\
\text{Carol's fake commitment:} \mspace{18mu} C\_3(\frac{v\_1}3,k\_3) &= (\frac{v\_1}3H + k\_3G)
\end{aligned}
$$

Notice that 

$$
\begin{aligned} 
C\_m(v\_1, k\_1 + k\_2 + k\_3) &= C\_1(\frac{v\_1}3,k\_1) + C\_2(\frac{v\_1}3,k\_2) + C\_3(\frac{v\_1}3,k\_3) \\\\
(v\_1H + (k\_1 + k\_2 + k\_3)G) &= (\frac{v\_1}3H + k\_1G) + (\frac{v\_1}3H + k\_1G) + (\frac{v\_1}3H + k\_1G)
\end{aligned}
$$

and that rounding implementation of $ ^{v\_1} / \_3  $ can ensure that adding these components for all parties will 
produce the original value $ v\_1 $.

Running the Bulletproof MPC range proof will result in a proof share for each party for their fake commitments, which 
will be aggregated by the dealer according to the MPC protocol. Any one of the party members can be the dealer, as the 
objective here is just to create the aggregated range proof. Let the aggregated range proof for the set 
$ \lbrace C\_1, C\_2, C\_3 \rbrace $ be depicted by $ RP\_{agg} $. The UTXO will then consist of the tuple 
$ (C\_m , RP\_{agg}) $ and metadata $ \lbrace flag, C\_1, C\_2, C\_3, hash_{C_{m}} \rbrace $. The hash that will secure 
the metadata is proposed as: 
$$
hash_{C\_m} = \text{Hash}(C\_m || RP\_{agg} || flag || C\_1 || C\_2 || C\_3)
$$


 Range proof validation by miners will involve
$$
\begin{aligned} 
hash_{C\_m} &\overset{?}{=} \text{Hash}(C\_m || RP\_{agg} || flag || C\_1 || C\_2 || C\_3) \\\\
C\_m &\overset{?}{=} C\_1 + C\_2 + C\_3 \\\\
\text{verify: }  &RP\_{agg}  \text{ for set }  \{ C\_1, C\_2, C\_3 \}
\end{aligned}
$$

instead of 

$$
\text{verify: }  RP\_m  \text{ for }  C\_m
$$


#### Utilizing Grin's Shared Bulletproof Computation

Grin extended the 
[Inner-product Range Proof](../../cryptography/bulletproofs-protocols/MainReport.md#inner-product-range-proof) 
implementation to allow for multiple parties to jointly construct a single Bulletproof range proof $ RP\_m $ for a known 
value $ v $, where each party can keep their partial blinding factor secret. The parties must share committed values 
deep within the inner-product range proof protocol ([[12]], [[13]], [[14]]). 

In order to construct the shared Bulletproof range proof $ RP\_m $, each party starts to calculate their own range proof 
for commitment $ C\_m(v\_1, \sum \_{j=1}^3 k\_jG) $ as follows:
$$
\begin{aligned} 
\text{Alice:} \mspace{18mu} C\_1(v\_1,k\_1) &= (v\_1H + k\_1G) \\\\
\text{Bob:} \mspace{18mu} C\_2(0,k\_2) &= (0H + k\_2G) \\\\
\text{Carol:} \mspace{18mu} C\_3(0,k\_3) &= (0H + k\_3G)
\end{aligned}
$$

With this implementation, Alice needs to act as the dealer. When they get to [steps (53) to (61) in Figure 5](../../cryptography/bulletproofs-protocols/MainReport.md#inner-product-range-proof) of the inner-product range proof 
protocol, they introduce the following changes:

1. Each party shares $ T\_{1\_j} $ and $ T\_{2\_j} $ with all other parties.
2. Each party calculates $ T\_1 = \sum\_{j=1}^k T\_{1\_j} $ and $ T_2 = \sum\_{j=1}^k T\_{2\_j} $.
3. Each party calculates $ \tau\_{x\_j} $ based on $ T\_1 $ and $ T\_2 $.
4. Each party shares $ \tau\_{x\_j} $ with the dealer.
5. The dealer calculates $\tau\_x = \sum\_{j=1}^k \tau\_{x\_j} $.
6. The dealer completes the protocol, using their own private $ k\_1 $, where a further blinding factor is required, and 
calculates $ RP\_m $.

Using this approach, the resulting shared commitment for Alice, Bob and Carol is

$$
C\_m(v\_1, \sum \_{j=1}^3 k\_jG) = (v\_1H + \sum \_{j=1}^3 k\_jG) = (v\_1H + (k\_1 + k\_2 + k\_3)G)
$$

with the UTXO tuple being $ (C\_m ,  RP\_m) ​$. Range proof validation by miners will involve verifying $ RP\_m ​$ for 
$ C\_m ​$.



#### Comparison of the Two Bulletproof Methods



| Consideration                 | Using Dalek's Bulletproofs MPC Protocol                      | Using Grin's Multiparty Bulletproof                          |
| ----------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ |
| Rounds of communication       | Three                                                        | Two                                                          |
| Security                      | Use of Merlin transcripts makes this method more secure against replay attacks. | No specific sharing protocol suggested.                      |
| Size of the Bulletproof       | Logarithmic Bulletproof range proof size, i.e. 672&nbsp;bytes up to 928&nbsp;bytes for 16&nbsp;range proofs. | Single Bulletproof range proof size of 672&nbsp;bytes.       |
| Colored coin                  | Coins are colored, i.e. distinguishable from normal commitments in the blockchain due to additional metadata. | Coins do not need to be colored, i.e. it may look exactly like any other commitment. |
| Wallet reconstructability     | Each individual range proof's data is accessible within the aggregated range proof. It is possible to identify the colored coin and then to reconstruct the wallet if the initial blinding factor seed is remembered in conjunction with [Bulletproof range proof rewinding](../../cryptography/bulletproofs-and-mimblewimble/MainReport.md#improved-implementation). | The wallet cannot be reconstructed, as a single party's blinding factor cannot be distinguished from the combined range proof. Even if these coins were colored with a flag to make them identifiable, it would not help. |
| Hiding and binding commitment | The main commitment and additional commitments in the UTXO's metadata retain all hiding and binding security aspects of the Pederson Commitment. | The commitment retains all hiding and binding security aspects of the Pederson Commitment. |



### Spending the Multiparty UTXO

Alice, Bob and Carol had a private bet going that Carol won, and they agree to spend the multiparty UTXO to pay Carol 
her winnings, with the change being used to set up a consecutive multiparty UTXO. This transaction looks as follows:
$$
\begin{aligned} 
C^{'}\_c(v^{'}\_c, k^{'}\_c) + C^{'}_m(v^{'}\_1, \sum \_{j=1}^3 k^{'}\_{j}G) - C_m(v\_1, \sum \_{j=1}^3 k\_jG) + fee &= (\mathbf{0}) \\\\
(v^{'}\_cH + k^{'}\_cG) + (v^{'}\_1H + (k^{'}\_1 + k^{'}\_2 + k^{'}\_3)G) - (v\_1H + (k\_1 + k\_2 + k\_3)G) + fee &= (\mathbf{0})
\end{aligned}
$$

Similar to the initial transaction, they each create their own private blinding factor $ k^{'}\_n $ for the new 
multiparty UTXO and share the public blinding factor $ k^{'}\_nG $ with the group. Carol also shares the public 
blinding factor $ k^{'}\_cG $ for her winnings:
$$
\begin{aligned} 
\text{share:} \mspace{9mu} &\lbrace k^{'}\_1G, k^{'}\_2G, k^{'}\_3G \rbrace \\\\
\text{share:} \mspace{9mu} &\lbrace  k^{'}\_cG \rbrace
\end{aligned}
$$

As before, they calculate their own total excess blinding factors as 
$ x^{'}\_{sn} = \sum k^{'}\_{n(change)} - \sum k^{'}\_{n(inputs)} - \phi^{'}\_n $: 

$$
\begin{aligned} 
x^{'}\_{sa} &= 0 - k\_1 - \phi^{'}\_a \\\\
x^{'}\_{sb} &= 0 - k\_2 - \phi^{'}\_b \\\\
x^{'}\_{sc} &= 0 - k\_3 - \phi^{'}\_c
\end{aligned}
$$

They share the public value of the excess $ x^{'}\_{sn}G $ with each other: 

$$
\text{share:} \mspace{9mu} \lbrace x^{'}\_{sa}G, x^{'}\_{sb}G, x^{'}\_{sc}G \rbrace
$$

The aggregated public key for the signature can then be calculated as:

$$
P^{'}\_{agg} = (k^{'}\_1G + x^{'}\_{sa}G) + (k^{'}\_2G + x^{'}\_{sb}G) + (k^{'}\_3G + x^{'}\_{sc}G + k^{'}\_cG) \\\\
P^{'}\_{agg} = (k^{'}\_1G - (k\_1 + \phi^{'}\_a)) + (k^{'}\_2G - (k\_2 + \phi^{'}\_a)) + (k^{'}\_3G - (k\_3 + \phi^{'}\_a) + k^{'}\_cG)
$$

Each party again selects a private nonce $ r^{'}\_n $, share the public value $  r^{'}\_nG $ with the group,

$$
\text{share:} \mspace{9mu} \lbrace r^{'}\_aG, r^{'}\_bG, r^{'}\_cG \rbrace
$$

and calculates the aggregated public nonce for the signature:

$$
R^{'}\_{agg} = r^{'}\_aG + r^{'}\_bG + r^{'}\_cG
$$

The new signature challenge $ e^{'} $ is then calculated as:

$$
e^{'} = \text{Hash}(R^{'}\_{agg} || P^{'}\_{agg} || m) \mspace{18mu} \text{ with } \mspace{18mu} m = fee||height
$$

Each party now calculates their partial Schnorr signature $ s^{'}\_n $ as before, except that Carol also adds her 
winnings' blinding factor $ k^{'}\_c $ to her signature:
$$
\begin{aligned} 
s^{'}\_a &= r^{'}\_a + e^{'} \cdot (k^{'}\_1 + x^{'}\_{sa})\\\\
s^{'}\_b &= r^{'}\_b + e^{'} \cdot (k^{'}\_2 + x^{'}\_{sa})\\\\
s^{'}\_c &= r^{'}\_c + e^{'} \cdot (k^{'}\_3 + x^{'}\_{sa} + k^{'}\_c)
\end{aligned}
$$

These partial signatures are then shared with the group 
$$
\text{share:} \mspace{9mu} \lbrace s^{'}\_a, s^{'}\_b, s^{'}\_c \rbrace
$$
to enable calculation of the aggregated Schnorr signature as
$$
s^{'}\_{agg} = s^{'}\_a + s^{'}\_b + s^{'}\_c
$$

The resulting signature for the transaction is the tuple $ (s^{'}\_{agg},R^{'}\_{agg}) $. The signature is again 
validated as done previously, using the publicly shared aggregated values $ R^{'}\_{agg} $ and $ P^{'}\_{agg} $:
$$
s^{'}\_{agg}G \overset{?}{=} R^{'}\_{agg} + e^{'} \cdot P^{'}\_{agg}
$$

Again the parties also share their own personal offset so that the total offset can be calculated:

$$
\text{share:} \mspace{9mu} \lbrace \phi^{'}\_a, \phi^{'}\_b, \phi^{'}\_c \rbrace \\\\
\phi^{'}\_{tot} = \phi^{'}\_a + \phi^{'}\_b + \phi^{'}\_c
$$

Lastly, the transaction balance can then be validated to be equal to a commitment to the value $ 0 $:

$$
(v^{'}\_cH + k^{'}\_cG) + (v^{'}\_1H + (k^{'}\_1 + k^{'}\_2 + k^{'}\_3)G) - (v\_1H + (k\_1 + k\_2 + k\_3)G) + fee \overset{?}{=} 
(0H + (P^{'}\_{agg} + \phi^{'}\_{tot}G))
$$


## Mimblewimble $ m\text{-of-}n $ Multiparty Bulletproof UTXO

As mentioned in the [Introduction](#introduction), Mimblewimble transactions cannot utilize a smart/redeem script in the 
form of a P2SH, but similar functionality can be implemented in the users' wallets. For the $ m\text{-of-}n ​$ multiparty 
Bulletproof UTXO, Shamir's Secret Sharing Scheme<sup>[def][ssss~]</sup> (SSSS)  ([[8]], [[15]]) will be used to enable 
$ m\text{-of-}n ​$ parties to complete a transaction. The SSSS is a method for $ n ​$ parties to carry shards (shares) 
$ s\_i ​$ of a message $ s ​$ such that any $ m ​$ of them can reconstruct the message.


### Secret Sharing

Our friends Alice, Bob and Carol decide to set up a $ 2\text{-of-}3 $ scheme, whereby any two of them can authorize a 
spend of their multiparty UTXO. They also want to be able to set up the scheme such that they can perform three rounds 
of spending, with the last round being the closing round. They have heard of the SSSS and decide to use that. 


### Multiple Rounds' Data

They will each pre-calculate three $ 3 $ private blinding factors $ k\_{n\text{-}i} $ and $ 3 $ shards 
$ k\_{n\text{-}party\text{-}i} $ for each round according to the SSSS. ([Appendix C](#appendix-c-shamirs-secret-sharing-example) 
shows an example of Alice's shards for one private blinding factor.) In addition to the shards, they will hash each 
round's private blinding factor $ \text{H}\_{s}( k\_{n\text{-}i}) ​$ and share that as well so that its correctness can 
be verified in each round. They continue to do this until they have all their information set up, ready and stored in 
their wallets.

| Round | Blinding<br />Factor                                         | $ \text{H}\_{s}( k\_{n\text{-}i}) $                          | Alice's <br />Shards                                         | Bob's <br />Shards                                           | Carol's <br />Shards                                         |
| ----- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| 1     | Alice: $ k\_{1\text{-}1} $ <br />Bob:&nbsp;&nbsp; $ k\_{2\text{-}1} $ <br />Carol: $ k\_{3\text{-}1} $ | $ \text{H}\_{s}( k\_{1\text{-}1}) $ <br />$ \text{H}\_{s}( k\_{2\text{-}1}) $ <br />$\text{H}\_{s}( k\_{3\text{-}1}) $ | $ k\_{1\text{-}a1} $ <br />$ k\_{2\text{-}a1} $<br />$ k\_{3\text{-}a1} $ | $ k\_{1\text{-}b1} $ <br />$ k\_{2\text{-}b1} $<br />$ k\_{3\text{-}b1} $ | $ k\_{1\text{-}c1} $ <br />$ k\_{2\text{-}c1} $<br />$ k\_{3\text{-}c1} $ |
| 2     | Alice: $ k\_{1\text{-}2} $ <br />Bob:&nbsp;&nbsp; $ k\_{2\text{-}2} $ <br />Carol: $ k\_{3\text{-}2} $ | $ \text{H}\_{s}( k\_{1\text{-}2}) $ <br />$ \text{H}\_{s}( k\_{2\text{-}2}) $ <br />$\text{H}\_{s}( k\_{3\text{-}2}) $ | $ k\_{1\text{-}a2} $ <br />$ k\_{2\text{-}a2} $<br />$ k\_{3\text{-}a2} $ | $ k\_{1\text{-}b2} $ <br />$ k\_{2\text{-}b2} $<br />$ k\_{3\text{-}b2} $ | $ k\_{1\text{-}c2} $ <br />$ k\_{2\text{-}c2} $<br />$ k\_{3\text{-}c2} $ |
| 3     | Alice: $ k\_{1\text{-}3} $ <br />Bob:&nbsp;&nbsp; $ k\_{2\text{-}3} $ <br />Carol: $ k\_{3\text{-}3} $ | $ \text{H}\_{s}( k\_{1\text{-}3}) $ <br />$ \text{H}\_{s}( k\_{2\text{-}3}) $ <br />$\text{H}\_{s}( k\_{3\text{-}3}) $ | $ k\_{1\text{-}a3} $ <br />$ k\_{2\text{-}a3} $<br />$ k\_{3\text{-}a3} $ | $ k\_{1\text{-}b3} $ <br />$ k\_{2\text{-}b3} $<br />$ k\_{3\text{-}b3} $ | $ k\_{1\text{-}c3} $ <br />$ k\_{2\text{-}c3} $<br />$ k\_{3\text{-}c3} $ |


### How it Works

The three parties set up the initial multiparty [funding transaction](#setting-up-the-multiparty-funding-transaction) 
and [Bulletproof range proof](#creating-the-multiparty-bulletproof-range-proof) exactly the same as they did for the 
$ 3\text{-of-}3 $ case. For this, they use the private blinding factors they pre-calculated for round&nbsp;1. Now, when 
they decide to spend the multiparty UTXO, only two of them need to be present.

Bob and Carol decide to spend funds, exactly [like before](#spending-the-multiparty-utxo), and for this they need to 
reconstruct Alice's private blinding factor for round&nbsp;1 and round&nbsp;2. Because Alice did not win anything, she 
does not need to be present to set up a private blinding factor for an output UTXO the way Carol needs to do. Bob and Carol 
consequently share the shards Alice gave them: 
$$
\text{share:} \mspace{9mu} \lbrace  k\_{1\text{-}b1},  k\_{1\text{-}c1},  k\_{1\text{-}b2},  k\_{1\text{-}c2}   \rbrace
$$

They are now able to reconstruct the blinding factors and verify the hashes. If the verification fails, they stop the 
protocol and schedule a meeting with Alice to have a word with her. With all three together, they will be able to 
identify the source of the misinformation.
$$
\text{reconstruct:} \mspace{9mu} k\_{1\text{-}1}, k\_{1\text{-}2} \\\\
\text{verify:} \mspace{9mu} \text{H}\_{s}( k\_{1\text{-}1})\_{shared} \overset{?}{=} \text{H}\_{s}( k\_{1\text{-}1})\_{calculated} \\\\
\text{verify:} \mspace{9mu} \text{H}\_{s}( k\_{1\text{-}2})\_{shared} \overset{?}{=} \text{H}\_{s}( k\_{1\text{-}2})\_{calculated}
$$

For this round they choose Bob to play Alice's part when they set up and conclude the transaction. Bob is able to do 
this, because he now has Alice's private blinding factors $ k\_{1\text{-}1} $ and $ k\_{1\text{-}2} $. When constructing 
the signature on Alice's behalf, he chooses a private nonce $ r^{'}\_n $ she does not know, as it will only be used to 
construct the signature and then never again. Bob and Carol conclude the transaction, let Alice know this and inform her 
that a consecutive multi-spend needs to start at round&nbsp;2.

The next time two of our friends want to spend some or all of the remainder of their multiparty UTXO, they will just 
repeat these steps, starting at round&nbsp;2. The only thing that will be different will be the person nominated to play 
the part of the party that is absent; they have to take turns at this.


### Spending Protocol

Alice, Bob and Carol are now seasoned at setting up their $ 2\text{-of-}3 $ scheme and spending the UTXO until all funds 
are depleted. They have come to an agreement on a simple spending protocol, something to help keep all of them honest:

- All parties must always know who shared shards and who played the missing party's role for each round. Therefore, all 
parties must always be included in all sharing communication, even if they are offline. They can then receive those 
messages when they become available again.

- The spend is aborted if any verification step does not complete successfully. To recommence, the parties have to 
cancel all unused shards, calculate new shards for the remainder and start again. For this, all parties need to be 
present.

- No party may play the role of an absent party twice in a row. If Alice is absent, which usually happens, Bob and Carol 
must take turns.


## Conclusions, Observations and Recommendations


### Comparison to Bitcoin

Given the basic differences between Bitcoin and Mimblewimble as mentioned in the [Introduction](#introduction), and the 
multiparty payment scheme introduced here, the following observations can be made:

1. Miner Validation

   In Bitcoin, the P2SH multisig redeem script is validated by miners whenever a multisig payment is invoked. In 
   addition, they also validate the public keys used in the transaction.

   With Mimblewimble multiparty transactions, miners cannot validate who may be part of a multiparty transaction, only 
   that the transaction itself complies with basic Mimblewimble rules and that all Bulletproof range proofs are valid. The onus 
   is on the parties themselves to enforce basic spending protocol validation rules.

1. $ m\text{-of-}n $

   In both Bitcoin and Mimblewimble, $ m\text{-of-}n $ transactions are possible. The difference is in where and how 
   validation rules are applied.

1. Security

   Bitcoin multiparty transactions are more secure due to the validation rules being enforced by the miners.

1. Complexity

   Implementing $ m\text{-of-}n $ multiparty transactions in Mimblewimble will involve wallets to store more data and 
   implement more functionality than when compared to Bitcoin, e.g. the SSSS.


### General

1. Bulletproof Range Proof

   The choice between using Dalek's Bulletproofs MPC Protocol or Grin's multiparty Bulletproof to construct a multiparty 
   range proof is important. Although using Dalek's method involves more rounds of communication, with a slightly larger 
   proof size and the requirement to have the coins colored, it has definite advantages. It trumps on security while 
   executing the protocol and wallet reconstructability.

1. Practicality

   The Mimblewimble multiparty Bulletproof UTXO as proposed can be practically implemented for both the 
   $ n\text{-of-}n $ and $ m\text{-of-}n $ cases.

1. Information Sharing Protocol

   The [Simple Sharing Protocol](#simple-sharing-protocol) as suggested here may need some more work to make it optimal.
   
1. Generalization

   Although all examples presented here were for three parties, this could be easily generalized.


## References

[[1]] "The Best Step-by-Step Bitcoin Script Guide Part 2" [online]. Available: 
<https://blockgeeks.com/guides/bitcoin-script-guide-part-2>. Date accessed: 2019&#8209;05&#8209;02.

[1]: https://blockgeeks.com/guides/bitcoin-script-guide-part-2
"The Best Step-by-Step Bitcoin Script Guide Part 2"

[[2]] "Script" [online]. Available: <https://en.bitcoin.it/wiki/Script>. Date accessed: 2019&#8209;05&#8209;06.

[2]: https://en.bitcoin.it/wiki/Script
"Script"

[[3]] "Multisignature" [online]. Available: <https://en.bitcoin.it/wiki/Multisignature>. 
Date accessed: 2019&#8209;05&#8209;06.

[3]: https://en.bitcoin.it/wiki/Multisignature
"Multisignature"

[[4]] "Transaction" [online]. Available: <https://en.bitcoin.it/wiki/Transaction>. Date accessed: 2019&#8209;05&#8209;06.

[4]: https://en.bitcoin.it/wiki/Transaction
"Transaction"

[[5]] S. Pour, "Bitcoin Multisig the Hard Way: Understanding Raw P2SH Multisig Transactions" [online]. Available: <https://www.soroushjp.com/2014/12/20/bitcoin-multisig-the-hard-way-understanding-raw-multisignature-bitcoin-transactions>. 
Date accessed: 2019&#8209;05&#8209;06.

[5]: https://www.soroushjp.com/2014/12/20/bitcoin-multisig-the-hard-way-understanding-raw-multisignature-bitcoin-transactions
"Bitcoin Multisig the Hard Way: 
Understanding Raw P2SH Multisig Transactions"

[[6]] GitHub: "gavinandresen/TwoOfThree.sh" [online]. Available: <https://gist.github.com/gavinandresen/3966071>. 
Date accessed: 2019&#8209;05&#8209;06.

[6]: https://gist.github.com/gavinandresen/3966071
"GitHub: gavinandresen/TwoOfThree.sh"

[[7]] H.de Valence, I. Lovecruft and O. Andreev, "Merlin Transcripts" [online]. Available: <https://merlin.cool/index.html> 
and <https://doc-internal.dalek.rs/merlin/index.html>. Date accessed: 2019&#8209;05&#8209;10.

[7]: https://doc-internal.dalek.rs/merlin/index.html
"Merlin Transcripts" 

[[8]] T. Pedersen. "Non-interactive and Information-theoretic Secure Verifiable Secret Sharing" 
[online]. Available: <https://www.cs.cornell.edu/courses/cs754/2001fa/129.pdf>. 
Date accessed: 2019&#8209;05&#8209;10.

[8]: https://www.cs.cornell.edu/courses/cs754/2001fa/129.pdf
"Non-interactive and Information-theoretic
Secure Verifiable Secret Sharing"

[[9]] "GrinExplorer, Block 164,690" [online]. Available: <https://grinexplorer.net/block/0000016c1ceb1cf588a45d0c167dbfb15d153c4d1d33a0fbfe0c55dbf7635410>. 
Date accessed: 2019&#8209;05&#8209;10.

[9]: https://grinexplorer.net/block/0000016c1ceb1cf588a45d0c167dbfb15d153c4d1d33a0fbfe0c55dbf7635410
"GitHub: gavinandresen/TwoOfThree.sh"

[[10]] "Dalek Cryptography - Crate Bulletproofs - Module bulletproofs::range_proof_mpc" [online]. Available: 
<https://doc-internal.dalek.rs/bulletproofs/range_proof_mpc/index.html>. Date accessed: 2019&#8209;05&#8209;10.

[10]: https://doc-internal.dalek.rs/bulletproofs/range_proof_mpc/index.html
"Dalek Cryptography - Crate Bulletproofs
Module bulletproofs::range_proof_mpc"

[[11]] B. Bünz, J. Bootle, D. Boneh, A. Poelstra, P. Wuille and G. Maxwell, "Bulletproofs: Short Proofs for 
Confidential Transactions and More" (Slides) [online]. Available: 
<https://cyber.stanford.edu/sites/default/files/bpase18.pptx>. Date accessed: 2019&#8209;05&#8209;11. 

[11]: https://cyber.stanford.edu/sites/default/files/bpase18.pptx
"Bulletproofs: Short Proofs for Confidential Transactions and More (Slides)"

[[12]] "Grin Multiparty Bulletproof - jaspervdm" [online]. Available: <https://i.imgur.com/s7exNSf.png>. 
Date accessed: 2019&#8209;05&#8209;10.

[12]: https://i.imgur.com/s7exNSf.png
"Grin Multiparty Bulletproof - jaspervdm"

[[13]] GitHub: "Multi-party bulletproof PR#24" [online]. Available: <https://github.com/mimblewimble/secp256k1-zkp/pull/24>. 
Date accessed: 2019&#8209;05&#8209;10.

[13]: https://github.com/mimblewimble/secp256k1-zkp/pull/24
"GitHub: Multi-party bulletproof PR#24"

[[14]] GitHub: "secp256k1-zkp/src/modules/bulletproofs/tests_impl.h, test_multi_party_bulletproof" [online]. Available: 
<https://github.com/mimblewimble/secp256k1-zkp/blob/master/src/modules/bulletproofs/tests_impl.h>. 
Date accessed: 2019&#8209;05&#8209;10.

[14]: https://github.com/mimblewimble/secp256k1-zkp/blob/master/src/modules/bulletproofs/tests_impl.h
"GitHub: secp256k1-zkp/src/modules/bulletproofs/tests_impl.h, 
test_multi_party_bulletproof"

[[15]] "MATH3024 Elementary Cryptography and Protocols: Lecture 11, Secret Sharing", University of Sydney, School of 
Mathematics and Statistics [online]. Available: <http://iml.univ-mrs.fr/~kohel/tch/MATH3024/Lectures/lectures_11.pdf>. 
Date accessed: 2018&#8209;09&#8209;18.

[15]: http://iml.univ-mrs.fr/~kohel/tch/MATH3024/Lectures/lectures_11.pdf
"MATH3024 Elementary Cryptography and Protocols: 
Lecture 11, Secret Sharing" 

[[16]] I. Coleman, "Online Tool: Shamir Secret Sharing Scheme" [online]. Available: <https://iancoleman.io/shamir>. 
Date accessed: 2019&#8209;05&#8209;27.

[16]: https://iancoleman.io/shamir
"Online Tool: Shamir Secret Sharing Scheme" 


## Appendices


### Appendix A: Notation Used

This section gives the general notation of mathematical expressions used. It provides important pre-knowledge for the 
remainder of the report.

- All Pederson Commitments will be of the [elliptic derivative]((../../cryptography/bulletproofs-protocols/MainReport.md#pedersen-commitments-and-elliptic-curve-pedersen-commitments)) 
depicted by $  C(v,k) = (vH + kG)  $ with $ v $ being the value committed to and $ k $ being the blinding factor.

- Scalar multiplication will be depicted by "$ \cdot $", as an example $ e \cdot (vH + kG) = e \cdot vH + e \cdot kG  $.

- A Pederson Commitment to the value of $ 0 $ will be depicted by $ C(0,k) = (0H + kG) = (kG) = (\mathbf{0}) $.

- Let $ \text{H}\_{s}(arg) $ be a collision-resistant hash function used in an information sharing protocol where 
$ arg $ is the value being committed to.

- Let $  RP\_n  $ be Bulletproof range proof data for commitment $ C\_n $.

- Let $  RP\_{agg} $ be aggregated Bulletproof range proof data for a set of commitments 
$ \lbrace C\_1, C\_2, ... , C\_n \rbrace $.


### Appendix B: Definition of Terms

Definitions of terms presented here are high level and general in nature. Full mathematical definitions are available in 
the cited references.

- **Shamir's Secret Sharing Scheme:**<a name="ssss"> </a>A $ (m, n) ​$ threshold secret sharing scheme is a method for 
$ n ​$ parties to carry shards/shares $ s\_i ​$ of a secret message $ s ​$ such that any $ m ​$ of them can reconstruct 
the message [[15]]. 
  
  - The threshold scheme is perfect if knowledge of $ m − 1 $ or fewer shards provides no information regarding $ s $. 
  - Shamir's Secret Sharing Scheme provides a perfect $ (m, n) $ threshold scheme using Lagrange interpolation. 
  - Given $ m $ distinct points $ (x\_i, y\_i) $ of the form $ (x\_i, f(x\_i)) $, where $ f(x) $ is a polynomial of 
  degree less than $ m $, then $ f(x) $ is determined by
  
  $$
  f ( x ) = \sum \_ { i = 1 } ^ { m } y \_ { i } \prod \_ { 1 \leq j \leq m \atop i \neq j } \frac { x - x \_ { j } } { x \_ { i } - x \_ { j } }
  $$
  
  - Shamir’s scheme is defined for a secret message $ s \in \mathbb{Z}/p\mathbb{Z} $ with prime $ p $, by setting 
  $ a\_0 = s$, and choosing $ a\_1, . . . , a\_{m−1} $ at random in $ \mathbb{Z}/p\mathbb{Z} $. The trusted party 
  computes $ f(i) $ for all $ 1 \leq i \leq n $, where
  
  $$
  f ( x ) = \sum \_ { k = 0 } ^ { m - 1 } a \_ { k } x ^ { k }
  $$
  
  - The shards $ (i, f(i)) ​$ are distributed to the $ n ​$ distinct parties. Since the secret is the constant term 
  $ s = a\_0 = f(0) ​$, the secret is recovered from any $ m ​$ shards $ (i, f(i)) ​$ for $ I \subset \{ 1 , \ldots , n \} ​$ 
  by
  
  $$
  s = \sum _ { i \in I } c _ { i } f ( i ) , \text { where each } c _ { i } = \prod _ { j \in I \atop j \neq i } \frac { i } { j - i }
  $$

[ssss~]: #ssss
"Shamir's Secret Sharing Scheme is an (m, n) 
threshold scheme for n parties to carry shares 
of a secret message such that any m of the 
them can reconstruct the message."


### Appendix C: Shamir's Secret Sharing Example

One of the private blinding factors Alice calculated and wants to share with Bob and Carol is

<div class="mywrap">29bb078b7b2b01e62dd684cd20742b510ece6175fa58d7a79cceeefe5297a804</div>

Alice's shard:

<div class="mywrap">
80168f0598f73bc29f90aaeb0d87caf44a8f6baaaeabcf15af20dd1d27e28f30b7afb6c889da2eefc6cca9a1505a92d8b65f968f554dede39a23b293aa7ddd4d50499d0fbfe86d60b4ff87938609cda5d3aecf37d7fc8ff54f777e26ad2dc4d1268a7a4ba0c94706a017e0a1e3fd969e7c2627e28dc3e8ecb7f73426ffe2c5b72467905d072528f060c29f9d221a93dfb213add323fee3373ff11cace1a87f0b095029b32f0fcf6735ce8611f823c9b697452b728a34f0dee13d2b2a5f30f70d096cd7ba6cf9f7b0431673373ffa1e358d9d8c02f9bec54b53f66ec76069499cd21d95099f826a91d92665a6709f607793c1b2548ccb1f6b46c3cbc0af6ed55b02a
</div>

Bob's shard:

<div class="mywrap">
802d0fdb203e66552ef14417dadf843884df16949c965ffb4f91abfb9fc50fb16f4ebd80d2759c1e5d889292a0a4f5a0bcaefd0f7a8a1a1725976527453a7b5b7082fbdebe111b1169eedf270c025a9ba74c5fbfafe8de3a8f3eed9d4b9a59a24d05355691835e0d402fc143c7eafd2d399c4fc50a57c018bfee684dee158b6e48ff256bdb4a44b0cbb52b6b9094f2ceb1d740864d4c1c9e64c2226819113587d7c0472645ee5a8e614cd8a3e467888d242a42c50029e49c17cb9d6575e1e4fbd6187a551dc23500834ce3ae6ba5f8bb0f9ad315e8ec5f47736cc9feca035838714affa2f424c1c3a69cc12ce44f1a8f22136ec90cd7fa8758878c31457c7e17d02
</div>

Carol's shard:

<div class="mywrap">
803b80deb8c95d97b161eefcd7584eccce507d3e323d90eee0b176e6b8278081d8e10b485bafb2f19b443b33f0fe67780af16b802fc7f7f4bfb4d7b4ef47a61620cb66d101f97671dd1158b48a0b973e74e29088781451cfc04993bbe6b79d736b8f4f1d314a190be03821e224176bb345ba6827879428f408195c6b11f74ed96c88b676df6f6fc0ad67b786b11e62310094eb0568d2f9f95d633894fe894cbcdd006de56cf1969952c25d321f14472bb50f6ae7896d1752f596b01f2ce115a6dc74aeff726bc490c37a93d9572fe58e812759571762994c231aa459ac2a17e4a07769cb6df6a8427cbaa2aa97f07cd8b2a2dabd839ce69c1d4441d1ec32a8dcd1a
</div>

Using any two of these shards with the tool provided in [[16]], Alice's original private blinding factor can be recreated. Try it out!

**Note:** This example shows blinding factor shards created with the SSSS tool developed by Coleman [[16]]. It was merely developed for demonstration purposes and does not make any claims to meet cryptographic security rules other than using secure randomness.


## Contributors

- <https://github.com/hansieodendaal>
- <https://github.com/anselld>
