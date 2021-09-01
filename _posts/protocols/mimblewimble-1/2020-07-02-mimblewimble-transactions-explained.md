---
layout: post
title:  Mimblewimble Transactions Explained
menus:
  protocols:
    weight: 2
date:   2020-07-02 15:00:00 +0300
image:  '/images/banner-07.jpg'
category: protocols
tags:   [protocols, mimblewimble]
usemathjax: true
featured:
excerpttext: Mimblewimble is a privacy-oriented, cryptocurrency technology. It differs from Bitcoin in some key areas...
---

## Table of Contents

- [High-level Overview](#high-level-overview)
- [Transactions Explained](#transactions-explained)
  - [Basic Transaction](#basic-transaction)
  - [Blinding Factors](#blinding-factors)
  - [Alice Prepares a Transaction](#alice-prepares-a-transaction)
  - [Bob Prepares his Response](#bob-prepares-his-response)
  - [Alice Completes and Broadcasts the Transaction](#alice-completes-and-broadcasts-the-transaction)
- [Transaction Verification and Propagation](#transaction-verification-and-propagation)
- [Stopping Fraud](#stopping-fraud)
- [Transaction Summary](#transaction-summary)
- [References](#references)
- [Contributors](#contributors)


## High-level Overview

Mimblewimble is a privacy-oriented, cryptocurrency technology. It differs from Bitcoin in some key areas:

* No addresses. The concept of Mimblewimble addresses does not exist.
* Completely private. Every transaction is confidential.
* Compact blockchain. Mimblewimble uses a different set of security guarantees to Bitcoin, which leads to a far more
compact blockchain.

## Transactions Explained

Confidential transactions [[1]] were invented by Dr. Adam Back and are employed in several cryptocurrency projects,
including Monero and Tari - by way of Mimblewimble.

Recipients of Tari create the private keys for receiving coins on the fly. Therefore they must be involved in the
construction of a Tari transaction.

This doesn't necessarily mean that recipients have to be online. However, they do need to be able to communicate,
whether it
be by email, Instant Messaging (IM) or carrier pigeon.


### Basic Transaction

We'll explain how Alice can send Tari to Bob using a two-party protocol for Mimblewimble. Multiparty transactions are
similar, but the flow of information is a bit different and takes place over additional communication rounds.

Let's say Alice has 300 µT and she wants to send 200 µT to Bob.

Here’s the basic transaction:

| Inputs       | Outputs |        |      |
|:-------------|:------- |:-------|:-----|
| 300          | 200     | 90     | 10   |
| Alice's UTXO | To Bob  | Change | fee  |

If we write this as a mathematical equation, where outputs are positive and inputs are negative, we should be able to
balance things out so that there's no creation of coins out of thin air:

$$ -300 + 200 + 90 + 10 = 0 ​$$

This is basically the information that sits in the Bitcoin blockchain. Anyone can audit anyone else's transactions
simply by inspecting the global ledger's transaction history. This isn't great for privacy.

Here is where confidential transactions come in. We can start by multiplying both sides of the preceding equation by some
generator point _H_ on the elliptic curve (for an introduction to elliptic curve math, refer to
[this presentation](../../cryptography/crypto-1/sources/PITCHME.link.md)):

$$ 300.H = 200.H + 90.H + 10.H ​$$

Since _H_ is a constant, the math above still holds, so we can validate that Alice is not stealing by checking that

$$(3.H) - (2.H) - (1.H) - (f.H) \equiv 0.H = 0 $$

Notice that we _only see public keys_ and thus the values are hidden. Cool!

<div class="note info">
Technically, only scalar integer values are valid for elliptic curve multiplication. This is why we use µT in the
transactions so that the amounts are always integers.
</div>
There's a catch though. If _H_ is constant and known (it is), couldn’t someone just pre-calculate $$n.H​$$ for all
reasonable values of _n_ and scan the blockchain for those public keys?<sup>[^a]</sup>

In short, _yes_. So we’re not done yet.


[^a]: "This is called a pre-image attack."

### Blinding Factors

To prevent a pre-image attack from unblinding all the values in Tari transactions, we need to add randomness to each
input and output. The basic idea is to do this by adding a second public key to each transaction output.

So if we rewrite the inputs and outputs as follows:

$$ C_i = n_i.H + k_i.G $$

where _G_ is another generator on the same curve, this completely _blinds_ the inputs and outputs so that no pre-image
attack is possible. This formulation is called a _Pedersen commitment_ [[3]].

<div class="note warning">
The two generators, _H_ and _G_ must be selected in a way that it's impossible to convert values from one generator to
the other [[2]]. Specifically, if _G_ is the base generator, then there exists some _k_ where $$ H = kG $$

If anyone is able to figure out this _k_, the whole security of Confidential Transactions falls apart. It's left as an
exercise for the reader to figure out why.

For a semi-gentle introduction to these concepts, Adam Gibson's paper on the subject is excellent [[5]].

</div>


### Alice Prepares a Transaction

Alice can now start to build a transaction.

| Type          | Formula                 | Name |
|:--------------|:------------------------|:-----|
| Input         | $$ -300.H - k_1.G $$    | C1   |
| Change output | $$ 90.H + k_2.G $$      | C2   |
| Fee           | $$ 10.H + 0.G $$        | f    |
| Total spent   | $$ 200.H + 0.G $$       | C*   |
| _Sum_         | $$ 0.H + (k_2-k_1).G $$ | X    |

The \\( k_i \\)-values, \\(k_1, k_2\\) are the spending keys for those outputs.

<div class="note warning">
The _only pieces of information you need to spend Tari outputs_ are the spending key
(also called a blinding factor) and its associated value.

</div>

Therefore for this transaction, Alice's wallet, which tracks all of her Tari unspent outputs, would have provided the
blinding factor and the value "300" to complete the commitment _C1_.

Notice that when all the inputs and outputs are summed (including the fee), all the values cancel to zero, as they
should. Notice also that the only term left is multiplied by the point _G_. All the _H_ terms are gone. We call this sum
the _public excess_ for Alice's part of the transaction.

We define the _excess_ of the transaction to be

$$ x_s = k_2 - k_1 $$

A simple way for Alice to calculate her excess (and how the Tari wallet software does it) is to sum
her output blinding factors and minus the sum of her input blinding factors.

Let's say Alice was trying to create some money for herself and made the change 100 µT instead of 90. In this instance,
the sum of the outputs and inputs would not cancel on _H_ and we would have

$$X^* = 10.H + x_s.G$$

We'll see in a bit how the Mimblewimble protocol catches Alice out if she tries to pull shenanigans like this.

<div class="note info">
Alice actually also prepares a range proof for each output, which is a proof that the value of the output is between
zero and 2^64 µT. Without range proofs, Alice could send negative amounts to people, enriching herself, and not breaking
any of the accounting of Tari.

</div>

<div class="note info">
 In Tari and Grin, the excess value is actually split into two values for added privacy. The Grin team has a good
 explanation of why this `offset` value is necessary [[4]]. We leave off this step to keep the explanation simple(r).

</div>

Alice also chooses a random _nonce_,

$$ r_a $$

and calculates the corresponding public nonce,

$$ R_a = r_a.G $$

Alice then sends the following information to Bob:

| Field              | Value     |
|:-------------------|:----------|
| Inputs             | C1        |
| Outputs            | C2        |
| Fee                | 10        |
| Amount paid to Bob | 200       |
| Public nonce       | $$ R_a $$ |
| Public excess      | X         |
| Metadata           | m         |

The message metadata is some extra bits of information that pertains to the transaction (such as when the output can be spent).

### Bob Prepares his Response

Bob receives this information and then sets about completing his part of the transaction.

First, he can verify that Alice has sent over the correct information by checking that the public excess, _X_, is
correct by following the same procedure that Alice used to calculate it above. This step isn't strictly necessary,
since he doesn't have enough information to detect any fraud at this point.

He then builds a commitment from the `amount` field that Alice is sending to him:

$$ C_b = 200.H + k_b.G $$

where \\(k_b\\) is Bob's private spend key. He calculates

$$P_b = k_b.G$$

and generates a range proof for the commitment.

Bob then needs to sign that he's happy that everything is complete to his satisfaction. He creates a partial
[Schnorr Signature](../../cryptography/digital_signatures/introduction_schnorr_signatures.md) with the challenge,

$$ e = H(R_a + R_b \Vert X + P_b \Vert f \Vert m) $$

and the signature is given by

$$ s_b = r_b + ek_b $$

Bob sends back

| Field                               | Value   |
|:------------------------------------|:--------|
| Output (commitment and range proof) | $$C_b$$ |
| Public nonce                        | $$R_b$$ |
| Signature                           | $$s_b$$ |
| Public key                          | $$P_b$$ |

### Alice Completes and Broadcasts the Transaction

After hearing back from Bob, Alice can wrap things up.

First, she can now use Bob's public nonce and public key to independently calculate the same challenge that Bob signed:

$$ e = H(R_a + R_b \Vert X + P_b \Vert f \Vert m) $$

Alice then creates both her own partial signature,

$$ s_a = r_a + e.x_s $$

and the combined aggregate signature, $$ s = s_a + s_b, R = R_a + R_b $$.

Alice can now broadcast this transaction to the network. The final transaction looks as follows:

| Transaction Kernel        |                |
|:--------------------------|:---------------|
| Public excess             | $$ X + P_b $$  |
| Signature                 | $$ (s, R) $$   |
| Fee                       | 10             |
| Transaction metadata      | m              |

| Transaction Body          |                |
|:--------------------------|:---------------|
| Inputs with range proofs  | $$[C_1]$$      |
| Outputs with range proofs | $$[C_2, C_B]$$ |

## Transaction Verification and Propagation

When a full node receives Alice's transaction, it needs to verify that it's on the level before sending it on to its
peers. The node wants to check the following:

1. All inputs come from the current UTXO set

   All full nodes keep track of the set of unspent outputs, so the node will check that _C1_ is in that list.

2. All outputs have a valid range proof

   The range proof is verified against its matching commitment.

3. The values balance

   In this check, the node wants to make sure that no coins are created or destroyed in the transaction. But how can it
   do this if the values are blinded?

   Recall that in an honest transaction, all the values (which are multiplied by _H_) cancel, and you're left with the
   sum of the public keys of the outputs minus the public keys of the inputs. This non-coincidentally happens to be the
   same value that is stored in the kernel as the public excess.

   The summed public nonces, _R_ are also stored in the kernel, so this allows the node to verify the signature by
   checking the following, where the challenge _e_ is calculated as before:

$$ s.G \stackrel{?}{=} R + e(X + P_b) $$  

4. The signature in the kernel is valid

   Therefore by validating the kernel signature, we also prove to ourselves that the transaction accounting is correct.

5. Various other consensus checks

   Such as whether the fee is greater than the minimum.

If all these checks pass, then the node will forward the transaction on to its peers and it will eventually be mined and
be added to the blockchain.

## Stopping Fraud

Now let's say Alice tried to be sneaky and used \\( X^* \\) as her excess; the one where she gave herself 100 µT change
instead of 90 µT. Now the values won't balance. The sum of outputs, inputs and fees will look something like this:

$$ 10.H + (x_s + k_b).G ​$$

So now when a full node checks the signature:

$$
\begin{align}
R + e(X^* + P_b) &\stackrel{?}{=} s.G  \\\\
R + e(10.H + x_s.G + P_b) &\stackrel{?}{=} (r_a + r_b + e(x_s + k_b)).G \\\\
R + e(10.H + X + P_b) &\stackrel{?}{=} (r_a + r_b).G + e(x_s + k_b).G \\\\
\mathbf{10e.H} + R + e(X + P_b) &\stackrel{?}{=} R + e(X + P_b) \\\\
\end{align}
$$

The signature doesn't verify! The node can't tell exactly what is wrong with the transaction, but it knows something is
up, and so it will just drop the transaction silently and get on with its life.

## Transaction Summary

To sum up: a Tari/MimbleWimble transaction includes the following:

* From Alice, a set of inputs that reference and spend a set of previous outputs.
* From Alice and Bob, a set of new outputs, including:
  * A value and a blinding factor (which is just a new private key).
  * A range proof that shows that the value is non-negative.
* The transaction fee, in cleartext,
* The public excess, which is the sum of all blinding factors used in the transaction.
* Transaction metadata.
* The excess blinding value used as the private key to sign a message attesting to the transaction metadata, and the
  public excess.

## References

[1]: https://www.mycryptopedia.com/what-are-confidential-transactions/ "What are Bitcoin Confidential Transactions?"
[[1]] "What are Bitcoin Confidential Transactions?" [Online.] Available: <https://www.mycryptopedia.com/what-are-confidential-transactions/>
Date accessed: 2019&#8209;04&#8209;09.

[2]: https://en.wikipedia.org/w/index.php?title=Nothing-up-my-sleeve_number&oldid=889582749 "Nothing-Up-My_Sleeve Number"
[[2]] "Nothing-Up-My_Sleeve Number" [online].<br>Available:
<https://en.wikipedia.org/w/index.php?title=Nothing-up-my-sleeve_number&oldid=889582749>. Date accessed: 2019&#8209;04&#8209;09.

[3]: https://en.wikipedia.org/wiki/Commitment_scheme "Commitment Scheme"
[[3]] Wikipedia: "Commitment Scheme" [online]. Available: <https://en.wikipedia.org/wiki/Commitment_scheme>.
Date accessed: 2019&#8209;04&#8209;09.

[4]: https://github.com/mimblewimble/grin/blob/master/doc/intro.md#kernel-offsets "Kernel Offsets"
[[4]] "Kernel Offsets, in Introduction to MimbleWimble and Grin" [online]. Available:
<https://github.com/mimblewimble/grin/blob/master/doc/intro.md#kernel-offsets>. Date accessed: 2019&#8209;04&#8209;09.

[5]: https://joinmarket.me/static/FromZK2BPs_v1.pdf "From Zero (Knowledge) to BulletProofs"
[[5]] A. Gibson, "From Zero (Knowledge) to BulletProofs" [online].
Available: <https://joinmarket.me/static/FromZK2BPs_v1.pdf>.
Date accessed: 2019&#8209;04&#8209;10.

## Contributors

- <https://github.com/CjS77>
- <https://github.com/anselld>
