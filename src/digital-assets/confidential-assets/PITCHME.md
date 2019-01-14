<head>
<style>
div.LineHeight5per {
  line-height: 5%;
}
div.LineHeight10per {
  line-height: 10%;
}
div.LineHeight20per {
  line-height: 20%;
}
div.LineHeight50per {
  line-height: 50%;
}
div.LineHeight75per {
  line-height: 75%;
}
div.LineHeight100per {
  line-height: 100%;
}
div.LineHeight200per {
  line-height: 200%;
}
</style>
</head>



## Confidential Assets

<div class="LineHeight20per"> <br></div>

@div[text-left]

*Confidential assets have value, can be owned (right to use), has no physical presence (intangible), stored in digital form/online account. The confidentiality aspect implies that the amount of assets owned & the asset type transacted in can be confidential.*

@divend

<div class="LineHeight100per"> <br></div>

- Preliminaries
- Confidential Transactions Overview
- Asset Commitments and Surjection Proofs
- Asset Transactions
- Asset Issuance, Reissuance
- Implementations
- Ricardian Contracts vs. Smart Contracts
- Conclusions

---

## Preliminaries

<div class="LineHeight20per"> </div>

@div[text-left]

**General notation**

@divend

<div class="LineHeight20per"> <br></div>

- Let $ p $ be a large prime number
- Let $ \mathbb G $ denote a cyclic group of prime order $ p $ 
- Let $ \mathbb Z_p $ denote the ring of integers $ modulo \mspace{4mu} p $ 
- Let $ \mathbb F_p $ be a group of elliptic curve points over a finite (prime) field
- If not otherwise specified, lower case $ x,r,y $ etc. are ordinary numbers (integers), upper case $ H,G $ are curve points

<div class="LineHeight100per"> <br></div>

@div[text-left]

<u>Discrete Logarithm Problem (DLP):</u> The discrete logarithm $ \log_ba = k $ such that $ b^k=a $ for any integer $ k $ where $ a,b \in \mathbb G $ is hard to guess (has no efficient solution) for carefully chosen $ \mathbb F_p $. 

<div class="LineHeight100per"> <br></div>

<u>Surjection Function:</u> A surjection function simply means that for every element $ y $ in the codomain $ Y $ of function $ f $ there is at least one element $ x $ in the domain $ X $ of function $ f $ such that $ f(x) = y$. 

@divend

---

## Confidential Transactions Overview

@div[text-left]

<u>Confidential transactions</u> made confidential by replacing each UTXO with a homomorphic commitment (e.g. Pedersen Commitment), and made robust against overflow and inflation attacks by using efficient ZK range proofs (e.g. Bulletproofs).

<div class="LineHeight20per"> <br></div>

<u>Range proofs</u> provide proof that secret committed value lies in certain interval, prevents numbers coming near magnitude of large prime, say $ 2^{256} $, that can cause wrap around when adding a small number, e.g. proof that a number $ x \in [0,2^{64} - 1] $.

<div class="LineHeight20per"> <br></div>

<u>Pedersen Commitments</u> are perfectly hiding (an attacker with infinite computing power cannot tell what amount has been committed to) and computationally binding (no efficient algorithm running in a practical amount of time can produce fake commitments except with small probability).

@divend

+++

@div[text-left]

<u>Elliptic Curve (EC) Pedersen Commitment (PC)</u> to value $ x \in \mathbb Z_p $ with $ r \in \mathbb Z_p $ a random blinding factor is

`
$$
C(x,r) = xH + rG
$$
`

Here $ G \in \mathbb F_p $ is a random generator point and $ H \in \mathbb F_p $ specially chosen so that $ x_H $ satisfying $ H = x_H G $ cannot be found except if the EC DLP is solved. In secp256k1 $ H $ is the SHA256 hash of simple encoded $ x $-coordinate of generator point $ G $. The number $ H $ is what is known as a Nothing Up My Sleeve (NUMS) number. 

<div class="LineHeight100per"> <br></div>

A <u>PC implementation</u> uses three algorithms: **<code>Setup()</code>** to set up the commitment parameters $ G $ and $ H $; **<code>Commit()</code>** to commit to the message $ x $ using the commitment parameters $ r $, $ H $ and $ G $ and **<code>Open()</code>** to open and verify the commitment.

<div class="LineHeight100per"> <br></div>

Mimblewimble use these confidential transaction primitives, but <u>if confidentiality is not sought</u>, the homomorphic commitment to the given amount will have a blinding factor $ r = 0 $.

@divend



---

## Asset Commitments and Surjection Proofs

@div[text-left]

Confidential assets must be confidential and proven to not be inflationary; this is made possible by using asset commitments and Asset Surjection Proofs (ASP).

<div class="LineHeight100per"> <br></div>

Given unique asset description $ A ​$ the associated asset tag $ H_A \in \mathbb G ​$ is calculated using the PC function <code>Setup()</code> with $ A ​$ as auxiliary input. (*Selection of $ A ​$ is discussed later.*) Consider a transaction with 2 inputs & 2 outputs involving 2 asset types $ A ​$ and $ B ​$ 

@divend

`
$$
\begin{aligned}
in_A = x_1H_A + r_{A_1}G \\
out_A = x_2H_A + r_{A_2}G \\
in_B = y_1H_B + r_{B_1}G \\
out_B = y_2H_B + r_{B_2}G
\end{aligned}
$$
`

@div[text-left]

For this to hold the sum of the outputs minus the sum of the inputs must be zero:

@divend



+++

`
$$
\begin{aligned}
(out_A + out_B) - (in_A + in_B) &= 0 \\
(x_2H_A + r_{A_2}G) + (y_2H_B + r_{B_2}G) - (x_1H_A + r_{A_1}G) - (y_1H_B + r_{B_1}G) &= 0 \\
(r_{A_2} + r_{B_2} - r_{A_1} - r_{B_1})G + (x_2 - x_1)H_A + (y_2 - y_1)H_B &= 0
\end{aligned}
$$
`

@div[text-left]

Since $ H_A $ and $ H_B $ are both NUMS asset tags, total input and output amounts of assets $ A $ and $ B $ must be equal respectively. However, asset types are publicly visible, thus not confidential. Let's replace each asset tag with blinded version of itself, thus asset commitment to asset tag $ H_A $ (blinded asset tag) is then

@divend

`
$$
H_{0_A} = H_A + rG
$$
`

@div[text-left]

Thus, such a PC commits to the committed amount as well as to the underlying asset tag.

@divend

+++

@div[text-left]

A commitment to the value `$ x_1 $` using blinded asset tag `$ H_{0_A} $` is also a commitment to `$ x_1 $` using the asset tag `$ H_A $` as seen below:

@divend

`
$$
\begin{aligned}
C(x_1, r_{A_1}) &= x_1H_{0_A} + r_{A_1}G \\
&= x_1(H_A + rG) + r_{A_1}G \\
&= x_1H_A + (r_{A_1} + x_1r)G \\
&= C(x_1, r_{A_1} + x_1r)
\end{aligned}
$$
`

@div[text-left]

Correspondingly, the zero sum rule translates to:

@divend

`
$$
\begin{aligned}
(out_A + out_B) - (in_A + in_B) &= 0 \\
(x_2H_{0_A} + r_{A_2}G) + (y_2H_{0_B} + r_{B_2}G) - (x_1H_{0_A} + r_{A_1}G) - (y_1H_{0_B} + r_{B_1}G) &= 0 \\
(r_{A_2} + r_{B_2} - r_{A_1} - r_{B_1})G + (x_2 - x_1)H_{0_A} + (y_2 - y_1)H_{0_B} &= 0
\end{aligned}
$$
`

+++

@div[text-left]

However, using only the sum to zero rule it is still possible to introduce negative amounts of an asset type. Consider blinded asset tag

@divend

`
$$
H_{0_A} = -H_A + rG
$$
`

@div[text-left]

Any amount of blinded asset tag `$ H_{0_A} $` will correspond a negative amount of asset $ A $, thereby inflating its supply. Thus, the ASP is introduced.

<div class="LineHeight100per"> <br></div>

An ASP scheme provides a proof $ \pi $ for a set of input asset commitments `$ [ H_i ] ^n_{i=1} $`, an output commitment `$ H = H_{\hat i} + rG $` for $ \hat i = 1 \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu} n $ and blinding factor $ r $.

@divend

+++

@div[text-left]

It proofs that every output asset type is the same as some input asset type while blinding which outputs correspond to which inputs. Such a proof $ \pi $ is secure if it is a zero-knowledge proof of knowledge for the blinding factor $ r $. 

<div class="LineHeight100per"> <br></div>


Let `$ H_{0_{A1}} $` and `$ H_{0_{A2}} $` be blinded asset tags that commit to the same asset tag `$ H_A $`:

@divend

`
$$
H_{0_{A1}} = H_A + r_1G \\
H_{0_{A2}} = H_A + r_2G
$$
`

@div[text-left]

Taking the difference we have

@divend

`
$$
\begin{aligned}
\mathrm{difference}_{H_{0}} &= H_{0_{A1}} - H_{0_{A2}} \\
&= (H_A + r_1G) - (H_A + r_2G) \\
&= (r_1 - r_2)G
\end{aligned}
$$
`

@div[text-left]

This is a signature key with secret key $ r_1 - r_2 $. 

@divend

+++

@div[text-left]

Thus for $ n $ distinct multiple asset type transaction, differences can be calculated between each output and all inputs and so on for all outputs:

@divend

`
$$
(out_A - in_A) \mspace{3mu} , \mspace{3mu} (out_A - in_B)  \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu}  (out_A - in_n)
$$
`

@div[text-left]

This has the form of a ring signature. If $ out_A  ​$ has the same asset tag as an input, the transaction signer will know the secret key corresponding to one of these differences, and able to produce the ring signature. 

<div class="LineHeight100per"> <br></div>

The ASP is based on the *Back-Maxwell* range proof, a variation of *Borromean* ring signatures, in turn is a variant of the *Abe-Ohkubo-Suzuki* (AOS) ring signature. 

@divend

+++

@div[text-left]

An AOS ASP computes a ring signature that is equal to the proof $ \pi $ as follows:

@divend

- Calculate $ n $ differences $ H - H_{\hat i } $ for $  \hat i = 1 \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu} n  $, one of which will be equal to the blinding factor $ r $;
- Calculate a ring signature $ S $ of an empty message using the $ n $ differences. 

@div[text-left]

The resulting ring signature $ S $ is equal to the proof $ \pi $, and the ASP consist of this ring signature $ S ​$.

@divend

---

## Asset Transactions

???

---

## Asset Issuance, Reissuance

???

---

## Flexibility

???

---

## Implementations

<div class="LineHeight100per"> </div>

  - Elements Project
  - Chain Core Confidential Assets
  - Cloak

+++

### Elements Project

???

+++


### Chain Core Confidential Assets

???

+++


### Cloak

???

---


## Ricardian Contracts vs. Smart Contracts

@div[text-left]

“*<u>Ricardian Contract:</u> A digital contract that deﬁnes the terms and conditions of an interaction, between two or more peers, that is cryptographically signed and veriﬁed, being both human and machine readable and digitally signed.*”

<div class="LineHeight100per"> </div>

Ricardian contracts are robust (due to identification by cryptographic hash functions), transparent (due to readable text for legal prose) and efficient (due to computer markup language to extract essential information).

<div class="LineHeight100per"> </div>

“*<u>Smart Contract</u>: A computerized transaction protocol that executes the terms of a contract. The general objectives are to satisfy common contractual conditions.*”

<div class="LineHeight100per"> </div>

With smart contracts, digital assets can be exchanged in a transparent and non-conflicting way; it provides trust. 

@divend

+++

@div[s750px]

![Ricardian Contract](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/digital-assets/confidential-assets/sources/ricardian_contract.png)

@divend

+++

@div[left-50 text-left]

<u>Ricardian Contract:</u>

<ul>
<li>Human readable;
<li>Document is printable;
<li>Program parsable;
<li>All forms (displayed, printed, parsed) are manifestly equivalent;
<li>Signed by issuer;
<li>Can be identified securely, where security means that any attempts to change the linkage between a reference and the contract are impractical.
</ul>

@divend

@div[right-50 text-left]

<u>Smart Contract</u>

<ul>
<li>Self-executing (of course, it means that they don’t run unless someone initiates them)
<li>Immutable
<li>Self-verifying
<li>Auto-enforcing
<li>Cost saving
<li>Removes third parties or escrow agents
</ul>

@divend

---

## Conclusions

???
